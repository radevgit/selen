use crate::{props::{Propagate, Prune}, vars::{VarId, Val}, views::{Context, View}};

/// Multiply two views together: `x * y == s`.
#[derive(Clone, Copy, Debug)]
pub struct Mul<U, V> {
    x: U,
    y: V,
    s: VarId,
}

impl<U, V> Mul<U, V> {
    pub const fn new(x: U, y: V, s: VarId) -> Self {
        Self { x, y, s }
    }
}

impl<U: View, V: View> Prune for Mul<U, V> {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        // For s = x * y, we need to handle bounds propagation carefully
        // due to the possibility of negative values affecting the bounds
        
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        
        // Calculate all possible products at the corners of the intervals
        let products = [
            x_min * y_min,
            x_min * y_max,
            x_max * y_min,
            x_max * y_max,
        ];
        
        // Find the minimum and maximum of all products
        let s_min = products.iter().fold(products[0], |acc, &x| if x < acc { x } else { acc });
        let s_max = products.iter().fold(products[0], |acc, &x| if x > acc { x } else { acc });
        
        // Propagate bounds to s
        let _min = self.s.try_set_min(s_min, ctx)?;
        let _max = self.s.try_set_max(s_max, ctx)?;
        
        // Now propagate back to x and y
        // For x: if y != 0, then x = s / y
        let s_min = self.s.min(ctx);
        let s_max = self.s.max(ctx);
        
        // Propagate to x bounds (when y is safe to divide by)
        if !Val::range_contains_unsafe_divisor(y_min, y_max) {
            // y doesn't contain zero or values close to zero, safe to divide
            let mut x_candidates = Vec::new();
            
            // Try all combinations and filter out unsafe divisions
            for &s_val in &[s_min, s_max] {
                for &y_val in &[y_min, y_max] {
                    if let Some(x_val) = s_val.safe_div(y_val) {
                        x_candidates.push(x_val);
                    }
                }
            }
            
            if !x_candidates.is_empty() {
                let x_new_min = x_candidates.iter().fold(x_candidates[0], |acc, &x| if x < acc { x } else { acc });
                let x_new_max = x_candidates.iter().fold(x_candidates[0], |acc, &x| if x > acc { x } else { acc });
                
                let _min = self.x.try_set_min(x_new_min, ctx)?;
                let _max = self.x.try_set_max(x_new_max, ctx)?;
            }
        }
        
        // Propagate to y bounds (when x is safe to divide by)
        if !Val::range_contains_unsafe_divisor(x_min, x_max) {
            // x doesn't contain zero or values close to zero, safe to divide
            let mut y_candidates = Vec::new();
            
            // Try all combinations and filter out unsafe divisions
            for &s_val in &[s_min, s_max] {
                for &x_val in &[x_min, x_max] {
                    if let Some(y_val) = s_val.safe_div(x_val) {
                        y_candidates.push(y_val);
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

impl<U: View, V: View> Propagate for Mul<U, V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.s)
            .chain(self.x.get_underlying_var())
            .chain(self.y.get_underlying_var())
    }
}
