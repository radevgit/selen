//! Float Linear Constraints Example
//! 
//! This example demonstrates the use of float linear constraints
//! (float_lin_eq, float_lin_le, float_lin_ne) which are essential
//! for FlatZinc compatibility.
//! 
//! Example: Loan Balance Calculation
//! Given:
//! - Principal amount with interest rate
//! - Additional interest charges
//! - Payment amount
//! Calculate the remaining balance using:
//!   1.05*principal + 1.03*interest - payment = balance

use selen::prelude::*;

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Float Linear Constraints Example: Loan Balance");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Example 1: Simple loan balance calculation
    println!("Example 1: Calculate loan balance");
    println!("Equation: 1.05*principal + 1.03*interest - payment = balance\n");
    
    let mut m = Model::default();
    
    let principal = m.float(1000.0, 10000.0);
    let interest = m.float(0.0, 1000.0);
    let payment = m.float(0.0, 5000.0);
    let balance = m.float(0.0, 15000.0);
    
    // Post linear equation: 1.05*principal + 1.03*interest - 1.0*payment - 1.0*balance = 0.0
    m.float_lin_eq(
        &[1.05, 1.03, -1.0, -1.0],
        &[principal, interest, payment, balance],
        0.0
    );
    
    // Set known values
    m.new(principal.eq(5000.0));
    m.new(interest.eq(250.0));
    m.new(payment.eq(1000.0));
    
    match m.solve() {
        Ok(solution) => {
            if let (Val::ValF(p), Val::ValF(i), Val::ValF(pay), Val::ValF(bal)) = 
                (solution[principal], solution[interest], solution[payment], solution[balance]) {
                println!("  Principal:  ${:.2}", p);
                println!("  Interest:   ${:.2}", i);
                println!("  Payment:    ${:.2}", pay);
                println!("  Balance:    ${:.2}", bal);
                println!("  Verification: 1.05*{:.2} + 1.03*{:.2} - {:.2} = {:.2}\n", p, i, pay, bal);
            }
        }
        Err(e) => println!("  No solution found: {:?}\n", e),
    }

    // Example 2: Budget constraint with float_lin_le
    println!("Example 2: Budget constraint with multiple items");
    println!("Constraint: 2.5*item1 + 3.75*item2 + 1.25*item3 ≤ 50.0\n");
    
    let mut m2 = Model::default();
    
    let item1 = m2.int(0, 20);
    let item2 = m2.int(0, 20);
    let item3 = m2.int(0, 20);
    
    // Budget constraint: weighted sum must be ≤ 50.0
    m2.float_lin_le(
        &[2.5, 3.75, 1.25],
        &[item1, item2, item3],
        50.0
    );
    
    // Additional constraint: must buy at least 10 total items
    m2.int_lin_eq(&[1, 1, 1], &[item1, item2, item3], 10);
    
    match m2.solve() {
        Ok(solution) => {
            if let (Val::ValI(i1), Val::ValI(i2), Val::ValI(i3)) = 
                (solution[item1], solution[item2], solution[item3]) {
                let cost = 2.5 * i1 as f64 + 3.75 * i2 as f64 + 1.25 * i3 as f64;
                println!("  Item 1 (cost $2.50): {} units", i1);
                println!("  Item 2 (cost $3.75): {} units", i2);
                println!("  Item 3 (cost $1.25): {} units", i3);
                println!("  Total items: {}", i1 + i2 + i3);
                println!("  Total cost: ${:.2} (≤ $50.00)\n", cost);
            }
        }
        Err(e) => println!("  No solution found: {:?}\n", e),
    }

    // Example 3: Inequality constraint with float_lin_ne
    println!("Example 3: Avoid specific value");
    println!("Constraint: 2.0*x + 3.0*y ≠ 12.0\n");
    
    let mut m3 = Model::default();
    
    let x = m3.float(0.0, 10.0);
    let y = m3.float(0.0, 10.0);
    
    // Sum must NOT equal 12.0
    m3.float_lin_ne(&[2.0, 3.0], &[x, y], 12.0);
    
    // Additional constraint to narrow the search
    m3.new(x.eq(3.0));
    
    match m3.solve() {
        Ok(solution) => {
            if let (Val::ValF(x_val), Val::ValF(y_val)) = (solution[x], solution[y]) {
                let sum = 2.0 * x_val + 3.0 * y_val;
                println!("  x = {:.2}", x_val);
                println!("  y = {:.2}", y_val);
                println!("  2.0*x + 3.0*y = {:.2} (≠ 12.0)\n", sum);
            }
        }
        Err(e) => println!("  No solution found: {:?}\n", e),
    }

    // Example 4: Optimization with float linear constraint
    println!("Example 4: Maximize profit with cost constraint");
    println!("Maximize: profit = 5.0*A + 7.5*B");
    println!("Subject to: 2.5*A + 3.0*B ≤ 100.0\n");
    
    let mut m4 = Model::default();
    
    let product_a = m4.int(0, 50);
    let product_b = m4.int(0, 50);
    let profit = m4.float(0.0, 500.0);
    
    // Cost constraint
    m4.float_lin_le(
        &[2.5, 3.0],
        &[product_a, product_b],
        100.0
    );
    
    // Profit calculation
    m4.float_lin_eq(
        &[5.0, 7.5, -1.0],
        &[product_a, product_b, profit],
        0.0
    );
    
    match m4.maximize(profit) {
        Ok(solution) => {
            if let (Val::ValI(a), Val::ValI(b), Val::ValF(p)) = 
                (solution[product_a], solution[product_b], solution[profit]) {
                let cost = 2.5 * a as f64 + 3.0 * b as f64;
                println!("  Product A: {} units (profit: $5.00 each)", a);
                println!("  Product B: {} units (profit: $7.50 each)", b);
                println!("  Total cost: ${:.2} (≤ $100.00)", cost);
                println!("  Total profit: ${:.2}\n", p);
            }
        }
        Err(e) => println!("  No solution found: {:?}\n", e),
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("  Float linear constraints are essential for FlatZinc support!");
    println!("═══════════════════════════════════════════════════════════════");
}
