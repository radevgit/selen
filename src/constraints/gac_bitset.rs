/// BitSet-based GAC implementation for small domains (≤128 values)
/// 
/// This implementation uses u128 bitsets instead of SparseSet for domains with ≤128 values.
/// For small domains, bit operations can be significantly faster than sparse set operations.
/// 
/// Key advantages over SparseSet for small domains:
/// - O(1) intersection, union, difference operations using bitwise AND/OR/XOR
/// - O(1) cardinality using population count (u128::count_ones())
/// - O(1) membership testing using bit masks
/// - Better cache locality due to compact representation
/// - Vectorizable operations on modern CPUs

use std::collections::HashMap;
use crate::constraints::gac::Variable;
use crate::variables::domain::bitset_domain::BitSetDomain;

/// Generate combinations of k elements from a slice
fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.len() < k {
        return vec![];
    }
    if k == 1 {
        return items.iter().map(|item| vec![item.clone()]).collect();
    }
    
    let mut result = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let rest = &items[i + 1..];
        for mut sub_combination in combinations(rest, k - 1) {
            let mut combination = vec![item.clone()];
            combination.append(&mut sub_combination);
            result.push(combination);
        }
    }
    result
}

/// BitSet-based GAC implementation for small domains
pub struct BitSetGAC {
    /// Variable domains using BitSetDomain for ultra-fast operations
    pub domains: HashMap<Variable, BitSetDomain>,
    /// Track if domains have changed since last consistency check
    domains_changed: bool,
}

impl Default for BitSetGAC {
    fn default() -> Self {
        Self::new()
    }
}

impl BitSetGAC {
    /// Create a new BitSet-based GAC instance
    pub fn new() -> Self {
        Self {
            domains: HashMap::with_capacity(32),
            domains_changed: false,
        }
    }
    
    /// Add a variable with its initial domain range
    pub fn add_variable(&mut self, var: Variable, min_val: i32, max_val: i32) {
        let domain = BitSetDomain::new(min_val, max_val);
        self.domains.insert(var, domain);
        self.domains_changed = true;
    }
    
    /// Add a variable with specific domain values
    pub fn add_variable_with_values(&mut self, var: Variable, values: Vec<i32>) {
        let domain = BitSetDomain::new_from_values(values);
        self.domains.insert(var, domain);
        self.domains_changed = true;
    }
    
