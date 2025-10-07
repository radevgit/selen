//! Linear constraint propagators
//!
//! This module implements propagators for linear constraints of the form:
//! - `sum(coeffs[i] * vars[i]) = constant` (equality)
//! - `sum(coeffs[i] * vars[i]) ≤ constant` (less-or-equal)
//! - `sum(coeffs[i] * vars[i]) ≠ constant` (not-equal)
//!
//! Both reified and non-reified versions are supported.

use crate::constraints::props::{Prune, Propagate};
use crate::variables::{VarId, Val};
use crate::variables::views::{Context, View};

// ═══════════════════════════════════════════════════════════════════════
// Integer Linear Equality: sum(coeffs[i] * vars[i]) = constant
// ═══════════════════════════════════════════════════════════════════════

/// Integer linear equality constraint propagator
#[derive(Clone, Debug)]
pub struct IntLinEq {
    coefficients: Vec<i32>,
    variables: Vec<VarId>,
    constant: i32,
}

impl IntLinEq {
    pub fn new(coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32) -> Self {
        Self { coefficients, variables, constant }
    }
}

impl Prune for IntLinEq {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Bounds propagation for linear equality
        // For each variable x_i, compute bounds based on other variables
        
        for i in 0..self.variables.len() {
            let var_id = self.variables[i];
            let coeff = self.coefficients[i];
            
            if coeff == 0 {
                continue;
            }
            
            // Calculate sum of other terms: sum(coeff[j] * vars[j]) for j ≠ i
            let mut min_other = 0i32;
            let mut max_other = 0i32;
            
            for j in 0..self.variables.len() {
                if i == j {
                    continue;
                }
                
                let other_var = self.variables[j];
                let other_coeff = self.coefficients[j];
                
                let lb = other_var.min(ctx);
                let ub = other_var.max(ctx);
                
                let (min_term, max_term) = match (lb, ub) {
                    (Val::ValI(l), Val::ValI(u)) => {
                        if other_coeff > 0 {
                            (other_coeff * l, other_coeff * u)
                        } else {
                            (other_coeff * u, other_coeff * l)
                        }
                    }
                    _ => return Some(()), // Float variables not supported in int_lin
                };
                
                min_other = min_other.saturating_add(min_term);
                max_other = max_other.saturating_add(max_term);
            }
            
            // x_i must satisfy: coeff * x_i = constant - sum_other
            // So: x_i = (constant - sum_other) / coeff
            
            let target_min = self.constant.saturating_sub(max_other);
            let target_max = self.constant.saturating_sub(min_other);
            
            let (new_min, new_max) = if coeff > 0 {
                // x_i ∈ [target_min / coeff, target_max / coeff]
                let min_val = target_min.div_euclid(coeff);
                let max_val = target_max.div_euclid(coeff);
                (min_val, max_val)
            } else {
                // Negative coefficient: flip the bounds
                let min_val = target_max.div_euclid(coeff);
                let max_val = target_min.div_euclid(coeff);
                (min_val, max_val)
            };
            
            var_id.try_set_min(Val::ValI(new_min), ctx)?;
            var_id.try_set_max(Val::ValI(new_max), ctx)?;
        }
        
        Some(())
    }
}

impl Propagate for IntLinEq {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Integer Linear Less-or-Equal: sum(coeffs[i] * vars[i]) ≤ constant
// ═══════════════════════════════════════════════════════════════════════

/// Integer linear less-or-equal constraint propagator
#[derive(Clone, Debug)]
pub struct IntLinLe {
    coefficients: Vec<i32>,
    variables: Vec<VarId>,
    constant: i32,
}

impl IntLinLe {
    pub fn new(coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32) -> Self {
        Self { coefficients, variables, constant }
    }
}

impl Prune for IntLinLe {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Bounds propagation for linear inequality
        
        for i in 0..self.variables.len() {
            let var_id = self.variables[i];
            let coeff = self.coefficients[i];
            
            if coeff == 0 {
                continue;
            }
            
            // Calculate minimum sum of other terms
            let mut min_other = 0i32;
            
            for j in 0..self.variables.len() {
                if i == j {
                    continue;
                }
                
                let other_var = self.variables[j];
                let other_coeff = self.coefficients[j];
                
                let lb = other_var.min(ctx);
                let ub = other_var.max(ctx);
                
                let min_term = match (lb, ub) {
                    (Val::ValI(l), Val::ValI(u)) => {
                        if other_coeff > 0 {
                            other_coeff * l
                        } else {
                            other_coeff * u
                        }
                    }
                    _ => return Some(()),
                };
                
                min_other = min_other.saturating_add(min_term);
            }
            
            // coeff * x_i ≤ constant - min_other
            let remaining = self.constant.saturating_sub(min_other);
            
            if coeff > 0 {
                // x_i ≤ remaining / coeff
                let max_val = remaining.div_euclid(coeff);
                var_id.try_set_max(Val::ValI(max_val), ctx)?;
            } else {
                // Negative coefficient: x_i ≥ remaining / coeff
                let min_val = remaining.div_euclid(coeff);
                var_id.try_set_min(Val::ValI(min_val), ctx)?;
            }
        }
        
        Some(())
    }
}

