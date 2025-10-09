//! Debug test for reification issue

use selen::prelude::*;

#[test]
fn debug_int_ne_reif_false_simple() {
    let mut m = Model::default();
    
    let x = m.int(5, 5);  // x is already fixed to 5
    let y = m.int(1, 10);
    let b = m.int(0, 0);  // b is already fixed to 0
    
    // Post reified constraint: b ⇔ (x ≠ y)
    // Since b=0, this means x = y must hold
    m.int_ne_reif(x, y, b);
    
    // Solve
    match m.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  x = {:?}", solution[x]);
            println!("  y = {:?}", solution[y]);
            println!("  b = {:?}", solution[b]);
            
            // y must be 5 because b=0 implies x=y
            assert_eq!(solution[x], Val::ValI(5));
            assert_eq!(solution[y], Val::ValI(5));
            assert_eq!(solution[b], Val::ValI(0));
        },
        Err(e) => {
            panic!("Failed to find solution: {:?}", e);
        }
    }
}
