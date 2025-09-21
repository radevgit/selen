//! Variable views and domain transformations.
//!
//! This module provides views that allow applying domain transformations on the fly
//! to make propagators more generic. Views enable efficient constraint implementation
//! by allowing the same propagator to work with transformed variable domains.
//!
//! # View Types
//!
//! Views can produce different types of values:
//! - Integer views for operations that preserve integer domains
//! - Float views for operations that may introduce floating-point values
//!
//! # Internal Implementation
//!
//! Views are primarily internal implementation details used by the constraint
//! system to efficiently handle domain transformations without copying data.
//! Most users interact with views indirectly through constraint posting.
//!
//! Note: This module contains primarily internal APIs marked with `#[doc(hidden)]`.

use crate::variables::{Val, Var, VarId, VarIdBin, Vars};

#[doc(hidden)]
/// Represents the result type that a view produces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub enum ViewType {
    /// View produces integer values only
    Integer,
    /// View produces floating-point values (or mixed integer/float)
    Float,
}

#[doc(hidden)]
/// Apply simple domain transformations on the fly to make propagators more generic.
#[allow(private_bounds)]
pub trait View: ViewRaw {
    /// Get the handle of the variable this view depends on.
    fn get_underlying_var(self) -> Option<VarId> {
        self.get_underlying_var_raw()
    }

    /// Access domain minimum.
    fn min(self, ctx: &Context) -> Val {
        self.min_raw(ctx.vars)
    }

    /// Access domain maximum.
    fn max(self, ctx: &Context) -> Val {
        self.max_raw(ctx.vars)
    }

    /// Determine the result type that this view produces
    fn result_type(self, ctx: &Context) -> ViewType;

    /// Try to set the provided value as domain minimum, failing the search space on infeasibility.
    ///
    /// The `None` case signals failure, otherwise the new minimum is returned.
    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val>;

    /// Try to the set provided value as domain maximum, failing the search space on infeasibility.
    ///
    /// The `None` case signals failure, otherwise the new maximum is returned.
    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val>;
}

#[doc(hidden)]
/// Extension trait to provide helper methods on views.
#[doc(hidden)]
pub trait ViewExt: View {
    /// Invert the sign of the bounds of the underlying view.
    fn opposite(self) -> Opposite<Self>;

    /// Add a constant offset to the underlying view.
    fn plus(self, offset: Val) -> Plus<Self>;

    /// Subtract a constant offset from the underlying view.
    fn minus(self, offset: Val) -> Plus<Self>;

    /// Scale the underlying view by a constant factor.
    fn times(self, scale: Val) -> Times<Self>;

    /// Scale the underlying view by a strictly positive constant factor.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided scale is not strictly positive.
    fn times_pos(self, scale_pos: Val) -> TimesPos<Self>;

    /// Scale the underlying view by a strictly negative constant factor.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided scale is not strictly negative.
    fn times_neg(self, scale_neg: Val) -> TimesNeg<Self>;

    /// Get the next representable value using ULP-based approach.
    fn next(self) -> Next<Self>;

    /// Get the previous representable value using ULP-based approach.
    fn prev(self) -> Prev<Self>;
}

#[doc(hidden)]
/// Extension trait for debug formatting views with domain information.
#[doc(hidden)]
pub trait ViewDebugExt: View {
    /// Format view with domain bounds for debugging.
    fn debug_with_domain(&self, vars: &Vars) -> String {
        format!("{:?} [{:?}..{:?}]", 
                self, 
                self.min_raw(vars), 
                self.max_raw(vars))
    }
}

impl<V: View> ViewExt for V {
    fn opposite(self) -> Opposite<Self> {
        Opposite(self)
    }

    fn plus(self, offset: Val) -> Plus<Self> {
        Plus { x: self, offset }
    }

    fn minus(self, offset: Val) -> Plus<Self> {
        // Subtraction is addition with negative offset
        let neg_offset = match offset {
            Val::ValI(i) => Val::ValI(-i),
            Val::ValF(f) => Val::ValF(-f),
        };
        Plus { x: self, offset: neg_offset }
    }

    fn times(self, scale: Val) -> Times<Self> {
        Times::new(self, scale)
    }

