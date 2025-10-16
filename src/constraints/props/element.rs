use crate::{
    constraints::props::{Propagate, Prune},
    variables::{Val, VarId},
    variables::views::{Context, View},
};

/// Element constraint implementation
/// 
/// Enforces that array[index] = value, where:
/// - array: A vector of variables representing the array
/// - index: A variable representing the index (0-based)
/// - value: A variable that should equal array[index]
/// 
/// This constraint ensures that the value at position 'index' in the 'array'
/// equals the 'value' variable. It propagates in both directions:
/// - Changes to index/value constrain the possible values in array elements
/// - Changes to array elements constrain the possible index/value combinations
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct Element {
    array: Vec<VarId>,
    index: VarId,
    value: VarId,
}

impl Element {
    pub fn new(array: Vec<VarId>, index: VarId, value: VarId) -> Self {
        Self { array, index, value }
    }

    /// Get the effective domain of possible index values (0-based)
    fn get_valid_indices(&self, ctx: &Context) -> Vec<i32> {
        let index_min = self.index.min(ctx);
        let index_max = self.index.max(ctx);
        
        let min_idx = match index_min {
            Val::ValI(i) => i.max(0),
            Val::ValF(f) => (f.ceil() as i32).max(0),
        };
        
        let max_idx = match index_max {
            Val::ValI(i) => i.min(self.array.len() as i32 - 1),
            Val::ValF(f) => (f.floor() as i32).min(self.array.len() as i32 - 1),
        };

        if min_idx <= max_idx {
            (min_idx..=max_idx).collect()
        } else {
            vec![]
        }
    }

    /// Get the union of all possible values from valid array positions
    fn compute_possible_values(&self, ctx: &Context) -> Option<(Val, Val)> {
        let valid_indices = self.get_valid_indices(ctx);
        
        if valid_indices.is_empty() {
            return None;
        }

        let mut min_val: Option<Val> = None;
        let mut max_val: Option<Val> = None;

        for idx in valid_indices {
            if let Some(var) = self.array.get(idx as usize) {
                let var_min = var.min(ctx);
                let var_max = var.max(ctx);

                min_val = Some(match min_val {
                    None => var_min,
                    Some(current_min) => if var_min < current_min { var_min } else { current_min },
                });

                max_val = Some(match max_val {
                    None => var_max,
                    Some(current_max) => if var_max > current_max { var_max } else { current_max },
                });
            }
        }

        match (min_val, max_val) {
            (Some(min), Some(max)) => Some((min, max)),
            _ => None,
        }
    }

    /// Propagate constraints from value to array elements and index
    fn propagate_from_value(&self, ctx: &mut Context) -> Option<()> {
        let value_min = self.value.min(ctx);
        let value_max = self.value.max(ctx);
        let valid_indices = self.get_valid_indices(ctx);

        // If index is fixed, constrain the specific array element
        if valid_indices.len() == 1 {
            let idx = valid_indices[0] as usize;
            if let Some(&array_var) = self.array.get(idx) {
                // Force array[index] == value
                let array_min = array_var.min(ctx);
                let array_max = array_var.max(ctx);

                // Intersect domains
                let new_min = if array_min > value_min { array_min } else { value_min };
                let new_max = if array_max < value_max { array_max } else { value_max };

                if new_min > new_max {
                    return None;
                }

                if array_var.min(ctx) < new_min {
                    array_var.try_set_min(new_min, ctx)?;
                }
                if array_var.max(ctx) > new_max {
                    array_var.try_set_max(new_max, ctx)?;
                }
            }
        } else {
            // Multiple possible indices - remove invalid indices where no array element can match the value
            let mut valid_indices_filtered = Vec::new();
            
            for idx in valid_indices {
                if let Some(array_var) = self.array.get(idx as usize) {
                    let array_min = array_var.min(ctx);
                    let array_max = array_var.max(ctx);
                    
                    // Check if this array element's domain overlaps with value's domain
                    if !(array_max < value_min || array_min > value_max) {
                        valid_indices_filtered.push(idx);
                    }
                }
            }

            // Update index domain to only valid indices
            if !valid_indices_filtered.is_empty() {
                let new_index_min = *valid_indices_filtered.first().unwrap();
                let new_index_max = *valid_indices_filtered.last().unwrap();
                
                if self.index.min(ctx) < Val::ValI(new_index_min) {
                    self.index.try_set_min(Val::ValI(new_index_min), ctx)?;
                }
                if self.index.max(ctx) > Val::ValI(new_index_max) {
                    self.index.try_set_max(Val::ValI(new_index_max), ctx)?;
                }
            } else {
                // No valid indices - constraint is unsatisfiable
                return None;
            }
        }

        Some(())
    }