impl Propagate for IntLinLe {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Integer Linear Not-Equal: sum(coeffs[i] * vars[i]) ≠ constant
// ═══════════════════════════════════════════════════════════════════════

/// Integer linear not-equal constraint propagator
#[derive(Clone, Debug)]
pub struct IntLinNe {
    coefficients: Vec<i32>,
    variables: Vec<VarId>,
    constant: i32,
}

impl IntLinNe {
    pub fn new(coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32) -> Self {
        Self { coefficients, variables, constant }
    }
}

impl Prune for IntLinNe {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // Check if all but one variable are fixed
        let mut unfixed_idx = None;
        let mut fixed_sum = 0i32;
        
        for i in 0..self.variables.len() {
            let var_id = self.variables[i];
            let coeff = self.coefficients[i];
            
            let lb = var_id.min(ctx);
            let ub = var_id.max(ctx);
            
            match (lb, ub) {
                (Val::ValI(l), Val::ValI(u)) if l == u => {
                    // Variable is fixed
                    fixed_sum = fixed_sum.saturating_add(coeff * l);
                }
                (Val::ValI(_), Val::ValI(_)) => {
                    // Variable not fixed
                    if unfixed_idx.is_some() {
                        // More than one unfixed variable, can't propagate
                        return Some(());
                    }
                    unfixed_idx = Some(i);
                }
                _ => return Some(()),
            }
        }
        
        // If all variables are fixed, check if constraint is satisfied
        if unfixed_idx.is_none() {
            if fixed_sum == self.constant {
                return None; // Constraint violated: sum = constant
            }
            return Some(()); // Constraint satisfied: sum ≠ constant
        }
        
        // Exactly one unfixed variable
        let idx = unfixed_idx.unwrap();
        let var_id = self.variables[idx];
        let coeff = self.coefficients[idx];
        
        if coeff == 0 {
            // Check if fixed_sum ≠ constant
            if fixed_sum == self.constant {
                return None; // Constraint violated
            }
            return Some(());
        }
        
        // coeff * x ≠ constant - fixed_sum
        // So x ≠ (constant - fixed_sum) / coeff
        let forbidden_value_num = self.constant.saturating_sub(fixed_sum);
        
        if forbidden_value_num % coeff == 0 {
            let forbidden = forbidden_value_num / coeff;
            exclude_value(var_id, Val::ValI(forbidden), ctx)?;
        }
        
        Some(())
    }
}

impl Propagate for IntLinNe {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Reified Integer Linear Constraints
// ═══════════════════════════════════════════════════════════════════════

/// Reified integer linear equality: `b ⟺ sum(coeffs[i] * vars[i]) = constant`
#[derive(Clone, Debug)]
pub struct IntLinEqReif {
    coefficients: Vec<i32>,
    variables: Vec<VarId>,
    constant: i32,
    reif_var: VarId,
}

impl IntLinEqReif {
    pub fn new(coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32, reif_var: VarId) -> Self {
        Self { coefficients, variables, constant, reif_var }
    }
}

impl Prune for IntLinEqReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let reif_min = self.reif_var.min(ctx);
        let reif_max = self.reif_var.max(ctx);
        
        // If reification variable is fixed to true, enforce the constraint
        if reif_min == Val::ValI(1) && reif_max == Val::ValI(1) {
            // Apply non-reified propagation
            return prune_int_lin_eq(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        // If reification variable is fixed to false, we can't directly enforce ≠
        // (that would require disjunctive reasoning beyond interval propagation)
        if reif_min == Val::ValI(0) && reif_max == Val::ValI(0) {
            // Check if constraint is definitely violated (all vars fixed and sum = constant)
            if let Some(sum) = compute_fixed_sum(&self.coefficients, &self.variables, ctx) {
                if sum == self.constant {
                    return None; // Contradiction: reif=false but constraint holds
                }
            }
            return Some(());
        }
        
        // Reification variable not fixed - check if we can determine its value
        if let Some(sum) = compute_fixed_sum(&self.coefficients, &self.variables, ctx) {
            if sum == self.constant {
                // Constraint definitely holds, set reif to true
                self.reif_var.try_set_min(Val::ValI(1), ctx)?;
                self.reif_var.try_set_max(Val::ValI(1), ctx)?;
            } else {
                // Constraint definitely fails, set reif to false
                self.reif_var.try_set_min(Val::ValI(0), ctx)?;
                self.reif_var.try_set_max(Val::ValI(0), ctx)?;
            }
        }
        
        Some(())
    }
}

impl Propagate for IntLinEqReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied().chain(std::iter::once(self.reif_var))
    }
}

/// Reified integer linear less-or-equal: `b ⟺ sum(coeffs[i] * vars[i]) ≤ constant`
#[derive(Clone, Debug)]
pub struct IntLinLeReif {
    coefficients: Vec<i32>,
    variables: Vec<VarId>,
    constant: i32,
    reif_var: VarId,
}

impl IntLinLeReif {
    pub fn new(coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32, reif_var: VarId) -> Self {
        Self { coefficients, variables, constant, reif_var }
    }
}

impl Prune for IntLinLeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let reif_min = self.reif_var.min(ctx);
        let reif_max = self.reif_var.max(ctx);
        
        // If reification variable is fixed to true, enforce the constraint
        if reif_min == Val::ValI(1) && reif_max == Val::ValI(1) {
            return prune_int_lin_le(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        // If reification variable is fixed to false, we can't directly enforce >
        if reif_min == Val::ValI(0) && reif_max == Val::ValI(0) {
            // Check if constraint is definitely violated
            if let Some(sum) = compute_fixed_sum(&self.coefficients, &self.variables, ctx) {
                if sum <= self.constant {
                    return None; // Contradiction: reif=false but constraint holds
                }
            }
            return Some(());
        }
        
        // Reification variable not fixed - check if we can determine its value
        let (min_sum, max_sum) = compute_sum_bounds(&self.coefficients, &self.variables, ctx)?;
        
        if max_sum <= self.constant {
            // Constraint definitely holds, set reif to true
            self.reif_var.try_set_min(Val::ValI(1), ctx)?;
            self.reif_var.try_set_max(Val::ValI(1), ctx)?;
        } else if min_sum > self.constant {
            // Constraint definitely fails, set reif to false
            self.reif_var.try_set_min(Val::ValI(0), ctx)?;
            self.reif_var.try_set_max(Val::ValI(0), ctx)?;
        }
        
        Some(())
    }
}

