//! Temporaries whose values the translation could compute where they are read rather
//! than where they are assigned: each case's output or panic changes if a call, a read
//! of memory a call changes, or an overflow check moves past another, or a call runs
//! twice or not at all.

/// Steps `x` and gives its new value.
fn bump(x: &mut u8) -> u8 {
	*x = x.wrapping_mul(3).wrapping_add(1);
	*x
}

/// `x`, unless it is zero, which panics.
fn nonzero(x: u8) -> u8 {
	if x == 0 {
		panic!("zero");
	}
	x
}

struct Pair {
	small: bool,
	value: u8,
}

/// Steps `x`, giving its new value's remainder by 3 in a `Pair`.
fn stepped(x: &mut u8) -> Pair {
	Pair {
		small: false,
		value: bump(x) % 3,
	}
}

/// Run the case the first byte picks on the next two.
pub fn run(input: &[u8]) -> Vec<u8> {
	let Some((&pick, data)) = input.split_first() else {
		return Vec::new();
	};
	let mut a = data.first().copied().unwrap_or(0);
	let b = data.get(1).copied().unwrap_or(0);
	match pick % 11 {
		// A call's result, then memory it changed.
		0 => {
			let r = bump(&mut a);
			let s = a;
			vec![r, s]
		}
		// Memory, then a call that changes it.
		1 => {
			let s = a;
			let r = bump(&mut a);
			vec![s, r]
		}
		// Two calls' results in one value.
		2 => vec![bump(&mut a), bump(&mut a), a],
		// Arguments around a call that changes what the others read.
		3 => {
			let p = &mut a;
			let first = *p;
			let mid = bump(p);
			vec![first, mid, *p]
		}
		// A call that can panic, then an overflow check that can.
		4 => {
			let r = nonzero(a);
			let s = b + 1;
			vec![r, s]
		}
		// A call's result given to a binding, a closure's spread arguments, and an array
		// repeated from it: each runs the call once.
		5 => {
			let mut v = Vec::new();
			v.push(bump(&mut a));
			let add = |x: u8, y: u8| x.wrapping_add(y);
			v.push(add(bump(&mut a), b));
			v.extend_from_slice(&[bump(&mut a); 3]);
			v.push(a);
			v
		}
		// A struct read where a branch tests it.
		6 => match (Pair {
			small: a < b,
			value: a ^ b,
		})
		.small
		{
			true => vec![1, (Pair { small: false, value: b }).value],
			false => vec![0],
		},
		// A checked subtraction whose result a call then reads.
		7 => vec![nonzero(a - b)],
		// Calls right of booleans' `&`, `|`, and `<`, which run whatever is on the left.
		8 => {
			let and = (a > 9) & (bump(&mut a) > 9);
			let or = (b > 9) | (bump(&mut a) > 9);
			#[expect(clippy::bool_comparison, reason = "the comparison Go lowers to `&&`")]
			let lt = (b > 9) < (nonzero(b) > 9);
			vec![and as u8, or as u8, lt as u8, a]
		}
		// A field of a call's result tested against several values: the call runs once.
		9 => match stepped(&mut a).value {
			0 => vec![0, a],
			1 => vec![1, a],
			_ => vec![2, a],
		},
		// A call that can panic, then one that changes memory.
		_ => {
			let r = nonzero(b);
			let s = bump(&mut a);
			vec![r, s, a]
		}
	}
}
