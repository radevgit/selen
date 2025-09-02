use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
    utils::{float_equal, float_next, float_prev},
};

/// ULP-aware not-equals constraint: x != y
/// 
/// This propagator ensures that two variables cannot be assigned the same value.
/// It uses ULP-based precision for float comparisons to avoid floating-point
/// precision issues while maintaining exact semantics for integers.
#[derive(Clone, Debug)]
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
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);

        // Early return if domains don't overlap - constraint already satisfied
        if !domains_overlap(x_min, x_max, y_min, y_max) {
            return Some(());
        }

        // Case 1: x is assigned (singleton domain)
        if is_singleton(x_min, x_max) {
            let x_value = x_min;
            
            // Check for violation: both assigned to same value
            if is_singleton(y_min, y_max) && values_equal(x_value, y_min) {
                return None; // Constraint violated
            }
            
            // Exclude x_value from y's domain
            exclude_value_from_domain(&self.y, x_value, ctx)?;
        }
        // Case 2: y is assigned (singleton domain) 
        else if is_singleton(y_min, y_max) {
            let y_value = y_min;
            
            // Exclude y_value from x's domain
            exclude_value_from_domain(&self.x, y_value, ctx)?;
        }

        // Case 3: Neither variable is assigned yet
        // The constraint will be checked again when one becomes assigned
        
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

// Helper functions

/// Check if two domains overlap using ULP-aware comparison
fn domains_overlap(x_min: Val, x_max: Val, y_min: Val, y_max: Val) -> bool {
    // Domains overlap if: x_max >= y_min AND y_max >= x_min
    !values_less_than(x_max, y_min) && !values_less_than(y_max, x_min)
}

/// Check if a domain is a singleton (single value) using ULP-aware comparison
fn is_singleton(min_val: Val, max_val: Val) -> bool {
    values_equal(min_val, max_val)
}

/// Check if two values are equal using ULP-aware comparison
fn values_equal(a: Val, b: Val) -> bool {
    match (a, b) {
        (Val::ValI(a_int), Val::ValI(b_int)) => a_int == b_int,
        (Val::ValF(a_float), Val::ValF(b_float)) => float_equal(a_float, b_float),
        (Val::ValI(a_int), Val::ValF(b_float)) => float_equal(a_int as f32, b_float),
        (Val::ValF(a_float), Val::ValI(b_int)) => float_equal(a_float, b_int as f32),
    }
}

/// Check if a < b using ULP-aware comparison
fn values_less_than(a: Val, b: Val) -> bool {
    match (a, b) {
        (Val::ValI(a_int), Val::ValI(b_int)) => a_int < b_int,
        (Val::ValF(a_float), Val::ValF(b_float)) => {
            !float_equal(a_float, b_float) && a_float < b_float
        },
        (Val::ValI(a_int), Val::ValF(b_float)) => {
            let a_as_float = a_int as f32;
            !float_equal(a_as_float, b_float) && a_as_float < b_float
        },
        (Val::ValF(a_float), Val::ValI(b_int)) => {
            let b_as_float = b_int as f32;
            !float_equal(a_float, b_as_float) && a_float < b_as_float
        },
    }
}

/// Exclude a specific value from a view's domain by adjusting bounds
fn exclude_value_from_domain<W: View>(view: &W, forbidden_value: Val, ctx: &mut Context) -> Option<()> {
    let current_min = view.min(ctx);
    let current_max = view.max(ctx);
    
    // If forbidden value is at the minimum bound, move minimum up
    if values_equal(current_min, forbidden_value) {
        let new_min = get_next_value(forbidden_value);
        view.try_set_min(new_min, ctx)?;
    }
    
    // If forbidden value is at the maximum bound, move maximum down
    if values_equal(current_max, forbidden_value) {
        let new_max = get_prev_value(forbidden_value);
        view.try_set_max(new_max, ctx)?;
    }
    
    Some(())
}

/// Get the next representable value using ULP-based approach
fn get_next_value(value: Val) -> Val {
    match value {
        Val::ValI(i) => Val::ValI(i + 1),
        Val::ValF(f) => Val::ValF(float_next(f)),
    }
}

/// Get the previous representable value using ULP-based approach
fn get_prev_value(value: Val) -> Val {
    match value {
        Val::ValI(i) => Val::ValI(i - 1),
        Val::ValF(f) => Val::ValF(float_prev(f)),
    }
}
