#!/bin/bash

# CSP Solver Misc Examples Runner
# Usage: ./run_example.sh [example_name]
# Or: ./run_example.sh list

if [ "$1" = "list" ] || [ -z "$1" ]; then
    echo "📚 Available CSP Solver Examples:"
    echo "================================"
    echo ""
    echo "🔧 Core API Demonstrations:"
    echo "  api_evolution_demo       - API evolution showcase"
    echo "  clean_api_demo          - Clean, modern API usage"
    echo "  ultra_clean_api_demo    - Most streamlined patterns"
    echo "  model_with_values       - Working with solution values"
    echo ""
    echo "🧠 Boolean Logic & Constraints:"
    echo "  bool_logic_demo         - Boolean constraint demos"
    echo "  boolean_logic           - Advanced boolean logic"
    echo "  comprehensive_constraints - Wide variety of constraints"
    echo ""
    echo "🔢 Mathematical Operations:"
    echo "  min_max_demo            - Min/max constraints"
    echo "  multiplication_demo     - Multiplication patterns"
    echo ""
    echo "🏗️  Real-World Applications:"
    echo "  pc_builder              - PC building optimization"
    echo "  portfolio_optimization  - Investment optimization"
    echo "  resource_allocation     - Resource allocation problems"
    echo ""
    echo "⚙️  Configuration & Monitoring:"
    echo "  precision_config        - Precision configuration"
    echo "  error_handling_demo     - Error handling patterns"
    echo "  simple_config_demo      - Basic configuration"
    echo "  resource_cleanup_demo   - Resource management"
    echo "  simple_memory_monitoring_demo - Memory monitoring"
    echo "  solution_with_stats_demo - Solution statistics"
    echo ""
    echo "📊 Analysis:"
    echo "  classification_demo     - Problem classification"
    echo ""
    echo "Usage: ./run_example.sh <example_name>"
    echo "Example: ./run_example.sh pc_builder"
    exit 0
fi

EXAMPLE_NAME="$1"

echo "🚀 Running CSP Solver Example: $EXAMPLE_NAME"
echo "============================================"
echo ""

# Check if example exists
if ! grep -q "name = \"$EXAMPLE_NAME\"" Cargo.toml; then
    echo "❌ Example '$EXAMPLE_NAME' not found!"
    echo ""
    echo "Run './run_example.sh list' to see available examples."
    exit 1
fi

# Run the example
cargo run --bin "$EXAMPLE_NAME"

echo ""
echo "✅ Example '$EXAMPLE_NAME' completed!"