# CSP Solver Benchmarks

This directory contains performance benchmarks for validating the precision optimization system and runtime constraint API performance.

## Structure

- `precision_validation/` - Validate ULP-based precision optimization performance claims
- `scalability/` - Test performance limits with increasing problem complexity  
- `solver_limits/` - Engineering-scale constraint optimization scenarios
- `medium_scale_proposals/` - Optimization strategies for 25+ variable problems
- `runtime_api_performance_*.rs` - Runtime Constraint API performance benchmarks

## Running Benchmarks

**⚠️ IMPORTANT**: Always use `--release` flag for accurate performance measurements!

Benchmarks are separate from tests to avoid running them during normal `cargo test`:

```bash
# Run all benchmarks (RELEASE MODE REQUIRED)
cargo run --release --bin benchmark_suite

# Runtime API Performance Benchmarks
cargo run --release benchmarks/runtime_api_performance_simple.rs
cargo run --release benchmarks/runtime_api_performance_benchmarks.rs
cargo run --release benchmarks/runtime_api_performance_best_practices.rs

# Individual benchmark examples
cargo run --release --example step_2_4_performance_benchmarks
cargo run --release --example sudoku  # For Platinum puzzle timing

# Debug mode comparison (for development only - NOT for benchmarks)
cargo run --example sudoku  # ~7-10x slower than release mode
```

**Performance Impact of Build Modes:**
- **Release mode**: Full optimizations, accurate benchmark results
- **Debug mode**: No optimizations, 5-10x slower, bounds checking enabled

## Performance Goals

### Runtime API Performance (Target: <2x overhead vs post! macro)
- Simple constraints: <5x overhead vs post! macro (acceptable for flexibility)
- Complex expressions: <2x overhead vs post! macro (competitive for dynamic scenarios)
- Batch operations: Near-optimal performance with `postall()`
- Global constraints: Minimal overhead (<1.2x vs optimized implementations)

### Precision Optimization (Target: < 1ms)
- Single constraint problems: < 10 microseconds
- Multi-constraint problems: < 100 microseconds  
- Engineering precision scenarios: < 1 millisecond

### Scalability Limits
- Identify when precision optimization fails and falls back to CSP search
- Measure performance degradation with problem complexity
- Establish practical limits for engineering constraint problems

### Engineering Scale Testing
- Test constraint optimization with engineering-scale numerical values (cm to meters)
- Validate performance across different problem sizes and scales
- Quantify the performance advantage of ULP-based optimization

## Runtime API Benchmarks

### `runtime_api_performance_simple.rs`
Quick performance comparison between runtime API and post! macro for basic operations.

### `runtime_api_performance_benchmarks.rs`
Comprehensive benchmark suite covering:
- Basic constraint creation (runtime API vs post! macro)
- Global constraint performance 
- Constraint composition and boolean logic
- Solving performance comparison
- Scaling characteristics with problem size

### `runtime_api_performance_best_practices.rs`
Demonstrates optimal usage patterns for runtime API performance including:
- When to use runtime API vs post! macro
- Batch posting techniques
- Global constraint usage
- Memory optimization strategies
