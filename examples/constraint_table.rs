use selen::prelude::*;

fn main() {
    println!("🧩 Table Constraint Demonstrations");
    println!("=========================================");

    configuration_problem_demo();
    println!();
    
    compatibility_matrix_demo();
    println!();
    
    lookup_table_demo();
    println!();
    
    course_scheduling_demo();
}

/// Example 1: Configuration problem with valid combinations
fn configuration_problem_demo() {
    println!("📋 Example 1: System Configuration Problem");
    println!("   Variables: CPU (1=Intel, 2=AMD), GPU (1=NVIDIA, 2=AMD), Motherboard (1=Intel, 2=AMD)");
    println!("   Valid combinations enforce compatibility constraints");
    
    let mut m = Model::default();
    
    // Variables: CPU, GPU, Motherboard
    let cpu = m.int(1, 2);        // 1=Intel, 2=AMD
    let gpu = m.int(1, 2);        // 1=NVIDIA, 2=AMD  
    let motherboard = m.int(1, 2); // 1=Intel socket, 2=AMD socket
    
    // Valid configurations table
    // (CPU, GPU, Motherboard) - only certain combinations work
    let valid_configs = vec![
        vec![int(1), int(1), int(1)], // Intel CPU + NVIDIA GPU + Intel motherboard
        vec![int(1), int(2), int(1)], // Intel CPU + AMD GPU + Intel motherboard
        vec![int(2), int(1), int(2)], // AMD CPU + NVIDIA GPU + AMD motherboard
        vec![int(2), int(2), int(2)], // AMD CPU + AMD GPU + AMD motherboard
        // Note: No Intel CPU + AMD motherboard or vice versa
    ];
    
    // Explicitly use the table variable to avoid warnings
    let _ = &valid_configs;
    
    // Apply table constraint
    post!(m, table([cpu, gpu, motherboard], valid_configs));
    
    if let Ok(solution) = m.solve() {
        let cpu_val = match solution[cpu] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        let gpu_val = match solution[gpu] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        let mb_val = match solution[motherboard] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        
        let cpu_name = if cpu_val == 1 { "Intel" } else { "AMD" };
        let gpu_name = if gpu_val == 1 { "NVIDIA" } else { "AMD" };
        let mb_name = if mb_val == 1 { "Intel" } else { "AMD" };
        
        println!("   ✅ Valid configuration found:");
        println!("      CPU: {} ({})", cpu_val, cpu_name);
        println!("      GPU: {} ({})", gpu_val, gpu_name);
        println!("      Motherboard: {} socket ({})", mb_val, mb_name);
    } else {
        println!("   ❌ No valid configuration found");
    }
}

/// Example 2: Compatibility matrix
fn compatibility_matrix_demo() {
    println!("🔗 Example 2: Component Compatibility Matrix");
    println!("   Variables: Software (1-3), Hardware (1-3)");
    println!("   Table defines which software versions work with which hardware");
    
    let mut m = Model::default();
    
    let software = m.int(1, 3);  // Software versions 1, 2, 3
    let hardware = m.int(1, 3);  // Hardware versions 1, 2, 3
    
    // Compatibility matrix: (software_version, hardware_version)
    let compatibility_table = vec![
        vec![int(1), int(1)], // Software v1 works with Hardware v1
        vec![int(1), int(2)], // Software v1 works with Hardware v2
        vec![int(2), int(2)], // Software v2 works with Hardware v2
        vec![int(2), int(3)], // Software v2 works with Hardware v3
        vec![int(3), int(3)], // Software v3 works with Hardware v3
        // Note: Some combinations are incompatible (gaps in compatibility)
    ];
    
    // Explicitly use the table variable to avoid warnings
    let _ = &compatibility_table;
    post!(m, table([software, hardware], compatibility_table));
    
    // Add additional constraint: prefer newer software
    post!(m, software >= int(2));
    
    if let Ok(solution) = m.solve() {
        let sw_val = match solution[software] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        let hw_val = match solution[hardware] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        
        println!("   ✅ Compatible combination found:");
        println!("      Software Version: {}", sw_val);
        println!("      Hardware Version: {}", hw_val);
        println!("      Status: ✓ Compatible");
    } else {
        println!("   ❌ No compatible combination found");
    }
}

