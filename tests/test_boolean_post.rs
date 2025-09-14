use cspsolver::prelude::*;
use cspsolver::boolean_operators::BooleanModel;

#[test]
fn test_boolean_post_macro() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    // This works - using boolean_operators module
    let expr_result = m.bool_expr((a | b) & !c);
    post!(m, expr_result == 1);
    
    // Set specific values for testing
    post!(m, a == 1);
    post!(m, b == 0);
    post!(m, c == 0);
    
    if let Some(solution) = m.solve() {
        println!("Solution found!");
        println!("a = {:?}", solution[a]);
        println!("b = {:?}", solution[b]);
        println!("c = {:?}", solution[c]);
        println!("(a | b) & !c = {:?}", solution[expr_result]);
        
        // Verify the boolean logic using pattern matching
        if let Val::ValI(a_val) = solution[a] { assert_eq!(a_val, 1); }
        if let Val::ValI(b_val) = solution[b] { assert_eq!(b_val, 0); }
        if let Val::ValI(c_val) = solution[c] { assert_eq!(c_val, 0); }
        if let Val::ValI(expr_val) = solution[expr_result] { assert_eq!(expr_val, 1); }
    } else {
        panic!("No solution found");
    }
}

#[test]
fn test_post_macro_function_style_booleans() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    // Function-style boolean operations within post! macro
    // These are constraint postings, not value assignments
    post!(m, and(a, b));  // Posts constraint that a AND b must be true
    post!(m, or(a, b));   // Posts constraint that a OR b must be true  
    post!(m, not(c));     // Posts constraint that c must be false
    
    if let Some(solution) = m.solve() {
        println!("Function-style solution found!");
        println!("a = {:?}", solution[a]);
        println!("b = {:?}", solution[b]);
        println!("c = {:?}", solution[c]);
        
        // Should be a=1, b=1, c=0 for and(a,b), or(a,b), not(c)
        if let Val::ValI(a_val) = solution[a] { assert_eq!(a_val, 1); }
        if let Val::ValI(b_val) = solution[b] { assert_eq!(b_val, 1); }
        if let Val::ValI(c_val) = solution[c] { assert_eq!(c_val, 0); }
    } else {
        panic!("No function-style solution found");
    }
}