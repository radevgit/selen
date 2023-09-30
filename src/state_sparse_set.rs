#![allow(dead_code)]

use std::{
    cell::RefCell,
    fmt::{self, Display, Debug},
    rc::Rc,
};

use crate::{
    observer::MutObserver,
    state::{StateEvent, SubjectState},
    state_int::StateInt,
};

// https://github.com/minicp/minicp/blob/mooc/src/main/java/minicp/state/StateSparseSet.java
pub struct StateSparseSet {
    pub state: SubjectState,
    off: i32,       // the domain offset (fixed)
    n: u32,         // total number of values in domain
    min: StateInt,  // current minimum value in the set
    max: StateInt,  // current maximum value in the set
    size: StateInt, // domain size
    ind: Vec<u32>,  // sparse set indices
    val: Vec<u32>,  // sparse set values
}

impl Display for StateSparseSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        use std::fmt::Write;
        write!(s, "[").unwrap();
        if self.size() == 0 {
            write!(s, "|").unwrap();
        }
        for i in 0..self.n {
            write!(s, "{},", (self.val[i as usize] as i32 + self.off)).unwrap();
            if i + 1 == self.size() {
                s.pop(); // remove comma
                write!(s, "|").unwrap();
            } else {
            }
        }
        if self.size() != self.n {
            s.pop(); // remove comma
        }
        write!(s, "]").unwrap();
        write!(f, "{}", s)
    }
}

impl Debug for StateSparseSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StateSparseSet {{ off: {}, min: {}, max: {}, size: {}, set: {} }}", self.off, self.min.value(), self.max.value(), self.size.value(), self)
    }
}

impl StateSparseSet {
    pub fn new(min: i32, max: i32, state: Rc<RefCell<dyn MutObserver<StateEvent>>>) -> Self {
        let maxmin = (max - min) as u32;
        let n = maxmin + 1;
        StateSparseSet {
            state: SubjectState::new(),
            off: min,
            n: n,
            min: StateInt::new(0, state.clone()),
            max: StateInt::new(maxmin, state.clone()),
            size: StateInt::new(n, state),
            ind: Vec::from_iter(0..n),
            val: Vec::from_iter(0..n),
        }
    }

    pub fn exchange(&mut self, val1: u32, val2: u32) {
        let v1 = val1;
        let v2 = val2;
        let i1 = self.ind[v1 as usize];
        let i2 = self.ind[v2 as usize];
        self.val[i1 as usize] = v2;
        self.val[i2 as usize] = v1;
        self.ind[v1 as usize] = i2;
        self.ind[v2 as usize] = i1;
    }

    pub fn is_empty(&self) -> bool {
        self.size.value() == 0
    }

    pub fn size(&self) -> u32 {
        self.size.value()
    }

    // TODO: throws in original
    pub fn min(&self) -> i32 {
        debug_assert!(!self.is_empty());
        let x = self.min.value();
        let xx = x as i32 + self.off;
        println!("{}", xx);
        self.min.value() as i32 + self.off
    }

    pub fn max(&self) -> i32 {
        debug_assert!(!self.is_empty());
        self.max.value() as i32 + self.off
    }

    // If variable domain is fixed to value
    pub fn is_fixed(&self) -> bool {
        self.size.value() == 1
    }

    fn update_bounds_val_removed(&mut self, val: u32) {
        self.update_max_val_removed(val);
        self.update_min_val_removed(val);
    }
    // update after max value is removed
    fn update_max_val_removed(&mut self, val: u32) {
        if !self.is_empty() && self.max.value() == val {
            // The maximum was removed, search the new one
            for v in (self.min.value()..val).rev() {
                if self.contains_intl(v) {
                    self.max.set_value(v);
                    return;
                }
            }
        }
    }
    // update after min value is removed
    fn update_min_val_removed(&mut self, val: u32) {
        if !self.is_empty() && self.min.value() == val {
            // The minimum was removed, search the new one
            let vv = val + 1;
            let vvv = self.max.value() + 1;
            for v in vv..vvv {
                if self.contains_intl(v) {
                    self.min.set_value(v);
                    return;
                }
            }
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.contains(val) {
            // The value has already been removed
            return false;
        }
        let val = (val - self.off) as u32;
        self.exchange(val, self.val[(self.size() - 1) as usize]);
        self.size.set_value(self.size.value() - 1);
        self.update_bounds_val_removed(val);
        return true;
    }

    pub fn contains(&self, val: i32) -> bool {
        let val = val - self.off;
        if val < 0 || val as u32 >= self.n {
            return false;
        } else {
            self.ind[val as usize] < self.size()
        }
    }

    // This method operates on the shifted value (one cannot shift now).
    fn contains_intl(&self, val: u32) -> bool {
        if val >= self.n {
            false
        } else {
            self.ind[val as usize] < self.size()
        }
    }

    pub fn remove_all(&mut self) {
        self.size.set_value(0);
    }

    // Removes all the element from the set except the given value.
    pub fn remove_all_but(&mut self, v: i32) {
        // if out of domain range
        if !self.contains(v) {
            self.remove_all();
            return;
        }
        let v = (v - self.off) as u32;
        let val = self.val[0];
        let index = self.ind[v as usize];
        self.ind[v as usize] = 0;
        self.val[0] = v;
        self.ind[val as usize] = index;
        self.val[index as usize] = val;
        self.min.set_value(v);
        self.max.set_value(v);
        self.size.set_value(1);
    }

    pub fn remove_below(&mut self, val: i32) {
        if self.is_empty() {
            return;
        }
        if self.max() < val {
            self.remove_all();
        } else {
            for v in self.min()..val {
                self.remove(v);
            }
        }
    }

    pub fn remove_above(&mut self, val: i32) {
        if self.is_empty() {
            return;
        }
        if self.min() > val {
            self.remove_all();
        } else {
            let x = (val + 1)..(self.max() + 1);
            for v in x {
                self.remove(v);
            }
        }
    }
}

#[cfg(test)]
mod test_state_sparse_set {

