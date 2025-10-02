#!/bin/bash
# Quick script to show only failing tests from each batch
# This version runs tests but much faster since we only need failure info

echo "🔍 Scanning for failures across all batches..."
echo ""

total_failures=0

for batch in {01..10}; do
    echo "Batch $batch:"
    
    # Run test and capture only failures
    failures=$(cargo test --quiet --release --test test_flatzinc_batch_$batch -- --nocapture 2>&1 | grep "✗")
    
    if [ -z "$failures" ]; then
        echo "  ✅ All tests passing!"
    else
        echo "$failures" | sed 's/^/  /'
        count=$(echo "$failures" | wc -l)
        total_failures=$((total_failures + count))
    fi
    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Total failures across all batches: $total_failures"
echo ""
