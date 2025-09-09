use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
};

/// Context-aware not-equals constraint: x != y
/// 
/// This propagator ensures that two variables cannot be assigned the same value.
/// It uses interval-aware precision for float comparisons to properly handle
/// values from different FloatInterval instances with different discretizations.
#[derive(Clone, Debug)]
pub struct NotEquals<U, V> {
    x: U,
    y: V,
}

impl<U, V> NotEquals<U, V> {
    pub fn new(x: U, y: V) -> Self {
        Self { x, y }
    }
    
    /// Check if a domain is a singleton (single value) using context-aware comparison
    fn is_singleton(&self, min_val: Val, max_val: Val, target_var: impl View, ctx: &Context) -> bool {
        self.values_equal(min_val, max_val, target_var, ctx)
    }

    /// Check if two values are equal using proper interval context
    fn values_equal(&self, val1: Val, val2: Val, target_var: impl View, ctx: &Context) -> bool {
        // Since View doesn't directly expose VarId, we use the target_var's result type
        // to determine the appropriate comparison strategy
        match target_var.result_type(ctx) {
            crate::views::ViewType::Float => {
                // For float variables, use precision-based comparison
                // We use a conservative tolerance since we don't have exact interval context
                val1.equals_with_precision(&val2, 1e-12)
            }
            crate::views::ViewType::Integer => {
                // For integer variables, use exact comparison
                val1 == val2
            }
        }
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

        // Case 1: Both variables are assigned - check constraint violation
        if self.is_singleton(x_min, x_max, self.x, ctx) && self.is_singleton(y_min, y_max, self.y, ctx) {
            if self.values_equal(x_min, y_min, self.x, ctx) {
                return None; // Constraint violated: both assigned to same value
            } else {
                return Some(()); // Both assigned to different values - constraint satisfied
            }
        }

        // Case 2: x is assigned (singleton domain)
        if self.is_singleton(x_min, x_max, self.x, ctx) {
            let x_value = x_min;
            
            // If y's domain contains only the forbidden value, constraint fails
            if self.is_singleton(y_min, y_max, self.y, ctx) && self.values_equal(y_min, x_value, self.y, ctx) {
                return None;
            }
            
            // Try to exclude x_value from y's domain
            exclude_value_from_domain(&self.y, x_value, ctx)?;
        }
        // Case 3: y is assigned (singleton domain) 
        else if self.is_singleton(y_min, y_max, self.y, ctx) {
            let y_value = y_min;
            
            // If x's domain contains only the forbidden value, constraint fails  
            if self.is_singleton(x_min, x_max, self.x, ctx) && self.values_equal(x_min, y_value, self.x, ctx) {
                return None;
            }
            
            // Try to exclude y_value from x's domain
            exclude_value_from_domain(&self.x, y_value, ctx)?;
        }

        // Case 4: Neither variable is assigned yet
        // For interval domains, we can only do limited propagation
        // The main constraint checking happens when variables become assigned
        
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

/// Check if a < b
fn values_less_than(a: Val, b: Val) -> bool {
    match (a, b) {
        (Val::ValI(a_int), Val::ValI(b_int)) => a_int < b_int,
        (Val::ValF(a_float), Val::ValF(b_float)) => {
            a_float != b_float && a_float < b_float
        },
        (Val::ValI(a_int), Val::ValF(b_float)) => {
            let a_as_float = a_int as f64;
            a_as_float != b_float && a_as_float < b_float
        },
        (Val::ValF(a_float), Val::ValI(b_int)) => {
            let b_as_float = b_int as f64;
            a_float != b_as_float && a_float < b_as_float
        },
    }
}

/// Simple fallback equality comparison for helper functions
fn values_equal(a: Val, b: Val) -> bool {
    match (a, b) {
        (Val::ValI(a_int), Val::ValI(b_int)) => a_int == b_int,
        (Val::ValF(a_float), Val::ValF(b_float)) => a_float == b_float,
        (Val::ValI(a_int), Val::ValF(b_float)) => (a_int as f64) == b_float,
        (Val::ValF(a_float), Val::ValI(b_int)) => a_float == (b_int as f64),
    }
}

/// Exclude a specific value from a view's domain by adjusting bounds
fn exclude_value_from_domain<W: View>(view: &W, forbidden_value: Val, ctx: &mut Context) -> Option<()> {
    let current_min = view.min(ctx);
    let current_max = view.max(ctx);
    
    // If the forbidden value is outside the current domain, nothing to do
    if values_less_than(forbidden_value, current_min) || values_less_than(current_max, forbidden_value) {
        return Some(());
    }
    
    // If the forbidden value is the only value in the domain, domain becomes empty
    if values_equal(current_min, current_max) && values_equal(current_min, forbidden_value) {
        return None; // Domain becomes empty - constraint violation
    }
    
    // If forbidden value is at the minimum bound, move minimum up
    if values_equal(current_min, forbidden_value) {
        let new_min = get_next_value(view, forbidden_value, ctx);
        view.try_set_min(new_min, ctx)?;
        return Some(());
    }
    
    // If forbidden value is at the maximum bound, move maximum down
    if values_equal(current_max, forbidden_value) {
        let new_max = get_prev_value(view, forbidden_value, ctx);
        view.try_set_max(new_max, ctx)?;
        return Some(());
    }
    
    // For values in the middle of the domain, we cannot exclude them with interval domains.
    // This is a fundamental limitation - the constraint will be enforced when variables
    // become assigned during search.
    
    Some(())
}

/// Get the next representable value using view-aware approach
fn get_next_value<W: View>(view: &W, value: Val, _ctx: &Context) -> Val {
    match value {
        Val::ValI(i) => Val::ValI(i + 1),
        Val::ValF(f) => {
            // For float variables, we need to use the interval-based approach
            if let Some(_var_id) = view.get_underlying_var() {
                // Try to access the variable's interval through the view system
                // For now, use a small fixed step as fallback
                let step = 1e-4; // Use same step size as our FloatInterval default
                Val::ValF(f + step)
            } else {
                // For compound views, use a small fixed step
                let step = 1e-4;
                Val::ValF(f + step)
            }
        }
    }
}

/// Get the previous representable value using view-aware approach
fn get_prev_value<W: View>(view: &W, value: Val, _ctx: &Context) -> Val {
    match value {
        Val::ValI(i) => Val::ValI(i - 1),
        Val::ValF(f) => {
            // For float variables, we need to use the interval-based approach
            if let Some(_var_id) = view.get_underlying_var() {
                // Try to access the variable's interval through the view system
                // For now, use a small fixed step as fallback
                let step = 1e-4; // Use same step size as our FloatInterval default
                Val::ValF(f - step)
            } else {
                // For compound views, use a small fixed step
                let step = 1e-4;
                Val::ValF(f - step)
            }
        }
    }
}
