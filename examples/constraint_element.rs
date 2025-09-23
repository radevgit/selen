use selen::prelude::*;

fn main() {
    println!("Testing Element constraint...");
    
    // Test case 1: Simple element constraint with function syntax
    println!("\n=== Test 1: Basic element constraint ===");
    let mut model = Model::default();
    
    // Create an array of variables: [10, 20, 30, 40, 50]
    let a0 = model.int(10, 10);  // Fixed value 10
    let a1 = model.int(20, 20);  // Fixed value 20
    let a2 = model.int(30, 30);  // Fixed value 30
    let a3 = model.int(40, 40);  // Fixed value 40
    let a4 = model.int(50, 50);  // Fixed value 50
    
    // Index and value variables
    let index = model.int(0, 4);    // Index from 0 to 4
    let value = model.int(10, 50);  // Value should match array[index]
    
    // Element constraint: array[index] = value
    post!(model, element([a0, a1, a2, a3, a4], index, value));
    
    match model.solve() {
        Ok(solution) => {
            let index_val = solution[index];
            let value_val = solution[value];
            println!("Solution found:");
            println!("  index = {:?}", index_val);
            println!("  value = {:?}", value_val);
            
            // Verify the constraint
            let expected_values = [10, 20, 30, 40, 50];
            if let Val::ValI(idx) = index_val {
                if let Val::ValI(val) = value_val {
                    let expected = expected_values[idx as usize];
                    if val == expected {
                        println!("✓ Element constraint satisfied: array[{}] = {}", idx, val);
                    } else {
                        println!("✗ Element constraint failed: expected {}, got {}", expected, val);
                    }
                }
            }
        }
        Err(e) => {
            println!("No solution found: {:?}", e);
        }
    }
    
    // Test case 2: Element constraint with specific index
    println!("\n=== Test 2: Element with constrained index ===");
    let mut model2 = Model::default();
    
    let arr0 = model2.int(100, 100);
    let arr1 = model2.int(200, 200);
    let arr2 = model2.int(300, 300);
    
    let idx = model2.int(1, 1);     // Fixed to index 1
    let val = model2.int(150, 250); // Should be constrained to 200
    
    post!(model2, element([arr0, arr1, arr2], idx, val));
    
    match model2.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  index = {:?}", solution[idx]);
            println!("  value = {:?}", solution[val]);
            
            if let Val::ValI(v) = solution[val] {
                if v == 200 {
                    println!("✓ Correctly propagated: array[1] = 200");
                } else {
                    println!("✗ Expected 200, got {}", v);
                }
            }
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    
    // Test case 3: Element constraint with impossible constraint
    println!("\n=== Test 3: Element with impossible constraint ===");
    let mut model3 = Model::default();
    
    let b0 = model3.int(1, 1);
    let b1 = model3.int(2, 2);
    let b2 = model3.int(3, 3);
    
    let idx3 = model3.int(0, 2);
    let val3 = model3.int(10, 20);  // No array element can have this value
    
    post!(model3, element([b0, b1, b2], idx3, val3));
    
    match model3.solve() {
        Ok(_) => {
            println!("✗ Unexpected solution found for impossible constraint");
        }
        Err(_) => {
            println!("✓ No solution found as expected (value not in array)");
        }
    }
    
    // Test case 4: Element with array expression (using a vector)
    println!("\n=== Test 4: Element with array expression ===");
    let mut model4 = Model::default();
    
    let c0 = model4.int(5, 5);
    let c1 = model4.int(15, 15);
    let c2 = model4.int(25, 25);
    let array_vec = vec![c0, c1, c2];
    
    let idx4 = model4.int(0, 2);
    let val4 = model4.int(10, 30);
    
    post!(model4, element(array_vec, idx4, val4));
    
    match model4.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  index = {:?}", solution[idx4]);
            println!("  value = {:?}", solution[val4]);
            println!("✓ Element constraint with array expression works");
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    
    // Test case 5: Multiple solutions - show constraint propagation
    println!("\n=== Test 5: Element constraint propagation ===");
    let mut model5 = Model::default();
    
    let d0 = model5.int(10, 15);  // Variable ranges
    let d1 = model5.int(20, 25);
    let d2 = model5.int(30, 35);
    
    let idx5 = model5.int(0, 2);
    let val5 = model5.int(22, 33);  // Should constrain both index and array elements
    
    post!(model5, element([d0, d1, d2], idx5, val5));
    
    match model5.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  index = {:?}", solution[idx5]);
            println!("  value = {:?}", solution[val5]);
            println!("  array[0] = {:?}", solution[d0]);
            println!("  array[1] = {:?}", solution[d1]);
            println!("  array[2] = {:?}", solution[d2]);
            println!("✓ Element constraint propagation working");
        }
        Err(_) => {
            println!("No solution found");
        }
    }
}