    /// Propagate constraints from index to value and array elements
    fn propagate_from_index(&self, ctx: &mut Context) -> Option<()> {
        let valid_indices = self.get_valid_indices(ctx);
        
        if valid_indices.is_empty() {
            return None;
        }

        // If index is singleton, directly constrain value to equal array[index]
        if valid_indices.len() == 1 {
            let idx = valid_indices[0] as usize;
            if let Some(&array_var) = self.array.get(idx) {
                // Force array[index] == value
                let array_min = array_var.min(ctx);
                let array_max = array_var.max(ctx);
                let value_min = self.value.min(ctx);
                let value_max = self.value.max(ctx);

                // Update value to intersection of its domain and array[index] domain
                let new_min = if array_min > value_min { array_min } else { value_min };
                let new_max = if array_max < value_max { array_max } else { value_max };

                if new_min > new_max {
                    return None;
                }

                if self.value.min(ctx) < new_min {
                    self.value.try_set_min(new_min, ctx)?;
                }
                if self.value.max(ctx) > new_max {
                    self.value.try_set_max(new_max, ctx)?;
                }

                // Update array[index] to intersection as well
                if array_var.min(ctx) < new_min {
                    array_var.try_set_min(new_min, ctx)?;
                }
                if array_var.max(ctx) > new_max {
                    array_var.try_set_max(new_max, ctx)?;
                }
            }
        } else {
            // Multiple possible indices - constrain value to union of possible array values
            if let Some((min_possible, max_possible)) = self.compute_possible_values(ctx) {
                if self.value.min(ctx) < min_possible {
                    self.value.try_set_min(min_possible, ctx)?;
                }
                if self.value.max(ctx) > max_possible {
                    self.value.try_set_max(max_possible, ctx)?;
                }
            }
        }

        Some(())
    }
}

impl Prune for Element {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Validate that index is within array bounds
        let array_len = self.array.len() as i32;
        if self.index.max(ctx) < Val::ValI(0) || self.index.min(ctx) >= Val::ValI(array_len) {
            return None;
        }

        // Constrain index to valid range [0, array.len()-1]
        if self.index.min(ctx) < Val::ValI(0) {
            self.index.try_set_min(Val::ValI(0), ctx)?;
        }
        if self.index.max(ctx) >= Val::ValI(array_len) {
            self.index.try_set_max(Val::ValI(array_len - 1), ctx)?;
        }

        // Check if index is fixed (singleton domain)
        let index_min = self.index.min(ctx);
        let index_max = self.index.max(ctx);
        
        if index_min == index_max {
            // Index is fixed - enforce array[index] == value
            let idx = match index_min {
                Val::ValI(i) => i as usize,
                Val::ValF(f) => f as usize,
            };
            
            if let Some(&array_var) = self.array.get(idx) {
                let array_min = array_var.min(ctx);
                let array_max = array_var.max(ctx);
                let value_min = self.value.min(ctx);
                let value_max = self.value.max(ctx);
                
                // Compute intersection
                let new_min = if array_min > value_min { array_min } else { value_min };
                let new_max = if array_max < value_max { array_max } else { value_max };

                if new_min > new_max {
                    return None; // No overlap - constraint is violated
                }
                
                // Update both array[index] and value to their intersection
                if array_var.min(ctx) < new_min {
                    array_var.try_set_min(new_min, ctx)?;
                }
                if array_var.max(ctx) > new_max {
                    array_var.try_set_max(new_max, ctx)?;
                }
                
                if self.value.min(ctx) < new_min {
                    self.value.try_set_min(new_min, ctx)?;
                }
                if self.value.max(ctx) > new_max {
                    self.value.try_set_max(new_max, ctx)?;
                }
            }
        } else {
            // Index not fixed - use general propagation
            self.propagate_from_value(ctx)?;
            self.propagate_from_index(ctx)?;
        }

        Some(())
    }
}

impl Propagate for Element {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        let mut vars = self.array.clone();
        vars.push(self.index);
        vars.push(self.value);
        vars.into_iter()
    }
}