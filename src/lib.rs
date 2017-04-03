use std::time::{Duration, Instant};
use std::fmt;
use std::io::Cursor;
use std::io::Write;

/// A wrapper around `std::time::Duration` providing
/// nicer `Display` implementation and convenience
/// methods to get elapsed time with different granularity
#[derive(Clone, Copy, Debug)]
pub struct ElapsedDuration(Duration);

impl ElapsedDuration {
    /// Wrap an existing `std::time::Duration`. Useful
    /// for converting `Duration` to string.
    ///
    /// # Examples
    ///
    /// ```
    /// use elapsed::ElapsedDuration;
    ///
    /// let duration = ::std::time::Duration::from_millis(1922);
    /// let s = format!("{}", ElapsedDuration::new(duration));
    /// assert_eq!(s, "1.92 s")
    /// ```
    ///
    pub fn new(duration: Duration) -> ElapsedDuration {
        ElapsedDuration(duration)
    }

    /// The underlying `std::time::Duration`.
    pub fn duration(&self) -> Duration {
        self.0
    }

    /// Number of whole seconds elapsed.
    pub fn seconds(&self) -> u64 {
        self.0.as_secs()
    }

    /// Number of whole milliseconds elapsed.
    pub fn millis(&self) -> u64 {
        self.seconds() * 1000 + self.subsec_nanos() / 1_000_000
    }

    /// Number of whole microseconds elapsed.
    pub fn micros(&self) -> u64 {
        self.seconds() * 1_000_000 + self.subsec_nanos() / 1_000
    }

    /// Number of whole nanoseconds elapsed.
    pub fn nanos(&self) -> u64 {
        self.seconds() * 1_000_000_000 + self.subsec_nanos()
    }

    fn subsec_nanos(&self) -> u64 {
        self.0.subsec_nanos() as u64
    }
}

/// Measures the time needed to execute a block of code.
///
///
/// # Examples
///
/// ```
///    extern crate elapsed;
///    use elapsed::measure_time;
///
///    fn main() {
///        let (elapsed, sum) = measure_time(|| {
///            (0..10_000).sum::<u64>()
///        });
///        println!("elapsed = {}", elapsed);
///        println!("sum = {}", sum);
///
///        // Prints
///        // elapsed = 227.81 μs
///        // sum = 49995000
///    }
/// ```
pub fn measure_time<T, F: FnOnce() -> T>(f: F) -> (ElapsedDuration, T) {
    let start = Instant::now();
    let r = f();
    let elapsed = Instant::now() - start;
    (ElapsedDuration(elapsed), r)
}

impl fmt::Display for ElapsedDuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (d1, d2, t) = if self.seconds() > 0 {
            (self.seconds(), self.millis(), "s")
        } else if self.millis() > 0 {
            (self.millis(), self.micros(), "ms")
        } else if self.micros() > 0 {
            (self.micros(), self.nanos(), "μs")
        } else {
            (self.nanos(), self.nanos() * 1000, "ns")
        };

        let frac_time = d1 as f64 + ((d2 - d1 * 1000) as f64) / 1000.0;
        let buff: &mut[u8] = &mut [0; 128];
        let s = {
            let mut cursor = Cursor::new(buff);
            write!(cursor, "{:.2} {}", frac_time, t).unwrap();
            let p = cursor.position();
            let buff = cursor.into_inner();
            ::std::str::from_utf8(&buff[..p as usize]).unwrap()
        };
        f.pad(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        fn check(s: u64, mil: u32, mic: u32, ns: u32, expected: &str) {
            let actual = format!("{}", new_ed(s, mil, mic, ns));
            assert_eq!(expected, actual);
        }

        check(1, 0, 0, 0,    "1.00 s");
        check(1, 300, 0, 0,  "1.30 s");
        check(0, 1, 300, 0,  "1.30 ms");
        check(0, 0, 1, 300,  "1.30 μs");
        check(0, 0, 0, 1,    "1.00 ns");

        check(20, 0, 0, 0,   "20.00 s");
        check(20, 300, 0, 0, "20.30 s");
        check(0, 20, 300, 0, "20.30 ms");

        check(1, 1, 1, 1,    "1.00 s");

        check(1_000_000_000, 999, 999, 999,    "1000000001.00 s");
    }

    #[test]
    fn padding_works() {
        let ed = new_ed(20, 0, 0, 0);
        assert_eq!(format!("{:20}",  ed), "20.00 s             ");
        assert_eq!(format!("{:<20}", ed), "20.00 s             ");
        assert_eq!(format!("{:>20}", ed), "             20.00 s");
        assert_eq!(format!("{:^20}", ed), "      20.00 s       ");
    }

    #[test]
    fn test_accessors() {
        let ed = new_ed(92, 62, 13, 47);
        assert_eq!(ed.seconds(), 92);
        assert_eq!(ed.millis(), 92062);
        assert_eq!(ed.micros(), 92062013);
        assert_eq!(ed.nanos(), 92062013047);
        assert_eq!(ed.0.subsec_nanos() as u64, ed.nanos() - ed.seconds() * 1_000_000_000);
    }

    #[test]
    #[should_panic]
    fn panics_on_huge_times() {
        // In theory, we could handle this,
        format!("{}", ElapsedDuration::new(::std::time::Duration::from_secs(::std::u64::MAX)));
    }

    #[test]
    fn test_measure_time() {
        let (elapsed, result) = measure_time(|| {
            ::std::thread::sleep(Duration::from_millis(100));
            92
        });
        assert!(elapsed.millis() > 90);
        assert!(elapsed.millis() < 1000);
        assert_eq!(result, 92);
    }

    fn new_ed(s: u64, mil: u32, mic: u32, ns: u32) -> ElapsedDuration {
        let duration = ::std::time::Duration::new(s, (mil * 1000 + mic) * 1000 + ns);
        ElapsedDuration(duration)
    }
}
