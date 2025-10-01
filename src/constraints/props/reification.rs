//! Reification constraints
//!
//! Reification allows constraints to be represented as boolean variables.
//! A reified constraint `b ⇔ C` means:
//! - If b = 1, then constraint C must hold
//! - If b = 0, then constraint C must NOT hold
//! - If C holds, then b must be 1
//! - If C does not hold, then b must be 0

use crate::{
    constraints::props::{Propagate, Prune},
    variables::{VarId, Val},
    variables::views::{Context, View},
};

/// Reified equality constraint: `b ⇔ (x = y)`
/// 
/// Enforces bidirectional implication:
/// - b = 1 implies x = y
/// - b = 0 implies x ≠ y
/// - x = y implies b = 1
/// - x ≠ y implies b = 0
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct IntEqReif {
    x: VarId,
    y: VarId,
    b: VarId, // boolean result: 0 or 1
}

impl IntEqReif {
    pub fn new(x: VarId, y: VarId, b: VarId) -> Self {
        Self { x, y, b }
    }
}

impl Prune for IntEqReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let b_min = self.b.min(ctx);
        let b_max = self.b.max(ctx);

        // CRITICAL: Check all inference directions to ensure proper propagation
        // This ensures we make progress regardless of constraint posting order
        
        // Direction 1: From x, y domains → infer b
        // If domains don't overlap at all, x ≠ y is certain
        if x_max < y_min || y_max < x_min {
            // x and y cannot be equal, so b must be 0
            self.b.try_set_max(Val::ValI(0), ctx)?;
        }
        // If both x and y are fixed to the same value
        else if x_min == x_max && y_min == y_max && x_min == y_min {
            // x = y is certain, so b must be 1
            self.b.try_set_min(Val::ValI(1), ctx)?;
        }

        // Direction 2: From b → enforce constraint on x, y
        // If b is fixed to 1 (true): enforce x = y
        if b_min >= Val::ValI(1) {
            // Enforce x = y by intersecting domains
            let new_min = if x_min > y_min { x_min } else { y_min };
            let new_max = if x_max < y_max { x_max } else { y_max };
            
            if new_min > new_max {
                // No intersection possible - constraint is violated
                return None;
            }
            
            self.x.try_set_min(new_min, ctx)?;
            self.x.try_set_max(new_max, ctx)?;
            self.y.try_set_min(new_min, ctx)?;
            self.y.try_set_max(new_max, ctx)?;
        }

        // If b is fixed to 0 (false): enforce x ≠ y
        if b_max <= Val::ValI(0) {
            // If one variable is fixed, remove that value from the other
            if x_min == x_max {
                // x is fixed, y cannot equal x
                if y_min == x_min && y_min < y_max {
                    // Remove x_min from y's domain by increasing min
                    self.y.try_set_min(y_min + Val::ValI(1), ctx)?;
                } else if y_max == x_min && y_min < y_max {
                    // Remove x_min from y's domain by decreasing max
                    self.y.try_set_max(y_max - Val::ValI(1), ctx)?;
                }
            } else if y_min == y_max {
                // y is fixed, x cannot equal y
                if x_min == y_min && x_min < x_max {
                    // Remove y_min from x's domain by increasing min
                    self.x.try_set_min(x_min + Val::ValI(1), ctx)?;
                } else if x_max == y_min && x_min < x_max {
                    // Remove y_min from x's domain by decreasing max
                    self.x.try_set_max(x_max - Val::ValI(1), ctx)?;
                }
            }
        }

        Some(())
    }
}

impl Propagate for IntEqReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.x, self.y, self.b].into_iter()
    }
}

/// Reified inequality constraint: `b ⇔ (x ≠ y)`
/// 
/// Enforces bidirectional implication:
/// - b = 1 implies x ≠ y
/// - b = 0 implies x = y
/// - x ≠ y implies b = 1
/// - x = y implies b = 0
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct IntNeReif {
    x: VarId,
    y: VarId,
    b: VarId, // boolean result: 0 or 1
}

impl IntNeReif {
    pub fn new(x: VarId, y: VarId, b: VarId) -> Self {
        Self { x, y, b }
    }
}

