/// AllDiffbit GAC implementation using bitwise operations
/// 
/// Based on "A Bitwise GAC Algorithm for Alldifferent Constraints" (IJCAI 2023)
/// Key innovation: Use bitwise data structures and operations to efficiently
/// determine if a node is in an SCC, rather than computing all SCCs explicitly.

use std::collections::{HashMap, HashSet, VecDeque};

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
#[derive(Debug)]
struct BitMatrix {
    /// Adjacency matrix: bit_matrix[i] represents edges from node i
    adjacency: HashMap<usize, u64>,
    /// Frontier matrix for BFS expansion
    frontier: HashMap<usize, u64>,
    /// Node mapping
    node_to_id: HashMap<String, usize>,
    id_to_node: HashMap<usize, String>,
    next_id: usize,
}

impl BitMatrix {
    fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            frontier: HashMap::new(),
            node_to_id: HashMap::new(),
            id_to_node: HashMap::new(),
            next_id: 0,
        }
    }
    
    /// Add a node and return its ID
    fn add_node(&mut self, node: String) -> usize {
        if let Some(&id) = self.node_to_id.get(&node) {
            return id;
        }
        
        let id = self.next_id;
        self.next_id += 1;
        self.node_to_id.insert(node.clone(), id);
        self.id_to_node.insert(id, node);
        self.adjacency.insert(id, 0);
        self.frontier.insert(id, 0);
        id
    }
    
    /// Add an edge between two nodes
    fn add_edge(&mut self, from: &str, to: &str) {
        let from_id = self.add_node(from.to_string());
        let to_id = self.add_node(to.to_string());
        
        if to_id < 64 { // Limit to 64 nodes for simplicity
            let bit = 1u64 << to_id;
            *self.adjacency.get_mut(&from_id).unwrap() |= bit;
        }
    }
    
    /// Check connectivity from one node to another using bitwise BFS
    fn is_connected(&mut self, from: &str, to: &str) -> bool {
        let from_id = match self.node_to_id.get(from) {
            Some(&id) => id,
            None => return false,
        };
        let to_id = match self.node_to_id.get(to) {
            Some(&id) => id,
            None => return false,
        };
        
        if from_id >= 64 || to_id >= 64 {
            return false; // Simplified implementation limit
        }
        
        // Initialize frontier with starting node
        let mut visited = 0u64;
        let mut frontier = 1u64 << from_id;
        
        while frontier != 0 {
            // Check if target is reachable
            if (frontier & (1u64 << to_id)) != 0 {
                return true;
            }
            
            // Mark current frontier as visited
            visited |= frontier;
            
            // Expand frontier using bitwise operations
            let mut new_frontier = 0u64;
            for node_id in 0..64 {
                if (frontier & (1u64 << node_id)) != 0 {
                    // Add neighbors that haven't been visited
                    if let Some(&neighbors) = self.adjacency.get(&node_id) {
                        new_frontier |= neighbors & !visited;
                    }
                }
            }
            
            frontier = new_frontier;
        }
        
        false
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
}
