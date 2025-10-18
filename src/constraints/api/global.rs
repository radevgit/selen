//! Global constraint operations
//!
//! This module contains global constraints that operate on collections of variables:
//! - alldiff: all different constraint
//! - alleq: all equal constraint  
//! - element: array element constraint
//! - table: table constraint (valid tuples)
//! - count: count constraint
//! - between: between constraint
//! - at_least, at_most, exactly: cardinality constraints

use crate::model::Model;
use crate::variables::{VarId, Val, View};
use crate::constraints::props::PropId;
use crate::runtime_api::ModelExt;

impl Model {
    /// Global constraint: all variables must have different values.
    ///
    /// This constraint ensures that no two variables in the list have the same value.
    /// It's more efficient than posting individual != constraints between all pairs.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 5);
    /// let y = m.int(1, 5);
    /// let z = m.int(1, 5);
    /// m.alldiff(&[x, y, z]);
    /// ```
    pub fn alldiff(&mut self, vars: &[VarId]) -> PropId {
        self.props.all_different(vars.to_vec())
    }

    /// Global constraint: all variables must have the same value.
    ///
    /// This constraint ensures that all variables in the list are equal.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 10);
    /// let y = m.int(1, 10);
    /// let z = m.int(1, 10);
    /// m.alleq(&[x, y, z]);
    /// ```
    pub fn alleq(&mut self, vars: &[VarId]) -> PropId {
        self.props.all_equal(vars.to_vec())
    }

    /// Element constraint: array[index] == value.
    ///
    /// This constraint enforces that accessing the array at the given index
    /// produces the specified value. The index is a variable, making this
    /// useful for dynamic array access in constraint models.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let array = vec![m.int(1, 10), m.int(5, 15), m.int(3, 8)];
    /// let index = m.int(0, 2);  // Valid indices: 0, 1, 2
    /// let value = m.int(1, 15);
    /// m.element(&array, index, value);
    /// ```
    pub fn element(&mut self, array: &[VarId], index: VarId, value: VarId) -> PropId {
        self.props.element(array.to_vec(), index, value)
    }

    /// 2D Element constraint: matrix[row_index][col_index] == value.
    ///
    /// This constraint enforces that accessing the 2D matrix at the given row and column indices
    /// produces the specified value. Both indices are variables, making this useful for
    /// dynamic 2D array access in constraint models.
    ///
    /// The matrix is linearized row-by-row, so the linear index is: row_index * num_cols + col_index
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let matrix = m.ints_2d(3, 4, 1, 10);  // 3×4 matrix
    /// let row_idx = m.int(0, 2);             // Valid row indices: 0, 1, 2
    /// let col_idx = m.int(0, 3);             // Valid col indices: 0, 1, 2, 3
    /// let value = m.int(1, 10);
    /// m.element_2d(&matrix, row_idx, col_idx, value);
    /// ```
    pub fn element_2d(&mut self, matrix: &[Vec<VarId>], row_idx: VarId, col_idx: VarId, value: VarId) -> PropId {
        use crate::constraints::functions::{add, mul};
        use crate::lpsolver::csp_integration::LinearConstraint;
        
        // Flatten the 2D matrix into a 1D array
        let flat: Vec<VarId> = matrix.iter().flat_map(|row| row.iter().copied()).collect();
        let cols = if matrix.is_empty() { 0 } else { matrix[0].len() };
        
        if cols == 0 {
            // Edge case: empty matrix, just create a dummy constraint
            return self.props.element(flat, row_idx, value);
        }
        
        // Compute linear index: row_idx * cols + col_idx
        // Use expression builder for the computation
        let linear_idx_expr = add(mul(row_idx, cols as i32), col_idx);
        
        // Create an intermediate variable for the computed index
        let computed_idx_min = 0;
        let computed_idx_max = flat.len() as i32 - 1;
        let computed_idx = self.int(computed_idx_min, computed_idx_max);
        
        // Constraint: computed_idx = row_idx * cols + col_idx
        self.new(linear_idx_expr.eq(computed_idx));
        
        // Extract linear constraint for LP solver for early bound propagation
        // The index computation is a naturally linear constraint:
        // computed_idx - row_idx*cols - col_idx = 0
        // LP solver uses this to compute bounds on valid index combinations
        // and detect infeasibility before the CSP element propagator runs.
        let lc = LinearConstraint::equality(
            vec![1.0, -(cols as f64), -1.0],
            vec![computed_idx, row_idx, col_idx],
            0.0,
        );
        self.pending_lp_constraints.push(lc);
        
        // Use regular element constraint on flattened array with computed index
        self.props.element(flat, computed_idx, value)
    }

