use crate::prelude::*;
use crate::domain::float_interval::{DEFAULT_FLOAT_PRECISION_DIGITS, precision_to_step_size};
use std::ops::Index;

#[derive(Debug)]
pub struct Model {
    vars: Vars,
    props: Propagators,
    /// Precision for float variables (decimal places)
    pub float_precision_digits: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self::with_float_precision(DEFAULT_FLOAT_PRECISION_DIGITS)
    }
}

impl Model {
    /// Create a new model with custom float precision
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::with_float_precision(4); // 4 decimal places
    /// let var = model.new_var_float(0.0, 1.0);
    /// ```
    pub fn with_float_precision(precision_digits: i32) -> Self {
        Self {
            vars: Vars::default(),
            props: Propagators::default(),
            float_precision_digits: precision_digits,
        }
    }

    /// Get the current float precision setting
    pub fn float_precision_digits(&self) -> i32 {
        self.float_precision_digits
    }

    /// Get the step size for the current float precision
    pub fn float_step_size(&self) -> f32 {
        precision_to_step_size(self.float_precision_digits)
    }

    /// Create a new decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// We don't want to deal with "unwrap" every time
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var(Val::int(1), Val::int(10));
    /// ```
    pub fn new_var(&mut self, min: Val, max: Val) -> VarId {
        if min < max {
            self.new_var_unchecked(min, max)
        } else {
            self.new_var_unchecked(max, min)
        }
    }

