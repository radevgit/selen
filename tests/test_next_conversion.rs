use cspsolver::{vars::Val, prelude::float, views::{ViewExt}};

#[test]
fn debug_next_conversion() {
    println!("=== Debug Next Value Conversion ===");
    
    let y = float(2.5);
    let y_next = y.next();
    
    println!("y = {:?}", y);
    println!("y.next() = {:?}", y_next);
    
    // Test what the next value actually is
    if let Val::ValF(next_float) = y.next() {
        println!("Next float value: {}", next_float);
        println!("Ceiling of next: {}", next_float.ceil());
        println!("Ceiling as int: {}", next_float.ceil() as i32);
        
        // Test the conversion logic from Context::try_set_min
        // When setting integer var min to float value
        let converted_min = next_float.ceil() as i32;
        println!("Converted min for integer: {}", converted_min);
        
        // The issue: this should be 3, but let's see what it actually is
        if converted_min != 3 {
            println!("❌ BUG FOUND: converted_min = {}, expected = 3", converted_min);
            println!("   The next() value is too large!");
        } else {
            println!("✅ Conversion is correct");
        }
    }
    
    // Test the ULP function directly
    use cspsolver::utils::float_next;
    let manual_next = float_next(2.5);
    println!("Manual next: {}", manual_next);
    println!("Manual next ceiling: {}", manual_next.ceil() as i32);
}
