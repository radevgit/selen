```
    https://github.com/taiki-e/cargo-llvm-cov
    cargo llvm-cov --html
    cargo llvm-cov --lcov --output-path lcov.info
    open ./target/llvm-cov/html/index.html

Only directory: cargo llvm-cov --html -- --include 'src/flatzinc/*' - does not work.
cargo llvm-cov --html --ignore-filename-regex '^(?!src/flatzinc/)'
cargo llvm-cov --html --ignore-filename-regex '(src/(?!flatzinc)|tests|examples|benchmarks|debug)'
```
