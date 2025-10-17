use crate::{constraints::props::{Propagate, Prune}, variables::{VarId, Val}, variables::views::{Context, View}};

/// Modulo constraint: `x % y == s`.
/// This constraint enforces that the remainder of x divided by y equals s.
#[derive(Clone, Copy, Debug)]
pub struct Modulo<U, V> {
    x: U,
    y: V,
    s: VarId,
}

impl<U, V> Modulo<U, V> {
    pub const fn new(x: U, y: V, s: VarId) -> Self {
        Self { x, y, s }
    }
}

impl<U: View, V: View> Prune for Modulo<U, V> {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // For s = x % y, we need to handle bounds propagation carefully
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let s_min = self.s.min(ctx);
        let s_max = self.s.max(ctx);
        
        // If y contains zero or values too close to zero, we can't safely compute modulo
        if Val::range_contains_unsafe_divisor(y_min, y_max) {
            // We can still try to propagate some constraints if parts of the domain are safe
            return Some(());
        }

        // CASE 1: Both x and y are fixed → exact computation
        if x_min == x_max && y_min == y_max {
            if let Some(exact_result) = x_min.safe_mod(y_min) {
                // Set s to this exact value
                self.s.try_set_min(exact_result, ctx)?;
                self.s.try_set_max(exact_result, ctx)?;
                return Some(());
            }
        }

        // CASE 2: y is fixed (and non-zero) → compute s bounds based on x range
        if y_min == y_max {
            if let Val::ValI(y_val) = y_min {
                if y_val != 0 {
                    // For modulo: s is in range [0, |y|-1] when y > 0
                    // or [-(|y|-1), 0] when y < 0
                    if y_val > 0 {
                        let s_theoretical_min = Val::ValI(0);
                        let s_theoretical_max = Val::ValI(y_val - 1);
                        
                        let new_s_min = if s_theoretical_min > s_min { s_theoretical_min } else { s_min };
                        let new_s_max = if s_theoretical_max < s_max { s_theoretical_max } else { s_max };
                        
                        self.s.try_set_min(new_s_min, ctx)?;
                        self.s.try_set_max(new_s_max, ctx)?;
                    } else {
                        // y_val < 0
                        let s_theoretical_min = Val::ValI(y_val + 1);
                        let s_theoretical_max = Val::ValI(0);
                        
                        let new_s_min = if s_theoretical_min > s_min { s_theoretical_min } else { s_min };
                        let new_s_max = if s_theoretical_max < s_max { s_theoretical_max } else { s_max };
                        
                        self.s.try_set_min(new_s_min, ctx)?;
                        self.s.try_set_max(new_s_max, ctx)?;
                    }
                }
            }
        }

        // CASE 3: Both x and y are in bounded ranges → compute s bounds
        let mut s_candidates = Vec::with_capacity(4);
        
        // Sample points at domain boundaries
        let x_samples = if x_min == x_max {
            vec![x_min]
        } else {
            vec![x_min, x_max]
        };
        
        let y_samples = if y_min == y_max {
            vec![y_min]
        } else {
            vec![y_min, y_max]
        };
        
        // Calculate modulo for all combinations of boundary values
        for &x_val in &x_samples {
            for &y_val in &y_samples {
                if let Some(mod_result) = x_val.safe_mod(y_val) {
                    // Check if the result is valid
                    match mod_result {
                        Val::ValF(f) if f.is_finite() => s_candidates.push(mod_result),
                        Val::ValI(_) => s_candidates.push(mod_result),
                        _ => {} // Skip NaN or infinite results
                    }
                }
            }
        }
        
        if !s_candidates.is_empty() {
            // Find bounds for s based on modulo properties
            let s_computed_min = s_candidates.iter().fold(s_candidates[0], |acc, &x| if x < acc { x } else { acc });
            let s_computed_max = s_candidates.iter().fold(s_candidates[0], |acc, &x| if x > acc { x } else { acc });
            
            // CRITICAL FIX: Allow expansion if current domain is too narrow
            // This can happen when result variable was created before deferred constraints applied
            // and those deferred constraints now require larger modulo values.
            // We must try to set the bounds, and if it fails, return None (fail the space)
            self.s.try_set_min(s_computed_min, ctx)?;
            self.s.try_set_max(s_computed_max, ctx)?;
        }

        // CASE 4: Back-propagation from s to x (when y and s are fixed)
        if y_min == y_max && s_min == s_max {
            if let (Val::ValI(y_val), Val::ValI(s_val)) = (y_min, s_min) {
                if y_val != 0 && s_val >= 0 && s_val < y_val.abs() {
                    // x = k * y + s for some integer k
                    // We need to find the range of k such that x remains in bounds
                    let x_current_min = x_min;
                    let x_current_max = x_max;
                    
                    // Find the minimum and maximum k
                    let mut valid_x_values = Vec::with_capacity(8);
                    
                    if let (Val::ValI(x_curr_min), Val::ValI(x_curr_max)) = (x_current_min, x_current_max) {
                        // Try k values that produce x in the valid range
                        let k_min_theoretical = (x_curr_min - s_val) / y_val;
                        let k_max_theoretical = (x_curr_max - s_val) / y_val;
                        
                        // Try a range around these theoretical k values
                        for k in (k_min_theoretical - 1)..=(k_max_theoretical + 1) {
                            let candidate_x = k * y_val + s_val;
                            if candidate_x >= x_curr_min && candidate_x <= x_curr_max {
                                valid_x_values.push(Val::ValI(candidate_x));
                            }
                        }
                        
                        if !valid_x_values.is_empty() {
                            let new_x_min = valid_x_values.iter().fold(valid_x_values[0], |acc, &x| if x < acc { x } else { acc });
                            let new_x_max = valid_x_values.iter().fold(valid_x_values[0], |acc, &x| if x > acc { x } else { acc });
                            
                            // Try to tighten x bounds
                            self.x.try_set_min(new_x_min, ctx)?;
                            self.x.try_set_max(new_x_max, ctx)?;
                        }
                    }
                }
            }
        }
        
        Some(())
    }
}

impl<U: View, V: View> Propagate for Modulo<U, V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.s)
            .chain(self.x.get_underlying_var())
            .chain(self.y.get_underlying_var())
    }
}