impl Prune for IntNeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let b_min = self.b.min(ctx);
        let b_max = self.b.max(ctx);

        // CRITICAL: Check all inference directions to ensure proper propagation
        // This ensures we make progress regardless of constraint posting order
        
        // Direction 1: From x, y domains → infer b
        // If domains don't overlap at all, x ≠ y is certain
        if x_max < y_min || y_max < x_min {
            // x and y cannot be equal, so b must be 1 (they ARE not equal)
            self.b.try_set_min(Val::ValI(1), ctx)?;
        }
        // If both x and y are fixed to the same value
        else if x_min == x_max && y_min == y_max && x_min == y_min {
            // x = y is certain, so b must be 0 (they are NOT not-equal)
            self.b.try_set_max(Val::ValI(0), ctx)?;
        }

        // Direction 2: From b → enforce constraint on x, y
        // If b is fixed to 1 (true): enforce x ≠ y
        if b_min >= Val::ValI(1) {
            // If one variable is fixed, remove that value from the other
            if x_min == x_max {
                // x is fixed, y cannot equal x
                if y_min == x_min && y_min < y_max {
                    // Remove x_min from y's domain by increasing min
                    self.y.try_set_min(y_min + Val::ValI(1), ctx)?;
                } else if y_max == x_min && y_min < y_max {
                    // Remove x_min from y's domain by decreasing max
                    self.y.try_set_max(y_max - Val::ValI(1), ctx)?;
                }
            } else if y_min == y_max {
                // y is fixed, x cannot equal y
                if x_min == y_min && x_min < x_max {
                    // Remove y_min from x's domain by increasing min
                    self.x.try_set_min(x_min + Val::ValI(1), ctx)?;
                } else if x_max == y_min && x_min < x_max {
                    // Remove y_min from x's domain by decreasing max
                    self.x.try_set_max(x_max - Val::ValI(1), ctx)?;
                }
            }
        }

        // If b is fixed to 0 (false): enforce x = y
        if b_max <= Val::ValI(0) {
            // Enforce x = y by intersecting domains
            let new_min = if x_min > y_min { x_min } else { y_min };
            let new_max = if x_max < y_max { x_max } else { y_max };
            
            if new_min > new_max {
                // No intersection possible - constraint is violated
                return None;
            }
            
            self.x.try_set_min(new_min, ctx)?;
            self.x.try_set_max(new_max, ctx)?;
            self.y.try_set_min(new_min, ctx)?;
            self.y.try_set_max(new_max, ctx)?;
        }

        Some(())
    }
}

impl Propagate for IntNeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.x, self.y, self.b].into_iter()
    }
}

/// Reified less-than constraint: `b ⇔ (x < y)`
/// 
/// Enforces bidirectional implication:
/// - b = 1 implies x < y
/// - b = 0 implies x ≥ y
/// - x < y implies b = 1
/// - x ≥ y implies b = 0
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct IntLtReif {
    x: VarId,
    y: VarId,
    b: VarId,
}

impl IntLtReif {
    pub fn new(x: VarId, y: VarId, b: VarId) -> Self {
        Self { x, y, b }
    }
}

impl Prune for IntLtReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let b_min = self.b.min(ctx);
        let b_max = self.b.max(ctx);

        // Direction 1: From x, y domains → infer b
        // If x_max < y_min, then x < y is certain
        if x_max < y_min {
            self.b.try_set_min(Val::ValI(1), ctx)?;
        }
        // If x_min >= y_max, then x ≥ y is certain (x < y is false)
        else if x_min >= y_max {
            self.b.try_set_max(Val::ValI(0), ctx)?;
        }

        // Direction 2: From b → enforce constraint on x, y
        // If b = 1: enforce x < y, i.e., x ≤ y-1
        if b_min >= Val::ValI(1) {
            // x < y means x_max must be < y_min (eventually)
            // More precisely: x ≤ y - 1
            self.x.try_set_max(y_max - Val::ValI(1), ctx)?;
            self.y.try_set_min(x_min + Val::ValI(1), ctx)?;
        }

        // If b = 0: enforce x ≥ y
        if b_max <= Val::ValI(0) {
            // x >= y
            self.x.try_set_min(y_min, ctx)?;
            self.y.try_set_max(x_max, ctx)?;
        }

        Some(())
    }
}

impl Propagate for IntLtReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.x, self.y, self.b].into_iter()
    }
}

/// Reified less-than-or-equal constraint: `b ⇔ (x ≤ y)`
/// 
/// Enforces bidirectional implication:
/// - b = 1 implies x ≤ y
/// - b = 0 implies x > y
/// - x ≤ y implies b = 1
/// - x > y implies b = 0
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct IntLeReif {
    x: VarId,
    y: VarId,
    b: VarId,
}

impl IntLeReif {
    pub fn new(x: VarId, y: VarId, b: VarId) -> Self {
        Self { x, y, b }
    }
}

