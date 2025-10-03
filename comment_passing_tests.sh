#!/bin/bash

# Run each batch and capture which tests pass/fail
for batch in {01..10}; do
    echo "Processing batch $batch..."
    
    # Run the test and capture output
    test_output=$(cargo test --release --test test_flatzinc_batch_$batch -- --nocapture 2>&1)
    
    # Extract passing test files (lines with ✓)
    passing_tests=$(echo "$test_output" | grep "✓" | sed 's/.*✓ //' | sed 's/\.fzn.*/\.fzn/')
    
    # Get the test file path
    test_file="tests/test_flatzinc_batch_${batch}.rs"
    
    if [ ! -f "$test_file" ]; then
        echo "Warning: $test_file not found"
        continue
    fi
    
    # Create backup
    cp "$test_file" "${test_file}.backup"
    
    # Comment out each passing test
    while IFS= read -r passing_test; do
        if [ -n "$passing_test" ]; then
            # Escape special characters for sed
            escaped_test=$(echo "$passing_test" | sed 's/[.[\*^$()+?{|]/\\&/g')
            
            # Comment out the line with this test file
            sed -i "s|^\(.*\"$escaped_test\".*\)|// \1  // ✓ PASSING|" "$test_file"
        fi
    done <<< "$passing_tests"
    
    echo "  ✓ Commented out passing tests in $test_file"
done

echo ""
echo "Done! All passing tests have been commented out."
echo "To restore, run: for f in tests/test_flatzinc_batch_*.rs.backup; do mv \"\$f\" \"\${f%.backup}\"; done"
