//! Reification Comparison Constraints Example
//!
//! This example demonstrates the use of reified comparison constraints
//! (int_lt_reif, int_le_reif, int_gt_reif, int_ge_reif).
//!
//! Reification allows us to represent constraints as boolean variables,
//! enabling conditional logic and complex constraint combinations.

use selen::prelude::*;

fn main() {
    println!("=== Reification Comparison Constraints Example ===\n");

    // Example 1: Conditional Constraint (if-then-else logic)
    // If x > 5, then y must equal 10, otherwise y must equal 0
    example_conditional_constraint();

    // Example 2: Maximum with Reification
    // z = max(x, y) using reification
    example_max_with_reification();

    // Example 3: Counting Constraints
    // Count how many variables in an array are greater than a threshold
    example_counting_greater_than();

    // Example 4: Ordering Constraints
    // Ensure at least 2 out of 3 constraints hold
    example_partial_ordering();

    // Example 5: Range Membership
    // Check if a value is within a range using reification
    example_range_membership();
}

fn example_conditional_constraint() {
    println!("Example 1: Conditional Constraint (if x > 5 then y = 10 else y = 0)");
    
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    
    // Create boolean: b ⇔ (x > 5)
    let b = m.bool();
    let five = m.int(5, 5);
    m.int_gt_reif(x, five, b);
    
    // If b = 1 (x > 5), then y = 10
    // If b = 0 (x ≤ 5), then y = 0
    let ten = m.int(10, 10);
    let zero = m.int(0, 0);
    
    let y_eq_10 = m.bool();
    let y_eq_0 = m.bool();
    m.int_eq_reif(y, ten, y_eq_10);
    m.int_eq_reif(y, zero, y_eq_0);
    
    // b implies y_eq_10, and not(b) implies y_eq_0
    // This is equivalent to: (b → y_eq_10) ∧ (¬b → y_eq_0)
    // Using bool_clause: (¬b ∨ y_eq_10) ∧ (b ∨ y_eq_0)
    m.bool_clause(&[y_eq_10], &[b]);  // ¬b ∨ y_eq_10
    m.bool_clause(&[b, y_eq_0], &[]); // b ∨ y_eq_0
    
    // Let's try x = 3 (should give y = 0)
    let three = m.int(3, 3);
    m.new(x.eq(three));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = if let Val::ValI(v) = sol[x] { v } else { 0 };
            let y_val = if let Val::ValI(v) = sol[y] { v } else { 0 };
            let b_val = if let Val::ValI(v) = sol[b] { v } else { 0 };
            println!("  x = {}, y = {}, x > 5 = {}", x_val, y_val, b_val != 0);
            println!("  ✓ Since x = {} ≤ 5, y = {} as expected\n", x_val, y_val);
        }
        Err(e) => println!("  No solution: {:?}\n", e),
    }
}

fn example_max_with_reification() {
    println!("Example 2: Maximum with Reification (z = max(x, y))");
    
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let z = m.int(1, 10);
    
    // z = max(x, y) means:
    // 1. z ≥ x and z ≥ y
    // 2. z = x OR z = y
    
    // Ensure z ≥ x and z ≥ y
    let x_le_z = m.bool();
    let y_le_z = m.bool();
    m.int_le_reif(x, z, x_le_z);
    m.int_le_reif(y, z, y_le_z);
    m.new(x_le_z.eq(1));
    m.new(y_le_z.eq(1));
    
    // z equals x OR z equals y
    let z_eq_x = m.bool();
    let z_eq_y = m.bool();
    m.int_eq_reif(z, x, z_eq_x);
    m.int_eq_reif(z, y, z_eq_y);
    m.bool_clause(&[z_eq_x, z_eq_y], &[]);  // At least one must be true
    
    // Set x = 7, y = 3
    m.new(x.eq(7));
    m.new(y.eq(3));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = if let Val::ValI(v) = sol[x] { v } else { 0 };
            let y_val = if let Val::ValI(v) = sol[y] { v } else { 0 };
            let z_val = if let Val::ValI(v) = sol[z] { v } else { 0 };
            println!("  x = {}, y = {}, z = max(x, y) = {}", x_val, y_val, z_val);
            println!("  ✓ z = {} is the maximum\n", z_val);
        }
        Err(e) => println!("  No solution: {:?}\n", e),
    }
}

