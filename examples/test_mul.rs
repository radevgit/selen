use cspsolver::prelude::*;

fn main() {
    println!("🧮 Testing Multiplication Constraint");
    println!("====================================");
    
    // Test: x * y = z where x=3, y=4, solve for z
    let mut model = Model::default();
    
    let x = model.new_var_int(3, 3); // x = 3
    let y = model.new_var_int(4, 4); // y = 4
    let z = model.mul(x, y);         // z = x * y = 3 * 4 = 12
    
    if let Some(solution) = model.solve() {
        let z_val = match solution[z] {
            Val::ValI(v) => v,
            Val::ValF(v) => v as i32,
        };
        println!("✅ Test 1: 3 * 4 = {}", z_val);
        assert_eq!(z_val, 12);
    } else {
        panic!("❌ Test 1 failed: No solution found");
    }
    
    // Test: x * y = z where z=15, y=3, solve for x
    let mut model2 = Model::default();
    
    let x2 = model2.new_var_int(1, 10);  // x unknown
    let y2 = model2.new_var_int(3, 3);   // y = 3
    let z2 = model2.new_var_int(15, 15); // z = 15
    
    // Create the constraint: x * y = z, so x = z / y = 15 / 3 = 5
    let product = model2.mul(x2, y2);
    model2.equals(product, z2);
    
    if let Some(solution) = model2.solve() {
        let x_val = match solution[x2] {
            Val::ValI(v) => v,
            Val::ValF(v) => v as i32,
        };
        println!("✅ Test 2: {} * 3 = 15", x_val);
        assert_eq!(x_val, 5);
    } else {
        panic!("❌ Test 2 failed: No solution found");
    }
    
    // Test: negative multiplication
    let mut model3 = Model::default();
    
    let x3 = model3.new_var_int(-2, -2); // x = -2
    let y3 = model3.new_var_int(6, 6);   // y = 6
    let z3 = model3.mul(x3, y3);         // z = -2 * 6 = -12
    
    if let Some(solution) = model3.solve() {
        let z_val = match solution[z3] {
            Val::ValI(v) => v,
            Val::ValF(v) => v as i32,
        };
        println!("✅ Test 3: -2 * 6 = {}", z_val);
        assert_eq!(z_val, -12);
    } else {
        panic!("❌ Test 3 failed: No solution found");
    }
    
    println!("🎉 All multiplication tests passed!");
}
