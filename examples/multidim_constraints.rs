//! Multidimensional Element and Table Constraints Example
//!
//! This example demonstrates the new 2D and 3D element and table constraints,
//! as well as 2D and 3D variable array creation.

use selen::prelude::*;
use selen::variables::Val;

fn main() {
    println!("=== Multidimensional Constraints Example ===\n");

    // Example 1: 2D Element Constraint
    println!("Example 1: 2D Element Constraint");
    println!("=================================");
    example_2d_element();

    println!("\n");

    // Example 2: 2D Table Constraint
    println!("Example 2: 2D Table Constraint");
    println!("==============================");
    example_2d_table();

    println!("\n");

    // Example 3: 3D Element Constraint
    println!("Example 3: 3D Element Constraint");
    println!("================================");
    example_3d_element();

    println!("\n");

    // Example 4: 3D Table Constraint
    println!("Example 4: 3D Table Constraint");
    println!("=============================");
    example_3d_table();
}

/// Example 1: 2D Element Constraint - Like accessing matrix[row][col]
/// 
/// Problem: We have a 3x4 matrix of values. We want to find row and column indices
/// such that matrix[row][col] = 7.
fn example_2d_element() {
    let mut m = Model::default();

    // Create a 3x4 matrix with values 1-10
    let matrix = m.ints_2d(3, 4, 1, 10);

    // Set specific values in the matrix (by constraining individual cells)
    // Matrix looks like:
    //  [1, 2, 3, 4]
    //  [5, 6, 7, 8]
    //  [9, 10, 1, 2]
    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let expected = ((i * 4 + j) % 10 + 1) as i32;
            m.new(cell.eq(expected));
        }
    }

    // Create index variables
    let row_idx = m.int(0, 2);  // Row index: 0, 1, or 2
    let col_idx = m.int(0, 3);  // Col index: 0, 1, 2, or 3
    let value = m.int(7, 7);    // We want value = 7

    // Constraint: matrix[row_idx][col_idx] = value
    m.element_2d(&matrix, row_idx, col_idx, value);

    match m.solve() {
        Ok(solution) => {
            let r = solution[row_idx].as_int().unwrap();
            let c = solution[col_idx].as_int().unwrap();
            let v = solution[value].as_int().unwrap();
            println!("✓ Found: matrix[{}][{}] = {}", r, c, v);
            println!("  Matrix value at ({}, {}) = {}", r, c, solution[matrix[r as usize][c as usize]].as_int().unwrap());
        }
        Err(e) => println!("✗ No solution: {:?}", e),
    }
}

/// Example 2: 2D Table Constraint - Each row must match a valid pattern
/// 
/// Problem: We have a 3x3 matrix where each row must match one of two valid patterns.
fn example_2d_table() {
    let mut m = Model::default();

    // Create a 3x3 matrix
    let matrix = m.ints_2d(3, 3, 1, 3);

    // Define valid row patterns
    let valid_tuples = vec![
        vec![Val::int(1), Val::int(1), Val::int(1)],  // All ones
        vec![Val::int(2), Val::int(2), Val::int(2)],  // All twos
        vec![Val::int(1), Val::int(2), Val::int(1)],  // Alternating
    ];

    // Apply table constraint to each row
    m.table_2d(&matrix, valid_tuples);

    match m.solve() {
        Ok(solution) => {
            println!("✓ Solution found:");
            for (i, row) in matrix.iter().enumerate() {
                print!("  Row {}: [", i);
                for (j, &cell) in row.iter().enumerate() {
                    if j > 0 { print!(", "); }
                    print!("{}", solution[cell].as_int().unwrap());
                }
                println!("]");
            }
        }
        Err(e) => println!("✗ No solution: {:?}", e),
    }
}

/// Example 3: 3D Element Constraint - Like accessing cube[depth][row][col]
/// 
/// Problem: We have a 2x2x2 cube. Find indices such that cube[d][r][c] = 5.
fn example_3d_element() {
    let mut m = Model::default();

    // Create a 2x2x2 cube
    let cube = m.ints_3d(2, 2, 2, 1, 8);

    // Set specific values in the cube
    for (d, layer) in cube.iter().enumerate() {
        for (r, row) in layer.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                let expected = (d * 4 + r * 2 + c + 1) as i32;
                m.new(cell.eq(expected));
            }
        }
    }

    // Create index variables
    let depth_idx = m.int(0, 1);  // Depth: 0 or 1
    let row_idx = m.int(0, 1);    // Row: 0 or 1
    let col_idx = m.int(0, 1);    // Col: 0 or 1
    let value = m.int(5, 5);      // We want value = 5

    // Constraint: cube[depth_idx][row_idx][col_idx] = value
    m.element_3d(&cube, depth_idx, row_idx, col_idx, value);

    match m.solve() {
        Ok(solution) => {
            let d = solution[depth_idx].as_int().unwrap();
            let r = solution[row_idx].as_int().unwrap();
            let c = solution[col_idx].as_int().unwrap();
            let v = solution[value].as_int().unwrap();
            println!("✓ Found: cube[{}][{}][{}] = {}", d, r, c, v);
        }
        Err(e) => println!("✗ No solution: {:?}", e),
    }
}

/// Example 4: 3D Table Constraint - Each layer's rows must match valid patterns
/// 
/// Problem: We have a 2x2x2 cube where each row across all layers must match patterns.
fn example_3d_table() {
    let mut m = Model::default();

    // Create a 2x2x2 cube
    let cube = m.ints_3d(2, 2, 2, 1, 2);

    // Define valid row patterns (simple for this example)
    let valid_tuples = vec![
        vec![Val::int(1), Val::int(1)],  // All ones
        vec![Val::int(1), Val::int(2)],  // Alternating
        vec![Val::int(2), Val::int(2)],  // All twos
    ];

    // Apply table constraint to all rows in all layers
    m.table_3d(&cube, valid_tuples);

    match m.solve() {
        Ok(solution) => {
            println!("✓ Solution found:");
            for (d, layer) in cube.iter().enumerate() {
                println!("  Layer {}:", d);
                for (r, row) in layer.iter().enumerate() {
                    print!("    Row {}: [", r);
                    for (c, &cell) in row.iter().enumerate() {
                        if c > 0 { print!(", "); }
                        print!("{}", solution[cell].as_int().unwrap());
                    }
                    println!("]");
                }
            }
        }
        Err(e) => println!("✗ No solution: {:?}", e),
    }
}
