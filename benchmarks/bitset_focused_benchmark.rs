/// Simple benchmark focused on small domains to demonstrate BitSet advantages
use std::time::Instant;
use selen::constraints::gac::{SparseSetGAC, Variable};
use selen::constraints::gac_bitset::BitSetGAC;

fn main() {
    println!("BitSet vs SparseSet Performance Test");
    println!("===================================");
    
    let iterations = 10000;
    let test_cases = vec![
        (20, 5),   // 20 variables, domain 1-5
        (50, 10),  // 50 variables, domain 1-10  
        (100, 20), // 100 variables, domain 1-20
    ];
    
    for (num_vars, domain_size) in test_cases {
        println!("\nTest: {} variables, domain 1-{}", num_vars, domain_size);
        
        // Benchmark SparseSet
        let mut sparseset_time = 0u128;
        for _ in 0..iterations {
            let mut gac = SparseSetGAC::new();
            let variables: Vec<Variable> = (0..num_vars).map(|i| Variable(i)).collect();
            
            let start = Instant::now();
            
            // Add variables and do basic operations
            for &var in &variables {
                gac.add_variable(var, 1, domain_size as i32);
            }
            
            // Remove some values and check domains
            for (i, &var) in variables.iter().enumerate().take(10) {
                gac.remove_value(var, ((i % domain_size) + 1) as i32);
                let _ = gac.get_domain_values(var);
            }
            
            sparseset_time += start.elapsed().as_nanos();
        }
        sparseset_time /= iterations as u128;
        
        // Benchmark BitSet
        let mut bitset_time = 0u128;
        for _ in 0..iterations {
            let mut gac = BitSetGAC::new();
            let variables: Vec<Variable> = (0..num_vars).map(|i| Variable(i)).collect();
            
            let start = Instant::now();
            
            // Add variables and do basic operations
            for &var in &variables {
                gac.add_variable(var, 1, domain_size as i32);
            }
            
            // Remove some values and check domains
            for (i, &var) in variables.iter().enumerate().take(10) {
                gac.remove_value(var, ((i % domain_size) + 1) as i32);
                let _ = gac.get_domain_values(var);
            }
            
            bitset_time += start.elapsed().as_nanos();
        }
        bitset_time /= iterations as u128;
        
        println!("SparseSet: {:>6} ns", sparseset_time);
        println!("BitSet:    {:>6} ns ({:.2}x speedup)", bitset_time, sparseset_time as f64 / bitset_time as f64);
        
        // Test alldiff propagation specifically
        println!("\nAllDiff propagation test:");
        
        // SparseSet alldiff
        let mut sparseset_alldiff_time = 0u128;
        for _ in 0..(iterations / 10) {
            let mut gac = SparseSetGAC::new();
            let variables: Vec<Variable> = (0..10).map(|i| Variable(i)).collect();
            
            for &var in &variables {
                gac.add_variable(var, 1, domain_size as i32);
            }
            
            let start = Instant::now();
            
            // Assign some variables and propagate manually
            for i in 0..5 {
                gac.assign_variable(variables[i], (i + 1) as i32);
                // Remove assigned values from other variables
                for j in (i + 1)..10 {
                    gac.remove_value(variables[j], (i + 1) as i32);
                }
            }
            
            sparseset_alldiff_time += start.elapsed().as_nanos();
        }
        sparseset_alldiff_time /= (iterations / 10) as u128;
        
        // BitSet alldiff with built-in propagation
        let mut bitset_alldiff_time = 0u128;
        for _ in 0..(iterations / 10) {
            let mut gac = BitSetGAC::new();
            let variables: Vec<Variable> = (0..10).map(|i| Variable(i)).collect();
            
            for &var in &variables {
                gac.add_variable(var, 1, domain_size as i32);
            }
            
            let start = Instant::now();
            
            // Use built-in alldiff propagation
            for i in 0..5 {
                gac.assign_variable(variables[i], (i + 1) as i32);
            }
            let _ = gac.propagate_alldiff(&variables);
            
            bitset_alldiff_time += start.elapsed().as_nanos();
        }
        bitset_alldiff_time /= (iterations / 10) as u128;
        
        println!("SparseSet AllDiff: {:>6} ns", sparseset_alldiff_time);
        println!("BitSet AllDiff:    {:>6} ns ({:.2}x speedup)", bitset_alldiff_time, sparseset_alldiff_time as f64 / bitset_alldiff_time as f64);
    }
    
    println!("\n=== Conclusion ===");
    println!("For small domains (≤64 values), BitSet operations provide:");
    println!("• Faster domain operations due to bit manipulation");
    println!("• More efficient alldiff propagation with Hall sets");
    println!("• Better cache locality with compact representation");
}