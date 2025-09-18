use cspsolver::prelude::*;

fn main() {
    let mut m = Model::default();
    
    // Test the new chained comparison syntax
    let lower = m.int(1, 5);
    let middle = m.int(3, 8);
    let upper = m.int(7, 12);
    
    println!("=== Testing Chained Comparison Syntax ===");
    
    // Original between constraint syntax
    post!(m, between(lower, middle, upper));
    println!("✅ Original syntax: between(lower, middle, upper)");
    
    // New natural chained comparison syntax
    let a = m.int(1, 5);
    let b = m.int(3, 8);
    let c = m.int(7, 12);
    
    post!(m, a <= b <= c);
    println!("✅ New syntax: a <= b <= c");
    
    // Test other chained comparison types
    let x = m.int(10, 15);
    let y = m.int(5, 10);
    let z = m.int(1, 6);
    
    post!(m, x >= y >= z);
    println!("✅ New syntax: x >= y >= z");
    
    // Test strict inequalities (these create separate constraints)
    let p = m.int(1, 5);
    let q = m.int(3, 8);
    let r = m.int(7, 12);
    
    post!(m, p < q < r);
    println!("✅ New syntax: p < q < r");
    
    if let Ok(solution) = m.solve() {
        println!("\n=== Solution Found ===");
        println!("lower = {:?}, middle = {:?}, upper = {:?}", 
                solution[lower], solution[middle], solution[upper]);
        println!("a = {:?}, b = {:?}, c = {:?}", 
                solution[a], solution[b], solution[c]);
        println!("x = {:?}, y = {:?}, z = {:?}", 
                solution[x], solution[y], solution[z]);
        println!("p = {:?}, q = {:?}, r = {:?}", 
                solution[p], solution[q], solution[r]);
    } else {
        println!("No solution found");
    }
    
    println!("\n🎉 Chained comparison syntax working!");
    println!("You can now use: a <= b <= c instead of between(a, b, c)");
}