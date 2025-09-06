use cspsolver::{model::Model, prelude::float, vars::Val};

#[test]
fn test_type_aware_greater_than() {
    // Test the type-aware greater_than method with mixed types
    let mut m = Model::default();

    let v0 = m.new_var_int(1, 10);

    // Mixed constraint: v0 > 2.5 (should result in v0 >= 3)
    println!("Debug: Adding constraint v0 > 2.5");
    println!("Debug: Expected v0 >= 3");
    m.greater_than(v0, float(2.5));

    // Check what domain we have after constraint is added
    // We need to look at the internal state somehow...
    
    println!("Debug: Testing what values are valid now...");
    
    // // Try creating models with fixed values to see what's valid
    // for test_val in 1..=6 {
    //     let mut test_m = Model::default();
    //     let test_v0 = test_m.new_var_int(test_val, test_val); // Fixed value
    //     test_m.greater_than(test_v0, float(2.5));
        
    //     match test_m.solve() {
    //         Some(_) => println!("  v0 = {} is VALID", test_val),
    //         None => println!("  v0 = {} is INVALID", test_val),
    //     }
    // }

    // Try enumerate instead of minimize to see what values are found
    println!("Debug: Enumerating first few solutions:");
    let solutions: Vec<_> = m.enumerate().take(3).collect();
    for (i, solution) in solutions.iter().enumerate() {
        let x = match solution[v0] {
            Val::ValI(int_val) => int_val,
            _ => panic!("Expected integer value"),
        };
        println!("  Solution {}: v0 = {}", i + 1, x);
    }
    
    // Now try minimize
    let mut m2 = Model::default();
    let v0_2 = m2.new_var_int(1, 10);
    m2.greater_than(v0_2, float(2.5));
    
    let solution = m2.minimize(v0_2).unwrap();
    let x = match solution[v0_2] {
        Val::ValI(int_val) => int_val,
        _ => panic!("Expected integer value"),
    };

    println!("Debug: Found x = {}, expected x = 3", x);
    
    // Should find v0 = 3 since v0 > 2.5
    assert_eq!(x, 3);
    println!(
        "Type-aware greater_than constraint correctly found x = {}",
        x
    );
}
