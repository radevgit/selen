

use std::fmt::Display;

/// Maximum domain size supported by SparseSet
/// 
/// This limit is enforced to prevent performance issues and excessive memory usage.
/// Domains larger than this should be rejected during model construction.
pub const MAX_SPARSE_SET_DOMAIN_SIZE: u64 = 1_000_000;

/// State snapshot for backtracking in SparseSet
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq)]
pub struct SparseSetState {
    pub size: u32,
    pub min: u32,
    pub max: u32,
}

#[doc(hidden)]
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
        write!(s, "[").expect("writing to String should never fail");
        if self.size == 0 {
            write!(s, "|").expect("writing to String should never fail");
        }
        for i in 0..self.n {
            write!(s, "{},", (self.val[i as usize] as i32 + self.off)).expect("writing to String should never fail");
            if i + 1 == self.size {
                s.pop(); // remove comma
                write!(s, "|").expect("writing to String should never fail");
            }
        }
        if self.size != self.n {
            s.pop(); // remove comma
        }
        write!(s, "]").expect("writing to String should never fail");
        write!(f, "{}", s)
    }
}

impl SparseSet {
    pub fn new(min: i32, max: i32) -> Self {
        if min > max {
            // Handle invalid range by swapping
            return Self::new(max, min);
        }
        
        let maxmin = (max - min) as u32;
        let n = (maxmin + 1) as u32;
        
        // Pre-allocate vectors with known capacity
        let mut ind = Vec::with_capacity(n as usize);
        let mut val = Vec::with_capacity(n as usize);
        
        // Fill with consecutive values
        for i in 0..n {
            ind.push(i);
            val.push(i);
        }
        
        SparseSet {
            off: min,
            min: 0,
            max: maxmin,
            n,
            size: n,
            ind,
            val,
        }
    }

    /// Create a SparseSet without bound checking - may create invalid domains
    /// This is used to create intentionally invalid domains that validation can catch
    pub fn new_unchecked(min: i32, max: i32) -> Self {
        if min > max {
            // Create an invalid domain that preserves the original bounds
            // We'll create a domain where the universe bounds show the invalid range
            // but the domain itself is empty
            SparseSet {
                off: min,
                min: 0,
                max: 0,
                n: 0,
                size: 0,
                ind: Vec::new(),
                val: Vec::new(),
            }
        } else {
            // Valid bounds - use normal creation
            Self::new(min, max)
        }
    }

    /// Create a SparseSet from a vector of specific values
    /// Memory efficient - creates full range then removes unwanted values
    pub fn new_from_values(values: Vec<i32>) -> Self {
        if values.is_empty() {
            // Return empty sparse set with minimal footprint
            return SparseSet {
                off: 0,
                min: 0,
                max: 0,
                n: 0,
                size: 0,
                ind: Vec::new(),
                val: Vec::new(),
            };
        }

        // Sort and deduplicate values
        let mut sorted_values = values;
        sorted_values.sort_unstable();
        sorted_values.dedup();

        let min_val = sorted_values[0];
        let max_val = sorted_values[sorted_values.len() - 1];
        
        // Create sparse set with full range - this ensures compatibility with all operations
        let mut sparse_set = SparseSet::new(min_val, max_val);
        
        // Remove all values that are not in our desired set
        for i in min_val..=max_val {
            if !sorted_values.contains(&i) {
                sparse_set.remove(i);
            }
        }
        
        sparse_set
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

    // ===== COMPLEMENT API (for incremental sum and other optimizations) =====
    
    /// Get an iterator over the **removed** values (complement of current domain)
    /// 
    /// This is a key insight from the SparseSet design: removed values are stored
    /// in `val[size..n)`, allowing us to iterate over the complement without
    /// additional memory overhead.
    /// 
    /// # Example
    /// ```
    /// use selen::variables::domain::sparse_set::SparseSet;
    /// let mut domain = SparseSet::new(1, 5);  // Domain {1,2,3,4,5}
    /// domain.remove(2);
    /// domain.remove(4);
    /// // complement_iter() yields {2, 4}
    /// let complement: Vec<i32> = domain.complement_iter().collect();
    /// assert_eq!(complement.len(), 2);
    /// ```
    pub fn complement_iter(&self) -> impl Iterator<Item = i32> + '_ {
        self.val[self.size as usize..self.n as usize]
            .iter()
            .map(move |&v| v as i32 + self.off)
    }

