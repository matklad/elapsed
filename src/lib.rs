extern crate floating_duration;

use std::time::{Duration, Instant};

pub use floating_duration::TimeFormat;

/// Measures the time needed to execute a block of code.
///
/// # Examples
///
/// ```
/// extern crate elapsed;
/// use elapsed::measure_time;
///
/// fn main() {
///     let (elapsed, sum) = measure_time(|| {
///         (0..10_000).sum::<u64>()
///     });
///     println!("elapsed = {}", elapsed);
///     println!("sum = {}", sum);
///
///     // Prints
///     // elapsed = 227.812μs
///     // sum = 49995000
/// }
/// ```
pub fn measure_time<T, F: FnOnce() -> T>(f: F) -> (TimeFormat<Duration>, T) {
    let start = Instant::now();
    let r = f();
    (TimeFormat(start.elapsed()), r)
}
