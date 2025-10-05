//! Boolean constraint operations
//!
//! This module contains boolean logic operations and clause constraints:
//! - Boolean logic: and, or, not
//! - CNF/SAT: bool_clause

use crate::model::Model;
use crate::variables::{VarId, Val};

impl Model {
    #[doc(hidden)]
    /// Create a variable representing the boolean AND of multiple operands.
    /// Returns a variable that is 1 if ALL operands are non-zero, 0 otherwise.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let b = m.bool();
    /// let c = m.bool();
    /// let and_result = m.bool_and(&[a, b, c]);
    /// ```
    pub fn bool_and(&mut self, operands: &[VarId]) -> VarId {
        let result = self.bool(); // Create a boolean variable (0 or 1)
        self.props.bool_and(operands.to_vec(), result);
        result
    }

    #[doc(hidden)]
    /// Create a variable representing the boolean OR of multiple operands.
    /// Returns a variable that is 1 if ANY operand is non-zero, 0 otherwise.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let b = m.bool();
    /// let or_result = m.bool_or(&[a, b]);
    /// ```
    pub fn bool_or(&mut self, operands: &[VarId]) -> VarId {
        let result = self.bool(); // Create a boolean variable (0 or 1)
        self.props.bool_or(operands.to_vec(), result);
        result
    }

    #[doc(hidden)]
    /// Create a variable representing the boolean NOT of an operand.
    /// Returns a variable that is 1 if the operand is 0, and 0 if the operand is non-zero.
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let not_a = m.bool_not(a);
    /// ```
    pub fn bool_not(&mut self, operand: VarId) -> VarId {
        let result = self.bool(); // Create a boolean variable (0 or 1)
        self.props.bool_not(operand, result);
        result
    }

    /// Post a boolean clause constraint: `(∨ pos[i]) ∨ (∨ ¬neg[i])`.
    /// 
    /// This implements the FlatZinc `bool_clause` constraint, which represents
    /// a clause in CNF (Conjunctive Normal Form). The clause is satisfied if:
    /// - At least one positive literal is true, OR
    /// - At least one negative literal is false
    /// 
    /// In other words: `pos[0] ∨ pos[1] ∨ ... ∨ ¬neg[0] ∨ ¬neg[1] ∨ ...`
    /// 
    /// # Arguments
    /// * `pos` - Array of positive boolean literals (variables that should be true)
    /// * `neg` - Array of negative boolean literals (variables that should be false)
    /// 
    /// # Examples
    /// ```
    /// use selen::prelude::*;
    /// let mut m = Model::default();
    /// let a = m.bool();
    /// let b = m.bool();
    /// let c = m.bool();
    /// 
    /// // At least one of: a is true, b is true, or c is false
    /// // Equivalent to: a ∨ b ∨ ¬c
    /// m.bool_clause(&[a, b], &[c]);
    /// ```
    /// 
    /// # Implementation
    /// 
    /// The clause is decomposed as:
    /// 1. If both arrays are empty, the clause is unsatisfiable (posts false)
    /// 2. Otherwise, we create: `(∨ pos[i]) ∨ (∨ ¬neg[i]) = true`
    ///    - This ensures at least one positive literal is 1, or one negative literal is 0
    pub fn bool_clause(&mut self, pos: &[VarId], neg: &[VarId]) {
        // Empty clause is unsatisfiable
        if pos.is_empty() && neg.is_empty() {
            // Post an unsatisfiable constraint: 0 = 1
            self.props.equals(Val::ValI(0), Val::ValI(1));
            return;
        }

        // Special case: only positive literals
        if neg.is_empty() {
            // At least one positive literal must be true: bool_or(pos) = 1
            let clause_result = self.bool_or(pos);
            self.props.equals(clause_result, Val::ValI(1));
            return;
        }

        // Special case: only negative literals
        if pos.is_empty() {
            // At least one negative literal must be false
            // ¬neg[0] ∨ ¬neg[1] ∨ ... = ¬(neg[0] ∧ neg[1] ∧ ...)
            let all_neg = self.bool_and(neg);
            let not_all_neg = self.bool_not(all_neg);
            self.props.equals(not_all_neg, Val::ValI(1));
            return;
        }

        // General case: both positive and negative literals
        // pos[0] ∨ ... ∨ ¬neg[0] ∨ ...
        // = (pos[0] ∨ ... ∨ pos[n]) ∨ (¬neg[0] ∨ ... ∨ ¬neg[m])
        // = (∨ pos[i]) ∨ ¬(∧ neg[i])
        
        let pos_clause = self.bool_or(pos);
        let all_neg = self.bool_and(neg);
        let not_all_neg = self.bool_not(all_neg);
        
        // At least one side must be true
        let final_clause = self.bool_or(&[pos_clause, not_all_neg]);
        self.props.equals(final_clause, Val::ValI(1));
    }
}
