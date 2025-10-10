//! Send More Money - Cryptarithmetic Puzzle (New API)
//! 
//! The classic puzzle: SEND + MORE = MONEY
//! Each letter represents a unique digit (0-9), and the arithmetic must be valid.
//!
//! This example demonstrates the new unified constraint API with:
//! - alldiff() function for all-different constraints
//! - add(), mul() functions that return expressions
//! - eq() for posting equality constraints with explicit int() constants

use selen::prelude::*;

fn main() {
    println!("🔤 SEND + MORE = MONEY Puzzle (New API)");
    println!("=======================================");
    
    let mut mm = Model::default();
    
    // Create variables for each letter (0-9)
    let s = mm.int(1, 9); // S cannot be 0 (leading digit)
    let e = mm.int(0, 9);
    let n = mm.int(0, 9);
    let d = mm.int(0, 9);
    let m = mm.int(1, 9); // M cannot be 0 (leading digit)
    let o = mm.int(0, 9);
    let r = mm.int(0, 9);
    let y = mm.int(0, 9);
    
    // All letters must have different values - use method syntax
    let all_vars = vec![s, e, n, d, m, o, r, y];
    mm.alldiff(&all_vars);
    
    // SEND = 1000*S + 100*E + 10*N + D
    // Using new expression-based API with int() for explicit constants
    let send = mm.int(1000, 9999);
    mm.new(add(
        add(
            add(mul(s, int(1000)), mul(e, int(100))),
            mul(n, int(10))
        ),
        d
    ).eq(send));
    
    // MORE = 1000*M + 100*O + 10*R + E
    let more = mm.int(1000, 9999);
    mm.new(add(
        add(
            add(mul(m, int(1000)), mul(o, int(100))),
            mul(r, int(10))
        ),
        e
    ).eq(more));
    
    // MONEY = 10000*M + 1000*O + 100*N + 10*E + Y  
    let money = mm.int(10000, 99999);
    mm.new(add(
        add(
            add(
                add(mul(m, int(10000)), mul(o, int(1000))),
                mul(n, int(100))
            ),
            mul(e, int(10))
        ),
        y
    ).eq(money));
    
    // SEND + MORE = MONEY - using runtime API for clarity
    mm.new(add(send, more).eq(money));
    
    println!("🔍 Solving...");
    match mm.solve() {
        Ok(solution) => {
            println!("✅ Solution found!");
            
            let s_val = solution.get_int(s);
            let e_val = solution.get_int(e);
            let n_val = solution.get_int(n);
            let d_val = solution.get_int(d);
            let m_val = solution.get_int(m);
            let o_val = solution.get_int(o);
            let r_val = solution.get_int(r);
            let y_val = solution.get_int(y);
            
            let send_num = s_val * 1000 + e_val * 100 + n_val * 10 + d_val;
            let more_num = m_val * 1000 + o_val * 100 + r_val * 10 + e_val;
            let money_num = m_val * 10000 + o_val * 1000 + n_val * 100 + e_val * 10 + y_val;
            
            println!("\n📋 Letter assignments:");
            println!("  S={} E={} N={} D={}", s_val, e_val, n_val, d_val);
            println!("  M={} O={} R={} Y={}", m_val, o_val, r_val, y_val);
            println!("\n🔢 Equation:");
            println!("  SEND = {}", send_num);
            println!("  MORE = {}", more_num);
            println!("  MONEY = {}", money_num);
            println!("  {} + {} = {}", send_num, more_num, money_num);
            
            if send_num + more_num == money_num {
                println!("  ✅ Verified: {} + {} = {}", send_num, more_num, money_num);
            } else {
                println!("  ❌ Verification failed!");
            }
        },
        Err(_) => {
            println!("❌ No solution found!");
        }
    }
}