fn example_counting_greater_than() {
    println!("Example 3: Counting (count how many of x, y, z are > 5)");
    
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    let five = m.int(5, 5);
    
    // Create booleans for each comparison
    let x_gt_5 = m.bool();
    let y_gt_5 = m.bool();
    let z_gt_5 = m.bool();
    
    m.int_gt_reif(x, five, x_gt_5);
    m.int_gt_reif(y, five, y_gt_5);
    m.int_gt_reif(z, five, z_gt_5);
    
    // Count = sum of booleans (we want exactly 2 to be greater than 5)
    // Since booleans are 0 or 1, we can sum them
    // x_gt_5 + y_gt_5 + z_gt_5 = 2
    m.int_lin_eq(&[1, 1, 1], &[x_gt_5, y_gt_5, z_gt_5], 2);
    
    // Also add constraint that x + y + z = 20
    m.int_lin_eq(&[1, 1, 1], &[x, y, z], 20);
    
    match m.solve() {
        Ok(sol) => {
            let x_val = if let Val::ValI(v) = sol[x] { v } else { 0 };
            let y_val = if let Val::ValI(v) = sol[y] { v } else { 0 };
            let z_val = if let Val::ValI(v) = sol[z] { v } else { 0 };
            
            println!("  x = {}, y = {}, z = {}", x_val, y_val, z_val);
            println!("  x + y + z = {}", x_val + y_val + z_val);
            
            let actual_count = ((x_val > 5) as i32) + ((y_val > 5) as i32) + ((z_val > 5) as i32);
            println!("  ✓ Exactly {} values are > 5 (required: 2)\n", actual_count);
        }
        Err(e) => println!("  No solution: {:?}\n", e),
    }
}

fn example_partial_ordering() {
    println!("Example 4: Partial Ordering (at least 2 of: x < y, y < z, x < z must hold)");
    
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    let y = m.int(0, 10);
    let z = m.int(0, 10);
    
    // Create boolean variables for each comparison
    let x_lt_y = m.bool();
    let y_lt_z = m.bool();
    let x_lt_z = m.bool();
    
    m.int_lt_reif(x, y, x_lt_y);
    m.int_lt_reif(y, z, y_lt_z);
    m.int_lt_reif(x, z, x_lt_z);
    
    // At least 2 must be true: x_lt_y + y_lt_z + x_lt_z >= 2
    // We can express this as: x_lt_y + y_lt_z + x_lt_z = 2 or 3
    // For simplicity, let's require exactly 2
    m.int_lin_eq(&[1, 1, 1], &[x_lt_y, y_lt_z, x_lt_z], 2);
    
    // Add some values to make it interesting
    m.new(x.eq(3));
    m.new(y.eq(5));
    m.new(z.eq(4));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = if let Val::ValI(v) = sol[x] { v } else { 0 };
            let y_val = if let Val::ValI(v) = sol[y] { v } else { 0 };
            let z_val = if let Val::ValI(v) = sol[z] { v } else { 0 };
            let x_lt_y_val = if let Val::ValI(v) = sol[x_lt_y] { v } else { 0 };
            let y_lt_z_val = if let Val::ValI(v) = sol[y_lt_z] { v } else { 0 };
            let x_lt_z_val = if let Val::ValI(v) = sol[x_lt_z] { v } else { 0 };
            
            println!("  x = {}, y = {}, z = {}", x_val, y_val, z_val);
            println!("  x < y: {}", x_lt_y_val != 0);
            println!("  y < z: {}", y_lt_z_val != 0);
            println!("  x < z: {}", x_lt_z_val != 0);
            
            let true_count = (x_lt_y_val != 0) as i32 + (y_lt_z_val != 0) as i32 + (x_lt_z_val != 0) as i32;
            println!("  ✓ {} comparisons are true (at least 2)\n", true_count);
        }
        Err(e) => println!("  No solution: {:?}\n", e),
    }
}

fn example_range_membership() {
    println!("Example 5: Range Membership (check if 3 ≤ x ≤ 7)");
    
    let mut m = Model::default();
    
    let x = m.int(0, 10);
    
    // Check if x is in range [3, 7]
    let three = m.int(3, 3);
    let seven = m.int(7, 7);
    
    let x_ge_3 = m.bool();
    let x_le_7 = m.bool();
    
    m.int_ge_reif(x, three, x_ge_3);
    m.int_le_reif(x, seven, x_le_7);
    
    // in_range ⇔ (x ≥ 3 ∧ x ≤ 7)
    let in_range = m.bool_and(&[x_ge_3, x_le_7]);
    
    // Require that x is in range
    m.new(in_range.eq(1));
    
    // Set x = 5 (should be in range)
    m.new(x.eq(5));
    
    match m.solve() {
        Ok(sol) => {
            let x_val = if let Val::ValI(v) = sol[x] { v } else { 0 };
            let in_range_val = if let Val::ValI(v) = sol[in_range] { v } else { 0 };
            
            println!("  x = {}", x_val);
            println!("  x in [3, 7]: {}", in_range_val != 0);
            println!("  ✓ x is {} the range [3, 7]\n", 
                     if in_range_val != 0 { "in" } else { "outside" });
        }
        Err(e) => println!("  No solution: {:?}\n", e),
    }
}
