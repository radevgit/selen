//! FlatZinc Parser
//!
//! Recursive-descent parser that converts tokens into an AST.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::tokenizer::{Token, TokenType, Location};

/// Parser state
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }
    
    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }
    
    fn peek(&self) -> &TokenType {
        &self.current().token_type
    }
    
    fn location(&self) -> Location {
        self.current().location
    }
    
    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        if !matches!(token.token_type, TokenType::Eof) {
            self.position += 1;
        }
        token
    }
    
    fn expect(&mut self, expected: TokenType) -> FlatZincResult<()> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            let loc = self.location();
            Err(FlatZincError::ParseError {
                message: format!("Expected {:?}, found {:?}", expected, self.peek()),
                line: loc.line,
                column: loc.column,
            })
        }
    }
    
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// Parse the entire FlatZinc model
    pub fn parse_model(&mut self) -> FlatZincResult<FlatZincModel> {
        let mut model = FlatZincModel::new();
        
        while !matches!(self.peek(), TokenType::Eof) {
            match self.peek() {
                TokenType::Predicate => {
                    model.predicates.push(self.parse_predicate()?);
                }
                TokenType::Var | TokenType::Array | TokenType::Bool | TokenType::Int | TokenType::Float => {
                    model.var_decls.push(self.parse_var_decl()?);
                }
                TokenType::Constraint => {
                    model.constraints.push(self.parse_constraint()?);
                }
                TokenType::Solve => {
                    model.solve_goal = self.parse_solve()?;
                }
                _ => {
                    let loc = self.location();
                    return Err(FlatZincError::ParseError {
                        message: format!("Unexpected token: {:?}", self.peek()),
                        line: loc.line,
                        column: loc.column,
                    });
                }
            }
        }
        
        Ok(model)
    }
    
    fn parse_predicate(&mut self) -> FlatZincResult<PredicateDecl> {
        let loc = self.location();
        self.expect(TokenType::Predicate)?;
        
        let name = if let TokenType::Identifier(s) = self.peek() {
            let n = s.clone();
            self.advance();
            n
        } else {
            return Err(FlatZincError::ParseError {
                message: "Expected predicate name".to_string(),
                line: loc.line,
                column: loc.column,
            });
        };
        
        self.expect(TokenType::LeftParen)?;
        let params = self.parse_pred_params()?;
        self.expect(TokenType::RightParen)?;
        self.expect(TokenType::Semicolon)?;
        
        Ok(PredicateDecl { name, params, location: loc })
    }
    
    fn parse_pred_params(&mut self) -> FlatZincResult<Vec<PredParam>> {
        let mut params = Vec::new();
        
        if matches!(self.peek(), TokenType::RightParen) {
            return Ok(params);
        }
        
        loop {
            let param_type = self.parse_type()?;
            self.expect(TokenType::Colon)?;
            
            let name = if let TokenType::Identifier(s) = self.peek() {
                let n = s.clone();
                self.advance();
                n
            } else {
                let loc = self.location();
                return Err(FlatZincError::ParseError {
                    message: "Expected parameter name".to_string(),
                    line: loc.line,
                    column: loc.column,
                });
            };
            
            params.push(PredParam { param_type, name });
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        Ok(params)
    }
    
    fn parse_var_decl(&mut self) -> FlatZincResult<VarDecl> {
        let loc = self.location();
        let var_type = self.parse_type()?;
        self.expect(TokenType::Colon)?;
        
        let name = if let TokenType::Identifier(s) = self.peek() {
            let n = s.clone();
            self.advance();
            n
        } else {
            return Err(FlatZincError::ParseError {
                message: "Expected variable name".to_string(),
                line: loc.line,
                column: loc.column,
            });
        };
        
        let annotations = self.parse_annotations()?;
        
        let init_value = if self.match_token(&TokenType::Equals) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        self.expect(TokenType::Semicolon)?;
        
        Ok(VarDecl {
            var_type,
            name,
            annotations,
            init_value,
            location: loc,
        })
    }
    
    fn parse_type(&mut self) -> FlatZincResult<Type> {
        // Handle 'var' prefix
        let is_var = self.match_token(&TokenType::Var);
        
        let base_type = match self.peek() {
            TokenType::Bool => {
                self.advance();
                Type::Bool
            }
            TokenType::Int => {
                self.advance();
                Type::Int
            }
            TokenType::Float => {
                self.advance();
                Type::Float
            }
            TokenType::IntLiteral(min) => {
                let min_val = *min;
                self.advance();
                self.expect(TokenType::DoubleDot)?;
                if let TokenType::IntLiteral(max) = self.peek() {
                    let max_val = *max;
                    self.advance();
                    Type::IntRange(min_val, max_val)
                } else {
                    let loc = self.location();
                    return Err(FlatZincError::ParseError {
                        message: "Expected integer for range upper bound".to_string(),
                        line: loc.line,
                        column: loc.column,
                    });
                }
            }
            TokenType::LeftBrace => {
                self.advance();
                let values = self.parse_int_set()?;
                self.expect(TokenType::RightBrace)?;
                Type::IntSet(values)
            }
            TokenType::Set => {
                self.advance();
                self.expect(TokenType::Of)?;
                // TODO: Handle set types more completely
                self.expect(TokenType::Int)?;
                Type::SetOfInt
            }
            TokenType::Array => {
                self.advance();
                self.expect(TokenType::LeftBracket)?;
                let index_sets = self.parse_index_sets()?;
                self.expect(TokenType::RightBracket)?;
                self.expect(TokenType::Of)?;
                let element_type = Box::new(self.parse_type()?);
                Type::Array { index_sets, element_type }
            }
            _ => {
                let loc = self.location();
                return Err(FlatZincError::ParseError {
                    message: format!("Expected type, found {:?}", self.peek()),
                    line: loc.line,
                    column: loc.column,
                });
            }
        };
        
        if is_var {
            Ok(Type::Var(Box::new(base_type)))
        } else {
            Ok(base_type)
        }
    }
    
    fn parse_int_set(&mut self) -> FlatZincResult<Vec<i64>> {
        let mut values = Vec::new();
        
        if matches!(self.peek(), TokenType::RightBrace) {
            return Ok(values);
        }
        
        loop {
            if let TokenType::IntLiteral(val) = self.peek() {
                values.push(*val);
                self.advance();
            } else {
                let loc = self.location();
                return Err(FlatZincError::ParseError {
                    message: "Expected integer in set".to_string(),
                    line: loc.line,
                    column: loc.column,
                });
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        Ok(values)
    }
    
    fn parse_index_sets(&mut self) -> FlatZincResult<Vec<IndexSet>> {
        let mut index_sets = Vec::new();
        
        loop {
            // Handle 'int' as index set type (for predicate declarations)
            if let TokenType::Int = self.peek() {
                self.advance();
                index_sets.push(IndexSet::Range(1, 1000000)); // Arbitrary large range for 'int'
            }
            // Handle numeric range like 1..8 OR single integer like [1] (meaning 1..1)
            else if let TokenType::IntLiteral(min) = self.peek() {
                let min_val = *min;
                self.advance();
                
                // Check if there's a range operator (..)
                if self.match_token(&TokenType::DoubleDot) {
                    // It's a range: min..max
                    if let TokenType::IntLiteral(max) = self.peek() {
                        let max_val = *max;
                        self.advance();
                        index_sets.push(IndexSet::Range(min_val, max_val));
                    } else {
                        let loc = self.location();
                        return Err(FlatZincError::ParseError {
                            message: "Expected integer for index range upper bound".to_string(),
                            line: loc.line,
                            column: loc.column,
                        });
                    }
                } else {
                    // It's a single integer: treat as range 1..min_val
                    // This handles array[1] or array[N] syntax
                    index_sets.push(IndexSet::Range(1, min_val));
                }
            } else {
                break;
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        Ok(index_sets)
    }
    
    fn parse_constraint(&mut self) -> FlatZincResult<Constraint> {
        let loc = self.location();
        self.expect(TokenType::Constraint)?;
        
        let predicate = if let TokenType::Identifier(s) = self.peek() {
            let n = s.clone();
            self.advance();
            n
        } else {
            return Err(FlatZincError::ParseError {
                message: "Expected constraint predicate name".to_string(),
                line: loc.line,
                column: loc.column,
            });
        };
        
        self.expect(TokenType::LeftParen)?;
        let args = self.parse_exprs()?;
        self.expect(TokenType::RightParen)?;
        
        let annotations = self.parse_annotations()?;
        self.expect(TokenType::Semicolon)?;
        
        Ok(Constraint {
            predicate,
            args,
            annotations,
            location: loc,
        })
    }
    
    fn parse_solve(&mut self) -> FlatZincResult<SolveGoal> {
        self.expect(TokenType::Solve)?;
        
        // Parse annotations that come before the goal (e.g., solve :: int_search(...) satisfy)
        let annotations = self.parse_annotations()?;
        
        let goal = match self.peek() {
            TokenType::Satisfy => {
                self.advance();
                SolveGoal::Satisfy { annotations }
            }
            TokenType::Minimize => {
                self.advance();
                let objective = self.parse_expr()?;
                SolveGoal::Minimize { objective, annotations }
            }
            TokenType::Maximize => {
                self.advance();
                let objective = self.parse_expr()?;
                SolveGoal::Maximize { objective, annotations }
            }
            _ => {
                let loc = self.location();
                return Err(FlatZincError::ParseError {
                    message: "Expected satisfy, minimize, or maximize".to_string(),
                    line: loc.line,
                    column: loc.column,
                });
            }
        };
        
        self.expect(TokenType::Semicolon)?;
        Ok(goal)
    }
    
    fn parse_annotations(&mut self) -> FlatZincResult<Vec<Annotation>> {
        let mut annotations = Vec::new();
        
        while self.match_token(&TokenType::DoubleColon) {
            if let TokenType::Identifier(name) = self.peek() {
                let ann_name = name.clone();
                self.advance();
                
                let args = if self.match_token(&TokenType::LeftParen) {
                    let exprs = self.parse_exprs()?;
                    self.expect(TokenType::RightParen)?;
                    exprs
                } else {
                    Vec::new()
                };
                
                annotations.push(Annotation { name: ann_name, args });
            }
        }
        
        Ok(annotations)
    }
    
    fn parse_exprs(&mut self) -> FlatZincResult<Vec<Expr>> {
        let mut exprs = Vec::new();
        
        if matches!(self.peek(), TokenType::RightParen) {
            return Ok(exprs);
        }
        
        loop {
            exprs.push(self.parse_expr()?);
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        Ok(exprs)
    }
    
    fn parse_expr(&mut self) -> FlatZincResult<Expr> {
        match self.peek() {
            TokenType::True => {
                self.advance();
                Ok(Expr::BoolLit(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::BoolLit(false))
            }
            TokenType::IntLiteral(val) => {
                let v = *val;
                self.advance();
                
                // Check for range
                if self.match_token(&TokenType::DoubleDot) {
                    if let TokenType::IntLiteral(max) = self.peek() {
                        let max_val = *max;
                        self.advance();
                        Ok(Expr::Range(Box::new(Expr::IntLit(v)), Box::new(Expr::IntLit(max_val))))
                    } else {
                        Ok(Expr::IntLit(v))
                    }
                } else {
                    Ok(Expr::IntLit(v))
                }
            }
            TokenType::FloatLiteral(val) => {
                let v = *val;
                self.advance();
                Ok(Expr::FloatLit(v))
            }
            TokenType::StringLiteral(s) => {
                let string = s.clone();
                self.advance();
                Ok(Expr::StringLit(string))
            }
            TokenType::Identifier(name) => {
                let id = name.clone();
                self.advance();
                
                // Check for array access
                if self.match_token(&TokenType::LeftBracket) {
                    let index = self.parse_expr()?;
                    self.expect(TokenType::RightBracket)?;
                    Ok(Expr::ArrayAccess {
                        array: Box::new(Expr::Ident(id)),
                        index: Box::new(index),
                    })
                } else {
                    Ok(Expr::Ident(id))
                }
            }
            TokenType::LeftBracket => {
                self.advance();
                let elements = self.parse_exprs()?;
                self.expect(TokenType::RightBracket)?;
                Ok(Expr::ArrayLit(elements))
            }
            TokenType::LeftBrace => {
                self.advance();
                let elements = self.parse_exprs()?;
                self.expect(TokenType::RightBrace)?;
                Ok(Expr::SetLit(elements))
            }
            _ => {
                let loc = self.location();
                Err(FlatZincError::ParseError {
                    message: format!("Unexpected token in expression: {:?}", self.peek()),
                    line: loc.line,
                    column: loc.column,
                })
            }
        }
    }
}

/// Parse a token stream into an AST
pub fn parse(tokens: Vec<Token>) -> FlatZincResult<FlatZincModel> {
    let mut parser = Parser::new(tokens);
    parser.parse_model()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flatzinc::tokenizer::tokenize;
    
    #[test]
    fn test_parse_simple_var() {
        let input = "var 1..10: x;\nsolve satisfy;";
        let tokens = tokenize(input).unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast.var_decls.len(), 1);
        assert_eq!(ast.var_decls[0].name, "x");
    }
    
    #[test]
    fn test_parse_constraint() {
        let input = "var 1..10: x;\nconstraint int_eq(x, 5);\nsolve satisfy;";
        let tokens = tokenize(input).unwrap();
        let ast = parse(tokens).unwrap();
        assert_eq!(ast.constraints.len(), 1);
        assert_eq!(ast.constraints[0].predicate, "int_eq");
    }
}
