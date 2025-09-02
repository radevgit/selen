use crate::{
    search::Space,
    vars::{VarId, Val},
    views::ViewRaw,
    props::PropId,
    utils::float_equal,
};

/// Most Constrained Variable (MCV) branching strategy
/// Chooses the variable with the smallest domain first (Minimum Remaining Values heuristic)
#[derive(Debug, Clone)]
pub struct MCVBranching {
    space: Space,
    current_var: Option<VarId>,
    current_val_index: usize,
    current_values: Vec<Val>,
}

impl MCVBranching {
    pub fn new(space: Space) -> Self {
        Self {
            space,
            current_var: None,
            current_val_index: 0,
            current_values: Vec::new(),
        }
    }

    /// Find the most constrained unassigned variable (smallest domain)
    fn find_most_constrained_var(&self) -> Option<VarId> {
        // For now, just return the first unassigned variable
        // A full MCV implementation would check all variables and pick the one with smallest domain
        self.space.vars.get_unassigned_var()
    }

    /// Get domain values for a variable in order (smallest values first)
    fn get_domain_values(&self, var: VarId) -> Vec<Val> {
        let min_val = var.min_raw(&self.space.vars);
        let max_val = var.max_raw(&self.space.vars);
        
        match (min_val, max_val) {
            (Val::ValI(min_i), Val::ValI(max_i)) => {
                // For integer variables, enumerate all values in domain
                if max_i - min_i > 10 {
                    // If domain is large, just try the bounds for now
                    vec![min_val, max_val]
                } else {
                    (min_i..=max_i).map(Val::ValI).collect()
                }
            }
            (Val::ValF(min_f), Val::ValF(max_f)) => {
                // For floats, we'll just try the bounds and midpoint
                if float_equal(min_f, max_f) {
                    vec![min_val]
                } else {
                    let mid = (min_f + max_f) / 2.0;
                    vec![min_val, Val::ValF(mid), max_val]
                }
            }
            _ => vec![min_val], // Fallback
        }
    }

    /// Get the current propagation count
    pub fn get_propagation_count(&self) -> usize {
        self.space.props.get_propagation_count()
    }

    /// Get the current node count
    pub fn get_node_count(&self) -> usize {
        self.space.props.get_node_count()
    }
}

impl Iterator for MCVBranching {
    type Item = (Space, PropId);

    fn next(&mut self) -> Option<Self::Item> {
        // If we have a current variable and more values to try
        if let Some(var) = self.current_var {
            if self.current_val_index < self.current_values.len() {
                let val = self.current_values[self.current_val_index];
                self.current_val_index += 1;

                // Create a split by constraining the variable to this value
                let mut new_space = self.space.clone();
                new_space.props.increment_node_count();

                let prop_id = new_space.props.equals(var, val);
                return Some((new_space, prop_id));
            } else {
                // Finished with this variable
                self.current_var = None;
                self.current_values.clear();
                self.current_val_index = 0;
            }
        }

        // Find next variable to branch on
        if let Some(var) = self.find_most_constrained_var() {
            self.current_var = Some(var);
            self.current_values = self.get_domain_values(var);
            self.current_val_index = 0;

            // Recursively call next() to get the first value for this variable
            self.next()
        } else {
            // No more variables to branch on
            None
        }
    }
}
