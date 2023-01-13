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

|                             | napi-rs                    | napi-rs-compat                | neon                         |
| --------------------------- | -------------------------- | ----------------------------- | ---------------------------- |
| Sum (a + b)                 | 30 145 025 ops/s  **100%** | 22 017 378 ops/s   **73.03%** | 18 583 192 ops/s  **61.65%** |
| Concat(Hello + " world")    | 5 932 133 ops/s **100%**   | 4 930 524 ops/s  **83.12%**   | 5 253 685 ops/s  **88.56%**  |
| Rect Area { width, height } | 2 898 963 ops/s **96.62%** | 2 957 163 ops/s **100%**      | 2 653 181 ops/s **89.72%**   |

