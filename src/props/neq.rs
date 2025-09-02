use crate::{props::{Propagate, Prune}, vars::{VarId, Val}, views::{Context, View, ViewType}};
use std::collections::HashMap;

/// Enhanced not-equals constraint that suggests branching around forbidden values.
/// This constraint tracks forbidden values and can suggest custom branching strategies.
#[derive(Clone, Debug)]
pub struct NotEquals<U, V> {
    x: U,
    y: V,
    /// Track forbidden values for variables to suggest better branching
    forbidden_values: HashMap<VarId, Vec<Val>>,
}

impl<U, V> NotEquals<U, V> {
    pub fn new(x: U, y: V) -> Self {
        Self { 
            x, 
            y,
            forbidden_values: HashMap::new(),
        }
    }
    
    /// Get forbidden values for a specific variable (for branching hints)
    pub fn get_forbidden_values(&self, var: VarId) -> Option<&Vec<Val>> {
        self.forbidden_values.get(&var)
    }
    
    /// Add a forbidden value for a variable (used during propagation)
    fn add_forbidden_value(&mut self, var: VarId, value: Val) {
        self.forbidden_values.entry(var).or_insert_with(Vec::new).push(value);
    }
}

impl<U: View, V: View> Prune for NotEquals<U, V> {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        // Get current domains
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        
        // Check if domains overlap - if not, constraint is already satisfied
        if x_max < y_min || y_max < x_min {
            return Some(());
        }
        
        // If x is assigned to a single value, ensure y is not equal to that value
        if x_min == x_max {
            let x_value = x_min;
            
            // If y is also assigned to the same value, constraint is violated
            if y_min == y_max && y_min == x_value {
                return None;
            }
            
            // Record this as a forbidden value for y (for future branching hints)
            if let Some(y_var) = self.y.get_underlying_var() {
                self.add_forbidden_value(y_var, x_value);
            }
            
            // For floating point, we only adjust bounds if the forbidden value is exactly at a bound
            // This prevents creating artificial micro-splits
            let x_type = self.x.result_type(ctx);
            let y_type = self.y.result_type(ctx);
            
            match (x_type, y_type) {
                // Integer case: we can precisely exclude the value
                (ViewType::Integer, ViewType::Integer) => {
                    if let Val::ValI(x_val) = x_value {
                        // If y's minimum equals the forbidden value, increase the minimum
                        if let Val::ValI(y_min_val) = y_min {
                            if y_min_val == x_val {
                                self.y.try_set_min(Val::ValI(x_val + 1), ctx)?;
                            }
                        }
                        
                        // If y's maximum equals the forbidden value, decrease the maximum
                        if let Val::ValI(y_max_val) = y_max {
                            if y_max_val == x_val {
                                self.y.try_set_max(Val::ValI(x_val - 1), ctx)?;
                            }
                        }
                    }
                }
                // Float case: only adjust if the domain is very small (likely a precision artifact)
                _ => {
                    // For floats, we're more conservative - only adjust if the domain is tiny
                    // This prevents unnecessary micro-splits
                    let domain_size = match (y_min, y_max) {
                        (Val::ValF(min_f), Val::ValF(max_f)) => max_f - min_f,
                        (Val::ValI(min_i), Val::ValI(max_i)) => (max_i - min_i) as f32,
                        (Val::ValI(min_i), Val::ValF(max_f)) => max_f - min_i as f32,
                        (Val::ValF(min_f), Val::ValI(max_i)) => max_i as f32 - min_f,
                    };
                    
                    // Only adjust bounds if domain is very small (suggesting it's nearly assigned)
                    if domain_size < 1e-3 {
                        let meaningful_delta = (domain_size * 0.5).max(1e-6);
                        
                        // If y's minimum equals the forbidden value, increase the minimum
                        if y_min == x_value {
                            let new_min = match x_value {
                                Val::ValF(x_val) => Val::ValF(x_val + meaningful_delta),
                                Val::ValI(x_val) => Val::ValF(x_val as f32 + meaningful_delta),
                            };
                            self.y.try_set_min(new_min, ctx)?;
                        }
                        
                        // If y's maximum equals the forbidden value, decrease the maximum
                        if y_max == x_value {
                            let new_max = match x_value {
                                Val::ValF(x_val) => Val::ValF(x_val - meaningful_delta),
                                Val::ValI(x_val) => Val::ValF(x_val as f32 - meaningful_delta),
                            };
                            self.y.try_set_max(new_max, ctx)?;
                        }
                    }
                }
            }
        }
        
        // If y is assigned to a single value, ensure x is not equal to that value
        if y_min == y_max {
            let y_value = y_min;
            
            // If x is also assigned to the same value, constraint is violated
            if x_min == x_max && x_min == y_value {
                return None;
            }
            
            // Record this as a forbidden value for x (for future branching hints)
            if let Some(x_var) = self.x.get_underlying_var() {
                self.add_forbidden_value(x_var, y_value);
            }
            
            // Apply the same logic as above but for x
            let x_type = self.x.result_type(ctx);
            let y_type = self.y.result_type(ctx);
            
            match (x_type, y_type) {
                // Integer case: we can precisely exclude the value
                (ViewType::Integer, ViewType::Integer) => {
                    if let Val::ValI(y_val) = y_value {
                        // If x's minimum equals the forbidden value, increase the minimum
                        if let Val::ValI(x_min_val) = x_min {
                            if x_min_val == y_val {
                                self.x.try_set_min(Val::ValI(y_val + 1), ctx)?;
                            }
                        }
                        
                        // If x's maximum equals the forbidden value, decrease the maximum
                        if let Val::ValI(x_max_val) = x_max {
                            if x_max_val == y_val {
                                self.x.try_set_max(Val::ValI(y_val - 1), ctx)?;
                            }
                        }
                    }
                }
                // Float case: only adjust if the domain is very small
                _ => {
                    let domain_size = match (x_min, x_max) {
                        (Val::ValF(min_f), Val::ValF(max_f)) => max_f - min_f,
                        (Val::ValI(min_i), Val::ValI(max_i)) => (max_i - min_i) as f32,
                        (Val::ValI(min_i), Val::ValF(max_f)) => max_f - min_i as f32,
                        (Val::ValF(min_f), Val::ValI(max_i)) => max_i as f32 - min_f,
                    };
                    
                    if domain_size < 1e-3 {
                        let meaningful_delta = (domain_size * 0.5).max(1e-6);
                        
                        if x_min == y_value {
                            let new_min = match y_value {
                                Val::ValF(y_val) => Val::ValF(y_val + meaningful_delta),
                                Val::ValI(y_val) => Val::ValF(y_val as f32 + meaningful_delta),
                            };
                            self.x.try_set_min(new_min, ctx)?;
                        }
                        
                        if x_max == y_value {
                            let new_max = match y_value {
                                Val::ValF(y_val) => Val::ValF(y_val - meaningful_delta),
                                Val::ValI(y_val) => Val::ValF(y_val as f32 - meaningful_delta),
                            };
                            self.x.try_set_max(new_max, ctx)?;
                        }
                    }
                }
            }
        }
        
        Some(())
    }
}

impl<U: View, V: View> Propagate for NotEquals<U, V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.x
            .get_underlying_var()
            .into_iter()
            .chain(self.y.get_underlying_var())
    }
}
