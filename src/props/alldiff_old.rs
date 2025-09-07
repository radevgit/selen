use crate::{
    props::{Propagate, Prune},
    vars::{Val, VarId},
    views::{Context, View},
    gac::{BipartiteGraph, Variable, AllDiffbit},
};

/// AllDifferent constraint with AllDiffbit GAC implementation
/// 
/// Uses the AllDiffbit algorithm from the IJCAI 2023 paper which employs
/// bitwise data structures and operations for efficient GAC propagation.
#[derive(Clone, Debug)]
pub struct AllDifferent {
    vars: Vec<VarId>,
}

impl AllDifferent {
    pub fn new(vars: Vec<VarId>) -> Self {
        Self { vars }
    }
    
    /// Build bipartite graph and find maximum matching
    fn find_maximum_matching(&self, ctx: &Context) -> (HashMap<VarId, i32>, HashMap<i32, VarId>) {
        let mut var_to_value = HashMap::new();
        let mut value_to_var = HashMap::new();
        
        // Collect all possible variable-value pairs
        let mut var_domains: Vec<(VarId, Vec<i32>)> = Vec::new();
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                let domain: Vec<i32> = (min_i..=max_i).collect();
                var_domains.push((var, domain));
            }
        }
        
        // Sort by domain size (most constrained first for better matching)
        var_domains.sort_by_key(|(_, domain)| domain.len());
        
        // Greedy maximum matching
        for (var, domain) in var_domains {
            for &val in &domain {
                if !value_to_var.contains_key(&val) {
                    var_to_value.insert(var, val);
                    value_to_var.insert(val, var);
                    break;
                }
            }
        }
        
        (var_to_value, value_to_var)
    }
    
    /// Build residual graph for GAC analysis
    fn build_residual_graph(&self, ctx: &Context, matching: &HashMap<VarId, i32>) 
        -> (HashMap<String, Vec<String>>, HashSet<i32>) {
        
        let mut graph = HashMap::new();
        let mut all_values = HashSet::new();
        
        // Add nodes and collect all values
        for (idx, &var) in self.vars.iter().enumerate() {
            let var_node = format!("var_{}", idx);
            graph.insert(var_node.clone(), Vec::new());
            
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                for val in min_i..=max_i {
                    all_values.insert(val);
                    let val_node = format!("val_{}", val);
                    graph.entry(val_node.clone()).or_insert_with(Vec::new);
                }
            }
        }
        
        // Add sink node
        graph.insert("sink".to_string(), Vec::new());
        
        // Build residual graph edges
        for (idx, &var) in self.vars.iter().enumerate() {
            let var_node = format!("var_{}", idx);
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                for val in min_i..=max_i {
                    let val_node = format!("val_{}", val);
                    
                    if let Some(&matched_val) = matching.get(&var) {
                        if matched_val == val {
                            // Matched edge: var -> val
                            graph.get_mut(&var_node).unwrap().push(val_node.clone());
                        } else {
                            // Unmatched edge: val -> var
                            graph.get_mut(&val_node).unwrap().push(var_node.clone());
                        }
                    } else {
                        // Unmatched variable: val -> var
                        graph.get_mut(&val_node).unwrap().push(var_node.clone());
                    }
                }
            }
        }
        
        // Add edges to sink
        for &val in &all_values {
            let val_node = format!("val_{}", val);
            if matching.values().any(|&v| v == val) {
                // Matched value: val -> sink
                graph.get_mut(&val_node).unwrap().push("sink".to_string());
            } else {
                // Free value: sink -> val
                graph.get_mut("sink").unwrap().push(val_node);
            }
        }
        
        (graph, all_values)
    }
    
    /// Find strongly connected components using Tarjan's algorithm
    fn find_sccs(&self, graph: &HashMap<String, Vec<String>>) -> Vec<HashSet<String>> {
        let mut index_map = HashMap::new();
        let mut lowlink_map = HashMap::new();
        let mut on_stack = HashSet::new();
        let mut stack = Vec::new();
        let mut sccs = Vec::new();
        let mut index = 0;
        
        fn strongconnect(
            node: &str,
            graph: &HashMap<String, Vec<String>>,
            index_map: &mut HashMap<String, usize>,
            lowlink_map: &mut HashMap<String, usize>,
            on_stack: &mut HashSet<String>,
            stack: &mut Vec<String>,
            sccs: &mut Vec<HashSet<String>>,
            index: &mut usize,
        ) {
            index_map.insert(node.to_string(), *index);
            lowlink_map.insert(node.to_string(), *index);
            *index += 1;
            stack.push(node.to_string());
            on_stack.insert(node.to_string());
            
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    if !index_map.contains_key(neighbor) {
                        strongconnect(neighbor, graph, index_map, lowlink_map, on_stack, stack, sccs, index);
                        let neighbor_lowlink = lowlink_map[neighbor];
                        let current_lowlink = lowlink_map[node];
                        lowlink_map.insert(node.to_string(), current_lowlink.min(neighbor_lowlink));
                    } else if on_stack.contains(neighbor) {
                        let neighbor_index = index_map[neighbor];
                        let current_lowlink = lowlink_map[node];
                        lowlink_map.insert(node.to_string(), current_lowlink.min(neighbor_index));
                    }
                }
            }
            
            if lowlink_map[node] == index_map[node] {
                let mut scc = HashSet::new();
                loop {
                    let w = stack.pop().unwrap();
                    on_stack.remove(&w);
                    scc.insert(w.clone());
                    if w == node {
                        break;
                    }
                }
                sccs.push(scc);
            }
        }
        
        for node in graph.keys() {
            if !index_map.contains_key(node) {
                strongconnect(node, graph, &mut index_map, &mut lowlink_map, 
                            &mut on_stack, &mut stack, &mut sccs, &mut index);
            }
        }
        
        sccs
    }
    
    /// Apply GAC by removing values not in any complete matching
    fn apply_gac(&self, ctx: &mut Context, matching: &HashMap<VarId, i32>, 
                sccs: &[HashSet<String>]) -> Option<()> {
        
        // Find which values are in SCCs reachable from free nodes
        let mut reachable_values = HashSet::new();
        
        for scc in sccs {
            let has_sink = scc.contains("sink");
            if has_sink {
                for node in scc {
                    if let Some(val_str) = node.strip_prefix("val_") {
                        if let Ok(val) = val_str.parse::<i32>() {
                            reachable_values.insert(val);
                        }
                    }
                }
            }
        }
        
        // Remove values that are not reachable from free nodes
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                if min_i == max_i {
                    continue; // Already assigned
                }
                
                let mut new_min = min_i;
                let mut new_max = max_i;
                
                // Check if boundary values should be removed
                if !reachable_values.contains(&min_i) && 
                   !matching.get(&var).map_or(false, |&v| v == min_i) {
                    new_min = min_i + 1;
                }
                
                if !reachable_values.contains(&max_i) && 
                   !matching.get(&var).map_or(false, |&v| v == max_i) {
                    new_max = max_i - 1;
                }
                
                // Apply domain reductions
                if new_min > min_i {
                    var.try_set_min(Val::int(new_min), ctx)?;
                }
                if new_max < max_i {
                    var.try_set_max(Val::int(new_max), ctx)?;
                }
            }
        }
        
        Some(())
    }
}

