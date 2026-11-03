//! `Option`'s and `Result`'s combinators, which lower from core's own MIR. Each closure
//! logs that std called it, so the output shows which ran, in what order, and how often.

/// Run case `kind % 4` over `data`, whose bytes pick each `Some` or `None`, `Ok` or `Err`.
pub fn combinators(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let a = data.first().copied().filter(|x| x % 3 != 0);
	let b = data.get(1).copied().filter(|x| x % 5 != 0);
	let limit = data.get(2).copied().unwrap_or(0x80);
	match kind % 4 {
		0 => options(a, b, limit, out),
		1 => results(a, b, limit, out),
		2 => owned(a, b, out),
		_ => panics(a, b, limit, out),
	}
}

fn options(a: Option<u8>, b: Option<u8>, limit: u8, out: &mut Vec<u8>) {
	let some = a.is_some_and(|x| {
		out.push(0xa0);
		x > limit
	});
	out.push(some as u8);
	let none_or = b.is_none_or(|x| {
		out.push(0xa1);
		x < limit
	});
	out.push(none_or as u8);
	let chained = a.and_then(|x| x.checked_add(limit)).and_then(|x| {
		out.push(0xa2);
		b.map(|y| x ^ y)
	});
	out.push(chained.unwrap_or(0xee));
	let kept = a.filter(|x| {
		out.push(0xa3);
		*x > limit
	});
	out.push(kept.map_or(0xdd, |x| x / 3));
	let either = a.or_else(|| {
		out.push(0xa4);
		b
	});
	let shown = either.map_or_else(
		|| {
			out.push(0xa5);
			0xcc
		},
		|x| x.wrapping_add(3),
	);
	out.push(shown);
	out.push(a.xor(b).unwrap_or_default());
	let both = a.and(b).unwrap_or_else(|| {
		out.push(0xa6);
		0xbb
	});
	out.push(both);
	out.push(a.or(b).unwrap_or(0xaa));
	out.push(a.zip(b).map_or(0, |(x, y)| x.wrapping_add(y)));
	let seen = a.inspect(|x| out.push(*x));
	out.push(seen.is_some() as u8);
	let error = b.ok_or_else(|| {
		out.push(0xa7);
		u16::from(limit) << 1
	});
	push(error.map(|x| x ^ limit), out);
}

fn results(a: Option<u8>, b: Option<u8>, limit: u8, out: &mut Vec<u8>) {
	let r: Result<u8, u16> = a.ok_or(u16::from(limit) * 3);
	let s: Result<u8, u16> = b.ok_or_else(|| 0x1ff);
	let mapped = r
		.map(|x| {
			out.push(0xb1);
			x.wrapping_add(limit)
		})
		.map_err(|e| {
			out.push(0xb2);
			e.wrapping_add(1)
		});
	push(mapped, out);
	let chained = r.and_then(|x| x.checked_mul(limit).ok_or(7)).or_else(|e| {
		out.push(0xb3);
		match e % 2 {
			0 => Ok(1),
			_ => Err(e),
		}
	});
	push(chained, out);
	out.push(r.is_ok_and(|v| v > 3) as u8);
	let large = s.is_err_and(|e| {
		out.push(0xb4);
		e > 0x100
	});
	out.push(large as u8);
	out.push(r.map_or(0x11, |v| v / 2));
	out.push(s.map_or_else(|e| e as u8, |v| v ^ 0x55));
	out.push(r.ok().unwrap_or(0));
	out.push(s.err().map_or(0, |e| (e >> 8) as u8));
	out.push(r.and(s).unwrap_or_default());
	out.push(r.or(s).unwrap_or_else(|e| e as u8));
	let inspected = s.inspect(|v| out.push(*v)).inspect_err(|e| out.push(*e as u8));
	out.push(inspected.is_ok() as u8);
	// `copied` maps by a closure of core's own.
	let by_ref: Result<&u8, u16> = r.as_ref().map_err(|e| *e);
	out.push(by_ref.copied().unwrap_or(0x22));
}

/// Values JS shares where Rust clones or moves them.
fn owned(a: Option<u8>, b: Option<u8>, out: &mut Vec<u8>) {
	let s: Option<String> = a.map(|x| x.to_string());
	let kept = s.clone().filter(|s| s.len() > 1);
	out.extend_from_slice(kept.unwrap_or_default().as_bytes());
	out.push(b'|');
	let other = s.or_else(|| b.map(|y| y.to_string()));
	out.extend_from_slice(other.unwrap_or_else(|| "-".to_string()).as_bytes());
	out.push(b'|');
	let v: Result<Vec<u8>, String> = a.map(|x| vec![x; 2]).ok_or_else(|| "none".to_string());
	// A clone changed in place leaves the original as it was.
	let mut copy = v.clone();
	if let Ok(inner) = &mut copy {
		inner.push(7);
	}
	let grown = v.clone().map(|mut v| {
		v.push(9);
		v
	});
	for r in [v, copy, grown] {
		match r {
			Ok(v) => out.extend_from_slice(&v),
			Err(e) => out.extend_from_slice(e.as_bytes()),
		}
		out.push(b'|');
	}
	let empty: Option<Vec<u8>> = b.filter(|y| y % 2 == 0).map(|y| vec![y]);
	out.push(empty.unwrap_or_default().len() as u8);
	// A `Copy` payload rebuilt into another `Result` and changed there leaves the first's.
	let pair: Result<[u32; 2], u8> = a.map(|x| [u32::from(x), 2]).ok_or(3);
	let mut wide = pair.map_err(u16::from);
	if let Ok(ref mut p) = wide {
		p[0] = 9;
	}
	let mut matched: Result<[u32; 2], u16> = match pair {
		Ok(p) => Ok(p),
		Err(e) => Err(e.into()),
	};
	if let Ok(ref mut p) = matched {
		p[1] = 8;
	}
	for r in [pair.map_err(u16::from), wide, matched] {
		match r {
			Ok([x, y]) => out.extend_from_slice(&[x as u8, y as u8]),
			Err(e) => out.push(e as u8),
		}
	}
}

/// A panic inside a closure std calls.
fn panics(a: Option<u8>, b: Option<u8>, limit: u8, out: &mut Vec<u8>) {
	let v = a.and_then(|x| match x == limit {
		true => panic!("closure saw {x}"),
		false => b,
	});
	out.push(v.unwrap_or(1));
	let r: Result<u8, u8> = b.ok_or(limit);
	let w = r.unwrap_or_else(|e| match e % 4 {
		0 => panic!("error {e}"),
		_ => e,
	});
	out.push(w);
}

fn push(r: Result<u8, u16>, out: &mut Vec<u8>) {
	match r {
		Ok(v) => out.extend_from_slice(&[1, v]),
		Err(e) => out.extend_from_slice(&e.to_be_bytes()),
	}
}
