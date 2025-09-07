// https://github.com/minicp/minicp/blob/mooc/src/main/java/minicp/state/StateSparseSet.java

use std::fmt::Display;

/// State snapshot for backtracking in SparseSet
#[derive(Clone, Debug, PartialEq)]
pub struct SparseSetState {
    pub size: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone)]
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

    /// Get an iterator over the values in the set
    pub fn iter(&self) -> impl Iterator<Item = i32> + '_ {
        self.val[0..self.size as usize]
            .iter()
            .map(move |&v| v as i32 + self.off)
    }

    /// Get the first value in the set (arbitrary order)
    pub fn first(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.val[0] as i32 + self.off)
        }
    }

    /// Get the last value in the set (arbitrary order)
    pub fn last(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.val[self.size as usize - 1] as i32 + self.off)
        }
    }

    /// Convert to a vector of values (for compatibility)
    pub fn to_vec(&self) -> Vec<i32> {
        self.iter().collect()
    }

    /// Get the universe size (total possible values)
    pub fn universe_size(&self) -> usize {
        self.n as usize
    }

    /// Get the minimum value in the universe
    pub fn min_universe_value(&self) -> i32 {
        self.off
    }

    /// Get the maximum value in the universe
    pub fn max_universe_value(&self) -> i32 {
        self.off + self.n as i32 - 1
    }

    /// Set intersection - modify this set to contain only elements in both sets
    pub fn intersect_with(&mut self, other: &SparseSet) {
        // Create a list of values to remove to avoid modifying while iterating
        let mut to_remove = Vec::new();
        
        for val in self.iter() {
            if !other.contains(val) {
                to_remove.push(val);
            }
        }
        
        for val in to_remove {
            self.remove(val);
        }
    }

    /// Set union - add all elements from other set to this set
    /// Note: This requires that both sets have compatible universes
    pub fn union_with(&mut self, other: &SparseSet) {
        for val in other.iter() {
            if self.contains(val) {
                continue; // Already present
            }
            
            // Check if value is in our universe
            if val >= self.off && val < self.off + self.n as i32 {
                // Manually add the value (similar to remove but in reverse)
                let val_internal = (val - self.off) as u32;
                if !self.contains_intl(val_internal) {
                    // Add value to the end of the active set
                    let new_pos = self.size;
                    let old_val_at_pos = self.val[new_pos as usize];
                    
                    // Swap the value to the active part
                    self.exchange(val_internal, old_val_at_pos);
                    self.size += 1;
                    
                    // Update bounds if necessary
                    if self.size == 1 {
                        self.min = val_internal;
                        self.max = val_internal;
                    } else {
                        if val_internal < self.min {
                            self.min = val_internal;
                        }
                        if val_internal > self.max {
                            self.max = val_internal;
                        }
                    }
                }
            }
        }
    }

    /// Check if this set is a subset of another set
    pub fn is_subset_of(&self, other: &Self) -> bool {
        // Adjust for different offsets
        for i in 0..self.size {
            let val = self.val[i as usize];
            let external_val = val as i32 + self.off;
            
            if !other.contains(external_val) {
                return false;
            }
        }
        true
    }

    /// Check if two sets are equal
    pub fn equals(&self, other: &SparseSet) -> bool {
        if self.size != other.size {
            return false;
        }
        self.iter().all(|val| other.contains(val))
    }

    /// Create a new sparse set from a vector of values
    pub fn from_values(values: Vec<i32>) -> Self {
        if values.is_empty() {
            // Create an empty set with minimal valid range
            let mut set = Self::new(0, 0);
            set.remove_all(); // Make it empty
            return set;
        }
        
        let min_val = *values.iter().min().unwrap();
        let max_val = *values.iter().max().unwrap();
        let mut set = Self::new(min_val, max_val);
        
        // Remove all values first, then add only the specified ones
        set.remove_all();
        
        for val in values {
            if val >= min_val && val <= max_val {
                let val_internal = (val - set.off) as u32;
                // Add value to the active set
                let new_pos = set.size;
                let old_val_at_pos = set.val[new_pos as usize];
                
                // Swap the value to the active part
                set.exchange(val_internal, old_val_at_pos);
                set.size += 1;
                
                // Update bounds
                if set.size == 1 {
                    set.min = val_internal;
                    set.max = val_internal;
                } else {
                    if val_internal < set.min {
                        set.min = val_internal;
                    }
                    if val_internal > set.max {
                        set.max = val_internal;
                    }
                }
            }
        }
        
        set
    }

    // ===== BACKTRACKING SUPPORT =====
    
    /// Save the current state for backtracking
    pub fn save_state(&self) -> SparseSetState {
        SparseSetState {
            size: self.size,
            min: self.min,
            max: self.max,
        }
    }
    
    /// Restore a previously saved state
    pub fn restore_state(&mut self, state: &SparseSetState) {
        self.size = state.size;
        self.min = state.min;
        self.max = state.max;
    }
    
    /// Get current state size (for simple size-only backtracking)
    pub fn current_size(&self) -> u32 {
        self.size
    }
    
    /// Restore to a specific size (assumes min/max are still valid)
    /// WARNING: Only use if you're certain min/max haven't changed since the size was recorded
    pub fn restore_size(&mut self, size: u32) {
        debug_assert!(size <= self.n, "Cannot restore to size larger than universe");
        self.size = size;
    }
}

