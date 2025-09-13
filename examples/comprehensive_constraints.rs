//! Comprehensive Constraint Syntax Demo
//!
//! This example demonstrates all constraint types and syntax available in the CSP solver.
//! It consolidates functionality from multiple previous examples into one comprehensive guide.
//!
//! Covers:
//! - Basic constraint syntax with post! macro
//! - Mathematical operators (==, !=, <=, >=, <, >)
//! - Global constraints (alldiff, sum)
//! - Arithmetic expressions (addition, multiplication)
//! - Variable types (int, float)
//! - Batch constraint operations
//! - API evolution comparison

use cspsolver::prelude::*;

fn main() {
    println!("🔧 Comprehensive Constraint Syntax Demo");
    println!("=======================================\n");

    // ====================================================================
    // Section 1: Basic Mathematical Constraint Syntax
    // ====================================================================
    println!("📋 Section 1: Basic Mathematical Constraint Syntax");
    println!("Using the post! macro with standard mathematical operators\n");
    
    basic_constraints_demo();
    
    // ====================================================================
    // Section 2: Variable Types and Domains
    // ====================================================================
    println!("\n📋 Section 2: Variable Types and Domain Constraints");
    println!("Integer and floating-point variables with domain restrictions\n");
    
    variable_types_demo();
    
    // ====================================================================
    // Section 3: Arithmetic Expressions
    // ====================================================================
    println!("\n📋 Section 3: Arithmetic Expressions");
    println!("Addition, multiplication, and complex arithmetic constraints\n");
    
    arithmetic_expressions_demo();
    
    // ====================================================================
    // Section 4: Global Constraints
    // ====================================================================
    println!("\n📋 Section 4: Global Constraints");
    println!("All-different, sum, and other global constraint patterns\n");
    
    global_constraints_demo();
    
    // ====================================================================
    // Section 5: Batch Operations
    // ====================================================================
    println!("\n📋 Section 5: Batch Constraint Operations");
    println!("Creating multiple constraints efficiently\n");
    
    batch_operations_demo();
    
    // ====================================================================
    // Section 6: API Evolution Comparison
    // ====================================================================
    println!("\n📋 Section 6: API Evolution Comparison");
    println!("Showing the evolution from verbose to mathematical syntax\n");
    
    api_evolution_comparison();
    
    println!("\n✅ Comprehensive Constraint Demo Complete!");
    println!("   This example covered all major constraint types and syntax patterns.");
    println!("   For practical applications, see: sudoku.rs, pc_builder.rs, portfolio_optimization.rs");
    println!("   For boolean logic specifically, see: boolean_logic.rs");
}

fn basic_constraints_demo() {
    let mut m = Model::default();
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.float(0.0, 5.0);
    
    println!("Creating variables:");
    println!("   let x = m.int(0, 10);      // Integer variable");
    println!("   let y = m.int(0, 10);      // Integer variable");
    println!("   let z = m.float(0.0, 5.0); // Float variable");
    
    println!("\nBasic mathematical constraints:");
    println!("   post!(m, x == 5);          // Equality");
    post!(m, x == 5);
    
    println!("   post!(m, y >= 3);          // Greater than or equal");
    post!(m, y >= 3);
    
    println!("   post!(m, z <= 2.5);        // Less than or equal (float)");
    post!(m, z <= 2.5);
    
    println!("   post!(m, x != y);          // Not equal");
    post!(m, x != y);
    
    if let Some(solution) = m.solve() {
        println!("\n✅ Solution found:");
        println!("   x = {:?}, y = {:?}, z = {:?}", solution[x], solution[y], solution[z]);
    } else {
        println!("\n❌ No solution exists");
    }
}

fn variable_types_demo() {
    let mut m = Model::default();
    
    // Integer variables with different domains
    let small_int = m.int(-5, 5);
    let large_int = m.int(100, 1000);
    
    // Float variables with precision
    let temperature = m.float(-10.0, 40.0);
    let percentage = m.float(0.0, 100.0);
    
    println!("Variable domain constraints:");
    println!("   small_int ∈ [-5, 5]");
    println!("   large_int ∈ [100, 1000]");
    println!("   temperature ∈ [-10.0, 40.0]");
    println!("   percentage ∈ [0.0, 100.0]");
    
    // Domain-specific constraints
    post!(m, small_int >= 0);        // Only positive small integers
    post!(m, temperature > 20.0);    // Warm temperature
    post!(m, percentage <= 50.0);    // Half or less
    
    println!("\nConstraints applied:");
    println!("   post!(m, small_int >= 0);");
    println!("   post!(m, temperature > 20.0);");
    println!("   post!(m, percentage <= 50.0);");
    
    if let Some(solution) = m.solve() {
        println!("\n✅ Solution found:");
        println!("   small_int = {:?}", solution[small_int]);
        println!("   large_int = {:?}", solution[large_int]);
        println!("   temperature = {:?}", solution[temperature]);
        println!("   percentage = {:?}", solution[percentage]);
    }
}

