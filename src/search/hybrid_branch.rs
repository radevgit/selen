use crate::props::PropId;
use crate::search::Space;
use crate::vars::{Var};
use crate::search::value_branch::ValueBasedBranching;
use crate::search::branch::SplitOnUnassigned;

/// Hybrid branching strategy that intelligently chooses between:
/// - Value-based branching for float variables (reduces search tree)
/// - Domain splitting for integer variables (traditional approach)
pub struct HybridBranching {
    state: HybridState,
}

#[derive(Debug)]
enum HybridState {
    /// Look for variables to branch on
    FindStrategy(Space),
    /// Use value-based branching
    ValueBased(ValueBasedBranching),
    /// Use domain splitting  
    DomainSplit(SplitOnUnassigned),
    /// No more branches
    Done,
}

impl HybridBranching {
    /// Create a new hybrid branching strategy
    pub fn new(space: Space) -> Self {
        Self {
            state: HybridState::FindStrategy(space),
        }
    }
    
    /// Analyze the space and choose the best branching strategy
    fn choose_strategy(&self, space: Space) -> HybridState {
        // Find the first unassigned variable to determine strategy
        if let Some(var) = space.vars.get_unassigned_var() {
            match &space.vars[var] {
                Var::VarI { .. } => {
                    // For integers, use traditional domain splitting
                    // This works well since integer domains are discrete
                    HybridState::DomainSplit(crate::search::branch::split_on_unassigned(space))
                }
                Var::VarF { .. } => {
                    // For floats, use value-based branching to reduce search tree
                    // This avoids creating many narrow float intervals
                    HybridState::ValueBased(ValueBasedBranching::new(space))
                }
            }
        } else {
            // No unassigned variables
            HybridState::Done
        }
    }
    
    /// Get the current propagation count
    pub fn get_propagation_count(&self) -> usize {
        match &self.state {
            HybridState::FindStrategy(space) => space.get_propagation_count(),
            HybridState::ValueBased(vb) => vb.get_propagation_count(),
            HybridState::DomainSplit(ds) => ds.get_propagation_count(),
            HybridState::Done => 0,
        }
    }

    /// Get the current node count
    pub fn get_node_count(&self) -> usize {
        match &self.state {
            HybridState::FindStrategy(space) => space.get_node_count(),
            HybridState::ValueBased(vb) => vb.get_node_count(),
            HybridState::DomainSplit(ds) => ds.get_node_count(),
            HybridState::Done => 0,
        }
    }
}

impl Iterator for HybridBranching {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.state {
                HybridState::FindStrategy(_) => {
                    // Choose strategy and transition
                    let space = match std::mem::replace(&mut self.state, HybridState::Done) {
                        HybridState::FindStrategy(space) => space,
                        _ => unreachable!(),
                    };
                    
                    self.state = self.choose_strategy(space);
                    // Continue to execute the chosen strategy
                }
                HybridState::ValueBased(value_branching) => {
                    if let Some(result) = value_branching.next() {
                        return Some(result);
                    } else {
                        // Value-based branching is exhausted
                        self.state = HybridState::Done;
                        return None;
                    }
                }
                HybridState::DomainSplit(domain_splitting) => {
                    if let Some(result) = domain_splitting.next() {
                        return Some(result);
                    } else {
                        // Domain splitting is exhausted
                        self.state = HybridState::Done;
                        return None;
                    }
                }
                HybridState::Done => {
                    return None;
                }
            }
        }
    }
}

/// Create a hybrid branching strategy that chooses the best approach per variable type
pub fn split_with_hybrid_strategy(space: Space) -> HybridBranching {
    HybridBranching::new(space)
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use crate::vars::Val;

    #[test]
    fn test_hybrid_branching_mixed_types() {
        let mut m = Model::default();
        
        // Mixed variable types
        let int_var = m.new_var(Val::ValI(1), Val::ValI(3));
        let float_var = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
        
        // Add constraint
        m.not_equals(int_var, float_var);
        
        let solutions: Vec<_> = m.enumerate().take(10).collect();
        
        // Should find solutions efficiently
        assert!(!solutions.is_empty());
        
        // Verify constraints are satisfied
        for solution in &solutions {
            let int_val = solution[int_var];
            let float_val = solution[float_var];
            assert_ne!(int_val, float_val, "Solution should satisfy constraint");
        }
    }
    
    #[test]
    fn test_hybrid_branching_all_floats() {
        let mut m = Model::default();
        
        let x = m.new_var(Val::ValF(0.0), Val::ValF(1.0));
        let y = m.new_var(Val::ValF(0.0), Val::ValF(1.0));
        
        // This should use value-based branching for both variables
        m.not_equals(x, y);
        
        let solutions: Vec<_> = m.enumerate().take(10).collect();
        assert!(!solutions.is_empty());
        
        for solution in &solutions {
            let x_val = solution[x];
            let y_val = solution[y];
            assert_ne!(x_val, y_val);
        }
    }
    
    #[test]
    fn test_hybrid_branching_all_ints() {
        let mut m = Model::default();
        
        let x = m.new_var(Val::ValI(1), Val::ValI(5));
        let y = m.new_var(Val::ValI(1), Val::ValI(5));
        
        // This should use domain splitting for both variables
        m.not_equals(x, y);
        
        // Limit solutions to avoid infinite enumeration
        let solutions: Vec<_> = m.enumerate().take(10).collect();
        assert!(!solutions.is_empty());
        
        for solution in &solutions {
            let x_val = solution[x];
            let y_val = solution[y];
            assert_ne!(x_val, y_val);
        }
    }
}
