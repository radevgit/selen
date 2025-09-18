//! Count Constraint Demonstration
//!
//! This demo showcases the count constraint with practical examples:
//! 1. Simple counting example with basic constraints
//! 2. Resource allocation with count constraints

use cspsolver::prelude::*;

fn main() {
    println!("=== Count Constraint Demonstrations ===\n");
    
    simple_counting_demo();
    resource_allocation_demo();
}

/// Demo 1: Simple counting with color preferences
fn simple_counting_demo() {
    println!("1. Simple Counting Demo: Color Preferences");
    println!("   Problem: 5 people choose colors (1=red, 2=blue, 3=green)");
    println!("   Constraint: Exactly 2 people must choose red (1)");
    
    let mut m = Model::default();
    
    // 5 people choosing colors: 1=red, 2=blue, 3=green
    let people = [
        m.int(1, 3), // Person A
        m.int(1, 3), // Person B  
        m.int(1, 3), // Person C
        m.int(1, 3), // Person D
        m.int(1, 3), // Person E
    ];
    
    // Count variable: how many choose red (1)
    let red_count = m.int(2, 2); // Exactly 2
    
    // Count constraint: exactly 2 people choose red
    post!(m, count(people, int(1), red_count));
    
    // At least one person chooses blue
    let blue_count = m.int(1, 5);
    post!(m, count(people, int(2), blue_count));
    
    match m.solve() {
        Ok(solution) => {
            println!("   Solution found:");
            let person_names = ["A", "B", "C", "D", "E"];
            let colors = ["", "Red", "Blue", "Green"];
            
            for (i, &person) in people.iter().enumerate() {
                if let Val::ValI(color) = solution[person] {
                    println!("     Person {}: {} ({})", person_names[i], color, colors[color as usize]);
                }
            }
            
            if let (Val::ValI(red_cnt), Val::ValI(blue_cnt)) = (solution[red_count], solution[blue_count]) {
                println!("     Red count: {}, Blue count: {}", red_cnt, blue_cnt);
            }
        }
        Err(_) => {
            println!("   No solution found");
        }
    }
    println!();
}

/// Demo 2: Resource Allocation
/// Assign workers to 3 shifts ensuring exactly 2 workers per night shift
fn resource_allocation_demo() {
    println!("2. Resource Allocation: Worker Shift Assignment");
    println!("   Problem: Assign 6 workers to shifts (1=day, 2=evening, 3=night)");
    println!("   Constraint: Exactly 2 workers must work night shifts (3)");
    
    let mut m = Model::default();
    
    // Workers assigned to shifts: 1=day, 2=evening, 3=night
    let workers = [
        m.int(1, 3), // Worker A
        m.int(1, 3), // Worker B  
        m.int(1, 3), // Worker C
        m.int(1, 3), // Worker D
        m.int(1, 3), // Worker E
        m.int(1, 3), // Worker F
    ];
    
    // Count variable: how many work night shift (3)
    let night_workers = m.int(2, 2); // Exactly 2
    
    // Count constraint: exactly 2 workers on night shift
    post!(m, count(workers, int(3), night_workers));
    
    // Additional constraint: at least 2 on day shift  
    let day_workers = m.int(2, 6);
    post!(m, count(workers, int(1), day_workers));
    
    match m.solve() {
        Ok(solution) => {
            println!("   Solution found:");
            let worker_names = ["A", "B", "C", "D", "E", "F"];
            let shifts = ["", "Day", "Evening", "Night"];
            
            for (i, &worker) in workers.iter().enumerate() {
                if let Val::ValI(shift) = solution[worker] {
                    println!("     Worker {}: {} shift", worker_names[i], shifts[shift as usize]);
                }
            }
            
            if let (Val::ValI(night_count), Val::ValI(day_count)) = (solution[night_workers], solution[day_workers]) {
                println!("     Night workers: {}, Day workers: {}", night_count, day_count);
            }
        }
        Err(_) => {
            println!("   No solution found");
        }
    }
    
    println!("\n=== Count constraint demonstrations complete! ===");
    println!("The count constraint enables powerful counting and cardinality constraints");
    println!("for scheduling, resource allocation, and distribution problems!");
}