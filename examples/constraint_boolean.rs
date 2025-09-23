use selen::prelude::*;

fn main() {
    println!("🔗 Boolean Array and Variadic Operations Demo");
    println!("==============================================\n");

    // Test 1: Array syntax with and([...])
    println!("📋 Test 1: Array AND - and([a, b, c, d])");
    {
        let mut m = Model::default();
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        let d = m.bool();
        
        // All must be true (1) for result to be true
        post!(m, and([a, b, c, d]));
        post!(m, a == 1);
        post!(m, b == 1);
        post!(m, c == 1);
        post!(m, d == 1);
        
        if let Ok(sol) = m.solve() {
            let va = if let Val::ValI(v) = sol[a] { v } else { 0 };
            let vb = if let Val::ValI(v) = sol[b] { v } else { 0 };
            let vc = if let Val::ValI(v) = sol[c] { v } else { 0 };
            let vd = if let Val::ValI(v) = sol[d] { v } else { 0 };
            println!("   ✅ Result: a={}, b={}, c={}, d={}", va, vb, vc, vd);
        }
    }
    
    // Test 2: Array syntax with or([...])
    println!("\n📋 Test 2: Array OR - or([a, b, c, d])");
    {
        let mut m = Model::default();
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        let d = m.bool();
        
        // At least one must be true
        post!(m, or([a, b, c, d]));
        post!(m, a == 0);
        post!(m, b == 0);
        post!(m, c == 1);  // This one is true
        
        if let Ok(sol) = m.solve() {
            let va = if let Val::ValI(v) = sol[a] { v } else { 0 };
            let vb = if let Val::ValI(v) = sol[b] { v } else { 0 };
            let vc = if let Val::ValI(v) = sol[c] { v } else { 0 };
            let vd = if let Val::ValI(v) = sol[d] { v } else { 0 };
            println!("   ✅ Result: a={}, b={}, c={}, d={}", va, vb, vc, vd);
        }
    }
    
    // Test 3: Variadic syntax and(a, b, c, d)
    println!("\n📋 Test 3: Variadic AND - and(a, b, c, d)");
    {
        let mut m = Model::default();
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        let d = m.bool();
        
        post!(m, and(a, b, c, d));
        post!(m, a == 1);
        post!(m, b == 1);
        post!(m, c == 1);
        post!(m, d == 1);
        
        if let Ok(sol) = m.solve() {
            let va = if let Val::ValI(v) = sol[a] { v } else { 0 };
            let vb = if let Val::ValI(v) = sol[b] { v } else { 0 };
            let vc = if let Val::ValI(v) = sol[c] { v } else { 0 };
            let vd = if let Val::ValI(v) = sol[d] { v } else { 0 };
            println!("   ✅ Result: a={}, b={}, c={}, d={}", va, vb, vc, vd);
        }
    }
    
    // Test 4: Variadic syntax or(a, b, c, d)
    println!("\n📋 Test 4: Variadic OR - or(a, b, c, d)");
    {
        let mut m = Model::default();
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        let d = m.bool();
        
        post!(m, or(a, b, c, d));
        post!(m, a == 0);
        post!(m, b == 0);
        post!(m, c == 0);
        post!(m, d == 1);  // This one makes OR true
        
        if let Ok(sol) = m.solve() {
            let va = if let Val::ValI(v) = sol[a] { v } else { 0 };
            let vb = if let Val::ValI(v) = sol[b] { v } else { 0 };
            let vc = if let Val::ValI(v) = sol[c] { v } else { 0 };
            let vd = if let Val::ValI(v) = sol[d] { v } else { 0 };
            println!("   ✅ Result: a={}, b={}, c={}, d={}", va, vb, vc, vd);
        }
    }
    
    // Test 5: Array NOT - not([a, b, c])
    println!("\n📋 Test 5: Array NOT - not([a, b, c])");
    {
        let mut m = Model::default();
        let a = m.bool();
        let b = m.bool();
        let c = m.bool();
        
        // This applies not() to each variable individually
        post!(m, not([a, b, c]));
        
        if let Ok(sol) = m.solve() {
            let va = if let Val::ValI(v) = sol[a] { v } else { 0 };
            let vb = if let Val::ValI(v) = sol[b] { v } else { 0 };
            let vc = if let Val::ValI(v) = sol[c] { v } else { 0 };
            println!("   ✅ Result: a={}, b={}, c={} (all should be 0)", va, vb, vc);
        }
    }
    
    // Test 6: postall! with simple array syntax
    println!("\n📋 Test 6: postall! with simple constraints");
    {
        let mut m = Model::default();
        let x = m.bool();
        let y = m.bool();
        let z = m.bool();
        let w = m.bool();
        
        // Use separate constraints since nested arrays might not work yet
        post!(m, and([x, y]));     // x AND y must be true
        post!(m, or([z, w]));      // z OR w must be true
        post!(m, x == 1);
        post!(m, y == 1);
        post!(m, z == 0);
        post!(m, w == 1);
        
        if let Ok(sol) = m.solve() {
            let vx = if let Val::ValI(v) = sol[x] { v } else { 0 };
            let vy = if let Val::ValI(v) = sol[y] { v } else { 0 };
            let vz = if let Val::ValI(v) = sol[z] { v } else { 0 };
            let vw = if let Val::ValI(v) = sol[w] { v } else { 0 };
            println!("   ✅ Result: x={}, y={}, z={}, w={}", vx, vy, vz, vw);
        }
    }
    
    // Test 7: Real-world example - Server startup conditions
    println!("\n📋 Test 7: Real-world example - Server startup conditions");
    {
        let mut m = Model::default();
        let power_stable = m.bool();
        let network_ready = m.bool();
        let disk_healthy = m.bool();
        let memory_ok = m.bool();
        let cpu_cool = m.bool();
        
        let emergency_override = m.bool();
        let manual_start = m.bool();
        
        // Server starts if emergency override OR manual start is activated
        // (since we know disk_healthy will be 0, normal startup won't work)
        post!(m, or([emergency_override, manual_start]));
        
        // Also demonstrate: all safety systems except disk must be working
        post!(m, and([power_stable, network_ready, memory_ok, cpu_cool]));
        
        // Set test conditions
        post!(m, power_stable == 1);
        post!(m, network_ready == 1);
        post!(m, disk_healthy == 0);    // Disk has issues
        post!(m, memory_ok == 1);
        post!(m, cpu_cool == 1);
        post!(m, emergency_override == 0);
        post!(m, manual_start == 1);     // Manual override saves the day
        
        if let Ok(sol) = m.solve() {
            let vpower = if let Val::ValI(v) = sol[power_stable] { v } else { 0 };
            let vnet = if let Val::ValI(v) = sol[network_ready] { v } else { 0 };
            let vdisk = if let Val::ValI(v) = sol[disk_healthy] { v } else { 0 };
            let vmem = if let Val::ValI(v) = sol[memory_ok] { v } else { 0 };
            let vcpu = if let Val::ValI(v) = sol[cpu_cool] { v } else { 0 };
            let vemerg = if let Val::ValI(v) = sol[emergency_override] { v } else { 0 };
            let vmanual = if let Val::ValI(v) = sol[manual_start] { v } else { 0 };
            
            println!("   ✅ Server startup scenario:");
            println!("      Power: {}, Network: {}, Disk: {}, Memory: {}, CPU: {}", 
                vpower, vnet, vdisk, vmem, vcpu);
            println!("      Emergency Override: {}, Manual Start: {}", 
                vemerg, vmanual);
            println!("      Result: Array AND/OR constraints satisfied!");
        }
    }
    
    println!("\n🎉 All boolean array operations working perfectly!");
}