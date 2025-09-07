/// AllDiffbit GAC implementation using bitwise operations
/// 
/// Based on "A Bitwise GAC Algorithm for Alldifferent Constraints" (IJCAI 2023)
/// Key innovation: Use bitwise data structures and operations to efficiently
/// determine if a node is in an SCC, rather than computing all SCCs explicitly.

use std::collections::{HashMap, HashSet, VecDeque};
use crate::domain::sparse_set::SparseSet;

/// Represents a variable in the bipartite graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable(pub usize);

/// Represents a value in the bipartite graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub i32);

/// A bipartite graph between variables and their possible values
#[derive(Debug, Clone)]
pub struct BipartiteGraph {
    /// Variables and their domains
    pub var_domains: HashMap<Variable, Vec<Value>>,
    /// Values and which variables can take them
    pub value_vars: HashMap<Value, Vec<Variable>>,
}

impl BipartiteGraph {
    pub fn new() -> Self {
        Self {
            var_domains: HashMap::new(),
            value_vars: HashMap::new(),
        }
    }
    
    /// Add a variable with its domain
    pub fn add_variable(&mut self, var: Variable, domain: Vec<i32>) {
        let values: Vec<Value> = domain.iter().map(|&v| Value(v)).collect();
        
        // Add to var_domains
        self.var_domains.insert(var, values.clone());
        
        // Add to value_vars
        for value in values {
            self.value_vars.entry(value).or_insert_with(Vec::new).push(var);
        }
    }
    
