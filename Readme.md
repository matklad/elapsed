# Time a block of code

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
