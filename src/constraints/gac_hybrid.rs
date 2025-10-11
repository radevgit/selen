/// Hybrid GAC implementation that automatically selects between SparseSet and BitSet
/// based on domain size for optimal performance
/// 
/// This module serves as the central hub for all GAC operations, providing:
/// - Common GAC types (Variable, Value, BipartiteGraph, Matching, GACStats)
/// - Automatic selection between BitSet (≤128 values) and SparseSet (>128 values)
/// - Unified interface hiding implementation details
/// - Efficient domain operations for both small and large domains

use std::collections::HashMap;
use crate::constraints::gac_sparseset::SparseSetGAC;
use crate::constraints::gac_bitset::BitSetGAC;
use crate::variables::domain::bitset_domain::{BitSetDomain, MAX_BITSET_DOMAIN_SIZE};
use crate::variables::domain::sparse_set::SparseSet;

/// Represents a variable in the bipartite graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable(pub usize);

/// Represents a value in the bipartite graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub i32);

/// Statistics about GAC state
#[derive(Debug, Clone)]
pub struct GACStats {
    pub total_variables: usize,
    pub assigned_variables: usize,
    pub total_domain_size: usize,
    pub min_domain_size: usize,
    pub max_domain_size: usize,
}

/// Domain abstraction that works efficiently with both BitSet and SparseSet
#[derive(Debug, Clone)]
pub enum DomainType {
    /// BitSet domain for small domains (≤128 values) - ultra-fast bit operations
    BitSet(BitSetDomain),
    /// SparseSet domain for larger domains - memory efficient
    SparseSet(SparseSet),
}

impl DomainType {
    /// Create domain automatically selecting best representation
    pub fn new(min_val: i32, max_val: i32) -> Self {
        let domain_size = (max_val - min_val + 1) as usize;
        if domain_size <= MAX_BITSET_DOMAIN_SIZE {
            Self::BitSet(BitSetDomain::new(min_val, max_val))
        } else {
            Self::SparseSet(SparseSet::new(min_val, max_val))
        }
    }
    
    /// Create domain from values automatically selecting best representation
    pub fn new_from_values(values: Vec<i32>) -> Self {
        if values.is_empty() {
            return Self::SparseSet(SparseSet::new_from_values(values));
        }
        
        let min_val = *values.iter().min().unwrap();
        let max_val = *values.iter().max().unwrap();
        let range_size = (max_val - min_val + 1) as usize;
        
        if range_size <= MAX_BITSET_DOMAIN_SIZE && values.len() > range_size / 2 {
            // Use BitSet if domain is dense and small
            Self::BitSet(BitSetDomain::new_from_values(values))
        } else {
            // Use SparseSet for sparse or large domains
            Self::SparseSet(SparseSet::new_from_values(values))
        }
    }
    
    /// Check if domain contains a value
    pub fn contains(&self, val: i32) -> bool {
        match self {
            Self::BitSet(bitset) => bitset.contains(val),
            Self::SparseSet(sparse) => sparse.contains(val),
        }
    }
    
    /// Ultra-fast contains check - optimized for hot paths
    #[inline(always)]
    pub fn contains_fast(&self, val: i32) -> bool {
        match self {
            // BitSet: single bit check - fastest possible
            Self::BitSet(bitset) => bitset.contains(val),
            // SparseSet: O(1) lookup in dense array
            Self::SparseSet(sparse) => sparse.contains(val),
        }
    }
    
    /// Check if domain intersects with a range efficiently
    pub fn intersects_range(&self, min_val: i32, max_val: i32) -> bool {
        match self {
            Self::BitSet(bitset) => {
                // Simple range check using existing contains method
                for val in min_val..=max_val {
                    if bitset.contains(val) {
                        return true;
                    }
                }
                false
            }
            Self::SparseSet(sparse) => {
                // Iterate through sparse values in range
                sparse.iter().any(|val| val >= min_val && val <= max_val)
            }
        }
    }
    
    /// Get domain size
    pub fn size(&self) -> usize {
        match self {
            Self::BitSet(bitset) => bitset.size(),
            Self::SparseSet(sparse) => sparse.size(),
        }
    }
    
