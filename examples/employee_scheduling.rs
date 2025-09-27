use selen::prelude::*;

type Employee = (&'static str, usize, usize, bool); // (name, min_shift, max_shift, is_supervisor)
type Schedule = Vec<Vec<Vec<VarId>>>; // [day][shift][employee]

fn can_work_shift(employee: &Employee, shift: usize) -> bool {
    shift >= employee.1 && shift <= employee.2
}

fn create_work_variable(m: &mut Model, can_work: bool) -> VarId {
    if can_work {
        m.int(0, 1) // Binary: works this shift or not
    } else {
        m.int(0, 0) // Cannot work - fixed to 0
    }
}

fn build_schedule(m: &mut Model, staff: &[Employee], days: usize, shifts: usize) -> Schedule {
    let mut schedule = Vec::new();
    
    for _day in 0..days {
        let mut day_schedule = Vec::new();
        
        for shift in 0..shifts {
            let shift_workers: Vec<VarId> = staff.iter()
                .map(|employee| create_work_variable(m, can_work_shift(employee, shift)))
                .collect();
            day_schedule.push(shift_workers);
        }
        
        schedule.push(day_schedule);
    }
    
    schedule
}

fn add_staffing_constraints(m: &mut Model, schedule: &Schedule, needed: &[i32]) {
    for (_day, day_schedule) in schedule.iter().enumerate() {
        for (shift, workers) in day_schedule.iter().enumerate() {
            let worker_sum = m.sum(workers);
            m.c(worker_sum).eq(int(needed[shift]));
        }
    }
}

/// Ensures each shift has at least one supervisor working
/// For every shift on every day, at least one supervisor must be scheduled
fn add_supervisor_constraints(m: &mut Model, schedule: &Schedule, staff: &[Employee]) {
    // Go through each day in the schedule
    for day_schedule in schedule {
        // Go through each shift in that day (morning, afternoon, night)
        for shift_workers in day_schedule {
            // Find all supervisor variables for this specific shift
            // We look at each employee and check if they're a supervisor
            let supervisors: Vec<VarId> = staff.iter()
                .enumerate() // Get employee index and their data
                .filter_map(|(emp_id, (_, _, _, is_supervisor))| {
                    if *is_supervisor {
                        // This employee is a supervisor, so include their work variable for this shift
                        Some(shift_workers[emp_id])
                    } else {
                        // Regular employee, skip them
                        None
                    }
                })
                .collect();
            
            // Only add constraint if we have supervisors available for this shift
            if !supervisors.is_empty() {
                // Constraint: sum of supervisor work variables >= 1
                // This means at least one supervisor must work this shift
                let supervisor_sum = m.sum(&supervisors);
                m.c(supervisor_sum).ge(int(1));
            }
        }
    }
}

fn add_workload_constraints(m: &mut Model, schedule: &Schedule, staff: &[Employee]) {
    for (emp_id, _) in staff.iter().enumerate() {
        for day_schedule in schedule {
            let daily_shifts: Vec<VarId> = day_schedule.iter()
                .map(|shift_workers| shift_workers[emp_id])
                .collect();
            let daily_sum = m.sum(&daily_shifts);
            m.c(daily_sum).le(int(2));
        }
    }
}

fn main() {
    println!("Employee Scheduling System");
    println!("==========================");
    
    let mut m = Model::default();
    
    let staff: [Employee; 9] = [
        ("Alice", 0, 2, true),   // Any shift, supervisor
        ("Bob", 0, 1, false),    // Morning/afternoon only
        ("Carol", 1, 2, true),   // Afternoon/night, supervisor  
        ("David", 0, 2, false),  // Any shift
        ("Eve", 2, 2, false),    // Night only
        ("Frank", 0, 1, true),   // Morning/afternoon, supervisor
        ("Grace", 0, 2, false),  // Any shift
        ("Henry", 1, 2, true),   // Afternoon/night, supervisor
        ("Ivy", 0, 1, false),    // Morning/afternoon only
    ];
    
    let days = 2;
    let shifts = 3; // 0=Morning, 1=Afternoon, 2=Night
    let needed = [3, 4, 3]; // Staff needed per shift
    
    // Build the scheduling variables
    let schedule = build_schedule(&mut m, &staff, days, shifts);
    
    // Add all constraints using helper functions
    add_staffing_constraints(&mut m, &schedule, &needed);
    add_supervisor_constraints(&mut m, &schedule, &staff);
    add_workload_constraints(&mut m, &schedule, &staff);
    
    println!("Solving...");
    
    match m.solve() {
        Ok(solution) => {
            println!("Schedule found!\n");
            
            // Calculate column widths based on name lengths
            let mut col_widths = Vec::new();
            for (name, _, _, is_supervisor) in &staff {
                let display_name = if *is_supervisor {
                    format!("{}(S)", name)
                } else {
                    name.to_string()
                };
                col_widths.push(display_name.len().max(3)); // minimum 3 chars for X/-
            }
            
            // Print header
            println!("Employee Schedule Table:");
            print!("┌───────────");
            for &width in &col_widths {
                print!("┬{}", "─".repeat(width + 2));
            }
            println!("┐");
            
            print!("│   Shift   ");
            for (i, (name, _, _, is_supervisor)) in staff.iter().enumerate() {
                let display = if *is_supervisor {
                    format!("{}(S)", name)
                } else {
                    name.to_string()
                };
                print!("│ {:<width$} ", display, width = col_widths[i]);
            }
            println!("│");
            
            print!("├───────────");
            for &width in &col_widths {
                print!("┼{}", "─".repeat(width + 2));
            }
            println!("┤");
            
            // Print each shift as a row
            let shift_names = ["Morning", "Afternoon", "Night"];
            for day in 0..days {
                for shift in 0..shifts {
                    let shift_label = if days == 1 {
                        format!("{:<9}", shift_names[shift])
                    } else {
                        format!("{} D{}", &shift_names[shift][..3], day + 1)
                    };
                    print!("│ {:<9} ", shift_label);
                    
                    for emp_id in 0..staff.len() {
                        let works = solution.get::<i32>(schedule[day][shift][emp_id]) == 1;
                        let symbol = if works { "X" } else { "-" };
                        print!("│ {:<width$} ", symbol, width = col_widths[emp_id]);
                    }
                    println!("│");
                }
                
                // No separator between days to avoid empty rows
            }
            
            print!("└───────────");
            for &width in &col_widths {
                print!("┴{}", "─".repeat(width + 2));
            }
            println!("┘");
            
            // Show daily schedule breakdown
            println!("\nDaily Breakdown:");
            let shift_names = ["Morning", "Afternoon", "Night"];
            
            for day in 0..days {
                println!("\nDay {}:", day + 1);
                for shift in 0..shifts {
                    let workers: Vec<_> = staff.iter()
                        .enumerate()
                        .filter_map(|(emp_id, (name, _, _, is_sup))| {
                            if solution.get::<i32>(schedule[day][shift][emp_id]) == 1 {
                                let role_marker = if *is_sup { "(S)" } else { "" };
                                Some(format!("{}{}", name, role_marker))
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    println!("  {:<10}: {} (needed: {})", 
                        shift_names[shift], 
                        if workers.is_empty() { "No one assigned".to_string() } else { workers.join(", ") },
                        needed[shift]
                    );
                }
            }
        }
        Err(_) => println!("No solution found!"),
    }
}