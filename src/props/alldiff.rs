use crate::{
    props::{Propagate, Prune},
    vars::VarId,
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
    
    /// Convert constraint solver variables to GAC bipartite graph representation
    fn build_bipartite_graph(&self, ctx: &Context) -> BipartiteGraph {
        let mut graph = BipartiteGraph::new();
        
        for (idx, &var) in self.vars.iter().enumerate() {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (crate::vars::Val::ValI(min_i), crate::vars::Val::ValI(max_i)) = (min_val, max_val) {
                let domain: Vec<i32> = (min_i..=max_i).collect();
                graph.add_variable(Variable(idx), domain);
            }
        }
        
        graph
    }
    
    /// Apply domain reductions from GAC back to constraint solver variables
    fn apply_reductions(&self, ctx: &mut Context, graph: &BipartiteGraph) -> Option<()> {
        for (idx, &var) in self.vars.iter().enumerate() {
            let gac_var = Variable(idx);
            let gac_domain = graph.domain(gac_var);
            
            if gac_domain.is_empty() {
                return None; // Variable has no valid values
            }
            
            // Find the min and max values in the reduced GAC domain
            let gac_values: Vec<i32> = gac_domain.iter().map(|v| v.0).collect();
            let gac_min = *gac_values.iter().min().unwrap();
            let gac_max = *gac_values.iter().max().unwrap();
            
            // Constrain the variable to the GAC domain bounds
            var.try_set_min(crate::vars::Val::ValI(gac_min), ctx)?;
            var.try_set_max(crate::vars::Val::ValI(gac_max), ctx)?;
            
            // Note: We can only constrain the bounds, not remove holes in the middle.
            // This is a limitation of the current constraint solver domain representation.
            // For more precise GAC, we would need support for sparse domain representation.
        }
        
        Some(())
    }
}

impl Prune for AllDifferent {
    fn prune(&mut self, ctx: &mut Context) -> Option<()> {
        // Build bipartite graph from current domains
        let mut graph = self.build_bipartite_graph(ctx);
        
        // Apply AllDiffbit GAC propagation
        if !AllDiffbit::propagate(&mut graph) {
            // No solution possible
            return None;
        }
        
        // Apply the domain reductions back to the constraint solver
        self.apply_reductions(ctx, &graph)
    }
}

impl Propagate for AllDifferent {
    fn list_trigger_vars(&self) -> impl Iterator<Item = VarId> {
        self.vars.iter().copied()
    }
}