    /// 3D Element constraint: cube[depth_index][row_index][col_index] == value.
    ///
    /// This constraint enforces that accessing the 3D cube at the given indices
    /// produces the specified value. All three indices are variables.
    /// 
    /// The cube is linearized depth-first (then row-by-row within each depth layer),
    /// so the linear index is: depth_index * (num_rows * num_cols) + row_index * num_cols + col_index
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let cube = m.ints_3d(4, 5, 6, 1, 10);  // 4×5×6 cube
    /// let d_idx = m.int(0, 3);
    /// let r_idx = m.int(0, 4);
    /// let c_idx = m.int(0, 5);
    /// let value = m.int(1, 10);
    /// m.element_3d(&cube, d_idx, r_idx, c_idx, value);
    /// ```
    pub fn element_3d(&mut self, cube: &[Vec<Vec<VarId>>], depth_idx: VarId, row_idx: VarId, col_idx: VarId, value: VarId) -> PropId {
        use crate::constraints::functions::{add, mul};
        
        // Flatten the 3D cube into a 1D array
        let flat: Vec<VarId> = cube.iter()
            .flat_map(|matrix| matrix.iter())
            .flat_map(|row| row.iter().copied())
            .collect();
        
        let rows = if cube.is_empty() { 0 } else { cube[0].len() };
        let cols = if rows == 0 { 0 } else { cube[0][0].len() };
        
        if rows == 0 || cols == 0 {
            return self.props.element(flat, depth_idx, value);
        }
        
        // Compute linear index: depth_idx * (rows * cols) + row_idx * cols + col_idx
        let linear_idx_expr = add(
            mul(depth_idx, (rows * cols) as i32),
            add(mul(row_idx, cols as i32), col_idx)
        );
        
        // Create an intermediate variable for the computed index
        let computed_idx_min = 0;
        let computed_idx_max = flat.len() as i32 - 1;
        let computed_idx = self.int(computed_idx_min, computed_idx_max);
        
        // Constraint: computed_idx = depth_idx * (rows * cols) + row_idx * cols + col_idx
        self.new(linear_idx_expr.eq(computed_idx));
        
        // Extract linear constraint for LP solver for early bound propagation
        // The 3D index computation is also naturally linear:
        // computed_idx - depth_idx*(rows*cols) - row_idx*cols - col_idx = 0
        use crate::lpsolver::csp_integration::LinearConstraint;
        let lc = LinearConstraint::equality(
            vec![1.0, -((rows * cols) as f64), -(cols as f64), -1.0],
            vec![computed_idx, depth_idx, row_idx, col_idx],
            0.0,
        );
        self.pending_lp_constraints.push(lc);
        
        // Use regular element constraint on flattened array with computed index
        self.props.element(flat, computed_idx, value)
    }

    /// Table constraint: variables must match one of the valid tuples.
    ///
    /// This constraint enforces that the values assigned to the variables
    /// must match one of the tuples in the provided list. This is useful
    /// for encoding complex relationships or compatibility tables.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let x = m.int(1, 3);
    /// let y = m.int(1, 3);
    /// let z = m.int(1, 3);
    /// 
    /// // Valid combinations: (1,2,3), (2,1,3), (3,3,3)
    /// let tuples = vec![
    ///     vec![int(1), int(2), int(3)],
    ///     vec![int(2), int(1), int(3)],
    ///     vec![int(3), int(3), int(3)],
    /// ];
    /// m.table(&[x, y, z], tuples);
    /// ```
    pub fn table(&mut self, vars: &[VarId], tuples: Vec<Vec<Val>>) -> PropId {
        self.props.table_constraint(vars.to_vec(), tuples)
    }

    /// 2D Table constraint: matrix rows must match one of the valid tuples.
    ///
    /// This constraint enforces that each row of the 2D matrix, when read as a tuple,
    /// must match one of the valid tuples. This is useful for matrix-based constraint problems.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let matrix = m.ints_2d(3, 3, 1, 5);  // 3×3 matrix
    /// 
    /// // Each row must be one of these tuples
    /// let tuples = vec![
    ///     vec![Val::int(1), Val::int(2), Val::int(3)],
    ///     vec![Val::int(2), Val::int(3), Val::int(4)],
    /// ];
    /// m.table_2d(&matrix, tuples);
    /// ```
    pub fn table_2d(&mut self, matrix: &[Vec<VarId>], tuples: Vec<Vec<Val>>) -> Vec<PropId> {
        let mut prop_ids = Vec::with_capacity(matrix.len());
        for row in matrix {
            let prop_id = self.props.table_constraint(row.to_vec(), tuples.clone());
            prop_ids.push(prop_id);
        }
        prop_ids
    }

