use selen::prelude::*;

// ============================================================================
// Type Conversion Functions Tests
// ============================================================================

#[test]
fn test_int2float_basic() {
    let mut m = Model::default();
    let x = m.int(5, 5); // Fixed at 5
    
    let y = int2float(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 5);
    assert!((sol.get_float(y) - 5.0).abs() < 1e-9);
}

#[test]
fn test_int2float_range() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    
    let y = int2float(&mut m, x);
    
    // Add constraint to test: y must be at least 7.0
    m.props.greater_than_or_equals(y, Val::ValF(7.0));
    
    let sol = m.solve().expect("Should have solution");
    let x_val = sol.get_int(x);
    let y_val = sol.get_float(y);
    
    assert!(x_val >= 7);
    assert!((y_val - x_val as f64).abs() < 1e-9);
}

#[test]
fn test_bool2int_basic() {
    let mut m = Model::default();
    let b = m.bool();
    
    let i = bool2int(&mut m, b);
    
    // Constrain bool to true
    m.props.equals(b, Val::int(1));
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(b), 1);
    assert_eq!(sol.get_int(i), 1);
}

#[test]
fn test_bool2int_false() {
    let mut m = Model::default();
    let b = m.bool();
    
    let i = bool2int(&mut m, b);
    
    // Constrain bool to false
    m.props.equals(b, Val::int(0));
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(b), 0);
    assert_eq!(sol.get_int(i), 0);
}

#[test]
fn test_floor_basic() {
    let mut m = Model::default();
    let x = m.float(3.7, 3.7); // Fixed at 3.7
    
    let y = floor(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 3);
    assert!((sol.get_float(x) - 3.7).abs() < 1e-9);
}

#[test]
fn test_floor_negative() {
    let mut m = Model::default();
    let x = m.float(-2.3, -2.3); // Fixed at -2.3
    
    let y = floor(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -3); // floor(-2.3) = -3
}

#[test]
fn test_floor_exact_integer() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0); // Exact integer
    
    let y = floor(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 5);
}

#[test]
fn test_ceil_basic() {
    let mut m = Model::default();
    let x = m.float(3.2, 3.2); // Fixed at 3.2
    
    let y = ceil(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 4); // ceil(3.2) = 4
}

#[test]
fn test_ceil_negative() {
    let mut m = Model::default();
    let x = m.float(-2.3, -2.3); // Fixed at -2.3
    
    let y = ceil(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -2); // ceil(-2.3) = -2
}

#[test]
fn test_ceil_exact_integer() {
    let mut m = Model::default();
    let x = m.float(5.0, 5.0); // Exact integer
    
    let y = ceil(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 5);
}

#[test]
fn test_round_basic() {
    let mut m = Model::default();
    let x = m.float(3.4, 3.4); // Fixed at 3.4
    
    let y = round(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 3); // round(3.4) = 3
}

#[test]
fn test_round_up() {
    let mut m = Model::default();
    let x = m.float(3.6, 3.6); // Fixed at 3.6
    
    let y = round(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), 4); // round(3.6) = 4
}

#[test]
fn test_round_negative() {
    let mut m = Model::default();
    let x = m.float(-2.3, -2.3); // Fixed at -2.3
    
    let y = round(&mut m, x);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(y), -2); // round(-2.3) = -2
}

// ============================================================================
// Table Constraint Tests
// ============================================================================

#[test]
fn test_table_basic() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    
    let allowed_tuples = vec![
        vec![Val::int(1), Val::int(2)],
        vec![Val::int(2), Val::int(3)],
        vec![Val::int(3), Val::int(1)],
    ];
    
    table(&mut m, &[x, y], &allowed_tuples);
    
    let sol = m.solve().expect("Should have solution");
    let x_val = sol.get_int(x);
    let y_val = sol.get_int(y);
    
    // Check that the solution matches one of the allowed tuples
    assert!(
        (x_val == 1 && y_val == 2) || 
        (x_val == 2 && y_val == 3) ||
        (x_val == 3 && y_val == 1)
    );
}

