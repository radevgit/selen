use cspsolver::prelude::*;

/// Demonstrates the value-based branching strategy for float variables
/// This shows how value assignment can reduce search tree size compared to domain splitting
fn main() {
    println!("🔬 Value-Based Branching for Float Variables");
    println!("===========================================");
    
    // Test 1: Basic float constraint problem
    println!("\n📋 Test 1: Simple float not-equals constraint");
    test_simple_float_neq();
    
    // Test 2: Multiple float variables 
    println!("\n📋 Test 2: Multiple float variables with constraints");
    test_multiple_float_vars();
    
    // Test 3: Mixed integer and float variables
    println!("\n📋 Test 3: Mixed integer and float variables");
    test_mixed_variable_types();
    
    // Test 4: Performance comparison hint
    println!("\n📋 Test 4: Search efficiency observation");
    test_search_efficiency();
}

fn test_simple_float_neq() {
    let mut m = Model::default();
    
    let x = m.new_var(Val::ValF(0.0), Val::ValF(1.0));
    let y = m.new_var(Val::ValF(0.0), Val::ValF(1.0));
    
    // Constraint: x ≠ y
    m.not_equals(x, y);
    
    println!("Variables: x ∈ [0.0, 1.0], y ∈ [0.0, 1.0]");
    println!("Constraint: x ≠ y");
    
    // Find a solution
    match m.solve() {
        Some(solution) => {
            let x_val = solution[x];
            let y_val = solution[y];
            println!("✅ Solution found: x = {:?}, y = {:?}", x_val, y_val);
            
            if let (Val::ValF(x_f), Val::ValF(y_f)) = (x_val, y_val) {
                let diff = (x_f - y_f).abs();
                println!("   Difference: |{} - {}| = {}", x_f, y_f, diff);
            }
        }
        None => println!("❌ No solution found"),
    }
}

fn test_multiple_float_vars() {
    let mut m = Model::default();
    
    let x = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
    let y = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
    let z = m.new_var(Val::ValF(1.0), Val::ValF(3.0));
    
    // Constraints: all different
    m.not_equals(x, y);
    m.not_equals(x, z);
    m.not_equals(y, z);
    
    println!("Variables: x, y, z ∈ [1.0, 3.0]");
    println!("Constraints: x ≠ y, x ≠ z, y ≠ z");
    
    match m.solve() {
        Some(solution) => {
            let x_val = solution[x];
            let y_val = solution[y];
            let z_val = solution[z];
            println!("✅ Solution found:");
            println!("   x = {:?}", x_val);
            println!("   y = {:?}", y_val);
            println!("   z = {:?}", z_val);
            
            // Verify constraints
            if x_val != y_val && x_val != z_val && y_val != z_val {
                println!("   ✓ All constraints satisfied");
            } else {
                println!("   ⚠️ Constraint violation detected");
            }
        }
        None => println!("❌ No solution found"),
    }
}

fn test_mixed_variable_types() {
    let mut m = Model::default();
    
    // Mix of integer and float variables
    let int_var = m.new_var(Val::ValI(1), Val::ValI(5));
    let float_var = m.new_var(Val::ValF(1.0), Val::ValF(5.0));
    
    // Constraint: they should be different
    m.not_equals(int_var, float_var);
    
    // Additional constraint: float should be greater than int
    m.greater_than(float_var, int_var);
    
    println!("Variables: int_var ∈ [1, 5], float_var ∈ [1.0, 5.0]");
    println!("Constraints: int_var ≠ float_var, float_var > int_var");
    
    match m.solve() {
        Some(solution) => {
            let int_val = solution[int_var];
            let float_val = solution[float_var];
            println!("✅ Solution found:");
            println!("   int_var = {:?}", int_val);
            println!("   float_var = {:?}", float_val);
            
            // Check constraint satisfaction
            if int_val != float_val {
                println!("   ✓ Variables are different");
            }
            if float_val > int_val {
                println!("   ✓ float_var > int_var");
            }
        }
        None => println!("❌ No solution found"),
    }
}

fn test_search_efficiency() {
    println!("🔍 Analyzing search behavior with float variables:");
    println!();
    
    // Create a model with float variables that would traditionally create many splits
    let mut m = Model::default();
    
    let x = m.new_var(Val::ValF(0.0), Val::ValF(10.0));
    let y = m.new_var(Val::ValF(0.0), Val::ValF(10.0));
    
    // Simple constraint that should be easy to solve
    m.less_than(x, y);
    
    println!("Variables: x, y ∈ [0.0, 10.0]");
    println!("Constraint: x < y");
    println!();
    println!("💡 Benefits of Value-Based Branching for floats:");
    println!("   • Reduces search tree size by making direct assignments");
    println!("   • Avoids creating many narrow float intervals");
    println!("   • Works well with ULP-based float equality");
    println!("   • Preserves traditional domain splitting for integers");
    
    match m.solve() {
        Some(solution) => {
            let x_val = solution[x];
            let y_val = solution[y];
            println!();
            println!("✅ Example solution: x = {:?}, y = {:?}", x_val, y_val);
            
            if x_val < y_val {
                println!("   ✓ Constraint x < y satisfied");
            }
        }
        None => println!("❌ No solution found"),
    }
    
    println!();
    println!("🎯 Next Steps:");
    println!("   • The hybrid branching strategy automatically chooses:");
    println!("     - Value assignment for float variables");
    println!("     - Domain splitting for integer variables");
    println!("   • This reduces splits for float problems while preserving");
    println!("     efficiency for integer problems");
}