    /// Check if domain is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::BitSet(bitset) => bitset.is_empty(),
            Self::SparseSet(sparse) => sparse.is_empty(),
        }
    }
    
    /// Check if domain has exactly one value
    pub fn is_fixed(&self) -> bool {
        match self {
            Self::BitSet(bitset) => bitset.is_fixed(),
            Self::SparseSet(sparse) => sparse.is_fixed(),
        }
    }
    
    /// Get minimum value
    pub fn min(&self) -> i32 {
        match self {
            Self::BitSet(bitset) => bitset.min().unwrap_or(0),
            Self::SparseSet(sparse) => sparse.min(),
        }
    }
    
    /// Get maximum value
    pub fn max(&self) -> i32 {
        match self {
            Self::BitSet(bitset) => bitset.max().unwrap_or(0),
            Self::SparseSet(sparse) => sparse.max(),
        }
    }
    
    /// Remove a value
    pub fn remove(&mut self, val: i32) -> bool {
        match self {
            Self::BitSet(bitset) => bitset.remove(val),
            Self::SparseSet(sparse) => sparse.remove(val),
        }
    }
    
    /// Get iterator over domain values
    pub fn iter(&self) -> Box<dyn Iterator<Item = i32> + '_> {
        match self {
            Self::BitSet(bitset) => Box::new(bitset.iter()),
            Self::SparseSet(sparse) => Box::new(sparse.iter()),
        }
    }
}

/// A bipartite graph between variables and their possible values
/// Automatically uses the most efficient domain representation
#[derive(Debug, Clone)]
pub struct BipartiteGraph {
    /// Variables and their domains using optimal representation
    pub var_domains: HashMap<Variable, DomainType>,
    /// Values and which variables can take them
    pub value_vars: HashMap<Value, Vec<Variable>>,
}