    /// Create new decision variables, with the provided domain bounds.
    ///
    /// All created variables will have the same starting domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars(3, Val::int(0), Val::int(5)).collect();
    /// ```
    pub fn new_vars(&mut self, n: usize, min: Val, max: Val) -> impl Iterator<Item = VarId> + '_ {
        let (actual_min, actual_max) = if min < max { (min, max) } else { (max, min) };
        core::iter::repeat_with(move || self.new_var_unchecked(actual_min, actual_max)).take(n)
    }

    /// Create a new integer decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_int(1, 10);
    /// ```
    pub fn new_var_int(&mut self, min: i32, max: i32) -> VarId {
        self.new_var(Val::ValI(min), Val::ValI(max))
    }

    /// Create new integer decision variables, with the provided domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(5, 0, 9).collect();
    /// ```
    pub fn new_vars_int(
        &mut self,
        n: usize,
        min: i32,
        max: i32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValI(min), Val::ValI(max))
    }

    /// Create a new integer decision variable from a vector of specific values.
    /// This is useful for creating variables with non-contiguous domains.
    ///
    /// # Arguments
    /// * `values` - Vector of integer values that the variable can take
    ///
    /// # Returns
    /// A new VarId for the created variable
    ///
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_with_values(vec![2, 4, 6, 8]); // Even numbers only
    /// ```
    pub fn new_var_with_values(&mut self, values: Vec<i32>) -> VarId {
        self.props.on_new_var();
        self.vars.new_var_with_values(values)
    }

    /// Create a new float decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_float(0.0, 10.5);
    /// ```
    pub fn new_var_float(&mut self, min: f32, max: f32) -> VarId {
        self.new_var(Val::ValF(min), Val::ValF(max))
    }

    /// Create new float decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_float(3, 0.0, 1.0).collect();
    /// ```
    pub fn new_vars_float(
        &mut self,
        n: usize,
        min: f32,
        max: f32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValF(min), Val::ValF(max))
    }

    /// Create a new binary decision variable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let var = model.new_var_binary();
    /// ```
    pub fn new_var_binary(&mut self) -> VarIdBin {
        VarIdBin(self.new_var_unchecked(Val::ValI(0), Val::ValI(1)))
    }

    /// Create new binary decision variables.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_binary(4).collect();
    /// ```
    pub fn new_vars_binary(&mut self, n: usize) -> impl Iterator<Item = VarIdBin> + '_ {
        core::iter::repeat_with(|| self.new_var_binary()).take(n)
    }

    /// Create a new integer decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    ///
    /// This function assumes that `min < max`.
    fn new_var_unchecked(&mut self, min: Val, max: Val) -> VarId {
        self.props.on_new_var();
        let step_size = self.float_step_size();
        self.vars.new_var_with_bounds_and_step(min, max, step_size)
    }

    /// Create an expression of two views added together.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 5);
    /// let y = model.new_var_int(2, 8);
    /// let sum = model.add(x, y);
    /// ```
    pub fn add(&mut self, x: impl View, y: impl View) -> VarId {
        let min = x.min_raw(&self.vars) + y.min_raw(&self.vars);
        let max = x.max_raw(&self.vars) + y.max_raw(&self.vars);
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.add(x, y, s);

        s
    }

    /// Create an expression of the sum of a slice of views.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(3, 1, 10).collect();
    /// let total = model.sum(&vars);
    /// ```
    pub fn sum(&mut self, xs: &[impl View]) -> VarId {
        self.sum_iter(xs.iter().copied())
    }

    /// Create an expression of the sum of an iterator of views.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(3, 1, 10).collect();
    /// let total = model.sum_iter(vars.iter().copied());
    /// ```
    pub fn sum_iter(&mut self, xs: impl IntoIterator<Item = impl View>) -> VarId {
        let xs: Vec<_> = xs.into_iter().collect();

        let min: Val = xs.iter().map(|x| x.min_raw(&self.vars)).sum();
        let max: Val = xs.iter().map(|x| x.max_raw(&self.vars)).sum();
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sum(xs, s);

        s
    }

    /// Declare two expressions to be equal.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.equals(x, y);
    /// ```
    pub fn equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.equals(x, y);
    }

    /// Declare two expressions to be not equal.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.not_equals(x, y);
    /// ```
    pub fn not_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.not_equals(x, y);
    }

    /// Declare constraint `x <= y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(5, 15);
    /// model.less_than_or_equals(x, y);
    /// ```
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than_or_equals(x, y);
    }

    /// Declare constraint `x < y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.less_than(x, Val::int(5));
    /// ```
    pub fn less_than(&mut self, x: impl View, y: impl View) {
        //let mut events = Vec::new();
        //let ctx = Context::new(&mut self.vars, &mut events);
        let _p = self.props.less_than(x, y);
    }

    /// Declare constraint `x >= y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(5, 15);
    /// let y = model.new_var_int(1, 10);
    /// model.greater_than_or_equals(x, y);
    /// ```
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than_or_equals(x, y);
    }

    /// Declare constraint `x > y`.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.greater_than(x, float(2.5));
    /// ```
    pub fn greater_than(&mut self, x: impl View, y: impl View) {
        //let mut events = Vec::new();
        //let ctx = Context::new(&mut self.vars, &mut events);
        let _p = self.props.greater_than(x, y);
    }

    /// Declare all-different constraint: all variables must have distinct values.
    /// This is more efficient than adding pairwise not-equals constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(4, 1, 4).collect();
    /// model.all_different(vars);
    /// ```
    pub fn all_different(&mut self, vars: Vec<VarId>) {
        let _p = self.props.all_different(vars);
    }

    /// Find assignment that minimizes objective expression while satisfying all constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.greater_than(x, Val::int(3));
    /// let solution = model.minimize(x);
    /// ```
    #[must_use]
    pub fn minimize(self, objective: impl View) -> Option<Solution> {
        self.minimize_and_iterate(objective).last()
    }

    /// Find assignment that minimizes objective expression with callback to capture solving statistics.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let solution = model.minimize_with_callback(x, |stats| {
    ///     println!("Propagations: {}", stats.propagation_count);
    /// });
    /// ```
    #[must_use]
    pub fn minimize_with_callback<F>(self, objective: impl View, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // For optimization problems, we need a different approach since we iterate through all solutions
        let (vars, props) = self.prepare_for_search();

        let mut search_iter = search(vars, props, mode::Minimize::new(objective));
        let mut last_solution = None;
        let mut current_count = 0;

        // Iterate through all solutions to find the optimal one
        while let Some(solution) = search_iter.next() {
            last_solution = Some(solution);
            // Capture the count each iteration, as it might get lost when iterator is consumed
            current_count = search_iter.get_propagation_count();
        }

        let stats = crate::solution::SolveStats {
            propagation_count: current_count,
            node_count: search_iter.get_node_count(),
        };

        callback(&stats);
        last_solution
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 5);
    /// let solutions: Vec<_> = model.minimize_and_iterate(x).collect();
    /// ```
    pub fn minimize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        let (vars, props) = self.prepare_for_search();
        search(vars, props, mode::Minimize::new(objective))
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression, with callback.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn minimize_and_iterate_with_callback<F>(
        self,
        objective: impl View,
        callback: F,
    ) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        let (vars, props) = self.prepare_for_search();

        let mut search_iter = search(vars, props, mode::Minimize::new(objective));
        let mut solutions = Vec::new();
        let mut current_count = 0;

        // Collect all solutions manually and capture count during iteration
        while let Some(solution) = search_iter.next() {
            solutions.push(solution);
            // Capture the count each iteration, as it might get lost when iterator is consumed
            current_count = search_iter.get_propagation_count();
        }

        let stats = crate::solution::SolveStats {
            propagation_count: current_count,
            node_count: search_iter.get_node_count(),
        };

        callback(&stats);
        solutions
    }

    /// Find assignment that maximizes objective expression while satisfying all constraints.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// model.less_than(x, Val::int(8));
    /// let solution = model.maximize(x);
    /// ```
    #[must_use]
    pub fn maximize(self, objective: impl View) -> Option<Solution> {
        self.minimize(objective.opposite())
    }

    /// Find assignment that maximizes objective expression with callback to capture solving statistics.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let solution = model.maximize_with_callback(x, |stats| {
    ///     println!("Nodes explored: {}", stats.node_count);
    /// });
    /// ```
    #[must_use]
    pub fn maximize_with_callback<F>(self, objective: impl View, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        self.minimize_with_callback(objective.opposite(), callback)
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 5);
    /// let solutions: Vec<_> = model.maximize_and_iterate(x).collect();
    /// ```
    pub fn maximize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        self.minimize_and_iterate(objective.opposite())
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression, with callback.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn maximize_and_iterate_with_callback<F>(
        self,
        objective: impl View,
        callback: F,
    ) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        self.minimize_and_iterate_with_callback(objective.opposite(), callback)
    }

    /// Validate that all integer variable domains fit within the u16 optimization range.
    ///
    /// This method checks that all integer variables have domains that can be represented
    /// using u16 optimization (domain size ≤ 65535). Since we've already replaced VarI
    /// with VarSparse in the new_var_with_bounds method, this validation mainly serves
    /// as a safety check and provides clear error messages for invalid domain sizes.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if validation succeeds, or `Err(String)` with error details if validation fails.
    pub fn validate(&self) -> Result<(), String> {
        for (i, var) in self.vars.iter_with_indices() {
            match var {
                Var::VarI(sparse_set) => {
                    let domain_size = sparse_set.universe_size();
                    if domain_size > 65535 {
                        return Err(format!(
                            "Variable {} has domain size {} which exceeds the maximum of 65535 for u16 optimization. \
                            Consider using smaller domains or splitting large domains into multiple variables.",
                            i, domain_size
                        ));
                    }

                    // Additional validation: check if domain range is reasonable
                    let min_val = sparse_set.min_universe_value();
                    let max_val = sparse_set.max_universe_value();
                    let actual_range = max_val - min_val + 1;

                    if actual_range < 0 || actual_range > 65535 {
                        return Err(format!(
                            "Variable {} has invalid domain range [{}, {}] which results in {} values. \
                            Domain range must be positive and ≤ 65535.",
                            i, min_val, max_val, actual_range
                        ));
                    }
                }
                Var::VarF { .. } => {
                    // Float variables use interval representation, no validation needed
                }
            }
        }
        Ok(())
    }

    #[doc(hidden)]
    /// Optimize constraint processing order based on constraint characteristics.
    ///
    /// This method analyzes constraints (particularly AllDifferent) and reorders them
    /// to prioritize constraints with more fixed values, which tend to propagate more effectively.
    /// This can significantly improve solving performance by doing more effective propagation earlier.
    ///
    /// Should be called after all constraints are added but before solving.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let vars: Vec<_> = model.new_vars_int(4, 1, 4).collect();
    /// model.all_different(vars);
    /// model.optimize_constraint_order();
    /// ```
    pub fn optimize_constraint_order(&mut self) -> &mut Self {
        // Since we can't downcast trait objects easily, we'll implement this optimization
        // at the Propagators level by adding a method there
        self.props.optimize_alldiff_order(&self.vars);
        self
    }

    /// Search for assignment that satisfies all constraints within bounds of decision variables.

    /// Search for assignment that satisfies all constraints within bounds of decision variables.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let y = model.new_var_int(1, 10);
    /// model.not_equals(x, y);
    /// let solution = model.solve();
    /// ```
    #[must_use]
    pub fn solve(self) -> Option<Solution> {
        self.enumerate().next()
    }

    /// Search for assignment with a callback to capture solving statistics.
    ///
    /// The callback receives the solving statistics when the search completes.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 10);
    /// let solution = model.solve_with_callback(|stats| {
    ///     println!("Search completed with {} propagations", stats.propagation_count);
    /// });
    /// ```
    #[must_use]
    pub fn solve_with_callback<F>(self, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // Run the solving process
        let (vars, props) = self.prepare_for_search();

        // Create a search and run it to completion to capture final stats
        let mut search_iter = search(vars, props, mode::Enumerate);
        let result = search_iter.next();

        // Get the final stats from the search
        let final_propagation_count = search_iter.get_propagation_count();
        let final_node_count = search_iter.get_node_count();

        let stats = crate::solution::SolveStats {
            propagation_count: final_propagation_count,
            node_count: final_node_count,
        };

        callback(&stats);
        result
    }

    #[doc(hidden)]
    /// Internal helper that automatically optimizes constraints before search.
    /// This ensures all solving methods benefit from constraint optimization.
    fn prepare_for_search(mut self) -> (crate::vars::Vars, crate::props::Propagators) {
        // Automatically optimize constraint order for better performance
        self.optimize_constraint_order();
        (self.vars, self.props)
    }

    /// Enumerate all assignments that satisfy all constraints.
    ///
    /// The order in which assignments are yielded is not stable.
    /// 
    ///
    /// ```
    /// use cspsolver::prelude::*;
    /// let mut model = Model::default();
    /// let x = model.new_var_int(1, 3);
    /// let y = model.new_var_int(1, 3);
    /// model.not_equals(x, y);
    /// let solutions: Vec<_> = model.enumerate().collect();
    /// ```
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        let (vars, props) = self.prepare_for_search();
        search(vars, props, mode::Enumerate)
    }

    /// Enumerate all assignments that satisfy all constraints with callback to capture solving statistics.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn enumerate_with_callback<F>(self, callback: F) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        let (vars, props) = self.prepare_for_search();

        let mut search_iter = search(vars, props, mode::Enumerate);
        let mut solutions = Vec::new();

        // CRITICAL: Get the stats BEFORE calling any next() methods,
        // because Search::Done(Some(space)) becomes Search::Done(None) after the first next()
        let final_count = search_iter.get_propagation_count();
        let final_node_count = search_iter.get_node_count();

        // Collect all solutions
        while let Some(solution) = search_iter.next() {
            solutions.push(solution);
        }

        let stats = crate::solution::SolveStats {
            propagation_count: final_count,
            node_count: final_node_count,
        };

        callback(&stats);
        solutions
    }
}

