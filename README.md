# benchmark_xz
Benchmarks for various Rust XZ crates.

* Benchmark File: Sqlite database, 9.2 MB compressed, 122.8 MB uncompressed.

* Benchmark Machine: Mid-2014 Macbook Pro 15 - Haswell

| Crate      | Time |
| ----------- | ----------- |
| xz2 | 876.91 ms |
| rust-lzma | 886.08 ms |
| xz-decom | 1.2409 s |
| lzma-rs | 1.7473 s |


* Benchmark Machine: 2021 Macbook Pro 16 - M1 Pro

| Crate      | Time |
| ----------- | ----------- |
| xz2 | 603.42 ms |
| rust-lzma | 605.06 ms |
| xz-decom | 981.99 ms |
| lzma-rs | 1.0625 s |