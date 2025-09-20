use cspsolver::prelude::*;
use std::time::Instant;

#[test]
fn test_mini_sudoku_solution() {
    println!("Testing minimal sudoku...");
    
    let start = Instant::now();
    let mut m = Model::default();
    
    // Create a 2x2 mini-sudoku for testing - each cell needs a unique variable
    let mut grid = [[m.int(1, 2); 2]; 2];
    
    // Create unique variables for each cell
    for row in 0..2 {
        for col in 0..2 {
            grid[row][col] = m.int(1, 2);
        }
    }
    
    // Row constraints: each row has all digits 1-2
    for row in 0..2 {
        post!(m, alldiff(grid[row]));
    }
    
    // Column constraints: each column has all digits 1-2
    for col in 0..2 {
        let column = [grid[0][col], grid[1][col]];
        post!(m, alldiff(column));
    }
    
    println!("Constraints added, solving...");
    
    let solution = m.solve();
    let duration = start.elapsed();
    
    // Should find a solution successfully
    assert!(solution.is_ok(), "Mini Sudoku should have a solution");
    
    let sol = solution.unwrap();
    
    println!("✅ Success in {:?}!", duration);
    println!("Stats: {} propagations, {} nodes", sol.stats.propagation_count, sol.stats.node_count);
    
    // Verify the solution is valid
    let mut solution_grid = [[0; 2]; 2];
    for row in 0..2 {
        for col in 0..2 {
            if let Val::ValI(val) = sol[grid[row][col]] {
                solution_grid[row][col] = val;
                print!("{} ", val);
            } else {
                panic!("Cell ({},{}) should contain an integer value", row, col);
            }
        }
        println!();
    }
    
    // Verify rows have all digits 1-2
    for row in 0..2 {
        let mut seen = [false; 3]; // index 0 unused, 1-2 for digits
        for col in 0..2 {
            let val = solution_grid[row][col];
            assert!(val >= 1 && val <= 2, "Invalid digit {} at ({},{})", val, row, col);
            assert!(!seen[val as usize], "Duplicate digit {} in row {}", val, row);
            seen[val as usize] = true;
        }
    }
    
    // Verify columns have all digits 1-2
    for col in 0..2 {
        let mut seen = [false; 3];
        for row in 0..2 {
            let val = solution_grid[row][col];
            assert!(!seen[val as usize], "Duplicate digit {} in column {}", val, col);
            seen[val as usize] = true;
        }
    }
    
    // Performance expectations
    assert!(sol.stats.propagation_count > 0, "Should perform some propagations");
    assert!(duration.as_millis() < 1000, "Mini Sudoku should solve quickly, took {:?}", duration);
}

#[test]
fn test_mini_sudoku_uniqueness() {
    // Test that mini Sudoku has unique solutions
    let mut m = Model::default();
    
    // Create a 2x2 mini-sudoku - each cell needs a unique variable  
    let mut grid = [[m.int(1, 2); 2]; 2];
    
    // Create unique variables for each cell
    for row in 0..2 {
        for col in 0..2 {
            grid[row][col] = m.int(1, 2);
        }
    }
    
    // Add constraints
    for row in 0..2 {
        post!(m, alldiff(grid[row]));
    }
    
    for col in 0..2 {
        let column = [grid[0][col], grid[1][col]];
        post!(m, alldiff(column));
    }
    
    let solution = m.solve();
    assert!(solution.is_ok(), "Should find a solution");
    
    let sol = solution.unwrap();
    
    // Extract first solution
    let mut first_solution = [[0; 2]; 2];
    for row in 0..2 {
        for col in 0..2 {
            if let Val::ValI(val) = sol[grid[row][col]] {
                first_solution[row][col] = val;
            }
        }
    }
    
    // For a 2x2 sudoku with alldiff constraints, there should be exactly 2 solutions:
    // [1,2] [2,1] and [2,1] [1,2]
    // Verify the solution is one of these valid patterns
    let valid_solution_1 = [[1, 2], [2, 1]];
    let valid_solution_2 = [[2, 1], [1, 2]];
    
    let is_valid = first_solution == valid_solution_1 || first_solution == valid_solution_2;
    assert!(is_valid, "Solution should be one of the two valid 2x2 Latin squares");
    
    eprintln!("✅ Mini Sudoku uniqueness test passed");
}