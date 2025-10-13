#[cfg(test)]
mod simple_alldiff_test {
    use selen::prelude::*;

    #[test]
    fn test_simple_alldiff() {
        let mut m = Model::default();
        let vars: Vec<_> = (0..3).map(|_| m.int(1, 3)).collect();
        
        println!("Created variables with domains [1,3]:");
        for (i, &var) in vars.iter().enumerate() {
            println!("  var[{}] = {:?}", i, var);
        }
        
        // All variables must have different values
        println!("Adding alldiff constraint...");
        m.alldiff(&vars);
        
        println!("Added alldiff constraint, attempting to solve...");
        let result = m.solve();
        
        match &result {
            Ok(solution) => {
                println!("Solution found!");
                let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
                println!("Values: {:?}", values);
                
                // Check that all values are different
                let mut unique_values = std::collections::HashSet::new();
                for &val in &values {
                    assert!(unique_values.insert(val), "Duplicate value found: {}", val);
                }
                println!("All values are different - test passed!");
            }
            Err(e) => {
                println!("Solve failed with error: {:?}", e);
            }
        }
        
        assert!(result.is_ok(), "Should find a solution for 3 vars in domain [1,3] with alldiff");
    }
    
    #[test] 
    fn test_simple_without_alldiff() {
        println!("Testing without alldiff constraint...");
        let mut m = Model::default();
        let vars: Vec<_> = (0..3).map(|_| m.int(1, 3)).collect();
        
        println!("Created variables with domains [1,3] - no constraints");
        let result = m.solve();
        
        match &result {
            Ok(solution) => {
                println!("Solution found without alldiff!");
                let values: Vec<i32> = vars.iter().map(|&v| solution.get_int(v)).collect();
                println!("Values: {:?}", values);
            }
            Err(e) => {
                println!("Solve failed even without alldiff: {:?}", e);
            }
        }
        
        assert!(result.is_ok(), "Should find a solution without constraints");
    }
}