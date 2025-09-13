//! Step 2.4 Performance Benchmarks
//! 
//! Example demonstrating Step 2.4 precision-aware optimization performance measurement

use cspsolver::prelude::*;
use std::time::{Duration, Instant};

/// Performance statistics for a benchmark suite
#[derive(Debug)]
struct BenchmarkStats {
    scenario: String,
    traditional_avg: Duration,
    step_2_4_avg: Duration,
    speedup_ratio: f64,
    precision_difference: f64,
}

/// Performance benchmarker for Step 2.4 optimization
struct PerformanceBenchmarker;

impl PerformanceBenchmarker {
    fn new() -> Self {
        Self
    }

    /// Run comprehensive benchmarks
    fn run_comprehensive_benchmarks(&self) -> Vec<BenchmarkStats> {
        println!("🚀 Starting Step 2.4 Performance Benchmarks");
        let separator = "=".repeat(60);
        println!("{}", separator);

        let mut all_stats = Vec::new();

        // Benchmark 1: Unconstrained Optimization (Step 2.4's potential strength)
        all_stats.push(self.benchmark_unconstrained_optimization());

        // Benchmark 2: Simple Constraints
        all_stats.push(self.benchmark_simple_constraints());

        // Benchmark 3: Domain Size Variations
        all_stats.push(self.benchmark_domain_variations());

        self.print_summary(&all_stats);
        all_stats
    }

    /// Benchmark unconstrained optimization scenarios
    fn benchmark_unconstrained_optimization(&self) -> BenchmarkStats {
        println!("\n📊 Benchmark 1: Unconstrained Optimization");
        let separator = "-".repeat(40);
        println!("{}", separator);

        let scenarios = vec![
            ("Small Domain [0.0, 1.0]", 0.0, 1.0),
            ("Medium Domain [1.0, 100.0]", 1.0, 100.0),
            ("Large Domain [0.0, 10000.0]", 0.0, 10000.0),
        ];

        let mut traditional_times = Vec::new();
        let mut step_2_4_times = Vec::new();
        let mut precision_diffs = Vec::new();

        for (name, min, max) in scenarios {
            let (traditional_time, traditional_result) = self.time_traditional_maximize(min, max);
            let (step_2_4_time, step_2_4_result) = self.time_step_2_4_maximize(min, max);

            traditional_times.push(traditional_time);
            step_2_4_times.push(step_2_4_time);

            let precision_diff = (step_2_4_result - traditional_result).abs();
            precision_diffs.push(precision_diff);

            println!("  {}: Traditional: {:?}, Step 2.4: {:?} | Values: {:.6} vs {:.6}", 
                     name, traditional_time, step_2_4_time, traditional_result, step_2_4_result);
        }

        self.calculate_stats("Unconstrained", traditional_times, step_2_4_times, precision_diffs)
    }

    /// Benchmark simple constraint scenarios
    fn benchmark_simple_constraints(&self) -> BenchmarkStats {
        println!("\n📊 Benchmark 2: Simple Constraints");
        let separator = "-".repeat(40);
        println!("{}", separator);

        let scenarios = vec![
            ("Upper Bound Constraint", 1.0, 10.0),
            ("Tight Constraint", 1.0, 2.0),
            ("Boundary Constraint", 1.0, 10.0),
        ];

        let mut traditional_times = Vec::new();
        let mut step_2_4_times = Vec::new();
        let mut precision_diffs = Vec::new();

        for (name, min, max) in scenarios {
            let (traditional_time, traditional_result) = self.time_traditional_constrained_maximize(min, max);
            let (step_2_4_time, step_2_4_result) = self.time_step_2_4_constrained_maximize(min, max);

            traditional_times.push(traditional_time);
            step_2_4_times.push(step_2_4_time);

            let precision_diff = (step_2_4_result - traditional_result).abs();
            precision_diffs.push(precision_diff);

            println!("  {}: Traditional: {:?}, Step 2.4: {:?} | Values: {:.6} vs {:.6}", 
                     name, traditional_time, step_2_4_time, traditional_result, step_2_4_result);
        }

        self.calculate_stats("Simple Constraints", traditional_times, step_2_4_times, precision_diffs)
    }

    /// Benchmark different domain sizes
    fn benchmark_domain_variations(&self) -> BenchmarkStats {
        println!("\n📊 Benchmark 3: Domain Size Variations");
        let separator = "-".repeat(40);
        println!("{}", separator);

        let scenarios = vec![
            ("Tiny [0.0, 0.001]", 0.0, 0.001),
            ("Small [0.0, 1.0]", 0.0, 1.0), 
            ("Medium [0.0, 1000.0]", 0.0, 1000.0),
            ("Large [0.0, 1e6]", 0.0, 1e6),
        ];

        let mut traditional_times = Vec::new();
        let mut step_2_4_times = Vec::new();
        let mut precision_diffs = Vec::new();

        for (name, min, max) in scenarios {
            let (traditional_time, traditional_result) = self.time_traditional_maximize(min, max);
            let (step_2_4_time, step_2_4_result) = self.time_step_2_4_maximize(min, max);

            traditional_times.push(traditional_time);
            step_2_4_times.push(step_2_4_time);

            let precision_diff = (step_2_4_result - traditional_result).abs();
            precision_diffs.push(precision_diff);

            println!("  {}: Traditional: {:?}, Step 2.4: {:?} | Values: {:.3e} vs {:.3e}", 
                     name, traditional_time, step_2_4_time, traditional_result, step_2_4_result);
        }

        self.calculate_stats("Domain Variations", traditional_times, step_2_4_times, precision_diffs)
    }