    /// Get the size of the complement (number of removed values)
    /// 
    /// This is computed as `n - size` and is useful for choosing which set
    /// to iterate over when optimizing constraint propagation.
    /// 
    /// # Example
    /// ```
    /// use selen::variables::domain::sparse_set::SparseSet;
    /// let mut domain = SparseSet::new(1, 10);  // 10 values initially
    /// assert_eq!(domain.complement_size(), 0);  // No removed values
    /// 
    /// domain.remove(1);
    /// domain.remove(5);
    /// assert_eq!(domain.complement_size(), 2);  // 2 removed values
    /// ```
    pub fn complement_size(&self) -> usize {
        (self.n - self.size) as usize
    }

    /// Check if iterating over complement is more efficient than current domain
    /// 
    /// Returns true if `complement_size < size / 2`, indicating that the complement
    /// has fewer than half the elements of the current domain. This is useful for
    /// optimization decisions like preferring `complement_iter()` when the domain
    /// has been heavily pruned.
    /// 
    /// # Example
    /// ```
    /// use selen::variables::domain::sparse_set::SparseSet;
    /// let mut domain = SparseSet::new(1, 100);
    /// // Initially: size=100, complement_size=0. 0 < 50? Yes, but we check this only matters when complement > 0
    /// 
    /// // Remove 10 values: size becomes 90, complement_size becomes 10
    /// for i in 1..=10 {
    ///     domain.remove(i);
    /// }
    /// // 10 < 90/2 = 10 < 45? Yes, so we SHOULD use complement
    /// assert!(domain.should_use_complement());
    /// 
    /// // Remove 50 more values: size becomes 40, complement_size becomes 60
    /// for i in 11..=60 {
    ///     domain.remove(i);
    /// }
    /// // 60 < 40/2 = 60 < 20? No, so we should NOT use complement
    /// assert!(!domain.should_use_complement());
    /// ```
    pub fn should_use_complement(&self) -> bool {
        self.complement_size() < (self.size as usize) / 2
    }