    fn times_pos(self, scale_pos: Val) -> TimesPos<Self> {
        TimesPos::new(self, scale_pos)
    }

    fn times_neg(self, scale_neg: Val) -> TimesNeg<Self> {
        match scale_neg {
            Val::ValI(scale_val) => TimesPos::new(self.opposite(), Val::ValI(-scale_val)),
            Val::ValF(scale_val) => TimesPos::new(self.opposite(), Val::ValF(-scale_val)),
        }
    }

    fn next(self) -> Next<Self> {
        Next { x: self }
    }

    fn prev(self) -> Prev<Self> {
        Prev { x: self }
    }
}

// Implement ViewDebugExt for all views - the blanket implementation covers all View types
impl<V: View> ViewDebugExt for V {}

/// Wrapper around search space object to restrict exposed interface and track changes.
#[derive(Debug)]
#[doc(hidden)]
pub struct Context<'s> {
    vars: &'s mut Vars,
    events: &'s mut Vec<VarId>,
}

impl<'s> Context<'s> {
    #[doc(hidden)]
    /// Initialize context from mutable references to outside objects.
    pub(crate) fn new(vars: &'s mut Vars, events: &'s mut Vec<VarId>) -> Self {
        Self { vars, events }
    }
    
    #[doc(hidden)]
    /// Get access to the variables for interval context-aware operations.
    pub fn vars(&self) -> &Vars {
        self.vars
    }

    #[doc(hidden)]
    /// Try to set provided value as domain minimum, failing the space on infeasibility.
    pub fn try_set_min(&mut self, v: VarId, min: Val) -> Option<Val> {
        // Access domain of variable using the provided handle
        let var = &mut self.vars[v];

        match (var, min) {
            (
                Var::VarI(sparse_set),
                Val::ValI(min_i),
            ) => {
                // Infeasible, fail space
                if min_i > sparse_set.max() {
                    return None;
                }

                let old_min = sparse_set.min();
                if min_i > old_min {
                    // Remove values below min_i
                    sparse_set.remove_below(min_i);
                    
                    // Check if domain became empty
                    if sparse_set.is_empty() {
                        return None;
                    }

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValI(sparse_set.min()))
            }
            (
                Var::VarF(interval),
                Val::ValF(min_f),
            ) => {
                // Infeasible, fail space
                if min_f > interval.max {
                    return None;
                }

                if min_f > interval.min {
                    // Set new minimum
                    interval.min = min_f;

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValF(interval.min))
            }
            (
                Var::VarI(sparse_set),
                Val::ValF(min_f),
            ) => {
                // Convert float to integer using ceiling (to ensure the bound is not violated)
                let min_converted = min_f.ceil() as i32;
                
                // Infeasible, fail space
                if min_converted > sparse_set.max() {
                    return None;
                }

                let old_min = sparse_set.min();
                if min_converted > old_min {
                    // Remove values below min_converted
                    sparse_set.remove_below(min_converted);
                    
                    // Check if domain became empty
                    if sparse_set.is_empty() {
                        return None;
                    }

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValI(sparse_set.min()))
            }
            (
                Var::VarF(interval),
                Val::ValI(min_i),
            ) => {
                // Convert integer to float
                let min_converted = min_i as f64;
                
                // Infeasible, fail space
                if min_converted > interval.max {
                    return None;
                }

                if min_converted > interval.min {
                    // Set new minimum
                    interval.min = min_converted;

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValF(interval.min))
            }
        }
    }