impl PartialEq for SparseSet {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for SparseSet {}

/// Iterator implementation for sparse set
impl IntoIterator for SparseSet {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<'a> IntoIterator for &'a SparseSet {
    type Item = i32;
    type IntoIter = Box<dyn Iterator<Item = i32> + 'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn debug() {
        let mut v = SparseSet::new(1, 2);
        v.remove_all();
        assert_eq!(
            format!("{:?}", v),
            "SparseSet { off: 1, n: 2, min: 0, max: 1, size: 0, ind: [0, 1], val: [0, 1] }"
        );
    }

    #[test]
    fn display() {
        let mut v = SparseSet::new(1, 2);
        v.remove_all();
        assert_eq!(format!("{}", v), "[|1,2]");
    }

    #[test]
    fn display2() {
        let mut v = SparseSet::new(1, 2);
        v.remove(2);
        assert_eq!(format!("{}", v), "[1|2]");
    }

    #[test]
    fn display3() {
        let v = SparseSet::new(1, 2);
        assert_eq!(format!("{}", v), "[1,2|]");
    }

    #[test]
    fn new() {
        let v = SparseSet::new(1, 2);
        assert_eq!(v.off, 1);
        assert_eq!(v.min, 0);
        assert_eq!(v.max, 1);
        assert_eq!(format!("{}", v), "[1,2|]");
    }

    #[test]
    fn remove() {
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
    fn remove2() {
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
    fn remove3() {
        let mut v = SparseSet::new(1, 5);
        v.remove(2);
        v.remove(2); // remove non existent
        assert_eq!(format!("{}", v), "[1,5,3,4|2]");
    }

    #[test]
    fn remove_all_but0() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn remove_all_but1() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(7);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn remove_all_but2() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "[2|1,3,4,5]");
        v.remove_all_but(3);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
    }

    #[test]
    fn remove_all_but3() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all_but(2);
        assert_eq!(format!("{}", v), "[2|1,3,4,5]");
        v.remove_all_but(0);
        assert_eq!(format!("{}", v), "[|2,1,3,4,5]");
    }