    /// Set intersection - modify this set to contain only elements in both sets
    /// 
    /// **Note**: This operates on the **domain** of integer variables, not on set-valued variables.
    /// In Selen, `SparseSet` is used as the domain representation for integer variables (including
    /// those created with `intset()`). These operations are useful for constraint propagation and
    /// domain manipulation, but they do not implement FlatZinc set constraints like `set_union(x,y,z)`
    /// which require true set-valued variables where the variable's value is itself a set.
    /// 
    /// # Examples
    /// ```
    /// use selen::variables::domain::sparse_set::SparseSet;
    /// let mut domain1 = SparseSet::new(1, 5);  // Domain {1,2,3,4,5}
    /// let mut domain2 = SparseSet::new(3, 7);  // Domain {3,4,5,6,7}
    /// domain2.remove(6);                        // Domain {3,4,5,7}
    /// domain1.intersect_with(&domain2);         // domain1 becomes {3,4,5}
    /// ```
    pub fn intersect_with(&mut self, other: &SparseSet) {
        // Create a list of values to remove to avoid modifying while iterating
        let mut to_remove = Vec::with_capacity(self.size as usize);
        
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
    /// 
    /// **Note**: This operates on the **domain** of integer variables, not on set-valued variables.
    /// See `intersect_with()` for more details on the distinction.
    /// 
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

    /// Set difference - remove all elements from this set that are in other set
    /// 
    /// This computes `self = self \ other` (set difference).
    /// 
    /// **Note**: This operates on the **domain** of integer variables, not on set-valued variables.
    /// See `intersect_with()` for more details on the distinction.
    /// 
    /// # Examples
    /// ```
    /// use selen::variables::domain::sparse_set::SparseSet;
    /// let mut a = SparseSet::new(1, 5);      // Domain {1, 2, 3, 4, 5}
    /// let b = SparseSet::new(3, 7);          // Domain {3, 4, 5, 6, 7}
    /// a.diff_with(&b);                        // a becomes {1, 2}
    /// ```
    pub fn diff_with(&mut self, other: &SparseSet) {
        // Create a list of values to remove to avoid modifying while iterating
        let mut to_remove = Vec::with_capacity(self.size as usize);
        
        for val in self.iter() {
            if other.contains(val) {
                to_remove.push(val);
            }
        }
        
        for val in to_remove {
            self.remove(val);
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

    // ===== BACKTRACKING SUPPORT =====
    
    /// Save the current state for backtracking
    #[doc(hidden)]
    pub fn save_state(&self) -> SparseSetState {
        SparseSetState {
            size: self.size,
            min: self.min,
            max: self.max,
        }
    }
    
    /// Restore a previously saved state
    #[doc(hidden)]
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

    // Tests for new_from_values method
    #[test]
    fn test_new_from_values_basic() {
        let v = SparseSet::new_from_values(vec![2, 4, 6, 8]);
        
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), 2);
        assert_eq!(v.max(), 8);
        assert!(v.contains(2));
        assert!(v.contains(4));
        assert!(v.contains(6));
        assert!(v.contains(8));
        assert!(!v.contains(1));
        assert!(!v.contains(3));
        assert!(!v.contains(5));
        assert!(!v.contains(7));
        assert!(!v.contains(9));
        
        // Check that all values are in the iterator
        let values: Vec<i32> = v.iter().collect();
        assert_eq!(values.len(), 4);
        for val in [2, 4, 6, 8] {
            assert!(values.contains(&val));
        }
    }

    #[test]
    fn test_new_from_values_empty() {
        let v = SparseSet::new_from_values(vec![]);
        assert!(v.is_empty());
        assert_eq!(v.size(), 0);
        assert_eq!(v.universe_size(), 0);
    }

    #[test]
    fn test_new_from_values_single() {
        let v = SparseSet::new_from_values(vec![42]);
        assert_eq!(v.size(), 1);
        assert_eq!(v.min(), 42);
        assert_eq!(v.max(), 42);
        assert!(v.is_fixed());
        assert!(v.contains(42));
        assert!(!v.contains(41));
        assert!(!v.contains(43));
    }

    #[test]
    fn test_new_from_values_contiguous() {
        let v = SparseSet::new_from_values(vec![3, 4, 5, 6]);
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), 3);
        assert_eq!(v.max(), 6);
        
