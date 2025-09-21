use crate::prelude::*;

#[test]
fn test_modulo_basic() {
    let mut m = Model::default();
    
    // Test: 7 % 3 = 1
    let x = m.int(7, 7);
    let y = m.int(3, 3);
    let result = m.modulo(x, y);
    
    let solution = m.solve().unwrap();
    assert_eq!(solution[result], Val::ValI(1));
}

#[test]
fn test_modulo_range() {
    let mut m = Model::default();
    
    // Test: x % 5 where x in [10, 14] should give results in [0, 4]
    let x = m.int(10, 14);
    let y = m.int(5, 5);
    let result = m.modulo(x, y);
    
    // The result should be constrained to [0, 4]
    m.le(result, int(4));
    m.ge(result, int(0));
    
    let solution = m.solve().unwrap();
    let result_val = match solution[result] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    assert!(result_val >= 0 && result_val <= 4);
}

#[test]
fn test_modulo_negative() {
    let mut m = Model::default();
    
    // Test: -7 % 3 should equal -1 (in Rust's definition)
    let x = m.int(-7, -7);
    let y = m.int(3, 3);
    let result = m.modulo(x, y);
    
    let solution = m.solve().unwrap();
    assert_eq!(solution[result], Val::ValI(-1));
}

#[test]
fn test_modulo_with_constraint() {
    let mut m = Model::default();
    
    // Find x such that x % 7 = 3 and x is in [10, 30]
    let x = m.int(10, 30);
    let seven = m.int(7, 7);
    let remainder = m.modulo(x, seven);
    
    m.equals(remainder, int(3));
    
    let solution = m.solve().unwrap();
    let x_val = match solution[x] {
        Val::ValI(i) => i,
        _ => panic!("Expected integer result"),
    };
    
    // Valid solutions: 10, 17, 24 (all have remainder 3 when divided by 7)
    assert!(x_val % 7 == 3);
    assert!(x_val >= 10 && x_val <= 30);
}

#[test]
fn test_modulo_float() {
    let mut m = Model::default();
    
    // Test: 7.5 % 2.0 = 1.5
    let x = m.float(7.5, 7.5);
    let y = m.float(2.0, 2.0);
    let result = m.modulo(x, y);
    
    let solution = m.solve().unwrap();
    match solution[result] {
        Val::ValF(f) => assert!((f - 1.5).abs() < 1e-6),
        _ => panic!("Expected float result"),
    }
}
