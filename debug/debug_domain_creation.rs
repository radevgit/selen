use cspsolver::prelude::*;

fn main() {
    println!("Debugging domain creation for int(10, 5)...");
    
    let mut model = Model::default();
    let x = model.int(10, 5);
    println!("Variable created: {:?}", x);
    
    // Let's inspect the variable's domain directly
    let vars = &model.vars;
    if let Some(var) = vars.get(x.0) {
        match var {
            Var::VarI(sparse_set) => {
                println!("Domain type: Integer (SparseSet)");
                println!("Is empty: {}", sparse_set.is_empty());
                println!("Universe size: {}", sparse_set.universe_size());
                println!("Min universe value: {}", sparse_set.min_universe_value());
                println!("Max universe value: {}", sparse_set.max_universe_value());
                println!("Domain contents: {:?}", sparse_set.to_vec());
            },
            Var::VarF(interval) => {
                println!("Domain type: Float (Interval)");
                println!("Min: {}", interval.min);
                println!("Max: {}", interval.max);
            },
        }
    }
    
    println!("Running validation...");
    match model.validate() {
        Ok(_) => println!("❌ Validation passed (unexpected)"),
        Err(e) => println!("✅ Validation caught error: {}", e),
    }
}