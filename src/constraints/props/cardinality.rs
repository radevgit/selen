use crate::{
    constraints::props::{Prune, Propagate},
    variables::{Val, VarId},
    variables::views::{Context, View},
};

/// Cardinality constraint variants
#[derive(Debug, Clone)]
pub enum CardinalityType {
    AtLeast(i32),    // At least N variables are true/equal to value
    AtMost(i32),     // At most N variables are true/equal to value
    Exactly(i32),    // Exactly N variables are true/equal to value
}

/// Cardinality constraint: counts variables equal to a target value
/// Supports at_least, at_most, and exactly variants
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct CardinalityConstraint {
    variables: Vec<VarId>,
    target_value: Val,
    cardinality_type: CardinalityType,
}

impl CardinalityConstraint {
    pub fn at_least(variables: Vec<VarId>, target_value: i32, count: i32) -> Self {
        CardinalityConstraint {
            variables,
            target_value: Val::ValI(target_value),
            cardinality_type: CardinalityType::AtLeast(count),
        }
    }

    pub fn at_most(variables: Vec<VarId>, target_value: i32, count: i32) -> Self {
        CardinalityConstraint {
            variables,
            target_value: Val::ValI(target_value),
            cardinality_type: CardinalityType::AtMost(count),
        }
    }

    pub fn exactly(variables: Vec<VarId>, target_value: i32, count: i32) -> Self {
        CardinalityConstraint {
            variables,
            target_value: Val::ValI(target_value),
            cardinality_type: CardinalityType::Exactly(count),
        }
    }

    /// Count how many variables must equal target_value (lower bound)
    fn count_must_equal(&self, ctx: &Context) -> i32 {
        let mut count = 0;
        for &var_id in &self.variables {
            let min = var_id.min(ctx);
            let max = var_id.max(ctx);
            // Variable must equal target if domain contains only target_value
            if min == max && min == self.target_value {
                count += 1;
            }
        }
        count
    }

    /// Count how many variables can equal target_value (upper bound)
    fn count_can_equal(&self, ctx: &Context) -> i32 {
        let mut count = 0;
        for &var_id in &self.variables {
            let min = var_id.min(ctx);
            let max = var_id.max(ctx);
            // Variable can equal target if domain contains target_value
            if min <= self.target_value && self.target_value <= max {
                count += 1;
            }
        }
        count
    }
}

impl Prune for CardinalityConstraint {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let must_equal = self.count_must_equal(ctx);
        let can_equal = self.count_can_equal(ctx);

        match &self.cardinality_type {
            CardinalityType::AtLeast(required) => {
                // If we already have enough variables that must equal target, 
                // no additional constraints needed
                if must_equal >= *required {
                    return Some(());
                }

                // If we can't possibly reach the required count, fail
                if can_equal < *required {
                    return None;
                }

                // If we need exactly (required - must_equal) more variables,
                // and we have exactly that many candidates, force them all
                let needed = *required - must_equal;
                let candidates = can_equal - must_equal;
                
                if needed == candidates && needed > 0 {
                    for &var_id in &self.variables {
                        let min = var_id.min(ctx);
                        let max = var_id.max(ctx);
                        if min <= self.target_value && self.target_value <= max && min != max {
                            // Force this variable to equal target_value
                            var_id.try_set_min(self.target_value, ctx)?;
                            var_id.try_set_max(self.target_value, ctx)?;
                        }
                    }
                }
            }

            CardinalityType::AtMost(limit) => {
                // If we already have too many variables that must equal target, fail
                if must_equal > *limit {
                    return None;
                }

                // If we've reached the limit, prevent any more variables from equaling target
                if must_equal == *limit {
                    for &var_id in &self.variables {
                        let min = var_id.min(ctx);
                        let max = var_id.max(ctx);
                        if min <= self.target_value && self.target_value <= max && min != max {
                            // Remove target_value from this variable's domain
                            if self.target_value == min {
                                var_id.try_set_min(self.target_value + Val::ValI(1), ctx)?;
                            } else if self.target_value == max {
                                var_id.try_set_max(self.target_value - Val::ValI(1), ctx)?;
                            }
                            // For values in the middle, we can't easily remove them with min/max
                            // This is a limitation of the current API
                        }
                    }
                }
            }

            CardinalityType::Exactly(target) => {
                // Exactly is equivalent to AtLeast(target) AND AtMost(target)
                
                // Apply AtLeast logic
                if must_equal > *target {
                    return None;
                }

                if can_equal < *target {
                    return None;
                }

                // If we need exactly (target - must_equal) more variables
                let needed = *target - must_equal;
                let candidates = can_equal - must_equal;

                if needed == candidates && needed > 0 {
                    // Force all candidates to equal target_value
                    for &var_id in &self.variables {
                        let min = var_id.min(ctx);
                        let max = var_id.max(ctx);
                        if min <= self.target_value && self.target_value <= max && min != max {
                            var_id.try_set_min(self.target_value, ctx)?;
                            var_id.try_set_max(self.target_value, ctx)?;
                        }
                    }
                } else if needed == 0 {
                    // We have enough, prevent any more
                    for &var_id in &self.variables {
                        let min = var_id.min(ctx);
                        let max = var_id.max(ctx);
                        if min <= self.target_value && self.target_value <= max && min != max {
                            // Remove target_value from domain (limited by min/max API)
                            if self.target_value == min {
                                var_id.try_set_min(self.target_value + Val::ValI(1), ctx)?;
                            } else if self.target_value == max {
                                var_id.try_set_max(self.target_value - Val::ValI(1), ctx)?;
                            }
                        }
                    }
                }
            }
        }

        Some(())
    }
}

impl Propagate for CardinalityConstraint {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_cardinality_constraint_creation() {
        let mut m = Model::default();
        let x = m.int(0, 1);
        let y = m.int(0, 1);
        let z = m.int(0, 1);

        // Test creating different cardinality constraints
        let _at_least = CardinalityConstraint::at_least(vec![x, y, z], 1, 2);
        let _at_most = CardinalityConstraint::at_most(vec![x, y, z], 1, 2);
        let _exactly = CardinalityConstraint::exactly(vec![x, y, z], 1, 1);
    }

    #[test]
    fn test_cardinality_helper_methods() {
        let mut m = Model::default();
        let x = m.int(0, 1);
        let y = m.int(0, 1);

        // Test the helper methods work
        m.props.at_least_constraint(vec![x, y], 1, 1);
        m.props.at_most_constraint(vec![x, y], 1, 2);
        m.props.exactly_constraint(vec![x, y], 1, 1);
    }
}