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
    
    /// Convert constraint solver variables to GAC bipartite graph representation
    fn build_bipartite_graph(&self, ctx: &Context) -> BipartiteGraph {
        let mut graph = BipartiteGraph::new();
        
        for (idx, &var) in self.vars.iter().enumerate() {
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                let domain: Vec<i32> = (min_i..=max_i).collect();
                graph.add_variable(Variable(idx), domain);
            }
        }
        
        graph
    }
    
    /// Apply domain reductions from GAC back to constraint solver variables
    fn apply_reductions(&self, ctx: &mut Context, graph: &BipartiteGraph) -> Prune {
        let mut changed = false;
        
        for (idx, &var) in self.vars.iter().enumerate() {
            let gac_var = Variable(idx);
            let gac_domain = graph.domain(gac_var);
            
            // Get current domain from constraint solver
            let min_val = var.min(ctx);
            let max_val = var.max(ctx);
            
            if let (Val::ValI(min_i), Val::ValI(max_i)) = (min_val, max_val) {
                // Check which values to remove
                for val in min_i..=max_i {
                    let gac_val = crate::gac::Value(val);
                    if !gac_domain.contains(&gac_val) {
                        // This value was removed by GAC, remove it from the constraint solver
                        if var.remove(ctx, val) {
                            changed = true;
                        }
                    }
                }
            }
        }
        
        if changed {
            Prune::Changed
        } else {
            Prune::Unchanged
        }
    }
}

impl Propagate for AllDifferent {
    fn propagate(&self, ctx: &mut Context) -> Prune {
        // Build bipartite graph from current domains
        let mut graph = self.build_bipartite_graph(ctx);
        
        // Apply AllDiffbit GAC propagation
        if !AllDiffbit::propagate(&mut graph) {
            // No solution possible
            return Prune::Failed;
        }
        
        // Apply the domain reductions back to the constraint solver
        self.apply_reductions(ctx, &graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vars::{IntVar, IntoVar};
    use crate::model::Model;

    #[test]
    fn test_alldiff_basic() {
        let mut model = Model::new();
        
        let x = model.new_var(1, 3);
        let y = model.new_var(1, 3);
        let z = model.new_var(1, 3);
        
        let alldiff = AllDifferent::new(vec![x.into(), y.into(), z.into()]);
        
        // Initial propagation
        let mut ctx = model.context();
        let result = alldiff.propagate(&mut ctx);
        
        // Should succeed but may not change anything initially
        assert!(result != Prune::Failed);
    }
    
    #[test]
    fn test_alldiff_impossible() {
        let mut model = Model::new();
        
        // 3 variables with only 2 possible values - impossible
        let x = model.new_var(1, 2);
        let y = model.new_var(1, 2);
        let z = model.new_var(1, 2);
        
        let alldiff = AllDifferent::new(vec![x.into(), y.into(), z.into()]);
        
        let mut ctx = model.context();
        let result = alldiff.propagate(&mut ctx);
        
        // Should detect impossibility
        assert_eq!(result, Prune::Failed);
    }
    
    #[test]
    fn test_alldiff_propagation() {
        let mut model = Model::new();
        
        let x = model.new_var(1, 1); // Fixed to 1
        let y = model.new_var(1, 3);
        let z = model.new_var(1, 3);
        
        let alldiff = AllDifferent::new(vec![x.into(), y.into(), z.into()]);
        
        let mut ctx = model.context();
        let result = alldiff.propagate(&mut ctx);
        
        // Should succeed and potentially reduce domains
        assert!(result != Prune::Failed);
        
        println!("After AllDiff propagation:");
        println!("x domain: [{}, {}]", x.min(&ctx).as_int(), x.max(&ctx).as_int());
        println!("y domain: [{}, {}]", y.min(&ctx).as_int(), y.max(&ctx).as_int());
        println!("z domain: [{}, {}]", z.min(&ctx).as_int(), z.max(&ctx).as_int());
    }
}