    #[doc(hidden)]
    /// Try to set provided value as domain maximum, failing the space on infeasibility.
    pub fn try_set_max(&mut self, v: VarId, max: Val) -> Option<Val> {
        // Access domain of variable using the provided handle
        let var = &mut self.vars[v];

        match (var, max) {
            (
                Var::VarI(sparse_set),
                Val::ValI(max_i),
            ) => {
                // Infeasible, fail space
                if max_i < sparse_set.min() {
                    return None;
                }

                let old_max = sparse_set.max();
                if max_i < old_max {
                    // Remove values above max_i
                    sparse_set.remove_above(max_i);
                    
                    // Check if domain became empty
                    if sparse_set.is_empty() {
                        return None;
                    }

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValI(sparse_set.max()))
            }
            (
                Var::VarF(interval),
                Val::ValF(max_f),
            ) => {
                // Infeasible, fail space
                if max_f < interval.min {
                    return None;
                }

                if max_f < interval.max {
                    // Set new maximum
                    interval.max = max_f;

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValF(interval.max))
            }
            (
                Var::VarI(sparse_set),
                Val::ValF(max_f),
            ) => {
                // Convert float to integer using floor (to ensure the bound is not violated)
                let max_converted = max_f.floor() as i32;
                
                // Infeasible, fail space
                if max_converted < sparse_set.min() {
                    return None;
                }

                let old_max = sparse_set.max();
                if max_converted < old_max {
                    // Remove values above max_converted
                    sparse_set.remove_above(max_converted);
                    
                    // Check if domain became empty
                    if sparse_set.is_empty() {
                        return None;
                    }

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValI(sparse_set.max()))
            }
            (
                Var::VarF(interval),
                Val::ValI(max_i),
            ) => {
                // Convert integer to float
                let max_converted = max_i as f64;
                
                // Infeasible, fail space
                if max_converted < interval.min {
                    return None;
                }

                if max_converted < interval.max {
                    // Set new maximum
                    interval.max = max_converted;

                    // Record modification event
                    self.events.push(v);
                }

                Some(Val::ValF(interval.max))
            }
        }
    }
}

#[doc(hidden)]
// Trait kept internal, to prevent users from declaring their own views.
pub(crate) trait ViewRaw: Copy + core::fmt::Debug + 'static {
    /// Get the handle of the variable this view depends on.
    fn get_underlying_var_raw(self) -> Option<VarId>;

    /// Access domain minimum.
    fn min_raw(self, vars: &Vars) -> Val;

    /// Access domain maximum.
    fn max_raw(self, vars: &Vars) -> Val;
}

impl ViewRaw for Val {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        None
    }

    fn min_raw(self, _vars: &Vars) -> Val {
        self
    }

    fn max_raw(self, _vars: &Vars) -> Val {
        self
    }
}

impl View for Val {
    fn result_type(self, _ctx: &Context) -> ViewType {
        match self {
            Val::ValI(_) => ViewType::Integer,
            Val::ValF(_) => ViewType::Float,
        }
    }

    fn try_set_min(self, min: Val, _ctx: &mut Context) -> Option<Val> {
        if min <= self { Some(self) } else { None }
    }

    fn try_set_max(self, max: Val, _ctx: &mut Context) -> Option<Val> {
        if max >= self { Some(self) } else { None }
    }
}

impl ViewRaw for VarId {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        Some(self)
    }

    fn min_raw(self, vars: &Vars) -> Val {
        match vars[self] {
            Var::VarI(ref sparse_set) => Val::ValI(sparse_set.min()),
            Var::VarF(ref interval) => Val::ValF(interval.min),
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        match vars[self] {
            Var::VarI(ref sparse_set) => Val::ValI(sparse_set.max()),
            Var::VarF(ref interval) => Val::ValF(interval.max),
        }
    }
}

impl View for VarId {
    fn result_type(self, ctx: &Context) -> ViewType {
        match &ctx.vars[self] {
            Var::VarF { .. } => ViewType::Float,
            Var::VarI { .. } => ViewType::Integer,
        }
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        ctx.try_set_min(self, min)
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        ctx.try_set_max(self, max)
    }
}

impl ViewRaw for VarIdBin {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        self.0.get_underlying_var_raw()
    }

    fn min_raw(self, vars: &Vars) -> Val {
        self.0.min_raw(vars)
    }

    fn max_raw(self, vars: &Vars) -> Val {
        self.0.max_raw(vars)
    }
}

impl View for VarIdBin {
    fn result_type(self, ctx: &Context) -> ViewType {
        match &ctx.vars[self.0] {
            Var::VarF { .. } => ViewType::Float,
            Var::VarI { .. } => ViewType::Integer,
        }
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        self.0.try_set_min(min, ctx)
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        self.0.try_set_max(max, ctx)
    }
}

#[doc(hidden)]
/// Invert the sign of the bounds of the underlying view.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Opposite<V>(V);

impl<V: std::fmt::Debug> std::fmt::Debug for Opposite<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Opposite({:?})", self.0)
    }
}

