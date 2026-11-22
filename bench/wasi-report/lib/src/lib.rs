//! The binding benchmark's call shapes, for a WASI core module and a component alike.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// Two counts copied across.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
	pub start: u64,
	pub end: u64,
}

/// A new object per call.
pub struct Info {
	priority: i8,
}

impl Info {
	pub fn priority(&self) -> i8 {
		self.priority
	}
}

/// One handle each case borrows shared.
pub struct Track {
	closed: AtomicBool,
	start: u64,
	frame: Mutex<Vec<u8>>,
	payload: bytes::Bytes,
}

impl Default for Track {
	fn default() -> Self {
		Self::new()
	}
}

impl Track {
	pub fn new() -> Self {
		Self {
			closed: AtomicBool::new(false),
			start: 7,
			frame: Mutex::new(Vec::with_capacity(4096)),
			payload: bytes::Bytes::from(vec![1u8; 100]),
		}
	}

	pub fn is_closed(&self) -> bool {
		self.closed.load(Ordering::Relaxed)
	}

	pub fn start(&self) -> u64 {
		self.start
	}

	pub fn largest(&self) -> Option<Span> {
		Some(Span { start: self.start, end: self.start + 1 })
	}

	pub fn info(&self) -> Info {
		Info { priority: 3 }
	}

	/// A frame copied into Rust.
	pub fn write(&self, frame: &[u8]) {
		let mut held = self.frame.lock().unwrap();
		held.clear();
		held.extend_from_slice(frame);
	}

	/// 100 bytes handed out.
	pub fn payload(&self) -> bytes::Bytes {
		self.payload.clone()
	}

	/// The first `len` bytes of 1 MiB handed out.
	pub fn big(&self, len: u32) -> bytes::Bytes {
		static BIG: std::sync::OnceLock<bytes::Bytes> = std::sync::OnceLock::new();
		BIG.get_or_init(|| bytes::Bytes::from(vec![2u8; 1 << 20])).slice(..len as usize)
	}

	/// A read ready at its first poll.
	pub async fn read(&self) -> u64 {
		self.start
	}
}

/// What `visit` returns, once it called `f`: a closure lent for the call.
pub fn visit(f: impl FnOnce() -> u64) -> u64 {
	f() + 1
}

/// A panic, which aborts.
pub fn boom() {
	panic!("boom")
}

/// Nothing: the floor of a call.
pub fn noop() {}
