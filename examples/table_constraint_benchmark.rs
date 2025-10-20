//! Table Constraint Benchmark
//!
//! Benchmarks the Table constraint with various problem sizes and table densities.
//! This benchmark is used to measure performance improvements from GAC enhancements.
//!
//! Run with: cargo run --release --example table_constraint_benchmark

use selen::prelude::*;
use std::time::Instant;

/// Structure to hold benchmark results
#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    iterations: usize,
    total_ms: f64,
    avg_ms: f64,
    min_ms: f64,
    max_ms: f64,
}

impl BenchmarkResult {
    fn print(&self) {
        println!(
            "{:<35} | {:>10.3} ms",
            self.name, self.avg_ms
        );
    }
}

/// Run a benchmark and collect timing statistics
fn benchmark<F>(name: &str, iterations: usize, mut f: F) -> BenchmarkResult
where
    F: FnMut() -> (),
{
    let mut times = Vec::new();

    // Warmup
    for _ in 0..2 {
        f();
    }

    // Actual benchmark
    for _ in 0..iterations {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        times.push(elapsed);
    }

    let total_ms: f64 = times.iter().sum();
    let avg_ms = total_ms / iterations as f64;
    let min_ms = times.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_ms = times.iter().cloned().fold(0.0, f64::max);

    BenchmarkResult {
        name: name.to_string(),
        iterations,
        total_ms,
        avg_ms,
        min_ms,
        max_ms,
    }
}

/// Benchmark 1: Simple 2-variable configuration problem (MEGA)
/// Represents compatibility matrices common in configuration/scheduling
fn table_small_2vars() {
    let mut m = Model::default();

    // Two variables with domain 1-200 (massive)
    let x = m.int(1, 200);
    let y = m.int(1, 200);

    // Table: only allow x <= y (~20k tuples)
    let mut tuples = Vec::new();
    for i in 1..=200 {
        for j in i..=200 {
            tuples.push(vec![Val::int(i), Val::int(j)]);
        }
    }

    m.table(&[x, y], tuples);
    let _sol = m.solve();
}

/// Benchmark 2: Medium 3-variable table (MEGA)
/// Representative of small configuration problems
fn table_medium_3vars() {
    let mut m = Model::default();

    let x = m.int(1, 50);
    let y = m.int(1, 50);
    let z = m.int(1, 50);

    // Table: tuples where x + y = z (~2500 tuples)
    let mut tuples = Vec::new();
    for i in 1..=50 {
        for j in 1..=50 {
            let sum = i + j;
            if sum <= 100 {
                tuples.push(vec![Val::int(i), Val::int(j), Val::int(sum)]);
            }
        }
    }

    m.table(&[x, y, z], tuples);
    let _sol = m.solve();
}

/// Benchmark 3: Large table - many tuples, small arity (MEGA)
/// Tests scalability with respect to table size
fn table_large_tuples_2vars() {
    let mut m = Model::default();

    let x = m.int(1, 100);
    let y = m.int(1, 100);

    // Large table: ~3300 tuples (33% density)
    let mut tuples = Vec::new();
    for i in 1..=100 {
        for j in 1..=100 {
            if (i * j) % 3 == 0 {
                tuples.push(vec![Val::int(i), Val::int(j)]);
            }
        }
    }

    m.table(&[x, y], tuples);
    let _sol = m.solve();
}

/// Benchmark 4: High arity table - many variables, sparse table (MEGA)
/// Tests scalability with respect to arity
fn table_high_arity_5vars() {
    let mut m = Model::default();

    let vars = vec![m.int(1, 15), m.int(1, 15), m.int(1, 15), m.int(1, 15), m.int(1, 15)];

    // Sparse table: ~500 tuples out of 759375 possible
    let mut tuples = Vec::new();
    for i in 1..=15 {
        for j in (1..=15).step_by(2) {
            for k in 1..=3 {
                tuples.push(vec![
                    Val::int(i),
                    Val::int(j),
                    Val::int((i + j) % 15),
                    Val::int(i),
                    Val::int((j * k) % 15),
                ]);
                if tuples.len() >= 500 {
                    break;
                }
            }
            if tuples.len() >= 500 {
                break;
            }
        }
        if tuples.len() >= 500 {
            break;
        }
    }

    m.table(&vars, tuples);
    let _sol = m.solve();
}

/// Benchmark 5: Dense table - high tuple density, many variables (MEGA)
/// Represents problems with few constraints
fn table_dense_3vars() {
    let mut m = Model::default();

    let x = m.int(1, 50);
    let y = m.int(1, 50);
    let z = m.int(1, 50);

    // Very dense table: include all tuples where x + y + z is even (~62k tuples)
    let mut tuples = Vec::new();
    for i in 1..=50 {
        for j in 1..=50 {
            for k in 1..=50 {
                if (i + j + k) % 2 == 0 {
                    tuples.push(vec![Val::int(i), Val::int(j), Val::int(k)]);
                }
            }
        }
    }

    m.table(&[x, y, z], tuples);
    let _sol = m.solve();
}

