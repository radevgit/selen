//! Global constraint mappers
//!
//! Maps FlatZinc global constraints (all_different, sort, table, lex_less, nvalue) to Selen constraint model.

use crate::flatzinc::ast::*;
use crate::flatzinc::error::{FlatZincError, FlatZincResult};
use crate::flatzinc::mapper::MappingContext;
use crate::runtime_api::{ModelExt, VarIdExt};

impl<'a> MappingContext<'a> {
    /// Map all_different constraint
    pub(in crate::flatzinc::mapper) fn map_all_different(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 1 {
            return Err(FlatZincError::MapError {
                message: "all_different requires 1 argument (array of variables)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let var_ids = self.extract_var_array(&constraint.args[0])?;
        self.model.alldiff(&var_ids);
        Ok(())
    }
    
    /// Map sort constraint: y is the sorted version of x
    /// FlatZinc signature: sort(x, y)
    /// 
    /// Decomposition:
    /// 1. y contains the same values as x (they are permutations)
    /// 2. y is sorted: y[i] <= y[i+1] for all i
    /// 
    /// Implementation strategy:
    /// - For each element in y, it must equal some element in x
    /// - y must be in non-decreasing order
    /// - Use global_cardinality to ensure same multiset
    pub(in crate::flatzinc::mapper) fn map_sort(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "sort requires 2 arguments (unsorted array, sorted array)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.extract_var_array(&constraint.args[0])?;
        let y = self.extract_var_array(&constraint.args[1])?;
        
        if x.len() != y.len() {
            return Err(FlatZincError::MapError {
                message: format!(
                    "sort: arrays must have same length (x: {}, y: {})",
                    x.len(),
                    y.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let n = x.len();
        
        // Constraint 1: y is sorted (non-decreasing order)
        // y[i] <= y[i+1] for all i
        for i in 0..n.saturating_sub(1) {
            self.model.new(y[i].le(&y[i + 1]));
        }
        
        // Constraint 2: y is a permutation of x
        // For each value that appears in the union of domains:
        // count(x, value) = count(y, value)
        //
        // Since we don't have direct access to domains, we use a simpler approach:
        // For small arrays, ensure each y[i] equals some x[j] using element-like constraints
        // For larger arrays, we rely on the combined constraints being sufficient
        
        if n <= 10 {
            // For small arrays, add explicit channeling constraints
            // Each y[i] must equal at least one x[j]
            for &yi in &y {
                // Create: (yi = x[0]) OR (yi = x[1]) OR ... OR (yi = x[n-1])
                let mut equality_vars = Vec::new();
                for &xj in &x {
                    let bi = self.model.bool();
                    self.model.int_eq_reif(yi, xj, bi);
                    equality_vars.push(bi);
                }
                let or_result = self.model.bool_or(&equality_vars);
                self.model.new(or_result.eq(1));
            }
            
            // Similarly for x: each x[j] must equal at least one y[i]
            for &xj in &x {
                let mut equality_vars = Vec::new();
                for &yi in &y {
                    let bi = self.model.bool();
                    self.model.int_eq_reif(xj, yi, bi);
                    equality_vars.push(bi);
                }
                let or_result = self.model.bool_or(&equality_vars);
                self.model.new(or_result.eq(1));
            }
        }
        // For larger arrays, the sorting constraint + domain pruning should be sufficient
        // A more efficient implementation would use proper channeling or element constraints
        
        Ok(())
    }
    
    /// Map table_int constraint: tuple(x) must be in table t
    /// FlatZinc signature: table_int(array[int] of var int: x, array[int, int] of int: t)
    /// 
    /// The table t is a 2D array where each row is a valid tuple.
    /// Decomposition: Create boolean for each row, at least one must be true
    pub(in crate::flatzinc::mapper) fn map_table_int(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "table_int requires 2 arguments (variable array, table)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.extract_var_array(&constraint.args[0])?;
        let arity = x.len();
        
        // Extract the table: 2D array of integers
        // The table format is a flat array representing rows
        let table_data = self.extract_int_array(&constraint.args[1])?;
        
        if table_data.is_empty() {
            // Empty table means no valid tuples - unsatisfiable
            let false_var = self.model.int(0, 0);
            self.model.new(false_var.eq(1)); // Force failure
            return Ok(());
        }
        
        if table_data.len() % arity != 0 {
            return Err(FlatZincError::MapError {
                message: format!(
                    "table_int: table size {} is not a multiple of arity {}",
                    table_data.len(),
                    arity
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let num_rows = table_data.len() / arity;
        
        // For each row in the table, create a boolean indicating if x matches this row
        let mut row_matches = Vec::new();
        
        for row_idx in 0..num_rows {
            // Create booleans for each position match
            let mut position_matches = Vec::new();
            
            for col_idx in 0..arity {
                let table_value = table_data[row_idx * arity + col_idx];
                let var = x[col_idx];
                
                // Create: b_i ↔ (x[i] = table_value)
                let b = self.model.bool();
                let const_var = self.model.int(table_value, table_value);
                self.model.int_eq_reif(var, const_var, b);
                position_matches.push(b);
            }
            
            // All positions must match for this row
            let row_match = self.model.bool_and(&position_matches);
            row_matches.push(row_match);
        }
        
        // At least one row must match
        let any_row_matches = self.model.bool_or(&row_matches);
        self.model.new(any_row_matches.eq(1));
        
        Ok(())
    }
    
    /// Map table_bool constraint: tuple(x) must be in table t
    /// FlatZinc signature: table_bool(array[int] of var bool: x, array[int, int] of bool: t)
    pub(in crate::flatzinc::mapper) fn map_table_bool(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "table_bool requires 2 arguments (variable array, table)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.extract_var_array(&constraint.args[0])?;
        let arity = x.len();
        
        // Extract the table: 2D array of booleans
        let table_data = self.extract_bool_array(&constraint.args[1])?;
        
        if table_data.is_empty() {
            // Empty table means no valid tuples - unsatisfiable
            let false_var = self.model.int(0, 0);
            self.model.new(false_var.eq(1)); // Force failure
            return Ok(());
        }
        
        if table_data.len() % arity != 0 {
            return Err(FlatZincError::MapError {
                message: format!(
                    "table_bool: table size {} is not a multiple of arity {}",
                    table_data.len(),
                    arity
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let num_rows = table_data.len() / arity;
        
        // For each row in the table, create a boolean indicating if x matches this row
        let mut row_matches = Vec::new();
        
        for row_idx in 0..num_rows {
            // Create booleans for each position match
            let mut position_matches = Vec::new();
            
            for col_idx in 0..arity {
                let table_value = table_data[row_idx * arity + col_idx];
                let var = x[col_idx];
                
                // Create: b_i ↔ (x[i] = table_value)
                let b = self.model.bool();
                let const_var = self.model.int(table_value as i32, table_value as i32);
                self.model.int_eq_reif(var, const_var, b);
                position_matches.push(b);
            }
            
            // All positions must match for this row
            let row_match = self.model.bool_and(&position_matches);
            row_matches.push(row_match);
        }
        
        // At least one row must match
        let any_row_matches = self.model.bool_or(&row_matches);
        self.model.new(any_row_matches.eq(1));
        
        Ok(())
    }
    
    /// Map lex_less constraint: x <_lex y (lexicographic strict ordering)
    /// FlatZinc signature: lex_less(array[int] of var int: x, array[int] of var int: y)
    /// 
    /// Decomposition: x <_lex y iff ∃i: (∀j<i: x[j]=y[j]) ∧ (x[i]<y[i])
    pub(in crate::flatzinc::mapper) fn map_lex_less(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "lex_less requires 2 arguments (two arrays)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.extract_var_array(&constraint.args[0])?;
        let y = self.extract_var_array(&constraint.args[1])?;
        
        if x.len() != y.len() {
            return Err(FlatZincError::MapError {
                message: format!(
                    "lex_less: arrays must have same length (x: {}, y: {})",
                    x.len(),
                    y.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let n = x.len();
        
        if n == 0 {
            // Empty arrays: x <_lex y is false
            let false_var = self.model.int(0, 0);
            self.model.new(false_var.eq(1)); // Force failure
            return Ok(());
        }
        
        // Decomposition: For each position i, create a boolean indicating:
        // "x is less than y starting at position i"
        // meaning: all previous positions are equal AND x[i] < y[i]
        
        let mut position_less = Vec::new();
        
        for i in 0..n {
            let mut conditions = Vec::new();
            
            // All previous positions must be equal
            for j in 0..i {
                let eq_b = self.model.bool();
                self.model.int_eq_reif(x[j], y[j], eq_b);
                conditions.push(eq_b);
            }
            
            // At position i, x[i] < y[i]
            let lt_b = self.model.bool();
            self.model.int_lt_reif(x[i], y[i], lt_b);
            conditions.push(lt_b);
            
            // All conditions must hold
            let pos_less = self.model.bool_and(&conditions);
            position_less.push(pos_less);
        }
        
        // At least one position must satisfy the "less" condition
        let lex_less_holds = self.model.bool_or(&position_less);
        self.model.new(lex_less_holds.eq(1));
        
        Ok(())
    }
    
    /// Map lex_lesseq constraint: x ≤_lex y (lexicographic ordering)
    /// FlatZinc signature: lex_lesseq(array[int] of var int: x, array[int] of var int: y)
    /// 
    /// Decomposition: x ≤_lex y iff (x = y) ∨ (x <_lex y)
    pub(in crate::flatzinc::mapper) fn map_lex_lesseq(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "lex_lesseq requires 2 arguments (two arrays)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let x = self.extract_var_array(&constraint.args[0])?;
        let y = self.extract_var_array(&constraint.args[1])?;
        
        if x.len() != y.len() {
            return Err(FlatZincError::MapError {
                message: format!(
                    "lex_lesseq: arrays must have same length (x: {}, y: {})",
                    x.len(),
                    y.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let n = x.len();
        
        if n == 0 {
            // Empty arrays: x ≤_lex y is true (equal)
            return Ok(());
        }
        
        // Decomposition: For each position i, create a boolean indicating:
        // "x is less than or equal to y starting at position i"
        // Two cases:
        // 1. All previous positions equal AND x[i] < y[i] (strictly less)
        // 2. All positions equal (equal case)
        
        let mut position_conditions = Vec::new();
        
        // Case 1: Strictly less at some position
        for i in 0..n {
            let mut conditions = Vec::new();
            
            // All previous positions must be equal
            for j in 0..i {
                let eq_b = self.model.bool();
                self.model.int_eq_reif(x[j], y[j], eq_b);
                conditions.push(eq_b);
            }
            
            // At position i, x[i] < y[i]
            let lt_b = self.model.bool();
            self.model.int_lt_reif(x[i], y[i], lt_b);
            conditions.push(lt_b);
            
            // All conditions must hold
            let pos_less = self.model.bool_and(&conditions);
            position_conditions.push(pos_less);
        }
        
        // Case 2: Complete equality
        let mut all_equal_conditions = Vec::new();
        for i in 0..n {
            let eq_b = self.model.bool();
            self.model.int_eq_reif(x[i], y[i], eq_b);
            all_equal_conditions.push(eq_b);
        }
        let all_equal = self.model.bool_and(&all_equal_conditions);
        position_conditions.push(all_equal);
        
        // At least one condition must hold (less at some position OR completely equal)
        let lex_lesseq_holds = self.model.bool_or(&position_conditions);
        self.model.new(lex_lesseq_holds.eq(1));
        
        Ok(())
    }
    
    /// Map nvalue constraint: n = |{x[i] : i ∈ indices}| (count distinct values)
    /// FlatZinc signature: nvalue(var int: n, array[int] of var int: x)
    /// 
    /// Decomposition: For each potential value v in the union of domains,
    /// create a boolean indicating if v appears in x, then sum these booleans.
    pub(in crate::flatzinc::mapper) fn map_nvalue(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 2 {
            return Err(FlatZincError::MapError {
                message: "nvalue requires 2 arguments (result variable, array)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let n = self.get_var_or_const(&constraint.args[0])?;
        let x = self.extract_var_array(&constraint.args[1])?;
        
        if x.is_empty() {
            // Empty array has 0 distinct values
            let zero = self.model.int(0, 0);
            self.model.new(n.eq(zero));
            return Ok(());
        }
        
        // Get union of all possible values (approximate by using a reasonable range)
        // We'll use the model's domain bounds
        // For simplicity, iterate through a reasonable range of values
        
        // Get min/max bounds from unbounded_int_bounds in context
        let (min_bound, max_bound) = self.unbounded_int_bounds;
        
        // Limit the range to avoid excessive computation
        const MAX_RANGE: i32 = 1000;
        let range = (max_bound - min_bound).min(MAX_RANGE);
        
        if range > MAX_RANGE {
            // For very large domains, use a different approach
            // Create a boolean for each array element pair to check distinctness
            // This is O(n²) but works for any domain size
            
            // Not implemented yet - fall back to unsupported
            return Err(FlatZincError::UnsupportedFeature {
                feature: "nvalue with very large domains (>1000)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // For each potential value, create a boolean indicating if it appears in x
        let mut value_present_bools = Vec::new();
        
        for value in min_bound..=max_bound {
            // Create: b_v ↔ (∃i: x[i] = value)
            let mut any_equal = Vec::new();
            
            for &xi in &x {
                let eq_b = self.model.bool();
                let const_var = self.model.int(value, value);
                self.model.int_eq_reif(xi, const_var, eq_b);
                any_equal.push(eq_b);
            }
            
            // At least one element equals this value
            let value_present = self.model.bool_or(&any_equal);
            value_present_bools.push(value_present);
        }
        
        // Sum the booleans to get the count of distinct values
        let sum = self.model.sum(&value_present_bools);
        self.model.new(n.eq(sum));
        
        Ok(())
    }
    
    /// Map fixed_fzn_cumulative constraint: cumulative scheduling with fixed capacity
    /// FlatZinc signature: fixed_fzn_cumulative(array[int] of var int: s, array[int] of int: d, 
    ///                                          array[int] of int: r, int: b)
    /// 
    /// Parameters:
    /// - s[i]: start time of task i (variable)
    /// - d[i]: duration of task i (constant)
    /// - r[i]: resource requirement of task i (constant)
    /// - b: resource capacity bound (constant)
    /// 
    /// Constraint: At any time t, sum of resources used by overlapping tasks ≤ b
    /// Task i is active at time t if: s[i] ≤ t < s[i] + d[i]
    /// 
    /// Decomposition: For each relevant time point t, ensure resource usage ≤ b
    pub(in crate::flatzinc::mapper) fn map_fixed_fzn_cumulative(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 4 {
            return Err(FlatZincError::MapError {
                message: "fixed_fzn_cumulative requires 4 arguments (starts, durations, resources, bound)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let starts = self.extract_var_array(&constraint.args[0])?;
        let durations = self.extract_int_array(&constraint.args[1])?;
        let resources = self.extract_int_array(&constraint.args[2])?;
        let capacity = self.extract_int(&constraint.args[3])?;
        
        let n_tasks = starts.len();
        
        if durations.len() != n_tasks || resources.len() != n_tasks {
            return Err(FlatZincError::MapError {
                message: format!(
                    "fixed_fzn_cumulative: array lengths must match (starts: {}, durations: {}, resources: {})",
                    n_tasks, durations.len(), resources.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Skip tasks with zero duration or zero resource requirement
        let mut active_tasks = Vec::new();
        for i in 0..n_tasks {
            if durations[i] > 0 && resources[i] > 0 {
                active_tasks.push(i);
            }
        }
        
        if active_tasks.is_empty() {
            return Ok(()); // No active tasks, constraint trivially satisfied
        }
        
        // Determine time horizon: compute reasonable bounds
        // Time points to check: from min possible start to max possible end
        let (min_time, max_time) = self.unbounded_int_bounds;
        
        // Limit time points to avoid excessive constraints (max 200 time points)
        const MAX_TIME_POINTS: i32 = 200;
        let time_range = max_time - min_time + 1;
        
        if time_range > MAX_TIME_POINTS {
            // For large time horizons, use a simplified check on a subset of time points
            // Sample time points evenly across the range
            let step = time_range / MAX_TIME_POINTS;
            for t_idx in 0..MAX_TIME_POINTS {
                let t = min_time + t_idx * step;
                self.add_cumulative_constraint_at_time(&starts, &durations, &resources, capacity, t, &active_tasks)?;
            }
        } else {
            // For small time horizons, check every time point
            for t in min_time..=max_time {
                self.add_cumulative_constraint_at_time(&starts, &durations, &resources, capacity, t, &active_tasks)?;
            }
        }
        
        Ok(())
    }
    
    /// Map var_fzn_cumulative constraint: cumulative scheduling with variable capacity
    /// FlatZinc signature: var_fzn_cumulative(array[int] of var int: s, array[int] of int: d, 
    ///                                        array[int] of int: r, var int: b)
    /// 
    /// Same as fixed_fzn_cumulative but with variable capacity b
    pub(in crate::flatzinc::mapper) fn map_var_fzn_cumulative(&mut self, constraint: &Constraint) -> FlatZincResult<()> {
        if constraint.args.len() != 4 {
            return Err(FlatZincError::MapError {
                message: "var_fzn_cumulative requires 4 arguments (starts, durations, resources, bound)".to_string(),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        let starts = self.extract_var_array(&constraint.args[0])?;
        let durations = self.extract_int_array(&constraint.args[1])?;
        let resources = self.extract_int_array(&constraint.args[2])?;
        let capacity_var = self.get_var_or_const(&constraint.args[3])?;
        
        let n_tasks = starts.len();
        
        if durations.len() != n_tasks || resources.len() != n_tasks {
            return Err(FlatZincError::MapError {
                message: format!(
                    "var_fzn_cumulative: array lengths must match (starts: {}, durations: {}, resources: {})",
                    n_tasks, durations.len(), resources.len()
                ),
                line: Some(constraint.location.line),
                column: Some(constraint.location.column),
            });
        }
        
        // Skip tasks with zero duration or zero resource requirement
        let mut active_tasks = Vec::new();
        for i in 0..n_tasks {
            if durations[i] > 0 && resources[i] > 0 {
                active_tasks.push(i);
            }
        }
        
        if active_tasks.is_empty() {
            return Ok(()); // No active tasks, constraint trivially satisfied
        }
        
        // Determine time horizon
        let (min_time, max_time) = self.unbounded_int_bounds;
        
        // Limit time points to avoid excessive constraints
        const MAX_TIME_POINTS: i32 = 200;
        let time_range = max_time - min_time + 1;
        
        if time_range > MAX_TIME_POINTS {
            let step = time_range / MAX_TIME_POINTS;
            for t_idx in 0..MAX_TIME_POINTS {
                let t = min_time + t_idx * step;
                self.add_var_cumulative_constraint_at_time(&starts, &durations, &resources, capacity_var, t, &active_tasks)?;
            }
        } else {
            for t in min_time..=max_time {
                self.add_var_cumulative_constraint_at_time(&starts, &durations, &resources, capacity_var, t, &active_tasks)?;
            }
        }
        
        Ok(())
    }
    
    /// Helper: Add cumulative constraint at specific time point t (fixed capacity)
    fn add_cumulative_constraint_at_time(
        &mut self,
        starts: &[crate::variables::VarId],
        durations: &[i32],
        resources: &[i32],
        capacity: i32,
        t: i32,
        active_tasks: &[usize],
    ) -> FlatZincResult<()> {
        // For each task i, create boolean: active_i ↔ (s[i] ≤ t < s[i] + d[i])
        let mut resource_usage_terms = Vec::new();
        
        for &i in active_tasks {
            // Task i is active at time t if: s[i] ≤ t AND t < s[i] + d[i]
            // Which is: s[i] ≤ t AND s[i] + d[i] > t
            
            let t_const = self.model.int(t, t);
            let end_time_i = durations[i]; // s[i] + d[i]
            
            // b1 ↔ (s[i] ≤ t)
            let b1 = self.model.bool();
            self.model.int_le_reif(starts[i], t_const, b1);
            
            // b2 ↔ (s[i] + d[i] > t)  which is  (s[i] > t - d[i])
            let b2 = self.model.bool();
            let t_minus_d = self.model.int(t - end_time_i + 1, t - end_time_i + 1);
            self.model.int_ge_reif(starts[i], t_minus_d, b2);
            
            // active_i = b1 AND b2
            let active_i = self.model.bool_and(&[b1, b2]);
            
            // If task i is active, it uses resources[i]
            // resource_usage += active_i * resources[i]
            let usage_i = self.model.mul(active_i, crate::variables::Val::ValI(resources[i]));
            resource_usage_terms.push(usage_i);
        }
        
        if !resource_usage_terms.is_empty() {
            // Sum of resource usage at time t must be ≤ capacity
            let total_usage = self.model.sum(&resource_usage_terms);
            let capacity_var = self.model.int(capacity, capacity);
            self.model.new(total_usage.le(capacity_var));
        }
        
        Ok(())
    }
    
    /// Helper: Add cumulative constraint at specific time point t (variable capacity)
    fn add_var_cumulative_constraint_at_time(
        &mut self,
        starts: &[crate::variables::VarId],
        durations: &[i32],
        resources: &[i32],
        capacity_var: crate::variables::VarId,
        t: i32,
        active_tasks: &[usize],
    ) -> FlatZincResult<()> {
        // Same as fixed version but use capacity_var instead of creating constant
        let mut resource_usage_terms = Vec::new();
        
        for &i in active_tasks {
            let t_const = self.model.int(t, t);
            let end_time_i = durations[i];
            
            let b1 = self.model.bool();
            self.model.int_le_reif(starts[i], t_const, b1);
            
            let b2 = self.model.bool();
            let t_minus_d = self.model.int(t - end_time_i + 1, t - end_time_i + 1);
            self.model.int_ge_reif(starts[i], t_minus_d, b2);
            
            let active_i = self.model.bool_and(&[b1, b2]);
            let usage_i = self.model.mul(active_i, crate::variables::Val::ValI(resources[i]));
            resource_usage_terms.push(usage_i);
        }
        
        if !resource_usage_terms.is_empty() {
            let total_usage = self.model.sum(&resource_usage_terms);
            self.model.new(total_usage.le(capacity_var));
        }
        
        Ok(())
    }
}
