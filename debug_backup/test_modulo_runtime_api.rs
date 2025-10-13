use selen::prelude::*;

fn main() {
    // Test 1: Basic modulo constraint
    println!("Test 1: Basic modulo - x % 3 == 1, x in 0..10");
    let mut m = Model::default();
    let x = m.int(0, 10);
    let divisor = m.int(3, 3); // Constant 3 as a variable
    
    m.new(x.modulo(divisor).eq(1));
    
    match m.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            println!("  Solution: x = {}", x_val);
            println!("  Verification: {} % 3 = {}", x_val, x_val % 3);
            assert_eq!(x_val % 3, 1, "Modulo constraint not satisfied!");
        }
        Err(e) => {
            println!("  Error: {:?}", e);
            panic!("Should have found a solution!");
        }
    }
    
    // Test 2: Modulo with variable bounds
    println!("\nTest 2: x % 4 == 2, x in 0..15");
    let mut m = Model::default();
    let x = m.int(0, 15);
    let divisor = m.int(4, 4);
    
    m.new(x.modulo(divisor).eq(2));
    
    match m.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            println!("  Solution: x = {} (check: {} % 4 = {})", 
                     x_val, x_val, x_val % 4);
            assert_eq!(x_val % 4, 2);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
            panic!("Should have found a solution!");
        }
    }
    
    // Test 3: Modulo in more complex expression
    println!("\nTest 3: Complex - (x % 5) + y == 7, x in 0..20, y in 0..10");
    let mut m = Model::default();
    let x = m.int(0, 20);
    let y = m.int(0, 10);
    let divisor = m.int(5, 5);
    
    m.new(x.modulo(divisor).add(y).eq(7));
    
    match m.solve() {
        Ok(solution) => {
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            println!("  Solution: x = {}, y = {}", x_val, y_val);
            println!("  Verification: ({} % 5) + {} = {}", 
                     x_val, y_val, (x_val % 5) + y_val);
            assert_eq!((x_val % 5) + y_val, 7, "Constraint not satisfied!");
        }
        Err(e) => {
            println!("  Error: {:?}", e);
            panic!("Should have found a solution!");
        }
    }
    
    println!("\n✅ All modulo tests passed!");
}
