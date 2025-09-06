use cspsolver::{model::Model, prelude::float, vars::Val, utils::float_next};

#[test]
fn test_comprehensive_propagation_trace() {
    println!("=== Comprehensive Propagation Trace ===");
    
    // Test what the issue actually is with a complete trace
    println!("\n1. Testing individual conversions:");
    
    let y = float(2.5);
    println!("y = {:?}", y);
    
    let y_next = y.next();
    println!("y.next() = {:?}", y_next);
    
    if let Val::ValF(next_val) = y_next {
        println!("Next value as f32: {}", next_val);
        println!("Ceiling: {}", next_val.ceil());
        println!("As i32: {}", next_val.ceil() as i32);
    }
    
    println!("\n2. Testing constraint individually:");
    
    // Test the final constraint that should be created: less_than_or_equals(y.next(), v0)
    println!("Testing less_than_or_equals(y.next(), v0) with v0=[3,3]:");
    
    let model_test = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.less_than_or_equals(y_next, v0);
        m.solve()
    };
    
    println!("Result: {}", model_test.is_some());
    
    // Test with slightly different values around the boundary
    println!("\n3. Testing boundary values:");
    
    for test_int in [2, 3, 4] {
        let model_boundary = {
            let mut m = Model::default();
            let v0 = m.new_var_int(test_int, test_int);
            m.less_than_or_equals(y_next, v0);
            m.solve()
        };
        println!("less_than_or_equals(y.next(), {}) = {}", test_int, model_boundary.is_some());
        
        // Also test the mathematical expectation
        if let Val::ValF(next_val) = y_next {
            let should_be_valid = next_val <= test_int as f32;
            println!("  Mathematical: {} <= {} = {}", next_val, test_int, should_be_valid);
            
            if model_boundary.is_some() != should_be_valid {
                println!("  ❌ MISMATCH!");
            } else {
                println!("  ✅ Correct");
            }
        }
    }
    
    println!("\n4. Testing the actual greater_than chain:");
    
    // Test each step of the chain
    println!("Step 1: greater_than(v0, y) -> greater_than_or_equals(v0, y.next())");
    let step1 = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.greater_than_or_equals(v0, y_next);
        m.solve()
    };
    println!("greater_than_or_equals(3, y.next()) = {}", step1.is_some());
    
    println!("Step 2: greater_than_or_equals(v0, y.next()) -> less_than_or_equals(y.next(), v0)");
    let step2 = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.less_than_or_equals(y_next, v0);
        m.solve()
    };
    println!("less_than_or_equals(y.next(), 3) = {}", step2.is_some());
    
    println!("Step 3: Full greater_than(v0, y)");
    let step3 = {
        let mut m = Model::default();
        let v0 = m.new_var_int(3, 3);
        m.greater_than(v0, y);
        m.solve()
    };
    println!("greater_than(3, y) = {}", step3.is_some());
    
    // The mathematical expectation
    if let Val::ValF(next_val) = y_next {
        println!("\n5. Mathematical verification:");
        println!("Original: 3 > 2.5 = {}", 3.0 > 2.5);
        println!("Via next: 3 >= {} = {}", next_val, 3.0 >= next_val);
        println!("Via less_than_or_equals: {} <= 3 = {}", next_val, next_val <= 3.0);
    }
}
