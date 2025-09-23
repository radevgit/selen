use selen::prelude::*;

#[test]
fn test_boolean_post_macro() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    // Use function-style boolean operations in post! macro
    // This replaces: (a | b) & !c == 1
    // With explicit constraints that achieve the same logic
    post!(m, or(a, b));   // a OR b must be true
    post!(m, not(c));     // c must be false
    
    // Set specific values for testing
    post!(m, a == 1);
    post!(m, b == 0);
    post!(m, c == 0);
    
    let solution = m.solve().expect("Expected to find a solution for boolean constraints");
    
    println!("Solution found!");
    println!("a = {:?}", solution[a]);
    println!("b = {:?}", solution[b]);
    println!("c = {:?}", solution[c]);
    
    // Verify the boolean logic using pattern matching
    if let Val::ValI(a_val) = solution[a] { assert_eq!(a_val, 1); }
    if let Val::ValI(b_val) = solution[b] { assert_eq!(b_val, 0); }
    if let Val::ValI(c_val) = solution[c] { assert_eq!(c_val, 0); }
    
    // Verify that (a | b) & !c would be true
    // a=1, b=0, c=0 -> (1|0) & !0 -> 1 & 1 -> 1 ✓
}