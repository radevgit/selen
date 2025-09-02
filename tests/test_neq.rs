#[cfg(test)]
mod test_neq {
    use cspsolver::prelude::*;

    #[test]
    fn test_neq_basic_integer() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(0), Val::ValI(3));
        let y = model.new_var(Val::ValI(0), Val::ValI(3));
        
        // Create x != y constraint
        model.not_equals(x, y);
        
        // Should be satisfiable
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x != y in the solution
        assert_ne!(x_val, y_val);
    }

    #[test]
    fn test_neq_violation() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(5), Val::ValI(5));
        let y = model.new_var(Val::ValI(5), Val::ValI(5));
        
        // Both variables must be assigned to 5, constraint x != y should fail
        model.not_equals(x, y);
        
        let solution = model.solve();
        assert!(solution.is_none()); // Should be unsatisfiable
    }

    #[test]
    fn test_neq_floats() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValF(1.0), Val::ValF(2.0));
        let y = model.new_var(Val::ValF(1.5), Val::ValF(2.5));
        
        model.not_equals(x, y);
        
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x != y in the solution
        assert_ne!(x_val, y_val);
    }

    #[test]
    fn test_neq_mixed_types() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(1), Val::ValI(3));
        let y = model.new_var(Val::ValF(2.0), Val::ValF(4.0));
        
        model.not_equals(x, y);
        
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x != y in the solution  
        assert_ne!(x_val, y_val);
    }

    #[test]
    fn test_neq_no_overlap_domains() {
        let mut model = Model::default();
        let x = model.new_var(Val::ValI(1), Val::ValI(3));
        let y = model.new_var(Val::ValI(5), Val::ValI(7));
        
        // Domains don't overlap, constraint is automatically satisfied
        model.not_equals(x, y);
        
        let solution = model.solve();
        assert!(solution.is_some());
        
        let sol = solution.unwrap();
        let x_val = sol[x];
        let y_val = sol[y];
        
        // Verify x != y in the solution (should be automatic given non-overlapping domains)
        assert_ne!(x_val, y_val);
    }
}
