# Quick Reference - Multidimensional Constraints

## Creating Multidimensional Variables

### 2D Arrays
```rust
let matrix_i = m.ints_2d(3, 4, 1, 10);       // 3×4 ints [1..10]
let matrix_f = m.floats_2d(3, 4, 0.0, 1.0);  // 3×4 floats [0..1]
let matrix_b = m.bools_2d(3, 4);              // 3×4 booleans
```

### 3D Arrays
```rust
let cube_i = m.ints_3d(2, 3, 4, 1, 10);       // 2×3×4 ints [1..10]
let cube_f = m.floats_3d(2, 3, 4, 0.0, 1.0);  // 2×3×4 floats [0..1]
let cube_b = m.bools_3d(2, 3, 4);              // 2×3×4 booleans
```

## Element Constraints

### 2D Access
```rust
let row = m.int(0, 2);       // row index
let col = m.int(0, 3);       // col index  
let val = m.int(1, 10);      // value
m.element_2d(&matrix, row, col, val);
// Constraint: matrix[row][col] = val
```

### 3D Access
```rust
let d = m.int(0, 1);         // depth index
let r = m.int(0, 2);         // row index
let c = m.int(0, 3);         // col index
let v = m.int(1, 10);        // value
m.element_3d(&cube, d, r, c, v);
// Constraint: cube[d][r][c] = v
```

## Table Constraints

### Define Valid Tuples
```rust
let tuples = vec![
    vec![Val::int(1), Val::int(2), Val::int(3)],
    vec![Val::int(4), Val::int(5), Val::int(6)],
    vec![Val::int(7), Val::int(8), Val::int(9)],
];
```

### Apply to 2D Matrix
```rust
let matrix = m.ints_2d(3, 3, 1, 9);
m.table_2d(&matrix, tuples);
// Each row must match one of the tuples
```

### Apply to 3D Cube
```rust
let cube = m.ints_3d(2, 3, 3, 1, 9);
m.table_3d(&cube, tuples);
// Each row in each layer must match one of the tuples
```

## Common Patterns

### Sudoku (9×9 grid)
```rust
let grid = m.ints_2d(9, 9, 1, 9);  // 9×9 grid of digits 1-9
// Add row constraints, column constraints, etc.
```

### Schedule (workers × time slots)
```rust
let schedule = m.ints_2d(num_workers, num_slots, 0, 1);  // 0=off, 1=on
// Add availability and demand constraints
```

### 3D Scheduling (time × machines × tasks)
```rust
let assignments = m.ints_3d(num_times, num_machines, num_tasks, 0, 1);
// 0=not assigned, 1=assigned
```

### Temperature Grid (time × latitude × longitude)
```rust
let temps = m.floats_3d(24, 10, 10, -10.0, 50.0);  // 24h × 10×10 grid
// Add spatial and temporal constraints
```

## Complete Example

```rust
use selen::prelude::*;
use selen::variables::Val;

fn main() {
    let mut m = Model::default();
    
    // Create a 3×3 matrix
    let matrix = m.ints_2d(3, 3, 1, 9);
    
    // Constraint 1: matrix[1][1] = 5
    let one = m.int(1, 1);
    let five = m.int(5, 5);
    m.element_2d(&matrix, one, one, five);
    
    // Constraint 2: Each row must be [1,2,3], [4,5,6], or [7,8,9]
    let patterns = vec![
        vec![Val::int(1), Val::int(2), Val::int(3)],
        vec![Val::int(4), Val::int(5), Val::int(6)],
        vec![Val::int(7), Val::int(8), Val::int(9)],
    ];
    m.table_2d(&matrix, patterns);
    
    // Solve
    match m.solve() {
        Ok(solution) => {
            println!("Solution found:");
            for (i, row) in matrix.iter().enumerate() {
                print!("Row {}: ", i);
                for cell in row {
                    print!("{} ", solution[*cell].as_int().unwrap());
                }
                println!();
            }
        }
        Err(e) => println!("No solution: {:?}", e),
    }
}
```

## Performance Notes

### When LP Helps Most
- ✅ Multiple element_2d/3d constraints
- ✅ Large matrices (100×100+)
- ✅ Optimization problems with matrix access
- ✅ Complex coordination between constraints

### When LP Overhead is Negligible
- ✅ Single element constraint
- ✅ Small matrices (<10×10)
- ✅ Tight integer domains already
- ✅ Pure constraint satisfaction (no objective)

**Default:** LP solver is always enabled and automatically optimizes index computations.

## API Summary

| Method | Type | Parameters | Returns |
|--------|------|-----------|---------|
| `ints_2d` | Factory | (rows, cols, min, max) | `Vec<Vec<VarId>>` |
| `floats_2d` | Factory | (rows, cols, min, max) | `Vec<Vec<VarId>>` |
| `bools_2d` | Factory | (rows, cols) | `Vec<Vec<VarId>>` |
| `ints_3d` | Factory | (d, rows, cols, min, max) | `Vec<Vec<Vec<VarId>>>` |
| `floats_3d` | Factory | (d, rows, cols, min, max) | `Vec<Vec<Vec<VarId>>>` |
| `bools_3d` | Factory | (d, rows, cols) | `Vec<Vec<Vec<VarId>>>` |
| `element_2d` | Constraint | (matrix, row_idx, col_idx, val) | `PropId` |
| `element_3d` | Constraint | (cube, d_idx, r_idx, c_idx, val) | `PropId` |
| `table_2d` | Constraint | (matrix, tuples) | `Vec<PropId>` |
| `table_3d` | Constraint | (cube, tuples) | `Vec<PropId>` |

## Tips & Tricks

### Tip 1: Access All Cells
```rust
for (i, row) in matrix.iter().enumerate() {
    for (j, &cell) in row.iter().enumerate() {
        // cell is VarId at [i][j]
    }
}
```

### Tip 2: Extract and Use Values
```rust
match m.solve() {
    Ok(solution) => {
        let value = solution[matrix[i][j]].as_int().unwrap();
    }
    Err(e) => {}
}
```

### Tip 3: Dynamic Indexing
```rust
// Create index variables that can be optimized
let best_row = m.int(0, 8);    // Find best row
let best_col = m.int(0, 8);    // Find best col
m.element_2d(&matrix, best_row, best_col, best_value);
```

### Tip 4: Multiple Tables
```rust
let table1 = vec![...];
let table2 = vec![...];
m.table_2d(&matrix1, table1);
m.table_2d(&matrix2, table2);
// Both constraints must be satisfied
```

## Troubleshooting

### "No solution found"
- ✓ Check element constraint bounds match matrix dimensions
- ✓ Verify table tuples are valid
- ✓ Ensure index variables have correct domains

### "Index out of bounds"
- ✓ element_2d: row ∈ [0, rows-1], col ∈ [0, cols-1]
- ✓ element_3d: depth ∈ [0, depth-1], row ∈ [0, rows-1], col ∈ [0, cols-1]

### "Slow performance"
- ✓ Try reducing matrix size
- ✓ Add more constraints to prune search space
- ✓ Check if LP solver is active (default: yes)
- ✓ Profile with `cargo flamegraph`

## See Also

- Complete example: `examples/multidim_constraints.rs`
- LP integration details: `docs/lp_2d_constraints_analysis.rs`
- Feature reference: `docs/multidim_constraints_summary.rs`
- Implementation report: `IMPLEMENTATION_REPORT.md`
