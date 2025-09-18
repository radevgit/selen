use cspsolver::prelude::*;

fn main() {
    println!("Testing AllEqual constraint...");
    
    // Test case 1: Simple AllEqual with integer variables
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(5, 15);
    let z = model.int(3, 8);
    
    // All variables must be equal
    post!(model, allequal([x, y, z]));
    
    match model.solve() {
        Ok(solution) => {
            let x_val = solution[x];
            let y_val = solution[y];
            let z_val = solution[z];
            println!("Solution found:");
            println!("  x = {:?}", x_val);
            println!("  y = {:?}", y_val);
            println!("  z = {:?}", z_val);
            
            // Verify they are all equal
            if x_val == y_val && y_val == z_val {
                println!("✓ All variables are equal as expected");
            } else {
                println!("✗ Variables are not equal - constraint failed!");
            }
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    
    // Test case 2: AllEqual with impossible constraints
    println!("\nTesting impossible AllEqual constraint...");
    let mut model2 = Model::default();
    let a = model2.int(1, 5);
    let b = model2.int(10, 15);
    
    post!(model2, allequal([a, b])); // Should be unsatisfiable
    
    match model2.solve() {
        Ok(_) => {
            println!("✗ Unexpected solution found for impossible constraint");
        }
        Err(_) => {
            println!("✓ No solution found as expected (domains don't overlap)");
        }
    }
    
    // Test case 3: AllEqual with floating point variables
    println!("\nTesting AllEqual with floating point variables...");
    let mut model3 = Model::default();
    let f1 = model3.float(1.0, 10.0);
    let f2 = model3.float(5.0, 15.0);
    let f3 = model3.float(3.0, 8.0);
    
    post!(model3, allequal([f1, f2, f3]));
    
    match model3.solve() {
        Ok(solution) => {
            let f1_val = solution[f1];
            let f2_val = solution[f2];
            let f3_val = solution[f3];
            println!("Floating point solution:");
            println!("  f1 = {:?}", f1_val);
            println!("  f2 = {:?}", f2_val);
            println!("  f3 = {:?}", f3_val);
        }
        Err(_) => {
            println!("No floating point solution found");
        }
    }
}