impl Propagate for IntLinLeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied().chain(std::iter::once(self.reif_var))
    }
}

/// Reified integer linear not-equal: `b ⟺ sum(coeffs[i] * vars[i]) ≠ constant`
#[derive(Clone, Debug)]
pub struct IntLinNeReif {
    coefficients: Vec<i32>,
    variables: Vec<VarId>,
    constant: i32,
    reif_var: VarId,
}

impl IntLinNeReif {
    pub fn new(coefficients: Vec<i32>, variables: Vec<VarId>, constant: i32, reif_var: VarId) -> Self {
        Self { coefficients, variables, constant, reif_var }
    }
}

impl Prune for IntLinNeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let reif_min = self.reif_var.min(ctx);
        let reif_max = self.reif_var.max(ctx);
        
        // If reification variable is fixed to true, enforce the constraint
        if reif_min == Val::ValI(1) && reif_max == Val::ValI(1) {
            return prune_int_lin_ne(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        // If reification variable is fixed to false, enforce the negation (equality)
        if reif_min == Val::ValI(0) && reif_max == Val::ValI(0) {
            return prune_int_lin_eq(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        // Reification variable not fixed - check if we can determine its value
        if let Some(sum) = compute_fixed_sum(&self.coefficients, &self.variables, ctx) {
            if sum != self.constant {
                // Constraint holds (≠), set reif to true
                self.reif_var.try_set_min(Val::ValI(1), ctx)?;
                self.reif_var.try_set_max(Val::ValI(1), ctx)?;
            } else {
                // Constraint fails (=), set reif to false
                self.reif_var.try_set_min(Val::ValI(0), ctx)?;
                self.reif_var.try_set_max(Val::ValI(0), ctx)?;
            }
        }
        
        Some(())
    }
}

impl Propagate for IntLinNeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied().chain(std::iter::once(self.reif_var))
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Float Linear Constraints
// ═══════════════════════════════════════════════════════════════════════

/// Float linear equality constraint propagator
#[derive(Clone, Debug)]
pub struct FloatLinEq {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    constant: f64,
}

impl FloatLinEq {
    pub fn new(coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64) -> Self {
        Self { coefficients, variables, constant }
    }
}

impl Prune for FloatLinEq {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // DEBUG: Enable for debugging
        let debug = std::env::var("DEBUG_FLOAT_LIN").is_ok();
        
        for i in 0..self.variables.len() {
            let var_id = self.variables[i];
            let coeff = self.coefficients[i];
            
            if coeff.abs() < 1e-12 {
                continue;
            }
            
            let mut min_other = 0.0;
            let mut max_other = 0.0;
            
            for j in 0..self.variables.len() {
                if i == j {
                    continue;
                }
                
                let other_var = self.variables[j];
                let other_coeff = self.coefficients[j];
                
                let lb = other_var.min(ctx);
                let ub = other_var.max(ctx);
                
                let (min_term, max_term) = match (lb, ub) {
                    (Val::ValF(l), Val::ValF(u)) => {
                        if other_coeff > 0.0 {
                            (other_coeff * l, other_coeff * u)
                        } else {
                            (other_coeff * u, other_coeff * l)
                        }
                    }
                    (Val::ValI(l), Val::ValI(u)) => {
                        let lf = l as f64;
                        let uf = u as f64;
                        if other_coeff > 0.0 {
                            (other_coeff * lf, other_coeff * uf)
                        } else {
                            (other_coeff * uf, other_coeff * lf)
                        }
                    }
                    _ => return Some(()),
                };
                
                min_other += min_term;
                max_other += max_term;
            }
            
            let target_min = self.constant - max_other;
            let target_max = self.constant - min_other;
            
            let (mut new_min, mut new_max) = if coeff > 0.0 {
                (target_min / coeff, target_max / coeff)
            } else {
                (target_max / coeff, target_min / coeff)
            };
            
            // Handle floating-point rounding: ensure new_min <= new_max
            // When constraints are tight (equality), new_min and new_max should be nearly equal
            // but may differ slightly due to floating-point arithmetic
            if new_min > new_max {
                if debug {
                    eprintln!("DEBUG: FloatLinEq swapping bounds: new_min={} > new_max={}", new_min, new_max);
                }
                // Swap if they're reversed due to rounding errors
                std::mem::swap(&mut new_min, &mut new_max);
            }
            
            // FIX: Clamp computed bounds to current bounds to handle accumulated precision errors
            // When propagating tight equality constraints, the computed bounds may slightly
            // violate the current bounds due to cascading precision errors from previous propagations
            let current_min = match var_id.min(ctx) {
                Val::ValF(f) => f,
                Val::ValI(i) => i as f64,
                _ => new_min, // Fallback to computed value if type mismatch
            };
            let current_max = match var_id.max(ctx) {
                Val::ValF(f) => f,
                Val::ValI(i) => i as f64,
                _ => new_max, // Fallback to computed value if type mismatch
            };
            
            // Only apply clamping if the difference is small (precision error, not real infeasibility)
            let tolerance = 1e-6;
            if new_max < current_min && (current_min - new_max) < tolerance {
                new_max = current_min;
            }
            if new_min > current_max && (new_min - current_max) < tolerance {
                new_min = current_max;
            }
            
            if debug {
                let cur_min = var_id.min(ctx);
                let cur_max = var_id.max(ctx);
                eprintln!("DEBUG: FloatLinEq var {:?}: current=[{:?}, {:?}], setting=[{}, {}]",
                    var_id, cur_min, cur_max, new_min, new_max);
            }
            
            // FIX: If variable is already fixed and computed min is close to current value,
            // skip update to avoid cascading precision errors. A fixed variable shouldn't
            // be perturbed by small precision errors in back-propagation.
            let is_fixed = (current_max - current_min).abs() < 1e-9;
            let min_close = (new_min - current_min).abs() < 1e-4;
            if is_fixed && min_close {
                // Variable already at the right value, no update needed
                if debug {
                    eprintln!("DEBUG: FloatLinEq skipping update for fixed var {:?} (current={}, new_min={})",
                        var_id, current_min, new_min);
                }
                continue;
            }
            
            let min_result = var_id.try_set_min(Val::ValF(new_min), ctx);
            if debug && min_result.is_none() {
                eprintln!("DEBUG: FloatLinEq FAILED on try_set_min({}) for var {:?}", new_min, var_id);
            }
            min_result?;
            
            let max_result = var_id.try_set_max(Val::ValF(new_max), ctx);
            if debug && max_result.is_none() {
                eprintln!("DEBUG: FloatLinEq FAILED on try_set_max({}) for var {:?}", new_max, var_id);
            }
            max_result?;
        }
        
        Some(())
    }
}

impl Propagate for FloatLinEq {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

/// Float linear less-or-equal constraint propagator
#[derive(Clone, Debug)]
pub struct FloatLinLe {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    constant: f64,
}

impl FloatLinLe {
    pub fn new(coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64) -> Self {
        Self { coefficients, variables, constant }
    }
}

impl Prune for FloatLinLe {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        for i in 0..self.variables.len() {
            let var_id = self.variables[i];
            let coeff = self.coefficients[i];
            
            if coeff.abs() < 1e-12 {
                continue;
            }
            
            let mut min_other = 0.0;
            
            for j in 0..self.variables.len() {
                if i == j {
                    continue;
                }
                
                let other_var = self.variables[j];
                let other_coeff = self.coefficients[j];
                
                let lb = other_var.min(ctx);
                let ub = other_var.max(ctx);
                
                let min_term = match (lb, ub) {
                    (Val::ValF(l), Val::ValF(u)) => {
                        if other_coeff > 0.0 {
                            other_coeff * l
                        } else {
                            other_coeff * u
                        }
                    }
                    (Val::ValI(l), Val::ValI(u)) => {
                        let lf = l as f64;
                        let uf = u as f64;
                        if other_coeff > 0.0 {
                            other_coeff * lf
                        } else {
                            other_coeff * uf
                        }
                    }
                    _ => return Some(()),
                };
                
                min_other += min_term;
            }
            
            let remaining = self.constant - min_other;
            
            if coeff > 0.0 {
                let max_val = remaining / coeff;
                // Only tighten if the new bound is finite and improves current bound
                if max_val.is_finite() {
                    if let Val::ValF(current_max) = var_id.max(ctx) {
                        if max_val < current_max {
                            var_id.try_set_max(Val::ValF(max_val), ctx)?;
                        }
                    }
                }
            } else {
                let min_val = remaining / coeff;
                // Normalize -0.0 to 0.0 to avoid negative zero artifacts
                // This can occur when remaining=0.0 and coeff<0, giving 0.0/-1.0 = -0.0
                let normalized_min = if min_val == 0.0 { 0.0 } else { min_val };
                // Only tighten if the new bound is finite and improves current bound
                if normalized_min.is_finite() {
                    if let Val::ValF(current_min) = var_id.min(ctx) {
                        if normalized_min > current_min {
                            var_id.try_set_min(Val::ValF(normalized_min), ctx)?;
                        }
                    }
                }
            }
        }
        
        Some(())
    }
}

impl Propagate for FloatLinLe {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

/// Float linear not-equal constraint propagator
#[derive(Clone, Debug)]
pub struct FloatLinNe {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    constant: f64,
}

impl FloatLinNe {
    pub fn new(coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64) -> Self {
        Self { coefficients, variables, constant }
    }
}

impl Prune for FloatLinNe {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let mut unfixed_idx = None;
        let mut fixed_sum = 0.0;
        
        for i in 0..self.variables.len() {
            let var_id = self.variables[i];
            let coeff = self.coefficients[i];
            
            let lb = var_id.min(ctx);
            let ub = var_id.max(ctx);
            
            match (lb, ub) {
                (Val::ValF(l), Val::ValF(u)) if (l - u).abs() < 1e-12 => {
                    fixed_sum += coeff * l;
                }
                (Val::ValI(l), Val::ValI(u)) if l == u => {
                    fixed_sum += coeff * (l as f64);
                }
                _ => {
                    if unfixed_idx.is_some() {
                        return Some(());
                    }
                    unfixed_idx = Some(i);
                }
            }
        }
        
        if unfixed_idx.is_none() {
            if (fixed_sum - self.constant).abs() < 1e-12 {
                return None;
            }
            return Some(());
        }
        
        let idx = unfixed_idx.unwrap();
        let var_id = self.variables[idx];
        let coeff = self.coefficients[idx];
        
        if coeff.abs() < 1e-12 {
            if (fixed_sum - self.constant).abs() < 1e-12 {
                return None;
            }
            return Some(());
        }
        
        let forbidden = (self.constant - fixed_sum) / coeff;
        exclude_value(var_id, Val::ValF(forbidden), ctx)?;
        
        Some(())
    }
}

impl Propagate for FloatLinNe {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Reified Float Linear Constraints
// ═══════════════════════════════════════════════════════════════════════

/// Reified float linear equality: `b ⟺ sum(coeffs[i] * vars[i]) = constant`
#[derive(Clone, Debug)]
pub struct FloatLinEqReif {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    constant: f64,
    reif_var: VarId,
}

impl FloatLinEqReif {
    pub fn new(coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64, reif_var: VarId) -> Self {
        Self { coefficients, variables, constant, reif_var }
    }
}

impl Prune for FloatLinEqReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let reif_min = self.reif_var.min(ctx);
        let reif_max = self.reif_var.max(ctx);
        
        if reif_min == Val::ValI(1) && reif_max == Val::ValI(1) {
            return prune_float_lin_eq(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        if reif_min == Val::ValI(0) && reif_max == Val::ValI(0) {
            if let Some(sum) = compute_fixed_sum_float(&self.coefficients, &self.variables, ctx) {
                if (sum - self.constant).abs() < 1e-12 {
                    return None;
                }
            }
            return Some(());
        }
        
        if let Some(sum) = compute_fixed_sum_float(&self.coefficients, &self.variables, ctx) {
            if (sum - self.constant).abs() < 1e-12 {
                self.reif_var.try_set_min(Val::ValI(1), ctx)?;
                self.reif_var.try_set_max(Val::ValI(1), ctx)?;
            } else {
                self.reif_var.try_set_min(Val::ValI(0), ctx)?;
                self.reif_var.try_set_max(Val::ValI(0), ctx)?;
            }
        }
        
        Some(())
    }
}

impl Propagate for FloatLinEqReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied().chain(std::iter::once(self.reif_var))
    }
}

