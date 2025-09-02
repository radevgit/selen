#[cfg(test)]
mod test_less_greater_ulp {
    use cspsolver::prelude::*;
    use cspsolver::utils::{float_next, float_prev};

    // ========== LESS_THAN TESTS ==========

    #[test]
    fn test_less_than_integers() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(1), Val::ValI(10));
        let y = model.new_var(Val::ValI(5), Val::ValI(15));
        
        // Add constraint x < y (should work with integers)
        model.less_than(x, y);
        
        // Solve to check it's satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x < y in the solution
        assert!(x_val < y_val);
    }

    #[test]
    fn test_less_than_floats() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValF(1.0), Val::ValF(10.0));
        let y = model.new_var(Val::ValF(5.0), Val::ValF(15.0));
        
        // Add constraint x < y (should work with floats using ULP precision)
        model.less_than(x, y);
        
        // Solve to check it's satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x < y in the solution
        assert!(x_val < y_val);
    }

    #[test]
    fn test_less_than_edge_case() {
        let mut model = Model::default();
        let base = 1.0f32;
        let next = float_next(base);
        
        let x = model.new_var(Val::ValF(base), Val::ValF(base));
        let y = model.new_var(Val::ValF(next), Val::ValF(next));
        
        // Add constraint x < y where y is exactly the next representable float
        model.less_than(x, y);
        
        // This should be satisfiable with ULP-based precision
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify the solution respects the constraint
        assert!(x_val < y_val);
    }

    #[test]
    fn test_less_than_unsatisfiable() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(10), Val::ValI(20));
        let y = model.new_var(Val::ValI(1), Val::ValI(5));
        
        // Add constraint x < y (impossible since x >= 10 and y <= 5)
        model.less_than(x, y);
        
        // Should be unsatisfiable
        let solution = model.solve();
        assert!(solution.is_none());
    }

    // ========== GREATER_THAN TESTS ==========

    #[test]
    fn test_greater_than_integers() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(10), Val::ValI(20));
        let y = model.new_var(Val::ValI(1), Val::ValI(15));
        
        // Add constraint x > y (should work with integers)
        model.greater_than(x, y);
        
        // Solve to check it's satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x > y in the solution
        assert!(x_val > y_val);
    }

    #[test]
    fn test_greater_than_floats() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValF(10.0), Val::ValF(20.0));
        let y = model.new_var(Val::ValF(1.0), Val::ValF(15.0));
        
        // Add constraint x > y (should work with floats using ULP precision)
        model.greater_than(x, y);
        
        // Solve to check it's satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x > y in the solution
        assert!(x_val > y_val);
    }

    #[test]
    fn test_greater_than_edge_case() {
        let mut model = Model::default();
        let base = 1.0f32;
        let prev = float_prev(base);
        
        let x = model.new_var(Val::ValF(base), Val::ValF(base));
        let y = model.new_var(Val::ValF(prev), Val::ValF(prev));
        
        // Add constraint x > y where y is exactly the previous representable float
        model.greater_than(x, y);
        
        // This should be satisfiable with ULP-based precision
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify the solution respects the constraint
        assert!(x_val > y_val);
    }

    #[test]
    fn test_greater_than_unsatisfiable() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(1), Val::ValI(5));
        let y = model.new_var(Val::ValI(10), Val::ValI(20));
        
        // Add constraint x > y (impossible since x <= 5 and y >= 10)
        model.greater_than(x, y);
        
        // Should be unsatisfiable
        let solution = model.solve();
        assert!(solution.is_none());
    }

    // ========== MIXED TESTS ==========

    #[test]
    fn test_less_and_greater_together() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(1), Val::ValI(10));
        let y = model.new_var(Val::ValI(5), Val::ValI(15));
        let z = model.new_var(Val::ValI(10), Val::ValI(20));
        
        // Add constraints x < y < z
        model.less_than(x, y);
        model.less_than(y, z);
        
        // Solve to check it's satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        let z_val = sol[z];
        
        // Verify x < y < z in the solution
        assert!(x_val < y_val);
        assert!(y_val < z_val);
    }

    #[test]
    fn test_mixed_integer_float_constraints() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(1), Val::ValI(10));
        let y = model.new_var(Val::ValF(5.5), Val::ValF(15.5));
        
        // Add constraint x < y (integer < float)
        model.less_than(x, y);
        
        // Solve to check it's satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x < y in the solution
        assert!(x_val < y_val);
    }

    #[test]
    fn test_ulp_precision_boundary() {
        let mut model = Model::default();
        
        // Test with consecutive representable floats
        let base = 1.0f32;
        let next = float_next(base);
        let next_next = float_next(next);
        
        let x = model.new_var(Val::ValF(base), Val::ValF(next));
        let y = model.new_var(Val::ValF(next), Val::ValF(next_next));
        
        // x < y should be satisfiable even with minimal ULP difference
        model.less_than(x, y);
        
        let solution = model.solve();
        assert!(solution.is_some());
    }
}
