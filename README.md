# benchmark_xz
Benchmarks for various Rust XZ crates.

* Benchmark Machine: 2014 Haswell
* Benchmark File: Sqlite database, 9.2 MB compressed, 122.8 MB uncompressed.

| Crate      | Time |
| ----------- | ----------- |
| xz2 | 876.91 ms |
| rust-lzma | 886.08 ms |
| xz-decom | 1.2409 s |
| lzma-rs | 1.7473 s |