    /// Remove a value from a variable's domain
    pub fn remove_value(&mut self, var: Variable, val: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            let changed = domain.remove(val);
            if changed {
                self.domains_changed = true;
            }
            changed
        } else {
            false
        }
    }
    
    /// Fix a variable to a specific value (remove all others)
    pub fn assign_variable(&mut self, var: Variable, val: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            let old_size = domain.size();
            domain.remove_all_but(val);
            let changed = domain.size() != old_size;
            if changed {
                self.domains_changed = true;
            }
            changed
        } else {
            false
        }
    }
    
    /// Remove values above a threshold
    pub fn remove_above(&mut self, var: Variable, threshold: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            let old_size = domain.size();
            domain.remove_above(threshold);
            let changed = domain.size() != old_size;
            if changed {
                self.domains_changed = true;
            }
            changed
        } else {
            false
        }
    }
    
    /// Remove values below a threshold
    pub fn remove_below(&mut self, var: Variable, threshold: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            let old_size = domain.size();
            domain.remove_below(threshold);
            let changed = domain.size() != old_size;
            if changed {
                self.domains_changed = true;
            }
            changed
        } else {
            false
        }
    }
    
    /// Get the current domain values for a variable
    pub fn get_domain_values(&self, var: Variable) -> Vec<i32> {
        if let Some(domain) = self.domains.get(&var) {
            domain.to_vec()
        } else {
            Vec::new()
        }
    }
    
    /// Get domain size
    pub fn domain_size(&self, var: Variable) -> usize {
        self.domains.get(&var).map_or(0, |d| d.size())
    }
    
    /// Check if variable is assigned (domain size = 1)
    pub fn is_assigned(&self, var: Variable) -> bool {
        self.domains.get(&var).map_or(false, |d| d.is_fixed())
    }
    
    /// Get assigned value if variable is assigned
    pub fn assigned_value(&self, var: Variable) -> Option<i32> {
        let domain = self.domains.get(&var)?;
        if domain.is_fixed() {
            domain.fixed_value()
        } else {
            None
        }
    }
    
    /// Check if domain is empty (inconsistent)
    pub fn is_inconsistent(&self, var: Variable) -> bool {
        self.domains.get(&var).map_or(true, |d| d.is_empty())
    }
    
    /// Get domain bounds efficiently using bit operations
    pub fn get_bounds(&self, var: Variable) -> Option<(i32, i32)> {
        let domain = self.domains.get(&var)?;
        if let (Some(min), Some(max)) = (domain.min(), domain.max()) {
            Some((min, max))
        } else {
            None
        }
    }
    
    /// Apply alldifferent constraint using ultra-fast bit operations
    /// This is where BitSet really shines compared to SparseSet
    pub fn propagate_alldiff(&mut self, variables: &[Variable]) -> Result<bool, String> {
        if variables.len() <= 1 {
            return Ok(false); // Nothing to propagate
        }
        
        let mut changed = false;
        
        // For each assigned variable, remove its value from all other variables
        let assigned_values: Vec<(Variable, i32)> = variables
            .iter()
            .filter_map(|&var| {
                let domain = self.domains.get(&var)?;
                if domain.is_fixed() {
                    Some((var, domain.fixed_value()?))
                } else {
                    None
                }
            })
            .collect();
        
        for (assigned_var, assigned_val) in assigned_values {
            for &var in variables {
                if var != assigned_var {
                    if self.remove_value(var, assigned_val) {
                        changed = true;
                        
                        // Check for failure
                        if self.is_inconsistent(var) {
                            return Err("Inconsistent domain after alldiff propagation".to_string());
                        }
                    }
                }
            }
        }
        
        // Advanced propagation: Hall sets using bit operations
        // If we have n variables with domains that union to exactly n values,
        // then those variables must take those values (Hall's theorem)
        changed |= self.propagate_hall_sets(variables)?;
        
        Ok(changed)
    }
    
    /// Propagate Hall sets using efficient bit operations (optimized for practical use)
    fn propagate_hall_sets(&mut self, variables: &[Variable]) -> Result<bool, String> {
        let mut changed = false;
        
        // Only do Hall set propagation for very small sets to avoid exponential blowup
        if variables.len() <= 6 { // Much more conservative limit
            for subset_size in 2..=variables.len().min(4) { // Only check subsets of size 2-4
                // Generate combinations of the given subset_size
                let indices: Vec<usize> = (0..variables.len()).collect();
                for subset_indices in combinations(&indices, subset_size) {
                    let subset: Vec<Variable> = subset_indices.iter()
                        .map(|&i| variables[i])
                        .collect();
                    
                    // Calculate union of domains using bit operations
                    let mut union_mask = 0u128;
                    let mut compatible = true;
                    let mut min_val = None;
                    
                    for &var in &subset {
                        if let Some(domain) = self.domains.get(&var) {
                            if let Some(existing_min) = min_val {
                                if domain.get_min_val() != existing_min {
                                    compatible = false;
                                    break;
                                }
                            } else {
                                min_val = Some(domain.get_min_val());
                            }
                            union_mask |= domain.get_mask();
                        }
                    }
                    
                    if !compatible {
                        continue;
                    }
                    
                    let union_size = union_mask.count_ones() as usize;
                    
                    // Hall's theorem: if |subset| = |union of domains|, 
                    // then values in union can only be assigned to variables in subset
                    if subset.len() == union_size {
                        // Remove union values from all variables NOT in subset
                        for &var in variables {
                            if !subset.contains(&var) {
                                if let Some(domain) = self.domains.get_mut(&var) {
                                    if domain.remove_by_mask(union_mask) {
                                        changed = true;
                                        if domain.is_empty() {
                                            return Err("Hall set propagation caused empty domain".to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(changed)
    }
    
    /// Get all variables
    pub fn variables(&self) -> impl Iterator<Item = Variable> + '_ {
        self.domains.keys().copied()
    }
    
    /// Check if any domains have changed since last check
    pub fn domains_changed(&self) -> bool {
        self.domains_changed
    }
    
    /// Reset the domains changed flag
    pub fn reset_changed_flag(&mut self) {
        self.domains_changed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bitset_domain_basic() {
        let domain = BitSetDomain::new(1, 5);
        assert_eq!(domain.size(), 5);
        assert!(domain.contains(1));
        assert!(domain.contains(5));
        assert!(!domain.contains(0));
        assert!(!domain.contains(6));
    }
    
    #[test]
    fn test_bitset_domain_operations() {
        let mut domain = BitSetDomain::new(1, 10);
        
        // Test removal
        assert!(domain.remove(5));
        assert!(!domain.contains(5));
        assert_eq!(domain.size(), 9);
        
        // Test bounds
        assert_eq!(domain.min(), Some(1));
        assert_eq!(domain.max(), Some(10));
        
        // Test remove_above
        domain.remove_above(7);
        assert!(!domain.contains(8));
        assert!(!domain.contains(9));
        assert!(!domain.contains(10));
        assert!(domain.contains(7));
        
        // Test remove_below
        domain.remove_below(3);
        assert!(!domain.contains(1));
        assert!(!domain.contains(2));
        assert!(domain.contains(3));
    }
    
    #[test]
    fn test_bitset_gac_basic() {
        let mut gac = BitSetGAC::new();
        
        // Add variables
        gac.add_variable(Variable(0), 1, 3);
        gac.add_variable(Variable(1), 1, 3);
        gac.add_variable(Variable(2), 1, 3);
        
        // Test alldiff propagation
        gac.assign_variable(Variable(0), 1);
        let result = gac.propagate_alldiff(&[Variable(0), Variable(1), Variable(2)]);
        assert!(result.is_ok());
        
        // Variable 1 and 2 should no longer contain value 1
        assert!(!gac.domains.get(&Variable(1)).unwrap().contains(1));
        assert!(!gac.domains.get(&Variable(2)).unwrap().contains(1));
    }
    
    #[test]
    fn test_bitset_intersection_union() {
        let domain1 = BitSetDomain::new_from_values(vec![1, 2, 3, 4]);
        let domain2 = BitSetDomain::new_from_values(vec![3, 4, 5, 6]);
        
        // Different ranges should not be compatible for union_mask
        assert!(domain1.union_mask(&domain2).is_none());
        
        // Create domains with same range
        let domain1 = BitSetDomain::new(1, 6);
        let mut domain2 = BitSetDomain::new(1, 6);
        
        // Remove some values to create different sets
        domain2.remove(1);
        domain2.remove(2);
        
        // Test union mask
        let union_mask = domain1.union_mask(&domain2).unwrap();
        let expected_size = (domain1.get_mask() | domain2.get_mask()).count_ones() as usize;
        assert_eq!(union_mask.count_ones() as usize, expected_size);
        
        // Test intersection using existing methods
        let mut intersection = domain1.clone();
        let _ = intersection.intersect_with(&domain2);
        assert_eq!(intersection.size(), 4); // Should contain 3, 4, 5, 6
        assert!(!intersection.contains(1));
        assert!(!intersection.contains(2));
        assert!(intersection.contains(3));
    }
}