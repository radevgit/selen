


#[allow(unused_imports)]
use crate::prelude::*;

#[test]
fn new_var() {
    // constraint: v0(int) * 1.5 < 5.0
    // solving for maximum v0
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 3);
    println!("v0 domain: [1, 3]");

    m.less_than(v0.times(float(1.5)), float(5.0));

    let solution = m.maximize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    assert!(x == 3);
}

#[test]
fn test_type_aware_constraint() {
    // Test the ViewType-based less_than method
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 3);
    println!("v0 domain: [1, 3] for type-aware test");

    // Use the type-aware constraint method
    m.less_than(v0.times_pos(float(1.5)), float(5.0));

    let solution = m.maximize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    assert_eq!(x, 3);
    println!("Type-aware constraint correctly found x = {}", x);
}

#[test]
fn test_pure_integer_constraint() {
    // Test that integer-only constraints use integer delta
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 10);
    let v1 = m.new_var_int(1, 5);

    // Pure integer constraint: v0 < v1
    m.less_than(v0, v1);

    let solution = m.maximize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    // Should find v0 = 4 since v0 < v1 and max(v1) = 5
    assert_eq!(x, 4);
    println!("Pure integer constraint correctly found x = {}", x);
}

#[test]
fn test_type_aware_greater_than() {
    // Test the type-aware greater_than method with mixed types
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 10);

    // Mixed constraint: v0 > 2.5 (should result in v0 >= 3)
    m.greater_than(v0, float(2.5));

    let solution = m.minimize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    // Should find v0 = 3 since v0 > 2.5
    assert_eq!(x, 3);
    println!(
        "Type-aware greater_than constraint correctly found x = {}",
        x
    );
}

#[test]
fn test_pure_integer_greater_than() {
    // Test that integer-only greater_than constraints use integer delta
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 10);
    let v1 = m.new_var_int(1, 5);

    // Pure integer constraint: v0 > v1
    m.greater_than(v0, v1);

    let solution = m.minimize(v0).unwrap();
    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    // Should find v0 = 2 since v0 > v1 and min(v1) = 1, so v0 > 1 means v0 >= 2
    assert_eq!(x, 2);
    println!(
        "Pure integer greater_than constraint correctly found x = {}",
        x
    );
}

#[test]
fn test_counter() {
    // Test the propagation counter using solve_with_callback
    println!("\n=== Testing Propagation Counter with Callback ===");
    
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 10);
    let v1 = m.new_var_int(1, 5);

    // Add constraint: v0 > v1
    m.greater_than(v0, v1);

    // Use the callback approach to capture solving statistics
    let mut stats = SolveStats::default();
    let solution = m.solve_with_callback(|solve_stats| {
        stats.propagation_count = solve_stats.propagation_count;
        println!("Propagation steps during solving: {}", solve_stats.propagation_count);
    }).unwrap();

    let x = match solution[v0] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    // Should find v0 = 2 since v0 > v1 and min(v1) = 1, so v0 > 1 means v0 >= 2
    assert_eq!(x, 2);
    println!("Solution found: v0 = {} (constraint v0 > v1 satisfied)", x);
    
    // Verify we captured the propagation count
    println!("Final captured propagation count: {}", stats.propagation_count);
    assert!(stats.propagation_count > 0, "Should have performed some propagation steps");
    
    println!("✓ Callback approach working!");
    println!("=== Test Complete ===\n");
}
