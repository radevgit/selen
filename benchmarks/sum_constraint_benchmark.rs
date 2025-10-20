//! Benchmark tests for sum constraint propagation
//!
//! This benchmark suite measures the performance of the sum constraint
//! before and after incremental implementation.
//!
//! Run with: `cargo run --release --bin sum_constraint_benchmark`

use std::time::Instant;
use selen::prelude::*;

/// Run a benchmark and report timing
fn benchmark<F>(name: &str, iterations: usize, mut f: F)
where
    F: FnMut(),
{
    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();
    let avg_ms = elapsed.as_secs_f64() * 1000.0 / iterations as f64;
    println!("{:50} | {:6.3} ms/iter | total: {:8.3} ms", name, avg_ms, elapsed.as_secs_f64() * 1000.0);
}

/// Benchmark: Sum constraint on 10 variables with small domains
fn sum_forward_10vars() {
    benchmark("sum_forward_10vars_domain_1_10", 100, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();
        let target = m.int(20, 100);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum constraint on 20 variables
fn sum_forward_20vars() {
    benchmark("sum_forward_20vars_domain_1_10", 50, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..20).map(|_| m.int(1, 10)).collect();
        let target = m.int(50, 200);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum constraint on 50 variables (larger problem)
fn sum_forward_50vars() {
    benchmark("sum_forward_50vars_domain_1_10", 10, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..50).map(|_| m.int(1, 10)).collect();
        let target = m.int(150, 500);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum constraint on 100 variables (very large problem)
fn sum_forward_100vars() {
    benchmark("sum_forward_100vars_domain_1_10", 3, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..100).map(|_| m.int(1, 10)).collect();
        let target = m.int(300, 1000);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum constraint on 200 variables (extreme size)
fn sum_forward_200vars() {
    benchmark("sum_forward_200vars_domain_1_10", 1, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..200).map(|_| m.int(1, 10)).collect();
        let target = m.int(600, 2000);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum constraint on 500 variables (very extreme)
fn sum_forward_500vars() {
    benchmark("sum_forward_500vars_domain_1_10", 1, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..500).map(|_| m.int(1, 10)).collect();
        let target = m.int(2000, 5000);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum with large domain and many variables
fn sum_100vars_large_domain() {
    benchmark("sum_100vars_domain_1_100", 1, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..100).map(|_| m.int(1, 100)).collect();
        let target = m.int(3000, 8000);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Multiple overlapping sum constraints (complex network)
fn multiple_overlapping_sums_50vars() {
    benchmark("multiple_overlapping_sums_50vars", 2, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..50).map(|_| m.int(1, 10)).collect();

        // Create overlapping sum constraints
        let sum1 = sum(&mut m, &vars[0..25]);
        let sum2 = sum(&mut m, &vars[25..50]);
        let sum3 = sum(&mut m, &vars[0..10]);  // Overlaps with sum1
        let sum4 = sum(&mut m, &vars[40..50]); // Overlaps with sum2

        let t1 = m.int(75, 200);
        let t2 = m.int(75, 200);
        let t3 = m.int(30, 80);
        let t4 = m.int(30, 80);

        m.new(sum1.eq(t1));
        m.new(sum2.eq(t2));
        m.new(sum3.eq(t3));
        m.new(sum4.eq(t4));

        let _sol = m.solve();
    });
}

/// Benchmark: Sum with very tight bounds (extreme constraint)
fn sum_50vars_ultra_tight_bounds() {
    benchmark("sum_50vars_ultra_tight_bounds", 2, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..50).map(|_| m.int(1, 10)).collect();

        // Extreme tight constraint: sum must be in tiny range
        let target = m.int(249, 251);  // Out of [50..500], extremely tight!
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));

        let _sol = m.solve();
    });
}

/// Benchmark: Sum with alldiff on larger set
fn sum_with_alldiff_30vars() {
    benchmark("sum_with_alldiff_30vars_domain_1_30", 2, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..30).map(|_| m.int(1, 30)).collect();
        alldiff(&mut m, &vars);
        let target = m.int(350, 550);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum with many small domain variables
fn sum_200vars_wide_domain() {
    benchmark("sum_200vars_domain_1_100", 1, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..200).map(|_| m.int(1, 100)).collect();
        let target = m.int(8000, 15000);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum with alldiff (adds reverse propagation pressure)
fn sum_with_alldiff_10vars() {
    benchmark("sum_with_alldiff_10vars_domain_1_10", 50, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();
        alldiff(&mut m, &vars);
        let target = m.int(40, 60);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum with multiple sums (more complex constraint network)
fn multiple_sums_10vars() {
    benchmark("multiple_sums_10vars_domain_1_10", 50, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();

        // Multiple sum constraints on different subsets
        let sum1 = sum(&mut m, &vars[0..5]);
        let sum2 = sum(&mut m, &vars[5..10]);
        let target1 = m.int(15, 40);
        let target2 = m.int(15, 40);

        m.new(sum1.eq(target1));
        m.new(sum2.eq(target2));

        let _sol = m.solve();
    });
}

/// Benchmark: 4x4 Sudoku puzzle
fn sudoku_4x4() {
    benchmark("sudoku_4x4", 20, || {
        let mut m = Model::default();

        // 4x4 grid
        let mut grid = Vec::new();
        for _ in 0..4 {
            let row = (0..4).map(|_| m.int(1, 4)).collect::<Vec<_>>();
            grid.push(row);
        }

        // Row constraints
        for row in 0..4 {
            let row_vars: Vec<_> = (0..4).map(|col| grid[row][col]).collect();
            alldiff(&mut m, &row_vars);
        }

        // Column constraints
        for col in 0..4 {
            let col_vars: Vec<_> = (0..4).map(|row| grid[row][col]).collect();
            alldiff(&mut m, &col_vars);
        }

        let _sol = m.solve();
    });
}

/// Benchmark: Sum on wide domain (tests with larger numbers)
fn sum_large_domain() {
    benchmark("sum_10vars_domain_1_100", 50, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 100)).collect();
        let target = m.int(200, 1000);
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));
        let _sol = m.solve();
    });
}

