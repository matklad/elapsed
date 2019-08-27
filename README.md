# Time a block of Rust code

[![Build Status](https://travis-ci.org/matklad/elapsed.svg?branch=master)](https://travis-ci.org/matklad/elapsed)
[![Docs](https://docs.rs/elapsed/badge.svg)](https://docs.rs/elapsed)
[![crates.io](https://img.shields.io/crates/v/elapsed.svg)](https://crates.io/crates/elapsed)

**Deprecated**

Debug representation for `std::time::Duration` is human-readable now,
so this snipper works like a charm and doesn't require using external libraries:

```rust
let start = std::time::Instant();
let sum = (0..10_000).sum::<u64>();
eprintln!("elapsed {:?}", start.elapsed()); // note :?
println!("sum = {}", sum);
```

```TOML
[dependencies]
elapsed = "0.1"
```

```Rust
extern crate elapsed;
use elapsed::measure_time;

fn main() {
    let (elapsed, sum) = measure_time(|| {
        (0..10_000).sum::<u64>()
    });
    println!("elapsed = {}", elapsed);
    println!("sum = {}", sum);

    // Prints
    // elapsed = 227.81 μs
    // sum = 49995000
}
```

Inspired by [`measureTimeMillis`](https://kotlinlang.org/api/latest/jvm/stdlib/kotlin.system/measure-time-millis.html) in
Kotlin.
