//! Core view system traits and types

use crate::variables::{Val, VarId, Vars};
use std::marker::PhantomData;

/// Represents the result type that a view produces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub enum ViewType {
    /// View produces integer values only
    Integer,
    /// View produces floating-point values (or mixed integer/float)
    Float,
}

/// Apply simple domain transformations on the fly to make propagators more generic.
#[allow(private_bounds)]
pub trait View: ViewRaw {
    /// Get the handle of the variable this view depends on.
    fn get_underlying_var(self) -> Option<VarId> {
        self.get_underlying_var_raw()
    }

    /// Access domain minimum.
    fn min(self, ctx: &Context) -> Val {
        self.min_raw(&ctx.vars)
    }

    /// Access domain maximum.
    fn max(self, ctx: &Context) -> Val {
        self.max_raw(&ctx.vars)
    }

    /// Check if domain contains a value.
    fn contains(self, ctx: &Context, val: Val) -> bool {
        self.contains_raw(&ctx.vars, val)
    }

    /// Check if variable is assigned to a single value.
    fn is_fixed(self, ctx: &Context) -> bool {
        self.is_fixed_raw(&ctx.vars)
    }

    /// Get the assigned value if variable is fixed.
    fn assigned_value(self, ctx: &Context) -> Option<Val> {
        if self.is_fixed(ctx) {
            Some(self.min(ctx))
        } else {
            None
        }
    }

    /// Get the type of values this view produces
    fn view_type(self) -> ViewType;
}

/// Raw view operations that work directly with Vars
#[doc(hidden)]
pub trait ViewRaw: Copy + core::fmt::Debug + 'static {
    fn get_underlying_var_raw(self) -> Option<VarId>;
    fn min_raw(self, vars: &Vars) -> Val;
    fn max_raw(self, vars: &Vars) -> Val;
    fn contains_raw(self, vars: &Vars, val: Val) -> bool;
    fn is_fixed_raw(self, vars: &Vars) -> bool;
}

/// Context for view operations
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct Context {
    pub vars: Vars,
}

impl Context {
    pub fn new(vars: Vars) -> Self {
        Self { vars }
    }
}