#[doc(hidden)]
/// Apply next operation using ULP-based approach.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Next<V> {
    x: V,
}

impl<V: std::fmt::Debug> std::fmt::Debug for Next<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Next({:?})", self.x)
    }
}

#[doc(hidden)]
/// Apply prev operation using ULP-based approach.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Prev<V> {
    x: V,
}

impl<V: std::fmt::Debug> std::fmt::Debug for Prev<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Prev({:?})", self.x)
    }
}

impl<V: View> ViewRaw for Opposite<V> {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        self.0.get_underlying_var_raw()
    }

    fn min_raw(self, vars: &Vars) -> Val {
        match self.0.max_raw(vars) {
            Val::ValI(max) => Val::ValI(-max),
            Val::ValF(max) => Val::ValF(-max),
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        match self.0.min_raw(vars) {
            Val::ValI(min) => Val::ValI(-min),
            Val::ValF(min) => Val::ValF(-min),
        }
    }
}

impl<V: View> View for Opposite<V> {
    fn result_type(self, ctx: &Context) -> ViewType {
        // Opposite preserves the type of the underlying view
        self.0.result_type(ctx)
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        // For opposite view: min_opposite = -max_original
        // So to set min_opposite = min, we need max_original = -min
        match min {
            Val::ValI(min_val) => self.0.try_set_max(Val::ValI(-min_val), ctx),
            Val::ValF(min_val) => self.0.try_set_max(Val::ValF(-min_val), ctx),
        }
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        // For opposite view: max_opposite = -min_original
        // So to set max_opposite = max, we need min_original = -max
        match max {
            Val::ValI(max_val) => self.0.try_set_min(Val::ValI(-max_val), ctx),
            Val::ValF(max_val) => self.0.try_set_min(Val::ValF(-max_val), ctx),
        }
    }
}

impl<V: View> ViewRaw for Next<V> {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        self.x.get_underlying_var_raw()
    }

    fn min_raw(self, vars: &Vars) -> Val {
        let base_min = self.x.min_raw(vars);
        // If this is a variable with a FloatInterval, use its next() method
        if let Some(var_id) = self.x.get_underlying_var_raw() {
            if let Var::VarF(interval) = &vars[var_id] {
                if let Val::ValF(f) = base_min {
                    return Val::ValF(interval.next(f));
                }
            }
        }
        // For non-float or non-variable cases, only work with integers
        match base_min {
            Val::ValI(i) => Val::ValI(i + 1),
            Val::ValF(_) => base_min, // Return unchanged for floats without interval info
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        let base_max = self.x.max_raw(vars);
        // If this is a variable with a FloatInterval, use its next() method
        if let Some(var_id) = self.x.get_underlying_var_raw() {
            if let Var::VarF(interval) = &vars[var_id] {
                if let Val::ValF(f) = base_max {
                    return Val::ValF(interval.next(f));
                }
            }
        }
        // For non-float or non-variable cases, only work with integers
        match base_max {
            Val::ValI(i) => Val::ValI(i + 1),
            Val::ValF(_) => base_max, // Return unchanged for floats without interval info
        }
    }
}