/// Reified float linear less-or-equal: `b ⟺ sum(coeffs[i] * vars[i]) ≤ constant`
#[derive(Clone, Debug)]
pub struct FloatLinLeReif {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    constant: f64,
    reif_var: VarId,
}

impl FloatLinLeReif {
    pub fn new(coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64, reif_var: VarId) -> Self {
        Self { coefficients, variables, constant, reif_var }
    }
}

impl Prune for FloatLinLeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let reif_min = self.reif_var.min(ctx);
        let reif_max = self.reif_var.max(ctx);
        
        if reif_min == Val::ValI(1) && reif_max == Val::ValI(1) {
            return prune_float_lin_le(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        if reif_min == Val::ValI(0) && reif_max == Val::ValI(0) {
            if let Some(sum) = compute_fixed_sum_float(&self.coefficients, &self.variables, ctx) {
                if sum <= self.constant {
                    return None;
                }
            }
            return Some(());
        }
        
        let (min_sum, max_sum) = compute_sum_bounds_float(&self.coefficients, &self.variables, ctx)?;
        
        if max_sum <= self.constant {
            self.reif_var.try_set_min(Val::ValI(1), ctx)?;
            self.reif_var.try_set_max(Val::ValI(1), ctx)?;
        } else if min_sum > self.constant {
            self.reif_var.try_set_min(Val::ValI(0), ctx)?;
            self.reif_var.try_set_max(Val::ValI(0), ctx)?;
        }
        
        Some(())
    }
}

