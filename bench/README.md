How to run benchmarks:

```sh
# Run all benchmark suites
cargo bench

# Run specific benchmark suite
cargo bench parse

# Compare two branches
git checkout -b master
cargo bench -- -s main
git checkout -b your-branch
cargo bench -- -b main
```