impl<V: View> View for Next<V> {
    fn result_type(self, ctx: &Context) -> ViewType {
        // Next preserves the type of the underlying view
        self.x.result_type(ctx)
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        // To set min of next view, we need to reverse the operation
        let target_min = match (min, self.x.result_type(ctx)) {
            (Val::ValI(min_int), ViewType::Float) => {
                // Convert integer to float, then compute prev using step size
                let min_f = min_int as f64;
                if let Some(var_id) = self.x.get_underlying_var_raw() {
                    if let Var::VarF(interval) = &ctx.vars()[var_id] {
                        Val::ValF(interval.prev(min_f))
                    } else {
                        // Fallback: return original float value
                        Val::ValF(min_f)
                    }
                } else {
                    // Fallback: return original float value  
                    Val::ValF(min_f)
                }
            }
            _ => {
                // For float values, try to use step size if available
                match min {
                    Val::ValF(f) => {
                        if let Some(var_id) = self.x.get_underlying_var_raw() {
                            if let Var::VarF(interval) = &ctx.vars()[var_id] {
                                Val::ValF(interval.prev(f))
                            } else {
                                // Fallback: return original value
                                min
                            }
                        } else {
                            // Fallback: return original value
                            min
                        }
                    }
                    Val::ValI(i) => Val::ValI(i - 1), // integer case works fine
                }
            }
        };
        self.x.try_set_min(target_min, ctx)
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        // To set max of next view, we need to reverse the operation
        let target_max = match (max, self.x.result_type(ctx)) {
            (Val::ValI(max_int), ViewType::Float) => {
                // Convert integer to float, then compute prev using step size
                let max_f = max_int as f64;
                if let Some(var_id) = self.x.get_underlying_var_raw() {
                    if let Var::VarF(interval) = &ctx.vars()[var_id] {
                        Val::ValF(interval.prev(max_f))
                    } else {
                        // Fallback: return original float value
                        Val::ValF(max_f)
                    }
                } else {
                    // Fallback: return original float value
                    Val::ValF(max_f)
                }
            }
            _ => {
                // For float values, try to use step size if available
                match max {
                    Val::ValF(f) => {
                        if let Some(var_id) = self.x.get_underlying_var_raw() {
                            if let Var::VarF(interval) = &ctx.vars()[var_id] {
                                Val::ValF(interval.prev(f))
                            } else {
                                // Fallback: return original value
                                max
                            }
                        } else {
                            // Fallback: return original value
                            max
                        }
                    }
                    Val::ValI(i) => Val::ValI(i - 1), // integer case works fine
                }
            }
        };
        self.x.try_set_max(target_max, ctx)
    }
}

impl<V: View> ViewRaw for Prev<V> {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        self.x.get_underlying_var_raw()
    }

    fn min_raw(self, vars: &Vars) -> Val {
        let base_min = self.x.min_raw(vars);
        // If this is a variable with a FloatInterval, use its prev() method
        if let Some(var_id) = self.x.get_underlying_var_raw() {
            if let Var::VarF(interval) = &vars[var_id] {
                if let Val::ValF(f) = base_min {
                    return Val::ValF(interval.prev(f));
                }
            }
        }
        // For non-float or non-variable cases, only work with integers
        match base_min {
            Val::ValI(i) => Val::ValI(i - 1),
            Val::ValF(_) => base_min, // Return unchanged for floats without interval info
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        let base_max = self.x.max_raw(vars);
        // If this is a variable with a FloatInterval, use its prev() method
        if let Some(var_id) = self.x.get_underlying_var_raw() {
            if let Var::VarF(interval) = &vars[var_id] {
                if let Val::ValF(f) = base_max {
                    return Val::ValF(interval.prev(f));
                }
            }
        }
        // For non-float or non-variable cases, only work with integers
        match base_max {
            Val::ValI(i) => Val::ValI(i - 1),
            Val::ValF(_) => base_max, // Return unchanged for floats without interval info
        }
    }
}

impl<V: View> View for Prev<V> {
    fn result_type(self, ctx: &Context) -> ViewType {
        // Prev preserves the type of the underlying view
        self.x.result_type(ctx)
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        // To set min of prev view, we need to reverse the operation
        let target_min = match min {
            Val::ValF(f) => {
                if let Some(var_id) = self.x.get_underlying_var_raw() {
                    if let Var::VarF(interval) = &ctx.vars()[var_id] {
                        Val::ValF(interval.next(f))
                    } else {
                        // Fallback: return original value
                        min
                    }
                } else {
                    // Fallback: return original value
                    min
                }
            }
            Val::ValI(i) => Val::ValI(i + 1), // integer case works fine
        };
        self.x.try_set_min(target_min, ctx)
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        // To set max of prev view, we need to reverse the operation
        let target_max = match max {
            Val::ValF(f) => {
                if let Some(var_id) = self.x.get_underlying_var_raw() {
                    if let Var::VarF(interval) = &ctx.vars()[var_id] {
                        Val::ValF(interval.next(f))
                    } else {
                        // Fallback: return original value
                        max
                    }
                } else {
                    // Fallback: return original value
                    max
                }
            }
            Val::ValI(i) => Val::ValI(i + 1), // integer case works fine
        };
        self.x.try_set_max(target_max, ctx)
    }
}

