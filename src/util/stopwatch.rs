use std::fmt;

use time::{Duration, SteadyTime};

#[derive(Debug)]
pub struct Stopwatch {
	start: SteadyTime,
	stop: SteadyTime,
}

impl Stopwatch {
	pub fn new() -> Stopwatch {
		let time = SteadyTime::now();
		Stopwatch {
			start: time,
			stop: time,
		}
	}

	pub fn start(&mut self) {
		self.start = SteadyTime::now();
	}

	pub fn stop(&mut self) {
		self.stop = SteadyTime::now();
	}
}

impl Default for Stopwatch {
	fn default() -> Self {
		Self::new()
	}
}

fn format_long_seconds(f: &mut fmt::Formatter, mut seconds: i64) -> fmt::Result {
	debug_assert!(seconds > 1);
	let mut div: i64 = 1000;
	while div <= seconds {
		div *= 1000;
	}
	{
		div /= 1000;
		let step = seconds / div;
		write!(f, "{} ", step)?;
		seconds -= step * div;
	}
	while div > 1 {
		div /= 1000;
		let step = seconds / div;
		write!(f, "{:03} ", step)?;
		seconds -= step * div;
	}
	write!(f, "s")
}

fn format_duration(f: &mut fmt::Formatter, duration: &Duration) -> fmt::Result {
	let ms = duration.num_milliseconds();
	debug_assert!(ms >= 0);
	if let Some(ns) = duration.num_nanoseconds() {
		if ns >= 999_950_000_000 {
			let ns = ns + ns % 1_000_000_000;
			format_long_seconds(f, ns / 1_000_000_000)
		} else if ns >= 99_995_000_000 {
			let ns = ns + ns % 100_000_000;
			write!(f, "{}.{:01} s", ns / 1_000_000_000, ns % 1_000_000_000 / 100_000_000)
		} else if ns >= 9_999_500_000 {
			let ns = ns + ns % 10_000_000;
			write!(f, "{}.{:02} s", ns / 1_000_000_000, ns % 1_000_000_000 / 10_000_000)
		} else if ns >= 999_950_000 {
			let ns = ns + ns % 1_000_000;
			write!(f, "{}.{:03} s", ns / 1_000_000_000, ns % 1_000_000_000 / 1_000_000)
		} else if ns >= 99_995_000 {
			let ns = ns + ns % 100_000;
			write!(f, "{}.{:01} ms", ns / 1_000_000, ns % 1_000_000 / 100_000)
		} else if ns >= 9_999_500 {
			let ns = ns + ns % 10_000;
			write!(f, "{}.{:02} ms", ns / 1_000_000, ns % 1_000_000 / 10_000)
		} else if ns >= 999_950 {
			let ns = ns + ns % 1_000;
			write!(f, "{}.{:03} ms", ns / 1_000_000, ns % 1_000_000 / 1_000)
		} else if ns >= 99_995 {
			let ns = ns + ns % 100;
			write!(f, "{}.{:01} us", ns / 1_000, ns % 1_000 / 100)
		} else if ns >= 10_000 {
			let ns = ns + ns % 10;
			write!(f, "{}.{:02} us", ns / 1_000, ns % 1_000 / 10)
		} else if ns >= 1_000 {
			write!(f, "{}.{:03} us", ns / 1_000, ns % 1_000)
		} else {
			write!(f, "{:3} ns", ns)
		}
	} else {
		let ms = duration.num_milliseconds();
		let ms = ms + ms % 1_000;
		format_long_seconds(f, ms / 1_000)
	}
}

struct DurationWrapper<'a> {
	duration: &'a Duration,
}

impl<'a> fmt::Display for DurationWrapper<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		format_duration(f, self.duration)
	}
}

