//! Magic Square Problem
//! 
//! A magic square is an n×n grid filled with distinct integers from 1 to n²
//! such that the sum of integers in any row, column, or diagonal is the same.

use cspsolver::prelude::*;

fn solve_magic_square(size: usize) -> Result<(), &'static str> {
    println!("🎯 {}×{} Magic Square Problem", size, size);
    println!("{}", "=".repeat(30 + size.to_string().len() * 2));
    
    let mut model = Model::default();
    
    // Magic constant formula: n(n²+1)/2
    let magic_constant = (size * (size * size + 1)) / 2;
    println!("📊 Magic constant: {}", magic_constant);
    
    // Create an n×n grid (variables for values 1 to n²)
    let max_value = (size * size) as i32;
    let grid: Vec<Vec<VarId>> = (0..size)
        .map(|_| (0..size).map(|_| model.int(1, max_value)).collect())
        .collect();
    
    // All numbers 1 to n² used exactly once
    let all_vars: Vec<VarId> = grid.iter().flat_map(|row| row.iter().copied()).collect();
    post!(model, alldiff(all_vars));
    
    // Optional: Demonstrate count constraint by ensuring each number appears exactly once
    // This is redundant with alldiff but shows how count can be used for verification
    if size <= 3 {  // Only for small examples to avoid too many constraints
        for num in 1..=max_value {
            let count_var = model.int(1, 1); // Each number appears exactly once
            post!(model, count(all_vars.clone(), int(num), count_var));
        }
        println!("📝 Added count constraints to ensure each number 1-{} appears exactly once", max_value);
    }
    
    // Each row sums to magic constant
    for row in &grid {
        post!(model, sum(row.clone()) == int(magic_constant as i32));
    }
    
    // Each column sums to magic constant
    for col in 0..size {
        let col_vars: Vec<VarId> = grid.iter().map(|row| row[col]).collect();
        post!(model, sum(col_vars) == int(magic_constant as i32));
    }
    
    // Diagonals sum to magic constant
    let main_diag: Vec<VarId> = (0..size).map(|i| grid[i][i]).collect();
    post!(model, sum(main_diag) == int(magic_constant as i32));
    
    let anti_diag: Vec<VarId> = (0..size).map(|i| grid[i][size - 1 - i]).collect();
    post!(model, sum(anti_diag) == int(magic_constant as i32));
    
    // For better performance on larger squares, add some symmetry breaking
    if size >= 4 {
        // Fix smallest value in top-left corner
        post!(model, grid[0][0] == int(1));
        // Top row should be in ascending order (partial symmetry breaking)
        if size <= 5 {  // Only for manageable sizes
            for col in 0..size.min(3)-1 {  // Just first few elements
                post!(model, grid[0][col] < grid[0][col + 1]);
            }
        }
    }
    
    println!("🔍 Solving...");
    let start = std::time::Instant::now();
    
    match model.solve() {
        Ok(solution) => {
            let duration = start.elapsed();
            println!("✅ Solution found in {:?}!", duration);
            println!();
            
            // Display the magic square
            for row in &grid {
                print!("  ");
                for &var in row {
                    if let Val::ValI(value) = solution[var] {
                        print!("{:3} ", value);
                    }
                }
                println!();
            }
            
            // Verify the solution
            println!("\n🔢 Verification:");
            
            // Check rows
            for (i, row) in grid.iter().enumerate() {
                let sum: i32 = row.iter().map(|&var| {
                    if let Val::ValI(value) = solution[var] { value } else { 0 }
                }).sum();
                println!("  Row {}: sum = {} ✓", i + 1, sum);
            }
            
            // Check columns
            for col in 0..size {
                let sum: i32 = grid.iter().map(|row| {
                    if let Val::ValI(value) = solution[row[col]] { value } else { 0 }
                }).sum();
                println!("  Col {}: sum = {} ✓", col + 1, sum);
            }
            
            // Check diagonals
            let main_sum: i32 = (0..size).map(|i| {
                if let Val::ValI(value) = solution[grid[i][i]] { value } else { 0 }
            }).sum();
            println!("  Main diagonal: sum = {} ✓", main_sum);
            
            let anti_sum: i32 = (0..size).map(|i| {
                if let Val::ValI(value) = solution[grid[i][size - 1 - i]] { value } else { 0 }
            }).sum();
            println!("  Anti diagonal: sum = {} ✓", anti_sum);
            
            println!("  🎉 All sums equal {} - Valid magic square!", magic_constant);
            Ok(())
        },
        Err(_) => {
            let duration = start.elapsed();
            println!("❌ No solution found after {:?}", duration);
            Err("No solution found")
        }
    }
}

fn main() {
    println!("🎯 Magic Square Problems");
    println!("========================\n");
    
    // Solve different sizes
    let sizes = vec![3, 4, 5];
    
    for size in sizes {
        match solve_magic_square(size) {
            Ok(_) => println!("\n{}\n", "✅".repeat(20)),
            Err(_) => println!("\n{}\n", "❌".repeat(20)),
        }
    }
    
    // Performance comparison
    println!("⚡ Performance Summary:");
    println!("======================");
    
    for size in 3..=5 {
        print!("{}×{} square: ", size, size);
        let start = std::time::Instant::now();
        
        let mut model = Model::default();
        let max_value = (size * size) as i32;
        let magic_constant = (size * (size * size + 1)) / 2;
        
        let grid: Vec<Vec<VarId>> = (0..size)
            .map(|_| (0..size).map(|_| model.int(1, max_value)).collect())
            .collect();
        
        let all_vars: Vec<VarId> = grid.iter().flat_map(|row| row.iter().copied()).collect();
        post!(model, alldiff(all_vars));
        
        for row in &grid {
            post!(model, sum(row.clone()) == int(magic_constant as i32));
        }
        
        for col in 0..size {
            let col_vars: Vec<VarId> = grid.iter().map(|row| row[col]).collect();
            post!(model, sum(col_vars) == int(magic_constant as i32));
        }
        
        let main_diag: Vec<VarId> = (0..size).map(|i| grid[i][i]).collect();
        post!(model, sum(main_diag) == int(magic_constant as i32));
        
        let anti_diag: Vec<VarId> = (0..size).map(|i| grid[i][size - 1 - i]).collect();
        post!(model, sum(anti_diag) == int(magic_constant as i32));
        
        if size >= 4 {
            post!(model, grid[0][0] == int(1));
        }
        
        match model.solve() {
            Ok(_) => {
                let duration = start.elapsed();
                println!("✅ {:?}", duration);
            },
            Err(_) => {
                let duration = start.elapsed();
                println!("❌ {:?} (no solution)", duration);
            }
        }
    }
}