/// Add a constant offset to the underlying view.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct Plus<V> {
    x: V,
    offset: Val,
}

impl<V: std::fmt::Debug> std::fmt::Debug for Plus<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plus(x: {:?}, offset: {:?})", self.x, self.offset)
    }
}

impl<V: View> ViewRaw for Plus<V> {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        self.x.get_underlying_var_raw()
    }

    fn min_raw(self, vars: &Vars) -> Val {
        match (self.x.min_raw(vars), self.offset) {
            (Val::ValI(min), Val::ValI(offset)) => Val::ValI(min + offset),
            (Val::ValF(min), Val::ValF(offset)) => Val::ValF(min + offset),
            // type coercion
            (Val::ValI(min), Val::ValF(offset)) => Val::ValF(min as f64 + offset),
            (Val::ValF(min), Val::ValI(offset)) => Val::ValF(min + offset as f64),
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        match (self.x.max_raw(vars), self.offset) {
            (Val::ValI(max), Val::ValI(offset)) => Val::ValI(max + offset),
            (Val::ValF(max), Val::ValF(offset)) => Val::ValF(max + offset),
            // type coercion
            (Val::ValI(min), Val::ValF(max)) => Val::ValF(min as f64 + max),
            (Val::ValF(min), Val::ValI(max)) => Val::ValF(min + max as f64),
        }
    }
}

impl<V: View> View for Plus<V> {
    fn result_type(self, ctx: &Context) -> ViewType {
        // Plus operation can promote integers to floats if offset is float
        let base_type = self.x.result_type(ctx);
        let offset_type = match self.offset {
            Val::ValI(_) => ViewType::Integer,
            Val::ValF(_) => ViewType::Float,
        };
        
        // If either operand is float, result is float
        match (base_type, offset_type) {
            (ViewType::Float, _) | (_, ViewType::Float) => ViewType::Float,
            (ViewType::Integer, ViewType::Integer) => ViewType::Integer,
        }
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        match (min, self.offset) {
            (Val::ValI(min_val), Val::ValI(offset)) => {
                self.x.try_set_min(Val::ValI(min_val - offset), ctx)
            }
            (Val::ValF(min_val), Val::ValF(offset)) => {
                self.x.try_set_min(Val::ValF(min_val - offset), ctx)
            }
            // Mixed type cases with automatic conversion
            (Val::ValI(min_val), Val::ValF(offset)) => {
                let required_min = min_val as f64 - offset;
                self.x.try_set_min(Val::ValF(required_min), ctx)
            }
            (Val::ValF(min_val), Val::ValI(offset)) => {
                let required_min = min_val - offset as f64;
                self.x.try_set_min(Val::ValF(required_min), ctx)
            }
        }
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        match (max, self.offset) {
            (Val::ValI(max_val), Val::ValI(offset)) => {
                self.x.try_set_max(Val::ValI(max_val - offset), ctx)
            }
            (Val::ValF(max_val), Val::ValF(offset)) => {
                self.x.try_set_max(Val::ValF(max_val - offset), ctx)
            }
            // Mixed type cases with automatic conversion
            (Val::ValI(max_val), Val::ValF(offset)) => {
                let required_max = max_val as f64 - offset;
                self.x.try_set_max(Val::ValF(required_max), ctx)
            }
            (Val::ValF(max_val), Val::ValI(offset)) => {
                let required_max = max_val - offset as f64;
                self.x.try_set_max(Val::ValF(required_max), ctx)
            }
        }
    }
}

#[doc(hidden)]
/// Scale the underlying view by a constant factor.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub enum Times<V: View> {
    /// Provided factor was strictly negative.
    Neg(TimesNeg<V>),

    /// Provided factor was exactly zero.
    ZeroI,

    ZeroF,

    /// Provided factor was strictly positive.
    Pos(TimesPos<V>),
}

impl<V: View + std::fmt::Debug> std::fmt::Debug for Times<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neg(neg) => write!(f, "Times::Neg({:?})", neg),
            Self::ZeroI => write!(f, "Times::ZeroI"),
            Self::ZeroF => write!(f, "Times::ZeroF"),
            Self::Pos(pos) => write!(f, "Times::Pos({:?})", pos),
        }
    }
}

