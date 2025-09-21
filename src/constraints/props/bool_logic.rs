use crate::{constraints::props::{Propagate, Prune}, variables::{VarId, Val}, variables::views::{Context, View}};

/// Boolean AND constraint: `result = a AND b AND c AND ...`.
/// This constraint enforces that the result variable equals the logical AND of all input variables.
/// All variables are treated as boolean: 0 = false, non-zero = true.
#[derive(Clone, Debug)]
pub struct BoolAnd {
    operands: Vec<VarId>,
    result: VarId,
}

impl BoolAnd {
    pub fn new(operands: Vec<VarId>, result: VarId) -> Self {
        Self { operands, result }
    }
}

impl Prune for BoolAnd {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        if self.operands.is_empty() {
            // Empty AND is typically true
            let _r = self.result.try_set_min(Val::ValI(1), ctx)?;
            let _r = self.result.try_set_max(Val::ValI(1), ctx)?;
            return Some(());
        }

        // For boolean AND:
        // 1. If result = 1, then all operands must be 1 (non-zero)
        // 2. If any operand = 0, then result must be 0
        // 3. If result = 0, then at least one operand must be 0

        let result_min = self.result.min(ctx);
        let result_max = self.result.max(ctx);

        // Check if result is fixed to true (1)
        if result_min >= Val::ValI(1) {
            // All operands must be true (>= 1)
            for &operand in &self.operands {
                let _min = operand.try_set_min(Val::ValI(1), ctx)?;
            }
        }

        // Check if result is fixed to false (0)
        if result_max <= Val::ValI(0) {
            // At least one operand must be false (0)
            // If all but one operands are already true, force the remaining one to be false
            let mut false_operands = 0;
            let mut undetermined_operands = Vec::new();

            for &operand in &self.operands {
                let op_min = operand.min(ctx);
                let op_max = operand.max(ctx);

                if op_max <= Val::ValI(0) {
                    false_operands += 1;
                } else if op_min <= Val::ValI(0) && op_max >= Val::ValI(1) {
                    undetermined_operands.push(operand);
                }
                // If op_min >= 1, the operand is definitely true
            }

            // If no operands are false and only one is undetermined, it must be false
            if false_operands == 0 && undetermined_operands.len() == 1 {
                let _max = undetermined_operands[0].try_set_max(Val::ValI(0), ctx)?;
            }
        }

        // Now propagate from operands to result
        let mut all_true = true;
        let mut any_false = false;

        for &operand in &self.operands {
            let op_min = operand.min(ctx);
            let op_max = operand.max(ctx);

            if op_max <= Val::ValI(0) {
                // This operand is definitely false
                any_false = true;
                break;
            } else if op_min <= Val::ValI(0) {
                // This operand could be false
                all_true = false;
            }
            // If op_min >= 1, this operand is definitely true
        }

        if any_false {
            // At least one operand is false, so result must be false
            let _max = self.result.try_set_max(Val::ValI(0), ctx)?;
        } else if all_true {
            // All operands are definitely true, so result must be true
            let _min = self.result.try_set_min(Val::ValI(1), ctx)?;
        }

        Some(())
    }
}

impl Propagate for BoolAnd {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.result)
            .chain(self.operands.iter().copied())
    }
}

/// Boolean OR constraint: `result = a OR b OR c OR ...`.
/// This constraint enforces that the result variable equals the logical OR of all input variables.
/// All variables are treated as boolean: 0 = false, non-zero = true.
#[derive(Clone, Debug)]
pub struct BoolOr {
    operands: Vec<VarId>,
    result: VarId,
}

impl BoolOr {
    pub fn new(operands: Vec<VarId>, result: VarId) -> Self {
        Self { operands, result }
    }
}

impl Prune for BoolOr {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        if self.operands.is_empty() {
            // Empty OR is typically false
            let _r = self.result.try_set_min(Val::ValI(0), ctx)?;
            let _r = self.result.try_set_max(Val::ValI(0), ctx)?;
            return Some(());
        }

        // For boolean OR:
        // 1. If result = 0, then all operands must be 0
        // 2. If any operand = 1, then result must be 1
        // 3. If result = 1, then at least one operand must be 1

