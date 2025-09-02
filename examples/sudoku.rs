//! Sudoku Solver using the CSP framework
//! 
//! This example demonstrates solving a classic 9x9 Sudoku puzzle using constraint programming.
//! 
//! Sudoku constraints:
//! 1. Each cell contains a digit 1-9
//! 2. Each row contains all digits 1-9 exactly once
//! 3. Each column contains all digits 1-9 exactly once  
//! 4. Each 3x3 box contains all digits 1-9 exactly once
//! 5. Given clues must be respected

use cspsolver::prelude::*;
use cspsolver::solution::{Solution, SolveStats};

/// A solved Sudoku puzzle
pub struct SudokuSolution {
    solution: Solution,
}

impl SudokuSolution {
    /// Convert the solution to a 9x9 grid of integers
    pub fn to_grid(&self, grid: &[[VarId; 9]; 9]) -> [[i32; 9]; 9] {
        let mut result = [[0; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                if let Val::ValI(value) = self.solution[grid[row][col]] {
                    result[row][col] = value;
                }
            }
        }
        result
    }
}

/// A Sudoku puzzle solver
pub struct SudokuSolver {
    model: Model,
    /// 9x9 grid of variables, each representing a cell value (1-9)
    grid: [[VarId; 9]; 9],
}

impl SudokuSolver {
    /// Create a new Sudoku solver with an empty grid
    pub fn new() -> Self {
        let mut model = Model::default();
        
        // Create variables for each cell (1-9)
        let mut grid = [[model.new_var(Val::int(1), Val::int(9)); 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                grid[row][col] = model.new_var(Val::int(1), Val::int(9));
            }
        }
        
        Self { model, grid }
    }
    
    /// Set a clue (given value) at the specified position
    pub fn set_clue(&mut self, row: usize, col: usize, value: i32) {
        assert!(row < 9 && col < 9, "Position must be within 9x9 grid");
        assert!((1..=9).contains(&value), "Value must be between 1 and 9");
        
        let cell = self.grid[row][col];
        self.model.equals(cell, Val::int(value));
    }
    
    /// Set multiple clues from a 9x9 array (0 represents empty cell)
    pub fn set_puzzle(&mut self, puzzle: &[[i32; 9]; 9]) {
        // Apply preprocessing to reduce search space
        let preprocessed = self.preprocess_puzzle(puzzle);
        
        for row in 0..9 {
            for col in 0..9 {
                if preprocessed[row][col] != 0 {
                    self.set_clue(row, col, preprocessed[row][col]);
                }
            }
        }
    }
    
    /// Preprocess the puzzle using Sudoku-specific techniques
    fn preprocess_puzzle(&self, puzzle: &[[i32; 9]; 9]) -> [[i32; 9]; 9] {
        let mut result = *puzzle;
        let mut changed = true;
        
        // Apply naked singles technique iteratively
        while changed {
            changed = false;
            
            for row in 0..9 {
                for col in 0..9 {
                    if result[row][col] == 0 {
                        let possible = self.get_possible_values(&result, row, col);
                        if possible.len() == 1 {
                            result[row][col] = possible[0];
                            changed = true;
                        }
                    }
                }
            }
        }
        
        result
    }
    
