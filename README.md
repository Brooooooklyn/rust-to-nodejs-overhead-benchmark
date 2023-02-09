# Benchmark for Node.js binding frameworks

## Hardware info

```
OS: macOS 13.2 22D49 arm64
Host: MacBookPro18,2
Kernel: 22.3.0
Terminal: WarpTerminal
CPU: Apple M1 Max
Memory: 65536MiB
```

## Results

|                             | napi-rs                    | napi-rs-compat                | neon                         | node-bindgen |
| --------------------------- | -------------------------- | ----------------------------- | ---------------------------- | --------------------------- |
| Sum (a + b)                 | 37 987 529 ops/s, ±0.07%  **100%** | 33 650 128 ops/s, ±0.26%   **73.03%** | 23 977 894 ops/s, ±0.09%  **61.65%** | 19 623 929 ops/s, ±0.11% **51.66%** |
| Concat(Hello + " world")    | 9 962 492 ops/s, ±0.29% **100%** | 8 406 430 ops/s, ±0.25%  **84.38%** | 9 615 266 ops/s, ±0.38%  **96.51%** | 6 283 797 ops/s, ±0.21% **63.07%** |
| Rect Area { width, height } | 4 333 189 ops/s, ±0.26% **98.18%** | 4 413 512 ops/s, ±0.23% **100%** | 3 614 248 ops/s, ±0.25% **81.89%** | 2 458 340 ops/s, ±0.27% **55.70%** |

