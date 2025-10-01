//! Test with detailed propagation tracing

use selen::prelude::*;

#[test]
fn trace_propagation_steps() {
    println!("\n=== Detailed Propagation Trace ===");
    
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    println!("Step 1: Post int_ne_reif(x, y, b)");
    m.int_ne_reif(x, y, b);
    println!("  Propagators registered: {}", m.constraint_count());
    
    println!("\nStep 2: Post b = 0");
    m.new(b.eq(0));
    println!("  Propagators registered: {}", m.constraint_count());
    
    println!("\nStep 3: Post x = 5");
    m.new(x.eq(5));
    println!("  Propagators registered: {}", m.constraint_count());
    
    println!("\nStep 4: Solve");
    match m.solve() {
        Ok(solution) => {
            println!("  Solution found:");
            println!("    x = {:?}", solution[x]);
            println!("    y = {:?}", solution[y]);
            println!("    b = {:?}", solution[b]);
            println!("    Propagation count: {}", solution.stats.propagation_count);
            
            if solution[y] == Val::ValI(5) {
                println!("  ✓ CORRECT");
            } else {
                println!("  ✗ WRONG - y should be 5");
            }
        },
        Err(e) => {
            println!("  ✗ Failed: {:?}", e);
        }
    }
}

#[test]
fn trace_reverse_order() {
    println!("\n=== Reverse Order Trace ===");
    
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    println!("Step 1: Post b = 0");
    m.new(b.eq(0));
    println!("  Propagators registered: {}", m.constraint_count());
    
    println!("\nStep 2: Post x = 5");
    m.new(x.eq(5));
    println!("  Propagators registered: {}", m.constraint_count());
    
    println!("\nStep 3: Post int_ne_reif(x, y, b)");
    m.int_ne_reif(x, y, b);
    println!("  Propagators registered: {}", m.constraint_count());
    
    println!("\nStep 4: Solve");
    match m.solve() {
        Ok(solution) => {
            println!("  Solution found:");
            println!("    x = {:?}", solution[x]);
            println!("    y = {:?}", solution[y]);
            println!("    b = {:?}", solution[b]);
            println!("    Propagation count: {}", solution.stats.propagation_count);
            
            if solution[y] == Val::ValI(5) {
                println!("  ✓ CORRECT");
            } else {
                println!("  ✗ WRONG - y should be 5");
            }
        },
        Err(e) => {
            println!("  ✗ Failed: {:?}", e);
        }
    }
}
