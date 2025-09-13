use cspsolver::prelude::*;
use cspsolver::optimization::classification::ProblemClassifier;

fn main() {
    println!("Problem Classification Demo");
    println!("===========================");

    // Test 1: Pure float problem
    let mut m1 = Model::default();
    let x = m1.float(1.0, 10.0);
    let y = m1.float(2.0, 20.0);
    post!(m1, x < y);
    
    let problem_type1 = ProblemClassifier::classify(m1.get_vars(), m1.get_props());
    println!("Pure float problem: {:?}", problem_type1);
    println!("Strategy: {}", problem_type1.strategy_description());
    println!();

    // Test 2: Pure integer problem  
    let mut m2 = Model::default();
    let a = m2.int(1, 10);
    let b = m2.int(5, 15);
    post!(m2, a != b);
    
    let problem_type2 = ProblemClassifier::classify(m2.get_vars(), m2.get_props());
    println!("Pure integer problem: {:?}", problem_type2);
    println!("Strategy: {}", problem_type2.strategy_description());
    println!();

    // Test 3: Mixed problem with constraints (should detect coupling)
    let mut m3 = Model::default();
    let int_var = m3.int(1, 5);
    let float_var = m3.float(1.0, 10.0);
    post!(m3, int_var == float_var); // This creates coupling
    
    let problem_type3 = ProblemClassifier::classify(m3.get_vars(), m3.get_props());
    println!("Mixed problem with coupling: {:?}", problem_type3);
    println!("Strategy: {}", problem_type3.strategy_description());
    println!();

    // Test 4: Mixed problem without cross-type constraints (should be separable)
    let mut m4 = Model::default();
    let int_var1 = m4.int(1, 5);
    let int_var2 = m4.int(3, 8);
    let float_var1 = m4.float(1.0, 10.0);
    let float_var2 = m4.float(5.0, 15.0);
    
    // Add constraints within each type only
    post!(m4, int_var1 != int_var2);
    post!(m4, float_var1 < float_var2);
    
    let problem_type4 = ProblemClassifier::classify(m4.get_vars(), m4.get_props());
    println!("Mixed problem without coupling: {:?}", problem_type4);
    println!("Strategy: {}", problem_type4.strategy_description());
}
