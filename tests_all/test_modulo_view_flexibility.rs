/// Demonstration that modulo works with both variables and constants

use selen::prelude::*;
use selen::variables::Val;

#[test]
fn test_modulo_with_constants() {
    let mut m = Model::default();
    let x = m.int(1, 20);
    
    // Modulo with constant: x % 5
    let result = m.modulo(x, Val::int(5));
    
    m.new(x.eq(13));
    
    if let Ok(solution) = m.solve() {
        let x_val = solution[x].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(x_val, 13);
        assert_eq!(result_val, 13 % 5, "13 % 5 should be 3");
    }
}

#[test]
fn test_modulo_with_variables() {
    let mut m = Model::default();
    let x = m.int(1, 20);
    let y = m.int(2, 6);
    
    // Modulo with variable: x % y
    let result = m.modulo(x, y);
    
    m.new(x.eq(13));
    m.new(y.eq(5));
    
    if let Ok(solution) = m.solve() {
        let x_val = solution[x].as_int().unwrap();
        let y_val = solution[y].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(x_val, 13);
        assert_eq!(y_val, 5);
        assert_eq!(result_val, 13 % 5, "13 % 5 should be 3");
    }
}

#[test]
fn test_modulo_with_computed_expression() {
    let mut m = Model::default();
    let x = m.int(1, 20);
    let y = m.int(1, 5);
    
    // Modulo with computed expression: x % (y + 2)
    let divisor = m.add(y, Val::int(2));  // y + 2
    let result = m.modulo(x, divisor);
    
    m.new(x.eq(13));
    m.new(y.eq(1));  // divisor = 1 + 2 = 3
    
    if let Ok(solution) = m.solve() {
        let x_val = solution[x].as_int().unwrap();
        let y_val = solution[y].as_int().unwrap();
        let divisor_val = solution[divisor].as_int().unwrap();
        let result_val = solution[result].as_int().unwrap();
        
        assert_eq!(x_val, 13);
        assert_eq!(y_val, 1);
        assert_eq!(divisor_val, 3);
        assert_eq!(result_val, 13 % 3, "13 % 3 should be 1");
    }
}

#[test]
fn test_modulo_mixed_types() {
    let mut m = Model::default();
    let x = m.int(10, 30);
    
    // Can mix: variable % constant
    let result1 = m.modulo(x, Val::int(7));
    
    // Or: constant % variable (though less common)
    let result2 = m.modulo(Val::int(25), x);
    
    m.new(x.eq(15));
    
    if let Ok(solution) = m.solve() {
        let x_val = solution[x].as_int().unwrap();
        let r1 = solution[result1].as_int().unwrap();
        let r2 = solution[result2].as_int().unwrap();
        
        assert_eq!(x_val, 15);
        assert_eq!(r1, 15 % 7, "15 % 7 = 1");
        assert_eq!(r2, 25 % 15, "25 % 15 = 10");
    }
}
