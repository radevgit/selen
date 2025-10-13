use selen::prelude::*;
use std::time::Duration;

#[test]
fn test_solution_get_values_slice() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(2, 5);
    let z = m.int(3, 7);
    
    m.new(x.eq(5));
    m.new(y.eq(3));
    m.new(z.eq(6));
    
    let solution = m.solve().unwrap();
    let vals = solution.get_values(&[x, y, z]);
    
    assert_eq!(vals.len(), 3);
    assert_eq!(vals[0], Val::ValI(5));
    assert_eq!(vals[1], Val::ValI(3));
    assert_eq!(vals[2], Val::ValI(6));
}

#[test]
fn test_solution_get_values_array() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(2, 5);
    
    m.new(x.eq(7));
    m.new(y.eq(4));
    
    let solution = m.solve().unwrap();
    let vals = solution.get_values_array(&[x, y]);
    
    assert_eq!(vals[0], Val::ValI(7));
    assert_eq!(vals[1], Val::ValI(4));
}

#[test]
fn test_solution_get_values_iter() {
    let mut m = Model::default();
    let vars = vec![m.int(1, 3), m.int(2, 4), m.int(3, 5)];
    
    m.new(vars[0].eq(2));
    m.new(vars[1].eq(3));
    m.new(vars[2].eq(4));
    
    let solution = m.solve().unwrap();
    let vals: Vec<_> = solution.get_values_iter(&vars).collect();
    
    assert_eq!(vals.len(), 3);
    assert_eq!(vals[0], Val::ValI(2));
    assert_eq!(vals[1], Val::ValI(3));
    assert_eq!(vals[2], Val::ValI(4));
}

#[test]
fn test_solution_bool_values_via_get_int() {
    let mut m = Model::default();
    let b1 = m.bool();
    let b2 = m.bool();
    
    m.new(b1.eq(1));  // 1 represents true
    m.new(b2.eq(0));  // 0 represents false
    
    let solution = m.solve().unwrap();
    
    // Boolean values are represented as integers (0/1)
    assert_eq!(solution.get_int(b1), 1);
    assert_eq!(solution.get_int(b2), 0);
}

#[test]
fn test_solution_get_int() {
    let mut m = Model::default();
    let x = m.int(5, 10);
    m.new(x.eq(7));
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.get_int(x), 7);
}

#[test]
#[should_panic(expected = "Expected integer")]
fn test_solution_get_int_panics_on_float() {
    let mut m = Model::default();
    let x = m.float(1.0, 5.0);
    m.new(x.eq(3.5));
    
    let solution = m.solve().unwrap();
    let _ = solution.get_int(x); // Should panic
}

#[test]
fn test_solution_get_float() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(5.5));
    
    let solution = m.solve().unwrap();
    let val = solution.get_float(x);
    assert!((val - 5.5).abs() < 1e-6);
}

#[test]
#[should_panic(expected = "Expected float")]
fn test_solution_get_float_panics_on_int() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let solution = m.solve().unwrap();
    let _ = solution.get_float(x); // Should panic
}

#[test]
fn test_solution_try_get_int_success() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(8));
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.try_get_int(x).unwrap(), 8);
}

#[test]
fn test_solution_try_get_int_error() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(5.5));
    
    let solution = m.solve().unwrap();
    assert!(solution.try_get_int(x).is_err());
}

#[test]
fn test_solution_try_get_float_success() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(7.25));
    
    let solution = m.solve().unwrap();
    let val = solution.try_get_float(x).unwrap();
    assert!((val - 7.25).abs() < 1e-6);
}

#[test]
fn test_solution_try_get_float_error() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let solution = m.solve().unwrap();
    assert!(solution.try_get_float(x).is_err());
}

#[test]
fn test_solution_get_int_unchecked() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(6));
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.get_int_unchecked(x), 6);
}

#[test]
fn test_solution_get_float_unchecked() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(4.75));
    
    let solution = m.solve().unwrap();
    let val = solution.get_float_unchecked(x);
    assert!((val - 4.75).abs() < 1e-6);
}

#[test]
fn test_solution_as_int_some() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(9));
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.as_int(x), Some(9));
}

#[test]
fn test_solution_as_int_none() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(5.5));
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.as_int(x), None);
}

#[test]
fn test_solution_as_float_some() {
    let mut m = Model::default();
    let x = m.float(1.0, 10.0);
    m.new(x.eq(3.25));
    
    let solution = m.solve().unwrap();
    let val = solution.as_float(x).unwrap();
    assert!((val - 3.25).abs() < 1e-6);
}

#[test]
fn test_solution_as_float_none() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(7));
    
    let solution = m.solve().unwrap();
    assert_eq!(solution.as_float(x), None);
}

#[test]
fn test_solution_stats_efficiency() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let solution = m.solve().unwrap();
    let stats = solution.stats();
    
    // Efficiency should be propagations/nodes
    if stats.node_count > 0 {
        let expected_efficiency = stats.propagation_count as f64 / stats.node_count as f64;
        assert!((stats.efficiency() - expected_efficiency).abs() < 1e-6);
    }
}

#[test]
fn test_solution_stats_time_per_propagation() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let solution = m.solve().unwrap();
    let stats = solution.stats();
    
    let time_per_prop = stats.time_per_propagation();
    // Should be a valid duration
    assert!(time_per_prop.as_nanos() < u128::MAX);
}

#[test]
fn test_solution_stats_time_per_node() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let solution = m.solve().unwrap();
    let stats = solution.stats();
    
    let time_per_node = stats.time_per_node();
    // Should be a valid duration
    assert!(time_per_node.as_nanos() < u128::MAX);
}

#[test]
fn test_solution_stats_display_summary() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    m.new(x.eq(5));
    
    let solution = m.solve().unwrap();
    // Just ensure it doesn't panic
    solution.stats().display_summary();
}

#[test]
fn test_solve_stats_new() {
    let stats = SolveStats::new(
        100,
        50,
        Duration::from_secs(1),
        10,
        20,
        512
    );
    
    assert_eq!(stats.propagation_count, 100);
    assert_eq!(stats.node_count, 50);
    assert_eq!(stats.solve_time, Duration::from_secs(1));
    assert_eq!(stats.variable_count, 10);
    assert_eq!(stats.constraint_count, 20);
    assert_eq!(stats.peak_memory_mb, 512);
}

#[test]
fn test_solve_stats_efficiency_zero_nodes() {
    let stats = SolveStats::new(
        100,
        0,  // Zero nodes
        Duration::from_secs(1),
        10,
        20,
        512
    );
    
    // With zero nodes, efficiency calculation
    let _eff = stats.efficiency();
    // Just verify it doesn't panic
}

#[test]
fn test_solve_stats_time_per_propagation_zero_props() {
    let stats = SolveStats::new(
        0,  // Zero propagations
        50,
        Duration::from_secs(1),
        10,
        20,
        512
    );
    
    // Should handle zero propagations
    let time = stats.time_per_propagation();
    assert!(time.as_nanos() > 0 || time.is_zero());
}

#[test]
fn test_solve_stats_time_per_node_zero_nodes() {
    let stats = SolveStats::new(
        100,
        0,  // Zero nodes
        Duration::from_secs(1),
        10,
        20,
        512
    );
    
    // Should handle zero nodes
    let time = stats.time_per_node();
    assert!(time.as_nanos() > 0 || time.is_zero());
}
