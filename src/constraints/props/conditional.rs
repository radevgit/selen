use crate::{
    constraints::props::{Prune, Propagate},
    variables::{Val, VarId},
    variables::views::{Context, View},
};

/// Represents a simple condition for if-then-else constraints
#[derive(Debug, Clone)]
#[doc(hidden)]
pub enum Condition {
    /// Variable equals a specific value
    Equals(VarId, Val),
    /// Variable not equals a specific value  
    NotEquals(VarId, Val),
    /// Variable is greater than a value
    GreaterThan(VarId, Val),
    /// Variable is less than a value
    LessThan(VarId, Val),
}

impl Condition {
    /// Check if condition is definitely true given current domains
    fn is_definitely_true(&self, ctx: &Context) -> bool {
        match self {
            Condition::Equals(var, value) => {
                let min = var.min(ctx);
                let max = var.max(ctx);
                min == max && min == *value
            }
            Condition::NotEquals(var, value) => {
                let min = var.min(ctx);
                let max = var.max(ctx);
                max < *value || min > *value
            }
            Condition::GreaterThan(var, value) => {
                let min = var.min(ctx);
                min > *value
            }
            Condition::LessThan(var, value) => {
                let max = var.max(ctx);
                max < *value
            }
        }
    }

    /// Check if condition is definitely false given current domains
    fn is_definitely_false(&self, ctx: &Context) -> bool {
        match self {
            Condition::Equals(var, value) => {
                let min = var.min(ctx);
                let max = var.max(ctx);
                max < *value || min > *value
            }
            Condition::NotEquals(var, value) => {
                let min = var.min(ctx);
                let max = var.max(ctx);
                min == max && min == *value
            }
            Condition::GreaterThan(var, value) => {
                let max = var.max(ctx);
                max <= *value
            }
            Condition::LessThan(var, value) => {
                let min = var.min(ctx);
                min >= *value
            }
        }
    }

    /// Get all variables involved in this condition
    fn variables(&self) -> Vec<VarId> {
        match self {
            Condition::Equals(var, _) |
            Condition::NotEquals(var, _) |
            Condition::GreaterThan(var, _) |
            Condition::LessThan(var, _) => vec![*var],
        }
    }
}

/// Simple constraint representation for then/else branches
#[derive(Debug, Clone)]
#[doc(hidden)]
pub enum SimpleConstraint {
    /// Variable equals value
    Equals(VarId, Val),
    /// Variable not equals value
    NotEquals(VarId, Val),
    /// Variable greater than value
    GreaterThan(VarId, Val),
    /// Variable less than value
    LessThan(VarId, Val),
    /// Variable greater than or equal to value
    GreaterOrEqual(VarId, Val),
    /// Variable less than or equal to value
    LessOrEqual(VarId, Val),
}

impl SimpleConstraint {
    /// Apply this constraint to domains
    fn apply(&self, ctx: &mut Context) -> Option<()> {
        match self {
            SimpleConstraint::Equals(var, value) => {
                var.try_set_min(*value, ctx)?;
                var.try_set_max(*value, ctx)?;
            }
            SimpleConstraint::NotEquals(var, value) => {
                // Limited ability to enforce not-equals with min/max API
                if var.min(ctx) == *value {
                    var.try_set_min(*value + Val::ValI(1), ctx)?;
                } else if var.max(ctx) == *value {
                    var.try_set_max(*value - Val::ValI(1), ctx)?;
                }
                // For values in the middle, we can't easily remove them
            }
            SimpleConstraint::GreaterThan(var, value) => {
                var.try_set_min(*value + Val::ValI(1), ctx)?;
            }
            SimpleConstraint::LessThan(var, value) => {
                var.try_set_max(*value - Val::ValI(1), ctx)?;
            }
            SimpleConstraint::GreaterOrEqual(var, value) => {
                var.try_set_min(*value, ctx)?;
            }
            SimpleConstraint::LessOrEqual(var, value) => {
                var.try_set_max(*value, ctx)?;
            }
        }
        Some(())
    }

    /// Get all variables involved in this constraint
    fn variables(&self) -> Vec<VarId> {
        match self {
            SimpleConstraint::Equals(var, _) |
            SimpleConstraint::NotEquals(var, _) |
            SimpleConstraint::GreaterThan(var, _) |
            SimpleConstraint::LessThan(var, _) |
            SimpleConstraint::GreaterOrEqual(var, _) |
            SimpleConstraint::LessOrEqual(var, _) => vec![*var],
        }
    }
}

/// If-then-else constraint: if condition then constraint1 else constraint2
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct IfThenElseConstraint {
    condition: Condition,
    then_constraint: SimpleConstraint,
    else_constraint: Option<SimpleConstraint>, // else is optional
}

impl IfThenElseConstraint {
    pub fn new(
        condition: Condition,
        then_constraint: SimpleConstraint,
        else_constraint: Option<SimpleConstraint>,
    ) -> Self {
        IfThenElseConstraint {
            condition,
            then_constraint,
            else_constraint,
        }
    }

    /// Create a simple if-then constraint (without else)
    pub fn if_then(condition: Condition, then_constraint: SimpleConstraint) -> Self {
        Self::new(condition, then_constraint, None)
    }

    /// Get all variables involved in this constraint
    pub fn variables(&self) -> Vec<VarId> {
        let mut vars = self.condition.variables();
        vars.extend(self.then_constraint.variables());
        if let Some(ref else_constraint) = self.else_constraint {
            vars.extend(else_constraint.variables());
        }
        vars.dedup();
        vars
    }
}

impl Prune for IfThenElseConstraint {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        if self.condition.is_definitely_true(ctx) {
            // Condition is true, apply then constraint
            self.then_constraint.apply(ctx)?;
        } else if self.condition.is_definitely_false(ctx) {
            // Condition is false, apply else constraint if present
            if let Some(ref else_constraint) = self.else_constraint {
                else_constraint.apply(ctx)?;
            }
        }
        // If condition is neither definitely true nor false, we can't propagate yet

        Some(())
    }
}

impl Propagate for IfThenElseConstraint {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_conditional_constraint_creation() {
        let mut m = Model::default();
        let x = m.int(1, 1);     // condition variable: must be 1
        let y = m.int(0, 10);    // then variable: 0..=10

        // Test creating constraints
        let condition = Condition::Equals(x, Val::ValI(1));
        let then_constraint = SimpleConstraint::Equals(y, Val::ValI(5));
        let _constraint = IfThenElseConstraint::if_then(condition, then_constraint);
        
        // Just test that we can create the constraint successfully
        assert!(_constraint.variables().len() >= 2);
    }

    #[test]
    fn test_conditional_helper_methods() {
        let mut m = Model::default();
        let x = m.int(0, 1);
        let y = m.int(0, 10);

        // Test the helper methods are accessible
        let condition = Condition::Equals(x, Val::ValI(1));
        let then_constraint = SimpleConstraint::Equals(y, Val::ValI(5));
        let else_constraint = SimpleConstraint::Equals(y, Val::ValI(3));
        
        // Test if-then constraint
        let _constraint1 = IfThenElseConstraint::if_then(condition.clone(), then_constraint.clone());
        
        // Test if-then-else constraint
        let _constraint2 = IfThenElseConstraint::new(condition, then_constraint, Some(else_constraint));
        
        // Test that variables() method works
        assert!(_constraint1.variables().len() >= 2);
        assert!(_constraint2.variables().len() >= 2);
    }
}