/// Example 3: Lookup table for non-linear relationships
fn lookup_table_demo() {
    println!("📊 Example 3: Non-linear Lookup Table");
    println!("   Variables: Input (1-5), Output (function values)");
    println!("   Table represents a complex function: f(x) = x² - 2x + 3");
    
    let mut m = Model::default();
    
    let input = m.int(1, 5);
    let output = m.int(1, 20);
    
    // Lookup table for f(x) = x² - 2x + 3
    let function_table = vec![
        vec![int(1), int(2)],  // f(1) = 1 - 2 + 3 = 2
        vec![int(2), int(3)],  // f(2) = 4 - 4 + 3 = 3
        vec![int(3), int(6)],  // f(3) = 9 - 6 + 3 = 6
        vec![int(4), int(11)], // f(4) = 16 - 8 + 3 = 11
        vec![int(5), int(18)], // f(5) = 25 - 10 + 3 = 18
    ];
    
    // Explicitly use the table variable to avoid warnings
    let _ = &function_table;
    post!(m, table([input, output], function_table));
    
    // Find input that gives output between 5 and 12
    post!(m, output >= int(5));
    post!(m, output <= int(12));
    
    if let Ok(solution) = m.solve() {
        let in_val = match solution[input] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        let out_val = match solution[output] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        
        println!("   ✅ Function evaluation found:");
        println!("      Input: {}", in_val);
        println!("      Output: f({}) = {}", in_val, out_val);
        println!("      Verification: {}² - 2×{} + 3 = {}", in_val, in_val, in_val*in_val - 2*in_val + 3);
    } else {
        println!("   ❌ No solution found");
    }
}

/// Example 4: Course scheduling with constraints
fn course_scheduling_demo() {
    println!("📚 Example 4: Course Scheduling Problem");
    println!("   Variables: Course (1-3), Time slot (1-4), Room (1-2)");
    println!("   Table defines valid (course, time, room) combinations");
    
    let mut m = Model::default();
    
    let course = m.int(1, 3);   // Courses: 1=Math, 2=Physics, 3=Chemistry
    let time_slot = m.int(1, 4); // Time slots: 1=9AM, 2=11AM, 3=1PM, 4=3PM
    let room = m.int(1, 2);     // Rooms: 1=Lab, 2=Classroom
    
    // Valid scheduling combinations: (course, time, room)
    let schedule_table = vec![
        // Math can be in classroom at any time
        vec![int(1), int(1), int(2)], // Math, 9AM, Classroom
        vec![int(1), int(2), int(2)], // Math, 11AM, Classroom  
        vec![int(1), int(3), int(2)], // Math, 1PM, Classroom
        vec![int(1), int(4), int(2)], // Math, 3PM, Classroom
        
        // Physics needs lab in afternoon (equipment setup time)
        vec![int(2), int(3), int(1)], // Physics, 1PM, Lab
        vec![int(2), int(4), int(1)], // Physics, 3PM, Lab
        
        // Chemistry needs lab but not at 1PM (room cleaning)
        vec![int(3), int(1), int(1)], // Chemistry, 9AM, Lab
        vec![int(3), int(2), int(1)], // Chemistry, 11AM, Lab
        vec![int(3), int(4), int(1)], // Chemistry, 3PM, Lab
    ];
    
    // Explicitly use the table variable to avoid warnings
    let _ = &schedule_table;
    
    post!(m, table([course, time_slot, room], schedule_table));
    
    // Prefer morning classes
    post!(m, time_slot <= int(2));
    
    if let Ok(solution) = m.solve() {
        let course_val = match solution[course] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        let time_val = match solution[time_slot] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        let room_val = match solution[room] {
            Val::ValI(i) => i,
            Val::ValF(f) => f as i32,
        };
        
        let course_name = match course_val {
            1 => "Math",
            2 => "Physics", 
            3 => "Chemistry",
            _ => "Unknown"
        };
        
        let time_name = match time_val {
            1 => "9:00 AM",
            2 => "11:00 AM",
            3 => "1:00 PM", 
            4 => "3:00 PM",
            _ => "Unknown"
        };
        
        let room_name = if room_val == 1 { "Lab" } else { "Classroom" };
        
        println!("   ✅ Schedule found:");
        println!("      Course: {} ({})", course_val, course_name);
        println!("      Time: {} ({})", time_val, time_name);
        println!("      Room: {} ({})", room_val, room_name);
    } else {
        println!("   ❌ No valid schedule found");
    }
}