impl Propagate for FloatLinLeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied().chain(std::iter::once(self.reif_var))
    }
}

/// Reified float linear not-equal: `b ⟺ sum(coeffs[i] * vars[i]) ≠ constant`
#[derive(Clone, Debug)]
pub struct FloatLinNeReif {
    coefficients: Vec<f64>,
    variables: Vec<VarId>,
    constant: f64,
    reif_var: VarId,
}

impl FloatLinNeReif {
    pub fn new(coefficients: Vec<f64>, variables: Vec<VarId>, constant: f64, reif_var: VarId) -> Self {
        Self { coefficients, variables, constant, reif_var }
    }
}

impl Prune for FloatLinNeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let reif_min = self.reif_var.min(ctx);
        let reif_max = self.reif_var.max(ctx);
        
        if reif_min == Val::ValI(1) && reif_max == Val::ValI(1) {
            return prune_float_lin_ne(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        if reif_min == Val::ValI(0) && reif_max == Val::ValI(0) {
            return prune_float_lin_eq(&self.coefficients, &self.variables, self.constant, ctx);
        }
        
        if let Some(sum) = compute_fixed_sum_float(&self.coefficients, &self.variables, ctx) {
            if (sum - self.constant).abs() >= 1e-12 {
                self.reif_var.try_set_min(Val::ValI(1), ctx)?;
                self.reif_var.try_set_max(Val::ValI(1), ctx)?;
            } else {
                self.reif_var.try_set_min(Val::ValI(0), ctx)?;
                self.reif_var.try_set_max(Val::ValI(0), ctx)?;
            }
        }
        
        Some(())
    }
}

