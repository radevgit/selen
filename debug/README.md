# Debug Directory

This directory contains all temporary debugging files, experimental tests, and development artifacts.

## Organization

- **constraint_tests/**: Debug tests for constraint behavior and edge cases
- **propagation_tests/**: Debug tests for constraint propagation mechanisms  
- **view_tests/**: Debug tests for view system (Next, Prev, type conversions)

## Files

### Current Issue Investigation
- `test_greater_than_bug.rs`: Main test reproducing the greater_than constraint bug with singleton domains

### Debug Tests by Category
- `debug_constraint*.rs`: Various constraint debugging utilities
- `debug_propagation*.rs`: Propagation engine debugging
- `debug_next.rs`, `debug_ulp.rs`: ULP and Next/Prev view debugging
- `test_*.rs`: Standalone test programs for specific issues

## Usage

These files are for development and debugging only. They are not part of the main test suite and may contain experimental or incomplete code.

To run a specific debug test:
```bash
cargo test --test debug_file_name
```

Note: Some files may not compile or may be incomplete - they are preserved for investigation purposes.
