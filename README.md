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

|                          | napi-rs                    | napi-rs-compat                | neon                        | node-bindgen                |
| ------------------------ | -------------------------- | ----------------------------- | --------------------------- | --------------------------- |
| Sum (a + b)              | 36 121 906 ops/s  **100%** | 36 021 114 ops/s   **99.72%** | 9 438 807 ops/s  **26.13%** | 7 522 062 ops/s  **20.82%** |
| Concat(Hello + " world") | 4 526 668 ops/s **100%**   | 4 336 416 ops/s  **95.95%**   | 3 973 988 ops/s  **85.09%** | 2 747 707 ops/s  **60.93%** |

