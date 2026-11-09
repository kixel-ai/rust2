//! The entry point both sides of the differential test run. The first byte picks a
//! case and the rest is its input.

use tokio::sync::mpsc;

/// What a case runs on the rest of the input, writing its log.
type Case = fn(&[u8], &mut String);

/// Each case's name and what it runs. The first byte picks one modulo their count, so a
/// case added at the end moves no other.
fn cases() -> Vec<(&'static str, Case)> {
	vec![
		("mpsc::bounded", crate::mpsc::bounded),
		("mpsc::unbounded", crate::mpsc::unbounded),
		("watch::script", crate::watch::script),
		("watch::copied", crate::watch::copied),
		("oneshot::script", crate::oneshot::script),
		("select::decoded", crate::select::decoded),
		("select::unbiased", crate::select::unbiased),
		("mpsc::zero", zero),
	]
}

/// Run one case.
pub fn run(input: &[u8]) -> Vec<u8> {
	// What an earlier case wrote before it panicked.
	crate::log::take();
	let mut out = String::new();
	let Some((&op, data)) = input.split_first() else {
		return Vec::new();
	};
	let cases = cases();
	let (_, case) = cases[usize::from(op) % cases.len()];
	case(data, &mut out);
	out.into_bytes()
}

/// The first byte that picks the case named `name`, which `caller.ts` targets by.
pub fn op(name: &str) -> u8 {
	match cases().iter().position(|&(n, _)| n == name) {
		Some(i) => i as u8,
		None => panic!("no case {name}"),
	}
}

/// A bounded channel of no room and of more than its semaphore holds, which panic, and of
/// the most room.
fn zero(data: &[u8], out: &mut String) {
	// The batch semaphore's most permits.
	let most = usize::MAX >> 3;
	let room = match data.first().map(|b| b % 3) {
		Some(0) => 0,
		Some(1) => most + 1,
		_ => most,
	};
	let (tx, _rx) = mpsc::channel::<u8>(room);
	out.push_str(&format!("{} {}", tx.capacity(), tx.max_capacity()));
}
