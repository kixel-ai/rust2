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

// Kept: the macro uses the token it is passed for more than the visibility, so narrowing
// it would change what `label` returns.
labeled!(pub label);

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
	println!("extra {}", extra::used());
	println!("crlf {}", api::crlf::spaced());
}
