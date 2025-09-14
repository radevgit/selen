use crate::{props::{Propagate, Prune}, vars::{VarId, Val}, views::{View, Context}};

/// Add a list of views together: `sum(x) == s`.
#[derive(Clone, Debug)]
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
        // Derive minimum and maximum values the sum of terms can reach
        let min_of_terms: Val = self.xs.iter().map(|x| x.min(ctx)).sum();
        let max_of_terms: Val = self.xs.iter().map(|x| x.max(ctx)).sum();

        let _ = self.s.try_set_min(min_of_terms, ctx)?;
        let _ = self.s.try_set_max(max_of_terms, ctx)?;

        // Current bounds of the sum of all terms
        let min = self.s.min(ctx);
        let max = self.s.max(ctx);

        for x in &self.xs {
            let _ = x.try_set_min(min - (max_of_terms - x.max(ctx)), ctx)?;
            let _ = x.try_set_max(max - (min_of_terms - x.min(ctx)), ctx)?;
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