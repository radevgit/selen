#![allow(dead_code)]

use std::fmt::Display;

// https://github.com/minicp/minicp/blob/mooc/src/main/java/minicp/state/StateSparseSet.java
#[derive(Debug)]
pub struct SparseSet {
    off: i32,      // the domain offset (fixed)
    n: u32,        // total number of values in domain
    min: u32,      // current minimum value in the set
    max: u32,      // current maximum value in the set
    size: u32,     // domain size
    ind: Vec<u32>, // sparse set indices
    val: Vec<u32>, // sparse set values
}

impl Display for SparseSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        use std::fmt::Write;
        write!(s, "[").unwrap();
        if self.size == 0 {
            write!(s, "|").unwrap();
        }
        for i in 0..self.n {
            write!(s, "{},", (self.val[i as usize] as i32 + self.off)).unwrap();
            if i + 1 == self.size {
                s.pop(); // remove comma
                write!(s, "|").unwrap();
            } else {
            }
        }
        if self.size != self.n {
            s.pop(); // remove comma
        }
        write!(s, "]").unwrap();
        write!(f, "{}", s)
    }
}

impl SparseSet {
    pub fn new(min: i32, max: i32) -> Self {
        let maxmin = (max - min) as u32;
        let n = maxmin + 1;
        SparseSet {
            off: min,
            min: 0,
            max: maxmin,
            n,
            size: n,
            ind: Vec::from_iter(0..n),
            val: Vec::from_iter(0..n),
        }
    }

    pub fn min(&self) -> i32 {
        debug_assert!(!self.is_empty());
        self.min as i32 + self.off
    }
    pub fn max(&self) -> i32 {
        debug_assert!(!self.is_empty());
        self.max as i32 + self.off
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
    // If variable domain is fixed to value
    pub fn is_fixed(&self) -> bool {
        self.size == 1
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn contains(&self, v: i32) -> bool {
        // outside range
        if v < self.off {
            return false;
        }
        let v = v - self.off;
        self.contains_intl(v as u32)
    }
    // This method operates on the shifted value (one cannot shift now).
    fn contains_intl(&self, v: u32) -> bool {
        if v >= self.n {
            false
        } else {
            self.ind[v as usize] < self.size
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

    fn update_bounds_val_removed(&mut self, val: u32) {
        self.update_max_val_removed(val);
        self.update_min_val_removed(val);
    }
    // update after max value is removed
    fn update_max_val_removed(&mut self, val: u32) {
        if !self.is_empty() && self.max == val {
            // The maximum was removed, search the new one
            for v in (self.min..val).rev() {
                if self.contains_intl(v) {
                    self.max = v;
                    return;
                }
            }
        }
    }
    // update after min value is removed
    fn update_min_val_removed(&mut self, val: u32) {
        if !self.is_empty() && self.min == val {
            // The minimum was removed, search the new one
            let vv = val + 1;
            let vvv = self.max + 1;
            for v in vv..vvv {
                if self.contains_intl(v) {
                    self.min = v;
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
        self.exchange(val, self.val[self.size() - 1]);
        self.size = self.size - 1;
        self.update_bounds_val_removed(val);

        return true;
    }

    pub fn remove_all(&mut self) {
        self.size = 0;
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
        self.min = v;
        self.max = v;
        self.size = 1;
    }

    pub fn remove_below(&mut self, val: i32) {
        if self.is_empty() { return; }
        if self.max() < val {
            self.remove_all();
        } else {
            for v in self.min()..val {
                self.remove(v);
            }
        }
    }

    pub fn remove_above(&mut self, val: i32) {
        if self.is_empty() { return; }
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
mod sparse_set_tests {

    use crate::scheduler::Scheduler;

    use super::*;
    type S = Scheduler;

    #[test]
    fn test_debug() {
        let mut v = SparseSet::new(1, 2);
        v.remove_all();
        assert_eq!(
            format!("{:?}", v),
            "SparseSet { off: 1, n: 2, min: 0, max: 1, size: 0, ind: [0, 1], val: [0, 1] }"
        );
    }

    #[test]
    fn test_display() {
        let mut v = SparseSet::new(1, 2);
        v.remove_all();
        assert_eq!(format!("{}", v), "[|1,2]");
    }
    #[test]
    fn test_display2() {
        let mut v = SparseSet::new(1, 2);
        v.remove(2);
        assert_eq!(format!("{}", v), "[1|2]");
    }

    #[test]
    fn test_display3() {
        let v = SparseSet::new(1, 2);
        assert_eq!(format!("{}", v), "[1,2|]");
    }

    #[test]
    fn test_new() {
        let v = SparseSet::new(1, 2);
        assert_eq!(v.off, 1);
        assert_eq!(v.min, 0);
        assert_eq!(v.max, 1);
        assert_eq!(format!("{}", v), "[1,2|]");
    }

    #[test]
    fn test_remove() {
        let mut v = SparseSet::new(1, 5);
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
        let mut v = SparseSet::new(1, 5);
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
        let mut v = SparseSet::new(1, 5);
        v.remove(2);
        v.remove(2); // remove non existent
        assert_eq!(format!("{}", v), "[1,5,3,4|2]");
    }

    #[test]
    fn test_remove_all_but0() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_remove_all_but1() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(7);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_remove_all_but2() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "[2|1,3,4,5]");
        v.remove_all_but(3);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
    }

    #[test]
    fn test_remove_all_but3() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "[2|1,3,4,5]");
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
    }

    #[test]
    fn test_remove_below() {
        let mut v = SparseSet::new(1, 5);
        v.remove_below(3);
        assert_eq!(format!("{}", v), "[5,4,3|2,1]");
        v.remove_below(6);
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn test_remove_below2() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all();
        v.remove_below(0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_remove_above() {
        let mut v = SparseSet::new(3, 7);
        v.remove_above(5);
        assert_eq!(format!("{}", v), "[3,4,5|7,6]");
        v.remove_above(2);
        assert_eq!(v.size(), 0);
        assert_eq!(format!("{}", v), "[|3,4,5,7,6]");
    }

    #[test]
    fn test_remove_above2() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all();
        v.remove_above(6);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn test_min() {
        let mut v = SparseSet::new(1, 5);
        v.remove(1);
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), 2);
        assert_eq!(format!("{}", v), "[5,2,3,4|1]");
    }

    #[test]
    fn test_max() {
        let mut v = SparseSet::new(1, 5);
        v.remove(5);
        assert_eq!(v.size(), 4);
        assert_eq!(v.max(), 4);
        assert_eq!(format!("{}", v), "[1,2,3,4|5]");
    }

    #[test]
    fn test_when_empty() {
        let mut v = SparseSet::new(1, 2);
        v.remove(1);
        v.remove(2);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(format!("{}", v), "[|2,1]");
    }

    #[test]
    fn test_is_fixed() {
        let mut v = SparseSet::new(1, 2);
        v.remove(1);
        assert!(v.is_fixed());
    }

    #[test]
    fn test_contains() {
        let v = SparseSet::new(1, 2);
        assert!(!v.contains(3));
        assert!(v.contains(2));
        assert!(!v.contains_intl(2));
        assert!(v.contains_intl(1));
    }
}