    /// 3D Table constraint: each 2D slice/layer must satisfy table constraints.
    ///
    /// This constraint enforces that each 2D layer of the 3D cube satisfies the table constraint.
    /// This is useful for complex 3D constraint problems where each layer has specific patterns.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let cube = m.ints_3d(4, 3, 3, 1, 5);  // 4×3×3 cube
    /// 
    /// // Each row of each layer must be one of these tuples
    /// let tuples = vec![
    ///     vec![Val::int(1), Val::int(2), Val::int(3)],
    ///     vec![Val::int(2), Val::int(3), Val::int(4)],
    /// ];
    /// m.table_3d(&cube, tuples);
    /// ```
    pub fn table_3d(&mut self, cube: &[Vec<Vec<VarId>>], tuples: Vec<Vec<Val>>) -> Vec<PropId> {
        let mut prop_ids = Vec::new();
        for matrix in cube {
            let layer_prop_ids = self.table_2d(matrix, tuples.clone());
            prop_ids.extend(layer_prop_ids);
        }
        prop_ids
    }

    /// Count constraint: count how many variables equal a target value.
    ///
    /// This constraint counts the number of variables in the list that equal
    /// the target_var and constrains the count to equal count_var.
    ///
    /// The target parameter accepts both variables and constants (impl View):
    /// - Use a VarId directly: `m.count(&vars, target_var, count)`
    /// - Use a Val constant: `m.count(&vars, Val::int(3), count)` 
    /// - Use an expression via explicit variable: `m.count(&vars, computed_var, count)`
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// use selen::variables::Val;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    /// let count = m.int(0, 3);
    /// 
    /// // Count with constant target - pass Val directly!
    /// m.count(&vars, Val::int(3), count);  // Count how many vars equal 3
    /// 
    /// // Count with variable target
    /// let var_target = m.int(1, 5);  // Variable with range
    /// m.count(&vars, var_target, count);  // Count how many vars equal var_target
    /// ```
    pub fn count(&mut self, vars: &[VarId], target_var: impl View, count_var: VarId) -> PropId {
        self.props.count_constraint(vars.to_vec(), target_var, count_var)
    }

    /// Between constraint: lower <= middle <= upper.
    ///
    /// This constraint enforces that middle is between lower and upper (inclusive).
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let lower = m.int(1, 5);
    /// let middle = m.int(1, 10);
    /// let upper = m.int(5, 15);
    /// m.between(lower, middle, upper);
    /// ```
    pub fn between(&mut self, lower: VarId, middle: VarId, upper: VarId) -> PropId {
        self.props.between_constraint(lower, middle, upper)
    }

    /// At least constraint: at least 'count' variables must equal 'target_value'.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    /// m.at_least(&vars, 3, 2);  // At least 2 variables must equal 3
    /// ```
    pub fn at_least(&mut self, vars: &[VarId], target_value: i32, count: i32) -> PropId {
        self.props.at_least_constraint(vars.to_vec(), target_value, count)
    }

    /// At most constraint: at most 'count' variables can equal 'target_value'.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    /// m.at_most(&vars, 3, 2);  // At most 2 variables can equal 3
    /// ```
    pub fn at_most(&mut self, vars: &[VarId], target_value: i32, count: i32) -> PropId {
        self.props.at_most_constraint(vars.to_vec(), target_value, count)
    }

    /// Exactly constraint: exactly 'count' variables must equal 'target_value'.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    /// m.exactly(&vars, 3, 2);  // Exactly 2 variables must equal 3
    /// ```
    pub fn exactly(&mut self, vars: &[VarId], target_value: i32, count: i32) -> PropId {
        self.props.exactly_constraint(vars.to_vec(), target_value, count)
    }

    /// Global cardinality constraint: count each value and match cardinalities.
    ///
    /// This constraint ensures that for each value in 'values', the number of
    /// variables equal to that value equals the corresponding count variable.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 3), m.int(1, 3), m.int(1, 3), m.int(1, 3)];
    /// let count1 = m.int(0, 4);  // Count of value 1
    /// let count2 = m.int(0, 4);  // Count of value 2
    /// let count3 = m.int(0, 4);  // Count of value 3
    /// m.gcc(&vars, &[1, 2, 3], &[count1, count2, count3]);
    /// ```
    pub fn gcc(&mut self, vars: &[VarId], values: &[i32], counts: &[VarId]) -> Vec<PropId> {
        let mut prop_ids = Vec::with_capacity(values.len());
        
        for (&value, &count_var) in values.iter().zip(counts.iter()) {
            // Create a fixed variable for the constant value
            let target_var = self.int(value, value);
            let prop_id = self.props.count_constraint(vars.to_vec(), target_var, count_var);
            prop_ids.push(prop_id);
        }
        
        prop_ids
    }
}
