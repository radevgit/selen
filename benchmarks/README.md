# CSP Solver Benchmarks

This directory contains performance benchmarks for validating the precision optimization system.

## Structure

- `precision_validation/` - Validate ULP-based precision optimization performance claims
- `scalability/` - Test performance limits with increasing problem complexity  
- `solver_limits/` - Engineering-scale constraint optimization scenarios
- `medium_scale_proposals/` - Optimization strategies for 25+ variable problems

## Running Benchmarks

Benchmarks are separate from tests to avoid running them during normal `cargo test`:

```bash
# Run all benchmarks
cargo run --release --bin benchmark_suite
```

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
