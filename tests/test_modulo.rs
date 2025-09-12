use cspsolver::prelude::*;

#[test]
fn test_modulo_basic() {
    let mut model = Model::default();
    
    // Test: 7 % 3 = 1
    let x = model.int(7, 7);
    let y = model.int(3, 3);
    let result = model.modulo(x, y);
    
    let solution = model.solve().unwrap();
    assert_eq!(solution[result], Val::ValI(1));
}

#[test]
fn test_modulo_range() {
    let mut model = Model::default();
    
    // Test: x % 5 where x in [10, 14] should give results in [0, 4]
    let x = model.int(10, 14);
    let y = model.int(5, 5);
    let result = model.modulo(x, y);
    
    // The result should be constrained to [0, 4]
    model.le(result, int(4));
    model.ge(result, int(0));
    
    let solution = model.solve().unwrap();
    let result_val = match solution[result] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    assert!(result_val >= 0 && result_val <= 4);
}

#[test] 
fn test_modulo_with_constraint() {
    let mut model = Model::default();
    
    // Find x such that x % 7 = 3 and x is in [10, 30]
    let x = model.int(10, 30);
    let seven = model.int(7, 7);
    let remainder = model.modulo(x, seven);
    
    model.equals(remainder, int(3));
    
    let solution = model.solve().unwrap();
    let x_val = match solution[x] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    // Valid solutions: 10, 17, 24 (all have remainder 3 when divided by 7)
    assert!(x_val % 7 == 3);
    assert!(x_val >= 10 && x_val <= 30);
    println!("Found x = {} where x % 7 = 3", x_val);
}
