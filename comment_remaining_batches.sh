#!/bin/bash
# Fast script to comment out passing tests in batches 03-10

echo "🚀 Commenting out passing tests in batches 03-10..."
echo ""

for batch in {03..10}; do
    echo "📝 Processing batch $batch..."
    
    test_file="tests/test_flatzinc_batch_${batch}.rs"
    
    # Create backup
    cp "$test_file" "${test_file}.backup_$(date +%s)"
    
    # Run test with timeout and capture passing tests
    passing=$(timeout 30 cargo test --quiet --release --test test_flatzinc_batch_$batch -- --nocapture 2>&1 | grep "✓" | awk '{print $2}' || true)
    
    if [ -z "$passing" ]; then
        echo "  ⚠️  No passing tests found or timeout"
        continue
    fi
    
    # Comment out each passing test
    count=0
    while IFS= read -r test_name; do
        if [ -n "$test_name" ]; then
            # Escape special regex characters
            escaped=$(echo "$test_name" | sed 's/[.[\*^$()+?{|]/\\&/g')
            # Comment out the line
            sed -i "s|^\(.*\"$escaped\".*\)$|// \1  // ✓ PASSING|" "$test_file"
            ((count++))
        fi
    done <<< "$passing"
    
    echo "  ✅ Commented out $count passing tests"
done

echo ""
echo "✅ Done! All passing tests in batches 03-10 are now commented out."
echo ""
echo "To restore backups if needed:"
echo "  ls tests/*.backup_* | sort -r | head -8 | xargs -I {} sh -c 'mv {} \$(echo {} | sed \"s/\.backup_[0-9]*//\")'"