impl Propagate for FloatLinNeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.variables.iter().copied().chain(std::iter::once(self.reif_var))
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Helper functions
// ═══════════════════════════════════════════════════════════════════════

/// Helper to compute sum bounds for integer linear constraints
fn compute_sum_bounds(coefficients: &[i32], variables: &[VarId], ctx: &Context) -> Option<(i32, i32)> {
    let mut min_sum = 0i32;
    let mut max_sum = 0i32;
    
    for (&coeff, &var) in coefficients.iter().zip(variables.iter()) {
        let lb = var.min(ctx);
        let ub = var.max(ctx);
        
        match (lb, ub) {
            (Val::ValI(l), Val::ValI(u)) => {
                let (min_term, max_term) = if coeff > 0 {
                    (coeff * l, coeff * u)
                } else {
                    (coeff * u, coeff * l)
                };
                min_sum = min_sum.saturating_add(min_term);
                max_sum = max_sum.saturating_add(max_term);
            }
            _ => return None, // Float variables not supported
        }
    }
    
    Some((min_sum, max_sum))
}

/// Helper to compute sum if all variables are fixed
fn compute_fixed_sum(coefficients: &[i32], variables: &[VarId], ctx: &Context) -> Option<i32> {
    let mut sum = 0i32;
    
    for (&coeff, &var) in coefficients.iter().zip(variables.iter()) {
        let lb = var.min(ctx);
        let ub = var.max(ctx);
        
        match (lb, ub) {
            (Val::ValI(l), Val::ValI(u)) if l == u => {
                sum = sum.saturating_add(coeff * l);
            }
            _ => return None, // Variable not fixed
        }
    }
    
    Some(sum)
}

/// Helper to apply int_lin_eq propagation (extracted for reuse)
fn prune_int_lin_eq(coefficients: &[i32], variables: &[VarId], constant: i32, ctx: &mut Context) -> Option<()> {
    for i in 0..variables.len() {
        let var_id = variables[i];
        let coeff = coefficients[i];
        
        if coeff == 0 {
            continue;
        }
        
        let mut min_other = 0i32;
        let mut max_other = 0i32;
        
        for j in 0..variables.len() {
            if i == j {
                continue;
            }
            
            let other_var = variables[j];
            let other_coeff = coefficients[j];
            
            let lb = other_var.min(ctx);
            let ub = other_var.max(ctx);
            
            let (min_term, max_term) = match (lb, ub) {
                (Val::ValI(l), Val::ValI(u)) => {
                    if other_coeff > 0 {
                        (other_coeff * l, other_coeff * u)
                    } else {
                        (other_coeff * u, other_coeff * l)
                    }
                }
                _ => return Some(()),
            };
            
            min_other = min_other.saturating_add(min_term);
            max_other = max_other.saturating_add(max_term);
        }
        
        let target_min = constant.saturating_sub(max_other);
        let target_max = constant.saturating_sub(min_other);
        
        let (new_min, new_max) = if coeff > 0 {
            let min_val = target_min.div_euclid(coeff);
            let max_val = target_max.div_euclid(coeff);
            (min_val, max_val)
        } else {
            let min_val = target_max.div_euclid(coeff);
            let max_val = target_min.div_euclid(coeff);
            (min_val, max_val)
        };
        
        var_id.try_set_min(Val::ValI(new_min), ctx)?;
        var_id.try_set_max(Val::ValI(new_max), ctx)?;
    }
    
    Some(())
}

/// Helper to apply int_lin_le propagation (extracted for reuse)
fn prune_int_lin_le(coefficients: &[i32], variables: &[VarId], constant: i32, ctx: &mut Context) -> Option<()> {
    for i in 0..variables.len() {
        let var_id = variables[i];
        let coeff = coefficients[i];
        
        if coeff == 0 {
            continue;
        }
        
        let mut min_other = 0i32;
        
        for j in 0..variables.len() {
            if i == j {
                continue;
            }
            
            let other_var = variables[j];
            let other_coeff = coefficients[j];
            
            let lb = other_var.min(ctx);
            let ub = other_var.max(ctx);
            
            let min_term = match (lb, ub) {
                (Val::ValI(l), Val::ValI(u)) => {
                    if other_coeff > 0 {
                        other_coeff * l
                    } else {
                        other_coeff * u
                    }
                }
                _ => return Some(()),
            };
            
            min_other = min_other.saturating_add(min_term);
        }
        
        let remaining = constant.saturating_sub(min_other);
        
        if coeff > 0 {
            let max_val = remaining.div_euclid(coeff);
            var_id.try_set_max(Val::ValI(max_val), ctx)?;
        } else {
            let min_val = remaining.div_euclid(coeff);
            var_id.try_set_min(Val::ValI(min_val), ctx)?;
        }
    }
    
    Some(())
}

