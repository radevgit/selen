//! FlatZinc Tokenizer (Lexer)
//!
//! Converts FlatZinc source text into a stream of tokens with location tracking.

use crate::flatzinc::error::{FlatZincError, FlatZincResult};

/// Source location for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}

/// Token types in FlatZinc grammar
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Predicate,
    Var,
    Array,
    Of,
    Constraint,
    Solve,
    Satisfy,
    Minimize,
    Maximize,
    Int,
    Bool,
    Float,
    Set,
    True,
    False,
    
    // Identifiers and literals
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    
    // Operators and punctuation
    DoubleColon,    // ::
    Colon,          // :
    Semicolon,      // ;
    Comma,          // ,
    Dot,            // .
    DoubleDot,      // ..
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftBrace,      // {
    RightBrace,     // }
    Equals,         // =
    
    // End of file
    Eof,
}

/// Token with location information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

impl Token {
    pub fn new(token_type: TokenType, location: Location) -> Self {
        Token { token_type, location }
    }
}

/// Tokenizer state
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    fn current_location(&self) -> Location {
        Location::new(self.line, self.column)
    }
    
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.input.get(self.position + offset).copied()
    }
    
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.input.get(self.position) {
            self.position += 1;
            if *ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(*ch)
        } else {
            None
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_line_comment(&mut self) {
        // Skip until end of line
        while let Some(ch) = self.peek() {
            self.advance();
            if ch == '\n' {
                break;
            }
        }
    }
    
    fn skip_block_comment(&mut self) -> FlatZincResult<()> {
        let start_loc = self.current_location();
        self.advance(); // skip '/'
        self.advance(); // skip '*'
        
        loop {
            match self.peek() {
                None => {
                    return Err(FlatZincError::LexError {
                        message: "Unterminated block comment".to_string(),
                        line: start_loc.line,
                        column: start_loc.column,
                    });
                }
                Some('*') if self.peek_ahead(1) == Some('/') => {
                    self.advance(); // skip '*'
                    self.advance(); // skip '/'
                    break;
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
        Ok(())
    }
    
    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        result
    }
    
    fn read_number(&mut self) -> FlatZincResult<TokenType> {
        let start_loc = self.current_location();
        let mut num_str = String::new();
        let mut is_float = false;
        
        // Handle negative sign
        if self.peek() == Some('-') {
            num_str.push('-');
            self.advance();
        }
        
        // Read digits
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float && self.peek_ahead(1).map_or(false, |c| c.is_ascii_digit()) {
                is_float = true;
                num_str.push(ch);
                self.advance();
            } else if ch == 'e' || ch == 'E' {
                is_float = true;
                num_str.push(ch);
                self.advance();
                // Handle optional +/- after 'e'
                if let Some('+') | Some('-') = self.peek() {
                    num_str.push(self.advance().unwrap());
                }
            } else {
                break;
            }
        }
        
        if is_float {
            num_str.parse::<f64>()
                .map(TokenType::FloatLiteral)
                .map_err(|_| FlatZincError::LexError {
                    message: format!("Invalid float literal: {}", num_str),
                    line: start_loc.line,
                    column: start_loc.column,
                })
        } else {
            num_str.parse::<i64>()
                .map(TokenType::IntLiteral)
                .map_err(|_| FlatZincError::LexError {
                    message: format!("Invalid integer literal: {}", num_str),
                    line: start_loc.line,
                    column: start_loc.column,
                })
        }
    }
    
    fn read_string(&mut self) -> FlatZincResult<String> {
        let start_loc = self.current_location();
        self.advance(); // skip opening quote
        
        let mut result = String::new();
        loop {
            match self.peek() {
                None | Some('\n') => {
                    return Err(FlatZincError::LexError {
                        message: "Unterminated string literal".to_string(),
                        line: start_loc.line,
                        column: start_loc.column,
                    });
                }
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.peek() {
                        Some('n') => { result.push('\n'); self.advance(); }
                        Some('t') => { result.push('\t'); self.advance(); }
                        Some('\\') => { result.push('\\'); self.advance(); }
                        Some('"') => { result.push('"'); self.advance(); }
                        _ => {
                            return Err(FlatZincError::LexError {
                                message: "Invalid escape sequence".to_string(),
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                Some(ch) => {
                    result.push(ch);
                    self.advance();
                }
            }
        }
        Ok(result)
    }
    
    pub fn next_token(&mut self) -> FlatZincResult<Token> {
        self.skip_whitespace();
        
        // Handle comments
        while self.peek() == Some('%') || (self.peek() == Some('/') && self.peek_ahead(1) == Some('*')) {
            if self.peek() == Some('%') {
                self.skip_line_comment();
            } else {
                self.skip_block_comment()?;
            }
            self.skip_whitespace();
        }
        
        let loc = self.current_location();
        
        match self.peek() {
            None => Ok(Token::new(TokenType::Eof, loc)),
            
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let ident = self.read_identifier();
                let token_type = match ident.as_str() {
                    "predicate" => TokenType::Predicate,
                    "var" => TokenType::Var,
                    "array" => TokenType::Array,
                    "of" => TokenType::Of,
                    "constraint" => TokenType::Constraint,
                    "solve" => TokenType::Solve,
                    "satisfy" => TokenType::Satisfy,
                    "minimize" => TokenType::Minimize,
                    "maximize" => TokenType::Maximize,
                    "int" => TokenType::Int,
                    "bool" => TokenType::Bool,
                    "float" => TokenType::Float,
                    "set" => TokenType::Set,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    _ => TokenType::Identifier(ident),
                };
                Ok(Token::new(token_type, loc))
            }
            
            Some(ch) if ch.is_ascii_digit() => {
                let token_type = self.read_number()?;
                Ok(Token::new(token_type, loc))
            }
            
            Some('-') if self.peek_ahead(1).map_or(false, |c| c.is_ascii_digit()) => {
                let token_type = self.read_number()?;
                Ok(Token::new(token_type, loc))
            }
            
            Some('"') => {
                let string = self.read_string()?;
                Ok(Token::new(TokenType::StringLiteral(string), loc))
            }
            
            Some(':') => {
                self.advance();
                if self.peek() == Some(':') {
                    self.advance();
                    Ok(Token::new(TokenType::DoubleColon, loc))
                } else {
                    Ok(Token::new(TokenType::Colon, loc))
                }
            }
            
            Some('.') => {
                self.advance();
                if self.peek() == Some('.') {
                    self.advance();
                    Ok(Token::new(TokenType::DoubleDot, loc))
                } else {
                    Ok(Token::new(TokenType::Dot, loc))
                }
            }
            
            Some(';') => { self.advance(); Ok(Token::new(TokenType::Semicolon, loc)) }
            Some(',') => { self.advance(); Ok(Token::new(TokenType::Comma, loc)) }
            Some('(') => { self.advance(); Ok(Token::new(TokenType::LeftParen, loc)) }
            Some(')') => { self.advance(); Ok(Token::new(TokenType::RightParen, loc)) }
            Some('[') => { self.advance(); Ok(Token::new(TokenType::LeftBracket, loc)) }
            Some(']') => { self.advance(); Ok(Token::new(TokenType::RightBracket, loc)) }
            Some('{') => { self.advance(); Ok(Token::new(TokenType::LeftBrace, loc)) }
            Some('}') => { self.advance(); Ok(Token::new(TokenType::RightBrace, loc)) }
            Some('=') => { self.advance(); Ok(Token::new(TokenType::Equals, loc)) }
            
            Some(ch) => {
                Err(FlatZincError::LexError {
                    message: format!("Unexpected character: '{}'", ch),
                    line: loc.line,
                    column: loc.column,
                })
            }
        }
    }
}

/// Tokenize a FlatZinc source string
pub fn tokenize(input: &str) -> FlatZincResult<Vec<Token>> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = tokenizer.next_token()?;
        let is_eof = matches!(token.token_type, TokenType::Eof);
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_keywords() {
        let input = "var int bool predicate constraint solve satisfy";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 8); // 7 keywords + EOF
        assert!(matches!(tokens[0].token_type, TokenType::Var));
        assert!(matches!(tokens[1].token_type, TokenType::Int));
        assert!(matches!(tokens[2].token_type, TokenType::Bool));
    }
    
    #[test]
    fn test_tokenize_numbers() {
        let input = "42 -17 3.14 -2.5 1e10";
        let tokens = tokenize(input).unwrap();
        assert!(matches!(tokens[0].token_type, TokenType::IntLiteral(42)));
        assert!(matches!(tokens[1].token_type, TokenType::IntLiteral(-17)));
        assert!(matches!(tokens[2].token_type, TokenType::FloatLiteral(_)));
    }
    
    #[test]
    fn test_tokenize_identifiers() {
        let input = "x y_1 foo_bar INT____00001";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 5); // 4 identifiers + EOF
    }
    
    #[test]
    fn test_tokenize_comment() {
        let input = "var x; % this is a comment\nvar y;";
        let tokens = tokenize(input).unwrap();
        // Should skip comment
        assert!(matches!(tokens[0].token_type, TokenType::Var));
        assert!(matches!(tokens[2].token_type, TokenType::Semicolon));
        assert!(matches!(tokens[3].token_type, TokenType::Var));
    }
}
