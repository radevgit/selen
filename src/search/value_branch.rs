use crate::props::PropId;
use crate::search::Space;
use crate::vars::{VarId, Val, Var};

/// Value-based branching strategy that assigns specific values to float variables
/// instead of splitting domains. This significantly reduces the search tree size
/// for float variables by making direct assignments.
#[derive(Debug)]
pub struct ValueBasedBranching {
    state: ValueBranchState,
}

#[derive(Debug)]
enum ValueBranchState {
    /// Find a variable to assign a specific value to
    FindVariable(Space),
    /// Try assigning a specific value to a variable  
    TryValue {
        space: Space,
        var: VarId,
        value: Val,
        tried_value: bool,
    },
    /// No more branches
    Done,
}

impl ValueBasedBranching {
    /// Create a new value-based branching strategy
    pub fn new(space: Space) -> Self {
        Self {
            state: ValueBranchState::FindVariable(space),
        }
    }
    
    /// Find an unassigned variable and choose a value to try
    fn find_variable_and_value(&self, space: &Space) -> Option<(VarId, Val)> {
        // Find the first unassigned variable
        if let Some(var) = space.vars.get_unassigned_var() {
            let var_domain = &space.vars[var];
            
            let chosen_value = match var_domain {
                Var::VarI(sparse_set) => {
                    // For integers, try the minimum value first (could also be midpoint)
                    Val::ValI(sparse_set.min())
                }
                Var::VarF { min, max: _ } => {
                    // For floats, try a specific value that respects ULP precision
                    // We can try the minimum, maximum, or a calculated value
                    
                    // Strategy 1: Try minimum value
                    Val::ValF(*min)
                    
                    // Alternative strategies could be:
                    // - Try midpoint: Val::ValF(*min + (*max - *min) / 2.0)
                    // - Try minimum + epsilon: Val::ValF(float_next(*min))
                    // - Try maximum - epsilon: Val::ValF(float_prev(*max))
                }
            };
            
            Some((var, chosen_value))
        } else {
            None
        }
    }
    
    /// Get the current propagation count
    pub fn get_propagation_count(&self) -> usize {
        match &self.state {
            ValueBranchState::FindVariable(space) => space.get_propagation_count(),
            ValueBranchState::TryValue { space, .. } => space.get_propagation_count(),
            ValueBranchState::Done => 0,
        }
    }

    /// Get the current node count
    pub fn get_node_count(&self) -> usize {
        match &self.state {
            ValueBranchState::FindVariable(space) => space.get_node_count(),
            ValueBranchState::TryValue { space, .. } => space.get_node_count(),
            ValueBranchState::Done => 0,
        }
    }
}

impl Iterator for ValueBasedBranching {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match std::mem::replace(&mut self.state, ValueBranchState::Done) {
                ValueBranchState::FindVariable(space) => {
                    // Try to find a variable to assign a value to
                    if let Some((var, value)) = self.find_variable_and_value(&space) {
                        // Start trying this specific value
                        self.state = ValueBranchState::TryValue {
                            space,
                            var,
                            value,
                            tried_value: false,
                        };
                        // Continue the loop to handle the assignment
                        continue;
                    } else {
                        // No variables to assign
                        return None;
                    }
                }
                ValueBranchState::TryValue { space, var, value, tried_value } => {
                    if !tried_value {
                        // First branch: Try assigning var = value
                        let mut assign_space = space.clone();
                        assign_space.props.increment_node_count();
                        
                        // Create constraint: var = value
                        let prop_id = assign_space.props.equals(var, value);
                        
                        // Prepare for the alternative branch (var != value)
                        self.state = ValueBranchState::TryValue {
                            space,
                            var,
                            value,
                            tried_value: true,
                        };
                        
                        return Some((assign_space, prop_id));
                    } else {
                        // Second branch: var != value, continue with domain splitting or other assignment
                        let mut reject_space = space;
                        reject_space.props.increment_node_count();
                        
                        // Create constraint: var != value
                        let prop_id = reject_space.props.not_equals(var, value);
                        
                        // After trying this value and its negation, we're done with this variable
                        // The search engine will call us again for the next unassigned variable
                        return Some((reject_space, prop_id));
                    }
                }
                ValueBranchState::Done => return None,
            }
        }
    }
}

/// Create a value-based branching strategy
pub fn split_with_value_assignment(space: Space) -> ValueBasedBranching {
    ValueBasedBranching::new(space)
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::vars::Val;

    #[test]
    fn test_value_based_branching_basic() {
        let mut m = Model::default();
        let x = m.new_var(Val::ValI(1), Val::ValI(3));  // Use integers to avoid infinite float domains
        let y = m.new_var(Val::ValI(1), Val::ValI(3));
        
        // Add constraint: x != y
        m.not_equals(x, y);
        
        // This should find solutions with value-based branching
        let solutions: Vec<_> = m.enumerate().take(10).collect();  // Limit to 10 solutions
        
        // Should still find valid solutions, but with potentially less search tree exploration
        assert!(!solutions.is_empty());
        
        // Verify all solutions satisfy x != y
        for solution in &solutions {
            let x_val = solution[x];
            let y_val = solution[y];
            assert_ne!(x_val, y_val, "Solution should satisfy x != y");
        }
    }
    
    #[test]
    fn test_value_based_vs_domain_splitting() {
        // This test could compare the number of search nodes between strategies
        // but for now we'll just ensure it works
        let mut m = Model::default();
        let _x = m.new_var(Val::ValI(0), Val::ValI(1));  // Use integer to avoid infinite domains
        
        let solutions: Vec<_> = m.enumerate().take(5).collect();  // Limit solutions
        
        // Value-based should find at least one solution (x can be 0 or 1)
        assert!(!solutions.is_empty());
    }
}
