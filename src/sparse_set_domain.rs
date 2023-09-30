#![allow(dead_code)]

// In addition to SparseSet, adds listeners and checks

use std::{cell::RefCell, fmt::{Display, Debug}, rc::Rc};

use crate::{
    observer::MutObserver,
    scheduler::{PropagateEvent, SubjectPropagate},
    state::StateEvent,
    state_sparse_set::StateSparseSet,
    var::VarId,
};

pub struct SparseSetDomain {
    pub var_id: VarId,
    pub(crate) propagator: SubjectPropagate,
    pub(crate) dom: StateSparseSet,
}

impl Display for SparseSetDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dom)
    }
}

impl Debug for SparseSetDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dom)
    }
}

impl SparseSetDomain {
    pub fn new(
        min: i32,
        max: i32,
        var_id: VarId,
        sched: Rc<RefCell<dyn MutObserver<PropagateEvent>>>,
        state: Rc<RefCell<dyn MutObserver<StateEvent>>>,
    ) -> Self {
        let mut dom = Self {
            var_id: var_id,
            propagator: SubjectPropagate::new(),
            dom: StateSparseSet::new(min, max, state),
        };
        dom.propagator.add_rc_refcell_mut_observer(sched);
        dom
    }

    pub fn min(&self) -> i32 {
        self.dom.min()
    }

    pub fn max(&self) -> i32 {
        self.dom.max()
    }

    pub fn size(&self) -> usize {
        self.dom.size().try_into().unwrap()
    }

    pub fn contains(&self, v: i32) -> bool {
        self.dom.contains(v)
    }

    // If variable domain is fixed to value
    pub fn is_fixed(&self) -> bool {
        self.dom.is_fixed()
    }

    pub fn is_empty(&self) -> bool {
        self.dom.is_empty()
    }

    pub fn remove(&mut self, val: i32) {
        if self.dom.contains(val) {
            let min_changed = if self.min() == val { true } else { false };
            let max_changed = if self.max() == val { true } else { false };
            self.dom.remove(val);
            if self.dom.size() == 0 {
                self.empty();
                return;
            }
            if min_changed {
                self.change_min();
            }
            if max_changed {
                self.change_max()
            }
            if self.dom.size() == 1 {
                self.fix();
            }
        }
    }

    // Removes all the element from the set except the given value.
    pub fn remove_all_but(&mut self, val: i32) {
        if self.dom.contains(val) {
            if self.dom.size() != 1 {
                let min_changed = if self.min() == val { true } else { false };
                let max_changed = if self.max() == val { true } else { false };
                self.dom.remove_all_but(val);
                // This should not happen since contains is checked
                // if self.dom.size() == 0 {
                //     self.empty();
                //     return;
                // }
                self.fix();
                self.change();
                if min_changed {
                    self.change_min();
                }
                if max_changed {
                    self.change_max();
                }
            }
        } else {
            self.dom.remove_all();
            self.empty();
        }
    }

    pub fn remove_below(&mut self, val: i32) {
        if self.dom.min() < val {
            self.dom.remove_below(val);
            match self.dom.size() {
                0 => {
                    self.empty();
                    return;
                }
                1 => {
                    self.fix();
                }
                _ => {
                    self.change_min();
                    self.change();
                }
            }
        }
    }

    pub fn remove_above(&mut self, val: i32) {
        if self.dom.max() > val {
            self.dom.remove_above(val);
            match self.dom.size() {
                0 => {
                    self.empty();
                    return;
                }
                1 => {
                    self.fix();
                }
                _ => {
                    self.change_max();
                    self.change();
                }
            }
        }
    }

    pub fn empty(&self) {
        self.propagator.notify(&PropagateEvent::Inconsistency);
    }

    pub fn fix(&self) {
        self.propagator.notify(&PropagateEvent::OnFix(self.var_id));
    }

    pub fn change(&self) {
        self.propagator
            .notify(&PropagateEvent::OnDomainChange(self.var_id));
    }

    pub fn change_min(&self) {
        self.propagator
            .notify(&PropagateEvent::OnBoundChange(self.var_id));
    }
    pub fn change_max(&self) {
        self.propagator
            .notify(&PropagateEvent::OnBoundChange(self.var_id));
    }
}


#[cfg(test)]
mod test_sparse_set_domain {
    use crate::{state::State, scheduler::Scheduler};

    use super::*;

    fn ssd_new(id: VarId, min: i32, max: i32) -> SparseSetDomain {
        let sched = Rc::new(RefCell::new(Scheduler::new()));
        let state = Rc::new(RefCell::new(State::new()));
        return SparseSetDomain::new(min, max, id, sched, state)
    }

    #[test]
    fn test_display() {
        let dom = ssd_new(0, 5, 7);
        assert_eq!(format!("{}", dom), "[5,6,7|]");
    }

    #[test]
    fn test_debug() {
        let dom = ssd_new(0, 5, 7);
        assert_eq!(format!("{:?}", dom), "[5,6,7|]");
    }

    #[test]
    fn test_remove_all_but() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_all_but(5);
        assert_eq!(format!("{:?}", dom), "[5|6]");
    }

    #[test]
    fn test_remove_all_but2() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_all_but(6);
        assert_eq!(format!("{:?}", dom), "[6|5]");
    }

    #[test]
    fn test_remove_all_but3() {
        let mut dom = ssd_new(0, 5, 5);
        dom.remove_all_but(5);
        assert_eq!(format!("{:?}", dom), "[5|]");
    }

    #[test]
    fn test_remove_all_but4() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_all_but(4);
        assert_eq!(format!("{:?}", dom), "[|5,6]");
    }

    #[test]
    fn test_remove_below() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_below(6);
        assert_eq!(format!("{:?}", dom), "[6|5]");
    }

    #[test]
    fn test_remove_below2() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_below(4);
        assert_eq!(format!("{:?}", dom), "[5,6|]");
    }

    #[test]
    fn test_remove_above() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_above(5);
        assert_eq!(format!("{:?}", dom), "[5|6]");
    }

    #[test]
    fn test_remove_above2() {
        let mut dom = ssd_new(0, 5, 6);
        dom.remove_above(7);
        assert_eq!(format!("{:?}", dom), "[5,6|]");
    }

}