/// Hybrid GAC implementation that automatically selects between SparseSet and BitSet
/// based on domain size for optimal performance
/// 
/// - Uses BitSetGAC for domains ≤128 values (ultra-fast bit operations)
/// - Uses SparseSetGAC for larger domains (memory efficient)
/// - Provides unified interface hiding the implementation details

use std::collections::HashMap;
use crate::constraints::gac::{SparseSetGAC, Variable};
use crate::constraints::gac_bitset::BitSetGAC;
use crate::variables::domain::bitset_domain::MAX_BITSET_DOMAIN_SIZE;

/// Hybrid GAC that automatically selects the best implementation
pub struct HybridGAC {
    /// Variables using BitSet implementation (≤128 domain size)
    bitset_vars: HashMap<Variable, ()>,
    /// Variables using SparseSet implementation (>128 domain size)
    sparseset_vars: HashMap<Variable, ()>,
    /// BitSet GAC instance for small domains
    bitset_gac: BitSetGAC,
    /// SparseSet GAC instance for large domains
    sparseset_gac: SparseSetGAC,
}

impl Default for HybridGAC {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridGAC {
    /// Create a new hybrid GAC instance
    pub fn new() -> Self {
        Self {
            bitset_vars: HashMap::new(),
            sparseset_vars: HashMap::new(),
            bitset_gac: BitSetGAC::new(),
            sparseset_gac: SparseSetGAC::new(),
        }
    }
    
    /// Add a variable with domain range, automatically selecting implementation
    pub fn add_variable(&mut self, var: Variable, min_val: i32, max_val: i32) -> Result<(), String> {
        if min_val > max_val {
            return Err(format!("Invalid range: min_val ({}) > max_val ({})", min_val, max_val));
        }
        
        let domain_size = (max_val - min_val + 1) as usize;
        
        if domain_size <= MAX_BITSET_DOMAIN_SIZE {
            // Use BitSet implementation for small domains
            self.bitset_gac.add_variable(var, min_val, max_val);
            self.bitset_vars.insert(var, ());
        } else {
            // Use SparseSet implementation for large domains
            self.sparseset_gac.add_variable(var, min_val, max_val);
            self.sparseset_vars.insert(var, ());
        }
        
        Ok(())
    }
    
    /// Add a variable with specific values, automatically selecting implementation
    pub fn add_variable_with_values(&mut self, var: Variable, values: Vec<i32>) -> Result<(), String> {
        if values.is_empty() {
            return Err("Cannot create domain from empty values".to_string());
        }
        
        let min_val = *values.iter().min().unwrap();
        let max_val = *values.iter().max().unwrap();
        let universe_size = (max_val - min_val + 1) as usize;
        
        if universe_size <= MAX_BITSET_DOMAIN_SIZE {
            // Use BitSet implementation
            self.bitset_gac.add_variable_with_values(var, values);
            self.bitset_vars.insert(var, ());
        } else {
            // Use SparseSet implementation - convert to range and add individual values
            self.sparseset_gac.add_variable(var, min_val, max_val);
            
            // Remove values not in the original set
            let all_values: std::collections::HashSet<i32> = values.into_iter().collect();
            for val in min_val..=max_val {
                if !all_values.contains(&val) {
                    self.sparseset_gac.remove_value(var, val);
                }
            }
            
            self.sparseset_vars.insert(var, ());
        }
        
        Ok(())
    }
    
    /// Remove a value from a variable's domain
    pub fn remove_value(&mut self, var: Variable, val: i32) -> bool {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.remove_value(var, val)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.remove_value(var, val)
        } else {
            false
        }
    }
    