impl Index<VarId> for Model {
    type Output = Var;

    fn index(&self, index: VarId) -> &Self::Output {
        &self.vars[index]
    }
}

#[test]
fn test_fix_type_aware_greater_than() {
    // Try minimize
    let mut m2 = Model::default();
    let v1_10 = m2.new_var_int(1, 10);
    m2.greater_than(v1_10, float(2.5));

    let solution = m2.minimize(v1_10).unwrap();
    let Val::ValI(x) = solution[v1_10] else {
        assert!(false, "Expected integer value");
        return;
    };

    println!("Debug: Found x = {}, expected x = 3", x);

    // Should find v0 = 3 since v0 > 2.5
    assert_eq!(x, 3);
    println!(
        "Type-aware greater_than constraint correctly found x = {}",
        x
    );
}

#[test]
fn test_precision_configuration() {
    // Test default precision
    let default_model = Model::default();
    assert_eq!(default_model.float_precision_digits(), 6);
    assert_eq!(default_model.float_step_size(), 1e-6);

    // Test custom precision
    let high_precision_model = Model::with_float_precision(10);
    assert_eq!(high_precision_model.float_precision_digits(), 10);
    assert_eq!(high_precision_model.float_step_size(), 1e-10);

    let low_precision_model = Model::with_float_precision(2);
    assert_eq!(low_precision_model.float_precision_digits(), 2);
    assert_eq!(low_precision_model.float_step_size(), 1e-2);

    // Test that variables can be created with different precisions
    let mut model1 = Model::with_float_precision(4);
    let mut model2 = Model::with_float_precision(8);
    
    let _var1 = model1.new_var_float(0.0, 1.0);
    let _var2 = model2.new_var_float(0.0, 1.0);
    
    // Both should succeed without errors
    assert_eq!(model1.float_step_size(), 1e-4);
    assert_eq!(model2.float_step_size(), 1e-8);
}

#[test]
fn test_precision_backward_compatibility() {
    // Verify that existing code using Model::default() continues to work
    let mut model = Model::default();
    
    // These should all work as before
    let _int_var = model.new_var_int(0, 10);
    let _float_var = model.new_var_float(0.0, 1.0);
    let _val_var = model.new_var(Val::int(0), Val::int(5));
    let _values_var = model.new_var_with_values(vec![1, 3, 5, 7]);
    
    // Default precision should be maintained
    assert_eq!(model.float_precision_digits(), 6);
}
