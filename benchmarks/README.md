# CSP Solver Benchmarks

This directory contains performance benchmarks for validating the precision optimization system.

## Structure

- `precision_validation/` - Validate ULP-based precision optimization performance claims
- `scalability/` - Test performance limits with increasing problem complexity  
- `solver_limits/` - Engineering-scale constraint optimization scenarios
- `medium_scale_proposals/` - Optimization strategies for 25+ variable problems

## Running Benchmarks

**⚠️ IMPORTANT**: Always use `--release` flag for accurate performance measurements!

Benchmarks are separate from tests to avoid running them during normal `cargo test`:

```bash
# Run all benchmarks (RELEASE MODE REQUIRED)
cargo run --release --bin benchmark_suite

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
