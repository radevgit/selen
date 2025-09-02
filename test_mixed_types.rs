use cspsolver::prelude::*;

fn main() {
    println!("Testing mixed type Val comparisons:");
    
    let int_val = Val::int(5);
    let float_val = Val::float(5.0);
    let close_float = Val::float(5.0000001);  // Within epsilon
    let far_float = Val::float(5.1);          // Outside epsilon
    
    println!("int(5) == float(5.0): {}", int_val == float_val);
    println!("int(5) == float(5.0000001): {}", int_val == close_float);
    println!("int(5) == float(5.1): {}", int_val == far_float);
    
    println!("float(5.0) == int(5): {}", float_val == int_val);
    println!("float(5.0000001) == int(5): {}", close_float == int_val);
    println!("float(5.1) == int(5): {}", far_float == int_val);
    
    // Test the enhanced not-equals constraint with mixed types
    let mut model = Model::new();
    let x = model.int_var(1, 10);
    let y = model.float_var(1.0, 10.0);
    
    // Add constraint that x != y
    model.not_equals(x, y);
    
    // Set x = 5
    model.equals(x, Val::int(5));
    
    // Try to set y = 5.0 (should fail due to not-equals constraint)
    model.equals(y, Val::float(5.0));
    
    let solver = model.build();
    match solver.solve() {
        Some(solution) => {
            println!("Found solution (unexpected): x={:?}, y={:?}", 
                     solution.get(x), solution.get(y));
        }
        None => {
            println!("No solution found (expected - mixed type not-equals working)");
        }
    }
}