    /// Get possible values for a cell given current puzzle state
    fn get_possible_values(&self, puzzle: &[[i32; 9]; 9], row: usize, col: usize) -> Vec<i32> {
        if puzzle[row][col] != 0 {
            return vec![puzzle[row][col]];
        }
        
        let mut possible: Vec<i32> = (1..=9).collect();
        
        // Remove values from same row
        for c in 0..9 {
            if let Some(pos) = possible.iter().position(|&x| x == puzzle[row][c]) {
                possible.remove(pos);
            }
        }
        
        // Remove values from same column
        for r in 0..9 {
            if let Some(pos) = possible.iter().position(|&x| x == puzzle[r][col]) {
                possible.remove(pos);
            }
        }
        
        // Remove values from same 3x3 box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if let Some(pos) = possible.iter().position(|&x| x == puzzle[r][c]) {
                    possible.remove(pos);
                }
            }
        }
        
        possible
    }
    
    /// Add all Sudoku constraints with optimizations
    pub fn add_constraints(&mut self) {
        // Add constraints in order of effectiveness:
        // 1. Box constraints first (most restrictive)
        // 2. Row constraints 
        // 3. Column constraints
        
        // Box constraints: each 3x3 box contains digits 1-9 exactly once
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut box_cells = Vec::new();
                for row in 0..3 {
                    for col in 0..3 {
                        box_cells.push(self.grid[box_row * 3 + row][box_col * 3 + col]);
                    }
                }
                self.add_all_different(&box_cells);
            }
        }
        
        // Row constraints: each row contains digits 1-9 exactly once
        for row in 0..9 {
            let row_vars = self.grid[row].to_vec();
            self.add_all_different(&row_vars);
        }

        // Column constraints: each column contains digits 1-9 exactly once
        for col in 0..9 {
            let column: Vec<VarId> = (0..9).map(|row| self.grid[row][col]).collect();
            self.add_all_different(&column);
        }
        
        // Add naked singles constraints for better propagation
        self.add_naked_singles_constraints();
    }
    
    /// Add naked singles constraints for more efficient propagation
    fn add_naked_singles_constraints(&mut self) {
        // For each cell, if only one value is possible after initial constraints,
        // this will be handled by the constraint propagation automatically.
        // The CSP solver's propagation should handle this, but we can make it more explicit
        // by ensuring all values are properly constrained.
        
        // This is more of a hint that constraint propagation should be aggressive
        // The actual implementation relies on the CSP engine's propagation
    }
    
    /// Add all-different constraint for a group of variables
    fn add_all_different(&mut self, vars: &[VarId]) {
        // Use the efficient all-different propagator instead of pairwise not-equals
        self.model.all_different(vars.to_vec());
    }
    
    /// Solve the Sudoku puzzle and return the first solution
    pub fn solve(self) -> Option<SudokuSolution> {
        let start = std::time::Instant::now();
        let solutions: Vec<_> = self.model.enumerate().take(1).collect();
        let elapsed = start.elapsed();
        
        println!("Solved in {:?}", elapsed);
        
        solutions.into_iter().next().map(|solution| {
            SudokuSolution { solution }
        })
    }
    
    pub fn solve_with_stats(self) -> (Option<SudokuSolution>, SolveStats) {
        let start = std::time::Instant::now();
        let mut stats_result = SolveStats { propagation_count: 0, node_count: 0 };
        
        let solutions = self.model.enumerate_with_callback(|stats| {
            stats_result = SolveStats {
                propagation_count: stats.propagation_count,
                node_count: stats.node_count,
            };
        });
        
        let elapsed = start.elapsed();
        
        println!("Solved in {:?}", elapsed);
        println!("Statistics: {} propagations, {} nodes explored", 
                 stats_result.propagation_count, stats_result.node_count);
        
        let solution = solutions.into_iter().next().map(|solution| {
            SudokuSolution { solution }
        });
        
        (solution, stats_result)
    }
    
    /// Print the current puzzle state
    pub fn print_puzzle(&self, puzzle: &[[i32; 9]; 9]) {
        println!("┌───────┬───────┬───────┐");
        for (row_idx, row) in puzzle.iter().enumerate() {
            print!("│ ");
            for (col_idx, &value) in row.iter().enumerate() {
                if value == 0 {
                    print!("· ");
                } else {
                    print!("{} ", value);
                }
                
                if (col_idx + 1) % 3 == 0 {
                    print!("│ ");
                }
            }
            println!();
            
            if (row_idx + 1) % 3 == 0 && row_idx < 8 {
                println!("├───────┼───────┼───────┤");
            }
        }
        println!("└───────┴───────┴───────┘");
    }
}

