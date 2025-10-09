//! Enumerate All Solutions Example
//!
//! Demonstrates how to find ALL solutions to a constraint problem,
//! not just the first one.

use selen::prelude::*;

fn main() {
    let mut m = Model::default();

    // Create variables
    let x = m.int(1, 10);       // Integer variable from 1 to 10
    let y = m.int(5, 15);       // Integer variable from 5 to 15

    // Add constraints
    m.new(x.lt(y));             // x must be less than y
    m.new(x.add(y).eq(12));     // x + y must equal 12
    
    // Enumerate ALL solutions (not just one)
    let mut count = 0;
    for solution in m.enumerate() {
        count += 1;
        println!("Solution {}: x = {:?}, y = {:?}", count, solution[x], solution[y]);
    }
    
    println!("\nTotal solutions found: {}", count);
}
