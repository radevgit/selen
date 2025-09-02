//! Tests for enhanced not_equals functionality

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_basic_not_equals() {
        let mut m = Model::default();
        let x = m.new_var_int(1, 5);
        let y = m.new_var_int(1, 5);
        
        // Add not_equals constraint
        m.not_equals(x, y);
        
        let solution = m.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        assert_ne!(x_val, y_val, "x and y should have different values");
    }

    #[test]
    fn test_not_equals_propagation() {
        let mut m = Model::default();
        let x = m.new_var_int(1, 3);
        let y = m.new_var_int(1, 3);
        
        // x != y, this should still have solutions
        m.not_equals(x, y);
        
        let mut solution_count = 0;
        let mut solutions = Vec::new();
        
        for sol in m.enumerate() {
            let x_val = sol[x];
            let y_val = sol[y];
            solutions.push((x_val, y_val));
            solution_count += 1;
        }
        
        assert!(solution_count > 0, "Should find solutions");
        
        // Verify no solution has x == y
        let invalid_solutions = solutions.iter().filter(|(x_val, y_val)| x_val == y_val).count();
        assert_eq!(invalid_solutions, 0, "All solutions should respect not_equals constraint");
        
        // With domains [1,3] for both x and y, we should have 6 valid solutions:
        // (1,2), (1,3), (2,1), (2,3), (3,1), (3,2)
        assert_eq!(solution_count, 6, "Should find exactly 6 solutions for x,y ∈ [1,3] with x ≠ y");
    }

    #[test]
    fn test_all_different_simulation() {
        let mut m = Model::default();
        let x = m.new_var_int(1, 3);
        let y = m.new_var_int(1, 3);
        let z = m.new_var_int(1, 3);
        
        // All different: x != y, y != z, x != z
        m.not_equals(x, y);
        m.not_equals(y, z);
        m.not_equals(x, z);
        
        let mut solution_count = 0;
        for _sol in m.enumerate() {
            solution_count += 1;
        }
        
        // With domains [1,3] for x, y, z and all different constraint,
        // we should have exactly 6 solutions (3! permutations)
        assert_eq!(solution_count, 6, "Should find exactly 6 solutions (3! permutations)");
    }

    #[test]
    fn test_not_equals_unsatisfiable() {
        let mut m = Model::default();
        let x = m.new_var_int(5, 5); // x is fixed to 5
        let y = m.new_var_int(5, 5); // y is fixed to 5
        
        // x != y should be unsatisfiable since both are fixed to the same value
        m.not_equals(x, y);
        
        let solution = m.solve();
        assert!(solution.is_none(), "Should be unsatisfiable when both variables are fixed to the same value");
    }
}
