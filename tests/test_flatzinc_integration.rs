//! Integration tests for FlatZinc parser and model import

use selen::prelude::*;

#[cfg(test)]
mod flatzinc_integration {
    use super::*;

    #[test]
    fn test_simple_variable_declaration() {
        let fzn = r#"
            var 1..10: x;
            var 1..10: y;
            solve satisfy;
        "#;

        let mut model = Model::default();
        let result = model.from_flatzinc_str(fzn);
        assert!(result.is_ok(), "Failed to parse simple FlatZinc: {:?}", result);
    }

    #[test]
    fn test_simple_constraint() {
        // Test int_eq with variable and literal
        let fzn = r#"
var 1..10: x;
constraint int_eq(x, x);
solve satisfy;
"#;

        let mut model = Model::default();
        model.from_flatzinc_str(fzn).expect("Should parse variable-to-variable equality");
        assert!(model.solve().is_ok());
        
        // Test int_ne with two variables
        let fzn2 = r#"
var 1..5: a;
var 1..5: b;
constraint int_ne(a, b);
solve satisfy;
"#;
        let mut model2 = Model::default();
        model2.from_flatzinc_str(fzn2).expect("Should parse int_ne");
        assert!(model2.solve().is_ok());
    }

    #[test]
    fn test_alldiff_constraint() {
        let fzn = r#"
            var 1..3: x;
            var 1..3: y;
            var 1..3: z;
            constraint all_different([x, y, z]);
            solve satisfy;
        "#;

        let mut model = Model::default();
        model.from_flatzinc_str(fzn).unwrap();
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Should find a solution with all_different");
    }

    #[test]
    fn test_linear_eq_constraint() {
        let fzn = r#"
            var 1..10: x;
            var 1..10: y;
            constraint int_lin_eq([1, 1], [x, y], 10);
            solve satisfy;
        "#;

        let mut model = Model::default();
        model.from_flatzinc_str(fzn).unwrap();
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Should find a solution for x + y = 10");
    }

    #[test]
    fn test_linear_ne_constraint() {
        let fzn = r#"
            var 1..10: x;
            var 1..10: y;
            constraint int_lin_ne([1, 1], [x, y], 10);
            solve satisfy;
        "#;

        let mut model = Model::default();
        model.from_flatzinc_str(fzn).unwrap();
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Should find a solution for x + y ≠ 10");
    }

    #[test]
    fn test_reification_constraint() {
        let fzn = r#"
            var 1..10: x;
            var 1..10: y;
            var bool: b;
            constraint int_eq_reif(x, y, b);
            solve satisfy;
        "#;

        let mut model = Model::default();
        model.from_flatzinc_str(fzn).unwrap();
        
        let solution = model.solve();
        assert!(solution.is_ok(), "Should find a solution with reification");
    }

    #[test]
    fn test_from_file() {
        use std::fs::File;
        use std::io::Write;
        
        let fzn = r#"
            var 1..5: x;
            var 1..5: y;
            constraint int_lt(x, y);
            solve satisfy;
        "#;

        // Create a temporary file
        let temp_path = "/tmp/test_flatzinc.fzn";
        let mut file = File::create(temp_path).unwrap();
        file.write_all(fzn.as_bytes()).unwrap();
        file.sync_all().unwrap();
        drop(file);

        // Test from_flatzinc_file
        let mut model = Model::default();
        let result = model.from_flatzinc_file(temp_path);
        assert!(result.is_ok(), "Failed to load FlatZinc from file: {:?}", result);

        // Clean up
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_parse_error_reporting() {
        let fzn = r#"
            var 1..10 x;  % Missing colon
            solve satisfy;
        "#;

        let mut model = Model::default();
        let result = model.from_flatzinc_str(fzn);
        assert!(result.is_err(), "Should fail on invalid syntax");
        
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("line") || error_msg.contains("column"), 
                    "Error should include location info: {}", error_msg);
        }
    }

    #[test]
    fn test_unsupported_constraint() {
        let fzn = r#"
            var 1..10: x;
            constraint some_unsupported_constraint(x);
            solve satisfy;
        "#;

        let mut model = Model::default();
        let result = model.from_flatzinc_str(fzn);
        assert!(result.is_err(), "Should fail on unsupported constraint");
    }
}
