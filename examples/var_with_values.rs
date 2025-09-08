use cspsolver::prelude::*;

fn main() {
    println!("CSP Solver - Creating Variables with Predefined Values\n");

    // Create a new model
    let mut model = Model::default();

    // Example 1: Variable with even numbers only
    println!("Example 1: Variable with even numbers [2, 4, 6, 8]");
    let even_var = model.new_var_with_values(vec![2, 4, 6, 8]);
    let var = &model[even_var];
    
    match var {
        Var::VarI(sparse_set) => {
            println!("  Variable created with {} possible values", sparse_set.size());
            println!("  Min value: {}", sparse_set.min());
            println!("  Max value: {}", sparse_set.max());
            println!("  Contains 3? {}", sparse_set.contains(3));
            println!("  Contains 4? {}", sparse_set.contains(4));
        }
        _ => println!("  Unexpected variable type"),
    }

    // Example 2: Variable with prime numbers
    println!("\nExample 2: Variable with small prime numbers [2, 3, 5, 7, 11]");
    let prime_var = model.new_var_with_values(vec![2, 3, 5, 7, 11]);
    let var = &model[prime_var];
    
    match var {
        Var::VarI(sparse_set) => {
            println!("  Variable created with {} possible values", sparse_set.size());
            println!("  Min value: {}", sparse_set.min());
            println!("  Max value: {}", sparse_set.max());
            println!("  Midpoint: {:?}", var.mid());
        }
        _ => println!("  Unexpected variable type"),
    }

    // Example 3: Variable with a single value (effectively constant)
    println!("\nExample 3: Variable with single value [42]");
    let const_var = model.new_var_with_values(vec![42]);
    let var = &model[const_var];
    
    match var {
        Var::VarI(sparse_set) => {
            println!("  Variable created with {} possible values", sparse_set.size());
            println!("  Is fixed (assigned)? {}", sparse_set.is_fixed());
            println!("  Value: {}", sparse_set.min());
        }
        _ => println!("  Unexpected variable type"),
    }

    // Example 4: Comparison with range-based variable
    println!("\nExample 4: Comparing with range-based variable creation");
    let range_var = model.new_var_int(1, 5);
    let values_var = model.new_var_with_values(vec![1, 2, 3, 4, 5]);
    
    let range_var_ref = &model[range_var];
    let values_var_ref = &model[values_var];
    
    match (range_var_ref, values_var_ref) {
        (Var::VarI(range_sparse), Var::VarI(values_sparse)) => {
            println!("  Range-based variable size: {}", range_sparse.size());
            println!("  Values-based variable size: {}", values_sparse.size());
            println!("  Both have same domain? {}", 
                range_sparse.size() == values_sparse.size() &&
                range_sparse.min() == values_sparse.min() &&
                range_sparse.max() == values_sparse.max()
            );
        }
        _ => println!("  Unexpected variable types"),
    }

    println!("\nAll examples completed successfully!");
}
