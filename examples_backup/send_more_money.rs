//! Send More Money - Cryptarithmetic Puzzle
//! 
//! The classic puzzle: SEND + MORE = MONEY
//! Each letter represents a unique digit (0-9), and the arithmetic must be valid.

use selen::prelude::*;

fn main() {
    println!("🔤 SEND + MORE = MONEY Puzzle");
    println!("==============================");
    
    let mut model = Model::default();
    
    // Create variables for each letter (0-9)
    let s = model.int(1, 9); // S cannot be 0 (leading digit)
    let e = model.int(0, 9);
    let n = model.int(0, 9);
    let d = model.int(0, 9);
    let m = model.int(1, 9); // M cannot be 0 (leading digit)
    let o = model.int(0, 9);
    let r = model.int(0, 9);
    let y = model.int(0, 9);
    
    // All letters must have different values
    let all_vars = vec![s, e, n, d, m, o, r, y];
    model.alldiff(&all_vars);
    
    // Create intermediate calculations using model methods
    // SEND = 1000*S + 100*E + 10*N + D
    let s_thousands = model.mul(s, int(1000));
    let e_hundreds = model.mul(e, int(100));  
    let n_tens = model.mul(n, int(10));
    let send_temp1 = model.add(s_thousands, e_hundreds);
    let send_temp2 = model.add(send_temp1, n_tens);
    let send = model.add(send_temp2, d);
    
    // MORE = 1000*M + 100*O + 10*R + E
    let m_thousands = model.mul(m, int(1000));
    let o_hundreds = model.mul(o, int(100));
    let r_tens = model.mul(r, int(10));
    let more_temp1 = model.add(m_thousands, o_hundreds);
    let more_temp2 = model.add(more_temp1, r_tens);
    let more = model.add(more_temp2, e);
    
    // MONEY = 10000*M + 1000*O + 100*N + 10*E + Y
    let m_ten_thousands = model.mul(m, int(10000));
    let o_thousands = model.mul(o, int(1000));
    let n_hundreds = model.mul(n, int(100));
    let e_tens = model.mul(e, int(10));
    let money_temp1 = model.add(m_ten_thousands, o_thousands);
    let money_temp2 = model.add(money_temp1, n_hundreds);
    let money_temp3 = model.add(money_temp2, e_tens);
    let money = model.add(money_temp3, y);
    
    // SEND + MORE = MONEY
    let sum = model.add(send, more);
    model.new(sum.eq(money));
    
    println!("🔍 Solving...");
    match model.solve() {
        Ok(solution) => {
            println!("✅ Solution found!");
            
            if let (Val::ValI(s_val), Val::ValI(e_val), Val::ValI(n_val), Val::ValI(d_val),
                    Val::ValI(m_val), Val::ValI(o_val), Val::ValI(r_val), Val::ValI(y_val)) = 
                (solution[s], solution[e], solution[n], solution[d],
                 solution[m], solution[o], solution[r], solution[y]) {
                
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
            }
        },
        Err(_) => {
            println!("❌ No solution found!");
        }
    }
}
