
# JSON Parser Benchmarks

## Overview

This benchmark compares the performance of three JSON parsers:

- Rust JSON parser (this project)
- Python standard library `json`
- Python `simplejson`

All benchmarks were performed using release-optimized Rust code and 1,000 iterations per parser.

Lower execution times indicate better performance.

## Baseline Results (Before Optimization)

The following results were collected before optimizing the Rust tokenizer.

| JSON Size | Rust (s) | Python json (s) | simplejson (s) |
|-----------|----------|-----------------|----------------|
| Small (16 bytes) | 0.000329 | 0.000975 | 0.001181 |
| Medium (304 bytes) | 0.005581 | 0.007086 | 0.009172 |
| Large (3904 bytes) | 0.048555 | 0.062375 | 0.080724 |

These results represent one baseline benchmark run.

## Optimization

Modified `src/tokenizer.rs` to preallocate memory for the token vector.

Before:

```rust
let mut tokens = Vec::new();
```

After:

```rust
let mut tokens = Vec::with_capacity(self.input.len() / 4 + 1);
```

Preallocating vector capacity can reduce memory reallocations during tokenization.

## Results After Optimization

The optimized Rust parser was benchmarked three additional times.

The following table shows the average execution time across those three runs.

| JSON Size | Rust (s) | Python json (s) | simplejson (s) |
|-----------|----------|-----------------|----------------|
| Small (16 bytes) | 0.000271 | 0.000972 | 0.001115 |
| Medium (304 bytes) | 0.005234 | 0.006817 | 0.008372 |
| Large (3904 bytes) | 0.046730 | 0.062923 | 0.077813 |

## Rust Performance Comparison

This table compares the original Rust baseline against the average of three optimized runs.

| JSON Size | Before (s) | After (s) | Improvement |
|-----------|------------|-----------|-------------|
| Small (16 bytes) | 0.000329 | 0.000271 | 17.6% |
| Medium (304 bytes) | 0.005581 | 0.005234 | 6.2% |
| Large (3904 bytes) | 0.048555 | 0.046730 | 3.8% |

## Conclusion

The Rust parser recorded lower execution times after the tokenizer optimization for all three JSON sizes.

The largest observed improvement was 17.6% for small JSON, followed by 6.2% for medium JSON and 3.8% for large JSON.

The Rust parser was also faster than Python's standard `json` parser and `simplejson` in these benchmark runs.

These results are preliminary because the original baseline was measured only once. Repeated baseline runs would be needed to confirm that the measured improvements are attributable to the optimization rather than normal benchmark variation.