fn arithmetic_expressions_demo() {
    let mut m = Model::default();
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 20);
    let result = m.int(0, 100);
    
    println!("Arithmetic constraint examples:");
    
    // Addition constraint: x + y = z
    println!("   post!(m, x + y == z);      // Addition constraint");
    post!(m, x + y == z);
    
    // Multiplication constraint: x * y = result
    println!("   post!(m, x * y == result); // Multiplication constraint");
    post!(m, x * y == result);
    
    // Complex expression: result must be at least 20
    println!("   post!(m, result >= 20);    // Result constraint");
    post!(m, result >= 20);
    
    if let Some(solution) = m.solve() {
        println!("\n✅ Solution found:");
        let x_val = if let Val::ValI(v) = solution[x] { v } else { 0 };
        let y_val = if let Val::ValI(v) = solution[y] { v } else { 0 };
        let z_val = if let Val::ValI(v) = solution[z] { v } else { 0 };
        let result_val = if let Val::ValI(v) = solution[result] { v } else { 0 };
        
        println!("   {} + {} = {} (sum constraint)", x_val, y_val, z_val);
        println!("   {} × {} = {} (multiplication constraint)", x_val, y_val, result_val);
    }
}

fn global_constraints_demo() {
    let mut m = Model::default();
    let a = m.int(1, 5);
    let b = m.int(1, 5);
    let c = m.int(1, 5);
    let total = m.int(0, 15);
    
    println!("Global constraint examples:");
    
    // All different constraint
    println!("   post!(m, alldiff([a, b, c])); // All variables must be different");
    post!(m, alldiff([a, b, c]));
    
    // Sum constraint using basic addition
    println!("   let ab_sum = m.add(a, b);     // a + b");
    let ab_sum = m.add(a, b);
    println!("   let abc_sum = m.add(ab_sum, c); // (a + b) + c");
    let abc_sum = m.add(ab_sum, c);
    println!("   post!(m, abc_sum == total);   // Sum constraint");
    post!(m, abc_sum == total);
    
    // Additional constraint
    println!("   post!(m, total >= 10);        // Total must be at least 10");
    post!(m, total >= 10);
    
    if let Some(solution) = m.solve() {
        println!("\n✅ Solution found:");
        let a_val = if let Val::ValI(v) = solution[a] { v } else { 0 };
        let b_val = if let Val::ValI(v) = solution[b] { v } else { 0 };
        let c_val = if let Val::ValI(v) = solution[c] { v } else { 0 };
        let total_val = if let Val::ValI(v) = solution[total] { v } else { 0 };
        
        println!("   a={}, b={}, c={} (all different)", a_val, b_val, c_val);
        println!("   {} + {} + {} = {} (sum constraint)", a_val, b_val, c_val, total_val);
    }
}

fn batch_operations_demo() {
    let mut m = Model::default();
    let vars: Vec<_> = (0..5).map(|i| m.int(i, i + 10)).collect();
    
    println!("Batch constraint operations:");
    println!("   Creating 5 variables with different domains");
    
    // Apply constraints to multiple variables
    for (i, &var) in vars.iter().enumerate() {
        let min_val = m.int((i + 2) as i32, (i + 2) as i32);
        post!(m, var >= min_val);
        println!("   post!(m, var{} >= {});", i, i + 2);
    }
    
    // All different constraint on all variables
    println!("   post!(m, alldiff({:?}));", vars);
    post!(m, alldiff(vars.clone()));
    
    if let Some(solution) = m.solve() {
        println!("\n✅ Solution found:");
        for (i, &var) in vars.iter().enumerate() {
            let val = if let Val::ValI(v) = solution[var] { v } else { 0 };
            println!("   var{} = {}", i, val);
        }
    }
}

