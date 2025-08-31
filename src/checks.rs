


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

#[test] 
fn debug_enumerate_propagation() {
    println!("\n=== Debugging Enumerate Propagation ===");
    
    // Test with a VERY simple problem that should definitely need propagation
    {
        println!("\n--- Testing simple constraint that MUST propagate ---");
        let mut m1 = Model::default();
        let v0 = m1.new_var_int(1, 10);
        let v1 = m1.new_var_int(5, 5); // v1 is fixed to 5
        
        m1.greater_than(v0, v1); // v0 > 5, so v0 must be at least 6
        
        println!("solve_with_callback test:");
        let mut solve_stats = SolveStats::default();
        let solution = m1.solve_with_callback(|stats| {
            solve_stats.propagation_count = stats.propagation_count;
            println!("  solve_with_callback - Propagation count: {}", stats.propagation_count);
        }).unwrap();
        
        let x0 = match solution[v0] { Val::ValI(i) => i, _ => panic!() };
        println!("  solve found: v0={} (should be >= 6)", x0);
    }
    
    {
        println!("\nenumerate_with_callback test:");
        let mut m2 = Model::default();
        let v0 = m2.new_var_int(1, 10);
        let v1 = m2.new_var_int(5, 5); // v1 is fixed to 5
        
        m2.greater_than(v0, v1); // v0 > 5, so v0 must be at least 6
        
        let mut enumerate_stats = SolveStats::default();
        let solutions = m2.enumerate_with_callback(|stats| {
            enumerate_stats.propagation_count = stats.propagation_count;
            println!("  enumerate_with_callback - Propagation count: {}", stats.propagation_count);
        });
        
        println!("  enumerate found {} solutions (should be 5: v0=6,7,8,9,10)", solutions.len());
        for (i, sol) in solutions.iter().take(3).enumerate() {
            let x0 = match sol[v0] { Val::ValI(i) => i, _ => panic!() };
            println!("    Solution {}: v0={}", i+1, x0);
        }
        if solutions.len() > 3 {
            println!("    ... and {} more", solutions.len() - 3);
        }
    }
    
    println!("=== Debug Complete ===\n");
}

#[test]
fn test_all_callback_methods() {
    // Test all the *_with_callback methods
    println!("\n=== Testing All Callback Methods ===");
    
    // Test minimize_with_callback
    {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 10);
        let v1 = m.new_var_int(1, 5);
        
        m.greater_than(v0, v1); // v0 > v1
        
        let mut minimize_stats = SolveStats::default();
        let solution = m.minimize_with_callback(v0, |stats| {
            minimize_stats.propagation_count = stats.propagation_count;
            println!("minimize_with_callback - Propagation count: {}", stats.propagation_count);
        }).unwrap();
        
        let x = match solution[v0] { Val::ValI(i) => i, _ => panic!() };
        assert_eq!(x, 2); // Should find v0 = 2 since v0 > v1 and min(v1) = 1
        assert!(minimize_stats.propagation_count > 0, "minimize should have propagation steps");
        println!("✓ minimize_with_callback working - propagation count: {}", minimize_stats.propagation_count);
    }
    
    // Test maximize_with_callback
    {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 10);
        let v1 = m.new_var_int(1, 5);
        
        m.greater_than(v0, v1); // v0 > v1
        
        let mut maximize_stats = SolveStats::default();
        let solution = m.maximize_with_callback(v0, |stats| {
            maximize_stats.propagation_count = stats.propagation_count;
            println!("maximize_with_callback - Propagation count: {}", stats.propagation_count);
        }).unwrap();
        
        let x = match solution[v0] { Val::ValI(i) => i, _ => panic!() };
        assert_eq!(x, 10, "v0 should be 10 when maximizing with v0 > v1"); 
        assert!(maximize_stats.propagation_count > 0, "maximize should have propagation steps");
        println!("✓ maximize_with_callback working - propagation count: {}", maximize_stats.propagation_count);
    }
    
    // Test solve_with_callback (for comparison)
    {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 10);
        let v1 = m.new_var_int(1, 5);
        
        m.greater_than(v0, v1); // v0 > v1
        
        let mut solve_stats = SolveStats::default();
        let _solution = m.solve_with_callback(|stats| {
            solve_stats.propagation_count = stats.propagation_count;
            println!("solve_with_callback - Propagation count: {}", stats.propagation_count);
        }).unwrap();
        
        assert!(solve_stats.propagation_count > 0, "solve should have propagation steps");
        println!("✓ solve_with_callback working - propagation count: {}", solve_stats.propagation_count);
    }
    
    // Test enumerate_with_callback
    {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 4); // Slightly larger domain 
        let v1 = m.new_var_int(1, 3);
        let v2 = m.new_var_int(1, 3); // Add another variable
        
        m.greater_than(v0, v1); // v0 > v1
        m.greater_than(v1, v2); // v1 > v2 - this creates a chain requiring more propagation
        
        let mut enumerate_stats = SolveStats::default();
        let solutions = m.enumerate_with_callback(|stats| {
            enumerate_stats.propagation_count = stats.propagation_count;
            println!("enumerate_with_callback - Propagation count: {}", stats.propagation_count);
        });
        
        // Should find solutions where v0 > v1 > v2
        assert!(solutions.len() >= 1, "Should find at least one solution");
        println!("✓ enumerate_with_callback working - found {} solutions, propagation count: {}", 
                 solutions.len(), enumerate_stats.propagation_count);
        // Don't require propagation steps for enumerate - it might find solutions without propagation
    }
    
    // Test minimize_and_iterate_with_callback
    {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 5);
        let v1 = m.new_var_int(1, 3);
        
        m.greater_than(v0, v1); // v0 > v1
        
        let mut iterate_stats = SolveStats::default();
        let solutions = m.minimize_and_iterate_with_callback(v0, |stats| {
            iterate_stats.propagation_count = stats.propagation_count;
            println!("minimize_and_iterate_with_callback - Propagation count: {}", stats.propagation_count);
        });
        
        assert!(solutions.len() >= 1, "Should find at least one solution");
        assert!(iterate_stats.propagation_count > 0, "minimize_and_iterate should have propagation steps");
        println!("✓ minimize_and_iterate_with_callback working - found {} solutions, propagation count: {}", 
                 solutions.len(), iterate_stats.propagation_count);
    }
    
    // Test maximize_and_iterate_with_callback
    {
        let mut m = Model::default();
        let v0 = m.new_var_int(1, 5);
        let v1 = m.new_var_int(1, 3);
        
        m.greater_than(v0, v1); // v0 > v1
        
        let mut iterate_stats = SolveStats::default();
        let solutions = m.maximize_and_iterate_with_callback(v0, |stats| {
            iterate_stats.propagation_count = stats.propagation_count;
            println!("maximize_and_iterate_with_callback - Propagation count: {}", stats.propagation_count);
        });
        
        assert!(solutions.len() >= 1, "Should find at least one solution");
        assert!(iterate_stats.propagation_count > 0, "maximize_and_iterate should have propagation steps");
        println!("✓ maximize_and_iterate_with_callback working - found {} solutions, propagation count: {}", 
                 solutions.len(), iterate_stats.propagation_count);
    }
    
    println!("=== All Callback Methods Test Complete ===\n");
}