impl Prune for IntLeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let b_min = self.b.min(ctx);
        let b_max = self.b.max(ctx);

        // Direction 1: From x, y domains → infer b
        // If x_max <= y_min, then x ≤ y is certain
        if x_max <= y_min {
            self.b.try_set_min(Val::ValI(1), ctx)?;
        }
        // If x_min > y_max, then x > y is certain (x ≤ y is false)
        else if x_min > y_max {
            self.b.try_set_max(Val::ValI(0), ctx)?;
        }

        // Direction 2: From b → enforce constraint on x, y
        // If b = 1: enforce x ≤ y
        if b_min >= Val::ValI(1) {
            self.x.try_set_max(y_max, ctx)?;
            self.y.try_set_min(x_min, ctx)?;
        }

        // If b = 0: enforce x > y, i.e., x ≥ y+1
        if b_max <= Val::ValI(0) {
            self.x.try_set_min(y_min + Val::ValI(1), ctx)?;
            self.y.try_set_max(x_max - Val::ValI(1), ctx)?;
        }

        Some(())
    }
}

impl Propagate for IntLeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.x, self.y, self.b].into_iter()
    }
}

/// Reified greater-than constraint: `b ⇔ (x > y)`
/// 
/// Enforces bidirectional implication:
/// - b = 1 implies x > y
/// - b = 0 implies x ≤ y
/// - x > y implies b = 1
/// - x ≤ y implies b = 0
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct IntGtReif {
    x: VarId,
    y: VarId,
    b: VarId,
}

impl IntGtReif {
    pub fn new(x: VarId, y: VarId, b: VarId) -> Self {
        Self { x, y, b }
    }
}

impl Prune for IntGtReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let b_min = self.b.min(ctx);
        let b_max = self.b.max(ctx);

        // Direction 1: From x, y domains → infer b
        // If x_min > y_max, then x > y is certain
        if x_min > y_max {
            self.b.try_set_min(Val::ValI(1), ctx)?;
        }
        // If x_max <= y_min, then x ≤ y is certain (x > y is false)
        else if x_max <= y_min {
            self.b.try_set_max(Val::ValI(0), ctx)?;
        }

        // Direction 2: From b → enforce constraint on x, y
        // If b = 1: enforce x > y, i.e., x ≥ y+1
        if b_min >= Val::ValI(1) {
            self.x.try_set_min(y_min + Val::ValI(1), ctx)?;
            self.y.try_set_max(x_max - Val::ValI(1), ctx)?;
        }

        // If b = 0: enforce x ≤ y
        if b_max <= Val::ValI(0) {
            self.x.try_set_max(y_max, ctx)?;
            self.y.try_set_min(x_min, ctx)?;
        }

        Some(())
    }
}

impl Propagate for IntGtReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.x, self.y, self.b].into_iter()
    }
}

/// Reified greater-than-or-equal constraint: `b ⇔ (x ≥ y)`
/// 
/// Enforces bidirectional implication:
/// - b = 1 implies x ≥ y
/// - b = 0 implies x < y
/// - x ≥ y implies b = 1
/// - x < y implies b = 0
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct IntGeReif {
    x: VarId,
    y: VarId,
    b: VarId,
}

impl IntGeReif {
    pub fn new(x: VarId, y: VarId, b: VarId) -> Self {
        Self { x, y, b }
    }
}

impl Prune for IntGeReif {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        let x_min = self.x.min(ctx);
        let x_max = self.x.max(ctx);
        let y_min = self.y.min(ctx);
        let y_max = self.y.max(ctx);
        let b_min = self.b.min(ctx);
        let b_max = self.b.max(ctx);

        // Direction 1: From x, y domains → infer b
        // If x_min >= y_max, then x ≥ y is certain
        if x_min >= y_max {
            self.b.try_set_min(Val::ValI(1), ctx)?;
        }
        // If x_max < y_min, then x < y is certain (x ≥ y is false)
        else if x_max < y_min {
            self.b.try_set_max(Val::ValI(0), ctx)?;
        }

        // Direction 2: From b → enforce constraint on x, y
        // If b = 1: enforce x ≥ y
        if b_min >= Val::ValI(1) {
            self.x.try_set_min(y_min, ctx)?;
            self.y.try_set_max(x_max, ctx)?;
        }

        // If b = 0: enforce x < y, i.e., x ≤ y-1
        if b_max <= Val::ValI(0) {
            self.x.try_set_max(y_max - Val::ValI(1), ctx)?;
            self.y.try_set_min(x_min + Val::ValI(1), ctx)?;
        }

        Some(())
    }
}

impl Propagate for IntGeReif {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        [self.x, self.y, self.b].into_iter()
    }
}
