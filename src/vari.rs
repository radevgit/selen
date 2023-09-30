#![allow(dead_code)]

use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    observer::MutObserver, scheduler::PropagateEvent, sparse_set_domain::SparseSetDomain,
    state::StateEvent, var::VarId,
};

// https://github.com/minicp/minicp/blob/f5a0db51b3e40ad233cc3ead6cef78f9204c3655/src/main/java/minicp/state/StateSparseSet.java
// Observer pattern: https://github.com/kan1-u/event-observer/blob/master/src/subject.rs

// Integer variable
pub struct VarI {
    pub id: VarId,
    dom: SparseSetDomain,
}

//pub type VIntRef = Rc<RefCell<VInt>>;

impl Display for VarI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.id, self.dom)
    }
}

impl VarI {
    pub fn new(
        id: VarId,
        min: i32,
        max: i32,
        sched: Rc<RefCell<dyn MutObserver<PropagateEvent>>>,
        state: Rc<RefCell<dyn MutObserver<StateEvent>>>,
    ) -> Self {
        let v = Self {
            id,
            dom: SparseSetDomain::new(min, max, id, sched, state),
        };
        v.notify(&PropagateEvent::VarAdd(v.id));
        v
    }

    // Subscribe observer to this var
    // pub fn subscribe(&mut self, var_id: VarId, sched: Rc<RefCell<dyn MutObserver<PropagateEvent>>>, state: Rc<RefCell<dyn MutObserver<StateEvent>>>) {
    //     self.id = var_id;
    //     // register domain for propagator
    //     self.dom.propagator.add_rc_refcell_mut_observer(sched);
    //     // register state sparse set for state manager
    //     self.dom.dom.state.add_rc_refcell_mut_observer(state);
    //     self.notify(&PropagateEvent::VarAdd(self.id))
    // }

    pub fn notify(&self, event: &PropagateEvent) {
        self.dom.propagator.notify(event);
    }

    pub fn id(&self) -> VarId {
        self.id
    }

    pub fn min(&self) -> i32 {
        self.dom.min()
    }

    pub fn max(&self) -> i32 {
        self.dom.max()
    }

    pub fn size(&self) -> usize {
        self.dom.size()
    }

    // If variable domain is fixed to value
    pub fn is_fixed(&self) -> bool {
        self.dom.is_fixed()
    }

    pub fn is_empty(&self) -> bool {
        self.dom.is_empty()
    }

    pub fn contains(&self, v: i32) -> bool {
        self.dom.contains(v)
    }

    pub fn remove(&mut self, val: i32) {
        self.dom.remove(val)
    }

    // Removes all the element from the set except the given value.
    pub fn remove_all_but(&mut self, v: i32) {
        self.dom.remove_all_but(v)
    }

    pub fn remove_below(&mut self, val: i32) {
        self.dom.remove_below(val)
    }

    pub fn remove_above(&mut self, val: i32) {
        self.dom.remove_above(val)
    }
}

#[cfg(test)]
mod vint_tests {

    use crate::{scheduler::Scheduler, state::State};

    use super::*;
    type S = Scheduler;

    fn var_new(id: VarId, min: i32, max: i32) -> VarI {
        let sched = Rc::new(RefCell::new(Scheduler::new()));
        let state = Rc::new(RefCell::new(State::new()));
        return VarI::new(id, min, max, sched, state);
    }

    #[test]
    fn test_remove() {
        let mut v = var_new(0, 1, 5);
        v.remove(3);
        assert_eq!(v.size(), 4);
        assert_eq!(format!("{}", v), "0 [1,2,5,4|3]");
        assert_eq!(v.id(), 0);
    }
    #[test]
    fn test_remove2() {
        let mut v = var_new(0, 1, 5);
        v.remove(5);
        v.remove(4);
        v.remove(3);
        v.remove(2);
        v.remove(1);
        assert_eq!(v.size(), 0);
        assert_eq!(format!("{}", v), "0 [|1,2,3,4,5]");
        assert!(v.is_empty())
    }

    #[test]
    fn test_remove3() {
        let mut v = var_new(0, 1, 5);
        v.remove(2);
        v.remove(2); // remove non existent
        assert_eq!(format!("{}", v), "0 [1,5,3,4|2]");
    }

    #[test]
    fn test_remove_all_but() {
        let mut v = var_new(0, 1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "0 [2|1,3,4,5]");
    }

    #[test]
    fn test_remove_below() {
        let mut v = var_new(0, 1, 5);
        v.remove_below(3);
        assert_eq!(format!("{}", v), "0 [5,4,3|2,1]");
        v.remove_below(6);
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn test_remove_above() {
        let mut v = var_new(0, 3, 7);
        v.remove_above(5);
        assert_eq!(format!("{}", v), "0 [3,4,5|7,6]");
        v.remove_above(2);
        assert_eq!(v.size(), 0);
        assert_eq!(format!("{}", v), "0 [|3,4,5,7,6]");
    }

    #[test]
    fn test_min() {
        let mut v = var_new(0, 1, 5);
        v.remove(1);
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), 2);
        assert_eq!(format!("{}", v), "0 [5,2,3,4|1]");
    }

    #[test]
    fn test_max() {
        let mut v = var_new(0, 1, 5);
        v.remove(5);
        assert_eq!(v.size(), 4);
        assert_eq!(v.max(), 4);
        assert_eq!(format!("{}", v), "0 [1,2,3,4|5]");
    }

    #[test]
    fn test_when_empty() {
        let mut v = var_new(0, 1, 2);
        v.remove(1);
        v.remove(2);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(format!("{}", v), "0 [|2,1]");
    }

    #[test]
    #[should_panic]
    fn test_when_empty2() {
        let mut v = var_new(0, 1, 2);
        v.remove(1);
        v.remove(2);
        v.min();
        v.max();
    }

    #[test]
    fn test_is_fixed() {
        let mut v = var_new(0, 1, 2);
        v.remove(1);
        assert!(v.is_fixed());
    }

    #[test]
    fn test_contains() {
        let v = var_new(0, 1, 2);
        assert!(!v.contains(0));
        assert!(v.contains(2));
        assert!(!v.contains(3));
    }
}
