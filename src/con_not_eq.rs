#![allow(dead_code)]

use std::{borrow::BorrowMut, ops::DerefMut};

use crate::{
    constraint::{ConId, Constraint},
    scheduler::PropagateEvent,
    var::{Var, VarRef},
};

// Example constraint implementation
// y != x + v
pub struct ConNotEq {
    pub id: ConId,
    pub active: bool,
    pub x: VarRef,
    pub y: VarRef,
    pub v: i32,
}

// x != y + offset
impl ConNotEq {
    pub fn new(id: ConId, x: VarRef, y: VarRef, offset: i32) -> Self {
        ConNotEq {
            id,
            active: true,
            x: x,
            y: y,
            v: offset,
        }
    }
    pub fn new_boxed(id: ConId, x: VarRef, y: VarRef, offset: i32) -> Box<ConNotEq> {
        Box::new(ConNotEq::new(id, x, y, offset))
    }
}

impl Constraint for ConNotEq {
    fn id(&self) -> ConId {
        self.id
    }
    fn set_active(&mut self, b: bool) {
        self.active = b;
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn post(&mut self) {
        let mut tx = (*self.x).borrow_mut();
        let mut ty = (*self.y).borrow_mut();

        if let Var::VarI(x) = tx.deref_mut() {
            if let Var::VarI(y) = ty.deref_mut() {
                if y.is_fixed() {
                    x.remove(y.min() + self.v);
                } else {
                    if x.is_fixed() {
                        y.remove(x.min() - self.v);
                    } else {
                        x.notify(&PropagateEvent::SubOnFix(x.id(), self.id()));
                        y.notify(&PropagateEvent::SubOnFix(y.id(), self.id()));
                    }
                }
            }
        }
    }

    fn propagate(&mut self) -> bool {
        let mut tx = (*self.x).borrow_mut();
        let mut ty = (*self.y).borrow_mut();
        let active = (self.active).borrow_mut();

        if let Var::VarI(x) = tx.deref_mut() {
            if let Var::VarI(y) = ty.deref_mut() {
                if y.is_fixed() {
                    x.remove(y.min() + self.v);
                } else {
                    y.remove(x.min() - self.v);
                }
            }
        }
        *active = true;
        return true;
    }

    fn on_fix(&self) {
        todo!()
    }

    fn on_domain_change(&self) {
        todo!()
    }

    fn on_bound_change(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test_not_eq {
    use std::ops::Deref;

    use crate::{solver::Solver, solver_options::SolverOptions};

    use super::*;

    fn test_init(a: i32, b: i32, c: i32, d: i32) -> (Solver, ConId) {
        let opt = SolverOptions {};
        let mut sol = Solver::new(opt);
        // create 2 variables
        let v0 = sol.var_int(a, b);
        let v1 = sol.var_int(c, d);
        // create constraint
        let c = sol.not_eq(v0, v1, 2);
        (sol, c)
    }

    #[test]
    fn test_noteq_new() {
        let (mut sol, c) = test_init(0, 1, 0, 1);
        let cc = (*sol.cons[c]).borrow_mut();
        assert_eq!(cc.is_active(), true);
        assert_eq!(cc.id(), 0);
        cc.set_active(false);
        assert_eq!(cc.is_active(), false);
    }

    #[test]
    #[ignore = "still not implemented"]
    fn test_noteq_propagate() {
        let (mut sol, c) = test_init(5, 9, 7, 7); // v1 fixed on 7
        let cc = (*sol.cons[c]).borrow_mut();
        cc.set_active(false);
        cc.propagate();
        assert_eq!(cc.is_active(), true);
        // Check v0 variable after propagation
        let v0 = sol.vars[0].clone();
        let tv0 = (*v0).borrow_mut();
        if let Var::VarI(v0) = tv0.deref() {
            assert_eq!(format!("{}", v0), "0 [10,11,12,13,14,15|]");
        }
        // Check v1 variable after propagation
        let v1 = sol.vars[1].clone();
        let tv1 = (*v1).borrow_mut();
        if let Var::VarI(v1) = tv1.deref() {
            assert_eq!(format!("{}", v1), "1 [12,13,14,15,16,17,18,19,20|]");
        }
    }
}
