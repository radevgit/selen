//! Custom branching strategy for not-equals constraints
//! 
//! This demonstrates how we could implement domain splitting to handle
//! not-equals constraints more efficiently by creating "holes" through branching.

use crate::{vars::{VarId, Val}, search::Space, props::PropId};

/// A branching strategy that specifically handles not-equals constraints
/// by splitting domains around forbidden values
pub struct SplitAroundForbiddenValue {
    /// The variable to split
    var: VarId,
    /// The forbidden value to split around
    forbidden_value: Val,
    /// Current branching state
    state: SplitState,
}

#[derive(Debug, Clone)]
enum SplitState {
    /// Ready to create left branch (x < forbidden_value)
    ReadyLeft(Space),
    /// Ready to create right branch (x > forbidden_value)  
    ReadyRight(Space),
    /// Done with both branches
    Done,
}

impl SplitAroundForbiddenValue {
    /// Create a new splitter that will split var's domain around forbidden_value
    pub fn new(space: Space, var: VarId, forbidden_value: Val) -> Self {
        Self {
            var,
            forbidden_value,
            state: SplitState::ReadyLeft(space),
        }
    }
}

impl Iterator for SplitAroundForbiddenValue {
    type Item = (Space, PropId);
    
    fn next(&mut self) -> Option<Self::Item> {
        match std::mem::replace(&mut self.state, SplitState::Done) {
            SplitState::ReadyLeft(space) => {
                // Create left branch: x < forbidden_value
                let mut left_space = space.clone();
                
                // Determine appropriate delta based on variable type
                let var_domain = &left_space.vars[self.var];
                let delta = match (var_domain, self.forbidden_value) {
                    (crate::vars::Var::VarI { .. }, Val::ValI(_)) => Val::ValI(1),
                    _ => Val::ValF(crate::prelude::VAR_EPSILON),
                };
                
                // Add constraint: x <= forbidden_value - delta (i.e., x < forbidden_value)
                let upper_bound = match (self.forbidden_value, delta) {
                    (Val::ValI(fv), Val::ValI(d)) => Val::ValI(fv - d),
                    (Val::ValF(fv), Val::ValF(d)) => Val::ValF(fv - d),
                    (Val::ValI(fv), Val::ValF(d)) => Val::ValF(fv as f32 - d),
                    (Val::ValF(fv), Val::ValI(d)) => Val::ValF(fv - d as f32),
                };
                
                let prop_id = left_space.props.less_than_or_equals(self.var, upper_bound);
                
                // Prepare for right branch
                self.state = SplitState::ReadyRight(space);
                
                Some((left_space, prop_id))
            }
            SplitState::ReadyRight(space) => {
                // Create right branch: x > forbidden_value
                let mut right_space = space;
                
                // Determine appropriate delta based on variable type
                let var_domain = &right_space.vars[self.var];
                let delta = match (var_domain, self.forbidden_value) {
                    (crate::vars::Var::VarI { .. }, Val::ValI(_)) => Val::ValI(1),
                    _ => Val::ValF(crate::prelude::VAR_EPSILON),
                };
                
                // Add constraint: x >= forbidden_value + delta (i.e., x > forbidden_value)
                let lower_bound = match (self.forbidden_value, delta) {
                    (Val::ValI(fv), Val::ValI(d)) => Val::ValI(fv + d),
                    (Val::ValF(fv), Val::ValF(d)) => Val::ValF(fv + d),
                    (Val::ValI(fv), Val::ValF(d)) => Val::ValF(fv as f32 + d),
                    (Val::ValF(fv), Val::ValI(d)) => Val::ValF(fv + d as f32),
                };
                
                let mut events = Vec::new();
                let ctx = crate::views::Context::new(&mut right_space.vars, &mut events);
                let prop_id = right_space.props.greater_than_or_equals(self.var, lower_bound);
                
                Some((right_space, prop_id))
            }
            SplitState::Done => None,
        }
    }
}

/// Example of how this could be used for enhanced not-equals constraints
pub fn example_usage() {
    println!("Example: Creating domain holes through branching");
    println!("================================================");
    
    println!("Problem: x ∈ [1, 5], x ≠ 3");
    println!();
    println!("Traditional approach:");
    println!("  - Single domain: [1, 5] with constraint x ≠ 3");
    println!("  - Relies on propagation when x becomes singleton");
    println!();
    println!("Enhanced branching approach:");
    println!("  - Branch 1: x < 3  → x ∈ [1, 2]");
    println!("  - Branch 2: x > 3  → x ∈ [4, 5]");
    println!("  - Effectively creates domain {{1, 2, 4, 5}} through branching");
    println!();
    println!("Benefits:");
    println!("  ✓ More aggressive constraint propagation");
    println!("  ✓ Earlier detection of conflicts");
    println!("  ✓ Better constraint interaction");
    println!("  ✓ Maintains interval domain representation");
}

/// Enhanced not-equals constraint that suggests custom branching
pub struct EnhancedNotEquals<U, V> {
    x: U,
    y: V,
    /// Track if we should suggest branching around specific values
    suggested_branches: Vec<(VarId, Val)>,
}

impl<U, V> EnhancedNotEquals<U, V> {
    pub fn new(x: U, y: V) -> Self {
        Self {
            x,
            y,
            suggested_branches: Vec::new(),
        }
    }
    
    /// Get suggested branching points for this constraint
    pub fn get_suggested_branches(&self) -> &[(VarId, Val)] {
        &self.suggested_branches
    }
}

// Implementation would go here...
// This shows the concept of how we could enhance the constraint system