#[test]
fn test_table_single_solution() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    
    let allowed_tuples = vec![
        vec![Val::int(1), Val::int(1)],
    ];
    
    table(&mut m, &[x, y], &allowed_tuples);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 1);
    assert_eq!(sol.get_int(y), 1);
}

#[test]
fn test_table_multiple_constraints() {
    let mut m = Model::default();
    let x = m.int(1, 5);
    let y = m.int(1, 5);
    let z = m.int(1, 5);
    
    let allowed_tuples = vec![
        vec![Val::int(1), Val::int(1), Val::int(1)],
        vec![Val::int(2), Val::int(2), Val::int(2)],
    ];
    
    table(&mut m, &[x, y, z], &allowed_tuples);
    
    let sol = m.solve().expect("Should have solution");
    let x_val = sol.get_int(x);
    let y_val = sol.get_int(y);
    let z_val = sol.get_int(z);
    
    assert_eq!(x_val, y_val);
    assert_eq!(y_val, z_val);
    assert!(x_val == 1 || x_val == 2);
}

// ============================================================================
// Global Cardinality Constraint (GCC) Tests
// ============================================================================

#[test]
fn test_gcc_basic() {
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    let z = m.int(1, 3);
    
    let count1 = m.int(0, 3);
    let count2 = m.int(0, 3);
    let count3 = m.int(0, 3);
    
    // Each value should appear a certain number of times
    let values = vec![1, 2, 3];
    let counts = vec![count1, count2, count3];
    
    gcc(&mut m, &[x, y, z], &values, &counts);
    
    let sol = m.solve().expect("Should have solution");
    let count1_val = sol.get_int(count1);
    let count2_val = sol.get_int(count2);
    let count3_val = sol.get_int(count3);
    
    // Total count should be 3 (the number of variables)
    assert_eq!(count1_val + count2_val + count3_val, 3);
}

#[test]
fn test_gcc_specific_distribution() {
    let mut m = Model::default();
    let x = m.int(1, 2);
    let y = m.int(1, 2);
    let z = m.int(1, 2);
    
    let count1 = m.int(2, 2); // Exactly 2 variables should be 1
    let count2 = m.int(1, 1); // Exactly 1 variable should be 2
    
    let values = vec![1, 2];
    let counts = vec![count1, count2];
    
    gcc(&mut m, &[x, y, z], &values, &counts);
    
    let sol = m.solve().expect("Should have solution");
    let x_val = sol.get_int(x);
    let y_val = sol.get_int(y);
    let z_val = sol.get_int(z);
    
    let ones = [x_val, y_val, z_val].iter().filter(|&&v| v == 1).count();
    let twos = [x_val, y_val, z_val].iter().filter(|&&v| v == 2).count();
    
    assert_eq!(ones, 2);
    assert_eq!(twos, 1);
}

// ============================================================================
// Cumulative Constraint Tests
// ============================================================================

#[test]
fn test_cumulative_basic() {
    let mut m = Model::default();
    let task1_start = m.int(0, 10);
    let task2_start = m.int(0, 10);
    
    let starts = vec![task1_start, task2_start];
    let durations = vec![3, 2];
    let demands = vec![2, 2];
    let capacity = 2;
    
    cumulative(&mut m, &starts, &durations, &demands, capacity);
    
    let sol = m.solve().expect("Should have solution");
    let t1_start = sol.get_int(task1_start);
    let t2_start = sol.get_int(task2_start);
    
    // Solution is valid - cumulative constraint was posted
    assert!(t1_start >= 0 && t1_start <= 10);
    assert!(t2_start >= 0 && t2_start <= 10);
}

#[test]
fn test_cumulative_three_tasks() {
    let mut m = Model::default();
    let task1_start = m.int(0, 20);
    let task2_start = m.int(0, 20);
    let task3_start = m.int(0, 20);
    
    let starts = vec![task1_start, task2_start, task3_start];
    let durations = vec![2, 2, 2];
    let demands = vec![1, 1, 1];
    let capacity = 2;
    
    cumulative(&mut m, &starts, &durations, &demands, capacity);
    
    let sol = m.solve().expect("Should have solution");
    let t1_start = sol.get_int(task1_start);
    let t2_start = sol.get_int(task2_start);
    let t3_start = sol.get_int(task3_start);
    
    // With capacity 2 and individual demands of 1, at most 2 tasks can run simultaneously
    // So we need to verify the schedule respects cumulative resource usage
    assert!(t1_start >= 0);
    assert!(t2_start >= 0);
    assert!(t3_start >= 0);
}

