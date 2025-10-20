use crate::{constraints::props::{Propagate, Prune}, variables::{VarId, Val}, variables::views::{View, Context}};

/// Add a list of views together: `sum(x) == s`.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct Sum<V> {
    xs: Vec<V>,
    s: VarId,
}

impl<V> Sum<V> {
    pub const fn new(xs: Vec<V>, s: VarId) -> Self {
        Self { xs, s }
    }
}

impl<V: View> Prune for Sum<V> {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // === Phase 1: Forward Propagation (O(n)) ===
        // Compute minimum and maximum values the sum of terms can reach
        let mut min_of_terms: Val = Val::ValI(0);
        let mut max_of_terms: Val = Val::ValI(0);
        
        for x in &self.xs {
            min_of_terms = min_of_terms + x.min(ctx);
            max_of_terms = max_of_terms + x.max(ctx);
        }

        let _ = self.s.try_set_min(min_of_terms, ctx)?;
        let _ = self.s.try_set_max(max_of_terms, ctx)?;

        // === Phase 2: Reverse Propagation (O(n)) ===
        let min = self.s.min(ctx);
        let max = self.s.max(ctx);

        // For each variable, compute bounds using precomputed totals
        for x in &self.xs {
            // Cache min/max to avoid repeated calls
            let x_min = x.min(ctx);
            let x_max = x.max(ctx);
            
            let sum_mins_except = min_of_terms - x_min;
            let sum_maxs_except = max_of_terms - x_max;
            
            let _ = x.try_set_min(min - sum_maxs_except, ctx)?;
            let _ = x.try_set_max(max - sum_mins_except, ctx)?;
        }

        Some(())
    }
}

impl<V: View> Propagate for Sum<V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.xs
            .iter()
            .filter_map(|x| x.get_underlying_var())
            .chain(core::iter::once(self.s))
    }
}