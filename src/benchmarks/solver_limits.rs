use std::time::{Duration, Instant};
use crate::prelude::*;

pub struct LimitResult {
    pub test_name: String,
    pub scale: String,
    pub duration: Duration,
    pub success: bool,
    pub precision_maintained: bool,
    pub problem_size: usize,
}

impl LimitResult {
    pub fn new(name: String, scale: String, duration: Duration, success: bool, size: usize) -> Self {
        // Check if precision optimization is likely still working (very fast)
        let precision_maintained = duration.as_micros() < 1000; // < 1ms suggests optimization
        
        Self {
            test_name: name,
            scale,
            duration,
            success,
            precision_maintained,
            problem_size: size,
        }
    }
}

// Test solver limits with engineering-scale values (cm to meters)
pub fn test_small_scale_precision() -> LimitResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    
    // Small precision: millimeter-level constraints (0.001m to 0.1m)
    let parts: Vec<_> = (0..10).map(|_i| {
        m.float(0.001, 0.1) // 1mm to 10cm
    }).collect();
    
    // Tight precision constraints at small scale
    for (i, &part) in parts.iter().enumerate() {
        let target = 0.01 + (i as f64 * 0.005); // 1cm to 5.5cm with 0.5cm increments
        post!(m, part > (target - 0.0001)); // ±0.1mm tolerance
        post!(m, part < (target + 0.0001));
    }
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    LimitResult::new("Small Scale (mm)".to_string(), "0.001-0.1m".to_string(), duration, success, 10)
}

pub fn test_medium_scale_precision() -> LimitResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    
    // Medium scale: centimeter to meter level (0.01m to 5m)
    let dimensions: Vec<_> = (0..25).map(|_i| {
        m.float(0.01, 5.0) // 1cm to 5m
    }).collect();
    
    // Engineering tolerances at medium scale
    for (i, &dim) in dimensions.iter().enumerate() {
        let target = 0.1 + (i as f64 * 0.2); // 10cm to 5m with 20cm increments
        post!(m, dim > (target - 0.001)); // ±1mm tolerance
        post!(m, dim < (target + 0.001));
    }
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    LimitResult::new("Medium Scale (cm-m)".to_string(), "0.01-5m".to_string(), duration, success, 25)
}

pub fn test_large_scale_precision() -> LimitResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    
    // Large scale: meter level (0.1m to 10m)
    let plates: Vec<_> = (0..50).map(|_i| {
        m.float(0.1, 10.0) // 10cm to 10m
    }).collect();
    
    // Large-scale engineering constraints
    for (i, &plate) in plates.iter().enumerate() {
        let target = 1.0 + (i as f64 * 0.18); // 1m to 9.82m with ~18cm increments
        post!(m, plate > (target - 0.01)); // ±1cm tolerance
        post!(m, plate < (target + 0.01));
    }
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    LimitResult::new("Large Scale (m)".to_string(), "0.1-10m".to_string(), duration, success, 50)
}

pub fn test_high_quantity_constraints() -> LimitResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    
    // Simulate high-quantity optimization: many parts with similar constraints
    let quantity = 100;
    let parts: Vec<_> = (0..quantity).map(|_| {
        m.float(0.05, 2.0) // 5cm to 2m parts
    }).collect();
    
    // Each part has positioning constraints
    for (i, &part) in parts.iter().enumerate() {
        let base_pos = (i % 10) as f64 * 0.2; // 20cm spacing pattern
        post!(m, part > (base_pos + 0.05));
        post!(m, part < (base_pos + 0.15)); // 10cm slot
    }
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    LimitResult::new("High Quantity".to_string(), "100 parts".to_string(), duration, success, quantity)
}

pub fn test_precision_boundary_limits() -> LimitResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    
    // Test very tight precision boundaries
    let precision_parts: Vec<_> = (0..20).map(|_i| {
        m.float(1.0, 2.0) // 1m to 2m range
    }).collect();
    
    // Extremely tight constraints - testing ULP precision limits
    for (i, &part) in precision_parts.iter().enumerate() {
        let target = 1.5 + (i as f64 * 0.000001); // Micrometer-level increments
        post!(m, part > (target - 0.0000005)); // ±0.5μm
        post!(m, part < (target + 0.0000005));
    }
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    LimitResult::new("Precision Boundary".to_string(), "μm tolerance".to_string(), duration, success, 20)
}