impl Default for BipartiteGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl BipartiteGraph {
    pub fn new() -> Self {
        Self::with_capacity(16)
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            var_domains: HashMap::with_capacity(capacity),
            value_vars: HashMap::with_capacity(capacity * 4),
        }
    }
    
    /// Add variable with optimal domain representation
    pub fn add_variable(&mut self, var: Variable, domain: Vec<i32>) {
        let optimal_domain = DomainType::new_from_values(domain.clone());
        self.var_domains.insert(var, optimal_domain);
        
        // Build reverse mapping efficiently
        for value in domain {
            let val = Value(value);
            self.value_vars.entry(val).or_default().push(var);
        }
    }
    
    /// Add variable with range using optimal domain representation
    pub fn add_variable_range(&mut self, var: Variable, min_val: i32, max_val: i32) {
        let optimal_domain = DomainType::new(min_val, max_val);
        self.var_domains.insert(var, optimal_domain);
        
        // Build reverse mapping for the range
        for val in min_val..=max_val {
            self.value_vars.entry(Value(val)).or_default().push(var);
        }
    }
    
    /// Remove a value from a variable's domain
    pub fn remove_value(&mut self, var: Variable, val: Value) -> bool {
        let mut changed = false;
        
        // Remove from var_domains using optimal domain
        if let Some(domain) = self.var_domains.get_mut(&var) {
            if domain.remove(val.0) {
                changed = true;
            }
        }
        
        // Update reverse mapping
        if let Some(vars) = self.value_vars.get_mut(&val) {
            if let Some(pos) = vars.iter().position(|&v| v == var) {
                vars.swap_remove(pos);
                changed = true;
            }
        }
        
        changed
    }
    
    /// Get all variables
    pub fn variables(&self) -> impl Iterator<Item = Variable> + '_ {
        self.var_domains.keys().copied()
    }
    
    /// Get all values
    pub fn values(&self) -> impl Iterator<Item = Value> + '_ {
        self.value_vars.keys().copied()
    }
    
    /// Get domain size efficiently
    pub fn domain_size(&self, var: Variable) -> usize {
        self.var_domains.get(&var).map_or(0, |d| d.size())
    }
    
    /// Check if variable contains a value efficiently
    pub fn contains_value(&self, var: Variable, val: Value) -> bool {
        self.var_domains.get(&var).map_or(false, |d| d.contains_fast(val.0))
    }
    
    /// Get domain values as Vec (when conversion is needed)
    pub fn domain(&self, var: Variable) -> Vec<Value> {
        if let Some(domain) = self.var_domains.get(&var) {
            domain.iter().map(|val| Value(val)).collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get domain values as iterator (most efficient)
    pub fn domain_iter(&self, var: Variable) -> Box<dyn Iterator<Item = Value> + '_> {
        if let Some(domain) = self.var_domains.get(&var) {
            Box::new(domain.iter().map(|val| Value(val)))
        } else {
            Box::new(std::iter::empty())
        }
    }
    
    /// Get minimum domain value efficiently
    pub fn domain_min(&self, var: Variable) -> Option<Value> {
        self.var_domains.get(&var)
            .filter(|d| !d.is_empty())
            .map(|d| Value(d.min()))
    }
    
    /// Get maximum domain value efficiently
    pub fn domain_max(&self, var: Variable) -> Option<Value> {
        self.var_domains.get(&var)
            .filter(|d| !d.is_empty())
            .map(|d| Value(d.max()))
    }
    
    /// Check if variable is assigned (domain size = 1)
    pub fn is_assigned(&self, var: Variable) -> bool {
        self.var_domains.get(&var).map_or(false, |d| d.is_fixed())
    }
    
    /// Get assigned value if variable is assigned
    pub fn assigned_value(&self, var: Variable) -> Option<Value> {
        let domain = self.var_domains.get(&var)?;
        if domain.is_fixed() {
            Some(Value(domain.min()))
        } else {
            None
        }
    }
    
    /// Ultra-fast domain intersection using optimal operations
    pub fn intersect_domains(&self, var1: Variable, var2: Variable) -> Vec<Value> {
        let (domain1, domain2) = match (self.var_domains.get(&var1), self.var_domains.get(&var2)) {
            (Some(d1), Some(d2)) => (d1, d2),
            _ => return Vec::new(),
        };
        
        match (domain1, domain2) {
            // BitSet ∩ BitSet - ultra-fast bitwise AND  
            (DomainType::BitSet(b1), DomainType::BitSet(b2)) => {
                // Manual intersection using contains checks
                b1.iter()
                    .filter(|&val| b2.contains(val))
                    .map(Value)
                    .collect()
            }
            // SparseSet ∩ SparseSet - efficient iteration over smaller set
            (DomainType::SparseSet(s1), DomainType::SparseSet(s2)) => {
                let (smaller, larger) = if s1.size() <= s2.size() { (s1, s2) } else { (s2, s1) };
                smaller.iter()
                    .filter(|&val| larger.contains(val))
                    .map(Value)
                    .collect()
            }
            // Mixed types - use general iterator approach
            _ => {
                let (smaller, larger) = if domain1.size() <= domain2.size() { 
                    (domain1, domain2) 
                } else { 
                    (domain2, domain1) 
                };
                smaller.iter()
                    .filter(|&val| larger.contains(val))
                    .map(Value)
                    .collect()
            }
        }
    }
    
    /// Ultra-fast domain union using optimal operations
    pub fn union_domains(&self, var1: Variable, var2: Variable) -> Vec<Value> {
        let (domain1, domain2) = match (self.var_domains.get(&var1), self.var_domains.get(&var2)) {
            (Some(d1), Some(d2)) => (d1, d2),
            _ => return Vec::new(),
        };
        
        match (domain1, domain2) {
            // BitSet ∪ BitSet - combine all values
            (DomainType::BitSet(b1), DomainType::BitSet(b2)) => {
                let mut result: Vec<Value> = b1.iter().map(Value).collect();
                result.extend(b2.iter().filter(|&val| !b1.contains(val)).map(Value));
                result
            }
            // For other combinations, use efficient set-like operations
            _ => {
                use std::collections::HashSet;
                let mut result_set = HashSet::with_capacity(domain1.size() + domain2.size());
                
                for val in domain1.iter() {
                    result_set.insert(Value(val));
                }
                for val in domain2.iter() {
                    result_set.insert(Value(val));
                }
                
                result_set.into_iter().collect()
            }
        }
    }
    
    /// Check if two domains overlap efficiently
    pub fn domains_overlap(&self, var1: Variable, var2: Variable) -> bool {
        let (domain1, domain2) = match (self.var_domains.get(&var1), self.var_domains.get(&var2)) {
            (Some(d1), Some(d2)) => (d1, d2),
            _ => return false,
        };
        
        match (domain1, domain2) {
            // BitSet overlap - check any common values
            (DomainType::BitSet(b1), DomainType::BitSet(b2)) => {
                b1.iter().any(|val| b2.contains(val))
            }
            // SparseSet overlap - iterate over smaller domain
            (DomainType::SparseSet(s1), DomainType::SparseSet(s2)) => {
                let (smaller, larger) = if s1.size() <= s2.size() { (s1, s2) } else { (s2, s1) };
                smaller.iter().any(|val| larger.contains(val))
            }
            // Mixed types - use general approach
            _ => {
                let (smaller, larger) = if domain1.size() <= domain2.size() { 
                    (domain1, domain2) 
                } else { 
                    (domain2, domain1) 
                };
                smaller.iter().any(|val| larger.contains(val))
            }
        }
    }
    
    /// Efficiently remove multiple values in one operation
    pub fn remove_values_bulk(&mut self, var: Variable, values_to_remove: &[i32]) -> bool {
        if let Some(domain) = self.var_domains.get_mut(&var) {
            let mut changed = false;
            
            match domain {
                // BitSet bulk removal - can be very efficient
                DomainType::BitSet(bitset) => {
                    for &val in values_to_remove {
                        if bitset.remove(val) {
                            changed = true;
                            // Update reverse mapping
                            if let Some(vars) = self.value_vars.get_mut(&Value(val)) {
                                if let Some(pos) = vars.iter().position(|&v| v == var) {
                                    vars.swap_remove(pos);
                                }
                            }
                        }
                    }
                }
                // SparseSet bulk removal
                DomainType::SparseSet(sparse) => {
                    for &val in values_to_remove {
                        if sparse.remove(val) {
                            changed = true;
                            // Update reverse mapping
                            if let Some(vars) = self.value_vars.get_mut(&Value(val)) {
                                if let Some(pos) = vars.iter().position(|&v| v == var) {
                                    vars.swap_remove(pos);
                                }
                            }
                        }
                    }
                }
            }
            
            changed
        } else {
            false
        }
    }
}

