/// AllDiffbit GAC implementation using bitwise operations
/// 
/// Based on "A Bitwise GAC Algorithm for Alldifferent Constraints" (IJCAI 2023)
/// Key innovation: Use bitwise data structures and operations to efficiently
/// determine if a node is in an SCC, rather than computing all SCCs explicitly.
use std::collections::{HashMap, HashSet};
use crate::variables::domain::sparse_set::SparseSet;
use crate::constraints::gac_hybrid::{Variable, Value, BipartiteGraph, Matching, GACStats};

/// Simple boolean-based consistency checking for GAC operations



/// Optimized bitwise adjacency matrix using integer node IDs
/// Eliminates string operations and hash lookups for better performance
#[derive(Debug)]
struct OptimizedBitMatrix {
    /// Adjacency using vector for direct indexing (no hash lookups)
    adjacency: Vec<Vec<u64>>,
    /// Maximum node ID we can handle
    max_nodes: usize,
    /// Number of u64 chunks per row
    chunks: usize,
}

impl OptimizedBitMatrix {
    fn new(max_variables: usize, max_values: usize) -> Self {
        // Total nodes = variables + values
        let max_nodes = max_variables + max_values;
        let chunks = (max_nodes + 63) / 64;
        
        Self {
            adjacency: vec![vec![0u64; chunks]; max_nodes],
            max_nodes,
            chunks,
        }
    }
    
    /// Convert Variable to node ID
    fn var_to_node_id(&self, var: Variable) -> usize {
        var.0
    }
    
    /// Convert Value to node ID (offset by max variables)

    

    
    /// Add edge using direct indexing (O(1))
    fn add_edge(&mut self, from_node: usize, to_node: usize) {
        if from_node < self.max_nodes && to_node < self.max_nodes {
            let chunk_idx = to_node / 64;
            let bit_pos = to_node % 64;
            
            if chunk_idx < self.chunks {
                self.adjacency[from_node][chunk_idx] |= 1u64 << bit_pos;
            }
        }
    }
    
    /// Optimized connectivity check using bitwise BFS
    fn is_connected(&self, from_node: usize, to_node: usize) -> bool {
        if from_node >= self.max_nodes || to_node >= self.max_nodes {
            return false;
        }
        
        if from_node == to_node {
            return true;
        }
        
        // Use bit vectors for visited and frontier
        let mut visited = vec![0u64; self.chunks];
        let mut frontier = vec![0u64; self.chunks];
        
        // Set start node
        let from_chunk = from_node / 64;
        let from_bit = from_node % 64;
        if from_chunk < self.chunks {
            frontier[from_chunk] |= 1u64 << from_bit;
        }
        
        // Bitwise BFS
        while !self.is_empty(&frontier) {
            // Check if target reached
            let to_chunk = to_node / 64;
            let to_bit = to_node % 64;
            if to_chunk < self.chunks && (frontier[to_chunk] & (1u64 << to_bit)) != 0 {
                return true;
            }
            
            // Mark frontier as visited
            for i in 0..self.chunks {
                visited[i] |= frontier[i];
            }
            
            // Expand frontier
            let mut new_frontier = vec![0u64; self.chunks];
            for node_id in 0..self.max_nodes {
                let node_chunk = node_id / 64;
                let node_bit = node_id % 64;
                
                if node_chunk < self.chunks 
                    && (frontier[node_chunk] & (1u64 << node_bit)) != 0 {
                    // Node is in frontier, add its neighbors
                    for i in 0..self.chunks {
                        new_frontier[i] |= self.adjacency[node_id][i] & !visited[i];
                    }
                }
            }
            
            frontier = new_frontier;
        }
        
        false
    }
    
    fn is_empty(&self, bitvec: &[u64]) -> bool {
        bitvec.iter().all(|&chunk| chunk == 0)
    }
}



/// Optimized AllDifferent GAC using SparseSet and integer-based BitMatrix
#[doc(hidden)]
pub struct SparseSetAllDiff;