pub fn test_mixed_scale_complexity() -> LimitResult {
    let start = Instant::now();
    
    let mut m = Model::default();
    
    // Mix of different scales in one problem
    let small_parts: Vec<_> = (0..10).map(|_| m.float(0.001, 0.01)).collect(); // mm scale
    let medium_parts: Vec<_> = (0..15).map(|_| m.float(0.1, 1.0)).collect();   // dm scale  
    let large_parts: Vec<_> = (0..5).map(|_| m.float(1.0, 10.0)).collect();    // m scale
    
    // Cross-scale constraints
    for &small in &small_parts {
        post!(m, small > 0.002); // 2mm minimum
    }
    for &medium in &medium_parts {
        post!(m, medium < 0.8); // 80cm maximum
    }
    for &large in &large_parts {
        post!(m, large > 2.0); // 2m minimum
    }
    
    let success = m.solve().is_ok();
    let duration = start.elapsed();
    
    LimitResult::new("Mixed Scale".to_string(), "mm-m range".to_string(), duration, success, 30)
}

pub fn run_solver_limit_investigation() {
    println!("=== CSP SOLVER LIMIT INVESTIGATION ===");
    println!("Testing solver performance and precision limits with engineering scales");
    println!();
    
    let tests = vec![
        test_small_scale_precision(),
        test_medium_scale_precision(),
        test_large_scale_precision(),
        test_high_quantity_constraints(),
        test_precision_boundary_limits(),
        test_mixed_scale_complexity(),
    ];
    
    println!("Results:");
    println!("{:<20} {:<15} {:<12} {:<8} {:<12} {:<6}", 
             "Test", "Scale", "Duration(μs)", "Success", "Precision", "Size");
    println!("{}", "-".repeat(80));
    
    for result in &tests {
        println!("{:<20} {:<15} {:<12} {:<8} {:<12} {:<6}",
                result.test_name,
                result.scale,
                result.duration.as_micros(),
                if result.success { "✓" } else { "✗" },
                if result.precision_maintained { "Fast" } else { "Slow" },
                result.problem_size);
    }
    
    println!();
    println!("=== ANALYSIS ===");
    
    // Performance analysis
    let all_successful = tests.iter().all(|r| r.success);
    let precision_maintained_count = tests.iter().filter(|r| r.precision_maintained).count();
    let avg_duration = tests.iter().map(|r| r.duration.as_micros()).sum::<u128>() as f64 / tests.len() as f64;
    
    println!("Success rate: {}/{} tests", tests.iter().filter(|r| r.success).count(), tests.len());
    println!("Precision optimization maintained: {}/{} tests", precision_maintained_count, tests.len());
    println!("Average duration: {:.1} μs", avg_duration);
    
    // Scale analysis
    let small_scale = tests.iter().find(|r| r.test_name.contains("Small")).unwrap();
    let large_scale = tests.iter().find(|r| r.test_name.contains("Large")).unwrap();
    let high_quantity = tests.iter().find(|r| r.test_name.contains("Quantity")).unwrap();
    
    println!();
    println!("Scale Performance:");
    println!("  Small scale (mm): {} μs", small_scale.duration.as_micros());
    println!("  Large scale (m):  {} μs", large_scale.duration.as_micros());
    println!("  High quantity:    {} μs", high_quantity.duration.as_micros());
    
    // Identify limits
    println!();
    println!("SOLVER LIMITS IDENTIFIED:");
    
    if precision_maintained_count >= tests.len() * 3 / 4 {
        println!("✅ Precision optimization working well across engineering scales");
    } else {
        println!("⚠️  Precision optimization degrading at some scales");
    }
    
    if avg_duration < 1000.0 {
        println!("✅ Performance suitable for real-time engineering applications");
    } else if avg_duration < 10000.0 {
        println!("⚠️  Performance suitable for interactive engineering applications");
    } else {
        println!("❌ Performance needs optimization for engineering applications");
    }
    
    if all_successful {
        println!("✅ Solver handles all tested engineering constraint scenarios");
    } else {
        println!("❌ Solver failing on some engineering constraint scenarios");
    }
}
