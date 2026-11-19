//! What an abort leaves on Android: stderr goes nowhere there, so a message that only
//! printed there vanishes. It goes to logcat and to bionic's abort message instead, which
//! the tombstone, `ApplicationExitInfo`, and Play vitals show.

use std::cell::RefCell;
use std::ffi::{CString, c_char, c_int};
use std::panic::PanicHookInfo;

#[link(name = "log")]
unsafe extern "C" {
	fn __android_log_write(priority: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

unsafe extern "C" {
	fn android_set_abort_message(message: *const c_char);
}

/// `ANDROID_LOG_FATAL`.
const FATAL: c_int = 7;

/// What a panic that reaches the C ABI prints second, after its own message, as the abort
/// it becomes.
const CANNOT_UNWIND: &str = "panic in a function that cannot unwind";

/// Write `text` to logcat and set it as the abort message, which the abort that follows
/// shows in its tombstone. Bionic keeps the first abort message and ignores each later one,
/// and liblog sets a fatal line as one too, so the first call in a process fixes what its
/// tombstone shows; logcat shows every call.
pub fn report(text: &str) {
	// A NUL would end either early.
	let text = CString::new(text.replace('\0', "\u{fffd}")).expect("no NUL is left");
	// SAFETY: both copy the NUL-terminated strings they are given.
	unsafe {
		__android_log_write(FATAL, c"rust2-ffi".as_ptr(), text.as_ptr());
		android_set_abort_message(text.as_ptr());
	}
}

thread_local! {
	/// The message of the last panic on this thread.
	static LAST: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Report each panic, as the standard library's panic runtime does only in a build that
/// aborts on one, which this library is not. A panic that crosses the C ABI aborts after a
/// second panic whose message does not name its cause, so that one reports both. On a thread
/// with no Rust frame to unwind to, such as ART's, it aborts at once with no second panic, so
/// the hook cannot tell a panic that aborts from one a caller catches: it reports each, and
/// bionic keeps the first, which a caught panic that precedes an abort takes.
fn hook(info: &PanicHookInfo<'_>) {
	let text = info.to_string();
	LAST.with_borrow_mut(|last| {
		if text.ends_with(CANNOT_UNWIND) && !last.is_empty() {
			report(&format!("{last}\n{text}"));
		} else {
			report(&text);
			*last = text;
		}
	});
}

/// Run as the library loads, so a panic before any call reports too.
extern "C" fn install() {
	let previous = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |info| {
		hook(info);
		previous(info);
	}));
}

#[used]
#[unsafe(link_section = ".init_array")]
static INSTALL: extern "C" fn() = install;
