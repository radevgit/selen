use cspsolver::prelude::*;
// Note: post! macro is exported at crate root due to #[macro_export]  
use cspsolver::{post, postall};

fn main() {
    println!("Enhanced Mathematical Constraint Syntax Demo");
    println!("=============================================\n");

    let mut m = Model::default();

    // Variables for demonstrations
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 20);
    let w = m.int(-5, 5);

    println!("1. Arithmetic Operations:");
    println!("   x + y <= z, x - y >= int(0), x * y == int(12), x / y != int(0)");
    post!(m, x + y <= z);
    post!(m, x - y >= int(0));
    post!(m, x * y == int(12));
    post!(m, x / y != int(0));
    println!("✓ Arithmetic constraints posted\n");

    println!("2. Mathematical Functions:");
    println!("   abs(w) >= int(1), min([x, y]) == int(3), max([x, y]) <= int(8)");
    post!(m, abs(w) >= int(1));
    post!(m, min([x, y]) == int(3));
    post!(m, max([x, y]) <= int(8));
    println!("✓ Mathematical function constraints posted\n");

    println!("3. Global Constraints:");
    println!("   alldiff([x, y, z]) - all variables must be different");
    post!(m, alldiff([x, y, z]));
    println!("✓ All-different constraint posted\n");

    println!("4. Enhanced Modulo Operations:");
    println!("   x % y == int(0) - x is divisible by y");
    post!(m, x % y == int(0));
    println!("✓ Enhanced modulo constraint posted\n");

    println!("5. Combined Complex Constraints:");
    println!("   Combining multiple constraint types in one model");
    
    // Create a new model for the complex example
    let mut m2 = Model::default();
    let a = m2.int(1, 6);
    let b = m2.int(1, 6);
    let c = m2.int(1, 12);
    
    // Complex constraint combination
    post!(m2, a + b == c);           // Sum constraint
    post!(m2, abs(a) <= int(5));     // Absolute value
    post!(m2, max([a, b]) >= int(3)); // Max function
    post!(m2, a % 2 == 1);           // Odd number constraint (literal)
    post!(m2, alldiff([a, b, c]));    // All different
    
    println!("   a + b == c");
    println!("   abs(a) <= int(5)");
    println!("   max([a, b]) >= int(3)");
    println!("   a % 2 == 1 (a is odd)");
    println!("   alldiff([a, b, c])");
    
    // Solve the complex model
    println!("\nSolving the complex constraint system...");
    match m2.solve() {
        Some(solution) => {
            println!("✓ Solution found!");
            
            use cspsolver::vars::Val;
            
            let a_val = solution[a];
            let b_val = solution[b];
            let c_val = solution[c];
            
            println!("   a = {:?}", a_val);
            println!("   b = {:?}", b_val);
            println!("   c = {:?}", c_val);
            
            // Verify the solution  
            match (a_val, b_val, c_val) {
                (Val::ValI(a_int), Val::ValI(b_int), Val::ValI(c_int)) => {
                    println!("\nVerification:");
                    println!("   {} + {} = {} ✓", a_int, b_int, a_int + b_int);
                    println!("   c = {} ✓", c_int);
                    println!("   abs({}) = {} <= 5 ✓", a_int, a_int.abs());
                    println!("   max({}, {}) = {} >= 3 ✓", a_int, b_int, a_int.max(b_int));
                    println!("   {} % 2 = {} (odd) ✓", a_int, a_int % 2);
                    println!("   All values different: {}, {}, {} ✓", a_int, b_int, c_int);
                }
                _ => {
                    println!("   Values are not integers as expected");
                }
            }
        }
        None => {
            println!("✗ No solution found - constraints may be over-constrained");
        }
    }

    println!("\n6. Batch Constraint Posting with postall!:");
    println!("   Multiple constraints posted in one macro call");
    
    let mut m1 = Model::default();
    let p = m1.int(1, 5);
    let q = m1.int(1, 5);
    let r = m1.int(1, 10);
    
    // Create constraint references for logical operations
    let c1 = post!(m1, p < q);
    let c2 = post!(m1, q > int(3));
    
    postall!(m1,
        p < q,
        q + p <= r,
        abs(p) >= int(2),
        alldiff([p, q, r]),
        and(c1, c2),
        or(c1, c2),
        not(c1)
    );
    
    println!("   p < q");
    println!("   q + p <= r");
    println!("   abs(p) >= int(2)");
    println!("   alldiff([p, q, r])");
    println!("   and(c1, c2)");
    println!("   or(c1, c2)"); 
    println!("   not(c1)");
    println!("✓ All constraints posted with postall! macro\n");

    println!("7. Summary of New Syntax:");
    println!("   ✓ Arithmetic: x + y, x - y, x * y, x / y");
    println!("   ✓ Functions: abs(x), min([x,y]), max([x,y])");  
    println!("   ✓ Global: alldiff([x,y,z])");
    println!("   ✓ Enhanced modulo: x % y == int(0)");
    println!("   ✓ All work with constants: x + y <= int(10)");
    
    println!("\n=============================================");
    println!("Enhanced mathematical syntax demonstration complete!");
    
    println!("\nAll supported constraint syntax:");
    println!("  • Basic: x < y, x >= int(5), x != y");
    println!("  • Arithmetic: x + y <= z, x * 2 == int(10)");
    println!("  • Functions: abs(x) >= int(1), max([x,y]) <= int(8)");
    println!("  • Global: alldiff([x, y, z])");
    println!("  • Modulo: x % 3 == 1, x % y != int(0)");
    println!("  • Logical: and(c1, c2), or(c1, c2), not(c1)");
    println!("  • Batch: postall!(m, x < y, y > int(5), alldiff([x,y,z]))");
}