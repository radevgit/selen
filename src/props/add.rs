use crate::{props::{Propagate, Prune}, vars::VarId, views::{Context, View}};

/// Add two views together: `x + y == s`.
#[derive(Clone, Copy, Debug)]
pub struct Add<U, V> {
    x: U,
    y: V,
    s: VarId,
}

impl<U, V> Add<U, V> {
    pub const fn new(x: U, y: V, s: VarId) -> Self {
        Self { x, y, s }
    }
}

impl<U: View, V: View> Prune for Add<U, V> {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let _min = self.s.try_set_min(self.x.min(ctx) + self.y.min(ctx), ctx)?;
        let _max = self.s.try_set_max(self.x.max(ctx) + self.y.max(ctx), ctx)?;

        let _min = self.x.try_set_min(self.s.min(ctx) - self.y.max(ctx), ctx)?;
        let _max = self.x.try_set_max(self.s.max(ctx) - self.y.min(ctx), ctx)?;

        let _min = self.y.try_set_min(self.s.min(ctx) - self.x.max(ctx), ctx)?;
        let _max = self.y.try_set_max(self.s.max(ctx) - self.x.min(ctx), ctx)?;

        Some(())
    }
}

impl<U: View, V: View> Propagate for Add<U, V> {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.s)
            .chain(self.x.get_underlying_var())
            .chain(self.y.get_underlying_var())
    }
}
