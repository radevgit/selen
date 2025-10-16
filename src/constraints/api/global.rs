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
use crate::variables::{VarId, Val};
use crate::constraints::props::PropId;

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

    /// Count constraint: count how many variables equal a target value.
    ///
    /// This constraint counts the number of variables in the list that equal
    /// the target value, and constrains the count to equal count_var.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    /// let count = m.int(0, 3);
    /// m.count(&vars, int(3), count);  // Count how many vars equal 3
    /// ```
    pub fn count(&mut self, vars: &[VarId], target_value: Val, count_var: VarId) -> PropId {
        self.props.count_constraint(vars.to_vec(), target_value, count_var)
    }

    /// Count constraint with variable target: count(vars, target_var) = count_var.
    ///
    /// This constraint counts how many variables in the array equal the target_var
    /// and constrains the count to equal count_var. This is a generalization of the
    /// count constraint that supports dynamic target values.
    ///
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let vars = vec![m.int(1, 5), m.int(1, 5), m.int(1, 5)];
    /// let target = m.int(1, 5);  // Variable target
    /// let count = m.int(0, 3);
    /// m.count_var(&vars, target, count);  // Count how many vars equal target
    /// ```
    pub fn count_var(&mut self, vars: &[VarId], target_var: VarId, count_var: VarId) -> PropId {
        self.props.count_var_constraint(vars.to_vec(), target_var, count_var)
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
            let prop_id = self.props.count_constraint(vars.to_vec(), Val::int(value), count_var);
            prop_ids.push(prop_id);
        }
        
        prop_ids
    }
}
