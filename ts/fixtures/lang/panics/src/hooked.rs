//! `panic::set_hook` and `take_hook`, and the `PanicHookInfo` a hook reads: its payload,
//! as text and downcast, its location, and its `Display`, for panics of the crate's own
//! code, of std's functions that take their caller's location, of those an integer's macro
//! defines, and of std's own code, at the path the native build names; a hook that
//! panics, which aborts; and `thread::panicking` within one and outside.
//!
//! Each case's hook writes what it read to [`NOTE`], which the host reads once the panic
//! traps, and passes the panic on to the hook it replaced, the harness's natively.

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::panic::{self, PanicHookInfo};
use std::time::Duration;

use crate::NOTE;

/// Write what `info` says: its `Display`, its payload's type and text, and its location's
/// parts, and whether the thread is panicking.
fn record(info: &PanicHookInfo<'_>) {
	let payload = info.payload();
	let kind = if payload.is::<&'static str>() {
		"&str".to_string()
	} else if payload.is::<String>() {
		"String".to_string()
	} else if let Some(n) = payload.downcast_ref::<u32>() {
		format!("u32 {n}")
	} else {
		"other".to_string()
	};
	let at = info.location().unwrap();
	let line = format!(
		"{}\n{kind} {:?} {} {} {} {}\n",
		info.to_string().replace(at.file(), &std_named(at.file())),
		info.payload_as_str(),
		std_named(at.file()),
		at.line(),
		at.column(),
		std::thread::panicking(),
	);
	NOTE.with_borrow_mut(|note| note.push_str(&line));
}

/// `file`, with std's source under one name. The native build names std's code by the
/// sysroot's copy of its source where it compiled that code into this crate, as it does a
/// generic or inlined function, and by `/rustc/<commit>/library/` where std's own build
/// did, as the translation always does.
fn std_named(file: &str) -> String {
	match file.split_once("/library/") {
		Some((root, rest)) if root.starts_with("/rustc/") || root.ends_with("/lib/rustlib/src/rust") => {
			format!("<std>/library/{rest}")
		}
		_ => file.to_string(),
	}
}

/// Replace the hook with one that records each panic, then passes it on.
fn hook() {
	let prev = panic::take_hook();
	panic::set_hook(Box::new(move |info| {
		record(info);
		prev(info);
	}));
}

#[track_caller]
fn tracked(n: u8) -> u8 {
	panic!("tracked {n}")
}

/// One panic, by `kind`, with a hook set. `data`'s first byte varies its operands.
pub fn panicked(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	hook();
	let v = vec![n, n];
	match kind % 26 {
		0 => panic!("literal"),
		1 => panic!("formatted {n}"),
		2 => out.push(v[usize::from(n) + 2]),
		3 => out.push(n.max(200) + n.max(100)),
		4 => out.push(Some(n).filter(|_| false).unwrap()),
		5 => out.push(Err::<u8, u8>(n).unwrap()),
		6 => out.push(Some(n).filter(|_| false).expect("expected")),
		7 => out.extend_from_slice(&v[usize::from(n % 2) + 2..1]),
		8 => {
			let cell = RefCell::new(n);
			let _held = cell.borrow_mut();
			out.push(*cell.borrow_mut());
		}
		9 => out.push(n.max(2).pow(9)),
		10 => out.push(100 / n.wrapping_mul(0)),
		11 => out.push(v.clone().remove(usize::from(n) + 2)),
		12 => out.push(v.clone().swap_remove(usize::from(n) + 2)),
		13 => v.clone().rotate_left(usize::from(n) + 3),
		14 => out.push(HashMap::<u8, u8>::new()[&n]),
		15 => panic::panic_any(u32::from(n)),
		16 => panic::panic_any(format!("any {n}")),
		17 => assert_eq!(n, n.wrapping_add(1), "differ"),
		18 => out.push(tracked(n)),
		19 => out.push(VecDeque::from(v.clone())[usize::from(n) + 2]),
		20 => out.push(u8::try_from((Duration::MAX + Duration::from_secs(u64::from(n.max(1)))).as_secs()).unwrap_or(0)),
		21 => out.extend_from_slice("éé"[..usize::from(n % 2) * 2 + 1].as_bytes()),
		22 => out.extend((0..n).step_by(usize::from(n) * 0)),
		23 => out.extend(v.clone().drain(usize::from(n) + 3..)),
		24 => panic::panic_any("any"),
		_ => {
			let len = HashMap::<u8, u8>::with_capacity(usize::MAX).len();
			out.push(len as u8);
		}
	}
}

/// A hook that panics, or that sets the hook, which aborts as a panic in a hook does,
/// without running the hook again.
pub fn aborted(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	let prev = panic::take_hook();
	panic::set_hook(Box::new(move |info| {
		record(info);
		match kind % 3 {
			0 => panic!("again"),
			1 => drop(panic::take_hook()),
			_ => prev(info),
		}
	}));
	out.push(n);
	panic!("first {n}");
}