/// Example Sudoku puzzles for testing
pub mod puzzles {
    /// Easy Sudoku puzzle
    pub const EASY: [[i32; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    
    /// Medium difficulty puzzle
    pub const MEDIUM: [[i32; 9]; 9] = [
        [0, 0, 0, 6, 0, 0, 4, 0, 0],
        [7, 0, 0, 0, 0, 3, 6, 0, 0],
        [0, 0, 0, 0, 9, 1, 0, 8, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 5, 0, 1, 8, 0, 0, 0, 3],
        [0, 0, 0, 3, 0, 6, 0, 4, 5],
        [0, 4, 0, 2, 0, 0, 0, 6, 0],
        [9, 0, 3, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 1, 0, 0],
    ];
    
    /// Hard puzzle with minimal clues
    pub const HARD: [[i32; 9]; 9] = [
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 4, 0, 7],
        [0, 0, 8, 0, 0, 0, 3, 0, 0],
        [0, 0, 1, 0, 9, 0, 0, 0, 0],
        [3, 0, 0, 4, 0, 0, 2, 0, 0],
        [0, 5, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 8, 0, 6, 0, 0, 0],
    ];
}

fn main() {
    println!("🔢 Sudoku Solver using CSP Framework");
    println!("=====================================\n");
    
    // Solve easy puzzle
    println!("📋 Solving EASY puzzle:");
    solve_puzzle(&puzzles::EASY, "Easy");
    
    println!("\n{}\n", "─".repeat(50));
    
    // Solve medium puzzle  
    println!("📋 Solving MEDIUM puzzle:");
    solve_puzzle(&puzzles::MEDIUM, "Medium");
    
    println!("\n{}\n", "─".repeat(50));
    
    // Solve hard puzzle
    println!("📋 Solving HARD puzzle:");
    solve_puzzle(&puzzles::HARD, "Hard");
}

fn solve_puzzle(puzzle: &[[i32; 9]; 9], difficulty: &str) {
    let mut solver = SudokuSolver::new();
    
    println!("🧩 {} Puzzle:", difficulty);
    solver.print_puzzle(puzzle);
    
    // Set up the puzzle
    solver.set_puzzle(puzzle);
    solver.add_constraints();
    
    // Count clues
    let clue_count = puzzle.iter()
        .flat_map(|row| row.iter())
        .filter(|&&cell| cell != 0)
        .count();
    
    println!("\n📊 Puzzle stats:");
    println!("   • Clues given: {}/81", clue_count);
    println!("   • Empty cells: {}/81", 81 - clue_count);
    
    // Solve
    println!("\n🔍 Solving...");
    let start_time = std::time::Instant::now();
    
    let grid = solver.grid; // Save grid before solver is consumed
    match solver.solve_with_stats() {
        (Some(solution), stats) => {
            let solve_time = start_time.elapsed();
            println!("✅ Solution found in {:?}!\n", solve_time);
            
            // Convert solution to grid format
            let solution_grid = solution.to_grid(&grid);
            
            println!("🎯 Solution:");
            let temp_solver = SudokuSolver::new();
            temp_solver.print_puzzle(&solution_grid);
            
            // Show statistics
            println!("\n📊 Solving Statistics:");
            println!("   • Propagations: {}", stats.propagation_count);
            println!("   • Nodes explored: {}", stats.node_count);
            
            // Verify solution
            if verify_solution(&solution_grid) {
                println!("\n✅ Solution verified correct!");
            } else {
                println!("\n❌ Solution verification failed!");
            }
        }
        (None, stats) => {
            let solve_time = start_time.elapsed();
            println!("❌ No solution found (took {:?})", solve_time);
            println!("📊 Search Statistics:");
            println!("   • Propagations: {}", stats.propagation_count);
            println!("   • Nodes explored: {}", stats.node_count);
        }
    }
}

/// Verify that a completed Sudoku solution is valid
fn verify_solution(solution: &[[i32; 9]; 9]) -> bool {
    // Check all values are 1-9
    for row in solution {
        for &cell in row {
            if !(1..=9).contains(&cell) {
                return false;
            }
        }
    }
    
    // Check rows
    for row in solution {
        let mut seen = vec![false; 10]; // index 0 unused, 1-9 for digits
        for &cell in row {
            if seen[cell as usize] {
                return false; // Duplicate
            }
            seen[cell as usize] = true;
        }
    }
    
    // Check columns
    for col in 0..9 {
        let mut seen = vec![false; 10];
        for row in 0..9 {
            let cell = solution[row][col];
            if seen[cell as usize] {
                return false; // Duplicate
            }
            seen[cell as usize] = true;
        }
    }
    
    // Check 3x3 boxes
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut seen = vec![false; 10];
            for row in 0..3 {
                for col in 0..3 {
                    let cell = solution[box_row * 3 + row][box_col * 3 + col];
                    if seen[cell as usize] {
                        return false; // Duplicate
                    }
                    seen[cell as usize] = true;
                }
            }
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solve_easy_sudoku() {
        let mut solver = SudokuSolver::new();
        solver.set_puzzle(&puzzles::EASY);
        solver.add_constraints();
        
        let solution = solver.solve().expect("Should find solution for easy puzzle");
        assert!(verify_solution(&solution), "Solution should be valid");
    }
    
    #[test]
    fn test_solve_medium_sudoku() {
        let mut solver = SudokuSolver::new();
        solver.set_puzzle(&puzzles::MEDIUM);
        solver.add_constraints();
        
        let solution = solver.solve().expect("Should find solution for medium puzzle");
        assert!(verify_solution(&solution), "Solution should be valid");
    }
    
    #[test]
    fn test_verify_solution() {
        // Test a known valid solution
        let valid_solution = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        
        assert!(verify_solution(&valid_solution));
        
        // Test an invalid solution (duplicate in first row)
        let invalid_solution = [
            [5, 5, 4, 6, 7, 8, 9, 1, 2], // Two 5s in first row
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];
        
        assert!(!verify_solution(&invalid_solution));
    }
    
    #[test]
    fn test_set_clue() {
        let mut solver = SudokuSolver::new();
        solver.set_clue(0, 0, 5);
        solver.add_constraints();
        
        let solution = solver.solve().unwrap();
        assert_eq!(solution[0][0], 5, "Clue should be preserved in solution");
    }
}
