use cspsolver::prelude::*;

fn main() {
    println!("🔧 Boolean Logic Constraints Demo");
    println!("==================================\n");

    // Demo 1: Basic AND operation
    {
        println!("📋 Demo 1: Boolean AND - Security Access Control");
        println!("Problem: Access granted only if user has BOTH badge AND PIN correct");
        
        let mut model = Model::default();
        let has_badge = model.new_var_int(0, 1);  // 0 = no badge, 1 = has badge
        let correct_pin = model.new_var_int(0, 1); // 0 = wrong PIN, 1 = correct PIN
        let access_granted = model.bool_and(&[has_badge, correct_pin]);
        
        // Scenario: User has badge but wrong PIN
        model.eq(has_badge, Val::int(1));
        model.eq(correct_pin, Val::int(0));
        
        if let Some(solution) = model.solve() {
            let badge = if let Val::ValI(v) = solution[has_badge] { v } else { 0 };
            let pin = if let Val::ValI(v) = solution[correct_pin] { v } else { 0 };
            let access = if let Val::ValI(v) = solution[access_granted] { v } else { 0 };
            
            println!("  Badge: {}, PIN: {}, Access: {}", badge, pin, access);
            println!("  Result: Access DENIED (need both badge AND correct PIN)\n");
        }
    }

    // Demo 2: Basic OR operation
    {
        println!("📋 Demo 2: Boolean OR - Emergency Exit");
        println!("Problem: Emergency exit opens if EITHER fire alarm OR manual override");
        
        let mut model = Model::default();
        let fire_alarm = model.new_var_int(0, 1);      // 0 = no fire, 1 = fire detected
        let manual_override = model.new_var_int(0, 1); // 0 = not pressed, 1 = pressed
        let exit_open = model.bool_or(&[fire_alarm, manual_override]);
        
        // Scenario: No fire but manual override pressed
        model.eq(fire_alarm, Val::int(0));
        model.eq(manual_override, Val::int(1));
        
        if let Some(solution) = model.solve() {
            let fire = if let Val::ValI(v) = solution[fire_alarm] { v } else { 0 };
            let manual = if let Val::ValI(v) = solution[manual_override] { v } else { 0 };
            let exit = if let Val::ValI(v) = solution[exit_open] { v } else { 0 };
            
            println!("  Fire Alarm: {}, Manual Override: {}, Exit Open: {}", fire, manual, exit);
            println!("  Result: Exit OPEN (manual override activated)\n");
        }
    }

    // Demo 3: Boolean NOT operation
    {
        println!("📋 Demo 3: Boolean NOT - Inverter Circuit");
        println!("Problem: Output signal is opposite of input signal");
        
        let mut model = Model::default();
        let input_signal = model.new_var_int(0, 1);   // 0 = low, 1 = high
        let output_signal = model.bool_not(input_signal);
        
        // Scenario: High input
        model.eq(input_signal, Val::int(1));
        
        if let Some(solution) = model.solve() {
            let input = if let Val::ValI(v) = solution[input_signal] { v } else { 0 };
            let output = if let Val::ValI(v) = solution[output_signal] { v } else { 0 };
            
            println!("  Input: {}, Output: {}", input, output);
            println!("  Result: High input produces LOW output\n");
        }
    }

    // Demo 4: Complex Boolean Expression
    {
        println!("📋 Demo 4: Complex Logic - Smart Home Security");
        println!("Problem: Alarm triggers if (motion AND night_mode) OR manual_panic");
        
        let mut model = Model::default();
        let motion_detected = model.new_var_int(0, 1);
        let night_mode = model.new_var_int(0, 1);
        let manual_panic = model.new_var_int(0, 1);
        
        // Build expression: (motion AND night_mode) OR manual_panic
        let motion_and_night = model.bool_and(&[motion_detected, night_mode]);
        let alarm_triggered = model.bool_or(&[motion_and_night, manual_panic]);
        
        // Scenario: Motion detected during day, no panic button
        model.eq(motion_detected, Val::int(1));
        model.eq(night_mode, Val::int(0));
        model.eq(manual_panic, Val::int(0));
        
        if let Some(solution) = model.solve() {
            let motion = if let Val::ValI(v) = solution[motion_detected] { v } else { 0 };
            let night = if let Val::ValI(v) = solution[night_mode] { v } else { 0 };
            let panic = if let Val::ValI(v) = solution[manual_panic] { v } else { 0 };
            let motion_night = if let Val::ValI(v) = solution[motion_and_night] { v } else { 0 };
            let alarm = if let Val::ValI(v) = solution[alarm_triggered] { v } else { 0 };
            
            println!("  Motion: {}, Night Mode: {}, Panic: {}", motion, night, panic);
            println!("  Motion AND Night: {}", motion_night);
            println!("  Alarm Triggered: {}", alarm);
            println!("  Result: No alarm (motion during day doesn't trigger alarm)\n");
        }
    }

    // Demo 5: Multi-condition Logic
    {
        println!("📋 Demo 5: Multi-condition AND - Server Safety Checks");
        println!("Problem: Server starts only if ALL conditions are met");
        
        let mut model = Model::default();
        let power_stable = model.new_var_int(0, 1);
        let network_ready = model.new_var_int(0, 1);
        let disk_healthy = model.new_var_int(0, 1);
        let memory_test_passed = model.new_var_int(0, 1);
        
        let server_start = model.bool_and(&[power_stable, network_ready, disk_healthy, memory_test_passed]);
        
        // Scenario: All systems good
        model.eq(power_stable, Val::int(1));
        model.eq(network_ready, Val::int(1));
        model.eq(disk_healthy, Val::int(1));
        model.eq(memory_test_passed, Val::int(1));
        
        if let Some(solution) = model.solve() {
            let power = if let Val::ValI(v) = solution[power_stable] { v } else { 0 };
            let network = if let Val::ValI(v) = solution[network_ready] { v } else { 0 };
            let disk = if let Val::ValI(v) = solution[disk_healthy] { v } else { 0 };
            let memory = if let Val::ValI(v) = solution[memory_test_passed] { v } else { 0 };
            let start = if let Val::ValI(v) = solution[server_start] { v } else { 0 };
            
            println!("  Power: {}, Network: {}, Disk: {}, Memory: {}", power, network, disk, memory);
            println!("  Server Start: {}", start);
            println!("  Result: Server STARTED (all systems GO!)\n");
        }
    }

    // Demo 6: Constraint Propagation
    {
        println!("📋 Demo 6: Constraint Propagation - Logic Puzzle");
        println!("Problem: If result must be true, what constraints apply to inputs?");
        
        let mut model = Model::default();
        let a = model.new_var_int(0, 1);
        let b = model.new_var_int(0, 1);
        let c = model.new_var_int(0, 1);
        
        // Create expression: a AND (b OR c) must be true
        let b_or_c = model.bool_or(&[b, c]);
        let final_result = model.bool_and(&[a, b_or_c]);
        
        // Constraint: result must be true
        model.eq(final_result, Val::int(1));
        
        // Additional constraint: b is false
        model.eq(b, Val::int(0));
        
        if let Some(solution) = model.solve() {
            let a_val = if let Val::ValI(v) = solution[a] { v } else { 0 };
            let b_val = if let Val::ValI(v) = solution[b] { v } else { 0 };
            let c_val = if let Val::ValI(v) = solution[c] { v } else { 0 };
            let b_or_c_val = if let Val::ValI(v) = solution[b_or_c] { v } else { 0 };
            
            println!("  Given: (a AND (b OR c)) = 1, and b = 0");
            println!("  Solution: a = {}, b = {}, c = {}", a_val, b_val, c_val);
            println!("  b OR c = {}", b_or_c_val);
            println!("  Result: Since b=0, both a=1 and c=1 are required!\n");
        }
    }

    println!("✅ Boolean Logic Constraints Demo Complete!");
    println!("   • AND: All operands must be true for result to be true");
    println!("   • OR:  Any operand can be true for result to be true");
    println!("   • NOT: Result is opposite of operand");
    println!("   • Full constraint propagation ensures logical consistency");
}
