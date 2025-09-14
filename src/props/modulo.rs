use crate::{props::{Propagate, Prune}, vars::{VarId, Val}, views::{Context, View}};

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
        
        // If y contains zero or values too close to zero, we can't safely compute modulo
        if Val::range_contains_unsafe_divisor(y_min, y_max) {
            // We can still try to propagate some constraints if parts of the domain are safe
            return Some(());
        }

        // Calculate possible modulo results
        let mut s_candidates = Vec::new();
        
        // For modulo, the result is always in range [0, |y|-1] for positive y
        // and [-|y|+1, 0] for negative y, but we need to be more careful with mixed signs
        
        // Sample points at domain boundaries and some intermediate values
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
                    // Check if the result is not NaN or infinite
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
            let s_min = s_candidates.iter().fold(s_candidates[0], |acc, &x| if x < acc { x } else { acc });
            let s_max = s_candidates.iter().fold(s_candidates[0], |acc, &x| if x > acc { x } else { acc });
            
            // For modulo, we know more about the bounds:
            // If y > 0: 0 <= s < y
            // If y < 0: y < s <= 0
            // We can use this to tighten bounds further
            let y_abs_min = match (y_min, y_max) {
                (Val::ValI(min_i), Val::ValI(max_i)) => {
                    if min_i > 0 { Some(Val::ValI(0)) }
                    else if max_i < 0 { Some(Val::ValI(max_i + 1)) }
                    else { None }
                },
                (Val::ValF(min_f), Val::ValF(max_f)) => {
                    if min_f > 0.0 { Some(Val::ValF(0.0)) }
                    else if max_f < 0.0 { Some(Val::ValF(max_f + 1.0)) }
                    else { None }
                },
                _ => None,
            };
            
            let y_abs_max = match (y_min, y_max) {
                (Val::ValI(min_i), Val::ValI(max_i)) => {
                    if min_i > 0 { Some(Val::ValI(max_i - 1)) }
                    else if max_i < 0 { Some(Val::ValI(0)) }
                    else { None }
                },
                (Val::ValF(min_f), Val::ValF(max_f)) => {
                    if min_f > 0.0 { Some(Val::ValF(max_f - f64::EPSILON)) }
                    else if max_f < 0.0 { Some(Val::ValF(0.0)) }
                    else { None }
                },
                _ => None,
            };
            
            // Use the tighter bounds if available
            let final_s_min = if let Some(theoretical_min) = y_abs_min {
                if theoretical_min > s_min { theoretical_min } else { s_min }
            } else { s_min };
            
            let final_s_max = if let Some(theoretical_max) = y_abs_max {
                if theoretical_max < s_max { theoretical_max } else { s_max }
            } else { s_max };
            
            // Propagate bounds to s
            let _min = self.s.try_set_min(final_s_min, ctx)?;
            let _max = self.s.try_set_max(final_s_max, ctx)?;
        }
        
        // Back-propagation is complex for modulo, so we do limited propagation
        // We can at least ensure that if s is known and y is known, we can constrain x
        let s_min = self.s.min(ctx);
        let s_max = self.s.max(ctx);
        
        // If y and s are both fixed, we can derive some constraints on x
        if y_min == y_max && s_min == s_max {
            // x = k * y + s for some integer k
            // We need to find valid values of k such that x is in its domain
            let y_val = y_min;
            let s_val = s_min;
            
            if let (Some(_), Some(_)) = (y_val.safe_div(Val::ValI(1)), s_val.safe_div(Val::ValI(1))) {
                // For now, we don't do complex back-propagation for modulo
                // This would require more sophisticated interval arithmetic
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
