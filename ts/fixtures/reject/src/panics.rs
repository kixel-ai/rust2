//! `std::panic` that needs where a panic or a call is: a `Location`, and the hook that reads
//! one.

/// The caller each `#[track_caller]` function is given.
#[track_caller]
pub fn caller() -> u32 {
	std::panic::Location::caller().line()
}

/// A hook reads each panic's `Location`.
pub fn hooked() {
	let hook = std::panic::take_hook();
	std::panic::set_hook(hook);
}