/// Benchmark: Sum with tight bounds (forces more propagation)
fn sum_tight_bounds() {
    benchmark("sum_10vars_tight_bounds", 50, || {
        let mut m = Model::default();
        let vars: Vec<_> = (0..10).map(|_| m.int(1, 10)).collect();

        // Tight constraint: sum must be very close to min/max
        let target = m.int(45, 50);  // Out of [10..100], very tight!
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));

        let _sol = m.solve();
    });
}

/// Benchmark: Sum with many search tree nodes
fn sum_deep_search_tree() {
    benchmark("sum_5vars_deep_search", 100, || {
        let mut m = Model::default();

        // Create choice points with explicit assignments
        let x1 = m.int(1, 3);
        let x2 = m.int(1, 3);
        let x3 = m.int(1, 3);
        let x4 = m.int(1, 3);
        let x5 = m.int(1, 3);

        let vars = vec![x1, x2, x3, x4, x5];
        let target = m.int(10, 15);

        // Add sum constraint
        let s = sum(&mut m, &vars);
        m.new(s.eq(target));

        let _sol = m.solve();
    });
}

fn main() {
    println!("\n=== Sum Constraint Benchmarks (Baseline) ===");
    println!("{:50} | {:14} | {:14}", "Benchmark", "Avg Time", "Total Time");
    println!("{}", "=".repeat(80));

    sum_forward_10vars();
    sum_forward_20vars();
    sum_forward_50vars();
    sum_forward_100vars();
    sum_forward_200vars();
    sum_forward_500vars();
    sum_100vars_large_domain();
    sum_with_alldiff_10vars();
    sum_with_alldiff_30vars();
    multiple_sums_10vars();
    multiple_overlapping_sums_50vars();
    sudoku_4x4();
    sum_large_domain();
    sum_200vars_wide_domain();
    sum_tight_bounds();
    sum_50vars_ultra_tight_bounds();
    sum_deep_search_tree();

    println!("{}", "=".repeat(80));
    println!("\nNote: These are baseline measurements before incremental optimization.");
    println!("After implementing incremental sum, run again to compare performance.");
}
