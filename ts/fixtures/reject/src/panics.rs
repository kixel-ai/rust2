//! `std::panic` that needs where a call is in std's source: a `#[track_caller]` function
//! that std's code calls, through its `Fn` impls, and the hook that reads each panic's
//! `Location`.

use std::panic::Location;

type Here = &'static Location<'static>;

#[track_caller]
fn here() -> Here {
	Location::caller()
}

#[track_caller]
fn at(_: u8) -> Here {
	Location::caller()
}

fn call(f: impl Fn() -> Here) -> Here {
	f()
}

/// A `#[track_caller]` function std's code calls, which natively passes a location in
/// std's source.
pub fn caller(data: &[u8]) -> u32 {
	let mapped = data.first().copied().map(at).map_or(0, Location::line);
	let iterated = data.iter().copied().map(at).count() as u32;
	mapped + iterated + call(here).column()
}

/// A hook reads each panic's `Location`.
pub fn hooked() {
	let hook = std::panic::take_hook();
	std::panic::set_hook(hook);
}
