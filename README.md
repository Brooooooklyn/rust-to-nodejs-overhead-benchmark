# Benchmark for Node.js binding frameworks

## Hardware info

```
OS: Windows 10 x86_64
Host: Micro-Star International Co., Ltd. MS-7C35
Kernel: 10.0.19043
Terminal: Windows Terminal
CPU: AMD Ryzen 9 5950X (32) @ 3.400GHz
Memory: 32688MiB
```

## Results

|             | napi-rs                      | napi-rs-compact             | neon                        | node-bindgen               |
| ----------- | ---------------------------- | --------------------------- | --------------------------- | -------------------------- |
| Sum (a + b) | 31 440 751 ops/s  **88.42%** | 35 557 436 ops/s   **100%** | 9 548 021 ops/s  **26.85%** | 8 150 171 ops/s  **22.92** |
