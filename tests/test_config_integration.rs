//! Integration tests for SolverConfig

use selen::prelude::*;

#[test]
fn test_model_with_config_basic() {
    let config = SolverConfig::default()
        .with_float_precision(4);
    
    let mut m = Model::with_config(config);
    assert_eq!(m.float_precision_digits(), 4);
    
    let x = m.float(0.0, 1.0);
    let y = m.float(0.0, 1.0);
    
    let target = m.float(1.0, 1.0);
    post!(m, x + y == target);
    
    // Should be able to solve with the custom configuration
    let solution = m.solve();
    assert!(solution.is_ok());
}

#[test]
fn test_config_builder_pattern() {
    let config = SolverConfig::new()
        .with_float_precision(8)
        .with_timeout_seconds(120)
        .with_max_memory_mb(2048);
        
    assert_eq!(config.float_precision_digits, 8);
    assert_eq!(config.timeout_seconds, Some(120));
    assert_eq!(config.max_memory_mb, Some(2048));
}

#[test]
fn test_config_vs_legacy_method() {
    // Both approaches should create identical models
    let config = SolverConfig::default().with_float_precision(3);
    let m1 = Model::with_config(config);
    let m2 = Model::with_float_precision(3);
    
    assert_eq!(m1.float_precision_digits(), m2.float_precision_digits());
    assert_eq!(m1.float_precision_digits(), 3);
}