        let result_min = self.result.min(ctx);
        let result_max = self.result.max(ctx);

        // Check if result is fixed to false (0)
        if result_max <= Val::ValI(0) {
            // All operands must be false (0)
            for &operand in &self.operands {
                let _max = operand.try_set_max(Val::ValI(0), ctx)?;
            }
        }

        // Check if result is fixed to true (1)
        if result_min >= Val::ValI(1) {
            // At least one operand must be true (>= 1)
            // If all but one operands are already false, force the remaining one to be true
            let mut true_operands = 0;
            let mut undetermined_operands = Vec::new();

            for &operand in &self.operands {
                let op_min = operand.min(ctx);
                let op_max = operand.max(ctx);

                if op_min >= Val::ValI(1) {
                    true_operands += 1;
                } else if op_min <= Val::ValI(0) && op_max >= Val::ValI(1) {
                    undetermined_operands.push(operand);
                }
                // If op_max <= 0, the operand is definitely false
            }

            // If no operands are true and only one is undetermined, it must be true
            if true_operands == 0 && undetermined_operands.len() == 1 {
                let _min = undetermined_operands[0].try_set_min(Val::ValI(1), ctx)?;
            }
        }

        // Now propagate from operands to result
        let mut all_false = true;
        let mut any_true = false;

        for &operand in &self.operands {
            let op_min = operand.min(ctx);
            let op_max = operand.max(ctx);

            if op_min >= Val::ValI(1) {
                // This operand is definitely true
                any_true = true;
                break;
            } else if op_max >= Val::ValI(1) {
                // This operand could be true
                all_false = false;
            }
            // If op_max <= 0, this operand is definitely false
        }

        if any_true {
            // At least one operand is true, so result must be true
            let _min = self.result.try_set_min(Val::ValI(1), ctx)?;
        } else if all_false {
            // All operands are definitely false, so result must be false
            let _max = self.result.try_set_max(Val::ValI(0), ctx)?;
        }

        Some(())
    }
}

impl Propagate for BoolOr {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.result)
            .chain(self.operands.iter().copied())
    }
}

/// Boolean NOT constraint: `result = NOT operand`.
/// This constraint enforces that the result variable equals the logical NOT of the operand.
/// Variables are treated as boolean: 0 = false, non-zero = true.
#[derive(Clone, Debug)]
pub struct BoolNot {
    operand: VarId,
    result: VarId,
}

impl BoolNot {
    pub fn new(operand: VarId, result: VarId) -> Self {
        Self { operand, result }
    }
}

impl Prune for BoolNot {
    fn prune(&self, ctx: &mut Context) -> Option<()> {
        // For boolean NOT:
        // If operand = 0, then result = 1
        // If operand != 0, then result = 0
        // Conversely:
        // If result = 0, then operand != 0 (operand >= 1)
        // If result = 1, then operand = 0

        let operand_min = self.operand.min(ctx);
        let operand_max = self.operand.max(ctx);
        let result_min = self.result.min(ctx);
        let result_max = self.result.max(ctx);

        // Propagate from operand to result
        if operand_max <= Val::ValI(0) {
            // Operand is definitely false (0), so result must be true (1)
            let _min = self.result.try_set_min(Val::ValI(1), ctx)?;
            let _max = self.result.try_set_max(Val::ValI(1), ctx)?;
        } else if operand_min >= Val::ValI(1) {
            // Operand is definitely true (>= 1), so result must be false (0)
            let _min = self.result.try_set_min(Val::ValI(0), ctx)?;
            let _max = self.result.try_set_max(Val::ValI(0), ctx)?;
        }

        // Propagate from result to operand
        if result_max <= Val::ValI(0) {
            // Result is definitely false (0), so operand must be true (>= 1)
            let _min = self.operand.try_set_min(Val::ValI(1), ctx)?;
        } else if result_min >= Val::ValI(1) {
            // Result is definitely true (1), so operand must be false (0)
            let _min = self.operand.try_set_min(Val::ValI(0), ctx)?;
            let _max = self.operand.try_set_max(Val::ValI(0), ctx)?;
        }

        Some(())
    }
}

impl Propagate for BoolNot {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        core::iter::once(self.result)
            .chain(core::iter::once(self.operand))
    }
}
