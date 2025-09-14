//! Resource Allocation and Task Assignment
//!
//! A practical example of resource allocation in project management.
//! This demonstrates:
//! - Assignment of workers with different skills to tasks
//! - Time and resource constraints
//! - Minimizing total project time
//! - Handling worker efficiency variations

use cspsolver::prelude::*;
use cspsolver::{post};

fn main() {
    println!("📋 Resource Allocation Optimizer");
    println!("================================");
    
    // Create a model for our task assignment problem
    let mut m = Model::default();
    
    // Worker efficiency ratings (time multipliers - lower is better)
    let workers = ["Alice", "Bob", "Charlie"];
    let worker_time_multipliers = [int(12), int(17), int(11)]; // Alice and Charlie are faster
    
    // Task definitions (base time requirements in hours)
    let tasks = ["UI Design", "Backend API", "Testing"];
    let task_base_time = [int(20), int(30), int(15)];
    
    println!("Workers and their time multipliers (lower = faster):");
    for (worker, mult) in workers.iter().zip(&worker_time_multipliers) {
        println!("  {}: {}x multiplier", worker, match *mult { Val::ValI(m) => m, _ => 0 });
    }
    
    println!("\nTasks and base time requirements:");
    for (task, time) in tasks.iter().zip(&task_base_time) {
        println!("  {}: {} hours", task, match *time { Val::ValI(t) => t, _ => 0 });
    }
    
    // Assignment variables: worker_assignments[task][worker] = 1 if assigned
    let mut worker_assignments = Vec::new();
    for _ in 0..tasks.len() {
        let task_assignments: Vec<_> = m.new_vars_binary(workers.len()).collect();
        worker_assignments.push(task_assignments);
    }
    
    // Each task must be assigned to exactly one worker
    for task_assignments in &worker_assignments {
        let task_assignment_sum = m.sum(task_assignments);
        post!(m, task_assignment_sum == int(1));
    }
    println!("Constraint: Each task must be assigned to exactly one worker");
    
    // Calculate actual completion times based on worker efficiency
    // We'll use time multipliers: base_time * multiplier = actual time  
    let mut task_completion_times = Vec::new();
    for (task_idx, task_assignments) in worker_assignments.iter().enumerate() {
        let time_terms: Vec<_> = task_assignments.iter().enumerate().map(|(worker_idx, assignment)| {
            // For each worker assignment: assignment * (base_time * time_multiplier)
            let base_time_val = match task_base_time[task_idx] {
                Val::ValI(t) => t,
                _ => 0
            };
            let multiplier_val = match worker_time_multipliers[worker_idx] {
                Val::ValI(m) => m, 
                _ => 0
            };
            let adjusted_time = int(base_time_val * multiplier_val);
            m.mul(*assignment, adjusted_time)
        }).collect();
        
        let total_time = m.sum_iter(time_terms);
        task_completion_times.push(total_time);
    }
    
    // Total project time is the sum of all task completion times
    let total_project_time = m.sum(&task_completion_times);
    
    // Minimize total project time
    let solution = m.minimize(total_project_time).unwrap();
    
    println!("\n🎯 Optimal Assignment Results:");
    println!("==============================");
    
    let mut total_time = 0;
    for (task_idx, task_assignments) in worker_assignments.iter().enumerate() {
        let assignments = solution.get_values_binary(task_assignments);
        for (worker_idx, &assigned) in assignments.iter().enumerate() {
            if assigned {
                let task_time = match solution[task_completion_times[task_idx]] {
                    Val::ValI(t) => t,
                    _ => 0
                };
                total_time += task_time;
                
                println!("  {} → {} ({} hours)", 
                    tasks[task_idx], 
                    workers[worker_idx], 
                    task_time
                );
            }
        }
    }
    
    println!("\n📊 Summary:");
    println!("  Total project time: {} hours", total_time);
    println!("  Average time per task: {:.1} hours", total_time as f32 / tasks.len() as f32);
    
    println!("\n✅ Optimal resource allocation found!");
    println!("   This assignment minimizes total project completion time");
    println!("   while respecting all assignment constraints.");
}