    /// Remove a value from a variable's domain
    pub fn remove_value(&mut self, var: Variable, val: Value) -> bool {
        let mut changed = false;
        
        // Remove from var_domains
        if let Some(domain) = self.var_domains.get_mut(&var) {
            if let Some(pos) = domain.iter().position(|&v| v == val) {
                domain.remove(pos);
                changed = true;
            }
        }
        
        // Remove from value_vars
        if let Some(vars) = self.value_vars.get_mut(&val) {
            if let Some(pos) = vars.iter().position(|&v| v == var) {
                vars.remove(pos);
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
    
    /// Get domain of a variable
    pub fn domain(&self, var: Variable) -> Vec<Value> {
        self.var_domains.get(&var).cloned().unwrap_or_default()
    }
    
    /// Check if variable is assigned (domain size = 1)
    pub fn is_assigned(&self, var: Variable) -> bool {
        self.var_domains.get(&var).map_or(false, |d| d.len() == 1)
    }
    
    /// Get assigned value if variable is assigned
    pub fn assigned_value(&self, var: Variable) -> Option<Value> {
        let domain = self.var_domains.get(&var)?;
        if domain.len() == 1 {
            Some(domain[0])
        } else {
            None
        }
    }
}

/// Bitwise adjacency matrix for efficient connectivity checking
/// Now supports arbitrary size graphs using bit vectors
#[derive(Debug)]
struct BitMatrix {
    /// Adjacency matrix: bit_matrix[i] represents edges from node i using bit vectors
    adjacency: HashMap<usize, Vec<u64>>,
    /// Node mapping
    node_to_id: HashMap<String, usize>,
    id_to_node: HashMap<usize, String>,
    next_id: usize,
    /// Number of u64 chunks needed to represent all nodes
    chunks: usize,
}

impl BitMatrix {
    fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            node_to_id: HashMap::new(),
            id_to_node: HashMap::new(),
            next_id: 0,
            chunks: 1, // Start with at least one chunk
        }
    }
    
    /// Calculate how many u64 chunks we need for n nodes
    fn calculate_chunks(max_nodes: usize) -> usize {
        (max_nodes + 63) / 64 // Round up division
    }
    
    /// Expand bit vectors if needed to accommodate new nodes
    fn ensure_capacity(&mut self, node_id: usize) {
        let required_chunks = Self::calculate_chunks(node_id + 1);
        if required_chunks > self.chunks {
            self.chunks = required_chunks;
            // Expand existing bit vectors
            for bit_vec in self.adjacency.values_mut() {
                bit_vec.resize(self.chunks, 0);
            }
        }
    }
    
    /// Add a node and return its ID
    fn add_node(&mut self, node: String) -> usize {
        if let Some(&id) = self.node_to_id.get(&node) {
            return id;
        }
        
        let id = self.next_id;
        self.next_id += 1;
        
        // Ensure we have enough capacity
        self.ensure_capacity(id);
        
        self.node_to_id.insert(node.clone(), id);
        self.id_to_node.insert(id, node);
        self.adjacency.insert(id, vec![0u64; self.chunks]);
        id
    }
    
    /// Add an edge between two nodes (now supports unlimited nodes)
    fn add_edge(&mut self, from: &str, to: &str) {
        let from_id = self.add_node(from.to_string());
        let to_id = self.add_node(to.to_string());
        
        // Calculate which chunk and bit position
        let chunk_index = to_id / 64;
        let bit_position = to_id % 64;
        
        if chunk_index < self.chunks {
            let bit = 1u64 << bit_position;
            if let Some(adjacency_vec) = self.adjacency.get_mut(&from_id) {
                adjacency_vec[chunk_index] |= bit;
            }
        }
    }
    
    /// Check connectivity from one node to another using bitwise BFS (now scalable)
    fn is_connected(&mut self, from: &str, to: &str) -> bool {
        let from_id = match self.node_to_id.get(from) {
            Some(&id) => id,
            None => return false,
        };
        let to_id = match self.node_to_id.get(to) {
            Some(&id) => id,
            None => return false,
        };
        
        // Initialize frontier and visited using bit vectors
        let mut visited = vec![0u64; self.chunks];
        let mut frontier = vec![0u64; self.chunks];
        
        // Set starting node in frontier
        let from_chunk = from_id / 64;
        let from_bit = from_id % 64;
        if from_chunk < self.chunks {
            frontier[from_chunk] |= 1u64 << from_bit;
        }
        
        // BFS using bitwise operations on vectors
        while !self.is_bitvec_empty(&frontier) {
            // Check if target is reachable
            let to_chunk = to_id / 64;
            let to_bit = to_id % 64;
            if to_chunk < self.chunks && (frontier[to_chunk] & (1u64 << to_bit)) != 0 {
                return true;
            }
            
            // Mark current frontier as visited
            for i in 0..self.chunks {
                visited[i] |= frontier[i];
            }
            
            // Expand frontier using bitwise operations on vectors
            let mut new_frontier = vec![0u64; self.chunks];
            for node_id in 0..self.next_id {
                let node_chunk = node_id / 64;
                let node_bit = node_id % 64;
                
                if node_chunk < self.chunks && (frontier[node_chunk] & (1u64 << node_bit)) != 0 {
                    // This node is in the current frontier, add its neighbors
                    if let Some(neighbors) = self.adjacency.get(&node_id) {
                        for i in 0..self.chunks {
                            new_frontier[i] |= neighbors[i] & !visited[i];
                        }
                    }
                }
            }
            
            frontier = new_frontier;
        }
        
        false
    }
    
    /// Helper function to check if a bit vector is empty
    fn is_bitvec_empty(&self, bitvec: &[u64]) -> bool {
        bitvec.iter().all(|&chunk| chunk == 0)
    }
}

/// Maximum bipartite matching using augmenting paths
#[derive(Debug)]
pub struct Matching {
    /// Variable to value mapping
    pub var_to_val: HashMap<Variable, Value>,
    /// Value to variable mapping
    pub val_to_var: HashMap<Value, Variable>,
}

impl Matching {
    pub fn new() -> Self {
        Self {
            var_to_val: HashMap::new(),
            val_to_var: HashMap::new(),
        }
    }
    
    /// Find maximum matching using augmenting paths
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
    