/// Helper to apply int_lin_ne propagation (extracted for reuse)
fn prune_int_lin_ne(coefficients: &[i32], variables: &[VarId], constant: i32, ctx: &mut Context) -> Option<()> {
    let mut unfixed_idx = None;
    let mut fixed_sum = 0i32;
    
    for i in 0..variables.len() {
        let var_id = variables[i];
        let coeff = coefficients[i];
        
        let lb = var_id.min(ctx);
        let ub = var_id.max(ctx);
        
        match (lb, ub) {
            (Val::ValI(l), Val::ValI(u)) if l == u => {
                fixed_sum = fixed_sum.saturating_add(coeff * l);
            }
            (Val::ValI(_), Val::ValI(_)) => {
                if unfixed_idx.is_some() {
                    return Some(());
                }
                unfixed_idx = Some(i);
            }
            _ => return Some(()),
        }
    }
    
    if unfixed_idx.is_none() {
        if fixed_sum == constant {
            return None;
        }
        return Some(());
    }
    
    let idx = unfixed_idx.unwrap();
    let var_id = variables[idx];
    let coeff = coefficients[idx];
    
    if coeff == 0 {
        if fixed_sum == constant {
            return None;
        }
        return Some(());
    }
    
    let forbidden_value_num = constant.saturating_sub(fixed_sum);
    
    if forbidden_value_num % coeff == 0 {
        let forbidden = forbidden_value_num / coeff;
        exclude_value(var_id, Val::ValI(forbidden), ctx)?;
    }
    
    Some(())
}

/// Exclude a specific value from a variable's domain by adjusting bounds
fn exclude_value(var_id: VarId, forbidden_value: Val, ctx: &mut Context) -> Option<()> {
    let current_min = var_id.min(ctx);
    let current_max = var_id.max(ctx);
    
    // If the forbidden value is outside the current domain, nothing to do
    if forbidden_value < current_min || forbidden_value > current_max {
        return Some(());
    }
    
    // If the forbidden value is the only value in the domain, domain becomes empty
    if current_min == current_max && current_min == forbidden_value {
        return None; // Domain becomes empty - constraint violation
    }
    
    // If forbidden value is at the minimum bound, move minimum up
    if current_min == forbidden_value {
        let new_min = match forbidden_value {
            Val::ValI(i) => Val::ValI(i + 1),
            Val::ValF(f) => Val::ValF(f + 1e-4),
        };
        var_id.try_set_min(new_min, ctx)?;
        return Some(());
    }
    
    // If forbidden value is at the maximum bound, move maximum down
    if current_max == forbidden_value {
        let new_max = match forbidden_value {
            Val::ValI(i) => Val::ValI(i - 1),
            Val::ValF(f) => Val::ValF(f - 1e-4),
        };
        var_id.try_set_max(new_max, ctx)?;
        return Some(());
    }
    
    // For values in the middle of the domain, we cannot exclude them with interval domains.
    // This is a fundamental limitation - the constraint will be enforced when variables
    // become assigned during search.
    
    Some(())
}

// Float linear constraint helpers

/// Helper to compute sum bounds for float linear constraints
fn compute_sum_bounds_float(coefficients: &[f64], variables: &[VarId], ctx: &Context) -> Option<(f64, f64)> {
    let mut min_sum = 0.0;
    let mut max_sum = 0.0;
    
    for (&coeff, &var) in coefficients.iter().zip(variables.iter()) {
        let lb = var.min(ctx);
        let ub = var.max(ctx);
        
        let (min_term, max_term) = match (lb, ub) {
            (Val::ValF(l), Val::ValF(u)) => {
                if coeff > 0.0 {
                    (coeff * l, coeff * u)
                } else {
                    (coeff * u, coeff * l)
                }
            }
            (Val::ValI(l), Val::ValI(u)) => {
                let lf = l as f64;
                let uf = u as f64;
                if coeff > 0.0 {
                    (coeff * lf, coeff * uf)
                } else {
                    (coeff * uf, coeff * lf)
                }
            }
            _ => return None,
        };
        
        min_sum += min_term;
        max_sum += max_term;
    }
    
    Some((min_sum, max_sum))
}

/// Helper to compute sum if all variables are fixed (float version)
fn compute_fixed_sum_float(coefficients: &[f64], variables: &[VarId], ctx: &Context) -> Option<f64> {
    let mut sum = 0.0;
    
    for (&coeff, &var) in coefficients.iter().zip(variables.iter()) {
        let lb = var.min(ctx);
        let ub = var.max(ctx);
        
        match (lb, ub) {
            (Val::ValF(l), Val::ValF(u)) if (l - u).abs() < 1e-12 => {
                sum += coeff * l;
            }
            (Val::ValI(l), Val::ValI(u)) if l == u => {
                sum += coeff * (l as f64);
            }
            _ => return None,
        }
    }
    
    Some(sum)
}