impl SparseSetAllDiff {
    /// Apply GAC using bitwise connectivity checking
    pub fn propagate(graph: &mut BipartiteGraph) -> bool {
        // Step 1: Find maximum matching
        let matching = Matching::find_maximum_matching(graph);
        
        // Step 2: Check if complete matching exists
        if !matching.is_complete(graph) {
            return false;
        }
        
        // Step 3: Build merged graph (combine matched var-val pairs)
        let mut bit_matrix = Self::build_merged_graph(graph, &matching);
        
        // Step 4: Apply bitwise GAC - check connectivity for each value
        let _changed = Self::apply_bitwise_gac(graph, &matching, &mut bit_matrix);
        
        true
    }
    
    /// Build connectivity graph using optimized BitMatrix
    fn build_merged_graph(graph: &BipartiteGraph, matching: &Matching) -> OptimizedBitMatrix {
        // Estimate sizes for optimal allocation
        let max_vars = graph.var_domains.len();
        let max_vals = graph.value_vars.len();
        let mut bit_matrix = OptimizedBitMatrix::new(max_vars, max_vals);
        
        // Build edges between variables through their potential values
        for var in graph.variables() {
            let var_node = bit_matrix.var_to_node_id(var);
            
            if let Some(&matched_val) = matching.var_to_val.get(&var) {
                // Add edges to other variables that can take this value
                if let Some(target_vars) = graph.value_vars.get(&matched_val) {
                    for &target_var in target_vars {
                        if target_var != var {
                            let target_node = bit_matrix.var_to_node_id(target_var);
                            bit_matrix.add_edge(var_node, target_node);
                        }
                    }
                }
            }
        }
        
        bit_matrix
    }
    
    /// Apply GAC filtering with SparseSet integration
    fn apply_bitwise_gac(graph: &mut BipartiteGraph, matching: &Matching, 
                        bit_matrix: &mut OptimizedBitMatrix) -> bool {
        let mut changed = false;
        
        // For each variable-value pair, check if it can participate in a complete matching
        let vars: Vec<Variable> = graph.variables().collect();
        for var in vars {
            let domain = graph.domain(var);
            for val in domain {
                // Skip if this is the matched value
                if matching.var_to_val.get(&var) == Some(&val) {
                    continue;
                }
                
                // Check if this value can be reached through alternating paths
                if !Self::is_value_reachable(var, val, matching, bit_matrix) {
                    if graph.remove_value(var, val) {
                        changed = true;
                    }
                }
            }
        }
        
        changed
    }
    
    /// Check value consistency using optimized connectivity
    fn is_value_reachable(var: Variable, val: Value, matching: &Matching, 
                         bit_matrix: &OptimizedBitMatrix) -> bool {
        
        // Free values are always consistent
        if !matching.val_to_var.contains_key(&val) {
            return true;
        }
        
        // Check connectivity to matched variable
        if let Some(&matched_var) = matching.val_to_var.get(&val) {
            let from_node = bit_matrix.var_to_node_id(var);
            let to_node = bit_matrix.var_to_node_id(matched_var);
            return bit_matrix.is_connected(from_node, to_node);
        }
        
        false
    }
}



/// Legacy compatibility aliases
#[doc(hidden)]
pub type AllDiffbit = SparseSetAllDiff;

#[doc(hidden)] 
pub struct GACAllDifferent;

