use selen::prelude::*;

fn main() {
    println!("=== Type Conversion Constraints Examples ===\n");
    
    // Example 1: int2float - Converting Integer to Float
    println!("📝 Example 1: int2float - Integer to Float Conversion");
    {
        let mut model = Model::default();
        let int_val = model.int(42, 42);
        let float_val = model.float(0.0, 100.0);
        
        // Convert integer to float
        model.int2float(int_val, float_val);
        
        match model.solve() {
            Ok(solution) => {
                let int_result: i32 = solution.get(int_val);
                let float_result: f64 = solution.get(float_val);
                println!("  Input integer: {}", int_result);
                println!("  Output float: {}", float_result);
                println!("  ✓ Successfully converted int to float");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 2: float2int_floor - Floor Operation
    println!("📝 Example 2: float2int_floor - Floor Conversion");
    {
        let mut model = Model::default();
        let float_val = model.float(3.7, 3.7);
        let int_val = model.int(-100, 100);
        
        // Floor the float value
        model.float2int_floor(float_val, int_val);
        
        match model.solve() {
            Ok(solution) => {
                let float_result: f64 = solution.get(float_val);
                let int_result: i32 = solution.get(int_val);
                println!("  Input float: {}", float_result);
                println!("  floor({}) = {}", float_result, int_result);
                assert_eq!(int_result, 3);
                println!("  ✓ Floor operation successful");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 3: float2int_ceil - Ceiling Operation
    println!("📝 Example 3: float2int_ceil - Ceiling Conversion");
    {
        let mut model = Model::default();
        let float_val = model.float(3.2, 3.2);
        let int_val = model.int(-100, 100);
        
        // Ceiling the float value
        model.float2int_ceil(float_val, int_val);
        
        match model.solve() {
            Ok(solution) => {
                let float_result: f64 = solution.get(float_val);
                let int_result: i32 = solution.get(int_val);
                println!("  Input float: {}", float_result);
                println!("  ceil({}) = {}", float_result, int_result);
                assert_eq!(int_result, 4);
                println!("  ✓ Ceiling operation successful");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 4: float2int_round - Rounding Operation
    println!("📝 Example 4: float2int_round - Rounding Conversion");
    {
        let mut model = Model::default();
        let float_val = model.float(3.6, 3.6);
        let int_val = model.int(-100, 100);
        
        // Round the float value
        model.float2int_round(float_val, int_val);
        
        match model.solve() {
            Ok(solution) => {
                let float_result: f64 = solution.get(float_val);
                let int_result: i32 = solution.get(int_val);
                println!("  Input float: {}", float_result);
                println!("  round({}) = {}", float_result, int_result);
                assert_eq!(int_result, 4);
                println!("  ✓ Rounding operation successful");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 5: Negative Numbers
    println!("📝 Example 5: Type Conversions with Negative Numbers");
    {
        let mut model = Model::default();
        let float_val = model.float(-2.7, -2.7);
        let floor_result = model.int(-100, 100);
        let ceil_result = model.int(-100, 100);
        let round_result = model.int(-100, 100);
        
        model.float2int_floor(float_val, floor_result);
        model.float2int_ceil(float_val, ceil_result);
        model.float2int_round(float_val, round_result);
        
        match model.solve() {
            Ok(solution) => {
                let float_v: f64 = solution.get(float_val);
                let floor_v: i32 = solution.get(floor_result);
                let ceil_v: i32 = solution.get(ceil_result);
                let round_v: i32 = solution.get(round_result);
                
                println!("  Input: {}", float_v);
                println!("  floor({}) = {}", float_v, floor_v);
                println!("  ceil({}) = {}", float_v, ceil_v);
                println!("  round({}) = {}", float_v, round_v);
                assert_eq!(floor_v, -3); // floor(-2.7) = -3
                assert_eq!(ceil_v, -2);  // ceil(-2.7) = -2
                assert_eq!(round_v, -3); // round(-2.7) = -3
                println!("  ✓ All negative number conversions correct");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 6: Combined Conversions
    println!("📝 Example 6: Combined Conversions");
    {
        let mut model = Model::default();
        
        // Start with an integer, convert to float, do arithmetic, convert back
        let int_input = model.int(5, 5);
        let as_float = model.float(0.0, 100.0);
        let multiplied = model.float(0.0, 200.0);
        let result = model.int(-100, 100);
        
        // Convert int to float
        model.int2float(int_input, as_float);
        
        // Multiply by 2.5
        let times_2_5 = model.mul(as_float, Val::ValF(2.5));
        model.props.equals(multiplied, times_2_5);
        
        // Round the result back to integer
        model.float2int_round(multiplied, result);
        
        match model.solve() {
            Ok(solution) => {
                let input_val: i32 = solution.get(int_input);
                let float_val: f64 = solution.get(as_float);
                let mult_val: f64 = solution.get(multiplied);
                let result_val: i32 = solution.get(result);
                
                println!("  Input integer: {}", input_val);
                println!("  As float: {}", float_val);
                println!("  Multiplied by 2.5: {}", mult_val);
                println!("  Rounded result: {}", result_val);
                println!("  Calculation: {} × 2.5 = {} → round({}) = {}", 
                         input_val, mult_val, mult_val, result_val);
                println!("  ✓ Combined conversion successful");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!();
    
    // Example 7: Resource Allocation with Fractional Constraints
    println!("📝 Example 7: Resource Allocation with Rounding");
    {
        let mut model = Model::default();
        
        // We have 10.7 units of resource, need to allocate to integer tasks
        let total_resource = model.float(10.7, 10.7);
        let allocated = model.int(0, 20);
        
        // Allocate using floor (conservative - don't over-allocate)
        model.float2int_floor(total_resource, allocated);
        
        match model.solve() {
            Ok(solution) => {
                let resource: f64 = solution.get(total_resource);
                let alloc: i32 = solution.get(allocated);
                
                println!("  Available resource: {} units", resource);
                println!("  Allocated (floor): {} units", alloc);
                println!("  Remaining: {:.1} units", resource - alloc as f64);
                assert_eq!(alloc, 10);
                println!("  ✓ Conservative allocation successful");
            }
            Err(e) => println!("  ❌ No solution: {:?}", e),
        }
    }
    
    println!("\n✅ All type conversion examples completed successfully!");
}
