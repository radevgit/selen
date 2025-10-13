//! Minimal test to reproduce the bug

use selen::prelude::*;

// This test passes
#[test]
fn test_a_passes() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    m.ne_reif(x, y, b);
    m.new(b.eq(1));
    m.new(x.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x], Val::ValI(5));
    assert_ne!(solution[y], Val::ValI(5)); // y should NOT be 5
    assert_eq!(solution[b], Val::ValI(1));
}

// This test should also pass
#[test]
fn test_b_flaky() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    m.ne_reif(x, y, b);
    m.new(b.eq(0)); // NOTE: b=0 here
    m.new(x.eq(5));
    
    let solution = m.solve().expect("Should find solution");
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5)); // y SHOULD be 5 when b=0
    assert_eq!(solution[b], Val::ValI(0));
}

// Run them in sequence
#[test]
fn test_c_combined() {
    test_a_passes();
    test_b_flaky();
}