    /// Fix a variable to a specific value
    pub fn assign_variable(&mut self, var: Variable, val: i32) -> bool {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.assign_variable(var, val)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.assign_variable(var, val)
        } else {
            false
        }
    }
    
    /// Remove values above threshold
    pub fn remove_above(&mut self, var: Variable, threshold: i32) -> bool {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.remove_above(var, threshold)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.remove_above(var, threshold)
        } else {
            false
        }
    }
    
    /// Remove values below threshold
    pub fn remove_below(&mut self, var: Variable, threshold: i32) -> bool {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.remove_below(var, threshold)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.remove_below(var, threshold)
        } else {
            false
        }
    }
    
    /// Get domain values
    pub fn get_domain_values(&self, var: Variable) -> Vec<i32> {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.get_domain_values(var)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.get_domain_values(var)
        } else {
            Vec::new()
        }
    }
    
    /// Get domain size
    pub fn domain_size(&self, var: Variable) -> usize {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.domain_size(var)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.domains.get(&var).map_or(0, |d| d.size())
        } else {
            0
        }
    }
    
    /// Check if variable is assigned
    pub fn is_assigned(&self, var: Variable) -> bool {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.is_assigned(var)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.domains.get(&var).map_or(false, |d| d.is_fixed())
        } else {
            false
        }
    }
    
    /// Get assigned value if variable is assigned
    pub fn assigned_value(&self, var: Variable) -> Option<i32> {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.assigned_value(var)
        } else if self.sparseset_vars.contains_key(&var) {
            let domain = self.sparseset_gac.domains.get(&var)?;
            if domain.is_fixed() {
                Some(domain.min())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Check if domain is inconsistent (empty)
    pub fn is_inconsistent(&self, var: Variable) -> bool {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.is_inconsistent(var)
        } else if self.sparseset_vars.contains_key(&var) {
            self.sparseset_gac.domains.get(&var).map_or(true, |d| d.is_empty())
        } else {
            true
        }
    }
    
    /// Get domain bounds
    pub fn get_bounds(&self, var: Variable) -> Option<(i32, i32)> {
        if self.bitset_vars.contains_key(&var) {
            self.bitset_gac.get_bounds(var)
        } else if self.sparseset_vars.contains_key(&var) {
            let domain = self.sparseset_gac.domains.get(&var)?;
            if !domain.is_empty() {
                Some((domain.min(), domain.max()))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Apply alldifferent constraint with hybrid optimization
    pub fn propagate_alldiff(&mut self, variables: &[Variable]) -> Result<bool, String> {
        if variables.is_empty() {
            return Ok(false);
        }
        
        // Separate variables by implementation
        let bitset_vars: Vec<Variable> = variables.iter()
            .filter(|var| self.bitset_vars.contains_key(var))
            .copied()
            .collect();
            
        let sparseset_vars: Vec<Variable> = variables.iter()
            .filter(|var| self.sparseset_vars.contains_key(var))
            .copied()
            .collect();
        
        let mut changed = false;
        
        // Propagate within each group using their optimized algorithms
        if !bitset_vars.is_empty() {
            changed |= self.bitset_gac.propagate_alldiff(&bitset_vars)?;
        }
        
        if !sparseset_vars.is_empty() {
            changed |= self.propagate_sparseset_alldiff(&sparseset_vars)?;
        }
        
        // Cross-propagation between groups
        if !bitset_vars.is_empty() && !sparseset_vars.is_empty() {
            changed |= self.cross_propagate_alldiff(&bitset_vars, &sparseset_vars)?;
        }
        
        Ok(changed)
    }
    
    /// Propagate alldiff for SparseSet variables
    fn propagate_sparseset_alldiff(&mut self, variables: &[Variable]) -> Result<bool, String> {
        let mut changed = false;
        
        // Basic alldiff: remove assigned values from other variables
        let assigned_values: Vec<(Variable, i32)> = variables
            .iter()
            .filter_map(|&var| {
                let domain = self.sparseset_gac.domains.get(&var)?;
                if domain.is_fixed() {
                    Some((var, domain.min()))
                } else {
                    None
                }
            })
            .collect();
        
        for (assigned_var, assigned_val) in assigned_values {
            for &var in variables {
                if var != assigned_var {
                    if self.sparseset_gac.remove_value(var, assigned_val) {
                        changed = true;
                        if self.sparseset_gac.domains.get(&var).unwrap().is_empty() {
                            return Err("Inconsistent domain after alldiff propagation".to_string());
                        }
                    }
                }
            }
        }
        
        Ok(changed)
    }
    
    /// Cross-propagate between BitSet and SparseSet variables
    fn cross_propagate_alldiff(&mut self, bitset_vars: &[Variable], sparseset_vars: &[Variable]) -> Result<bool, String> {
        let mut changed = false;
        
        // Get assigned values from BitSet variables
        let bitset_assigned: Vec<i32> = bitset_vars
            .iter()
            .filter_map(|&var| self.bitset_gac.assigned_value(var))
            .collect();
        
        // Remove these values from SparseSet variables
        for assigned_val in &bitset_assigned {
            for &var in sparseset_vars {
                if self.sparseset_gac.remove_value(var, *assigned_val) {
                    changed = true;
                    if self.sparseset_gac.domains.get(&var).unwrap().is_empty() {
                        return Err("Cross-propagation caused empty domain".to_string());
                    }
                }
            }
        }
        
        // Get assigned values from SparseSet variables
        let sparseset_assigned: Vec<i32> = sparseset_vars
            .iter()
            .filter_map(|&var| {
                let domain = self.sparseset_gac.domains.get(&var)?;
                if domain.is_fixed() {
                    Some(domain.min())
                } else {
                    None
                }
            })
            .collect();
        
        // Remove these values from BitSet variables
        for assigned_val in &sparseset_assigned {
            for &var in bitset_vars {
                if self.bitset_gac.remove_value(var, *assigned_val) {
                    changed = true;
                    if self.bitset_gac.is_inconsistent(var) {
                        return Err("Cross-propagation caused empty domain".to_string());
                    }
                }
            }
        }
        
        Ok(changed)
    }
    
    /// Get all variables
    pub fn variables(&self) -> impl Iterator<Item = Variable> + '_ {
        self.bitset_vars.keys().chain(self.sparseset_vars.keys()).copied()
    }
    
    /// Get statistics about implementation usage
    pub fn get_stats(&self) -> (usize, usize) {
        (self.bitset_vars.len(), self.sparseset_vars.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hybrid_gac_small_domains() {
        let mut gac = HybridGAC::new();
        
        // Small domains should use BitSet
        gac.add_variable(Variable(0), 1, 10).unwrap();
        gac.add_variable(Variable(1), 1, 10).unwrap();
        
        assert!(gac.bitset_vars.contains_key(&Variable(0)));
        assert!(gac.bitset_vars.contains_key(&Variable(1)));
        
        let (bitset_count, sparseset_count) = gac.get_stats();
        assert_eq!(bitset_count, 2);
        assert_eq!(sparseset_count, 0);
    }
    
    #[test]
    fn test_hybrid_gac_large_domains() {
        let mut gac = HybridGAC::new();
        
        // Large domains should use SparseSet (>128 values)
        gac.add_variable(Variable(0), 1, 150).unwrap();
        gac.add_variable(Variable(1), 1, 200).unwrap();
        
        assert!(gac.sparseset_vars.contains_key(&Variable(0)));
        assert!(gac.sparseset_vars.contains_key(&Variable(1)));
        
        let (bitset_count, sparseset_count) = gac.get_stats();
        assert_eq!(bitset_count, 0);
        assert_eq!(sparseset_count, 2);
    }
    
    #[test]
    fn test_hybrid_gac_mixed() {
        let mut gac = HybridGAC::new();
        
        // Mix of small and large domains
        gac.add_variable(Variable(0), 1, 64).unwrap();   // BitSet (≤128)
        gac.add_variable(Variable(1), 1, 150).unwrap();  // SparseSet (>128)
        
        assert!(gac.bitset_vars.contains_key(&Variable(0)));
        assert!(gac.sparseset_vars.contains_key(&Variable(1)));
        
        let (bitset_count, sparseset_count) = gac.get_stats();
        assert_eq!(bitset_count, 1);
        assert_eq!(sparseset_count, 1);
    }
    
    #[test]
    fn test_hybrid_alldiff_propagation() {
        let mut gac = HybridGAC::new();
        
        // Create mixed variables
        gac.add_variable(Variable(0), 1, 5).unwrap();    // BitSet
        gac.add_variable(Variable(1), 1, 5).unwrap();    // BitSet  
        gac.add_variable(Variable(2), 1, 100).unwrap();  // SparseSet
        
        // Assign one BitSet variable
        gac.assign_variable(Variable(0), 3);
        
        // Propagate alldiff
        let result = gac.propagate_alldiff(&[Variable(0), Variable(1), Variable(2)]);
        assert!(result.is_ok());
        
        // Other variables should not contain value 3
        let domain1 = gac.get_domain_values(Variable(1));
        let domain2 = gac.get_domain_values(Variable(2));
        
        assert!(!domain1.contains(&3));
        assert!(!domain2.contains(&3));
    }
}