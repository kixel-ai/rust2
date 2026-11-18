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

// `src/needs.ll`, which the build script compiles for wasm32 alone.
#[cfg(target_arch = "wasm32")]
#[link(name = "needs", kind = "static")]
unsafe extern "C" {
	/// A C function calling `lacks_callback` back.
	fn needs_callback();
	/// A C function calling one nothing defines.
	#[cfg(feature = "undefined")]
	fn needs_missing();
	/// A C function reading a static nothing defines.
	#[cfg(feature = "undefined")]
	fn needs_data() -> i32;
	/// A C function calling the function `LACKS_TABLE` holds.
	fn needs_table();
}

/// What the C library calls through, which reads a file.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
static LACKS_TABLE: extern "C" fn() = tabled;

#[cfg(target_arch = "wasm32")]
extern "C" fn tabled() {
	let _ = std::fs::metadata("table");
}

/// What the C library calls back, which reads a file.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
extern "C" fn lacks_callback() {
	let _ = std::fs::metadata("callback");
}

/// A call of C, which calls back into Rust.
#[cfg(target_arch = "wasm32")]
pub fn called_back() {
	// SAFETY: the fixture is only exported, never run.
	unsafe { needs_callback() }
}

/// A call of C, which calls what nothing defines.
#[cfg(all(target_arch = "wasm32", feature = "undefined"))]
pub fn missed() {
	// SAFETY: the fixture is only exported, never run.
	unsafe { needs_missing() }
}

/// A call of C, which reads what nothing defines.
#[cfg(all(target_arch = "wasm32", feature = "undefined"))]
pub fn read_missing() -> i32 {
	// SAFETY: the fixture is only exported, never run.
	unsafe { needs_data() }
}

/// A call of C, which calls through a static the crate defines.
#[cfg(target_arch = "wasm32")]
pub fn tabled_call() {
	// SAFETY: the fixture is only exported, never run.
	unsafe { needs_table() }
}

/// What the module runs when it loads, which no export calls.
#[used]
#[unsafe(link_section = ".init_array")]
static STARTUP: extern "C" fn() = startup;

extern "C" fn startup() {
	let _ = std::fs::metadata("startup");
}

#[link(wasm_import_module = "rust2")]
unsafe extern "C" {
	/// A function of the binding's own module that the binding does not implement.
	fn rust2_spoof();
}

/// A call of what the module imports from the binding's module by a name it lacks.
pub fn spoofed() {
	// SAFETY: the fixture is only exported, never run.
	unsafe { rust2_spoof() }
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

/// What the module exports as C's, which no export of the binding calls.
#[unsafe(no_mangle)]
extern "C" fn lacks_exported() {
	let _ = std::fs::metadata("exported");
}

/// What the module exports as C's, which no export of the binding calls, calling a browser
/// API: wasm-bindgen's panicking stand-in fails neither the module's load nor its link, so it
/// is not refused.
#[unsafe(no_mangle)]
extern "C" fn lacks_logged() {
	web_sys::console::log_1(&"exported".into());
}

/// Polymorphic recursion two ways at each step, which a breadth-first walk would expand
/// exponentially before reaching the limit.
#[cfg(feature = "recursion")]
#[allow(unconditional_recursion)]
fn nest<T>(deeper: bool) {
	if deeper {
		nest::<(T,)>(deeper)
	} else {
		nest::<[T; 1]>(deeper)
	}
}

/// A call of what instantiates itself without end.
#[cfg(feature = "recursion")]
pub fn nested(deeper: bool) {
	nest::<u8>(deeper)
}
