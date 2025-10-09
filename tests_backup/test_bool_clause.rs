use selen::prelude::*;

// ═══════════════════════════════════════════════════════════════════════════
// Test Suite: bool_clause Constraint
// ═══════════════════════════════════════════════════════════════════════════
//
// The `bool_clause` constraint represents a CNF clause:
//   bool_clause(pos, neg) ≡ (∨ pos[i]) ∨ (∨ ¬neg[i])
//
// This means: at least one positive literal is true OR at least one negative 
// literal is false.
//
// Coverage:
// - Simple positive-only clauses
// - Simple negative-only clauses
// - Mixed positive and negative literals
// - Edge cases: empty arrays, tautologies
// - SAT/UNSAT scenarios
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_bool_clause_positive_only_sat() {
    // Clause: a ∨ b ∨ c (at least one must be true)
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    m.bool_clause(&[a, b, c], &[]);
    
    let solution = m.solve().expect("Should find solution where at least one is true");
    
    let val_a = solution[a] == Val::ValI(1);
    let val_b = solution[b] == Val::ValI(1);
    let val_c = solution[c] == Val::ValI(1);
    
    // At least one must be true
    assert!(val_a || val_b || val_c, "At least one variable should be true");
}

#[test]
fn test_bool_clause_positive_only_with_fixed() {
    // Clause: a ∨ b where we force both false (should be UNSAT)
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.bool_clause(&[a, b], &[]);
    m.props.equals(a, Val::ValI(0)); // Force a = false
    m.props.equals(b, Val::ValI(0)); // Force b = false
    
    assert!(m.solve().is_err(), "Should be UNSAT when all positive literals forced false");
}

#[test]
fn test_bool_clause_negative_only_sat() {
    // Clause: ¬a ∨ ¬b ∨ ¬c (at least one must be false)
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    m.bool_clause(&[], &[a, b, c]);
    
    let solution = m.solve().expect("Should find solution where at least one is false");
    
    let val_a = solution[a] == Val::ValI(1);
    let val_b = solution[b] == Val::ValI(1);
    let val_c = solution[c] == Val::ValI(1);
    
    // At least one must be false
    assert!(!val_a || !val_b || !val_c, "At least one variable should be false");
}

#[test]
fn test_bool_clause_negative_only_with_fixed() {
    // Clause: ¬a ∨ ¬b where we force both true (should be UNSAT)
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.bool_clause(&[], &[a, b]);
    m.props.equals(a, Val::ValI(1)); // Force a = true
    m.props.equals(b, Val::ValI(1)); // Force b = true
    
    assert!(m.solve().is_err(), "Should be UNSAT when all negative literals forced true");
}

#[test]
fn test_bool_clause_mixed_sat() {
    // Clause: a ∨ b ∨ ¬c ∨ ¬d
    // At least one of: a is true, b is true, c is false, or d is false
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    let d = m.bool();
    
    m.bool_clause(&[a, b], &[c, d]);
    
    let solution = m.solve().expect("Should find a satisfying assignment");
    
    let val_a = solution[a] == Val::ValI(1);
    let val_b = solution[b] == Val::ValI(1);
    let val_c = solution[c] == Val::ValI(1);
    let val_d = solution[d] == Val::ValI(1);
    
    // At least one condition must hold
    assert!(
        val_a || val_b || !val_c || !val_d,
        "Clause should be satisfied: a ∨ b ∨ ¬c ∨ ¬d"
    );
}

#[test]
fn test_bool_clause_mixed_with_constraints() {
    // Clause: a ∨ ¬b with additional constraints
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.bool_clause(&[a], &[b]);
    
    // Force a = false, so b must be false too
    m.props.equals(a, Val::ValI(0));
    
    let solution = m.solve().expect("Should be SAT");
    
    assert_eq!(solution[a], Val::ValI(0), "a should be false (forced)");
    assert_eq!(solution[b], Val::ValI(0), "b must be false since a is false");
}

