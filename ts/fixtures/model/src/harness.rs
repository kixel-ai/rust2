//! The one entry point both sides of the differential test run: native Rust built for
//! wasm32, and its TypeScript translation. The first input byte picks what to exercise and
//! the rest is its input; the output records every result and error, so the two sides
//! agree only if each step does.

use std::cmp::Ordering;
use std::num::NonZero;
use std::time::Duration;

use moq_net::{TimeOverflow, Timescale, Timestamp};

/// What a case runs on the rest of the input.
type Case = fn(&[u8], &mut Vec<u8>);

/// Each case's name and what it runs. The first byte picks one modulo their count, so a
/// case added at the end moves no other.
fn cases() -> Vec<(&'static str, Case)> {
	vec![
		("timescales", timescales),
		("timestamps", timestamps),
		("conversions", conversions),
		("arithmetic", arithmetic),
		("durations", durations),
	]
}

/// Run one case.
pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	let Some((&op, data)) = input.split_first() else {
		return out;
	};
	let cases = cases();
	let (_, case) = cases[usize::from(op) % cases.len()];
	case(data, &mut out);
	out
}

/// Take up to 8 big-endian bytes as a `u64`, zero-padded on the right.
fn take_u64(data: &[u8]) -> (u64, &[u8]) {
	let mut buf = [0u8; 8];
	let n = data.len().min(8);
	buf[..n].copy_from_slice(&data[..n]);
	(u64::from_be_bytes(buf), &data[n..])
}

/// A value that lands in a timestamp's range about as often as past it: a full-width `u64`
/// shifted right by the selector's low six bits.
fn take_value(data: &[u8]) -> (u64, &[u8]) {
	let Some((&selector, data)) = data.split_first() else {
		return (0, data);
	};
	let (value, data) = take_u64(data);
	(value >> (selector % 64), data)
}

/// A scale: one of the named constants, or a raw one, which may be zero or past a varint.
fn take_scale(data: &[u8]) -> (u64, &[u8]) {
	let Some((&selector, data)) = data.split_first() else {
		return (1, data);
	};
	match selector % 6 {
		0 => (Timescale::SECOND.as_u64(), data),
		1 => (Timescale::MILLI.as_u64(), data),
		2 => (Timescale::MICRO.as_u64(), data),
		3 => (Timescale::NANO.as_u64(), data),
		4 => take_value(data),
		_ => take_u64(data),
	}
}

/// A timestamp, or the error its value or scale makes.
fn take_timestamp(data: &[u8]) -> (Result<Timestamp, TimeOverflow>, &[u8]) {
	let (value, data) = take_value(data);
	let (scale, data) = take_scale(data);
	(Timestamp::from_scale(value, scale), data)
}

fn put_u64(out: &mut Vec<u8>, value: u64) {
	out.extend_from_slice(&value.to_be_bytes());
}

fn put_u128(out: &mut Vec<u8>, value: u128) {
	out.extend_from_slice(&value.to_be_bytes());
}

/// Text callers see, such as a `Debug` or `Display`, ended by a zero byte.
fn put_text(out: &mut Vec<u8>, text: String) {
	out.extend_from_slice(text.as_bytes());
	out.push(0);
}

fn put_ordering(out: &mut Vec<u8>, ordering: Ordering) {
	out.push(ordering as i8 as u8);
}

/// A result's tag, then the value `ok` records or the error's `Display` and `Debug`.
fn put_result<T>(out: &mut Vec<u8>, result: Result<T, TimeOverflow>, ok: impl FnOnce(&mut Vec<u8>, T)) {
	match result {
		Ok(value) => {
			out.push(1);
			ok(out, value);
		}
		Err(err) => {
			out.push(0);
			put_text(out, err.to_string());
			put_text(out, format!("{err:?}"));
		}
	}
}

