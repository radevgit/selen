//! Debug test for negative coefficient issue

use selen::prelude::*;

#[test]
fn debug_simple_negative() {
    let mut m = Model::default();
    
    let x = m.int(4, 4); // Fixed to 4
    let y = m.int(0, 10);
    
    // 5x - 2y = 6
    // With x = 4: 20 - 2y = 6 => 2y = 14 => y = 7
    m.int_lin_eq(&[5, -2], &[x, y], 6);
    
    match m.solve() {
        Ok(solution) => {
            println!("Solution found!");
            println!("x = {:?}", solution[x]);
            println!("y = {:?}", solution[y]);
            
            if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
                let result = 5 * x_val - 2 * y_val;
                println!("5*{} - 2*{} = {}", x_val, y_val, result);
                assert_eq!(result, 6);
            }
        },
        Err(e) => {
            panic!("Failed to find solution: {:?}", e);
        }
    }
}

#[test]
fn debug_positive_only() {
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // 2x + 3y = 12
    m.int_lin_eq(&[2, 3], &[x, y], 12);
    m.new(x.eq(3));
    
    match m.solve() {
        Ok(solution) => {
            println!("Solution found!");
            println!("x = {:?}", solution[x]);
            println!("y = {:?}", solution[y]);
        },
        Err(e) => {
            panic!("Failed to find solution: {:?}", e);
        }
    }
}

#[test]
fn debug_manual_construction() {
    let mut m = Model::default();
    
    let x = m.int(4, 4);
    let y = m.int(0, 10);
    
    // Manually construct: 5x - 2y = 6
    // This is equivalent to 5x + (-2)y = 6
    let five_x = m.mul(x, Val::ValI(5));
    let neg_two_y = m.mul(y, Val::ValI(-2));
    let sum = m.add(five_x, neg_two_y);
    
    m.new(sum.eq(6));
    
    match m.solve() {
        Ok(solution) => {
            println!("Manual construction solution found!");
            println!("x = {:?}", solution[x]);
            println!("y = {:?}", solution[y]);
            
            if let (Val::ValI(x_val), Val::ValI(y_val)) = (solution[x], solution[y]) {
                let result = 5 * x_val - 2 * y_val;
                println!("5*{} - 2*{} = {}", x_val, y_val, result);
                assert_eq!(result, 6);
            }
        },
        Err(e) => {
            panic!("Failed to find solution: {:?}", e);
        }
    }
}
