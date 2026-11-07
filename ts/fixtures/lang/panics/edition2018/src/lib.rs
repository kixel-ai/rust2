//! `panic!` in a 2018-edition crate, whose message is any value: none, a literal, text, a
//! value of another type, and a format string with arguments.

/// Panic as `kind` picks, or write the first byte.
#[expect(non_fmt_panics, reason = "a 2018 crate's `panic!` takes any value as its payload")]
pub fn panicked(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	match kind % 7 {
		0 => panic!(),
		1 => panic!("a literal"),
		2 => panic!(format!("{} bytes", data.len())),
		3 => panic!(if first % 2 == 0 { "even" } else { "odd" }),
		4 => panic!(first),
		5 => panic!("{} bytes", data.len()),
		_ => {
			if first > 200 {
				panic!("past 200");
			}
			out.push(first);
		}
	}
}