    /// Find augmenting path using BFS
    fn find_augmenting_path(graph: &BipartiteGraph, matching: &mut Self, start_var: Variable) -> bool {
        let mut queue = VecDeque::new();
        let mut visited_vars = HashSet::new();
        let mut visited_vals = HashSet::new();
        let mut parent_var: HashMap<Variable, Value> = HashMap::new();
        let mut parent_val: HashMap<Value, Variable> = HashMap::new();
        
        queue.push_back(start_var);
        visited_vars.insert(start_var);
        
        while let Some(current_var) = queue.pop_front() {
            // Try all values in this variable's domain
            for &val in &graph.domain(current_var) {
                if visited_vals.contains(&val) {
                    continue;
                }
                
                visited_vals.insert(val);
                parent_val.insert(val, current_var);
                
                // If this value is free, we found an augmenting path
                if !matching.val_to_var.contains_key(&val) {
                    // Reconstruct and apply the augmenting path
                    Self::apply_augmenting_path(matching, start_var, val, &parent_var, &parent_val);
                    return true;
                }
                
                // Otherwise, follow the matching edge
                if let Some(&matched_var) = matching.val_to_var.get(&val) {
                    if !visited_vars.contains(&matched_var) {
                        visited_vars.insert(matched_var);
                        parent_var.insert(matched_var, val);
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

/// AllDiffbit GAC propagator using bitwise operations
pub struct AllDiffbit;

impl AllDiffbit {
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
    
    /// Build merged graph using bitwise representation
    fn build_merged_graph(graph: &BipartiteGraph, matching: &Matching) -> BitMatrix {
        let mut bit_matrix = BitMatrix::new();
        
        // For each variable, add edges to other variables through their matched values
        for var in graph.variables() {
            let var_node = format!("var_{}", var.0);
            
            if let Some(&matched_val) = matching.var_to_val.get(&var) {
                // Add edges to all variables that can take this matched value
                if let Some(target_vars) = graph.value_vars.get(&matched_val) {
                    for &target_var in target_vars {
                        if target_var != var {
                            let target_node = format!("var_{}", target_var.0);
                            bit_matrix.add_edge(&var_node, &target_node);
                        }
                    }
                }
            }
        }
        
        bit_matrix
    }
    
    /// Apply bitwise GAC by checking connectivity
    fn apply_bitwise_gac(graph: &mut BipartiteGraph, matching: &Matching, 
                        bit_matrix: &mut BitMatrix) -> bool {
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
    
    /// Check if a value is reachable through alternating paths (bitwise)
    fn is_value_reachable(var: Variable, val: Value, matching: &Matching, 
                         bit_matrix: &mut BitMatrix) -> bool {
        
        // If the value is free (not matched), it's always reachable
        if !matching.val_to_var.contains_key(&val) {
            return true;
        }
        
        // If the value is matched to another variable, check connectivity
        if let Some(&matched_var) = matching.val_to_var.get(&val) {
            let from_node = format!("var_{}", var.0);
            let to_node = format!("var_{}", matched_var.0);
            
            // Use bitwise connectivity checking
            return bit_matrix.is_connected(&from_node, &to_node);
        }
        
        false
    }
}

/// Residual graph for GAC analysis
#[derive(Debug)]
pub struct ResidualGraph {
    /// Adjacency list representation
    pub adj: HashMap<String, Vec<String>>,
}

impl ResidualGraph {
    /// Build residual graph from bipartite graph and matching
    pub fn build(graph: &BipartiteGraph, matching: &Matching) -> Self {
        let mut adj = HashMap::new();
        
        // Add all nodes
        for var in graph.variables() {
            adj.insert(format!("var_{}", var.0), Vec::new());
        }
        for val in graph.values() {
            adj.insert(format!("val_{}", val.0), Vec::new());
        }
        adj.insert("sink".to_string(), Vec::new());
        
        // Build residual graph edges according to Régin's algorithm:
        // 1. Matched edges: var -> val (direction of matching)
        // 2. Unmatched edges: val -> var (reverse direction)
        // 3. Edges to sink from matched values
        // 4. Edges from sink to unmatched values
        
        for var in graph.variables() {
            let var_node = format!("var_{}", var.0);
            
            for &val in &graph.domain(var) {
                let val_node = format!("val_{}", val.0);
                
                if matching.var_to_val.get(&var) == Some(&val) {
                    // This is a matched edge: var -> val
                    adj.get_mut(&var_node).unwrap().push(val_node.clone());
                } else {
                    // This is an unmatched edge: val -> var
                    adj.get_mut(&val_node).unwrap().push(var_node.clone());
                }
            }
        }
        
        // Add edges involving sink
        for val in graph.values() {
            let val_node = format!("val_{}", val.0);
            if matching.val_to_var.contains_key(&val) {
                // Matched value: val -> sink
                adj.get_mut(&val_node).unwrap().push("sink".to_string());
            } else {
                // Free value: sink -> val
                adj.get_mut("sink").unwrap().push(val_node.clone());
            }
        }
        
        Self { adj }
    }
}

/// Strongly Connected Components using Tarjan's algorithm
#[derive(Debug)]
pub struct SCCFinder {
    index: usize,
    stack: Vec<String>,
    indices: HashMap<String, usize>,
    lowlinks: HashMap<String, usize>,
    on_stack: HashSet<String>,
    sccs: Vec<HashSet<String>>,
}

impl SCCFinder {
    pub fn new() -> Self {
        Self {
            index: 0,
            stack: Vec::new(),
            indices: HashMap::new(),
            lowlinks: HashMap::new(),
            on_stack: HashSet::new(),
            sccs: Vec::new(),
        }
    }
    
    /// Find all SCCs in the residual graph
    pub fn find_sccs(graph: &ResidualGraph) -> Vec<HashSet<String>> {
        let mut finder = Self::new();
        
        for node in graph.adj.keys() {
            if !finder.indices.contains_key(node) {
                finder.strongconnect(node, graph);
            }
        }
        
        finder.sccs
    }
    
    fn strongconnect(&mut self, v: &str, graph: &ResidualGraph) {
        self.indices.insert(v.to_string(), self.index);
        self.lowlinks.insert(v.to_string(), self.index);
        self.index += 1;
        self.stack.push(v.to_string());
        self.on_stack.insert(v.to_string());
        
        if let Some(neighbors) = graph.adj.get(v) {
            for w in neighbors {
                if !self.indices.contains_key(w) {
                    self.strongconnect(w, graph);
                    let w_lowlink = self.lowlinks[w];
                    let v_lowlink = self.lowlinks[v];
                    self.lowlinks.insert(v.to_string(), v_lowlink.min(w_lowlink));
                } else if self.on_stack.contains(w) {
                    let w_index = self.indices[w];
                    let v_lowlink = self.lowlinks[v];
                    self.lowlinks.insert(v.to_string(), v_lowlink.min(w_index));
                }
            }
        }
        
        if self.lowlinks[v] == self.indices[v] {
            let mut component = HashSet::new();
            loop {
                let w = self.stack.pop().unwrap();
                self.on_stack.remove(&w);
                component.insert(w.clone());
                if w == v {
                    break;
                }
            }
            self.sccs.push(component);
        }
    }
}

/// Legacy alias for compatibility with existing code
pub struct GACAllDifferent;

impl GACAllDifferent {
    pub fn propagate(graph: &mut BipartiteGraph) -> bool {
        AllDiffbit::propagate(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bipartite_graph_basic() {
        let mut graph = BipartiteGraph::new();
        
        graph.add_variable(Variable(0), vec![1, 2, 3]);
        graph.add_variable(Variable(1), vec![1, 2, 3]);
        graph.add_variable(Variable(2), vec![1, 2, 3]);
        
        assert_eq!(graph.domain(Variable(0)), vec![Value(1), Value(2), Value(3)]);
        assert_eq!(graph.variables().count(), 3);
        assert_eq!(graph.values().count(), 3);
    }
    
    #[test]
    fn test_maximum_matching() {
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
    fn test_alliffbit_propagation() {
        let mut graph = BipartiteGraph::new();
        
        graph.add_variable(Variable(0), vec![1, 2, 3]);
        graph.add_variable(Variable(1), vec![1, 2, 3]);
        graph.add_variable(Variable(2), vec![1, 2, 3]);
        
        let result = AllDiffbit::propagate(&mut graph);
        
        assert!(result); // Should succeed
        
        // In a perfect 3x3 case, AllDiffbit should not remove values
        // since all values can participate in some complete matching
        for var in [Variable(0), Variable(1), Variable(2)] {
            let domain_size = graph.domain(var).len();
            assert!(domain_size > 0, "Variable should have non-empty domain");
        }
    }
    
    #[test]
    fn test_alliffbit_impossible() {
        let mut graph = BipartiteGraph::new();
        
        graph.add_variable(Variable(0), vec![1, 2]);
        graph.add_variable(Variable(1), vec![1, 2]);
        graph.add_variable(Variable(2), vec![1, 2]);
        
        let result = AllDiffbit::propagate(&mut graph);
        
        assert!(!result); // Should fail - impossible
    }
    
    #[test]
    fn test_alliffbit_domain_reduction() {
        let mut graph = BipartiteGraph::new();
        
        // Force a specific assignment
        graph.add_variable(Variable(0), vec![1]); // Fixed to 1
        graph.add_variable(Variable(1), vec![1, 2, 3]);
        graph.add_variable(Variable(2), vec![1, 2, 3]);
        
        let result = AllDiffbit::propagate(&mut graph);
        
        assert!(result);
        
        // Variables 1 and 2 might have value 1 removed by AllDiffbit
        let domain1 = graph.domain(Variable(1));
        let domain2 = graph.domain(Variable(2));
        
        // At minimum, domains should be non-empty
        assert!(!domain1.is_empty());
        assert!(!domain2.is_empty());
    }
    
    #[test]
    fn test_bitmatrix_large_scale() {
        // Test that we can handle more than 64 nodes (previous limitation)
        let mut bit_matrix = BitMatrix::new();
        
        // Add 100 nodes to test scalability
        for i in 0..100 {
            let node_name = format!("node_{}", i);
            bit_matrix.add_node(node_name);
        }
        
        // Add some edges across the 64-node boundary
        bit_matrix.add_edge("node_50", "node_70");  // Both > 64 would fail in old version
        bit_matrix.add_edge("node_10", "node_90");
        bit_matrix.add_edge("node_70", "node_80");
        
        // Test connectivity across large node IDs
        assert!(bit_matrix.is_connected("node_50", "node_70"));
        assert!(bit_matrix.is_connected("node_50", "node_80")); // Transitive
        assert!(bit_matrix.is_connected("node_10", "node_90"));
        assert!(!bit_matrix.is_connected("node_0", "node_99")); // No path
    }
    
    #[test]
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
pub struct SparseSetGAC {
    /// Variable domains using SparseSet for O(1) operations
    pub domains: HashMap<Variable, SparseSet>,
    /// Cached matching for incremental updates
    pub cached_matching: Option<Matching>,
}

impl SparseSetGAC {
    /// Create a new SparseSet-based GAC instance
    pub fn new() -> Self {
        Self {
            domains: HashMap::new(),
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
    
    /// Check if a variable is assigned (domain size = 1)
    pub fn is_assigned(&self, var: Variable) -> bool {
        self.domains.get(&var).map_or(false, |d| d.is_fixed())
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
        
        // Update sparse sets with filtered domains
        let mut changed = false;
        for (&var, sparse_domain) in &mut self.domains {
            let new_domain = graph.domain(var);
            let new_values: Vec<i32> = new_domain.iter().map(|v| v.0).collect();
            
            // Remove values that GAC eliminated
            let current_values = sparse_domain.to_vec();
            for val in current_values {
                if !new_values.contains(&val) {
                    if sparse_domain.remove(val) {
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
    
    /// Get statistics about the current state
    pub fn stats(&self) -> GACStats {
        let total_vars = self.domains.len();
        let assigned_vars = self.variables().into_iter()
            .filter(|&v| self.is_assigned(v))
            .count();
        let total_domain_size: usize = self.domains.values()
            .map(|d| d.size())
            .sum();
        let min_domain_size = self.domains.values()
            .map(|d| d.size())
            .min()
            .unwrap_or(0);
        let max_domain_size = self.domains.values()
            .map(|d| d.size())
            .max()
            .unwrap_or(0);
            
        GACStats {
            total_variables: total_vars,
            assigned_variables: assigned_vars,
            total_domain_size,
            min_domain_size,
            max_domain_size,
        }
    }
}

/// Statistics about GAC state
#[derive(Debug, Clone)]
pub struct GACStats {
    pub total_variables: usize,
    pub assigned_variables: usize,
    pub total_domain_size: usize,
    pub min_domain_size: usize,
    pub max_domain_size: usize,
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