impl GACAllDifferent {
    pub fn propagate(graph: &mut BipartiteGraph) -> bool {
        SparseSetAllDiff::propagate(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bipartite_graph_optimization() {
        let mut graph = BipartiteGraph::new();
        
        // Test range-based variable addition (most efficient)
        graph.add_variable_range(Variable(0), 1, 3);
        graph.add_variable_range(Variable(1), 1, 3);
        graph.add_variable_range(Variable(2), 1, 3);
        
        assert_eq!(graph.variables().count(), 3);
        assert_eq!(graph.values().count(), 3);
        
        // Test SparseSet integration
        assert!(graph.domain_iter(Variable(0)).count() > 0);
        assert_eq!(graph.domain(Variable(0)).len(), 3);
    }
    
    #[test]
    fn test_optimized_matching() {
        let mut graph = BipartiteGraph::new();
        
        graph.add_variable(Variable(0), vec![1, 2]);
        graph.add_variable(Variable(1), vec![2, 3]);
        graph.add_variable(Variable(2), vec![1, 3]);
        
        let matching = Matching::find_maximum_matching(&graph);
        
        assert!(matching.is_complete(&graph));
        assert_eq!(matching.var_to_val.len(), 3);
        assert_eq!(matching.val_to_var.len(), 3);
    }
    
    #[test]
    fn test_impossible_matching() {
        let mut graph = BipartiteGraph::new();
        
        // 3 variables, only 2 values - impossible
        graph.add_variable(Variable(0), vec![1, 2]);
        graph.add_variable(Variable(1), vec![1, 2]);
        graph.add_variable(Variable(2), vec![1, 2]);
        
        let matching = Matching::find_maximum_matching(&graph);
        
        assert!(!matching.is_complete(&graph));
    }
    
    #[test]  
    fn test_sparse_set_all_diff() {
        let mut graph = BipartiteGraph::new();
        
        graph.add_variable_range(Variable(0), 1, 3);
        graph.add_variable_range(Variable(1), 1, 3);
        graph.add_variable_range(Variable(2), 1, 3);
        
        let result = SparseSetAllDiff::propagate(&mut graph);
        
        assert!(result); // Should succeed
        
        // In a perfect 3x3 case, AllDiffbit should not remove values
        // since all values can participate in some complete matching
        for var in [Variable(0), Variable(1), Variable(2)] {
            let domain_size = graph.domain(var).len();
            assert!(domain_size > 0, "Variable should have non-empty domain");
        }
    }
    
    #[test]
    fn test_impossible_case() {
        let mut graph = BipartiteGraph::new();
        
        // 3 variables, only 2 values
        graph.add_variable_range(Variable(0), 1, 2);
        graph.add_variable_range(Variable(1), 1, 2);
        graph.add_variable_range(Variable(2), 1, 2);
        
        let result = SparseSetAllDiff::propagate(&mut graph);
        
        assert!(!result); // Should fail - impossible
    }
    
    #[test]
    fn test_sparse_set_domain_reduction() {
        let mut graph = BipartiteGraph::new();
        
        // Force a specific assignment
        graph.add_variable(Variable(0), vec![1]); // Fixed to 1
        graph.add_variable_range(Variable(1), 1, 3);
        graph.add_variable_range(Variable(2), 1, 3);
        
        let result = SparseSetAllDiff::propagate(&mut graph);
        
        assert!(result);
        
        // Variables 1 and 2 might have value 1 removed by AllDiffbit
        let domain1 = graph.domain(Variable(1));
        let domain2 = graph.domain(Variable(2));
        
        // At minimum, domains should be non-empty
        assert!(!domain1.is_empty());
        assert!(!domain2.is_empty());
    }
    
    #[test]
    fn test_optimized_bit_matrix() {
        // Test that our optimized BitMatrix works correctly
        let mut bit_matrix = OptimizedBitMatrix::new(10, 10);
        
        // Add some edges using direct node IDs
        bit_matrix.add_edge(0, 1); // Variable(0) -> Variable(1)  
        bit_matrix.add_edge(1, 2); // Variable(1) -> Variable(2)
        
        // Test connectivity
        assert!(bit_matrix.is_connected(0, 1));
        assert!(bit_matrix.is_connected(0, 2)); // Transitive
        assert!(!bit_matrix.is_connected(2, 0)); // No reverse path
    }
    
    #[test]
    #[ignore = "takes too mutch time"]
    fn test_large_scale_gac() {
        // Test GAC with more than 64 variables (would fail with old limit)
        let mut gac = SparseSetGAC::new();
        
        // Add 100 variables with domains [1, 100]
        for i in 0..100 {
            gac.add_variable(Variable(i), 1, 100);
        }
        
        // Should be consistent (100x100 is feasible)
        assert!(gac.fast_gac_propagate());
        
        let stats = gac.stats();
        assert_eq!(stats.total_variables, 100);
        assert_eq!(stats.total_domain_size, 10000); // 100 variables * 100 values each
    }
}

/// Enhanced GAC implementation using SparseSet for efficient domain operations
/// This integrates the robust sparse_set with GAC to provide better performance
/// and memory efficiency compared to vector-based implementations.
#[doc(hidden)]
pub struct SparseSetGAC {
    /// Variable domains using SparseSet for O(1) operations
    pub domains: HashMap<Variable, SparseSet>,
    /// Cached matching for incremental updates
    pub cached_matching: Option<Matching>,
}

impl Default for SparseSetGAC {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseSetGAC {
    /// Create a new SparseSet-based GAC instance
    pub fn new() -> Self {
        Self {
            domains: HashMap::with_capacity(32),
            cached_matching: None,
        }
    }
    
    /// Add a variable with its initial domain range
    pub fn add_variable(&mut self, var: Variable, min_val: i32, max_val: i32) {
        let sparse_set = SparseSet::new(min_val, max_val);
        self.domains.insert(var, sparse_set);
        // Invalidate cached matching when topology changes
        self.cached_matching = None;
    }
    
    /// Add a variable with specific values - more efficient than range + removal
    pub fn add_variable_with_values(&mut self, var: Variable, values: Vec<i32>) {
        let sparse_set = SparseSet::new_from_values(values);
        self.domains.insert(var, sparse_set);
        // Invalidate cached matching when topology changes
        self.cached_matching = None;
    }
    
    /// Remove a value from a variable's domain
    pub fn remove_value(&mut self, var: Variable, val: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            if domain.remove(val) {
                // Invalidate cached matching when domains change
                self.cached_matching = None;
                return true;
            }
        }
        false
    }
    
    /// Fix a variable to a specific value (remove all others)
    pub fn assign_variable(&mut self, var: Variable, val: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            if domain.contains(val) {
                domain.remove_all_but(val);
                // Invalidate cached matching when assignments change
                self.cached_matching = None;
                return true;
            }
        }
        false
    }
    
    /// Remove values above a threshold
    pub fn remove_above(&mut self, var: Variable, threshold: i32) -> bool {
        if let Some(domain) = self.domains.get_mut(&var) {
            let old_size = domain.size();
            domain.remove_above(threshold);
            let changed = domain.size() != old_size;
            if changed {
                self.cached_matching = None;
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
                self.cached_matching = None;
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
    
    /// Get domain bounds efficiently using SparseSet's O(1) min/max
    pub fn get_domain_bounds(&self, var: Variable) -> Option<(i32, i32)> {
        if let Some(domain) = self.domains.get(&var) {
            if domain.is_empty() {
                None
            } else {
                Some((domain.min(), domain.max()))
            }
        } else {
            None
        }
    }
    
    /// Check if a variable is assigned (domain size = 1)
    pub fn is_assigned(&self, var: Variable) -> bool {
        self.domains.get(&var).is_some_and(|d| d.is_fixed())
    }
    
    /// Get assigned value if variable is assigned
    pub fn get_assigned_value(&self, var: Variable) -> Option<i32> {
        let domain = self.domains.get(&var)?;
        if domain.is_fixed() {
            Some(domain.min())
        } else {
            None
        }
    }
    
    /// Check if any domain is empty (inconsistent state)
    pub fn has_empty_domain(&self) -> bool {
        self.domains.values().any(|d| d.is_empty())
    }
    
    /// Convert to BipartiteGraph for compatibility with existing GAC algorithms
    pub fn to_bipartite_graph(&self) -> BipartiteGraph {
        let mut graph = BipartiteGraph::new();
        
        for (&var, domain) in &self.domains {
            let values = domain.to_vec();
            graph.add_variable(var, values);
        }
        
        graph
    }
    
    /// Apply GAC propagation using sparse set operations
    pub fn propagate_gac(&mut self) -> bool {
        // Convert to bipartite graph for GAC algorithm
        let mut graph = self.to_bipartite_graph();
        
        // Apply existing GAC algorithm
        if !AllDiffbit::propagate(&mut graph) {
            return false; // Inconsistent
        }
        
        // Update domains based on GAC results
        let mut changed = false;
        for (&var, sparse_domain) in &mut self.domains {
            if let Some(new_domain) = graph.var_domains.get(&var) {
                // Compare domain sizes first for quick check
                if sparse_domain.size() != new_domain.size() {
                    // Extract values from the new domain (whether BitSet or SparseSet)
                    let new_values: Vec<i32> = new_domain.iter().collect();
                    *sparse_domain = SparseSet::new_from_values(new_values);
                    changed = true;
                } else {
                    // Check if domains are actually different
                    let mut domains_different = false;
                    for val in sparse_domain.clone() {
                        if !new_domain.contains(val) {
                            domains_different = true;
                            break;
                        }
                    }
                    if domains_different {
                        let new_values: Vec<i32> = new_domain.iter().collect();
                        *sparse_domain = SparseSet::new_from_values(new_values);
                        changed = true;
                    }
                }
            }
        }
        
        // Update cached matching if we have changes
        if changed {
            self.cached_matching = None;
        }
        
        true
    }
    
    /// Get all variables
    pub fn variables(&self) -> Vec<Variable> {
        self.domains.keys().copied().collect()
    }
    
    /// Optimized GAC that leverages SparseSet properties
    pub fn fast_gac_propagate(&mut self) -> bool {
        // Quick check: if any domain is empty, fail immediately
        if self.has_empty_domain() {
            return false;
        }
        
        // Quick check: if all variables are assigned, verify all-different
        let assigned_vars: Vec<_> = self.variables().into_iter()
            .filter(|&v| self.is_assigned(v))
            .collect();
            
        if assigned_vars.len() == self.domains.len() {
            // All variables assigned - just check they're all different
            let mut assigned_values = HashSet::new();
            for var in assigned_vars {
                if let Some(val) = self.get_assigned_value(var) {
                    if !assigned_values.insert(val) {
                        return false; // Duplicate value
                    }
                }
            }
            return true;
        }
        
        // Use full GAC algorithm for partial assignments
        self.propagate_gac()
    }
    
    /// Apply alldiff constraint to match BitSetGAC interface
    /// Uses the traditional GAC algorithm with bipartite graph and SCC analysis
    /// Returns (changed, consistent) where changed indicates if domains were modified
    /// and consistent indicates if the constraint is still satisfiable
    pub fn propagate_alldiff(&mut self, variables: &[Variable]) -> (bool, bool) {
        if variables.len() <= 1 {
            return (false, true); // Nothing to propagate, still consistent
        }
        
        // Filter to only the requested variables
        let filtered_domains: HashMap<Variable, SparseSet> = variables.iter()
            .filter_map(|&var| {
                self.domains.get(&var).map(|domain| (var, domain.clone()))
            })
            .collect();
        
        if filtered_domains.is_empty() {
            return (false, true);
        }
        
        // Create temporary GAC instance with only the requested variables
        let mut temp_gac = SparseSetGAC::new();
        temp_gac.domains = filtered_domains;
        
        // Apply GAC propagation
        let result = temp_gac.propagate_gac();
        if !result {
            // GAC propagation failed - inconsistent
            return (false, false);
        }
        
        // Update original domains and check for changes
        let mut changed = false;
        for &var in variables {
            if let (Some(original), Some(updated)) = (self.domains.get_mut(&var), temp_gac.domains.get(&var)) {
                if original.size() != updated.size() {
                    *original = updated.clone();
                    changed = true;
                }
            }
        }
        
        if changed {
            self.cached_matching = None;
        }
        
        (changed, true)
    }
    
    /// Get statistics about the current state
    pub fn stats(&self) -> GACStats {
        let total_vars = self.domains.len();
        let assigned_vars = self.variables().into_iter()
            .filter(|&v| self.is_assigned(v))
            .count();
            
        // Single pass to compute all domain size statistics
        let mut total_domain_size = 0;
        let mut min_domain_size = usize::MAX;
        let mut max_domain_size = 0;
        
        for domain in self.domains.values() {
            let size = domain.size();
            total_domain_size += size;
            min_domain_size = min_domain_size.min(size);
            max_domain_size = max_domain_size.max(size);
        }
        
        if self.domains.is_empty() {
            min_domain_size = 0;
        }
            
        GACStats {
            total_variables: total_vars,
            assigned_variables: assigned_vars,
            total_domain_size,
            min_domain_size,
            max_domain_size,
        }
    }
}



#[cfg(test)]
mod sparse_set_gac_tests {
    use super::*;
    
    #[test]
    fn test_sparse_set_gac_basic() {
        let mut gac = SparseSetGAC::new();
        
        // Add 3 variables with domains [1,3]
        gac.add_variable(Variable(0), 1, 3);
        gac.add_variable(Variable(1), 1, 3);
        gac.add_variable(Variable(2), 1, 3);
        
        // Should be consistent
        assert!(gac.fast_gac_propagate());
        
        // Check initial state
        let stats = gac.stats();
        assert_eq!(stats.total_variables, 3);
        assert_eq!(stats.assigned_variables, 0);
    }
    
    #[test]
    fn test_sparse_set_gac_assignment() {
        let mut gac = SparseSetGAC::new();
        
        gac.add_variable(Variable(0), 1, 3);
        gac.add_variable(Variable(1), 1, 3);
        gac.add_variable(Variable(2), 1, 3);
        
        // Assign variable 0 to value 1
        assert!(gac.assign_variable(Variable(0), 1));
        assert!(gac.is_assigned(Variable(0)));
        assert_eq!(gac.get_assigned_value(Variable(0)), Some(1));
        
        // Should still be consistent
        assert!(gac.fast_gac_propagate());
    }
    
    #[test]
    fn test_sparse_set_gac_impossible() {
        let mut gac = SparseSetGAC::new();
        
        // Add 3 variables with only 2 possible values
        gac.add_variable(Variable(0), 1, 2);
        gac.add_variable(Variable(1), 1, 2);
        gac.add_variable(Variable(2), 1, 2);
        
        // Should detect inconsistency
        assert!(!gac.fast_gac_propagate());
    }
    
    #[test]
    fn test_sparse_set_gac_domain_operations() {
        let mut gac = SparseSetGAC::new();
        
        gac.add_variable(Variable(0), 1, 5);
        
        // Test remove operations
        assert!(gac.remove_value(Variable(0), 3));
        assert!(!gac.remove_value(Variable(0), 3)); // Already removed
        
        assert!(gac.remove_above(Variable(0), 4));
        assert!(gac.remove_below(Variable(0), 2));
        
        // Should have domain [2, 4] with 3 removed = [2, 4]
        let mut domain = gac.get_domain_values(Variable(0));
        domain.sort(); // Sort for consistent comparison
        assert_eq!(domain, vec![2, 4]);
    }
    
    #[test]
    fn test_sparse_set_gac_all_assigned() {
        let mut gac = SparseSetGAC::new();
        
        gac.add_variable(Variable(0), 1, 3);
        gac.add_variable(Variable(1), 1, 3);
        gac.add_variable(Variable(2), 1, 3);
        
        // Assign all variables to different values
        assert!(gac.assign_variable(Variable(0), 1));
        assert!(gac.assign_variable(Variable(1), 2));
        assert!(gac.assign_variable(Variable(2), 3));
        
        // Should be consistent
        assert!(gac.fast_gac_propagate());
        
        let stats = gac.stats();
        assert_eq!(stats.assigned_variables, 3);
    }
    
    #[test]
    fn test_sparse_set_gac_duplicate_assignment() {
        let mut gac = SparseSetGAC::new();
        
        gac.add_variable(Variable(0), 1, 3);
        gac.add_variable(Variable(1), 1, 3);
        
        // Assign both to same value
        assert!(gac.assign_variable(Variable(0), 1));
        assert!(gac.assign_variable(Variable(1), 1));
        
        // Should detect conflict
        assert!(!gac.fast_gac_propagate());
    }
}
