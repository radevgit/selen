use crate::{
    constraints::props::{Propagate, Prune},
    variables::VarId,
    variables::views::{Context, View},
};

/// Not-equals constraint placeholder: x != y
/// 
/// Note: This propagator is not currently used during constraint solving.
/// The ne() constraint is implemented through alternative mechanisms
/// (likely reification or linear constraints). This struct exists to
/// maintain the constraint registration interface.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct NotEquals<U, V> {
    x: U,
    y: V,
}

impl<U, V> NotEquals<U, V> {
    pub fn new(x: U, y: V) -> Self {
        Self { x, y }
    }
}

impl<U: View, V: View> Prune for NotEquals<U, V> {
    fn prune(&self, _ctx: &mut Context) -> Option<()> {
        // This propagator is not used - the ne() constraint is implemented
        // through alternative mechanisms. This is a no-op placeholder.
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
