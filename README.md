## Simple SHA256
Implemented SHA256 from scratch

### Usage

Cargo.toml:
```toml
[dependencies]
simple_sha256 = { git = "https://github.com/RajeshRk18/simple-sha256.git" }
```

main.rs
```rust
use simple_sha256::{Sha256Hasher, Hasher};

fn main() {
    let mut hasher = Sha256Hasher::new();
    hasher.update("hello world".as_bytes());
    let hash = hasher.finish();

    println!("{}", hash);
}
```

### Benchmarks

```
SHA256/My implementation with size_16
                        time:   [478.87 ns 486.56 ns 495.46 ns]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
SHA256/My implementation with size_32
                        time:   [484.51 ns 494.01 ns 504.36 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
SHA256/My implementation with size_64
                        time:   [1.0041 µs 1.1393 µs 1.3158 µs]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
SHA256/My implementation with size_128
                        time:   [1.3451 µs 1.3779 µs 1.4177 µs]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe
SHA256/My implementation with size_256
                        time:   [2.2390 µs 2.3248 µs 2.4193 µs]
Found 14 outliers among 100 measurements (14.00%)
  4 (4.00%) high mild
  10 (10.00%) high severe
SHA256/My implementation with size_512
                        time:   [3.9112 µs 4.0030 µs 4.1138 µs]
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe
SHA256/My implementation with size_1024
                        time:   [7.5580 µs 7.9344 µs 8.3743 µs]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe
SHA256/My implementation with size_2048
                        time:   [14.038 µs 14.276 µs 14.586 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
SHA256/My implementation with size_4096
                        time:   [28.196 µs 28.786 µs 29.481 µs]
Found 17 outliers among 100 measurements (17.00%)
  7 (7.00%) high mild
  10 (10.00%) high severe
SHA256/My implementation with size_9192
                        time:   [60.277 µs 61.476 µs 62.985 µs]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
SHA256/My implementation with size_65536
                        time:   [445.56 µs 458.24 µs 474.09 µs]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) high mild
  8 (8.00%) high severe
```

To run benchmark,

```shell
cargo bench --bench my_bench
```