/// Maximum bipartite matching using augmenting paths
/// Works efficiently with both BitSet and SparseSet domains
#[derive(Debug)]
pub struct Matching {
    /// Variable to value mapping
    pub var_to_val: HashMap<Variable, Value>,
    /// Value to variable mapping
    pub val_to_var: HashMap<Value, Variable>,
}

impl Default for Matching {
    fn default() -> Self {
        Self::new()
    }
}

impl Matching {
    pub fn new() -> Self {
        Self {
            var_to_val: HashMap::with_capacity(16),
            val_to_var: HashMap::with_capacity(16),
        }
    }
    
    /// Find maximum matching using augmenting paths with optimal domain iteration
    pub fn find_maximum_matching(graph: &BipartiteGraph) -> Self {
        let mut matching = Self::new();
        
        // Start with greedy matching for assigned variables
        for var in graph.variables() {
            if graph.is_assigned(var) {
                if let Some(val) = graph.assigned_value(var) {
                    if !matching.val_to_var.contains_key(&val) {
                        matching.add_edge(var, val);
                    }
                }
            }
        }
        
        // Try to extend matching using augmenting paths
        for var in graph.variables() {
            if !matching.var_to_val.contains_key(&var) {
                Self::find_augmenting_path(graph, &mut matching, var);
            }
        }
        
        matching
    }
    
    /// Add an edge to the matching
    fn add_edge(&mut self, var: Variable, val: Value) {
        self.var_to_val.insert(var, val);
        self.val_to_var.insert(val, var);
    }
    
    /// Remove an edge from the matching
    fn remove_edge(&mut self, var: Variable, val: Value) {
        self.var_to_val.remove(&var);
        self.val_to_var.remove(&val);
    }
    
