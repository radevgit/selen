#!/bin/bash

cd /home/ross/devpublic/cspsolver

echo "Fixing min/max calls in examples and tests..."

# Fix examples/min_max_clean_demo.rs
echo "Fixing min_max_clean_demo.rs..."
sed -i 's/let minimum = model\.min(&\[/let minimum = model.min(\&[/g' examples/min_max_clean_demo.rs
sed -i 's/let maximum = model\.max(&\[/let maximum = model.max(\&[/g' examples/min_max_clean_demo.rs
sed -i 's/]);$/]).expect("non-empty variable list");/g' examples/min_max_clean_demo.rs

# Fix constraint_macros_programmatic_tests.rs
echo "Fixing constraint_macros_programmatic_tests.rs..."
sed -i 's/let min_result = m\.min(&vars_vec);/let min_result = m.min(\&vars_vec).expect("non-empty variable list");/g' src/constraint_macros_programmatic_tests.rs
sed -i 's/let min_yz = m\.min(&\[y, z\]);/let min_yz = m.min(\&[y, z]).expect("non-empty variable list");/g' src/constraint_macros_programmatic_tests.rs
sed -i 's/let min_vars = m\.min(&vars);/let min_vars = m.min(\&vars).expect("non-empty variable list");/g' src/constraint_macros_programmatic_tests.rs

echo "Fixed min/max calls"