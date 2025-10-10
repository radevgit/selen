// Loan Problem - Quarterly Loan Repayment Example
//
// Original MiniZinc: loan.mzn with I=0.04, P=1000, R=260 (from loan1.dzn)
// Expected Solution: B4 ≈ 65.78 (from Coin-BC solver)
//
// This example demonstrates:
//   - Float linear constraints with small coefficients (I=0.04)
//   - Multiplication constraints for interest calculations
//   - Proper handling of financial calculations with tolerance-based propagation
//
// Note: This example uses Selen's tolerance-based float propagation which
// properly handles small float values and accumulated rounding errors.

use selen::prelude::*;

fn main() {
    println!("=== Loan Problem Test (Selen Native API) ===\n");
    
    let mut model = Model::default();
    
    // ===== VARIABLES =====
    println!("Creating variables...");
    
    // Output variables (from FlatZinc)
    // Note: Using reasonable bounds instead of infinity to help solver
    // With data: P=1000, R=260, I=0.04, we expect B4≈65.78
    let r = model.float(-10000.0, 10000.0);  // Repayment per quarter
    let p = model.float(-10000.0, 10000.0);  // Principal borrowed
    let i = model.float(0.0, 10.0);           // Interest rate 0-10%
    let b1 = model.float(-10000.0, 10000.0);  // Balance after Q1
    let b2 = model.float(-10000.0, 10000.0);  // Balance after Q2
    let b3 = model.float(-10000.0, 10000.0);  // Balance after Q3
    let b4 = model.float(-10000.0, 10000.0);  // Balance after Q4
    
    // Introduced variables (from FlatZinc compilation)
    let x1 = model.float(1.0, 11.0);          // 1 + I (bounded 1-11)
    let x2 = model.float(-110000.0, 110000.0); // P * X1 (max ~10000*11)
    let x6 = model.float(-110000.0, 110000.0); // B1 * X1
    let x8 = model.float(-110000.0, 110000.0); // B2 * X1
    let x10 = model.float(-110000.0, 110000.0); // B3 * X1
    
    println!("  11 float variables created (7 unbounded, 4 bounded)\n");
    
    // ===== DATA CONSTRAINTS (from loan1.dzn) =====
    println!("Adding data constraints from loan1.dzn...");
    model.new(p.eq(1000.0));
    model.new(r.eq(260.0));
    model.new(i.eq(0.04));
    println!("  P = 1000.0 (principal borrowed)");
    println!("  R = 260.0 (quarterly repayment)");
    println!("  I = 0.04 (4% interest rate)\n");
    
    // ===== CONSTRAINTS =====
    println!("Posting constraints...");
    
    // From FlatZinc: constraint float_lin_eq([1.0,-1.0],[I,X_INTRODUCED_1_],-1.0);
    // Meaning: 1.0*I + (-1.0)*X1 = -1.0  =>  I - X1 = -1.0  =>  X1 = I + 1
    model.lin_eq(&[1.0, -1.0], &[i, x1], -1.0);
    println!("  1. X1 = I + 1  (convert interest to multiplier)");
    
    // From FlatZinc: constraint float_times(P,X_INTRODUCED_1_,X_INTRODUCED_2_);
    // Meaning: X2 = P * X1
    let x2_calc = model.mul(p, x1);
    model.new(x2.eq(x2_calc));
    println!("  2. X2 = P * X1");    // From FlatZinc: constraint float_lin_eq([1.0,-1.0,1.0],[B1,X_INTRODUCED_2_,R],-0.0);
    // Meaning: 1.0*B1 + (-1.0)*X2 + 1.0*R = 0  =>  B1 = X2 - R
    model.lin_eq(&[1.0, -1.0, 1.0], &[b1, x2, r], 0.0);
    println!("  3. B1 = X2 - R  (balance after Q1)");
    
    // From FlatZinc: constraint float_times(B1,X_INTRODUCED_1_,X_INTRODUCED_6_);
    let x6_result = model.mul(b1, x1);
    model.new(x6.eq(x6_result));
    println!("  4. X6 = B1 * X1");
    
    // From FlatZinc: constraint float_lin_eq([1.0,-1.0,1.0],[B2,X_INTRODUCED_6_,R],-0.0);
    model.lin_eq(&[1.0, -1.0, 1.0], &[b2, x6, r], 0.0);
    println!("  5. B2 = X6 - R  (balance after Q2)");
    
    // From FlatZinc: constraint float_times(B2,X_INTRODUCED_1_,X_INTRODUCED_8_);
    let x8_result = model.mul(b2, x1);
    model.new(x8.eq(x8_result));
    println!("  6. X8 = B2 * X1");
    
    // From FlatZinc: constraint float_lin_eq([1.0,-1.0,1.0],[B3,X_INTRODUCED_8_,R],-0.0);
    model.lin_eq(&[1.0, -1.0, 1.0], &[b3, x8, r], 0.0);
    println!("  7. B3 = X8 - R  (balance after Q3)");
    
    // From FlatZinc: constraint float_times(B3,X_INTRODUCED_1_,X_INTRODUCED_10_);
    let x10_result = model.mul(b3, x1);
    model.new(x10.eq(x10_result));
    println!("  8. X10 = B3 * X1");
    
    // From FlatZinc: constraint float_lin_eq([1.0,-1.0,1.0],[B4,X_INTRODUCED_10_,R],-0.0);
    model.lin_eq(&[1.0, -1.0, 1.0], &[b4, x10, r], 0.0);
    println!("  9. B4 = X10 - R  (balance after Q4)\n");
    
    println!("Total: 9 constraints posted\n");
    
    // ===== SOLVE =====
    println!("Solving...\n");
    
    match model.solve() {
        Ok(solution) => {
            println!("=== SOLUTION FOUND ===\n");
            
            // Extract float values from solution using get method
            let r_val: f64 = solution.get(r);
            let p_val: f64 = solution.get(p);
            let i_val: f64 = solution.get(i);
            let b1_val: f64 = solution.get(b1);
            let b2_val: f64 = solution.get(b2);
            let b3_val: f64 = solution.get(b3);
            let b4_val: f64 = solution.get(b4);
            let x1_val: f64 = solution.get(x1);
            
            println!("Primary Variables:");
            println!("  P (Principal)       = {:.4}", p_val);
            println!("  I (Interest %)      = {:.4}", i_val);
            println!("  R (Repayment/Q)     = {:.4}", r_val);
            println!("  X1 (1 + I)          = {:.4}", x1_val);
            println!();
            println!("Balance Variables:");
            println!("  B1 (after Q1)       = {:.4}", b1_val);
            println!("  B2 (after Q2)       = {:.4}", b2_val);
            println!("  B3 (after Q3)       = {:.4}", b3_val);
            println!("  B4 (after Q4/final) = {:.4}", b4_val);
            
            println!("\n=== EXPECTED SOLUTION (Coin-BC) ===");
            println!("  P (Principal)       ≈ 1000.00");
            println!("  I (Interest %)      ≈ 4.00");
            println!("  R (Repayment/Q)     ≈ 260.00");
            println!("  B4 (Final Balance)  ≈ 65.78");
            
            println!("\n=== VERIFICATION ===");
            
            // Check if values are reasonable
            let p_reasonable = p_val.abs() < 100000.0;
            let i_reasonable = i_val >= 0.0 && i_val <= 10.0;
            let r_reasonable = r_val.abs() < 10000.0;
            let b4_reasonable = b4_val.abs() < 10000.0;
            
            if p_reasonable && i_reasonable && r_reasonable && b4_reasonable {
                println!("✅ All values are in reasonable ranges");
            } else {
                println!("❌ EXTREME VALUES DETECTED:");
                if !p_reasonable { println!("   P = {:.2} is extreme", p_val); }
                if !i_reasonable { println!("   I = {:.2} is out of bounds [0, 10]", i_val); }
                if !r_reasonable { println!("   R = {:.2} is extreme", r_val); }
                if !b4_reasonable { println!("   B4 = {:.2} is extreme", b4_val); }
                println!("\n   This indicates Selen's bound inference needs tuning.");
            }
            
            // Check proximity to expected Coin-BC solution
            let p_delta = (p_val - 1000.0).abs();
            let i_delta = (i_val - 4.0).abs();
            let r_delta = (r_val - 260.0).abs();
            let b4_delta = (b4_val - 65.78).abs();
            
            if p_delta < 100.0 && i_delta < 1.0 && r_delta < 50.0 && b4_delta < 100.0 {
                println!("✅ Solution is close to Coin-BC expected values");
            } else {
                println!("⚠️  Solution differs from Coin-BC:");
                println!("   ΔP  = {:.2}", p_delta);
                println!("   ΔI  = {:.2}", i_delta);
                println!("   ΔR  = {:.2}", r_delta);
                println!("   ΔB4 = {:.2}", b4_delta);
            }
            
            // Manual constraint verification
            println!("\n=== MANUAL CONSTRAINT CHECK ===");
            let x1_expected = i_val + 1.0;
            let x1_error = (x1_val - x1_expected).abs();
            println!("  X1 = I + 1?  {:.4} vs {:.4}  (error: {:.6})", x1_val, x1_expected, x1_error);
            
            if x1_error < 0.001 {
                println!("  ✅ X1 constraint satisfied");
            } else {
                println!("  ❌ X1 constraint VIOLATED!");
            }
            
            println!("\n=== NOTES ===");
            println!("This problem is under-constrained (no optimization objective).");
            println!("Different solvers will find different valid solutions.");
            println!("To get realistic values, consider:");
            println!("  1. Adding bounds to P and R (e.g., P in [100, 10000])");
            println!("  2. Adding optimization objective (e.g., minimize B4)");
            println!("  3. Tuning Selen's bound inference for financial problems");
        }
        Err(e) => {
            println!("=== NO SOLUTION FOUND ===");
            println!("Error: {:?}", e);
            println!("\nThis indicates a problem with:");
            println!("  1. Float variable creation with infinite bounds");
            println!("  2. float_lin_eq or float_times constraint propagation");
            println!("  3. Bound inference producing conflicting bounds");
        }
    }
}
