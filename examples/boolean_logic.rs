use cspsolver::prelude::*;

/// Boolean Logic Constraints Example
/// 
/// This example demonstrates how to use boolean logic constraints (AND, OR, NOT)
/// to model logical relationships in constraint satisfaction problems.

fn main() {
    println!("🔧 Boolean Logic Constraints Example");
    println!("====================================\n");

    // Example 1: Security System Logic
    security_system_example();
    
    // Example 2: Smart Home Automation
    smart_home_example();
    
    // Example 3: Logic Circuit Simulation
    logic_circuit_example();
}

fn security_system_example() {
    println!("📋 Example 1: Security System Logic");
    println!("Problem: Access granted only if user has badge AND correct PIN");
    
    let mut model = Model::default();
    
    // Create boolean variables (0 = false, 1 = true)
    let has_badge = model.new_var_int(0, 1);
    let correct_pin = model.new_var_int(0, 1);
    let access_granted = model.bool_and(&[has_badge, correct_pin]);
    
    // Test scenario: User has badge but wrong PIN
    model.eq(has_badge, Val::int(1));
    model.eq(correct_pin, Val::int(0));
    
    if let Some(solution) = model.solve() {
        let badge = if let Val::ValI(v) = solution[has_badge] { v } else { 0 };
        let pin = if let Val::ValI(v) = solution[correct_pin] { v } else { 0 };
        let access = if let Val::ValI(v) = solution[access_granted] { v } else { 0 };
        
        println!("  Badge: {}, PIN: {}, Access Granted: {}", badge, pin, access);
        println!("  Result: Access {} (requires both badge AND correct PIN)\n", 
                 if access == 1 { "GRANTED" } else { "DENIED" });
    }
}

fn smart_home_example() {
    println!("📋 Example 2: Smart Home Automation");
    println!("Problem: Security alarm = (motion_detected AND night_mode) OR manual_panic");
    
    let mut model = Model::default();
    
    let motion_detected = model.new_var_int(0, 1);
    let night_mode = model.new_var_int(0, 1);
    let manual_panic = model.new_var_int(0, 1);
    
    // Build logical expression: (motion AND night) OR panic
    let motion_and_night = model.bool_and(&[motion_detected, night_mode]);
    let alarm_triggered = model.bool_or(&[motion_and_night, manual_panic]);
    
    // Test scenario: Motion during day, no panic button
    model.eq(motion_detected, Val::int(1));
    model.eq(night_mode, Val::int(0));
    model.eq(manual_panic, Val::int(0));
    
    if let Some(solution) = model.solve() {
        let motion = if let Val::ValI(v) = solution[motion_detected] { v } else { 0 };
        let night = if let Val::ValI(v) = solution[night_mode] { v } else { 0 };
        let panic = if let Val::ValI(v) = solution[manual_panic] { v } else { 0 };
        let alarm = if let Val::ValI(v) = solution[alarm_triggered] { v } else { 0 };
        
        println!("  Motion: {}, Night Mode: {}, Panic Button: {}", motion, night, panic);
        println!("  Alarm Triggered: {}", alarm);
        println!("  Result: {} (motion during day doesn't trigger alarm)\n", 
                 if alarm == 1 { "ALARM ON" } else { "NO ALARM" });
    }
}

fn logic_circuit_example() {
    println!("📋 Example 3: Digital Logic Circuit");
    println!("Problem: Implement a logic circuit with AND, OR, and NOT gates");
    
    let mut model = Model::default();
    
    // Input signals
    let input_a = model.new_var_int(0, 1);
    let input_b = model.new_var_int(0, 1);
    let input_c = model.new_var_int(0, 1);
    
    // Logic gates
    let not_a = model.bool_not(input_a);           // NOT gate
    let b_and_c = model.bool_and(&[input_b, input_c]);  // AND gate
    let final_output = model.bool_or(&[not_a, b_and_c]); // OR gate
    
    // Circuit: output = (NOT A) OR (B AND C)
    
    // Test with inputs: A=1, B=1, C=0
    model.eq(input_a, Val::int(1));
    model.eq(input_b, Val::int(1));
    model.eq(input_c, Val::int(0));
    
    if let Some(solution) = model.solve() {
        let a = if let Val::ValI(v) = solution[input_a] { v } else { 0 };
        let b = if let Val::ValI(v) = solution[input_b] { v } else { 0 };
        let c = if let Val::ValI(v) = solution[input_c] { v } else { 0 };
        let not_a_val = if let Val::ValI(v) = solution[not_a] { v } else { 0 };
        let and_val = if let Val::ValI(v) = solution[b_and_c] { v } else { 0 };
        let output = if let Val::ValI(v) = solution[final_output] { v } else { 0 };
        
        println!("  Inputs: A={}, B={}, C={}", a, b, c);
        println!("  NOT A = {}", not_a_val);
        println!("  B AND C = {}", and_val);
        println!("  Final Output = (NOT A) OR (B AND C) = {}", output);
        println!("  Circuit Logic: {} OR {} = {}\n", not_a_val, and_val, output);
    }
    
    println!("✅ Boolean logic constraints enable modeling of:");
    println!("   • Security and access control systems");
    println!("   • Smart home automation logic");
    println!("   • Digital circuit design and verification");
    println!("   • Any domain requiring logical reasoning");
}
