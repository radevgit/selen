#!/bin/bash

# Script to update all min() and max() calls in constraint macros

cd /home/ross/devpublic/cspsolver

# Update min() calls in constraint_macros.rs
sed -i 's/let _min_var = \$model\.min(&\[\$(\$vars),+\]);/let _min_var = $model.min(\&[$($vars),+]).expect("min macro requires non-empty variable list");/g' src/constraint_macros.rs

# Update max() calls in constraint_macros.rs
sed -i 's/let _max_var = \$model\.max(&\[\$(\$vars),+\]);/let _max_var = $model.max(\&[$($vars),+]).expect("max macro requires non-empty variable list");/g' src/constraint_macros.rs

# Update min() calls with arrays
sed -i 's/let _min_var = \$model\.min(&\$array);/let _min_var = $model.min(\&$array).expect("min macro requires non-empty variable list");/g' src/constraint_macros.rs

# Update max() calls with arrays
sed -i 's/let _max_var = \$model\.max(&\$array);/let _max_var = $model.max(\&$array).expect("max macro requires non-empty variable list");/g' src/constraint_macros.rs

echo "Updated constraint macros"