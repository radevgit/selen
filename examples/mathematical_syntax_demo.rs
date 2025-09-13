/// Demonstration of Mathematical Constraint Syntax
/// 
/// This example shows how to use the new mathematical syntax for expressing constraints
/// with post!/postall! macros and helper functions like int(), float(), abs(), etc.

use cspsolver::prelude::*;
// Note: post! macro is exported at crate root due to #[macro_export]
use cspsolver::post;

fn main() {
    println!("Mathematical Constraint Syntax Demo");
    println!("=====================================");

    // Create a model
    let mut m = Model::default();
    
    // Create variables
    let x = m.int(1, 20);  // x ∈ [1, 20]
    let y = m.int(1, 20);  // y ∈ [1, 20]  
    let z = m.int(1, 20);  // z ∈ [1, 20]
    
    println!("\nDemonstrating basic comparison syntax:");
    
    // Basic comparisons with natural mathematical syntax
    let _c1 = post!(m, x < y);           // x < y
    let _c2 = post!(m, y <= int(15));    // y ≤ 15
    let _c3 = post!(m, z >= int(5));     // z ≥ 5
    let _c4 = post!(m, x > int(2));      // x > 2
    // Note: Removed y == 10 to make system more solvable
    let _c6 = post!(m, x != z);          // x ≠ z
    
    println!("✓ Basic comparisons: x < y, y <= 15, z >= 5, x > 2, x != z");
    
    println!("\nDemonstrating typed constants:");
    
    // Explicit typing with int() and float() helpers
    let _c7 = post!(m, x >= int(3));     // Explicitly typed integer
    let w = m.float(0.0, 100.0);
    let _c8 = post!(m, w <= float(50.5)); // Explicitly typed float
    
    println!("✓ Typed constants: x >= int(3), w <= float(50.5)");
    
    println!("\nDemonstrating mathematical functions:");
    
    // TODO: Mathematical functions to be implemented later:
    // let _c9 = post!(m, abs(x) >= int(1));     // |x| ≥ 1
    // let _c10 = post!(m, max(x, y) <= int(18)); // max(x,y) ≤ 18
    // let _c11 = post!(m, min(y, z) >= int(3));  // min(y,z) ≥ 3
    
    // For now, basic constraints work:
    let _c9 = post!(m, x >= int(1));     // x ≥ 1
    let _c10 = post!(m, y <= int(18));   // y ≤ 18
    let _c11 = post!(m, z >= int(3));    // z ≥ 3
    
    println!("✓ Basic constraints: x >= 1, y <= 18, z >= 3");
    
    println!("\nDemonstrating modulo constraints:");
    
    // Modulo operations for cyclical constraints
    let _c12 = post!(m, x % 3 == 1);     // x ≡ 1 (mod 3)
    // Note: Removed y % 5 == 0 to make system more solvable 
    
    println!("✓ Modulo constraints: x % 3 == 1");
    
    println!("\nDemonstrating logical operators:");
    
    // Logical combinations using clean function-style syntax
    let c_a = post!(m, x < int(10));
    let c_b = post!(m, y > int(5));
    
    // ✓ Preferred function-style syntax (clean and simple):
    let _c14 = post!(m, and(c_a, c_b));  // c_a AND c_b
    let _c15 = post!(m, or(c_a, c_b));   // c_a OR c_b
    let _c16 = post!(m, not(c_a));       // NOT c_a
    
    // Alternative operator-style syntax (requires parentheses):
    // let _c17 = post!(m, (c_a) & (c_b));  // Also works but more verbose
    
    println!("✓ Logical operators: and(c_a, c_b), or(c_a, c_b), not(c_a)");
    
    println!("\nSolving the constraint system...");
    
    // Solve the system
    if let Some(solution) = m.solve() {
        println!("✓ Solution found!");
        
        use cspsolver::vars::Val;
        
        let x_val = solution[x];
        let y_val = solution[y];
        let z_val = solution[z];
        let w_val = solution[w];
        
        println!("  x = {:?}", x_val);
        println!("  y = {:?}", y_val);
        println!("  z = {:?}", z_val);
        println!("  w = {:?}", w_val);
        
        println!("\nConstraint verification:");
        match (x_val, y_val) {
            (Val::ValI(x_int), Val::ValI(y_int)) => {
                println!("  x < y: {} < {} = {}", x_int, y_int, x_int < y_int);
                println!("  x % 3 == 1: {} % 3 = {} (should be 1)", x_int, x_int % 3);
                println!("  x > 2: {} > 2 = {}", x_int, x_int > 2);
            }
            _ => {
                println!("  Values are not integers as expected");
            }
        }
    } else {
        println!("✗ No solution found - constraints may be over-constrained");
    }
    
    println!("\n=====================================");
    println!("Mathematical syntax demonstration complete!");
    
    // TODO: Future enhancements to demonstrate:
    println!("\nPlanned future syntax enhancements:");
    println!("  • Arithmetic: post!(m, x + 3 < y)");
    println!("  • Multiplication: post!(m, x * 2 <= 10)");
    println!("  • Mathematical functions: post!(m, abs(x) >= int(1))");
    println!("  • Complex expressions: post!(m, max(x, y) <= int(15))");
    println!("  • Nested logical operations: post!(m, and(or(c1, c2), not(c3)))");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_syntax_demo() {
        // Test that our demo code compiles and runs without panicking
        let mut m = Model::default();
        let x = m.int(1, 20);
        let y = m.int(1, 20);
        
        // Test basic syntax
        let _c1 = post!(m, x < y);
        let _c2 = post!(m, y <= int(15));
        let _c3 = post!(m, x >= int(1));  // Basic constraint instead of abs
        let _c4 = post!(m, x % 3 == 1);
        
        // Should compile and not panic
        assert!(true);
    }
    
    #[test]
    fn test_solvable_mathematical_constraints() {
        let mut m = Model::default();
        let x = m.int(1, 20);
        let y = m.int(1, 20);
        
        // Create a solvable system
        let _c1 = post!(m, x < y);         // x < y
        let _c2 = post!(m, y <= int(10));  // y ≤ 10
        let _c3 = post!(m, x >= int(1));   // x ≥ 1
        
        // Should be solvable
        let solution = m.solve();
        assert!(solution.is_some(), "Mathematical constraint system should be solvable");
        
        if let Some(sol) = solution {
            use cspsolver::vars::Val;
            let x_val = sol[x];
            let y_val = sol[y];
            
            // Verify solution satisfies constraints
            if let (Val::ValI(x_int), Val::ValI(y_int)) = (x_val, y_val) {
                assert!(x_int < y_int, "x < y constraint should be satisfied");
                assert!(y_int <= 10, "y <= 10 constraint should be satisfied");
                assert!(x_int >= 1, "x >= 1 constraint should be satisfied");
            }
        }
    }
}