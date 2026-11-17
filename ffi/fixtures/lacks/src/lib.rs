//! Exports reaching what the web target's host lacks, each by another route the walk
//! follows, and exports reaching only what it has, which export.

use std::os::fd::FromRawFd;
use std::time::{Duration, UNIX_EPOCH};

/// A file read: the file system, which a web page has none of.
pub fn read(path: String) -> Option<String> {
	std::fs::read_to_string(path).ok()
}

/// A sleep, which a web page's one thread cannot do.
pub fn nap(ms: u64) {
	std::thread::sleep(Duration::from_millis(ms));
}

/// A socket accepting a connection.
pub struct Listener(std::net::TcpListener);

impl Listener {
	/// The listener a host passed as `fd`.
	pub fn adopt(fd: i32) -> Listener {
		// SAFETY: the fixture is only exported, never run.
		Listener(unsafe { std::net::TcpListener::from_raw_fd(fd) })
	}

	/// Whether a connection came.
	pub fn accept(&self) -> bool {
		self.0.accept().is_ok()
	}
}

trait Source {
	fn load(&self) -> String;
}

struct Disk;

impl Source for Disk {
	fn load(&self) -> String {
		std::fs::read_to_string("disk").unwrap_or_default()
	}
}

/// A file read through `dyn`, refused in the implementation that reads.
pub fn loaded() -> String {
	let source: Box<dyn Source> = Box::new(Disk);
	source.load()
}

fn exists() -> bool {
	std::fs::metadata("pointer").is_ok()
}

/// A file read through a function pointer.
pub fn pointed() -> bool {
	let f: fn() -> bool = exists;
	f()
}

fn listed() -> bool {
	std::fs::exists("listed").unwrap_or(false)
}

static HANDLERS: [fn() -> bool; 1] = [listed];

/// A file read through a static's table of functions.
pub fn handled() -> bool {
	HANDLERS[0]()
}

/// A file removed when its handle drops.
pub struct Temp {
	path: String,
}

impl Temp {
	pub fn new(path: String) -> Temp {
		Temp { path }
	}
}

impl Drop for Temp {
	fn drop(&mut self) {
		let _ = std::fs::remove_file(&self.path);
	}
}

/// A file read once the future is polled.
pub async fn fetch(path: String) -> Option<String> {
	std::fs::read_to_string(path).ok()
}

#[cfg(feature = "undefined")]
unsafe extern "C" {
	/// A C function no library the module links defines.
	fn undefined();
}

/// A call of what nothing defines.
#[cfg(feature = "undefined")]
pub fn linked() {
	// SAFETY: the fixture is only exported, never run.
	unsafe { undefined() }
}

#[link(wasm_import_module = "page")]
unsafe extern "C" {
	/// A function of a module the binding does not implement.
	fn alert();
}

/// A call of what the module imports from elsewhere.
pub fn shout() {
	// SAFETY: the fixture is only exported, never run.
	unsafe { alert() }
}

/// A browser API through web-sys, whose wasm-bindgen panics on WASI rather than import it.
pub fn log(text: String) {
	web_sys::console::log_1(&text.into());
}

/// The real-time clock, which the host has.
pub fn now() -> u64 {
	let since = std::time::SystemTime::now().duration_since(UNIX_EPOCH);
	since.map(|d| d.as_secs()).unwrap_or(0)
}

/// Randomness, which the host has: a map's keys are seeded from it.
pub fn seeded() -> usize {
	std::collections::HashMap::<u8, u8>::new().capacity()
}

/// Output and the environment, which the host has.
pub fn greet() {
	println!("hello from {:?}", std::env::var("USER").ok());
}
