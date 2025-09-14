use cspsolver::prelude::*;

fn main() {
    println!("🧮 Multiplication Constraint Demo");
    println!("=================================");
    
    // Test: x * y = z where x=3, y=4, solve for z
    let mut m = Model::default();
    
    let x = m.int(3, 3); // x = 3 (constant for mul operation)
    let y = m.int(4, 4); // y = 4 (constant for mul operation)
    let z = m.mul(x, y);         // z = x * y = 3 * 4 = 12
    
    if let Some(solution) = m.solve() {
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
    let mut m2 = Model::default();
    
    let x2 = m2.int(1, 10);  // x unknown
    let y2 = m2.int(3, 3);   // y = 3 (constant for mul operation)
    
    // Create the constraint: x * y = 15, so x = 15 / 3 = 5
    let product = m2.mul(x2, y2);
    post!(m2, product == int(15));
    
    if let Some(solution) = m2.solve() {
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
    
    let x3 = model3.int(-2, -2); // x = -2 (constant for mul operation)
    let y3 = model3.int(6, 6);   // y = 6 (constant for mul operation)
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
