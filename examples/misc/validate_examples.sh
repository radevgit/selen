#!/bin/bash

# CSP Solver Examples Validation Script
# This script tests that all examples mentioned in the README work correctly

echo "🧪 CSP Solver Examples Validation"
echo "=================================="
echo ""

EXAMPLES=(
    "api_evolution_demo"
    "bool_logic_demo" 
    "boolean_logic"
    "classification_demo"
    "clean_api_demo"
    "comprehensive_constraints"
    "error_handling_demo"
    "min_max_demo"
    "model_with_values"
    "multiplication_demo"
    "pc_builder"
    "portfolio_optimization"
    "precision_config"
    "resource_allocation"
    "resource_cleanup_demo"
    "simple_config_demo"
    "simple_memory_monitoring_demo"
    "solution_with_stats_demo"
    "ultra_clean_api_demo"
)

PASSED=0
FAILED=0
FAILED_EXAMPLES=()

echo "Testing ${#EXAMPLES[@]} examples..."
echo ""

for example in "${EXAMPLES[@]}"; do
    echo -n "🔍 Testing $example... "
    
    # Run the example with a timeout and capture output
    if timeout 30s cargo run --bin "$example" > /dev/null 2>&1; then
        echo "✅ PASS"
        ((PASSED++))
    else
        echo "❌ FAIL"
        ((FAILED++))
        FAILED_EXAMPLES+=("$example")
    fi
done

echo ""
echo "📊 Results Summary:"
echo "=================="
echo "✅ Passed: $PASSED"
echo "❌ Failed: $FAILED"
echo "📈 Success Rate: $(( (PASSED * 100) / (PASSED + FAILED) ))%"

if [ $FAILED -gt 0 ]; then
    echo ""
    echo "❌ Failed Examples:"
    for failed in "${FAILED_EXAMPLES[@]}"; do
        echo "  - $failed"
    done
    echo ""
    echo "💡 Run individual examples with: ./run_example.sh <example_name>"
    echo "   Or debug with: cargo run --bin <example_name>"
fi

echo ""
if [ $FAILED -eq 0 ]; then
    echo "🎉 All examples are working correctly!"
    exit 0
else
    echo "⚠️  Some examples failed. Check the failed examples above."
    exit 1
fi