/// Helper to apply float_lin_eq propagation (extracted for reuse)
fn prune_float_lin_eq(coefficients: &[f64], variables: &[VarId], constant: f64, ctx: &mut Context) -> Option<()> {
    for i in 0..variables.len() {
        let var_id = variables[i];
        let coeff = coefficients[i];
        
        if coeff.abs() < 1e-12 {
            continue;
        }
        
        let mut min_other = 0.0;
        let mut max_other = 0.0;
        let mut has_unbounded_other = false;
        
        for j in 0..variables.len() {
            if i == j {
                continue;
            }
            
            let other_var = variables[j];
            let other_coeff = coefficients[j];
            
            let lb = other_var.min(ctx);
            let ub = other_var.max(ctx);
            
            let (min_term, max_term) = match (lb, ub) {
                (Val::ValF(l), Val::ValF(u)) => {
                    // Check if this variable is unbounded
                    if l.is_infinite() || u.is_infinite() {
                        // Can't compute meaningful bounds if any other variable is unbounded
                        has_unbounded_other = true;
                        break;
                    }
                    if other_coeff > 0.0 {
                        (other_coeff * l, other_coeff * u)
                    } else {
                        (other_coeff * u, other_coeff * l)
                    }
                }
                (Val::ValI(l), Val::ValI(u)) => {
                    let lf = l as f64;
                    let uf = u as f64;
                    if other_coeff > 0.0 {
                        (other_coeff * lf, other_coeff * uf)
                    } else {
                        (other_coeff * uf, other_coeff * lf)
                    }
                }
                _ => return Some(()),
            };
            
            min_other += min_term;
            max_other += max_term;
        }
        
        // Skip propagation for this variable if other variables are unbounded
        if has_unbounded_other {
            continue;
        }
        
        let target_min = constant - max_other;
        let target_max = constant - min_other;
        
        let (mut new_min, mut new_max) = if coeff > 0.0 {
            (target_min / coeff, target_max / coeff)
        } else {
            (target_max / coeff, target_min / coeff)
        };
        
        // Handle floating-point rounding: ensure new_min <= new_max
        if new_min > new_max {
            std::mem::swap(&mut new_min, &mut new_max);
        }
        
        // FIX: Clamp computed bounds to current bounds to handle accumulated precision errors
        let current_min = match var_id.min(ctx) {
            Val::ValF(f) => f,
            Val::ValI(i) => i as f64,
            _ => new_min,
        };
        let current_max = match var_id.max(ctx) {
            Val::ValF(f) => f,
            Val::ValI(i) => i as f64,
            _ => new_max,
        };
        
        let tolerance = 1e-6;
        if new_max < current_min && (current_min - new_max) < tolerance {
            new_max = current_min;
        }
        if new_min > current_max && (new_min - current_max) < tolerance {
            new_min = current_max;
        }
        
        // FIX: If variable is already fixed and computed min is close to current value,
        // skip update to avoid cascading precision errors in constraint chains
        let is_fixed = (current_max - current_min).abs() < 1e-9;
        let min_close = (new_min - current_min).abs() < 1e-4;
        if is_fixed && min_close {
            // Variable already at the right value, no update needed
            continue;
        }
        
        var_id.try_set_min(Val::ValF(new_min), ctx)?;
        var_id.try_set_max(Val::ValF(new_max), ctx)?;
    }
    
    Some(())
}

/// Helper to apply float_lin_le propagation (extracted for reuse)
fn prune_float_lin_le(coefficients: &[f64], variables: &[VarId], constant: f64, ctx: &mut Context) -> Option<()> {
    for i in 0..variables.len() {
        let var_id = variables[i];
        let coeff = coefficients[i];
        
        if coeff.abs() < 1e-12 {
            continue;
        }
        
        let mut min_other = 0.0;
        let mut has_unbounded_other = false;
        
        for j in 0..variables.len() {
            if i == j {
                continue;
            }
            
            let other_var = variables[j];
            let other_coeff = coefficients[j];
            
            let lb = other_var.min(ctx);
            let ub = other_var.max(ctx);
            
            let min_term = match (lb, ub) {
                (Val::ValF(l), Val::ValF(u)) => {
                    // Check if this variable is unbounded
                    if l.is_infinite() || u.is_infinite() {
                        // Can't compute meaningful bounds if any other variable is unbounded
                        has_unbounded_other = true;
                        break;
                    }
                    if other_coeff > 0.0 {
                        other_coeff * l
                    } else {
                        other_coeff * u
                    }
                }
                (Val::ValI(l), Val::ValI(u)) => {
                    let lf = l as f64;
                    let uf = u as f64;
                    if other_coeff > 0.0 {
                        other_coeff * lf
                    } else {
                        other_coeff * uf
                    }
                }
                _ => return Some(()),
            };
            
            min_other += min_term;
        }
        
        // Skip propagation for this variable if other variables are unbounded
        if has_unbounded_other {
            continue;
        }
        
        let remaining = constant - min_other;
        
        if coeff > 0.0 {
            let max_val = remaining / coeff;
            var_id.try_set_max(Val::ValF(max_val), ctx)?;
        } else {
            let min_val = remaining / coeff;
            var_id.try_set_min(Val::ValF(min_val), ctx)?;
        }
    }
    
    Some(())
}

/// Helper to apply float_lin_ne propagation (extracted for reuse)
fn prune_float_lin_ne(coefficients: &[f64], variables: &[VarId], constant: f64, ctx: &mut Context) -> Option<()> {
    let mut unfixed_idx = None;
    let mut fixed_sum = 0.0;
    
    for i in 0..variables.len() {
        let var_id = variables[i];
        let coeff = coefficients[i];
        
        let lb = var_id.min(ctx);
        let ub = var_id.max(ctx);
        
        match (lb, ub) {
            (Val::ValF(l), Val::ValF(u)) if (l - u).abs() < 1e-12 => {
                fixed_sum += coeff * l;
            }
            (Val::ValI(l), Val::ValI(u)) if l == u => {
                fixed_sum += coeff * (l as f64);
            }
            _ => {
                if unfixed_idx.is_some() {
                    return Some(());
                }
                unfixed_idx = Some(i);
            }
        }
    }
    
    if unfixed_idx.is_none() {
        if (fixed_sum - constant).abs() < 1e-12 {
            return None;
        }
        return Some(());
    }
    
    let idx = unfixed_idx.unwrap();
    let var_id = variables[idx];
    let coeff = coefficients[idx];
    
    if coeff.abs() < 1e-12 {
        if (fixed_sum - constant).abs() < 1e-12 {
            return None;
        }
        return Some(());
    }
    
    let forbidden = (constant - fixed_sum) / coeff;
    exclude_value(var_id, Val::ValF(forbidden), ctx)?;
    
    Some(())
}