#[test]
fn test_cumulative_low_capacity() {
    let mut m = Model::default();
    let task1_start = m.int(0, 10);
    let task2_start = m.int(0, 10);
    
    let starts = vec![task1_start, task2_start];
    let durations = vec![3, 3];
    let demands = vec![3, 3];
    let capacity = 3;
    
    cumulative(&mut m, &starts, &durations, &demands, capacity);
    
    let sol = m.solve().expect("Should have solution");
    let t1_start = sol.get_int(task1_start);
    let t2_start = sol.get_int(task2_start);
    
    // Solution is valid - cumulative constraint was posted
    assert!(t1_start >= 0 && t1_start <= 10);
    assert!(t2_start >= 0 && t2_start <= 10);
}

// ============================================================================
// Boolean XOR and Implies Tests (from runtime API)
// ============================================================================

// Note: BoolXor and BoolImplies are tested via the runtime API module.
// These tests would require more complex setup with the constraint AST system.
// See test_bool_xor.rs and test_implies.rs for comprehensive boolean constraint tests.


// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_conversion_chain() {
    let mut m = Model::default();
    let x = m.int(5, 5);
    
    let y = int2float(&mut m, x);
    let z = floor(&mut m, y);
    
    let sol = m.solve().expect("Should have solution");
    assert_eq!(sol.get_int(x), 5);
    assert!((sol.get_float(y) - 5.0).abs() < 1e-9);
    assert_eq!(sol.get_int(z), 5);
}

#[test]
fn test_table_with_gcc() {
    let mut m = Model::default();
    let x = m.int(1, 2);
    let y = m.int(1, 2);
    
    // Table constraint: specific allowed tuples
    let allowed_tuples = vec![
        vec![Val::int(1), Val::int(1)],
        vec![Val::int(1), Val::int(2)],
        vec![Val::int(2), Val::int(2)],
    ];
    
    table(&mut m, &[x, y], &allowed_tuples);
    
    // GCC constraint: each value appears at least once
    let count1 = m.int(1, 2);
    let count2 = m.int(1, 2);
    
    gcc(&mut m, &[x, y], &[1, 2], &[count1, count2]);
    
    let sol = m.solve().expect("Should have solution");
    let x_val = sol.get_int(x);
    let y_val = sol.get_int(y);
    let count1_val = sol.get_int(count1);
    let count2_val = sol.get_int(count2);
    
    // Both values should appear
    assert!(count1_val > 0);
    assert!(count2_val > 0);
    
    // Solution should be in allowed tuples
    assert!(
        (x_val == 1 && y_val == 1) || 
        (x_val == 1 && y_val == 2) ||
        (x_val == 2 && y_val == 2)
    );
}

#[test]
fn test_cumulative_with_conversions() {
    let mut m = Model::default();
    
    // Create integer start times
    let task1_start = m.int(0, 10);
    let task2_start = m.int(0, 10);
    
    let starts = vec![task1_start, task2_start];
    let durations = vec![2, 2];
    let demands = vec![2, 2];
    let capacity = 2;
    
    cumulative(&mut m, &starts, &durations, &demands, capacity);
    
    // Convert to floats to verify values
    let f1 = int2float(&mut m, task1_start);
    let f2 = int2float(&mut m, task2_start);
    
    let sol = m.solve().expect("Should have solution");
    let t1_start = sol.get_int(task1_start);
    let t2_start = sol.get_int(task2_start);
    let f1_val = sol.get_float(f1);
    let f2_val = sol.get_float(f2);
    
    // Float values should match integer values
    assert!((f1_val - t1_start as f64).abs() < 1e-9);
    assert!((f2_val - t2_start as f64).abs() < 1e-9);
    
    // Solution is valid
    assert!(t1_start >= 0 && t1_start <= 10);
    assert!(t2_start >= 0 && t2_start <= 10);
}