impl fmt::Display for Stopwatch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let duration = self.stop - self.start;
		write!(f, "{}", DurationWrapper { duration: &duration })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn duration_str(duration: &Duration) -> String {
		format!("{}", DurationWrapper { duration })
	}

	#[test]
	fn test_format_duration_1() {
		assert_eq!(duration_str(&Duration::nanoseconds(1)), "  1 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(2)), "  2 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(9)), "  9 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(0)), "  0 ns");
	}

	#[test]
	fn test_format_duration_2() {
		assert_eq!(duration_str(&Duration::nanoseconds(10)), " 10 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(12)), " 12 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(99)), " 99 ns");
	}

	#[test]
	fn test_format_duration_3() {
		assert_eq!(duration_str(&Duration::nanoseconds(100)), "100 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(123)), "123 ns");
		assert_eq!(duration_str(&Duration::nanoseconds(999)), "999 ns");
	}

	#[test]
	fn test_format_duration_4() {
		assert_eq!(duration_str(&Duration::nanoseconds(1_000)), "1.000 us");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234)), "1.234 us");
		assert_eq!(duration_str(&Duration::nanoseconds(9_999)), "9.999 us");
	}

	#[test]
	fn test_format_duration_5() {
		assert_eq!(duration_str(&Duration::nanoseconds(10_000)), "10.00 us");
		assert_eq!(duration_str(&Duration::nanoseconds(12_344)), "12.34 us");
		assert_eq!(duration_str(&Duration::nanoseconds(12_345)), "12.35 us");
		assert_eq!(duration_str(&Duration::nanoseconds(99_994)), "99.99 us");
	}

	#[test]
	fn test_format_duration_6() {
		assert_eq!(duration_str(&Duration::nanoseconds(99_995)), "100.0 us");
		assert_eq!(duration_str(&Duration::nanoseconds(123_449)), "123.4 us");
		assert_eq!(duration_str(&Duration::nanoseconds(123_450)), "123.5 us");
		assert_eq!(duration_str(&Duration::nanoseconds(999_949)), "999.9 us");
	}

	#[test]
	fn test_format_duration_7() {
		assert_eq!(duration_str(&Duration::nanoseconds(999_950)), "1.000 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234_499)), "1.234 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234_500)), "1.235 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(9_999_499)), "9.999 ms");
	}

	#[test]
	fn test_format_duration_8() {
		assert_eq!(duration_str(&Duration::nanoseconds(9_999_500)), "10.00 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(12_344_999)), "12.34 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(12_345_000)), "12.35 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(99_994_999)), "99.99 ms");
	}

	#[test]
	fn test_format_duration_9() {
		assert_eq!(duration_str(&Duration::nanoseconds(99_995_000)), "100.0 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(123_449_999)), "123.4 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(123_450_000)), "123.5 ms");
		assert_eq!(duration_str(&Duration::nanoseconds(999_949_999)), "999.9 ms");
	}

	#[test]
	fn test_format_duration_10() {
		assert_eq!(duration_str(&Duration::nanoseconds(999_950_000)), "1.000 s");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234_499_999)), "1.234 s");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234_500_000)), "1.235 s");
		assert_eq!(duration_str(&Duration::nanoseconds(9_999_499_999)), "9.999 s");
	}

	#[test]
	fn test_format_duration_11() {
		assert_eq!(duration_str(&Duration::nanoseconds(9_999_500_000)), "10.00 s");
		assert_eq!(duration_str(&Duration::nanoseconds(12_344_999_999)), "12.34 s");
		assert_eq!(duration_str(&Duration::nanoseconds(12_345_000_000)), "12.35 s");
		assert_eq!(duration_str(&Duration::nanoseconds(99_994_999_999)), "99.99 s");
	}

	#[test]
	fn test_format_duration_12() {
		assert_eq!(duration_str(&Duration::nanoseconds(99_995_000_000)), "100.0 s");
		assert_eq!(duration_str(&Duration::nanoseconds(123_449_999_999)), "123.4 s");
		assert_eq!(duration_str(&Duration::nanoseconds(123_450_000_000)), "123.5 s");
		assert_eq!(duration_str(&Duration::nanoseconds(999_949_999_999)), "999.9 s");
	}

	#[test]
	fn test_format_duration_13() {
		assert_eq!(duration_str(&Duration::nanoseconds(999_950_000_000)), "1 000 s");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234_499_999_999)), "1 234 s");
		assert_eq!(duration_str(&Duration::nanoseconds(1_234_500_000_000)), "1 235 s");
		assert_eq!(duration_str(&Duration::nanoseconds(9_999_499_999_999)), "9 999 s");
	}

	#[test]
	fn test_format_duration_14() {
		assert_eq!(duration_str(&Duration::nanoseconds(9_999_500_000_000)), "10 000 s");
		assert_eq!(duration_str(&Duration::nanoseconds(12_344_499_999_999)), "12 344 s");
		assert_eq!(duration_str(&Duration::nanoseconds(12_344_500_000_000)), "12 345 s");
		assert_eq!(duration_str(&Duration::nanoseconds(99_999_499_999_999)), "99 999 s");
	}

	#[test]
	fn test_format_duration_15() {
		assert_eq!(duration_str(&Duration::nanoseconds(99_999_500_000_000)), "100 000 s");
		assert_eq!(duration_str(&Duration::nanoseconds(123_444_499_999_999)), "123 444 s");
		assert_eq!(duration_str(&Duration::nanoseconds(123_444_500_000_000)), "123 445 s");
		assert_eq!(duration_str(&Duration::nanoseconds(999_999_499_999_999)), "999 999 s");
	}

	#[test]
	fn test_format_duration_16() {
		assert_eq!(duration_str(&Duration::nanoseconds(999_999_500_000_000)), "1 000 000 s");
		assert_eq!(
			duration_str(&Duration::nanoseconds(1_234_444_499_999_999)),
			"1 234 444 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(1_234_444_500_000_000)),
			"1 234 445 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(9_999_999_499_999_999)),
			"9 999 999 s"
		);
	}

	#[test]
	fn test_format_duration_17() {
		assert_eq!(
			duration_str(&Duration::nanoseconds(9_999_999_500_000_000)),
			"10 000 000 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(12_344_444_499_999_999)),
			"12 344 444 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(12_344_444_500_000_000)),
			"12 344 445 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(99_999_999_499_999_999)),
			"99 999 999 s"
		);
	}

	#[test]
	fn test_format_duration_18() {
		assert_eq!(
			duration_str(&Duration::nanoseconds(99_999_999_500_000_000)),
			"100 000 000 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(123_444_444_499_999_999)),
			"123 444 444 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(123_444_444_500_000_000)),
			"123 444 445 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(999_999_999_499_999_999)),
			"999 999 999 s"
		);
	}

	#[test]
	fn test_format_duration_19() {
		assert_eq!(
			duration_str(&Duration::nanoseconds(999_999_999_500_000_000)),
			"1 000 000 000 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(1_234_444_444_499_999_999)),
			"1 234 444 444 s"
		);
		assert_eq!(
			duration_str(&Duration::nanoseconds(1_234_444_444_500_000_000)),
			"1 234 444 445 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(9_999_999_999_499_999)),
			"9 999 999 999 s"
		);
	}

	#[test]
	fn test_format_duration_20() {
		assert_eq!(
			duration_str(&Duration::microseconds(9_999_999_999_500_000)),
			"10 000 000 000 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(12_344_444_444_499_999)),
			"12 344 444 444 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(12_344_444_444_500_000)),
			"12 344 444 445 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(99_999_999_999_499_999)),
			"99 999 999 999 s"
		);
	}

	#[test]
	fn test_format_duration_21() {
		assert_eq!(
			duration_str(&Duration::microseconds(99_999_999_999_500_000)),
			"100 000 000 000 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(123_444_444_444_499_999)),
			"123 444 444 444 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(123_444_444_444_500_000)),
			"123 444 444 445 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(999_999_999_999_499_999)),
			"999 999 999 999 s"
		);
	}

	#[test]
	fn test_format_duration_22() {
		assert_eq!(
			duration_str(&Duration::microseconds(999_999_999_999_500_000)),
			"1 000 000 000 000 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(1_234_444_444_444_499_999)),
			"1 234 444 444 444 s"
		);
		assert_eq!(
			duration_str(&Duration::microseconds(1_234_444_444_444_500_000)),
			"1 234 444 444 445 s"
		);
		assert_eq!(
			duration_str(&Duration::milliseconds(9_999_999_999_999_499)),
			"9 999 999 999 999 s"
		);
	}
}
