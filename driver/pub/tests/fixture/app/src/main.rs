use api::prelude::*;
use api::{Action, Holder, Source};

fn drain<S: Source>(source: &mut S) -> S::Item {
	source.next()
}

macro_rules! labeled {
	($v:vis $name:ident) => {
		$v fn $name() -> &'static str {
			stringify!($v)
		}
	};
}

// Restored: the macro stringifies the token it is passed, so narrowing it would change
// what `label` returns.
labeled!(pub label);

macro_rules! visible {
	($v:vis fn $name:ident() -> $ret:ty $body:block) => {
		$v fn $name() -> $ret $body
	};
}

// Narrowed: the macro uses the token it is passed as the visibility alone.
visible!(pub fn passed_through() -> u32 { 15 });

/// Restored: its derive prints its visibility, which narrowing would change.
#[derive(derives::Label)]
pub struct Labeled;

macro_rules! make_label {
	($name:ident, $label:literal) => {
		impl $name {
			fn nested() -> &'static str {
				$label
			}
		}
	};
}

/// Restored: its derive hands its visibility to another macro, `make_label!`.
#[derive(derives::NestedLabel)]
pub struct NestedLabeled;

/// Restored: its attribute adds a function returning its visibility.
#[derives::tag]
pub fn tagged() -> u32 {
	16
}

/// Narrowed: its attribute returns it unchanged.
#[derives::keep]
pub fn kept_through() -> u32 {
	17
}

/// Restored: `column!` reads a column that narrowing the `pub` before it shifts.
pub fn at_column() -> u32 { column!() }

/// Narrowed: narrowing shifts no line.
pub fn at_line() -> u32 {
	line!()
}

// External: the token is in an exported macro of extra, whose callers outside the audit
// get it too.
extra::make!(made);

fn main() {
	{
		let live = api::Live::new(42);
		println!("value {}", live.value);
		println!("generic {}", api::generic(&live));
		println!("dynamic {}", api::dynamic(&live));
		let built = api::Live::new(1);
		println!("run {}", (&built as &dyn Action).run());
	}
	println!("drops {}", api::drops());
	println!("expanded {}", api::expanded_used());
	println!("reexported {}", api::reexported());
	println!("globbed {}", globbed());
	let mut counter = api::Counter(0);
	println!("ticket {}", drain(&mut counter).0);
	println!("len {}", api::Outer::new().len());
	println!("held {}", api::Holds.held());
	let settings = api::Settings {
		named: 1,
		..Default::default()
	};
	println!("settings {}", settings.named);
	println!("label {}", label());
	println!("labeled {}", Labeled::label());
	println!("passed through {}", passed_through());
	println!("nested {}", NestedLabeled::nested());
	println!("tagged {} {}", tagged(), tagged_tag());
	println!("kept through {}", kept_through());
	println!("column {}", at_column());
	println!("line {}", at_line());
	println!("made {}", made());
	println!("extra {}", extra::used());
	println!("crlf {}", api::crlf::spaced());
}