    #[test]
    fn remove_below() {
        let mut v = SparseSet::new(1, 5);
        v.remove_below(3);
        assert_eq!(format!("{}", v), "[5,4,3|2,1]");
        v.remove_below(6);
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn remove_below2() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all();
        v.remove_below(0);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn remove_above() {
        let mut v = SparseSet::new(3, 7);
        v.remove_above(5);
        assert_eq!(format!("{}", v), "[3,4,5|7,6]");
        v.remove_above(2);
        assert_eq!(v.size(), 0);
        assert_eq!(format!("{}", v), "[|3,4,5,7,6]");
    }

    #[test]
    fn remove_above2() {
        let mut v = SparseSet::new(1, 5);
        v.remove_all();
        v.remove_above(6);
        assert_eq!(format!("{}", v), "[|1,2,3,4,5]");
    }

    #[test]
    fn min() {
        let mut v = SparseSet::new(1, 5);
        v.remove(1);
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), 2);
        assert_eq!(format!("{}", v), "[5,2,3,4|1]");
    }

    #[test]
    fn max() {
        let mut v = SparseSet::new(1, 5);
        v.remove(5);
        assert_eq!(v.size(), 4);
        assert_eq!(v.max(), 4);
        assert_eq!(format!("{}", v), "[1,2,3,4|5]");
    }

    #[test]
    fn when_empty() {
        let mut v = SparseSet::new(1, 2);
        v.remove(1);
        v.remove(2);
        assert_eq!(v.size(), 0);
        assert!(v.is_empty());
        assert_eq!(format!("{}", v), "[|2,1]");
    }

    #[test]
    fn is_fixed() {
        let mut v = SparseSet::new(1, 2);
        v.remove(1);
        assert!(v.is_fixed());
    }

    #[test]
    fn contains() {
        let v = SparseSet::new(1, 2);
        assert!(!v.contains(3));
        assert!(v.contains(2));
        assert!(!v.contains_intl(2));
        assert!(v.contains_intl(1));
    }

    // New tests for enhanced functionality

    #[test]
    fn test_iterator() {
        let mut v = SparseSet::new(1, 5);
        v.remove(3);
        
        let values: Vec<i32> = v.iter().collect();
        assert_eq!(values.len(), 4);
        assert!(values.contains(&1));
        assert!(values.contains(&2));
        assert!(!values.contains(&3));
        assert!(values.contains(&4));
        assert!(values.contains(&5));
    }

    #[test]
    fn test_into_iterator() {
        let mut v = SparseSet::new(1, 3);
        v.remove(2);
        
        let values: Vec<i32> = (&v).into_iter().collect();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&1));
        assert!(values.contains(&3));
        
        // Test owned iterator
        let values2: Vec<i32> = v.into_iter().collect();
        assert_eq!(values2.len(), 2);
        assert!(values2.contains(&1));
        assert!(values2.contains(&3));
    }

    #[test]
    fn test_first_last() {
        let mut v = SparseSet::new(1, 5);
        
        // Test with full set
        assert!(v.first().is_some());
        assert!(v.last().is_some());
        
        // Test with partial set
        v.remove(3);
        assert!(v.first().is_some());
        assert!(v.last().is_some());
        
        // Test with empty set
        v.remove_all();
        assert_eq!(v.first(), None);
        assert_eq!(v.last(), None);
    }

    #[test]
    fn test_to_vec() {
        let mut v = SparseSet::new(1, 3);
        v.remove(2);
        
        let vec = v.to_vec();
        assert_eq!(vec.len(), 2);
        assert!(vec.contains(&1));
        assert!(vec.contains(&3));
    }

    #[test]
    fn test_universe_info() {
        let v = SparseSet::new(5, 10);
        
        assert_eq!(v.universe_size(), 6);
        assert_eq!(v.min_universe_value(), 5);
        assert_eq!(v.max_universe_value(), 10);
    }

    #[test]
    fn test_from_values() {
        let v = SparseSet::from_values(vec![2, 4, 6, 8]);
        
        assert_eq!(v.size(), 4);
        assert!(v.contains(2));
        assert!(v.contains(4));
        assert!(v.contains(6));
        assert!(v.contains(8));
        assert!(!v.contains(1));
        assert!(!v.contains(3));
        assert!(!v.contains(5));
        assert!(!v.contains(7));
    }

    #[test]
    fn test_from_values_empty() {
        let v = SparseSet::from_values(vec![]);
        assert!(v.is_empty());
        assert_eq!(v.size(), 0);
    }

    #[test]
    fn test_intersect_with() {
        let mut v1 = SparseSet::new(1, 5);
        let mut v2 = SparseSet::new(1, 5);
        
        // v1 = {1, 2, 3, 4, 5}
        // v2 = {2, 4} after removing 1, 3, 5
        v2.remove(1);
        v2.remove(3);
        v2.remove(5);
        
        v1.intersect_with(&v2);
        
        // v1 should now be {2, 4}
        assert_eq!(v1.size(), 2);
        assert!(v1.contains(2));
        assert!(v1.contains(4));
        assert!(!v1.contains(1));
        assert!(!v1.contains(3));
        assert!(!v1.contains(5));
    }

    #[test]
    fn test_union_with() {
        let mut v1 = SparseSet::from_values(vec![1, 3, 5]);
        let v2 = SparseSet::from_values(vec![2, 4, 5]); // 5 is common
        
        v1.union_with(&v2);
        
        // v1 should now contain {1, 2, 3, 4, 5}
        assert_eq!(v1.size(), 5);
        for i in 1..=5 {
            assert!(v1.contains(i));
        }
    }

    #[test]
    fn test_is_subset_of() {
        let v1 = SparseSet::from_values(vec![2, 4]);
        let v2 = SparseSet::from_values(vec![1, 2, 3, 4, 5]);
        let v3 = SparseSet::from_values(vec![2, 4, 6]);
        
        assert!(v1.is_subset_of(&v2)); // {2, 4} ⊆ {1, 2, 3, 4, 5}
        assert!(v1.is_subset_of(&v3)); // {2, 4} ⊆ {2, 4, 6} - mathematically correct
        assert!(v1.is_subset_of(&v1)); // Set is subset of itself
        
        // Test empty set is subset of any set
        let empty = SparseSet::from_values(vec![]);
        assert!(empty.is_subset_of(&v1));
        assert!(empty.is_subset_of(&v2));
    }

    #[test]
    fn test_equals() {
        let v1 = SparseSet::from_values(vec![1, 3, 5]);
        let v2 = SparseSet::from_values(vec![5, 1, 3]); // Same values, different order
        let v3 = SparseSet::from_values(vec![1, 3]);
        
        assert!(v1.equals(&v2));
        assert!(!v1.equals(&v3));
        assert!(v1 == v2); // Test PartialEq implementation
        assert!(v1 != v3);
    }

    #[test]
    fn test_performance_large_domain() {
        // Test with a larger domain to ensure operations remain efficient
        let mut v = SparseSet::new(0, 1000);
        
        // Remove odd numbers
        for i in 1..1000 {
            if i % 2 == 1 {
                v.remove(i);
            }
        }

        assert_eq!(v.size(), 501); // Should have 501 even numbers: 0, 2, 4, ..., 1000
        
        // Test contains for all values
        for i in 0..1000 {
            if i % 2 == 0 {
                assert!(v.contains(i));
            } else {
                assert!(!v.contains(i));
            }
        }
        
        // Test iteration
        let even_values: Vec<i32> = v.iter().filter(|&x| x % 2 == 0).collect();
        assert_eq!(even_values.len(), 501);
    }

    #[test]
    fn test_csp_specific_operations() {
        let mut v = SparseSet::new(1, 9); // Sudoku domain
        
        // Test remove_below (useful for constraint propagation)
        v.remove_below(5);
        assert_eq!(v.size(), 5); // Should have {5, 6, 7, 8, 9}
        assert!(!v.contains(4));
        assert!(v.contains(5));
        assert!(v.contains(9));
        
        // Test remove_above
        v.remove_above(7);
        assert_eq!(v.size(), 3); // Should have {5, 6, 7}
        assert!(v.contains(5));
        assert!(v.contains(7));
        assert!(!v.contains(8));
        
        // Test remove_all_but (useful for variable assignment)
        v.remove_all_but(6);
        assert_eq!(v.size(), 1);
        assert!(v.is_fixed());
        assert!(v.contains(6));
        assert!(!v.contains(5));
        assert!(!v.contains(7));
    }

    #[test]
    fn test_bounds_maintenance() {
        let mut v = SparseSet::new(1, 10);
        
        // Test that bounds are correctly maintained after removals
        v.remove(1); // Remove minimum
        assert_eq!(v.min(), 2);
        
        v.remove(10); // Remove maximum  
        assert_eq!(v.max(), 9);
        
        v.remove(5); // Remove middle value
        assert_eq!(v.min(), 2); // Should still be 2
        assert_eq!(v.max(), 9); // Should still be 9
        
        // Test bounds after removing all but one
        v.remove_all_but(7);
        assert_eq!(v.min(), 7);
        assert_eq!(v.max(), 7);
    }

    #[test]
    fn test_backtracking_save_restore() {
        let mut set = SparseSet::new(1, 10);
        
        // Save initial state
        let initial_state = set.save_state();
        assert_eq!(initial_state.size, 10);
        assert_eq!(initial_state.min, 0); // Internal representation: 1-1=0
        assert_eq!(initial_state.max, 9); // Internal representation: 10-1=9
        
        // Make some changes
        set.remove(1);
        set.remove(10);
        set.remove(5);
        
        assert_eq!(set.size(), 7);
        assert_eq!(set.min(), 2);
        assert_eq!(set.max(), 9);
        
        // Save intermediate state
        let intermediate_state = set.save_state();
        
        // Make more changes
        set.remove_all_but(6);
        assert_eq!(set.size(), 1);
        assert_eq!(set.min(), 6);
        assert_eq!(set.max(), 6);
        
        // Restore to intermediate state
        set.restore_state(&intermediate_state);
        assert_eq!(set.size(), 7);
        assert_eq!(set.min(), 2);
        assert_eq!(set.max(), 9);
        assert!(!set.contains(1));
        assert!(!set.contains(10));
        assert!(!set.contains(5));
        assert!(set.contains(2));
        assert!(set.contains(6));
        assert!(set.contains(9));
        
        // Restore to initial state
        set.restore_state(&initial_state);
        assert_eq!(set.size(), 10);
        assert_eq!(set.min(), 1);
        assert_eq!(set.max(), 10);
        for i in 1..=10 {
            assert!(set.contains(i));
        }
    }

    #[test]
    fn test_size_only_backtracking() {
        let mut set = SparseSet::new(5, 8);
        
        let original_size = set.current_size();
        assert_eq!(original_size, 4);
        
        // Remove some elements (but not min/max)
        set.remove(6);
        set.remove(7);
        
        assert_eq!(set.size(), 2);
        assert_eq!(set.min(), 5); // Min unchanged
        assert_eq!(set.max(), 8); // Max unchanged
        
        // Restore size (safe because min/max didn't change)
        set.restore_size(original_size);
        assert_eq!(set.size(), 4);
        assert_eq!(set.min(), 5);
        assert_eq!(set.max(), 8);
        
        // All original elements should be back
        assert!(set.contains(5));
        assert!(set.contains(6));
        assert!(set.contains(7));
        assert!(set.contains(8));
    }

    #[test]
    fn test_backtracking_with_bounds_changes() {
        let mut set = SparseSet::new(1, 5);
        
        // Save state before removing min/max
        let before_bounds_change = set.save_state();
        
        // Remove min and max (this will update bounds)
        set.remove(1); // Remove min
        set.remove(5); // Remove max
        
        assert_eq!(set.min(), 2);
        assert_eq!(set.max(), 4);
        assert_eq!(set.size(), 3);
        
        // Save state after bounds change
        let after_bounds_change = set.save_state();
        
        // Remove more elements
        set.remove(3);
        assert_eq!(set.size(), 2);
        
        // Restore to after bounds change
        set.restore_state(&after_bounds_change);
        assert_eq!(set.size(), 3);
        assert_eq!(set.min(), 2);
        assert_eq!(set.max(), 4);
        assert!(set.contains(2));
        assert!(set.contains(3));
        assert!(set.contains(4));
        
        // Restore to before bounds change
        set.restore_state(&before_bounds_change);
        assert_eq!(set.size(), 5);
        assert_eq!(set.min(), 1);
        assert_eq!(set.max(), 5);
        for i in 1..=5 {
            assert!(set.contains(i));
        }
    }

    #[test]
    fn test_multiple_backtrack_levels() {
        let mut set = SparseSet::new(1, 6);
        
        // Level 0: Full set
        let level0 = set.save_state();
        
        // Level 1: Remove one element
        set.remove(3);
        let level1 = set.save_state();
        
        // Level 2: Remove more elements
        set.remove(1);
        set.remove(6);
        let level2 = set.save_state();
        
        // Level 3: Remove to single element
        set.remove_all_but(4);
        
        assert_eq!(set.size(), 1);
        assert!(set.contains(4));
        
        // Backtrack to level 2
        set.restore_state(&level2);
        assert_eq!(set.size(), 3);
        assert!(set.contains(2));
        assert!(set.contains(4));
        assert!(set.contains(5));
        
        // Backtrack to level 1
        set.restore_state(&level1);
        assert_eq!(set.size(), 5);
        assert!(!set.contains(3));
        for i in [1, 2, 4, 5, 6] {
            assert!(set.contains(i));
        }
        
        // Backtrack to level 0
        set.restore_state(&level0);
        assert_eq!(set.size(), 6);
        for i in 1..=6 {
            assert!(set.contains(i));
        }
    }
}
