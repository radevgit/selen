use crate::{
    constraints::props::{Prune, Propagate},
    variables::VarId,
    variables::views::{Context, View},
};

/// Between constraint: ensures lower <= middle <= upper
/// This is a ternary constraint that enforces ordering relationships
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct BetweenConstraint {
    lower: VarId,
    middle: VarId,
    upper: VarId,
}

impl BetweenConstraint {
    pub fn new(lower: VarId, middle: VarId, upper: VarId) -> Self {
        BetweenConstraint {
            lower,
            middle,
            upper,
        }
    }
}

impl Prune for BetweenConstraint {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Get current domain bounds
        let lower_min = self.lower.min(ctx);
        let _lower_max = self.lower.max(ctx);
        let middle_min = self.middle.min(ctx);
        let middle_max = self.middle.max(ctx);
        let _upper_min = self.upper.min(ctx);
        let upper_max = self.upper.max(ctx);

        // Constraint: lower <= middle <= upper
        
        // Propagate constraints for lower variable
        // lower <= middle, so lower_max <= middle_max
        self.lower.try_set_max(middle_max, ctx)?;
        
        // Propagate constraints for middle variable  
        // lower <= middle, so middle_min >= lower_min
        self.middle.try_set_min(lower_min, ctx)?;
        // middle <= upper, so middle_max <= upper_max
        self.middle.try_set_max(upper_max, ctx)?;

        // Propagate constraints for upper variable
        // middle <= upper, so upper_min >= middle_min
        self.upper.try_set_min(middle_min, ctx)?;

        Some(())
    }
}

impl Propagate for BetweenConstraint {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.lower, self.middle, self.upper].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_between_constraint_creation() {
        let mut m = Model::default();
        let lower = m.int(1, 10);
        let middle = m.int(1, 10);
        let upper = m.int(1, 10);

        // Test that we can create a between constraint without errors
        let _constraint = BetweenConstraint::new(lower, middle, upper);
    }

    #[test]
    fn test_between_helper_method() {
        let mut m = Model::default();
        let lower = m.int(5, 10);
        let middle = m.int(1, 15);
        let upper = m.int(3, 8);

        // Test that the helper method works
        m.props.between_constraint(lower, middle, upper);
    }
}