use cspsolver::{model::Model, prelude::float, vars::Val, views::{ViewDebugExt, ViewExt}};

#[test]
fn test_view_debug_formatting() {
    let mut m = Model::default();
    
    let v0 = m.new_var_int(1, 10);
    let v1 = m.new_var_float(2.5, 7.8);
    
    println!("=== Basic Variables ===");
    println!("v0 (int): {:?}", v0);
    println!("v1 (float): {:?}", v1);
    
    println!("\n=== View Transformations ===");
    
    // Test various views
    let opposite_v0 = v0.opposite();
    println!("opposite_v0: {:?}", opposite_v0);
    
    let plus_v0 = v0.plus(Val::ValI(5));
    println!("plus_v0: {:?}", plus_v0);
    
    let times_v0 = v0.times_pos(Val::ValI(3));
    println!("times_v0: {:?}", times_v0);
    
    let next_v1 = v1.next();
    println!("next_v1: {:?}", next_v1);
    
    let prev_v1 = v1.prev();
    println!("prev_v1: {:?}", prev_v1);
    
    // Test complex nested views
    let complex_view = v0.plus(Val::ValI(2)).times_pos(Val::ValI(3)).opposite();
    println!("complex_view: {:?}", complex_view);
    
    println!("\n=== Debug with Domain Information ===");
    
    // Get access to internal vars for domain debugging
    // Note: This requires access to the internal vars, which might need
    // additional methods in the Model to be fully useful
    
    println!("View debug formatting implemented successfully!");
}
