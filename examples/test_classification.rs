use cspsolver::prelude::*;
use cspsolver::optimization::classification::ProblemClassifier;

fn main() {
    println!("Testing problem classification...");

    // Test 1: Pure float problem
    let mut model1 = Model::default();
    let x = model1.new_var_float(1.0, 10.0);
    let y = model1.new_var_float(2.0, 20.0);
    model1.lt(x, y);
    
    let problem_type1 = ProblemClassifier::classify(model1.get_vars(), model1.get_props());
    println!("Pure float problem: {:?}", problem_type1);
    println!("Strategy: {}", problem_type1.strategy_description());
    println!();

    // Test 2: Pure integer problem  
    let mut model2 = Model::default();
    let a = model2.new_var_int(1, 10);
    let b = model2.new_var_int(5, 15);
    model2.ne(a, b);
    
    let problem_type2 = ProblemClassifier::classify(model2.get_vars(), model2.get_props());
    println!("Pure integer problem: {:?}", problem_type2);
    println!("Strategy: {}", problem_type2.strategy_description());
    println!();

    // Test 3: Mixed problem with constraints (should detect coupling)
    let mut model3 = Model::default();
    let int_var = model3.new_var_int(1, 5);
    let float_var = model3.new_var_float(1.0, 10.0);
    model3.equals(int_var, float_var); // This creates coupling
    
    let problem_type3 = ProblemClassifier::classify(model3.get_vars(), model3.get_props());
    println!("Mixed problem with coupling: {:?}", problem_type3);
    println!("Strategy: {}", problem_type3.strategy_description());
    println!();

    // Test 4: Mixed problem without cross-type constraints (should be separable)
    let mut model4 = Model::default();
    let int_var1 = model4.new_var_int(1, 5);
    let int_var2 = model4.new_var_int(3, 8);
    let float_var1 = model4.new_var_float(1.0, 10.0);
    let float_var2 = model4.new_var_float(5.0, 15.0);
    
    // Add constraints within each type only
    model4.ne(int_var1, int_var2);
    model4.lt(float_var1, float_var2);
    
    let problem_type4 = ProblemClassifier::classify(model4.get_vars(), model4.get_props());
    println!("Mixed problem without coupling: {:?}", problem_type4);
    println!("Strategy: {}", problem_type4.strategy_description());
}