impl<V: View> Times<V> {
    fn new(x: V, scale: Val) -> Self {
        use core::cmp::Ordering;

        match scale {
            Val::ValI(scale_val) => match scale_val.cmp(&0) {
                Ordering::Less => Self::Neg(TimesPos::new(x.opposite(), Val::ValI(-scale_val))),
                Ordering::Equal => Self::ZeroI,
                Ordering::Greater => Self::Pos(TimesPos::new(x, scale)),
            },
            Val::ValF(scale_val) => {
                if scale_val < 0.0 {
                    Self::Neg(TimesPos::new(x.opposite(), Val::ValF(-scale_val)))
                } else if scale_val == 0.0 {
                    Self::ZeroF
                } else {
                    Self::Pos(TimesPos::new(x, scale))
                }
            }
        }
    }
}

impl<V: View> ViewRaw for Times<V> {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        match self {
            Self::Neg(neg) => neg.get_underlying_var_raw(),
            Self::ZeroI | Self::ZeroF => None,
            Self::Pos(pos) => pos.get_underlying_var_raw(),
        }
    }

    fn min_raw(self, vars: &Vars) -> Val {
        match self {
            Self::Neg(neg) => neg.min_raw(vars),
            Self::ZeroI => Val::ValI(0).min_raw(vars),
            Self::ZeroF => Val::ValF(0.0).min_raw(vars),
            Self::Pos(pos) => pos.min_raw(vars),
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        match self {
            Self::Neg(neg) => neg.max_raw(vars),
            Self::ZeroI => Val::ValI(0).max_raw(vars),
            Self::ZeroF => Val::ValF(0.0).max_raw(vars),
            Self::Pos(pos) => pos.max_raw(vars),
        }
    }
}

impl<V: View> View for Times<V> {
    fn result_type(self, ctx: &Context) -> ViewType {
        match self {
            Self::Neg(neg) => neg.result_type(ctx),
            Self::ZeroI => ViewType::Integer,
            Self::ZeroF => ViewType::Float,
            Self::Pos(pos) => pos.result_type(ctx),
        }
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        match self {
            Self::Neg(neg) => neg.try_set_min(min, ctx),
            Self::ZeroI => match min {
                Val::ValI(min_val) => Val::ValI(0).try_set_min(Val::ValI(min_val), ctx),
                Val::ValF(min_val) => Val::ValI(0).try_set_min(Val::ValF(min_val), ctx),
            },
            Self::ZeroF => match min {
                Val::ValI(min_val) => Val::ValF(0.0).try_set_min(Val::ValI(min_val), ctx),
                Val::ValF(min_val) => Val::ValF(0.0).try_set_min(Val::ValF(min_val), ctx),
            },
            Self::Pos(pos) => pos.try_set_min(min, ctx),
        }
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        match self {
            Self::Neg(neg) => neg.try_set_max(max, ctx),
            Self::ZeroI => match max {
                Val::ValI(max_val) => Val::ValI(0).try_set_max(Val::ValI(max_val), ctx),
                Val::ValF(max_val) => Val::ValI(0).try_set_max(Val::ValF(max_val), ctx),
            },
            Self::ZeroF => match max {
                Val::ValI(max_val) => Val::ValF(0.0).try_set_max(Val::ValI(max_val), ctx),
                Val::ValF(max_val) => Val::ValF(0.0).try_set_max(Val::ValF(max_val), ctx),
            },
            Self::Pos(pos) => pos.try_set_max(max, ctx),
        }
    }
}

#[doc(hidden)]
/// Scale the underlying view by a strictly positive constant factor.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct TimesPos<V> {
    x: V,
    scale_pos: Val,
}

impl<V: std::fmt::Debug> std::fmt::Debug for TimesPos<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimesPos(x: {:?}, scale: {:?})", self.x, self.scale_pos)
    }
}

impl<V: View> TimesPos<V> {
    const fn new(x: V, scale_pos: Val) -> Self {
        match scale_pos {
            Val::ValI(_) => {
                Self { x, scale_pos }
            }
            Val::ValF(_) => {
                Self { x, scale_pos }
            }
        }
    }
}

impl<V: View> ViewRaw for TimesPos<V> {
    fn get_underlying_var_raw(self) -> Option<VarId> {
        self.x.get_underlying_var_raw()
    }