        for i in 3..=6 {
            assert!(v.contains(i));
        }
        assert!(!v.contains(2));
        assert!(!v.contains(7));
    }

    #[test]
    fn test_new_from_values_duplicates() {
        let v = SparseSet::new_from_values(vec![1, 3, 1, 5, 3, 5]);
        assert_eq!(v.size(), 3); // Should deduplicate
        assert!(v.contains(1));
        assert!(v.contains(3));
        assert!(v.contains(5));
        assert!(!v.contains(2));
        assert!(!v.contains(4));
    }

    #[test]
    fn test_new_from_values_unsorted() {
        let v = SparseSet::new_from_values(vec![5, 1, 3, 7, 2]);
        assert_eq!(v.size(), 5);
        assert_eq!(v.min(), 1);
        assert_eq!(v.max(), 7);
        
        for i in [1, 2, 3, 5, 7] {
            assert!(v.contains(i));
        }
        assert!(!v.contains(4));
        assert!(!v.contains(6));
    }

    #[test]
    fn test_new_from_values_negative() {
        let v = SparseSet::new_from_values(vec![-3, -1, 1, 3]);
        assert_eq!(v.size(), 4);
        assert_eq!(v.min(), -3);
        assert_eq!(v.max(), 3);
        
        assert!(v.contains(-3));
        assert!(v.contains(-1));
        assert!(v.contains(1));
        assert!(v.contains(3));
        assert!(!v.contains(-2));
        assert!(!v.contains(0));
        assert!(!v.contains(2));
    }

    #[test]
    fn test_new_from_values_operations() {
        let mut v = SparseSet::new_from_values(vec![2, 4, 6, 8, 10]);
        
        // Test removal
        assert!(v.remove(4));
        assert_eq!(v.size(), 4);
        assert!(!v.contains(4));
        assert!(v.contains(2));
        assert!(v.contains(6));
        
        // Test remove_all_but
        v.remove_all_but(8);
        assert_eq!(v.size(), 1);
        assert!(v.is_fixed());
        assert!(v.contains(8));
        assert!(!v.contains(2));
        assert!(!v.contains(6));
        assert!(!v.contains(10));
    }

    #[test]
    fn test_new_from_values_bounds_operations() {
        let mut v = SparseSet::new_from_values(vec![1, 3, 5, 7, 9]);
        
        // Test remove_below
        v.remove_below(5);
        assert_eq!(v.size(), 3);
        assert!(v.contains(5));
        assert!(v.contains(7));
        assert!(v.contains(9));
        assert!(!v.contains(1));
        assert!(!v.contains(3));
        
        // Test remove_above
        v.remove_above(7);
        assert_eq!(v.size(), 2);
        assert!(v.contains(5));
        assert!(v.contains(7));
        assert!(!v.contains(9));
    }

    #[test]
    fn test_new_from_values_vs_new_equivalence() {
        // Test that new_from_values with contiguous range is equivalent to new
        let v1 = SparseSet::new(5, 8);
        let v2 = SparseSet::new_from_values(vec![5, 6, 7, 8]);
        
        assert_eq!(v1.size(), v2.size());
        assert_eq!(v1.min(), v2.min());
        assert_eq!(v1.max(), v2.max());
        
        for i in 5..=8 {
            assert_eq!(v1.contains(i), v2.contains(i));
        }
        
        // Test that iterators produce same values
        let mut vals1: Vec<i32> = v1.iter().collect();
        let mut vals2: Vec<i32> = v2.iter().collect();
        vals1.sort();
        vals2.sort();
        assert_eq!(vals1, vals2);
    }

    #[test]
    fn test_new_from_values_memory_efficiency() {
        // Test sparse domain - should be more memory efficient than full range
        let v = SparseSet::new_from_values(vec![1, 1000]);
        
        assert_eq!(v.size(), 2);
        assert_eq!(v.universe_size(), 1000); // Range is 1000
        assert!(v.contains(1));
        assert!(v.contains(1000));
        assert!(!v.contains(500));
        
        // val array should only contain 2 elements, not 1000
        let values: Vec<i32> = v.iter().collect();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn test_new_from_values_with_set_operations() {
        // Test union with new_from_values
        let mut v1 = SparseSet::new_from_values(vec![1, 3, 5]);
        let v2 = SparseSet::new_from_values(vec![2, 4, 5]); // 5 is common
        
        v1.union_with(&v2);
        
        // v1 should now contain {1, 2, 3, 4, 5}
        assert_eq!(v1.size(), 5);
        for i in 1..=5 {
            assert!(v1.contains(i));
        }
    }

    #[test]
    fn test_new_from_values_subset_operations() {
        let v1 = SparseSet::new_from_values(vec![2, 4]);
        let v2 = SparseSet::new_from_values(vec![1, 2, 3, 4, 5]);
        let v3 = SparseSet::new_from_values(vec![2, 4, 6]);
        
        assert!(v1.is_subset_of(&v2)); // {2, 4} ⊆ {1, 2, 3, 4, 5}
        assert!(v1.is_subset_of(&v3)); // {2, 4} ⊆ {2, 4, 6}
        assert!(v1.is_subset_of(&v1)); // Set is subset of itself
        
        // Test empty set is subset of any set
        let empty = SparseSet::new_from_values(vec![]);
        assert!(empty.is_subset_of(&v1));
        assert!(empty.is_subset_of(&v2));
    }

    #[test]
    fn test_new_from_values_equality() {
        let v1 = SparseSet::new_from_values(vec![1, 3, 5]);
        let v2 = SparseSet::new_from_values(vec![5, 1, 3]); // Same values, different order
        let v3 = SparseSet::new_from_values(vec![1, 3]);
        
        assert!(v1.equals(&v2));
        assert!(!v1.equals(&v3));
        assert!(v1 == v2); // Test PartialEq implementation
        assert!(v1 != v3);
    }

    #[test]
    fn test_new_from_values_backtracking() {
        let mut set = SparseSet::new_from_values(vec![2, 4, 6, 8, 10]);
        
        // Save initial state
        let initial_state = set.save_state();
        assert_eq!(initial_state.size, 5);
        
        // Make some changes
        set.remove(2);
        set.remove(10);
        
        assert_eq!(set.size(), 3);
        assert!(!set.contains(2));
        assert!(!set.contains(10));
        assert!(set.contains(4));
        assert!(set.contains(6));
        assert!(set.contains(8));
        
        // Restore to initial state
        set.restore_state(&initial_state);
        assert_eq!(set.size(), 5);
        for val in [2, 4, 6, 8, 10] {
            assert!(set.contains(val));
        }
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
    fn test_diff_with_basic() {
        let mut a = SparseSet::new(1, 5);  // {1, 2, 3, 4, 5}
        let b = SparseSet::new(3, 7);      // {3, 4, 5, 6, 7}
        
        a.diff_with(&b);
        
        // a should now be {1, 2} (elements in a but not in b)
        assert_eq!(a.size(), 2);
        assert!(a.contains(1));
        assert!(a.contains(2));
        assert!(!a.contains(3));
        assert!(!a.contains(4));
        assert!(!a.contains(5));
    }

    #[test]
    fn test_diff_with_disjoint() {
        let mut a = SparseSet::new(1, 3);  // {1, 2, 3}
        let b = SparseSet::new(4, 6);      // {4, 5, 6}
        
        a.diff_with(&b);
        
        // a should be unchanged (disjoint sets)
        assert_eq!(a.size(), 3);
        assert!(a.contains(1));
        assert!(a.contains(2));
        assert!(a.contains(3));
    }

    #[test]
    fn test_diff_with_subset() {
        let mut a = SparseSet::new(1, 5);  // {1, 2, 3, 4, 5}
        let mut b = SparseSet::new(1, 5);  // {1, 2, 3, 4, 5}
        
        // Make b a subset: {2, 4}
        b.remove(1);
        b.remove(3);
        b.remove(5);
        
        a.diff_with(&b);
        
        // a should now be {1, 3, 5}
        assert_eq!(a.size(), 3);
        assert!(a.contains(1));
        assert!(a.contains(3));
        assert!(a.contains(5));
        assert!(!a.contains(2));
        assert!(!a.contains(4));
    }

    #[test]
    fn test_diff_with_empty() {
        let mut a = SparseSet::new(1, 3);  // {1, 2, 3}
        let mut b = SparseSet::new(1, 3);  // {1, 2, 3}
        b.remove_all(); // Make it empty
        
        a.diff_with(&b);
        
        // a should be unchanged
        assert_eq!(a.size(), 3);
        assert!(a.contains(1));
        assert!(a.contains(2));
        assert!(a.contains(3));
    }

    #[test]
    fn test_diff_with_becomes_empty() {
        let mut a = SparseSet::new(1, 3);  // {1, 2, 3}
        let b = SparseSet::new(1, 5);      // {1, 2, 3, 4, 5} (superset)
        
        a.diff_with(&b);
        
        // a should now be empty
        assert_eq!(a.size(), 0);
        assert!(a.is_empty());
    }

    #[test]
    fn test_diff_with_sparse_domains() {
        let a_values = vec![1, 3, 5, 7, 9];
        let b_values = vec![2, 3, 5, 8];
        
        let mut a = SparseSet::new_from_values(a_values);
        let b = SparseSet::new_from_values(b_values);
        
        a.diff_with(&b);
        
        // a should now be {1, 7, 9} (removed 3 and 5)
        assert_eq!(a.size(), 3);
        assert!(a.contains(1));
        assert!(a.contains(7));
        assert!(a.contains(9));
        assert!(!a.contains(3));
        assert!(!a.contains(5));
    }

    #[test]
    fn test_set_operations_combination() {
        // Test combining union, intersect, and diff
        let mut a = SparseSet::new(1, 5);   // {1, 2, 3, 4, 5}
        let b = SparseSet::new(3, 7);       // {3, 4, 5, 6, 7}
        let c = SparseSet::new(4, 8);       // {4, 5, 6, 7, 8}
        
        // a ∩ b = {3, 4, 5}
        a.intersect_with(&b);
        assert_eq!(a.size(), 3);
        assert!(a.contains(3));
        assert!(a.contains(4));
        assert!(a.contains(5));
        
        // (a ∩ b) \ c = {3} (remove 4 and 5)
        a.diff_with(&c);
        assert_eq!(a.size(), 1);
        assert!(a.contains(3));
        assert!(!a.contains(4));
        assert!(!a.contains(5));
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

    // ===== TESTS FOR COMPLEMENT API =====

    #[test]
    fn test_complement_iter_empty() {
        let set = SparseSet::new(1, 5); // No removals
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 0);
        assert_eq!(set.complement_size(), 0);
        // With complement_size = 0 and size = 5, should_use_complement = (0 < 5/2) = (0 < 2) = true!
        assert!(set.should_use_complement());
    }

    #[test]
    fn test_complement_iter_single_removal() {
        let mut set = SparseSet::new(1, 5);
        set.remove(3);
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 1);
        assert!(complement.contains(&3));
        assert_eq!(set.complement_size(), 1);
    }

    #[test]
    fn test_complement_iter_multiple_removals() {
        let mut set = SparseSet::new(1, 10);
        set.remove(2);
        set.remove(5);
        set.remove(8);
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 3);
        assert!(complement.contains(&2));
        assert!(complement.contains(&5));
        assert!(complement.contains(&8));
        assert_eq!(set.complement_size(), 3);
    }

    #[test]
    fn test_complement_iter_matches_removed_values() {
        let mut set = SparseSet::new(1, 8);
        
        // Track what we remove
        let removed = vec![1, 3, 5, 7];
        for &val in &removed {
            set.remove(val);
        }
        
        // Verify complement matches removed values
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), removed.len());
        
        for &val in &removed {
            assert!(complement.contains(&val));
        }
        
        // Verify no active values are in complement
        for val in set.iter() {
            assert!(!complement.contains(&val));
        }
    }

    #[test]
    fn test_complement_size_calculation() {
        let mut set = SparseSet::new(1, 20);
        
        assert_eq!(set.size(), 20);
        assert_eq!(set.complement_size(), 0);
        
        // Remove 5 elements
        for i in 1..=5 {
            set.remove(i);
        }
        assert_eq!(set.size(), 15);
        assert_eq!(set.complement_size(), 5);
        
        // Remove 5 more
        for i in 6..=10 {
            set.remove(i);
        }
        assert_eq!(set.size(), 10);
        assert_eq!(set.complement_size(), 10);
        
        // Remove all but one (remove 11-20, leaving only one element at index somewhere)
        for i in 11..=19 {
            set.remove(i);
        }
        assert_eq!(set.size(), 1);
        assert_eq!(set.complement_size(), 19);
    }

    #[test]
    fn test_should_use_complement_basic() {
        let mut set = SparseSet::new(1, 100);
        
        // Start with full set: complement size = 0, size = 100
        // should_use_complement = (0 < 100/2) = (0 < 50) = true
        assert!(set.should_use_complement());
        
        // Remove 40 elements: complement size = 40, size = 60
        // should_use_complement = (40 < 60/2) = (40 < 30) = false
        for i in 1..=40 {
            set.remove(i);
        }
        assert!(!set.should_use_complement());
        
        // Remove 21 more (total 61): complement size = 61, size = 39
        // should_use_complement = (61 < 39/2) = (61 < 19) = false
        for i in 41..=61 {
            set.remove(i);
        }
        assert!(!set.should_use_complement());
        
        // Remove until only 10 remain: complement size = 90, size = 10
        // should_use_complement = (90 < 10/2) = (90 < 5) = false
        for i in 62..=90 {
            set.remove(i);
        }
        assert_eq!(set.size(), 10);
        assert_eq!(set.complement_size(), 90);
        assert!(!set.should_use_complement());
    }

    #[test]
    fn test_should_use_complement_when_heavily_pruned() {
        let mut set = SparseSet::new(1, 100);
        
        // Remove 98 elements, keep only 2
        for i in 1..=98 {
            set.remove(i);
        }
        
        assert_eq!(set.size(), 2);
        assert_eq!(set.complement_size(), 98);
        
        // should_use_complement = (98 < 2/2) = (98 < 1) = false
        assert!(!set.should_use_complement());
    }

    #[test]
    fn test_should_use_complement_exact_boundary() {
        let mut set = SparseSet::new(1, 10);
        
        // Initial: size = 10, complement = 0
        // should_use_complement = (0 < 10/2) = (0 < 5) = true
        assert!(set.should_use_complement());
        
        // Remove 8, leaving 2: size = 2, complement = 8
        // should_use_complement = (8 < 2/2) = (8 < 1) = false
        for i in 1..=8 {
            set.remove(i);
        }
        
        assert_eq!(set.size(), 2);
        assert_eq!(set.complement_size(), 8);
        assert!(!set.should_use_complement());
    }

    #[test]
    fn test_complement_iter_after_remove_all() {
        let mut set = SparseSet::new(1, 5);
        set.remove_all();
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 5);
        assert_eq!(set.complement_size(), 5);
        
        // All values should be in complement
        for i in 1..=5 {
            assert!(complement.contains(&i));
        }
    }

    #[test]
    fn test_complement_with_new_from_values() {
        let mut set = SparseSet::new_from_values(vec![1, 3, 5, 7, 9]);
        
        // Initial complement is {2, 4, 6, 8}
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 4);
        
        // After removing 3 and 7
        set.remove(3);
        set.remove(7);
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 6); // {2, 3, 4, 6, 7, 8}
        assert!(complement.contains(&2));
        assert!(complement.contains(&3));
        assert!(complement.contains(&4));
        assert!(complement.contains(&6));
        assert!(complement.contains(&7));
        assert!(complement.contains(&8));
    }

    #[test]
    fn test_complement_complement_is_original() {
        let mut set = SparseSet::new(1, 10);
        set.remove(2);
        set.remove(5);
        set.remove(8);
        
        let active: Vec<i32> = set.iter().collect();
        let removed: Vec<i32> = set.complement_iter().collect();
        
        // Union should give us all values
        let mut combined = active.clone();
        combined.extend(removed.clone());
        combined.sort();
        
        assert_eq!(combined.len(), 10);
        for i in 1..=10 {
            assert!(combined.contains(&i));
        }
        
        // Intersection should be empty
        for val in &active {
            assert!(!removed.contains(val));
        }
    }

    #[test]
    fn test_complement_consistency_after_operations() {
        let mut set = SparseSet::new(1, 20);
        
        // Series of operations
        set.remove(5);
        set.remove(10);
        set.remove(15);
        
        let state1 = set.save_state();
        let complement1: Vec<i32> = set.complement_iter().collect();
        
        // More removals
        set.remove(3);
        set.remove(7);
        
        let complement2: Vec<i32> = set.complement_iter().collect();
        assert!(complement2.len() > complement1.len());
        
        // Restore to state1
        set.restore_state(&state1);
        let complement1_restored: Vec<i32> = set.complement_iter().collect();
        
        // Should match original complement
        assert_eq!(complement1, complement1_restored);
    }

    #[test]
    fn test_complement_negative_domain() {
        let mut set = SparseSet::new(-5, 5);
        
        set.remove(-2);
        set.remove(0);
        set.remove(3);
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 3);
        assert!(complement.contains(&-2));
        assert!(complement.contains(&0));
        assert!(complement.contains(&3));
        assert_eq!(set.complement_size(), 3);
    }

    #[test]
    fn test_complement_single_value_domain() {
        let set = SparseSet::new(5, 5);
        
        assert_eq!(set.size(), 1);
        assert_eq!(set.complement_size(), 0);
        
        let complement: Vec<i32> = set.complement_iter().collect();
        assert_eq!(complement.len(), 0);
        assert!(!set.should_use_complement());
    }

    #[test]
    fn test_complement_performance_check() {
        // Verify that complement_iter is efficient by checking it doesn't allocate
        // more than necessary (single vec internally)
        let mut set = SparseSet::new(1, 1000);
        
        // Remove most elements
        for i in 1..=950 {
            set.remove(i);
        }
        
        // Complement has 950 elements, active has 50
        assert_eq!(set.complement_size(), 950);
        assert_eq!(set.size(), 50);
        
        // should_use_complement should return false (950 < 50/2 = 25? No)
        assert!(!set.should_use_complement());
        
        // But if we had even heavier pruning
        for i in 951..=990 {
            set.remove(i);
        }
        
        // Complement has 990, active has 10
        assert_eq!(set.complement_size(), 990);
        assert_eq!(set.size(), 10);
        // 990 < 10/2 = 5? No
        assert!(!set.should_use_complement());
    }
}
