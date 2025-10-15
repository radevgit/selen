use selen::prelude::*;

#[test]
fn test_implies_true_implies_true() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.implies(a, b);  // a → b
    m.new(a.eq(bool(true)));  // a is true
    
    let solution = m.solve().unwrap();
    // If a is true, b must be true
    assert_eq!(solution.get_bool(a).unwrap(), true);
    assert_eq!(solution.get_bool(b).unwrap(), true);
}

#[test]
fn test_implies_false_allows_anything() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.implies(a, b);  // a → b
    m.new(a.eq(bool(false)));  // a is false
    
    let solution = m.solve().unwrap();
    // If a is false, b can be either true or false
    assert_eq!(solution.get_bool(a).unwrap(), false);
    // b can be anything, just check we got a solution
    let _ = solution.get_bool(b).unwrap();
}

#[test]
fn test_implies_violation() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.implies(a, b);  // a → b
    m.new(a.eq(bool(true)));  // a is true
    m.new(b.eq(bool(false))); // b is false - violates implication!
    
    // Should be unsatisfiable: true → false is false
    assert!(m.solve().is_err());
}

#[test]
fn test_implies_chain() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    m.implies(a, b);  // a → b
    m.implies(b, c);  // b → c
    m.new(a.eq(bool(true)));  // a is true
    
    let solution = m.solve().unwrap();
    // Chain: if a then b, if b then c, so if a then c
    assert_eq!(solution.get_bool(a).unwrap(), true);
    assert_eq!(solution.get_bool(b).unwrap(), true);
    assert_eq!(solution.get_bool(c).unwrap(), true);
}

#[test]
fn test_implies_multiple() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    m.implies(a, b);  // if a then b
    m.implies(a, c);  // if a then c
    m.new(a.eq(bool(true)));  // a is true
    
    let solution = m.solve().unwrap();
    // If a is true, both b and c must be true
    assert_eq!(solution.get_bool(a).unwrap(), true);
    assert_eq!(solution.get_bool(b).unwrap(), true);
    assert_eq!(solution.get_bool(c).unwrap(), true);
}

#[test]
fn test_implies_biconditional() {
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    // a ↔ b (biconditional) = (a → b) ∧ (b → a)
    m.implies(a, b);
    m.implies(b, a);
    m.new(a.eq(bool(true)));
    
    let solution = m.solve().unwrap();
    // Both must have the same value
    assert_eq!(solution.get_bool(a).unwrap(), true);
    assert_eq!(solution.get_bool(b).unwrap(), true);
}

#[test]
fn test_implies_with_int_conditions() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let a = m.bool();
    let b = m.bool();
    
    let five = m.int(5, 5);  // constant variable
    
    // Reified: a ↔ (x > 5)
    m.gt_reif(x, five, a);
    // Reified: b ↔ (y > 5)
    m.gt_reif(y, five, b);
    
    // If x > 5, then y > 5
    m.implies(a, b);
    
    m.new(x.eq(int(7)));  // x > 5, so a = true
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.get_int(x), 7);
    assert_eq!(solution.get_bool(a).unwrap(), true);
    assert_eq!(solution.get_bool(b).unwrap(), true);
    // y must be > 5
    assert!(solution.get_int(y) > 5);
}
