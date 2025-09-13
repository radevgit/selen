use cspsolver::prelude::*;

fn main() {
    println!("CSP Solver - Model API with ints\n");

    // Create a simple constraint satisfaction problem
    // We have three variables with predefined domains and they must all be different
    let mut m = Model::default();

    // Variable 1: Can only be even numbers
    let var1 = m.ints(vec![2, 4, 6, 8]);
    
    // Variable 2: Can only be prime numbers  
    let var2 = m.ints(vec![2, 3, 5, 7]);
    
    // Variable 3: Can only be odd numbers
    let var3 = m.ints(vec![1, 3, 5, 7, 9]);

    // All variables must be different
    post!(m, alldiff([var1, var2, var3]));

    println!("Problem setup:");
    println!("  var1 ∈ {{2, 4, 6, 8}} (even numbers)");
    println!("  var2 ∈ {{2, 3, 5, 7}} (prime numbers)");  
    println!("  var3 ∈ {{1, 3, 5, 7, 9}} (odd numbers)");
    println!("  Constraint: all_different(var1, var2, var3)");

    // Solve the problem
    if let Some(solution) = m.solve() {
        println!("\nSolution found:");
        println!("  var1 = {:?}", solution[var1]);
        println!("  var2 = {:?}", solution[var2]);
        println!("  var3 = {:?}", solution[var3]);
        
        // Verify the solution
        let val1 = if let Val::ValI(v) = solution[var1] { v } else { 0 };
        let val2 = if let Val::ValI(v) = solution[var2] { v } else { 0 };
        let val3 = if let Val::ValI(v) = solution[var3] { v } else { 0 };
        
        println!("\nVerification:");
        println!("  var1 ({}) is even: {}", val1, val1 % 2 == 0);
        println!("  var2 ({}) is prime: {}", val2, is_prime(val2));
        println!("  var3 ({}) is odd: {}", val3, val3 % 2 == 1);
        println!("  All different: {}", val1 != val2 && val2 != val3 && val1 != val3);
    } else {
        println!("\nNo solution found!");
    }
}

fn is_prime(n: i32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt_n = (n as f32).sqrt() as i32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
