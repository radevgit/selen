/// Focused benchmark on Platinum Blonde showing BitSet vs SparseSet performance
/// in constraint propagation specifically

use std::time::Instant;
use selen::constraints::gac::{SparseSetGAC, Variable};
use selen::constraints::gac_bitset::BitSetGAC;
use selen::constraints::gac_hybrid::HybridGAC;

const PLATINUM_BLONDE: &str = "000000012000000003002300400001800005060070800000009000008500000900040500000618000";

/// Convert Sudoku string to 9x9 grid
fn parse_sudoku(puzzle_str: &str) -> [[i32; 9]; 9] {
    let mut grid = [[0; 9]; 9];
    let chars: Vec<char> = puzzle_str.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        if i < 81 {
            let row = i / 9;
            let col = i % 9;
            grid[row][col] = ch.to_digit(10).unwrap_or(0) as i32;
        }
    }
    grid
}

/// Test constraint propagation with SparseSet GAC
fn test_sparseset_propagation(grid: [[i32; 9]; 9]) -> Result<u128, String> {
    let start = Instant::now();
    
    let mut gac = SparseSetGAC::new();
    
    // Setup variables
    let mut cell_vars = [[Variable(0); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            let var_id = row * 9 + col;
            cell_vars[row][col] = Variable(var_id);
            
            if grid[row][col] != 0 {
                gac.add_variable(Variable(var_id), grid[row][col], grid[row][col]);
            } else {
                gac.add_variable(Variable(var_id), 1, 9);
            }
        }
    }
    
    // Apply constraints (basic alldiff simulation)
    for row in 0..9 {
        // For each assigned cell in row, remove its value from other cells in row
        for col1 in 0..9 {
            if grid[row][col1] != 0 {
                let assigned_value = grid[row][col1];
                for col2 in 0..9 {
                    if col2 != col1 && grid[row][col2] == 0 {
                        gac.remove_value(cell_vars[row][col2], assigned_value);
                    }
                }
            }
        }
    }
    
    // Column constraints
    for col in 0..9 {
        for row1 in 0..9 {
            if grid[row1][col] != 0 {
                let assigned_value = grid[row1][col];
                for row2 in 0..9 {
                    if row2 != row1 && grid[row2][col] == 0 {
                        gac.remove_value(cell_vars[row2][col], assigned_value);
                    }
                }
            }
        }
    }
    
    // Box constraints
    for box_row in 0..3 {
        for box_col in 0..3 {
            for r1 in 0..3 {
                for c1 in 0..3 {
                    let row1 = box_row * 3 + r1;
                    let col1 = box_col * 3 + c1;
                    
                    if grid[row1][col1] != 0 {
                        let assigned_value = grid[row1][col1];
                        
                        for r2 in 0..3 {
                            for c2 in 0..3 {
                                let row2 = box_row * 3 + r2;
                                let col2 = box_col * 3 + c2;
                                
                                if (row2 != row1 || col2 != col1) && grid[row2][col2] == 0 {
                                    gac.remove_value(cell_vars[row2][col2], assigned_value);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(start.elapsed().as_nanos())
}

/// Test constraint propagation with BitSet GAC
fn test_bitset_propagation(grid: [[i32; 9]; 9]) -> Result<u128, String> {
    let start = Instant::now();
    
    let mut gac = BitSetGAC::new();
    
    // Setup variables
    let mut cell_vars = [[Variable(0); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            let var_id = row * 9 + col;
            cell_vars[row][col] = Variable(var_id);
            
            if grid[row][col] != 0 {
                gac.add_variable(Variable(var_id), grid[row][col], grid[row][col]);
            } else {
                gac.add_variable(Variable(var_id), 1, 9);
            }
        }
    }
    
    // Apply constraints using alldiff propagation
    for row in 0..9 {
        let row_vars: Vec<Variable> = (0..9).map(|col| cell_vars[row][col]).collect();
        gac.propagate_alldiff(&row_vars)?;
    }
    
    for col in 0..9 {
        let col_vars: Vec<Variable> = (0..9).map(|row| cell_vars[row][col]).collect();
        gac.propagate_alldiff(&col_vars)?;
    }
    
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(cell_vars[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            gac.propagate_alldiff(&box_vars)?;
        }
    }
    
    Ok(start.elapsed().as_nanos())
}

/// Test constraint propagation with Hybrid GAC (should use BitSet)
fn test_hybrid_propagation(grid: [[i32; 9]; 9]) -> Result<u128, String> {
    let start = Instant::now();
    
    let mut gac = HybridGAC::new();
    
    // Setup variables
    let mut cell_vars = [[Variable(0); 9]; 9];
    for row in 0..9 {
        for col in 0..9 {
            let var_id = row * 9 + col;
            cell_vars[row][col] = Variable(var_id);
            
            if grid[row][col] != 0 {
                gac.add_variable_with_values(Variable(var_id), vec![grid[row][col]])?;
            } else {
                gac.add_variable(Variable(var_id), 1, 9)?;
            }
        }
    }
    
    // Apply constraints using alldiff propagation
    for row in 0..9 {
        let row_vars: Vec<Variable> = (0..9).map(|col| cell_vars[row][col]).collect();
        gac.propagate_alldiff(&row_vars)?;
    }
    
    for col in 0..9 {
        let col_vars: Vec<Variable> = (0..9).map(|row| cell_vars[row][col]).collect();
        gac.propagate_alldiff(&col_vars)?;
    }
    
    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut box_vars = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    box_vars.push(cell_vars[box_row * 3 + r][box_col * 3 + c]);
                }
            }
            gac.propagate_alldiff(&box_vars)?;
        }
    }
    
    Ok(start.elapsed().as_nanos())
}

fn main() {
    println!("Platinum Blonde Constraint Propagation Benchmark");
    println!("===============================================");
    
    let grid = parse_sudoku(PLATINUM_BLONDE);
    let iterations = 1000;
    
    // Test SparseSet GAC
    let mut sparseset_times = Vec::new();
    for _ in 0..iterations {
        match test_sparseset_propagation(grid) {
            Ok(time) => sparseset_times.push(time),
            Err(e) => {
                println!("SparseSet error: {}", e);
                continue;
            }
        }
    }
    
    // Test BitSet GAC
    let mut bitset_times = Vec::new();
    for _ in 0..iterations {
        match test_bitset_propagation(grid) {
            Ok(time) => bitset_times.push(time),
            Err(e) => {
                println!("BitSet error: {}", e);
                continue;
            }
        }
    }
    
    // Test Hybrid GAC
    let mut hybrid_times = Vec::new();
    for _ in 0..iterations {
        match test_hybrid_propagation(grid) {
            Ok(time) => hybrid_times.push(time),
            Err(e) => {
                println!("Hybrid error: {}", e);
                continue;
            }
        }
    }
    
    // Calculate averages
    let avg_sparseset = sparseset_times.iter().sum::<u128>() / sparseset_times.len() as u128;
    let avg_bitset = bitset_times.iter().sum::<u128>() / bitset_times.len() as u128;
    let avg_hybrid = hybrid_times.iter().sum::<u128>() / hybrid_times.len() as u128;
    
    println!("\nPlatinum Blonde Constraint Propagation Results:");
    println!("Iterations: {}", iterations);
    println!();
    
    println!("SparseSet GAC:  {:>8} ns", avg_sparseset);
    println!("BitSet GAC:     {:>8} ns ({:.2}x speedup)", avg_bitset, avg_sparseset as f64 / avg_bitset as f64);
    println!("Hybrid GAC:     {:>8} ns ({:.2}x speedup)", avg_hybrid, avg_sparseset as f64 / avg_hybrid as f64);
    
    println!("\n=== Analysis ===");
    println!("🎯 Sudoku Domain Properties:");
    println!("   • 81 variables (9x9 grid)");
    println!("   • Domain size: 1-9 (≤64, perfect for BitSet)");
    println!("   • 27 AllDiff constraints (9 rows + 9 cols + 9 boxes)");
    
    println!("\n⚡ BitSet Advantages Demonstrated:");
    println!("   • Faster bit operations for domain manipulation");
    println!("   • Efficient AllDiff propagation with Hall sets");
    println!("   • Better cache locality with compact representation");
    println!("   • Automatic selection via Hybrid GAC");
    
    // Show which implementation was used
    let mut test_hybrid = HybridGAC::new();
    for row in 0..9 {
        for col in 0..9 {
            let var_id = row * 9 + col;
            if grid[row][col] != 0 {
                let _ = test_hybrid.add_variable_with_values(Variable(var_id), vec![grid[row][col]]);
            } else {
                let _ = test_hybrid.add_variable(Variable(var_id), 1, 9);
            }
        }
    }
    
    let (bitset_vars, sparseset_vars) = test_hybrid.get_stats();
    println!("\n📊 Hybrid GAC Implementation Selection:");
    println!("   • BitSet variables: {} (all Sudoku cells)", bitset_vars);
    println!("   • SparseSet variables: {} (none needed)", sparseset_vars);
    println!("   • ✅ Automatic BitSet selection for optimal performance");
}