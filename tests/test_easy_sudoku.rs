use cspsolver::prelude::*;
use cspsolver::{post};

fn solve_sudoku(puzzle: &[[i32; 9]; 9]) -> Option<([[i32; 9]; 9], usize, usize)> {
    let mut m = Model::default();
    
    // OPTIMIZATION 1: Create variables more efficiently
    // For clues, create singleton variables directly; for empty cells, create full domain
    let mut grid = [[m.int(1, 9); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] != 0 {
                // Create singleton variable for clues (much more efficient than equals constraint)
                let clue_val = puzzle[row][col];
                grid[row][col] = m.int(clue_val, clue_val);
            } else {
                grid[row][col] = m.int(1, 9);
            }
        }
    }
    
    // OPTIMIZATION 2: Pre-allocate vectors and use more efficient constraint posting
    // Row constraints: each row has all digits 1-9
    for row in 0..9 {
        post!(m, alldiff(grid[row]));
    }
    
    // Column constraints: each column has all digits 1-9
    for col in 0..9 {
        let column = [
            grid[0][col], grid[1][col], grid[2][col],
            grid[3][col], grid[4][col], grid[5][col],
            grid[6][col], grid[7][col], grid[8][col]
        ];
        post!(m, alldiff(column));
    }
    
    // Box constraints: each 3x3 box has all digits 1-9
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::with_capacity(9);
            for row in 0..3 {
                for col in 0..3 {
                    box_vars.push(grid[box_row * 3 + row][box_col * 3 + col]);
                }
            }
            post!(m, alldiff(box_vars));
        }
    }
    
    // OPTIMIZATION 3: Optimize constraint order for better propagation
    m.optimize_constraint_order();
    
    // Solve the model with embedded statistics
    let solution = m.solve();
    
    // Convert solution to grid
    match solution {
        Ok(sol) => {
            // Access statistics from the solution
            let propagation_count = sol.stats.propagation_count;
            let node_count = sol.stats.node_count;
            
            let mut result = [[0; 9]; 9];
            for row in 0..9 {
                for col in 0..9 {
                    if let Val::ValI(value) = sol[grid[row][col]] {
                        result[row][col] = value;
                    }
                }
            }
            Some((result, propagation_count, node_count))
        }
        Err(_) => None
    }
}

#[allow(dead_code)]
fn print_grid(title: &str, grid: &[[i32; 9]; 9]) {
    println!("\n{}", title);
    println!("┌───────┬───────┬───────┐");
    
    for (row_idx, row) in grid.iter().enumerate() {
        print!("│");
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 0 {
                print!(" ·");
            } else {
                print!(" {}", cell);
            }
            if col_idx % 3 == 2 && col_idx < 8 {
                print!(" │");
            }
        }
        println!(" │");
        
        if row_idx % 3 == 2 && row_idx < 8 {
            println!("├───────┼───────┼───────┤");
        }
    }
    println!("└───────┴───────┴───────┘");
}

#[test]
fn test_easy_sudoku_solution() {
    // Easy Sudoku puzzle (26 clues)
    let easy_puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 0],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 0],
    ];
    
    // Solve the puzzle
    let result = solve_sudoku(&easy_puzzle);
    
    // Verify solution exists
    assert!(result.is_some(), "Easy Sudoku should have a solution");
    
    let (solution, propagations, nodes) = result.unwrap();
    
    // Verify the solution is valid (not necessarily matching expected exactly)
    // Check rows have all digits 1-9
    for row in 0..9 {
        let mut seen = [false; 10]; // index 0 unused, 1-9 for digits
        for col in 0..9 {
            let val = solution[row][col];
            assert!(val >= 1 && val <= 9, "Invalid digit {} at ({},{})", val, row, col);
            assert!(!seen[val as usize], "Duplicate digit {} in row {}", val, row);
            seen[val as usize] = true;
        }
    }
    
    // Check columns have all digits 1-9
    for col in 0..9 {
        let mut seen = [false; 10];
        for row in 0..9 {
            let val = solution[row][col];
            assert!(!seen[val as usize], "Duplicate digit {} in column {}", val, col);
            seen[val as usize] = true;
        }
    }
    
    // Check 3x3 blocks have all digits 1-9
    for block_row in 0..3 {
        for block_col in 0..3 {
            let mut seen = [false; 10];
            for row in block_row * 3..(block_row + 1) * 3 {
                for col in block_col * 3..(block_col + 1) * 3 {
                    let val = solution[row][col];
                    assert!(!seen[val as usize], "Duplicate digit {} in block ({},{})", val, block_row, block_col);
                    seen[val as usize] = true;
                }
            }
        }
    }
    
    // Verify initial clues are preserved
    for row in 0..9 {
        for col in 0..9 {
            if easy_puzzle[row][col] != 0 {
                assert_eq!(solution[row][col], easy_puzzle[row][col], 
                    "Initial clue at ({},{}) should be preserved", row, col);
            }
        }
    }
    
    // Verify performance characteristics
    assert!(propagations > 0, "Should perform some propagations");
    // nodes is unsigned, so always >= 0
    
    // Print results for manual inspection during development
    eprintln!("✅ Easy Sudoku solved with {} propagations, {} nodes", propagations, nodes);
}

#[test]
fn test_sudoku_constraint_validation() {
    // Test that the solver properly handles invalid/unsolvable Sudoku puzzles
    let invalid_puzzle = [
        [1, 1, 0, 0, 0, 0, 0, 0, 0],  // Two 1s in same row - invalid
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    
    // This should either fail to solve or detect the constraint violation
    let result = solve_sudoku(&invalid_puzzle);
    assert!(result.is_none(), "Invalid Sudoku with duplicate values should not have a solution");
    
    // Test a valid but minimally constrained puzzle should solve
    let minimal_puzzle = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    
    let result = solve_sudoku(&minimal_puzzle);
    assert!(result.is_some(), "Minimal valid Sudoku should have a solution");
}

#[test]
fn test_sudoku_performance_reasonable() {
    // Test that easy Sudoku solves in reasonable time
    let easy_puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 0],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 0, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 0],
    ];
    
    let start = std::time::Instant::now();
    let result = solve_sudoku(&easy_puzzle);
    let duration = start.elapsed();
    
    assert!(result.is_some(), "Should solve easy puzzle");
    assert!(duration.as_millis() < 1000, "Easy Sudoku should solve in under 1 second");
    
    let (_, propagations, _) = result.unwrap();
    assert!(propagations < 10000, "Should not require excessive propagations for easy puzzle");
}