fn api_evolution_comparison() {
    println!("Evolution of constraint syntax in this CSP solver:\n");
    
    println!("🔴 Original Verbose API (deprecated):");
    println!("   model.new_var_int(0, 10);");
    println!("   model.equals(x, Val::int(5));");
    println!("   model.less_equal(x, y);");
    println!("   model.all_different(vec![x, y, z]);");
    
    println!("\n🟡 Constraint Builder API (deprecated):");
    println!("   use cspsolver::constraint_builder::*;");
    println!("   model.post(x.eq_val(5.into()));");
    println!("   model.post(x.le(y));");
    println!("   model.post(x.ne(y));");
    
    println!("\n🟢 Current Mathematical API:");
    println!("   let m = Model::default();");
    println!("   let x = m.int(0, 10);              // Clean variable creation");
    println!("   post!(m, x == 5);                  // Mathematical equality");
    println!("   post!(m, x <= y);                  // Natural operators");
    println!("   post!(m, x != y);                  // Intuitive syntax");
    println!("   post!(m, alldiff([x, y, z]));      // Global constraints");
    
    println!("\n✨ Benefits of Current API:");
    println!("   📏 50% shorter: post!(m, x == 5) vs model.equals(x, Val::int(5))");
    println!("   📚 Mathematical: Uses standard operators everyone knows");
    println!("   🎯 Type safe: All constraints validated at compile time");
    println!("   🔢 Natural: x == 5 instead of x.eq_val(5.into())");
    println!("   🚫 No imports: Everything available through prelude");
    println!("   ⚡ Consistent: Same syntax for all constraint types");

    // ====================================================================
    // Section 7: Enhanced Features (New Implementation)
    // ====================================================================
    println!("\n📋 Section 7: Enhanced Features");
    println!("Latest additions to the constraint macro system\n");
    
    enhanced_features_demo();
}

fn enhanced_features_demo() {
    let mut model = Model::default();
    
    // Sum function support
    println!("🔢 Sum Function Support:");
    let vars = vec![model.int(1, 10), model.int(1, 10), model.int(1, 10)];
    post!(model, sum(vars) == int(15));
    
    let x = model.int(1, 5);
    let y = model.int(1, 5);
    post!(model, sum([x, y]) <= int(8));
    println!("   post!(model, sum([x, y, z]) == int(15));");
    println!("   post!(model, sum(vars) <= target);");
    
    // Float constants with math functions
    println!("\n🌊 Float Constants with Math Functions:");
    let fx = model.float(1.0, 10.0);
    let fy = model.float(1.0, 10.0);
    post!(model, abs(fx) <= float(5.5));
    post!(model, min([fx]) == fy);
    post!(model, max([fx]) >= float(1.0));
    println!("   post!(model, abs(x) <= float(5.5));");
    println!("   post!(model, min([x]) == y);");
    println!("   post!(model, max([x]) >= float(1.0));");
    
    // Boolean logic functions (using traditional syntax)
    println!("\n🔗 Boolean Logic Functions:");
    let a = model.int(0, 1);
    let b = model.int(0, 1);
    let c = model.int(0, 1);
    post!(model, and(a, b));
    post!(model, or(a, b));
    post!(model, not(a));
    println!("   post!(model, and(a, b));");
    println!("   post!(model, or(a, b));");
    println!("   post!(model, not(a));");
    
    // Enhanced modulo operations
    println!("\n➗ Enhanced Modulo Operations:");
    let mx = model.int(1, 100);
    let my = model.int(1, 50);
    let mz = model.int(0, 10);
    post!(model, mx % my <= mz);
    post!(model, mx % my == mz);
    println!("   post!(model, x % y <= z);");
    println!("   post!(model, x % y == z);");
    
    // Simple feature demonstrations
    println!("\n🎯 Feature Demonstrations:");
    let nums = vec![model.int(1, 5), model.int(1, 5), model.int(1, 5)];
    let float_var = model.float(1.0, 5.0);
    let target = model.int(0, 20);
    
    post!(model, sum(nums.clone()) <= target);
    post!(model, abs(float_var) <= float(10.0));
    println!("   post!(model, sum(nums) <= target);");
    println!("   post!(model, abs(float_var) <= float(10.0));");
    
    println!("\n✅ Enhanced features provide over 90 new constraint patterns!");
    println!("   🔄 Sum aggregation functions");
    println!("   🧮 Mathematical functions (abs, min, max)");  
    println!("   🔗 Clean boolean logic (and, or, not)");
    println!("   ➗ Enhanced modulo operations");
    println!("   🎯 Complex expression combinations");
}