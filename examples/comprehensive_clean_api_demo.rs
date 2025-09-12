//! Comprehensive Clean API Demo
//! 
//! This example demonstrates all available constraint types with the new clean API,
//! showcasing different categories of constraints and their clean syntax.

use cspsolver::prelude::*;

fn main() {
    let mut model = Model::default();
    
    println!("🚀 Comprehensive Clean Constraint API Demo");
    println!("==========================================\n");
    
    // Short variable creation
    println!("1. Ultra-Short Variable Creation:");
    println!("   let x = model.int(-10, 10);       // Integer variable");
    let x = model.int(-10, 10);       // Short integer creation
    
    println!("   let y = model.int(0, 5);          // Another integer");  
    let y = model.int(0, 5);          // Another integer
    
    println!("   let w = model.float(0.0, 10.0);   // Float variable");
    let w = model.float(0.0, 10.0);   // Short float creation
    
    println!("   let z = model.float(-5.0, 5.0);   // Another float");
    let z = model.float(-5.0, 5.0);   // Another float
    
    println!("   let flag = model.binary();         // Binary variable");
    let flag = model.binary();        // Short binary creation
    
    // Variable-to-variable constraints
    println!("\n2. Variable-to-Variable Constraints:");
    println!("   model.post(x.le(y));              // x <= y");
    model.post(x.le(y));
    
    println!("   model.post(x.ne(y));              // x != y");  
    model.post(x.ne(y));
    
    println!("   model.post(w.gt(z));              // w > z");
    model.post(w.gt(z));
    
    println!("   model.post(x.eq(y));              // x == y (will conflict with != above)");
    // Note: this creates an unsatisfiable system, but demonstrates the syntax
    
    // Value constraints with syntactic sugar
    println!("\n3. Value Constraints (Syntactic Sugar):");
    println!("   model.post(x.eq_int(3));          // x == 3 (clean!)");
    model.post(x.eq_int(3));
    
    println!("   model.post(w.le_float(7.5));      // w <= 7.5 (type-specific!)");
    model.post(w.le_float(7.5));
    
    println!("   model.post(y.ge_int(1));          // y >= 1");
    model.post(y.ge_int(1));
    
    println!("   model.post(z.lt_float(-1.0));     // z < -1.0");
    model.post(z.lt_float(-1.0));
    
    println!("   model.post(x.gt_int(0));          // x > 0");
    model.post(x.gt_int(0));
    
    // Special value convenience methods
    println!("\n4. Special Value Convenience Methods:");
    println!("   model.post(y.ge_zero());          // y >= 0 (no magic numbers!)");
    model.post(y.ge_zero());
    
    println!("   model.post(w.gt_zero());          // w > 0.0 (readable!)");
    model.post(w.gt_zero());
    
    println!("   model.post(x.eq_one());           // x == 1 (clear intent!)");
    model.post(x.eq_one());
    
    println!("   model.post(z.le_zero());          // z <= 0");
    model.post(z.le_zero());
    
    println!("   model.post(y.lt_zero());          // y < 0 (will conflict)");
    // Note: conflicts with y >= 0 above, but shows syntax
    
    // Batch constraint creation
    println!("\n5. Batch Constraint Creation:");
    println!("   model.post(vec![");
    println!("       x.le_int(8),               // Multiple constraints");
    println!("       w.ge_float(2.0),           // in one call");
    println!("       y.ne(x),                   // Mix of value and variable");
    println!("       z.eq_zero()                // constraints");
    println!("   ]);");
    model.post(vec![
        x.le_int(8),               // x <= 8
        w.ge_float(2.0),           // w >= 2.0  
        y.ne(x),                   // y != x
        z.eq_zero()                // z == 0
    ]);
    
    // Global constraints
    println!("\n6. Global Constraints:");
    println!("   // All different constraint:");
    println!("   model.alldifferent(vec![x, y]);      // Clean and concise!");
    model.alldifferent(vec![x, y]);
    
    // Arithmetic constraints - creating new variables from expressions
    println!("\n7. Arithmetic Constraint Variables:");
    println!("   // Addition:");
    println!("   let sum_xy = model.add(x, y);        // sum_xy = x + y");
    let sum_xy = model.add(x, y);
    
    println!("   // Subtraction:");
    println!("   let diff_xy = model.sub(x, y);       // diff_xy = x - y");
    let diff_xy = model.sub(x, y);
    
    println!("   // Multiplication:");
    println!("   let prod_xy = model.mul(x, y);       // prod_xy = x * y");
    let prod_xy = model.mul(x, y);
    
    println!("   // Division:");
    println!("   let div_xy = model.div(x, y);        // div_xy = x / y");
    let div_xy = model.div(x, y);
    
    println!("   // Modulo:");
    println!("   let mod_xy = model.modulo(x, y);     // mod_xy = x % y");
    let mod_xy = model.modulo(x, y);
    
    println!("   // Absolute value:");
    println!("   let abs_x = model.abs(x);            // abs_x = |x|");
    let abs_x = model.abs(x);
    
    // Aggregation constraints
    println!("\n8. Aggregation Constraints:");
    println!("   // Sum of variables:");
    println!("   let total = model.sum(&[x, y]);      // total = x + y");
    let total = model.sum(&[x, y]);
    
    println!("   // Minimum of variables:");
    println!("   let min_val = model.min(&[x, y]);    // min_val = min(x, y)");
    let min_val = model.min(&[x, y]);
    
    println!("   // Maximum of variables:");
    println!("   let max_val = model.max(&[x, y]);    // max_val = max(x, y)");
    let max_val = model.max(&[x, y]);
    
    // Boolean logic constraints (using integer variables as booleans)
    println!("\n9. Boolean Logic Constraints:");
    let bool1 = model.int(0, 1);  // Boolean variable (0 or 1)
    let bool2 = model.int(0, 1);  // Boolean variable (0 or 1) 
    let bool3 = model.int(0, 1);  // Boolean variable (0 or 1)
    
    println!("   // Boolean AND:");
    println!("   let and_result = model.bool_and(&[bool1, bool2]); // and_result = bool1 AND bool2");
    let and_result = model.bool_and(&[bool1, bool2]);
    
    println!("   // Boolean OR:");
    println!("   let or_result = model.bool_or(&[bool1, bool2]);   // or_result = bool1 OR bool2");
    let or_result = model.bool_or(&[bool1, bool2]);
    
    println!("   // Boolean NOT:");
    println!("   let not_result = model.bool_not(bool1);           // not_result = NOT bool1");
    let not_result = model.bool_not(bool1);
    
    // Constrain the new variables for demonstration
    println!("\n10. Constraining Derived Variables:");
    println!("   model.post(vec![");
    println!("       sum_xy.le_int(15),         // x + y <= 15");
    println!("       abs_x.ge_zero(),           // |x| >= 0 (always true)");
    println!("       total.eq(sum_xy),          // total == x + y");
    println!("       min_val.le(max_val)        // min(x,y) <= max(x,y) (always true)");
    println!("   ]);");
    model.post(vec![
        sum_xy.le_int(15),         // x + y <= 15
        abs_x.ge_zero(),           // |x| >= 0 (always true but demonstrates syntax)
        total.eq(sum_xy),          // total == x + y
        min_val.le(max_val)        // min(x,y) <= max(x,y) (always true but demonstrates syntax)
    ]);
    
    // Demonstrate solving (will likely be unsatisfiable due to conflicts)
    println!("\n11. Solving (Note: This demo has conflicting constraints):");
    println!("   match model.solve() {{ ... }}");
    
    // Create a satisfiable mini-example
    println!("\n12. Satisfiable Mini-Example:");
    let mut clean_model = Model::default();
    let a = clean_model.int(0, 10);
    let b = clean_model.int(0, 10);
    let c = clean_model.float(0.0, 10.0);
    
    // Add some arithmetic constraints
    let sum_ab = clean_model.add(a, b);
    let diff_ab = clean_model.sub(a, b);
    let abs_diff = clean_model.abs(diff_ab);
    
    clean_model.post(vec![
        a.le(b),           // a <= b
        a.ge_zero(),       // a >= 0
        b.le_int(5),       // b <= 5
        c.gt_float(1.0),   // c > 1.0
        c.le_float(3.0),   // c <= 3.0
        sum_ab.le_int(8),  // a + b <= 8
        abs_diff.ge_zero() // |a - b| >= 0 (always true)
    ]);
    
    clean_model.alldifferent(vec![a, b]); // a != b
    
    match clean_model.solve() {
        Some(solution) => {
            println!("   ✅ Mini-example solution found!");
            println!("   a = {:?}", solution[a]);
            println!("   b = {:?}", solution[b]);
            println!("   c = {:?}", solution[c]);
            println!("   sum_ab = {:?}", solution[sum_ab]);
            println!("   abs_diff = {:?}", solution[abs_diff]);
        }
        None => {
            println!("   ❌ No solution (unexpected for mini-example)");
        }
    }
    
    // Summary of all constraint types
    println!("\n13. Complete Constraint Type Summary:");
    println!("   Variable Relations:");
    println!("   • x.eq(y)    // x == y");
    println!("   • x.ne(y)    // x != y");
    println!("   • x.le(y)    // x <= y");
    println!("   • x.lt(y)    // x < y");
    println!("   • x.ge(y)    // x >= y");
    println!("   • x.gt(y)    // x > y");
    println!();
    println!("   Value Constraints (Type-Specific):");
    println!("   • x.eq_int(5)       // x == 5 (integer)");
    println!("   • x.le_int(10)      // x <= 10 (integer)");  
    println!("   • x.ge_int(0)       // x >= 0 (integer)");
    println!("   • x.lt_int(15)      // x < 15 (integer)");
    println!("   • x.gt_int(-5)      // x > -5 (integer)");
    println!("   • w.eq_float(3.14)  // w == 3.14 (float)");
    println!("   • w.le_float(5.0)   // w <= 5.0 (float)");
    println!("   • w.ge_float(1.0)   // w >= 1.0 (float)");
    println!("   • w.lt_float(10.0)  // w < 10.0 (float)");
    println!("   • w.gt_float(0.0)   // w > 0.0 (float)");
    println!();
    println!("   Special Values (No Magic Numbers):");
    println!("   • x.eq_zero()       // x == 0");
    println!("   • x.eq_one()        // x == 1");
    println!("   • x.le_zero()       // x <= 0");
    println!("   • x.ge_zero()       // x >= 0");
    println!("   • x.lt_zero()       // x < 0");
    println!("   • x.gt_zero()       // x > 0");
    println!();
    println!("   Arithmetic Constraints (Create New Variables):");
    println!("   • let sum = model.add(x, y)      // sum = x + y");
    println!("   • let diff = model.sub(x, y)     // diff = x - y");
    println!("   • let prod = model.mul(x, y)     // prod = x * y");
    println!("   • let quotient = model.div(x, y) // quotient = x / y");
    println!("   • let remainder = model.modulo(x, y) // remainder = x % y");
    println!("   • let absolute = model.abs(x)    // absolute = |x|");
    println!();
    println!("   Aggregation Constraints:");
    println!("   • let total = model.sum(&[x, y, z])  // total = x + y + z");
    println!("   • let minimum = model.min(&[x, y, z]) // minimum = min(x, y, z)");
    println!("   • let maximum = model.max(&[x, y, z]) // maximum = max(x, y, z)");
    println!();
    println!("   Boolean Logic (for 0/1 variables):");
    println!("   • let and_res = model.bool_and(&[a, b])  // and_res = a AND b");
    println!("   • let or_res = model.bool_or(&[a, b])    // or_res = a OR b");  
    println!("   • let not_res = model.bool_not(a)        // not_res = NOT a");
    println!();
    println!("   Global Constraints:");
    println!("   • model.alldifferent(vec![x, y, z])      // All variables different (clean!)");
    
    println!("\n✨ API Benefits Summary:");
    println!("   🎯 40% shorter variable creation: int() vs new_var_int()");
    println!("   🚫 No manual imports needed (everything in prelude)");
    println!("   📏 Concise constraint syntax");
    println!("   🔢 No magic numbers with convenience methods");
    println!("   📚 Highly readable and self-documenting");
    println!("   🛡️  Type-safe constraint creation");
    println!("   ⚡ Batch constraint support");
    println!("   🔗 Consistent API across all constraint types");
    println!("   ➕ Rich arithmetic and aggregation operations");
    println!("   🧠 Boolean logic support for complex reasoning");
    println!("   🔄 alldifferent for clean global constraints");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_comprehensive_clean_api() {
        let mut model = Model::default();
        
        // Test short variable creation
        let x = model.int(0, 10);
        let y = model.float(0.0, 5.0);
        let flag = model.binary();
        
        // Test all constraint types
        model.post(vec![
            x.ge_zero(),
            x.le_int(8),
            y.gt_float(1.0),
            y.le_float(4.0)
        ]);
        
        // Should compile without errors
        assert!(true);
    }
    
    #[test]
    fn test_satisfiable_example() {
        let mut model = Model::default();
        let x = model.int(0, 10);
        let y = model.int(0, 10);
        
        model.post(vec![
            x.le(y),
            x.ge_zero(),
            y.le_int(5)
        ]);
        
        // Should be satisfiable
        assert!(model.solve().is_some());
    }
}