    fn min_raw(self, vars: &Vars) -> Val {
        match (self.x.min_raw(vars), self.scale_pos) {
            (Val::ValI(min), Val::ValI(scale)) => Val::ValI(min * scale),
            (Val::ValF(min), Val::ValF(scale)) => Val::ValF(min * scale),
            // Mixed type cases with automatic conversion to float
            (Val::ValI(min), Val::ValF(scale)) => Val::ValF(min as f64 * scale),
            (Val::ValF(min), Val::ValI(scale)) => Val::ValF(min * scale as f64),
        }
    }

    fn max_raw(self, vars: &Vars) -> Val {
        match (self.x.max_raw(vars), self.scale_pos) {
            (Val::ValI(max), Val::ValI(scale)) => Val::ValI(max * scale),
            (Val::ValF(max), Val::ValF(scale)) => Val::ValF(max * scale),
            // Mixed type cases with automatic conversion to float
            (Val::ValI(max), Val::ValF(scale)) => Val::ValF(max as f64 * scale),
            (Val::ValF(max), Val::ValI(scale)) => Val::ValF(max * scale as f64),
        }
    }
}

impl<V: View> View for TimesPos<V> {
    fn result_type(self, ctx: &Context) -> ViewType {
        // TimesPos operation can promote integers to floats if scale is float
        let base_type = self.x.result_type(ctx);
        let scale_type = match self.scale_pos {
            Val::ValI(_) => ViewType::Integer,
            Val::ValF(_) => ViewType::Float,
        };
        
        // If either operand is float, result is float
        match (base_type, scale_type) {
            (ViewType::Float, _) | (_, ViewType::Float) => ViewType::Float,
            (ViewType::Integer, ViewType::Integer) => ViewType::Integer,
        }
    }

    fn try_set_min(self, min: Val, ctx: &mut Context) -> Option<Val> {
        match (min, self.scale_pos) {
            (Val::ValI(min_val), Val::ValI(scale)) => {
                // For positive scaling: min = x * scale, so x >= min / scale
                // Use ceiling division for minimum bound
                let required_min = (min_val + scale - 1) / scale; // ceiling division
                self.x.try_set_min(Val::ValI(required_min), ctx)
            }
            (Val::ValF(min_val), Val::ValF(scale)) => {
                // For floating point, direct division
                let required_min = min_val / scale;
                self.x.try_set_min(Val::ValF(required_min), ctx)
            }
            // Mixed type cases with automatic conversion
            (Val::ValI(min_val), Val::ValF(scale)) => {
                // Convert to float and divide
                let required_min = min_val as f64 / scale;
                self.x.try_set_min(Val::ValF(required_min), ctx)
            }
            (Val::ValF(min_val), Val::ValI(scale)) => {
                // Convert scale to float and divide
                let required_min = min_val / scale as f64;
                self.x.try_set_min(Val::ValF(required_min), ctx)
            }
        }
    }

    fn try_set_max(self, max: Val, ctx: &mut Context) -> Option<Val> {
        match (max, self.scale_pos) {
            (Val::ValI(max_val), Val::ValI(scale)) => {
                // For positive scaling: max = x * scale, so x <= max / scale
                // Use floor division for maximum bound
                let required_max = max_val / scale; // floor division
                self.x.try_set_max(Val::ValI(required_max), ctx)
            }
            (Val::ValF(max_val), Val::ValF(scale)) => {
                // For floating point, direct division
                let required_max = max_val / scale;
                self.x.try_set_max(Val::ValF(required_max), ctx)
            }
            // Mixed type cases with automatic conversion
            (Val::ValI(max_val), Val::ValF(scale)) => {
                // Convert to float and divide
                let required_max = max_val as f64 / scale;
                self.x.try_set_max(Val::ValF(required_max), ctx)
            }
            (Val::ValF(max_val), Val::ValI(scale)) => {
                // Convert scale to float and divide
                let required_max = max_val / scale as f64;
                self.x.try_set_max(Val::ValF(required_max), ctx)
            }
        }
    }
}

/// Scale the underlying view by a strictly negative constant factor.
pub type TimesNeg<V> = TimesPos<Opposite<V>>;
