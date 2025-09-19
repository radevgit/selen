use cspsolver::prelude::*;

fn main() {
    println!("🎯 Clean API Demo - No More Ugly Unwraps!");
    
    // Create a simple model
    let mut model = Model::default();
    let x = model.int(1, 10);
    let y = model.int(1, 10);
    
    // Add some constraints
    model.post(x.ge(int(5)));
    model.post(y.le(int(8)));
    model.post(x.add(y).eq(int(12)));
    
    // Solve the model
    match model.solve() {
        Ok(solution) => {
            println!("\n✅ Solution found!");
            
            // OLD UGLY WAY:
            // let x_val = solution.get_value(x).as_int().unwrap();
            // let y_val = solution.get_value(y).as_int().unwrap();
            
            // NEW CLEAN WAYS:
            
            // Method 1: Direct methods (cleanest!)
            let x_val = solution.get_int(x);
            let y_val = solution.get_int(y);
            println!("Method 1 (direct): x = {}, y = {}", x_val, y_val);
            
            // Method 2: Indexing syntax
            let x_val2 = solution[x].as_int().unwrap();
            let y_val2 = solution[y].as_int().unwrap();
            println!("Method 2 (indexing): x = {}, y = {}", x_val2, y_val2);
            
            // Method 3: Safe methods (for when you're not sure)
            let x_val3 = solution.try_get_int(x).unwrap();
            let y_val3 = solution.try_get_int(y).unwrap();
            println!("Method 3 (safe): x = {}, y = {}", x_val3, y_val3);
            
            // Verify the solution
            assert_eq!(x_val, x_val2);
            assert_eq!(x_val, x_val3);
            assert!(x_val >= 5);
            assert!(y_val <= 8);
            assert_eq!(x_val + y_val, 12);
            
            println!("\n🎉 All three approaches work and are much cleaner!");
            println!("    No more .as_int().unwrap() chains!");
        }
        Err(e) => {
            println!("❌ No solution found: {:?}", e);
        }
    }
}