use selen::constraints::gac_bitset::BitSetGAC;
use selen::constraints::gac_hybrid::Variable;

#[test]
fn test_basic_bitset_gac_propagation() {
    let mut gac = BitSetGAC::new();
    
    // Create 3 variables with domains [1, 3]
    gac.add_variable(Variable(0), 1, 3);
    gac.add_variable(Variable(1), 1, 3);
    gac.add_variable(Variable(2), 1, 3);
    
    let variables = vec![Variable(0), Variable(1), Variable(2)];
    
    // Should succeed without changes for 3x3 case
    let (changed, consistent) = gac.propagate_alldiff(&variables);
    println!("Result: changed={}, consistent={}", changed, consistent);
    
    // Should be consistent (no error)
    assert!(consistent);
    
    // Variables should still have non-empty domains
    assert!(!gac.is_inconsistent(Variable(0)));
    assert!(!gac.is_inconsistent(Variable(1)));
    assert!(!gac.is_inconsistent(Variable(2)));
}

#[test]
fn test_impossible_bitset_gac() {
    let mut gac = BitSetGAC::new();
    
    // Create 3 variables with only 2 values - impossible
    gac.add_variable(Variable(0), 1, 2);
    gac.add_variable(Variable(1), 1, 2);
    gac.add_variable(Variable(2), 1, 2);
    
    let variables = vec![Variable(0), Variable(1), Variable(2)];
    
    // Should return Ok(false) indicating failure without error
    let (changed, consistent) = gac.propagate_alldiff(&variables);
    println!("Impossible case result: changed={}, consistent={}", changed, consistent);
    
    // Should be inconsistent (constraint violated)
    assert!(!consistent);
}

#[test]
fn test_assignment_propagation() {
    let mut gac = BitSetGAC::new();
    
    // Create 3 variables with domains [1, 3]
    gac.add_variable(Variable(0), 1, 3);
    gac.add_variable(Variable(1), 1, 3);
    gac.add_variable(Variable(2), 1, 3);
    
    // Assign first variable to 1
    gac.assign_variable(Variable(0), 1);
    
    let variables = vec![Variable(0), Variable(1), Variable(2)];
    
    // Should succeed and propagate
    let (changed, consistent) = gac.propagate_alldiff(&variables);
    println!("Assignment propagation result: changed={}, consistent={}", changed, consistent);
    
    assert!(consistent);
    assert!(changed); // Should have made changes
    
    // Other variables should no longer contain value 1
    if let Some(domain1) = gac.domains.get(&Variable(1)) {
        assert!(!domain1.contains(1), "Variable 1 should not contain value 1");
    }
    if let Some(domain2) = gac.domains.get(&Variable(2)) {
        assert!(!domain2.contains(1), "Variable 2 should not contain value 1");
    }
}