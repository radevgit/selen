use cspsolver::prelude::*;

fn main() {
    println!("🧪 Table Constraint Verification Tests");
    println!("=====================================");

    test_table_constraint_basic();
    test_table_constraint_no_solution();
    test_table_constraint_filtering();
}

/// Test basic table constraint functionality
fn test_table_constraint_basic() {
    println!("\n✅ Test 1: Basic table constraint functionality");
    
    let mut m = Model::default();
    let x = m.int(1, 3);
    let y = m.int(1, 3);
    
    // Simple table: only (1,1), (2,2), (3,3) allowed
    let table_data = vec![
        vec![int(1), int(1)],
        vec![int(2), int(2)],
        vec![int(3), int(3)],
    ];
    
    post!(m, table([x, y], table_data));
    
    if let Ok(solution) = m.solve() {
        let x_val = match solution[x] { Val::ValI(i) => i, Val::ValF(f) => f as i32 };
        let y_val = match solution[y] { Val::ValI(i) => i, Val::ValF(f) => f as i32 };
        
        println!("   Solution found: x={}, y={}", x_val, y_val);
        
        // Verify solution is in the table
        let valid = (x_val == 1 && y_val == 1) || 
                   (x_val == 2 && y_val == 2) || 
                   (x_val == 3 && y_val == 3);
        
        if valid {
            println!("   ✓ Solution is valid according to table constraint");
        } else {
            println!("   ❌ Solution violates table constraint!");
        }
    } else {
        println!("   ❌ No solution found (unexpected)");
    }
}

/// Test case where table constraint should make problem unsolvable
fn test_table_constraint_no_solution() {
    println!("\n🚫 Test 2: Table constraint with no valid solutions");
    
    let mut m = Model::default();
    let x = m.int(1, 2);
    let y = m.int(1, 2);
    
    // Table only allows (3,3) but variables are constrained to [1,2]
    let table_data = vec![
        vec![int(3), int(3)],
    ];
    
    post!(m, table([x, y], table_data));
    
    match m.solve() {
        Ok(_) => {
            println!("   ❌ Solution found when none should exist!");
        }
        Err(_) => {
            println!("   ✓ No solution found (as expected - table constraint correctly filtered impossible combinations)");
        }
    }
}

/// Test table constraint domain filtering
fn test_table_constraint_filtering() {
    println!("\n🔍 Test 3: Table constraint domain filtering");
    
    let mut m = Model::default();
    let x = m.int(1, 5);  // Broad domain
    let y = m.int(1, 5);  // Broad domain
    
    // Table only allows specific combinations
    let table_data = vec![
        vec![int(1), int(3)],
        vec![int(2), int(4)],
        vec![int(5), int(1)],
    ];
    
    post!(m, table([x, y], table_data));
    
    // Add constraint that should narrow it down further
    post!(m, x <= int(2));  // This should eliminate (5,1)
    
    if let Ok(solution) = m.solve() {
        let x_val = match solution[x] { Val::ValI(i) => i, Val::ValF(f) => f as i32 };
        let y_val = match solution[y] { Val::ValI(i) => i, Val::ValF(f) => f as i32 };
        
        println!("   Solution found: x={}, y={}", x_val, y_val);
        
        // Should be either (1,3) or (2,4) since x <= 2
        let valid = (x_val == 1 && y_val == 3) || (x_val == 2 && y_val == 4);
        
        if valid && x_val <= 2 {
            println!("   ✓ Solution respects both table constraint and additional constraint x <= 2");
        } else {
            println!("   ❌ Solution violates constraints!");
        }
    } else {
        println!("   ❌ No solution found (unexpected)");
    }
}