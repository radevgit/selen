//! Example: Simple FlatZinc test

use selen::prelude::*;

fn main() {
    // Test 1: Simple variable declaration
    println!("Test 1: Simple variable declaration");
    {
        let fzn = r#"
            var 1..10: x;
            var 1..10: y;
            solve satisfy;
        "#;

        let mut model = Model::default();
        match model.from_flatzinc_str(fzn) {
            Ok(_) => println!("✓ Parsed successfully"),
            Err(e) => println!("✗ Parse error: {}", e),
        }
    }
    
    // Test 2: Simple constraint
    println!("\nTest 2: Simple constraint with int_eq");
    {
        let fzn = r#"
            var 1..3: x;
            var 1..3: y;
            constraint int_eq(x, 1);
            solve satisfy;
        "#;

        let mut model = Model::default();
        match model.from_flatzinc_str(fzn) {
            Ok(_) => {
                println!("✓ Parsed successfully");
                match model.solve() {
                    Ok(_) => println!("✓ Solved successfully"),
                    Err(e) => println!("✗ Solve error: {:?}", e),
                }
            }
            Err(e) => println!("✗ Parse error: {}", e),
        }
    }
    
    // Test 3: all_different
    println!("\nTest 3: all_different constraint");
    {
        let fzn = r#"
            var 1..3: x;
            var 1..3: y;
            var 1..3: z;
            constraint all_different([x, y, z]);
            solve satisfy;
        "#;

        let mut model = Model::default();
        match model.from_flatzinc_str(fzn) {
            Ok(_) => {
                println!("✓ Parsed successfully");
                match model.solve() {
                    Ok(_) => println!("✓ Solved successfully"),
                    Err(e) => println!("✗ Solve error: {:?}", e),
                }
            }
            Err(e) => println!("✗ Parse error: {}", e),
        }
    }
}