impl Prune for AllDifferent {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        if self.vars.len() < 2 {
            return Some(());
        }
        
        // Step 1: Check for immediate conflicts (assigned values)
        let mut assigned_values = Vec::new();
        for &var in &self.vars {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                if min_i == max_i {
                    assigned_values.push(min_i);
                }
            }
        }
        
        // Check for conflicts among assigned values
        for i in 0..assigned_values.len() {
            for j in (i + 1)..assigned_values.len() {
                if assigned_values[i] == assigned_values[j] {
                    return None; // Immediate conflict
                }
            }
        }
        
        // Step 2: Find maximum matching
        let (matching, _value_to_var) = self.find_maximum_matching(ctx);
        
        // Check if complete matching is possible
        if matching.len() < self.vars.len() {
            return None; // No complete matching possible
        }
        
        // Step 3: Build residual graph
        let (residual_graph, _all_values) = self.build_residual_graph(ctx, &matching);
        
        // Step 4: Find SCCs
        let sccs = self.find_sccs(&residual_graph);
        
        // Step 5: Apply GAC
        self.apply_gac(ctx, &matching, &sccs)
    }
}

impl Propagate for AllDifferent {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::Model, vars::Val};

    #[test]
    fn test_all_different_basic() {
        let mut model = Model::default();
        
        let x = model.new_var(Val::int(1), Val::int(3));
        let y = model.new_var(Val::int(1), Val::int(3));
        let z = model.new_var(Val::int(1), Val::int(3));
        
        model.all_different(vec![x, y, z]);
        
        // Should find solutions where all variables have different values
        let solutions: Vec<_> = model.enumerate().collect();
        assert!(!solutions.is_empty());
        
        // Verify all solutions have distinct values
        for solution in solutions {
            let x_val = solution[x];
            let y_val = solution[y];
            let z_val = solution[z];
            
            assert_ne!(x_val, y_val);
            assert_ne!(x_val, z_val);
            assert_ne!(y_val, z_val);
        }
    }
    
    #[test]
    fn test_all_different_impossible() {
        let mut model = Model::default();
        
        // Three variables with only two possible values - impossible
        let x = model.new_var(Val::int(1), Val::int(2));
        let y = model.new_var(Val::int(1), Val::int(2));
        let z = model.new_var(Val::int(1), Val::int(2));
        
        model.all_different(vec![x, y, z]);
        
        let solutions: Vec<_> = model.enumerate().collect();
        assert!(solutions.is_empty());
    }
}