    /// Find augmenting path using BFS with optimized visit tracking
    fn find_augmenting_path(graph: &BipartiteGraph, matching: &mut Self, start_var: Variable) -> bool {
        // Use optimized visit tracking based on problem size
        let num_vars = graph.var_domains.len();
        let num_vals = graph.value_vars.len();
        
        // For small problems, use bit-based tracking for ultra-fast operations
        if num_vars <= 64 && num_vals <= 128 {
            Self::find_augmenting_path_bitset(graph, matching, start_var)
        } else {
            // For larger problems, use efficient HashMap tracking
            Self::find_augmenting_path_hashmap(graph, matching, start_var)
        }
    }
    
    /// Ultra-fast augmenting path for small problems using bitset tracking
    fn find_augmenting_path_bitset(graph: &BipartiteGraph, matching: &mut Self, start_var: Variable) -> bool {
        use std::collections::VecDeque;
        
        let mut queue: VecDeque<Variable> = VecDeque::new();
        let mut visited_vars = 0u64; // Bitset for variables
        let mut visited_vals = 0u128; // Bitset for values (up to 128 values)
        let mut parent_var: HashMap<Variable, Value> = HashMap::with_capacity(16);
        let mut parent_val: HashMap<Value, Variable> = HashMap::with_capacity(16);
        
        queue.push_back(start_var);
        visited_vars |= 1u64 << start_var.0;
        
        while let Some(current_var) = queue.pop_front() {
            // Use efficient domain iteration with bitset checking
            for val_int in graph.domain_iter(current_var) {
                let val_bit = 1u128 << (val_int.0 as usize);
                if (visited_vals & val_bit) != 0 {
                    continue;
                }
                
                visited_vals |= val_bit;
                parent_val.insert(val_int, current_var);
                
                // If this value is free, we found an augmenting path
                if !matching.val_to_var.contains_key(&val_int) {
                    Self::apply_augmenting_path(matching, start_var, val_int, &parent_var, &parent_val);
                    return true;
                }
                
                // Otherwise, follow the matching edge
                if let Some(&matched_var) = matching.val_to_var.get(&val_int) {
                    let var_bit = 1u64 << matched_var.0;
                    if (visited_vars & var_bit) == 0 {
                        visited_vars |= var_bit;
                        parent_var.insert(matched_var, val_int);
                        queue.push_back(matched_var);
                    }
                }
            }
        }
        
        false
    }
    
    /// Efficient augmenting path for larger problems using HashMap tracking
    fn find_augmenting_path_hashmap(graph: &BipartiteGraph, matching: &mut Self, start_var: Variable) -> bool {
        use std::collections::{VecDeque, HashSet};
        
        let mut queue = VecDeque::new();
        let mut visited_vars = HashSet::with_capacity(32);
        let mut visited_vals = HashSet::with_capacity(64);
        let mut parent_var: HashMap<Variable, Value> = HashMap::with_capacity(32);
        let mut parent_val: HashMap<Value, Variable> = HashMap::with_capacity(32);
        
        queue.push_back(start_var);
        visited_vars.insert(start_var);
        
        while let Some(current_var) = queue.pop_front() {
            // Use efficient domain iteration
            for val_int in graph.domain_iter(current_var) {
                if visited_vals.contains(&val_int) {
                    continue;
                }
                
                visited_vals.insert(val_int);
                parent_val.insert(val_int, current_var);
                
                // If this value is free, we found an augmenting path
                if !matching.val_to_var.contains_key(&val_int) {
                    Self::apply_augmenting_path(matching, start_var, val_int, &parent_var, &parent_val);
                    return true;
                }
                
                // Otherwise, follow the matching edge
                if let Some(&matched_var) = matching.val_to_var.get(&val_int) {
                    if !visited_vars.contains(&matched_var) {
                        visited_vars.insert(matched_var);
                        parent_var.insert(matched_var, val_int);
                        queue.push_back(matched_var);
                    }
                }
            }
        }
        
        false
    }
    