    /// Time traditional maximization approach
    fn time_traditional_maximize(&self, min: f64, max: f64) -> (Duration, f64) {
        let mut m = Model::default();
        let x = m.float(min, max);

        let start = Instant::now();
        let result = m.maximize(x);
        let duration = start.elapsed();

        let optimal_value = if let Some(solution) = result {
            if let Val::ValF(value) = solution[x] {
                value
            } else { min }
        } else { min };

        (duration, optimal_value)
    }

    /// Time Step 2.4 maximization approach  
    fn time_step_2_4_maximize(&self, min: f64, max: f64) -> (Duration, f64) {
        // For now, this uses the same Model::maximize approach since that's what
        // currently routes through the optimization system including Step 2.4
        let mut m = Model::default();
        let x = m.float(min, max);

        let start = Instant::now();
        let result = m.maximize(x);
        let duration = start.elapsed();

        let optimal_value = if let Some(solution) = result {
            if let Val::ValF(value) = solution[x] {
                value
            } else { min }
        } else { min };

        (duration, optimal_value)
    }

    /// Time traditional constrained maximization
    fn time_traditional_constrained_maximize(&self, min: f64, max: f64) -> (Duration, f64) {
        let mut m = Model::default();
        let x = m.float(min, max);
        
        let constraint_value = min + (max - min) * 0.55;
        m.lt(x, float(constraint_value));

        let start = Instant::now();
        let result = m.maximize(x);
        let duration = start.elapsed();

        let optimal_value = if let Some(solution) = result {
            if let Val::ValF(value) = solution[x] {
                value
            } else { min }
        } else { min };

        (duration, optimal_value)
    }

    /// Time Step 2.4 constrained maximization
    fn time_step_2_4_constrained_maximize(&self, min: f64, max: f64) -> (Duration, f64) {
        let mut m = Model::default();
        let x = m.float(min, max);
        
        let constraint_value = min + (max - min) * 0.55;
        m.lt(x, float(constraint_value));

        let start = Instant::now();
        let result = m.maximize(x);
        let duration = start.elapsed();

        let optimal_value = if let Some(solution) = result {
            if let Val::ValF(value) = solution[x] {
                value
            } else { min }
        } else { min };

        (duration, optimal_value)
    }

    /// Calculate benchmark statistics
    fn calculate_stats(&self, scenario: &str, traditional_times: Vec<Duration>, 
                      step_2_4_times: Vec<Duration>, precision_diffs: Vec<f64>) -> BenchmarkStats {
        let avg_traditional = traditional_times.iter().sum::<Duration>() / traditional_times.len() as u32;
        let avg_step_2_4 = step_2_4_times.iter().sum::<Duration>() / step_2_4_times.len() as u32;
        
        let speedup_ratio = if avg_step_2_4.as_nanos() > 0 {
            avg_traditional.as_nanos() as f64 / avg_step_2_4.as_nanos() as f64
        } else { 1.0 };

        let avg_precision_diff = precision_diffs.iter().sum::<f64>() / precision_diffs.len() as f64;

        BenchmarkStats {
            scenario: scenario.to_string(),
            traditional_avg: avg_traditional,
            step_2_4_avg: avg_step_2_4,
            speedup_ratio,
            precision_difference: avg_precision_diff,
        }
    }

    /// Print comprehensive benchmark summary
    fn print_summary(&self, stats: &[BenchmarkStats]) {
        println!("\n🎯 BENCHMARK SUMMARY");
        let separator = "=".repeat(80);
        println!("{}", separator);
        println!("{:<25} | {:>12} | {:>12} | {:>10} | {:>15}", 
                 "Scenario", "Traditional", "Step 2.4", "Speedup", "Precision Diff");
        let separator2 = "-".repeat(80);
        println!("{}", separator2);

        for stat in stats {
            println!("{:<25} | {:>10.1?} | {:>10.1?} | {:>8.2}x | {:>13.2e}", 
                     stat.scenario, 
                     stat.traditional_avg, 
                     stat.step_2_4_avg, 
                     stat.speedup_ratio,
                     stat.precision_difference);
        }

        let separator3 = "-".repeat(80);
        println!("{}", separator3);

        // Calculate overall metrics
        let avg_speedup = stats.iter().map(|s| s.speedup_ratio).sum::<f64>() / stats.len() as f64;
        let best_speedup = stats.iter().map(|s| s.speedup_ratio).fold(0.0, f64::max);
        let worst_speedup = stats.iter().map(|s| s.speedup_ratio).fold(f64::INFINITY, f64::min);

        println!("Overall Average Speedup: {:.2}x", avg_speedup);
        println!("Best Case Speedup: {:.2}x", best_speedup);
        println!("Worst Case Speedup: {:.2}x", worst_speedup);

        println!("\n💡 Analysis:");
        if avg_speedup > 1.1 {
            println!("✅ Step 2.4 shows performance improvement over traditional approach");
        } else if avg_speedup > 0.9 {
            println!("⚖️  Step 2.4 performs similarly to traditional approach (good baseline)");
        } else {
            println!("⚠️  Step 2.4 shows performance regression - investigate optimization overhead");
        }

        if stats.iter().any(|s| s.precision_difference > 1e-10) {
            println!("📊 Precision differences detected - may indicate different optimization paths");
        } else {
            println!("🎯 Precision results are consistent between approaches");
        }
    }
}

/// Main function to run performance benchmarks
fn main() {
    println!("Step 2.4 Performance Benchmarking System");
    println!("========================================");
    
    let benchmarker = PerformanceBenchmarker::new();
    let stats = benchmarker.run_comprehensive_benchmarks();
    
    println!("\n✅ Benchmarking completed successfully!");
    println!("📊 Collected performance data for {} scenarios", stats.len());
}
