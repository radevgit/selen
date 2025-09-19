use cspsolver::prelude::*;

fn main() {
    println!("🎯 Automatic Type Inference Demo");
    println!("================================");
    
    // Create a model with mixed variable types
    let mut model = Model::default();
    let x = model.int(1, 10);      // Integer variable  
    let y = model.float(0.0, 5.0); // Float variable
    
    // Add constraints
    model.post(x.ge(int(6)));
    model.post(y.le(float(3.0)));
    model.post(x.add(y).le(float(12.0)));
    
    match model.solve() {
        Ok(solution) => {
            println!("\n✅ Solution found!");
            
            println!("\n🔧 OLD WAY (explicit methods):");
            let x_old = solution.get_int(x);
            let y_old = solution.get_float(y);
            println!("  solution.get_int(x)   = {}", x_old);
            println!("  solution.get_float(y) = {}", y_old);
            
            println!("\n✨ NEW WAY (automatic type inference):");
            
            // Method 1: Type annotation inference
            let x_inferred: i32 = solution.get(x);
            let y_inferred: f64 = solution.get(y);
            println!("  let x: i32 = solution.get(x)  → {}", x_inferred);
            println!("  let y: f64 = solution.get(y)  → {}", y_inferred);
            
            // Method 2: Function parameter inference
            fn double_int(val: i32) -> i32 { val * 2 }
            fn square_float(val: f64) -> f64 { val * val }
            
            let x_doubled = double_int(solution.get(x));
            let y_squared = square_float(solution.get(y));
            println!("  double_int(solution.get(x))   → {}", x_doubled);
            println!("  square_float(solution.get(y)) → {}", y_squared);
            
            // Method 3: Option type inference
            let x_opt: Option<i32> = solution.get(x);
            let y_opt: Option<f64> = solution.get(y);
            println!("  let x_opt: Option<i32>        → {:?}", x_opt);
            println!("  let y_opt: Option<f64>        → {:?}", y_opt);
            
            // Method 4: Assignment to typed variables (works!)
            let int_value: i32 = solution.get(x);      // Clear type annotation
            let float_value: f64 = solution.get(y);    // Clear type annotation
            
            let mut int_accumulator: i32 = 0;
            let mut float_accumulator: f64 = 0.0;
            
            int_accumulator += int_value;
            float_accumulator += float_value;
            
            println!("  let int_value: i32 = solution.get(x)     → {}", int_value);
            println!("  let float_value: f64 = solution.get(y)   → {}", float_value);
            println!("  int_accumulator                          → {}", int_accumulator);
            println!("  float_accumulator                        → {}", float_accumulator);
            
            // Verify all values are consistent
            assert_eq!(x_inferred, x_old);
            assert_eq!(y_inferred, y_old);
            assert_eq!(x_opt, Some(x_old));
            assert_eq!(y_opt, Some(y_old));
            assert_eq!(int_value, x_old);
            assert_eq!(float_value, y_old);
            
            println!("\n🎉 Success! Type inference eliminates the need to specify types!");
            println!("   🚫 No more .get_int() and .get_float()");
            println!("   ✅ Just use .get() and let Rust infer the type!");
            
        }
        Err(e) => {
            println!("❌ No solution found: {:?}", e);
        }
    }
}