    /// Apply augmenting path to improve matching
    fn apply_augmenting_path(
        matching: &mut Self,
        start_var: Variable,
        end_val: Value,
        parent_var: &HashMap<Variable, Value>,
        parent_val: &HashMap<Value, Variable>,
    ) {
        let mut current_val = end_val;
        let mut current_var = start_var;
        
        // Follow the path backwards and flip edges
        loop {
            matching.add_edge(current_var, current_val);
            
            if let Some(&prev_val) = parent_var.get(&current_var) {
                if let Some(&prev_var) = parent_val.get(&prev_val) {
                    matching.remove_edge(prev_var, prev_val);
                    current_var = prev_var;
                    current_val = prev_val;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    /// Check if matching is complete (all variables matched)
    pub fn is_complete(&self, graph: &BipartiteGraph) -> bool {
        self.var_to_val.len() == graph.var_domains.len()
    }
}

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
            // Use SparseSet implementation - efficient values-only approach
            self.sparseset_gac.add_variable_with_values(var, values);
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
            let bounds = self.bitset_gac.get_bounds(var);

            bounds
        } else if self.sparseset_vars.contains_key(&var) {
            let domain = self.sparseset_gac.domains.get(&var)?;
            if !domain.is_empty() {
                let bounds = Some((domain.min(), domain.max()));

                bounds
            } else {

                None
            }
        } else {
            None
        }
    }
    
    /// Apply alldifferent constraint with hybrid optimization
    /// Returns (changed, consistent) where changed indicates if domains were modified
    /// and consistent indicates if the constraint is still satisfiable
    pub fn propagate_alldiff(&mut self, variables: &[Variable]) -> (bool, bool) {
        if variables.is_empty() {
            return (false, true); // No change, still consistent
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
            let (bitset_changed, bitset_consistent) = self.bitset_gac.propagate_alldiff(&bitset_vars);
            if !bitset_consistent {
                return (changed, false); // BitSet propagation detected inconsistency
            }
            changed |= bitset_changed;
        }
        
        if !sparseset_vars.is_empty() {
            let (sparse_changed, sparse_consistent) = self.propagate_sparseset_alldiff(&sparseset_vars);
            if !sparse_consistent {
                return (changed, false); // SparseSet propagation detected inconsistency
            }
            changed |= sparse_changed;
        }
        
        // Cross-propagation between groups
        if !bitset_vars.is_empty() && !sparseset_vars.is_empty() {
            let (cross_changed, cross_consistent) = self.cross_propagate_alldiff(&bitset_vars, &sparseset_vars);
            if !cross_consistent {
                return (changed, false); // Cross-propagation detected inconsistency
            }
            changed |= cross_changed;
        }
        
        (changed, true)
    }
    
    /// Propagate alldiff for SparseSet variables
    /// Returns (changed, consistent) tuple
    fn propagate_sparseset_alldiff(&mut self, variables: &[Variable]) -> (bool, bool) {
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
                            return (changed, false); // Domain became empty - inconsistent
                        }
                    }
                }
            }
        }
        
        (changed, true)
    }
    
    /// Cross-propagate between BitSet and SparseSet variables
    /// Returns (changed, consistent) tuple
    fn cross_propagate_alldiff(&mut self, bitset_vars: &[Variable], sparseset_vars: &[Variable]) -> (bool, bool) {
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
                        return (changed, false); // Domain became empty - inconsistent
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
                        return (changed, false); // Domain became empty - inconsistent
                    }
                }
            }
        }
        
        (changed, true)
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
        let (changed, consistent) = gac.propagate_alldiff(&[Variable(0), Variable(1), Variable(2)]);
        assert!(consistent);
        assert!(changed);
        
        // Other variables should not contain value 3
        let domain1 = gac.get_domain_values(Variable(1));
        let domain2 = gac.get_domain_values(Variable(2));
        
        assert!(!domain1.contains(&3));
        assert!(!domain2.contains(&3));
    }
    
    #[test]
    fn test_domain_type_automatic_selection() {
        // Small domain should use BitSet
        let small_domain = DomainType::new(1, 10);
        match small_domain {
            DomainType::BitSet(_) => {}, // Expected
            DomainType::SparseSet(_) => panic!("Small domain should use BitSet"),
        }
        
        // Large domain should use SparseSet
        let large_domain = DomainType::new(1, 200);
        match large_domain {
            DomainType::SparseSet(_) => {}, // Expected
            DomainType::BitSet(_) => panic!("Large domain should use SparseSet"),
        }
    }
    
    #[test]
    fn test_domain_type_operations() {
        let mut domain = DomainType::new(1, 5);
        
        // Test basic operations
        assert_eq!(domain.size(), 5);
        assert!(domain.contains(1));
        assert!(domain.contains(5));
        assert!(!domain.contains(0));
        assert!(!domain.contains(6));
        assert!(!domain.is_empty());
        assert!(!domain.is_fixed());
        
        // Test removal
        assert!(domain.remove(3));
        assert!(!domain.contains(3));
        assert_eq!(domain.size(), 4);
        assert!(!domain.remove(3)); // Already removed
        
        // Test min/max
        assert_eq!(domain.min(), 1);
        assert_eq!(domain.max(), 5);
    }
    
    #[test]
    fn test_bipartite_graph_hybrid() {
        let mut graph = BipartiteGraph::new();
        
        // Test with small domain (should use BitSet internally)
        graph.add_variable_range(Variable(0), 1, 3);
        // Test with larger domain (should use SparseSet internally)
        graph.add_variable_range(Variable(1), 1, 150);
        
        assert_eq!(graph.variables().count(), 2);
        
        // Test efficient operations work with both domain types
        assert_eq!(graph.domain_size(Variable(0)), 3);
        assert_eq!(graph.domain_size(Variable(1)), 150);
        
        assert!(graph.contains_value(Variable(0), Value(1)));
        assert!(graph.contains_value(Variable(1), Value(1)));
        assert!(graph.contains_value(Variable(1), Value(150)));
        assert!(!graph.contains_value(Variable(0), Value(4)));
        
        // Test min/max
        assert_eq!(graph.domain_min(Variable(0)), Some(Value(1)));
        assert_eq!(graph.domain_max(Variable(0)), Some(Value(3)));
        assert_eq!(graph.domain_min(Variable(1)), Some(Value(1)));
        assert_eq!(graph.domain_max(Variable(1)), Some(Value(150)));
        
        // Test removal works with both types
        assert!(graph.remove_value(Variable(0), Value(2)));
        assert_eq!(graph.domain_size(Variable(0)), 2);
        assert!(!graph.contains_value(Variable(0), Value(2)));
        
        assert!(graph.remove_value(Variable(1), Value(75)));
        assert_eq!(graph.domain_size(Variable(1)), 149);
        assert!(!graph.contains_value(Variable(1), Value(75)));
    }
    
    #[test]
    fn test_matching_hybrid() {
        let mut graph = BipartiteGraph::new();
        
        // Mix of small and large domains
        graph.add_variable(Variable(0), vec![1, 2]);
        graph.add_variable(Variable(1), vec![2, 3]);
        graph.add_variable_range(Variable(2), 1, 200); // Large domain
        
        let matching = Matching::find_maximum_matching(&graph);
        
        // Should be able to find complete matching
        assert!(matching.is_complete(&graph));
        assert_eq!(matching.var_to_val.len(), 3);
        assert_eq!(matching.val_to_var.len(), 3);
    }
    
    #[test]
    fn test_domain_type_from_values() {
        // Dense small domain should use BitSet
        let dense_small = DomainType::new_from_values(vec![1, 2, 3, 4, 5]);
        match dense_small {
            DomainType::BitSet(_) => {}, // Expected
            DomainType::SparseSet(_) => panic!("Dense small domain should use BitSet"),
        }
        
        // Sparse small domain should use SparseSet
        let sparse_small = DomainType::new_from_values(vec![1, 100]);
        match sparse_small {
            DomainType::SparseSet(_) => {}, // Expected  
            DomainType::BitSet(_) => panic!("Sparse domain should use SparseSet"),
        }
        
        // Large domain should always use SparseSet
        let large_values: Vec<i32> = (1..=200).collect();
        let large_domain = DomainType::new_from_values(large_values);
        match large_domain {
            DomainType::SparseSet(_) => {}, // Expected
            DomainType::BitSet(_) => panic!("Large domain should use SparseSet"),
        }
    }
}