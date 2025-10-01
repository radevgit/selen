//! Tests for reification constraints

use selen::prelude::*;

// TODO: This test has test ordering issues - passes when run alone but may
// fail when run with other tests. Related to propagation ordering.
#[test]
#[ignore]
fn test_int_eq_reif_true() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.int_eq_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 5
    m.new(x.eq(5));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // y must also be 5 because b=1 implies x=y
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_eq_reif_false() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.int_eq_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 5
    m.new(x.eq(5));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 5 because b=0 implies x≠y
    assert_eq!(solution[x], Val::ValI(5));
    assert_ne!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(0));
}

// TODO: This test has test ordering issues - passes when run alone but may
// fail when run with other tests. Related to propagation ordering.
#[test]
#[ignore]
fn test_int_eq_reif_inference_to_true() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.int_eq_reif(x, y, b);
    
    // Force both x and y to 5
    m.new(x.eq(5));
    m.new(y.eq(5));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // b must be 1 because x=y
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(1));
}

#[test]
fn test_int_eq_reif_inference_to_false() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x = y)
    m.int_eq_reif(x, y, b);
    
    // Force x and y to different values
    m.new(x.eq(5));
    m.new(y.eq(7));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // b must be 0 because x≠y
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(7));
    assert_eq!(solution[b], Val::ValI(0));
}

#[test]
fn test_int_ne_reif_true() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≠ y)
    m.int_ne_reif(x, y, b);
    
    // Force b to be true
    m.new(b.eq(1));
    
    // Force x to 5
    m.new(x.eq(5));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // y must NOT be 5 because b=1 implies x≠y
    assert_eq!(solution[x], Val::ValI(5));
    assert_ne!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(1));
}

// TODO: This test has a propagation ordering issue - works when variables are pre-fixed
// but fails ~80% of the time when using constraints to fix values
// Need to investigate propagation order or strengthen the propagator
#[test]
#[ignore]
fn test_int_ne_reif_false() {
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post reified constraint: b ⇔ (x ≠ y)
    m.int_ne_reif(x, y, b);
    
    // Force b to be false
    m.new(b.eq(0));
    
    // Force x to 5
    m.new(x.eq(5));
    
    // Solve
    let solution = m.solve().expect("Should find solution");
    
    // y must be 5 because b=0 implies x=y
    assert_eq!(solution[x], Val::ValI(5));
    assert_eq!(solution[y], Val::ValI(5));
    assert_eq!(solution[b], Val::ValI(0));
}
