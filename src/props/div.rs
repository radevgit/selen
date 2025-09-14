use crate::{props::{Propagate, Prune}, vars::{VarId, Val}, views::{Context, View}};

/// Division constraint: `x / y == s`.
/// This constraint enforces that x divided by y equals s.
#[derive(Clone, Copy, Debug)]
pub struct Div<U, V> {
    x: U,
    y: V,
    s: VarId,
}

impl<U, V> Div<U, V> {
    pub const fn new(x: U, y: V, s: VarId) -> Self {
        Self { x, y, s }
    }
}

impl<U: View, V: View> Prune for Div<U, V> {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // For s = x / y, we need to handle bounds propagation carefully
        // due to the possibility of division by zero and sign changes
        
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        
        // If y contains zero or values too close to zero, we can't safely compute division
        if Val::range_contains_unsafe_divisor(y_min, y_max) {
            // We can still try to propagate some constraints if parts of the domain are safe
            return Some(());
        }

        // Calculate possible division results
        let mut s_candidates = Vec::new();
        
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
        
        // Calculate division for all combinations of boundary values
        for &x_val in &x_samples {
            for &y_val in &y_samples {
                if let Some(div_result) = x_val.safe_div(y_val) {
                    // Check if the result is not infinite
                    match div_result {
                        Val::ValF(f) if f.is_finite() => s_candidates.push(div_result),
                        Val::ValI(_) => s_candidates.push(div_result),
                        _ => {} // Skip infinite results
                    }
                }
            }
        }
        
        if !s_candidates.is_empty() {
            // Find bounds for s
            let s_min = s_candidates.iter().fold(s_candidates[0], |acc, &x| if x < acc { x } else { acc });
            let s_max = s_candidates.iter().fold(s_candidates[0], |acc, &x| if x > acc { x } else { acc });
            
            // Propagate bounds to s
            let _min = self.s.try_set_min(s_min, ctx)?;
            let _max = self.s.try_set_max(s_max, ctx)?;
        }
        
        // Back-propagation: if s and y are known, constrain x
        // x = s * y
        let s_min = self.s.min(ctx);
        let s_max = self.s.max(ctx);
        
        // Calculate x bounds using x = s * y
        let mut x_candidates = Vec::new();
        
        for &s_val in &[s_min, s_max] {
            for &y_val in &[y_min, y_max] {
                let x_val = s_val * y_val;
                x_candidates.push(x_val);
            }
        }
        
        if !x_candidates.is_empty() {
            let x_new_min = x_candidates.iter().fold(x_candidates[0], |acc, &x| if x < acc { x } else { acc });
            let x_new_max = x_candidates.iter().fold(x_candidates[0], |acc, &x| if x > acc { x } else { acc });
            
            let _min = self.x.try_set_min(x_new_min, ctx)?;
            let _max = self.x.try_set_max(x_new_max, ctx)?;
        }
        
        // Back-propagation: if s and x are known, constrain y
        // y = x / s (but only if s is safe for division)
        if !Val::range_contains_unsafe_divisor(s_min, s_max) {
            let mut y_candidates = Vec::new();
            
            for &x_val in &[x_min, x_max] {
                for &s_val in &[s_min, s_max] {
                    if let Some(y_val) = x_val.safe_div(s_val) {
                        match y_val {
                            Val::ValF(f) if f.is_finite() => y_candidates.push(y_val),
                            Val::ValI(_) => y_candidates.push(y_val),
                            _ => {} // Skip infinite results
                        }
                    }
                }
            }
            
            if !y_candidates.is_empty() {
                let y_new_min = y_candidates.iter().fold(y_candidates[0], |acc, &x| if x < acc { x } else { acc });
                let y_new_max = y_candidates.iter().fold(y_candidates[0], |acc, &x| if x > acc { x } else { acc });
                
                let _min = self.y.try_set_min(y_new_min, ctx)?;
                let _max = self.y.try_set_max(y_new_max, ctx)?;
            }
        }
        
        Some(())
    }
}

impl<U: View, V: View> Propagate for Div<U, V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.s)
            .chain(self.x.get_underlying_var())
            .chain(self.y.get_underlying_var())
    }
}