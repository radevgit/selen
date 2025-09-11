use crate::{props::{Propagate, Prune}, vars::{VarId, Val}, views::{Context, View}};

/// Absolute value constraint: `|x| == s`.
/// This constraint enforces that the absolute value of x equals s.
#[derive(Clone, Copy, Debug)]
pub struct Abs<U> {
    x: U,
    s: VarId,
}

impl<U> Abs<U> {
    pub const fn new(x: U, s: VarId) -> Self {
        Self { x, s }
    }
}

impl<U: View> Prune for Abs<U> {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        // For s = |x|, we know:
        // 1. s >= 0 (absolute value is always non-negative)
        // 2. If s is known, then x ∈ [-s, s] and x ∈ [-s, -s] ∪ [s, s]
        // 3. If x >= 0, then s = x
        // 4. If x < 0, then s = -x
        
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let s_min = self.s.min(ctx);
        let s_max = self.s.max(ctx);
        
        // Step 1: Ensure s >= 0
        let zero = match s_min {
            Val::ValI(_) => Val::ValI(0),
            Val::ValF(_) => Val::ValF(0.0),
        };
        let _min = self.s.try_set_min(zero, ctx)?;
        
        // Step 2: Calculate bounds for s based on x
        // |x| is minimized when x is closest to 0, maximized at the extremes
        let abs_x_min = if x_min <= zero && x_max >= zero {
            // x contains 0, so minimum absolute value is 0
            zero
        } else if x_min > zero {
            // x is entirely positive, so min |x| = x_min
            x_min
        } else {
            // x is entirely negative, so min |x| = |x_max| = -x_max
            match x_max {
                Val::ValI(i) => Val::ValI(-i),
                Val::ValF(f) => Val::ValF(-f),
            }
        };
        
        let abs_x_max = match (x_min, x_max) {
            (Val::ValI(min_i), Val::ValI(max_i)) => {
                let abs_min = min_i.abs();
                let abs_max = max_i.abs();
                Val::ValI(if abs_min > abs_max { abs_min } else { abs_max })
            },
            (Val::ValF(min_f), Val::ValF(max_f)) => {
                let abs_min = min_f.abs();
                let abs_max = max_f.abs();
                Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
            },
            (Val::ValI(min_i), Val::ValF(max_f)) => {
                let abs_min = (min_i as f64).abs();
                let abs_max = max_f.abs();
                Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
            },
            (Val::ValF(min_f), Val::ValI(max_i)) => {
                let abs_min = min_f.abs();
                let abs_max = (max_i as f64).abs();
                Val::ValF(if abs_min > abs_max { abs_min } else { abs_max })
            },
        };
        
        // Propagate bounds to s
        let _min = self.s.try_set_min(abs_x_min, ctx)?;
        let _max = self.s.try_set_max(abs_x_max, ctx)?;
        
        // Step 3: Propagate bounds back to x based on s
        let s_min = self.s.min(ctx);
        let s_max = self.s.max(ctx);
        
        // If |x| = s, then x ∈ [-s_max, -s_min] ∪ [s_min, s_max]
        // But we can only represent intervals, so we use [-s_max, s_max]
        // and rely on other constraints or search to eliminate impossible values
        
        let neg_s_max = match s_max {
            Val::ValI(i) => Val::ValI(-i),
            Val::ValF(f) => Val::ValF(-f),
        };
        
        // x must be in range [-s_max, s_max]
        let _min = self.x.try_set_min(neg_s_max, ctx)?;
        let _max = self.x.try_set_max(s_max, ctx)?;
        
        // Additional constraint: if we know s is fixed to a value > 0,
        // and x domain doesn't include both positive and negative values that give that absolute value,
        // we can be more precise
        if s_min == s_max && s_min > zero {
            let x_min_new = self.x.min(ctx);
            let x_max_new = self.x.max(ctx);
            
            // Check if x can only be positive or only negative
            if x_min_new >= zero {
                // x is non-negative, so |x| = x, therefore x = s
                let _min = self.x.try_set_min(s_min, ctx)?;
                let _max = self.x.try_set_max(s_max, ctx)?;
            } else if x_max_new <= zero {
                // x is non-positive, so |x| = -x, therefore x = -s
                let neg_s_min = match s_min {
                    Val::ValI(i) => Val::ValI(-i),
                    Val::ValF(f) => Val::ValF(-f),
                };
                let neg_s_max = match s_max {
                    Val::ValI(i) => Val::ValI(-i),
                    Val::ValF(f) => Val::ValF(-f),
                };
                let _min = self.x.try_set_min(neg_s_max, ctx)?;
                let _max = self.x.try_set_max(neg_s_min, ctx)?;
            }
        }
        
        Some(())
    }
}

impl<U: View> Propagate for Abs<U> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.s)
            .chain(self.x.get_underlying_var())
    }
}