/// Benchmark 6: Pigeon hole variant using table (MEGA)
/// Tests with high search complexity
fn table_pigeon_hole() {
    let mut m = Model::default();

    // 8 pigeons, 5 holes
    let vars = (0..8).map(|_| m.int(0, 4)).collect::<Vec<_>>();

    // Table: at least 3 pigeons must be in hole 0
    let mut tuples = Vec::new();
    fn generate_tuples(vars: usize, holes: usize, min_in_hole_0: usize, tuples: &mut Vec<Vec<Val>>) {
        fn recurse(
            var_idx: usize,
            vars: usize,
            holes: usize,
            current: &mut Vec<i32>,
            count_in_0: usize,
            min_in_hole_0: usize,
            tuples: &mut Vec<Vec<Val>>,
        ) {
            if var_idx == vars {
                if count_in_0 >= min_in_hole_0 {
                    tuples.push(current.iter().map(|&x| Val::int(x)).collect());
                }
                return;
            }
            for h in 0..holes {
                current.push(h as i32);
                recurse(
                    var_idx + 1,
                    vars,
                    holes,
                    current,
                    if h == 0 { count_in_0 + 1 } else { count_in_0 },
                    min_in_hole_0,
                    tuples,
                );
                current.pop();
            }
        }
        let mut current = Vec::new();
        recurse(0, vars, holes, &mut current, 0, min_in_hole_0, tuples);
    }

    generate_tuples(8, 5, 3, &mut tuples);

    m.table(&vars, tuples);
    let _sol = m.solve();
}

/// Benchmark 7: Configuration problem - actual use case (MEGA)
/// Represents real-world configuration/compatibility checking
fn table_configuration() {
    let mut m = Model::default();

    // Configuration with 4 features, large domains
    let cpu = m.int(1, 8);
    let ram = m.int(1, 10);
    let storage = m.int(1, 8);
    let network = m.int(1, 6);

    let mut tuples = Vec::new();
    // Generate realistic combinations
    for c in 1..=8 {
        for r in 1..=10 {
            for s in 1..=8 {
                for n in 1..=6 {
                    // Only allow certain combinations
                    let cpu_compatible = c >= 2 || r <= 4;
                    let storage_compatible = s >= 3 || r <= 6;
                    let network_compatible = n <= 4 || c >= 5;
                    
                    if cpu_compatible && storage_compatible && network_compatible {
                        tuples.push(vec![Val::int(c), Val::int(r), Val::int(s), Val::int(n)]);
                    }
                }
            }
        }
    }

    m.table(&[cpu, ram, storage, network], tuples);
    let _sol = m.solve();
}

/// Benchmark 8: Sudoku-like table constraint (XL)
/// Tests table constraint on a problem typically solved with alldiff
fn table_sudoku_row() {
    let mut m = Model::default();

    // 12 variables, each 1-12, all different (larger than sudoku)
    let vars: Vec<_> = (0..12).map(|_| m.int(1, 12)).collect();

    // Generate many permutations
    let mut tuples = Vec::new();
    let mut perm = (1..=12).collect::<Vec<i32>>();
    // Add 500 permutations
    for _ in 0..500 {
        tuples.push(perm.iter().map(|&x| Val::int(x)).collect());
        // Simple permutation: rotate
        perm.rotate_left(1);
    }

    m.table(&vars, tuples);
    let _sol = m.solve();
}



fn main() {
    let mut results = Vec::new();
    println!("TABLE CONSTRAINT BENCHMARK (GAC - Generalized Arc Consistency)");
    println!("name                 |   ms/iter |  total ms");
    println!("─────────────────────┼───────────┼──────────");
    
    results.push(benchmark("2vars_xl", 5, || table_small_2vars()));
    results.last().unwrap().print();

    results.push(benchmark("3vars_xl", 4, || table_medium_3vars()));
    results.last().unwrap().print();

    results.push(benchmark("large_tup", 3, || table_large_tuples_2vars()));
    results.last().unwrap().print();

    results.push(benchmark("high_arity", 3, || table_high_arity_5vars()));
    results.last().unwrap().print();

    results.push(benchmark("dense_xl", 2, || table_dense_3vars()));
    results.last().unwrap().print();

    results.push(benchmark("pigeon_6v", 3, || table_pigeon_hole()));
    results.last().unwrap().print();

    results.push(benchmark("config_xl", 3, || table_configuration()));
    results.last().unwrap().print();

    results.push(benchmark("sudoku_12", 2, || table_sudoku_row()));
    results.last().unwrap().print();

    let total_ms: f64 = results.iter().map(|r| r.total_ms).sum();
    let avg_ms_per_iter = results.iter().map(|r| r.avg_ms).sum::<f64>() / results.len() as f64;
    println!("─────────────────────┼───────────┼──────────");
    println!("Total: {:.1}ms | Avg: {:.2}ms", total_ms, avg_ms_per_iter);
}