    use crate::{
        scheduler::Scheduler,
        state::State,
    };

    use super::*;
    type S = Scheduler;

    fn sps_new(min: i32, max: i32) -> StateSparseSet {
        let state = Rc::new(RefCell::new(State::new()));
        return StateSparseSet::new(min, max, state);
    }

    #[test]
    fn test_debug() {
        let mut v = sps_new(1, 2);
        assert_eq!(
            format!("{:?}", v),
            "StateSparseSet { off: 1, min: 0, max: 1, size: 2, set: [1,2|] }"
        );
        v.remove_all();
        assert_eq!(
            format!("{:?}", v),
            "StateSparseSet { off: 1, min: 0, max: 1, size: 0, set: [|1,2] }"
        );
    }

    #[test]
    fn test_display() {
        let mut v = sps_new(1, 2);
        v.remove_all();
        assert_eq!(format!("{}", v), "[|1,2]");
    }
    #[test]
    fn test_display2() {
        let mut v = sps_new(1, 2);
        v.remove(2);
        assert_eq!(format!("{}", v), "[1|2]");
    }

    #[test]
    fn test_display3() {
        let v = sps_new(1, 2);
        assert_eq!(format!("{}", v), "[1,2|]");
    }

    #[test]
    fn test_new() {
        let v = sps_new(1, 2);
        assert_eq!(v.off, 1);
        assert_eq!(v.min(), 1);
        assert_eq!(v.max(), 2);
        assert_eq!(format!("{}", v), "[1,2|]");
    }

    #[test]
    fn test_remove() {
        let mut v = sps_new(1, 5);
        v.remove(3);
        assert_eq!(v.size(), 4);
        assert_eq!(format!("{}", v), "[1,2,5,4|3]");
        v.remove(5);
        assert_eq!(format!("{}", v), "[1,2,4|5,3]");
        v.remove(1);
        assert_eq!(format!("{}", v), "[4,2|1,5,3]");
    }
    #[test]
    fn test_remove2() {
        let mut v = sps_new(1, 5);
        v.remove(5);
        v.remove(4);
        v.remove(3);
        v.remove(2);
        v.remove(1);
        assert_eq!(v.size(), 0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
        assert!(v.is_empty())
    }

    #[test]
    fn test_remove3() {
        let mut v = sps_new(1, 5);
        v.remove(2);
        v.remove(2); // remove non existent
        assert_eq!(format!("{}", v), "[1,5,3,4|2]");
    }

    #[test]
    fn test_remove_all_but0() {
        let mut v = sps_new(1, 5);
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_remove_all_but1() {
        let mut v = sps_new(1, 5);
        v.remove_all_but(7);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_remove_all_but2() {
        let mut v = sps_new(1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "[2|1,3,4,5]");
        v.remove_all_but(3);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
    }

    #[test]
    fn test_remove_all_but3() {
        let mut v = sps_new(1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "[2|1,3,4,5]");
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
    }

    #[test]
    fn test_remove_below() {
        let mut v = sps_new(1, 5);
        v.remove_below(3);
        assert_eq!(format!("{}", v), "[5,4,3|2,1]");
        v.remove_below(6);
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn test_remove_below2() {
        let mut v = sps_new(1, 5);
        v.remove_all();
        v.remove_below(0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_remove_above() {
        let mut v = sps_new(3, 7);
        v.remove_above(5);
        assert_eq!(format!("{}", v), "[3,4,5|7,6]");
        v.remove_above(2);
        assert_eq!(v.size(), 0);
        assert_eq!(format!("{}", v), "[|3,4,5,7,6]");
    }

    #[test]
    fn test_remove_above2() {
        let mut v = sps_new(1, 5);
        v.remove_all();
        v.remove_above(6);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_min() {
        let mut v = sps_new(1, 5);
        v.remove(1);
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), 2);
        assert_eq!(format!("{}", v), "[5,2,3,4|1]");
    }

    #[test]
    fn test_max() {
        let mut v = sps_new(1, 5);
        v.remove(5);
        assert_eq!(v.size(), 4);
        assert_eq!(v.max(), 4);
        assert_eq!(format!("{}", v), "[1,2,3,4|5]");
    }

    #[test]
    fn test_when_empty() {
        let mut v = sps_new(1, 2);
        v.remove(1);
        v.remove(2);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(format!("{}", v), "[|2,1]");
    }

    #[test]
    fn test_is_fixed() {
        let mut v = sps_new(1, 2);
        v.remove(1);
        assert!(v.is_fixed());
    }

    #[test]
    fn test_is_contains() {
        let v = sps_new(1, 2);
        assert!(!v.contains(3));
        assert!(v.contains(2));
        assert!(!v.contains_intl(2));
        assert!(v.contains_intl(1));
    }
}