#[test]
fn test_bool_clause_mixed_unsat() {
    // Clause: a ∨ ¬b where we force a=false and b=true (should be UNSAT)
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.bool_clause(&[a], &[b]);
    m.props.equals(a, Val::ValI(0)); // Force a = false
    m.props.equals(b, Val::ValI(1)); // Force b = true
    
    assert!(m.solve().is_err(), "Should be UNSAT: a=false and b=true violates a ∨ ¬b");
}

#[test]
fn test_bool_clause_single_positive() {
    // Clause: a (just one positive literal)
    let mut m = Model::default();
    let a = m.bool();
    
    m.bool_clause(&[a], &[]);
    
    let solution = m.solve().expect("Should be SAT");
    assert_eq!(solution[a], Val::ValI(1), "a must be true");
}

#[test]
fn test_bool_clause_single_negative() {
    // Clause: ¬a (just one negative literal)
    let mut m = Model::default();
    let a = m.bool();
    
    m.bool_clause(&[], &[a]);
    
    let solution = m.solve().expect("Should be SAT");
    assert_eq!(solution[a], Val::ValI(0), "a must be false");
}

#[test]
fn test_bool_clause_empty_unsat() {
    // Empty clause: no literals at all (should be UNSAT)
    let mut m = Model::default();
    
    m.bool_clause(&[], &[]);
    
    assert!(m.solve().is_err(), "Empty clause should be UNSAT");
}

#[test]
fn test_bool_clause_tautology() {
    // Clause: a ∨ ¬a (always true - tautology)
    let mut m = Model::default();
    let a = m.bool();
    
    m.bool_clause(&[a], &[a]);
    
    let _solution = m.solve().expect("Tautology should always be SAT");
}

#[test]
fn test_bool_clause_complex_cnf() {
    // Multiple clauses forming a CNF formula
    // (a ∨ b) ∧ (¬a ∨ c) ∧ (¬b ∨ ¬c)
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    let c = m.bool();
    
    m.bool_clause(&[a, b], &[]);      // a ∨ b
    m.bool_clause(&[c], &[a]);        // ¬a ∨ c
    m.bool_clause(&[], &[b, c]);      // ¬b ∨ ¬c
    
    let solution = m.solve().expect("CNF should be SAT");
    
    let val_a = solution[a] == Val::ValI(1);
    let val_b = solution[b] == Val::ValI(1);
    let val_c = solution[c] == Val::ValI(1);
    
    // Verify all clauses are satisfied
    assert!(val_a || val_b, "Clause 1: a ∨ b");
    assert!(!val_a || val_c, "Clause 2: ¬a ∨ c");
    assert!(!val_b || !val_c, "Clause 3: ¬b ∨ ¬c");
}

#[test]
fn test_bool_clause_large_positive() {
    // Large clause with many positive literals
    let mut m = Model::default();
    let vars: Vec<VarId> = (0..10).map(|_| m.bool()).collect();
    
    m.bool_clause(&vars, &[]);
    
    // Force all but last to false
    for i in 0..9 {
        m.props.equals(vars[i], Val::ValI(0));
    }
    
    let solution = m.solve().expect("Should be SAT");
    assert_eq!(solution[vars[9]], Val::ValI(1), "Last variable must be true");
}

#[test]
fn test_bool_clause_large_negative() {
    // Large clause with many negative literals
    let mut m = Model::default();
    let vars: Vec<VarId> = (0..10).map(|_| m.bool()).collect();
    
    m.bool_clause(&[], &vars);
    
    // Force all but last to true
    for i in 0..9 {
        m.props.equals(vars[i], Val::ValI(1));
    }
    
    let solution = m.solve().expect("Should be SAT");
    assert_eq!(solution[vars[9]], Val::ValI(0), "Last variable must be false");
}

#[test]
fn test_bool_clause_implications() {
    // Test clause as implication: a → b is equivalent to ¬a ∨ b
    let mut m = Model::default();
    let a = m.bool();
    let b = m.bool();
    
    m.bool_clause(&[b], &[a]); // ¬a ∨ b (i.e., a → b)
    m.props.equals(a, Val::ValI(1)); // Force a = true
    
    let solution = m.solve().expect("Should be SAT");
    assert_eq!(solution[a], Val::ValI(1), "a is true (forced)");
    assert_eq!(solution[b], Val::ValI(1), "b must be true (by implication)");
}
