use selen::prelude::*;

fn main() {
    eprintln!("Starting two-variable test...");
    let mut model = Model::default();
    let x = model.float(-1e6, 1e6);
    let y = model.float(-1e6, 1e6);
    let sum_var = model.float(-2e6, 2e6);
    
    model.new(x.add(y).eq(sum_var));
    model.new(sum_var.le(0.0));
    
    eprintln!("About to call maximize...");
    let result = model.maximize(x);
    eprintln!("Maximize returned!");
    
    match result {
        Ok(sol) => {
            eprintln!("Success:");
            eprintln!("  x = {}", sol.get_float(x));
            eprintln!("  y = {}", sol.get_float(y));
            eprintln!("  sum = {}", sol.get_float(sum_var));
        },
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
