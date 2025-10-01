//! Debug analysis of reification propagation issue

use selen::prelude::*;

#[test]
fn analyze_propagation_order_eq() {
    println!("\n=== Test: int_eq_reif with b=1, x=5 ===");
    
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    println!("Initial domains:");
    println!("  x: [{}, {}]", 1, 10);
    println!("  y: [{}, {}]", 1, 10);
    println!("  b: [0, 1]");
    
    // Post reified constraint FIRST
    println!("\nPosting: int_eq_reif(x, y, b) - meaning b ⇔ (x = y)");
    m.int_eq_reif(x, y, b);
    
    // THEN post constraints
    println!("Posting: b = 1");
    m.new(b.eq(1));
    
    println!("Posting: x = 5");
    m.new(x.eq(5));
    
    println!("\nSolving...");
    match m.solve() {
        Ok(solution) => {
            println!("✓ Solution found:");
            println!("  x = {:?}", solution[x]);
            println!("  y = {:?}", solution[y]);
            println!("  b = {:?}", solution[b]);
            
            if solution[y] == Val::ValI(5) {
                println!("✓ CORRECT: y = 5 (as expected since b=1 ⇒ x=y)");
            } else {
                println!("✗ WRONG: y = {:?} (expected 5)", solution[y]);
            }
        },
        Err(e) => {
            println!("✗ Failed to find solution: {:?}", e);
        }
    }
}

#[test]
fn analyze_propagation_order_ne() {
    println!("\n=== Test: int_ne_reif with b=0, x=5 ===");
    
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    println!("Initial domains:");
    println!("  x: [{}, {}]", 1, 10);
    println!("  y: [{}, {}]", 1, 10);
    println!("  b: [0, 1]");
    
    // Post reified constraint FIRST
    println!("\nPosting: int_ne_reif(x, y, b) - meaning b ⇔ (x ≠ y)");
    m.int_ne_reif(x, y, b);
    
    // THEN post constraints
    println!("Posting: b = 0");
    m.new(b.eq(0));
    
    println!("Posting: x = 5");
    m.new(x.eq(5));
    
    println!("\nSolving...");
    match m.solve() {
        Ok(solution) => {
            println!("✓ Solution found:");
            println!("  x = {:?}", solution[x]);
            println!("  y = {:?}", solution[y]);
            println!("  b = {:?}", solution[b]);
            
            if solution[y] == Val::ValI(5) {
                println!("✓ CORRECT: y = 5 (as expected since b=0 ⇒ x=y for ne_reif)");
            } else {
                println!("✗ WRONG: y = {:?} (expected 5)", solution[y]);
            }
        },
        Err(e) => {
            println!("✗ Failed to find solution: {:?}", e);
        }
    }
}

#[test]
fn compare_prefixed_vs_constrained() {
    println!("\n=== Comparison: Pre-fixed vs Constrained ===");
    
    // Test 1: Pre-fixed (known to work)
    println!("\nTest A: Pre-fixed variables");
    {
        let mut m = Model::default();
        let x = m.int(5, 5);  // pre-fixed
        let y = m.int(1, 10);
        let b = m.int(0, 0);  // pre-fixed
        
        m.int_ne_reif(x, y, b);
        
        match m.solve() {
            Ok(sol) => println!("  Result: x={:?}, y={:?}, b={:?}", sol[x], sol[y], sol[b]),
            Err(e) => println!("  Failed: {:?}", e),
        }
    }
    
    // Test 2: Constrained (problematic)
    println!("\nTest B: Constrained variables");
    {
        let mut m = Model::default();
        let x = m.int(1, 10);
        let y = m.int(1, 10);
        let b = m.bool();
        
        m.int_ne_reif(x, y, b);
        m.new(b.eq(0));
        m.new(x.eq(5));
        
        match m.solve() {
            Ok(sol) => println!("  Result: x={:?}, y={:?}, b={:?}", sol[x], sol[y], sol[b]),
            Err(e) => println!("  Failed: {:?}", e),
        }
    }
}

#[test]
fn test_reverse_constraint_order() {
    println!("\n=== Test: Posting constraints BEFORE reification ===");
    
    let mut m = Model::default();
    
    let x = m.int(1, 10);
    let y = m.int(1, 10);
    let b = m.bool();
    
    // Post constraints FIRST
    println!("Posting: b = 0");
    m.new(b.eq(0));
    
    println!("Posting: x = 5");
    m.new(x.eq(5));
    
    // THEN post reified constraint
    println!("\nPosting: int_ne_reif(x, y, b)");
    m.int_ne_reif(x, y, b);
    
    println!("\nSolving...");
    match m.solve() {
        Ok(solution) => {
            println!("✓ Solution found:");
            println!("  x = {:?}", solution[x]);
            println!("  y = {:?}", solution[y]);
            println!("  b = {:?}", solution[b]);
            
            if solution[y] == Val::ValI(5) {
                println!("✓ CORRECT: y = 5");
            } else {
                println!("✗ WRONG: y = {:?} (expected 5)", solution[y]);
            }
        },
        Err(e) => {
            println!("✗ Failed: {:?}", e);
        }
    }
}
