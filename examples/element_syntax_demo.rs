use cspsolver::prelude::*;

fn main() {
    println!("Testing Element constraint with x[y] = z syntax...");
    
    // Test case 1: Basic array[variable] == value syntax
    println!("\n=== Test 1: array[variable] == value syntax ===");
    let mut model = Model::default();
    
    // Create an array: [100, 200, 300]
    let arr = vec![
        model.int(100, 100),
        model.int(200, 200), 
        model.int(300, 300)
    ];
    
    let index = model.int(0, 2);      // Index variable
    let value = model.int(150, 250);  // Value variable
    
    // Using the natural syntax: arr[index] == value
    post!(model, arr[index] == value);
    
    match model.solve() {
        Ok(solution) => {
            let idx_val = solution[index];
            let val_val = solution[value];
            println!("Solution found:");
            println!("  index = {:?}", idx_val);
            println!("  value = {:?}", val_val);
            
            // Should be index=1, value=200 (only overlap in domains)
            if let (Val::ValI(i), Val::ValI(v)) = (idx_val, val_val) {
                if i == 1 && v == 200 {
                    println!("✓ Correct: arr[1] = 200");
                } else {
                    println!("✗ Unexpected values: arr[{}] = {}", i, v);
                }
            }
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    
    // Test case 2: Reverse syntax value == array[variable]
    println!("\n=== Test 2: value == array[variable] syntax ===");
    let mut model2 = Model::default();
    
    let arr2 = vec![
        model2.int(10, 10),
        model2.int(20, 20),
        model2.int(30, 30)
    ];
    
    let idx2 = model2.int(0, 2);
    let val2 = model2.int(25, 35);  // Should constrain to index=2, value=30
    
    // Using reverse syntax: value == arr[index]
    post!(model2, val2 == arr2[idx2]);
    
    match model2.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  index = {:?}", solution[idx2]);
            println!("  value = {:?}", solution[val2]);
            println!("✓ Reverse syntax works");
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    
    // Test case 3: Test with postall! macro
    println!("\n=== Test 3: Element syntax in postall! ===");
    let mut model3 = Model::default();
    
    let a = model3.int(1, 1);
    let b = model3.int(2, 2);
    let c = model3.int(3, 3);
    let array = vec![a, b, c];
    
    let idx = model3.int(0, 2);
    let val = model3.int(1, 3);
    let extra = model3.int(5, 10);
    
    // Test multiple constraints including element syntax
    postall!(model3,
        array[idx] == val,
        extra > int(7),
        val < int(3)
    );
    
    match model3.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  index = {:?}", solution[idx]);
            println!("  value = {:?}", solution[val]);
            println!("  extra = {:?}", solution[extra]);
            println!("✓ Element syntax works in postall!");
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    
    // Test case 4: Multiple element constraints
    println!("\n=== Test 4: Multiple element constraints ===");
    let mut model4 = Model::default();
    
    let arr1 = vec![
        model4.int(10, 10),
        model4.int(20, 20),
        model4.int(30, 30)
    ];
    
    let arr2 = vec![
        model4.int(100, 100),
        model4.int(200, 200),
        model4.int(300, 300)
    ];
    
    let idx1 = model4.int(0, 2);
    let idx2 = model4.int(0, 2);
    let val1 = model4.int(15, 25);  // Should be 20, idx1=1
    let val2 = model4.int(150, 250); // Should be 200, idx2=1
    
    postall!(model4,
        arr1[idx1] == val1,
        arr2[idx2] == val2,
        idx1 == idx2  // Same index for both arrays
    );
    
    match model4.solve() {
        Ok(solution) => {
            println!("Solution found:");
            println!("  idx1 = {:?}, val1 = {:?}", solution[idx1], solution[val1]);
            println!("  idx2 = {:?}, val2 = {:?}", solution[idx2], solution[val2]);
            println!("✓ Multiple element constraints work together");
        }
        Err(_) => {
            println!("No solution found");
        }
    }
}