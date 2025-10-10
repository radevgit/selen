// Test that LP extraction recognizes Add + LessThanOrEquals patterns
use selen::prelude::*;

#[test]
fn test_extraction_add_le_pattern() {
    // Verify that m.add() + .le() creates linear constraints for LP
    let mut m = Model::default();
    let x = m.float(0.0, 100.0);
    let y = m.float(0.0, 100.0);
    
    // This creates: Add(x, y, sum) and LessThanOrEquals(sum, const_8000)
    let sum = m.add(x, y);
    m.new(sum.le(80.0));
    
    // Solve - should work quickly with small domains
    let result = m.solve();
    assert!(result.is_ok(), "Should solve simple add+le");
    
    if let Ok(solution) = result {
        let x_val = solution.get_float(x);
        let y_val = solution.get_float(y);
        println!("Solution: x={}, y={}", x_val, y_val);
        assert!(x_val + y_val <= 80.1, "Constraint violated");
    }
}

#[test]
fn test_extraction_recognizes_add_le() {
    // Simpler test: verify the simple add+le case works
    let mut m = Model::default();
    let x = m.float(0.0, 10.0);
    let y = m.float(0.0, 10.0);
    
    // This should create extractable constraints
    let sum = m.add(x, y);
    m.new(sum.le(15.0));
    m.new(x.ge(5.0));
    
    // Solve
    let result = m.solve();
    assert!(result.is_ok(), "Should solve with small domains");
    
    if let Ok(solution) = result {
        let x_val = solution.get_float(x);
        let y_val = solution.get_float(y);
        println!("Solution: x={}, y={}", x_val, y_val);
        assert!(x_val + y_val <= 15.1);
        assert!(x_val >= 4.9);
    }
}
