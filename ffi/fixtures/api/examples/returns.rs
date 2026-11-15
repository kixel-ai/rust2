use api::*;

fn main() {
	let mut point = Point { x: 3, y: -5 };
	for fail in [false, true, false] {
		match return_values(&mut point, fail) {
			Ok((text, Some((a, b, c, d)), rows)) => {
				println!("values {text} {a} {b} {c} {d}");
				println!(
					"rows {} {} {} {} {} {}",
					rows[0].len(),
					rows[0][0],
					rows[0][1],
					rows[0][2],
					rows[1].len(),
					rows[1][9999]
				);
			}
			Err(_) => println!("values error"),
			_ => unreachable!(),
		}
		println!("point {} {}", point.x, point.y);
	}
	for fail in [false, true] {
		match return_moved(Counter::new("moved", u64::MAX), fail) {
			Ok(counter) => println!("moved {} {}", counter.name(), counter.get()),
			Err(_) => println!("moved error"),
		}
	}
	for hex in ["ff0080fe", "bad"] {
		match fingerprint(hex) {
			Ok(value) => println!("array {} {} {} {}", value[0], value[1], value[2], value[3]),
			Err(_) => println!("array error"),
		}
	}
	for unit in [true, false] {
		match corner(unit) {
			Ok((a, b)) => println!("corner {a} {b}"),
			Err(_) => println!("corner error"),
		}
	}
}
