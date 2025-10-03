//! Debug FlatZinc parsing

use selen::prelude::*;

fn main() {
    let fzn = r#"
        var 1..3: x;
        var 1..3: y;
        constraint int_eq(x, 1);
        solve satisfy;
    "#;

    let mut model = Model::default();
    let result = model.from_flatzinc_str(fzn);
    
    match result {
        Ok(_) => {
            println!("✓ FlatZinc parsed successfully!");
            match model.solve() {
                Ok(_) => println!("✓ Solution found!"),
                Err(e) => println!("✗ Solve failed: {:?}", e),
            }
        }
        Err(e) => {
            println!("✗ FlatZinc parsing failed: {:?}", e);
        }
    }
}
