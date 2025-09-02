use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Model {
    vars: Vars,
    props: Propagators,
}

impl Model {
    /// Create a new decision variable, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    /// We don't want to deal with "unwrap" every time
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
    pub fn new_vars(
        &mut self,
        n: usize,
        min: Val,
        max: Val,
    ) -> impl Iterator<Item = VarId> + '_ {
        let (actual_min, actual_max) = if min < max { (min, max) } else { (max, min) };
        core::iter::repeat_with(move || self.new_var_unchecked(actual_min, actual_max)).take(n)
    }

    /// Create a new integer decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    pub fn new_var_int(&mut self, min: i32, max: i32) -> VarId {
        self.new_var(Val::ValI(min), Val::ValI(max))
    }

    /// Create new integer decision variables, with the provided domain bounds.
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    pub fn new_vars_int(
        &mut self,
        n: usize,
        min: i32,
        max: i32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValI(min), Val::ValI(max))
    }

    /// Create a new float decision variable with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    pub fn new_var_float(&mut self, min: f32, max: f32) -> VarId {
        self.new_var(Val::ValF(min), Val::ValF(max))
    }

    /// Create new float decision variables, with the provided domain bounds.
    ///
    /// Both lower and upper bounds are included in the domain.
    /// In case `max < min` the bounds will be swapped.
    pub fn new_vars_float(
        &mut self,
        n: usize,
        min: f32,
        max: f32,
    ) -> impl Iterator<Item = VarId> + '_ {
        self.new_vars(n, Val::ValF(min), Val::ValF(max))
    }

    /// Create a new binary decision variable.
    pub fn new_var_binary(&mut self) -> VarIdBin {
        VarIdBin(self.new_var_unchecked(Val::ValI(0), Val::ValI(1)))
    }

    /// Create new binary decision variables.
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
        self.vars.new_var_with_bounds(min, max)
    }

    /// Create an expression of two views added together.
    pub fn add(&mut self, x: impl View, y: impl View) -> VarId {
        let min = x.min_raw(&self.vars) + y.min_raw(&self.vars);
        let max = x.max_raw(&self.vars) + y.max_raw(&self.vars);
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.add(x, y, s);

        s
    }

    /// Create an expression of the sum of a slice of views.
    pub fn sum(&mut self, xs: &[impl View]) -> VarId {
        self.sum_iter(xs.iter().copied())
    }

    /// Create an expression of the sum of an iterator of views.
    pub fn sum_iter(&mut self, xs: impl IntoIterator<Item = impl View>) -> VarId {
        let xs: Vec<_> = xs.into_iter().collect();

        let min: Val = xs.iter().map(|x| x.min_raw(&self.vars)).sum();
        let max: Val = xs.iter().map(|x| x.max_raw(&self.vars)).sum();
        let s = self.new_var_unchecked(min, max);

        let _p = self.props.sum(xs, s);

        s
    }

    /// Declare two expressions to be equal.
    pub fn equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.equals(x, y);
    }

    /// Declare two expressions to be not equal.
    pub fn not_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.not_equals(x, y);
    }

    /// Declare constraint `x <= y`.
    pub fn less_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.less_than_or_equals(x, y);
    }

    /// Declare constraint `x < y`.
    pub fn less_than(&mut self, x: impl View, y: impl View) {
        let mut events = Vec::new();
        let ctx = Context::new(&mut self.vars, &mut events);
        let _p = self.props.less_than(x, y);
    }

    /// Declare constraint `x >= y`.
    pub fn greater_than_or_equals(&mut self, x: impl View, y: impl View) {
        let _p = self.props.greater_than_or_equals(x, y);
    }

    /// Declare constraint `x > y`.
    pub fn greater_than(&mut self, x: impl View, y: impl View) {
        let mut events = Vec::new();
        let ctx = Context::new(&mut self.vars, &mut events);
        let _p = self.props.greater_than(x, y);
    }

    /// Declare all-different constraint: all variables must have distinct values.
    /// This is more efficient than adding pairwise not-equals constraints.
    pub fn all_different(&mut self, vars: Vec<VarId>) {
        let _p = self.props.all_different(vars);
    }

    /// Find assignment that minimizes objective expression while satisfying all constraints.
    #[must_use]
    pub fn minimize(self, objective: impl View) -> Option<Solution> {
        self.minimize_and_iterate(objective).last()
    }

    /// Find assignment that minimizes objective expression with callback to capture solving statistics.
    #[must_use]
    pub fn minimize_with_callback<F>(self, objective: impl View, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // For optimization problems, we need a different approach since we iterate through all solutions
        let vars = self.vars;
        let props = self.props;
        
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
    pub fn minimize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Minimize::new(objective))
    }

    /// Enumerate assignments that satisfy all constraints, while minimizing objective expression, with callback.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn minimize_and_iterate_with_callback<F>(self, objective: impl View, callback: F) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        let vars = self.vars;
        let props = self.props;
        
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
    #[must_use]
    pub fn maximize(self, objective: impl View) -> Option<Solution> {
        self.minimize(objective.opposite())
    }

    /// Find assignment that maximizes objective expression with callback to capture solving statistics.
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
    pub fn maximize_and_iterate(self, objective: impl View) -> impl Iterator<Item = Solution> {
        self.minimize_and_iterate(objective.opposite())
    }

    /// Enumerate assignments that satisfy all constraints, while maximizing objective expression, with callback.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn maximize_and_iterate_with_callback<F>(self, objective: impl View, callback: F) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        self.minimize_and_iterate_with_callback(objective.opposite(), callback)
    }

    /// Search for assignment that satisfies all constraints within bounds of decision variables.
    #[must_use]
    pub fn solve(self) -> Option<Solution> {
        self.enumerate().next()
    }

    /// Search for assignment with a callback to capture solving statistics.
    /// 
    /// The callback receives the solving statistics when the search completes.
    #[must_use]
    pub fn solve_with_callback<F>(self, callback: F) -> Option<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        // Run the solving process
        let vars = self.vars;
        let props = self.props;
        
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

    /// Enumerate all assignments that satisfy all constraints.
    ///
    /// The order in which assignments are yielded is not stable.
    pub fn enumerate(self) -> impl Iterator<Item = Solution> {
        search(self.vars, self.props, mode::Enumerate)
    }

    /// Enumerate all assignments that satisfy all constraints with callback to capture solving statistics.
    ///
    /// The callback is called with final statistics after all solutions are found.
    /// Returns a vector of all solutions found during the search.
    pub fn enumerate_with_callback<F>(self, callback: F) -> Vec<Solution>
    where
        F: FnOnce(&crate::solution::SolveStats),
    {
        let vars = self.vars;
        let props = self.props;
        
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
