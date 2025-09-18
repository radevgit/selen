# CSP Solver - Miscellaneous Examples

This directory contains various examples demonstrating different features and use cases of the CSP solver library. Each example is available as a standalone binary that can be run independently.

## 🚀 Quick Start

### Prerequisites
```bash
# Navigate to the misc examples directory
cd examples/misc

# Build all examples (optional - they'll build automatically when run)
cargo build
```

### Method 1: Using the Helper Script (Recommended)
```bash
# List all available examples with descriptions
./run_example.sh list

# Run any specific example
./run_example.sh <example_name>

# Examples:
./run_example.sh pc_builder
./run_example.sh clean_api_demo
./run_example.sh portfolio_optimization
```

### Method 2: Using Cargo Directly
```bash
# Run any example directly
cargo run --bin <example_name>

# Examples:
cargo run --bin pc_builder
cargo run --bin min_max_demo
cargo run --bin boolean_logic
```

## 📚 Available Examples

### 🔧 Core API Demonstrations
```bash
cargo run --bin api_evolution_demo       # API evolution showcase
cargo run --bin clean_api_demo           # Clean, modern API usage  
cargo run --bin ultra_clean_api_demo     # Most streamlined patterns
cargo run --bin model_with_values        # Working with solution values
```

### 🧠 Boolean Logic & Constraints
```bash
cargo run --bin bool_logic_demo          # Boolean constraint demos
cargo run --bin boolean_logic            # Advanced boolean logic
cargo run --bin comprehensive_constraints # Wide variety of constraints
```

### 🔢 Mathematical Operations
```bash
cargo run --bin min_max_demo             # Min/max constraints
cargo run --bin multiplication_demo      # Multiplication patterns
```

### 🏗️ Real-World Applications
```bash
cargo run --bin pc_builder               # PC building optimization
cargo run --bin portfolio_optimization   # Investment optimization
cargo run --bin resource_allocation      # Resource allocation problems
```

### ⚙️ Configuration & Monitoring
```bash
cargo run --bin precision_config         # Precision configuration
cargo run --bin error_handling_demo      # Error handling patterns
cargo run --bin simple_config_demo       # Basic configuration
cargo run --bin resource_cleanup_demo    # Resource management
cargo run --bin simple_memory_monitoring_demo # Memory monitoring
cargo run --bin solution_with_stats_demo # Solution statistics
```

### 📊 Analysis & Classification
```bash
cargo run --bin classification_demo      # Problem classification
```

## 🛠️ Development Commands

### Building
```bash
# Build all examples
cargo build

# Build a specific example
cargo build --bin pc_builder

# Build with optimizations
cargo build --release
```

### Running with Options
```bash
# Run with release optimizations
cargo run --release --bin pc_builder

# Run with verbose output
cargo run --bin clean_api_demo -- --verbose

# Set environment variables
RUST_LOG=debug cargo run --bin error_handling_demo
```

### Testing and Verification
```bash
# Check that all examples compile
cargo check

# Run clippy on examples
cargo clippy

# Format code
cargo fmt

# Validate all examples work correctly
./validate_examples.sh
```

## 🎯 Batch Operations

### Run All Examples
```bash
# Using the helper script
for example in api_evolution_demo bool_logic_demo boolean_logic classification_demo clean_api_demo comprehensive_constraints error_handling_demo min_max_demo model_with_values multiplication_demo pc_builder portfolio_optimization precision_config resource_allocation resource_cleanup_demo simple_config_demo simple_memory_monitoring_demo solution_with_stats_demo ultra_clean_api_demo; do
    echo "🚀 Running $example..."
    ./run_example.sh $example
    echo "✅ Completed $example"
    echo "---"
done
```

### Run Examples by Category
```bash
# Core API examples
for example in api_evolution_demo clean_api_demo ultra_clean_api_demo model_with_values; do
    cargo run --bin $example
done

# Boolean logic examples  
for example in bool_logic_demo boolean_logic comprehensive_constraints; do
    cargo run --bin $example
done

# Real-world applications
for example in pc_builder portfolio_optimization resource_allocation; do
    cargo run --bin $example
done
```

### Performance Testing
```bash
# Time all examples
for example in api_evolution_demo bool_logic_demo boolean_logic classification_demo clean_api_demo comprehensive_constraints error_handling_demo min_max_demo model_with_values multiplication_demo pc_builder portfolio_optimization precision_config resource_allocation resource_cleanup_demo simple_config_demo simple_memory_monitoring_demo solution_with_stats_demo ultra_clean_api_demo; do
    echo "⏱️  Timing $example..."
    time cargo run --release --bin $example > /dev/null
done
```

## 📖 Example Descriptions

| Example | Description | Key Features |
|---------|-------------|--------------|
| `api_evolution_demo` | Shows API evolution across versions | Version comparison, migration patterns |
| `bool_logic_demo` | Boolean constraint demonstrations | AND, OR, NOT operations |
| `boolean_logic` | Advanced boolean logic examples | Complex boolean expressions |
| `classification_demo` | Problem classification examples | Algorithm categorization |
| `clean_api_demo` | Clean, modern API usage | Best practices, clean syntax |
| `comprehensive_constraints` | Wide variety of constraint types | Multiple constraint patterns |
| `error_handling_demo` | Proper error handling patterns | Error recovery, validation |
| `min_max_demo` | Min/max constraint examples | Optimization bounds |
| `model_with_values` | Working with solution values | Value extraction, analysis |
| `multiplication_demo` | Multiplication constraint patterns | Arithmetic operations |
| `pc_builder` | PC building optimization | Real-world optimization |
| `portfolio_optimization` | Investment optimization | Financial modeling |
| `precision_config` | Precision configuration | Numerical precision control |
| `resource_allocation` | Resource allocation problems | Resource distribution |
| `resource_cleanup_demo` | Resource management | Memory/resource cleanup |
| `simple_config_demo` | Basic configuration examples | Configuration patterns |
| `simple_memory_monitoring_demo` | Memory usage monitoring | Performance monitoring |
| `solution_with_stats_demo` | Solution statistics and analysis | Result analysis |
| `ultra_clean_api_demo` | Most streamlined API patterns | Minimal syntax examples |

## 🔍 Troubleshooting

### Common Issues
```bash
# If cargo complains about workspace conflicts
cargo build --manifest-path Cargo.toml

# If binary not found
cargo build && cargo run --bin <example_name>

# Clean and rebuild
cargo clean && cargo build
```

### Getting Help
```bash
# List all available binaries
ls target/debug/ | grep -v '\.'

# Show example help (if supported)
cargo run --bin pc_builder -- --help

# Check dependencies
cargo tree
```

## 🤝 Contributing

To add a new example:

1. Create your `.rs` file in this directory
2. Add a `[[bin]]` section to `Cargo.toml`:
   ```toml
   [[bin]]
   name = "your_example"
   path = "your_example.rs"
   ```
3. Update this README with your example description
4. Test with `cargo run --bin your_example`