/// Everything a timestamp reads back.
fn put_timestamp(out: &mut Vec<u8>, time: Timestamp) {
	put_u64(out, time.value());
	put_u64(out, time.scale().as_u64());
	out.push(u8::from(time.is_zero()));
	put_u64(out, time.as_secs());
	put_u128(out, time.as_millis());
	put_u128(out, time.as_micros());
	put_u128(out, time.as_nanos());
	put_text(out, format!("{time:?}"));
}

/// A raw scale through each constructor, and what the scale reads back.
fn timescales(data: &[u8], out: &mut Vec<u8>) {
	let (raw, _) = take_scale(data);
	put_result(out, Timescale::new(raw), |out, scale| {
		put_u64(out, scale.as_u64());
		put_u64(out, u64::from(scale));
		put_u64(out, NonZero::from(scale).get());
		put_text(out, format!("{scale:?}"));
		put_text(out, format!("{scale}"));
	});
	put_result(out, Timescale::try_from(raw), |out, scale| put_u64(out, scale.as_u64()));
	if let Some(raw) = NonZero::new(raw) {
		put_result(out, Timescale::try_from(raw), |out, scale| put_u64(out, scale.as_u64()));
	}
}

/// A value through each constructor, and what the timestamp reads back.
fn timestamps(data: &[u8], out: &mut Vec<u8>) {
	let (value, data) = take_value(data);
	let (scale, _) = take_scale(data);
	put_result(out, Timestamp::from_scale(value, scale), put_timestamp);
	put_result(out, Timestamp::from_secs(value), put_timestamp);
	put_result(out, Timestamp::from_millis(value), put_timestamp);
	put_result(out, Timestamp::from_micros(value), put_timestamp);
	put_result(out, Timestamp::from_nanos(value), put_timestamp);
	put_timestamp(out, Timestamp::ZERO);
}

/// A timestamp re-expressed at another scale.
fn conversions(data: &[u8], out: &mut Vec<u8>) {
	let (time, data) = take_timestamp(data);
	let (target, _) = take_scale(data);
	let Ok(time) = time else {
		return out.push(0xee);
	};
	put_result(out, Timescale::new(target), |out, target| {
		put_u128(out, time.as_scale(target));
		put_result(out, time.convert(target), put_timestamp);
	});
	put_result(out, time.convert(Timescale::MILLI), put_timestamp);
}

/// Two timestamps added, subtracted, compared, and sorted with a third.
fn arithmetic(data: &[u8], out: &mut Vec<u8>) {
	let (a, data) = take_timestamp(data);
	let (b, data) = take_timestamp(data);
	let (c, _) = take_timestamp(data);
	let (Ok(a), Ok(b)) = (a, b) else {
		return out.push(0xee);
	};
	put_result(out, a.checked_add(b), put_timestamp);
	put_result(out, a.checked_sub(b), put_timestamp);
	put_result(out, b.checked_sub(a), put_timestamp);
	put_ordering(out, a.cmp(&b));
	out.push(u8::from(a == b));
	out.push(u8::from(a < b));
	out.push(u8::from(a.partial_cmp(&b) == Some(Ordering::Greater)));
	put_text(out, format!("{:?}", a.max(b)));
	let mut sorted: Vec<Timestamp> = [a, b].into_iter().chain(c).collect();
	sorted.sort();
	for time in sorted {
		put_timestamp(out, time);
	}
}

/// A `Duration` to a timestamp, and a timestamp to a `Duration`.
fn durations(data: &[u8], out: &mut Vec<u8>) {
	let (secs, data) = take_value(data);
	let (nanos, data) = take_u64(data);
	let duration = Duration::new(secs, (nanos % 1_000_000_000) as u32);
	put_result(out, Timestamp::try_from(duration), put_timestamp);
	let (time, _) = take_timestamp(data);
	put_result(out, time, |out, time| {
		let duration = Duration::from(time);
		put_u64(out, duration.as_secs());
		out.extend_from_slice(&duration.subsec_nanos().to_be_bytes());
	});
}
