//! Values translation holds back rather than reads once: a borrow read as its place, a
//! place lent as an `rt.Place`, a copy of a reference, an aggregate read by its parts, a
//! closure's captures, and a temporary folded into its read. Each case assigns a variable
//! the held value names between the hold and the read, in a loop and across its
//! iterations, and the read must see what Rust read when the value was made.

struct Row {
	cells: [u8; 4],
}

fn set(x: &mut u8, v: u8) {
	*x = v;
}

fn whole(p: (&str, &Row, u8)) -> u8 {
	p.0.len() as u8 ^ p.1.cells[1] ^ p.2
}

pub fn holds(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let a = data.first().copied().unwrap_or(0);
	let b = data.get(1).copied().unwrap_or(0);
	match kind % 7 {
		0 => places(a, b, data, out),
		1 => indexed(data, out),
		2 => reborrows(a, b, data, out),
		3 => copies(a, data, out),
		4 => closures(a, b, data, out),
		5 => parts(a, b, data, out),
		_ => ranges(data, out),
	}
}

/// A `&mut` through nested indexes lent as an `rt.Place`, each index assigned again while
/// it lives, and one through a reference assigned another.
fn places(a: u8, b: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut grid = [[0u8; 4]; 4];
	let mut rows = [Row { cells: [0; 4] }, Row { cells: [0; 4] }, Row { cells: [0; 4] }];
	let (mut i, mut j) = (usize::from(a % 4), usize::from(b % 4));
	for &k in data {
		let cell = &mut grid[i][j];
		let row = &mut rows[i % 3].cells[j];
		i = (i + usize::from(k)) % 4;
		j = (j + 1) % 4;
		set(cell, k);
		set(row, k ^ 1);
	}
	out.extend(grid.iter().flatten());
	for row in &rows {
		out.extend(row.cells);
	}
	let mut x = [a; 4];
	let mut y = [b; 4];
	let mut r: &mut [u8] = &mut x;
	let s = &mut r[usize::from(a % 4)];
	r = &mut y;
	set(s, 9);
	r[0] = r[0].wrapping_add(1);
	out.extend(x);
	out.extend(y);
}

/// Shared and mutable borrows of an element, the index assigned again before the read,
/// and kept across iterations.
fn indexed(data: &[u8], out: &mut Vec<u8>) {
	let mut i = 0;
	let mut kept = Vec::new();
	while i < data.len() {
		let r = &data[i];
		i += 1;
		out.push(*r);
		kept.push(r);
	}
	out.extend(kept.into_iter().copied());
	let mut v = data.to_vec();
	let mut i = 0;
	while i < v.len() {
		let r = &mut v[i];
		i += 2;
		*r = r.wrapping_add(i as u8);
	}
	out.extend(v);
}

/// A reborrow through a reference that is then assigned another, shared and mutable, in a
/// loop.
fn reborrows(a: u8, b: u8, data: &[u8], out: &mut Vec<u8>) {
	let (mut x, mut y) = (a, b);
	let mut r: &mut u8 = &mut x;
	let s: &mut u8 = &mut *r;
	r = &mut y;
	*s = s.wrapping_add(1);
	*r = r.wrapping_add(2);
	out.extend([x, y]);
	let mut r: &u8 = &a;
	let mut kept = Vec::new();
	for k in data {
		let s: &u8 = &*r;
		r = k;
		out.extend([*s, *r]);
		kept.push(s);
	}
	out.extend(kept.into_iter().copied());
}

/// A copy of a reference whose source is assigned another, and of an `Option` a range's
/// `next` assigns again.
fn copies(a: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut r: &u8 = &a;
	let mut kept = Vec::new();
	for k in data {
		let s = r;
		r = k;
		kept.push(s);
	}
	out.extend(kept.into_iter().copied());
	let mut range = 0..data.len();
	let mut cur = range.next();
	let some = cur.is_some();
	let copied = cur;
	cur = range.next();
	out.push(u8::from(some));
	out.push(copied.map_or(0xff, |c| c as u8));
	loop {
		let was = cur;
		cur = range.next();
		let Some(w) = was else { break };
		out.push(w as u8);
		out.push(cur.map_or(0xff, |c| c as u8));
	}
}

/// Closures made in a loop that capture a reference to what that iteration reads, called
/// after it: an element, a payload, a field, and a value a loop breaks with.
fn closures(a: u8, b: u8, data: &[u8], out: &mut Vec<u8>) {
	let arr = [a, b, 3, 4, 5, 6, 7, 8];
	let rows = [Row { cells: [a; 4] }, Row { cells: [b; 4] }, Row { cells: [7; 4] }];
	let v = data.to_vec();
	let mut fs: Vec<Box<dyn Fn() -> u8 + '_>> = Vec::new();
	for k in data {
		fs.push(Box::new(move || *k));
	}
	for i in 0..v.len() {
		let r = &v[i];
		fs.push(Box::new(move || r.wrapping_add(1)));
	}
	let mut range = 0..arr.len();
	loop {
		let Some(i) = range.next() else { break };
		let r = &arr[i];
		fs.push(Box::new(move || *r));
	}
	for (i, x) in data.iter().enumerate() {
		fs.push(Box::new(move || x.wrapping_add(i as u8)));
	}
	let mut it = rows.iter();
	while let Some(row) = it.next() {
		let cells = &row.cells;
		fs.push(Box::new(move || cells[1]));
	}
	for k in data {
		let mut n = *k;
		let row: &Row = loop {
			if n % 3 == 0 {
				break &rows[usize::from(*k) % 3];
			}
			n = n.wrapping_add(1);
		};
		fs.push(Box::new(move || row.cells[0] ^ n));
	}
	for f in &fs {
		out.push(f());
	}
}

/// A tuple of references and a number, read by its fields and whole after the variables
/// it was made of are assigned again, and one made in a loop from a variable it assigns.
fn parts(a: u8, b: u8, data: &[u8], out: &mut Vec<u8>) {
	let rows = [Row { cells: [a; 4] }, Row { cells: [b; 4] }];
	let mut s: &str = "ab";
	let mut x: &Row = &rows[0];
	let mut n = a;
	let pair = (s, x, n);
	s = "zzz";
	x = &rows[1];
	n = n.wrapping_add(1);
	out.extend_from_slice(pair.0.as_bytes());
	out.extend([pair.1.cells[0], pair.2, whole(pair)]);
	out.extend_from_slice(s.as_bytes());
	out.push(x.cells[0] ^ n);
	let mut total = 0u8;
	for k in data {
		let p = (n, *k);
		n = n.wrapping_mul(3);
		total = total.wrapping_add(p.0).wrapping_add(p.1);
	}
	out.push(total);
}

/// A range's start read, then the range stepped, once and in a loop.
fn ranges(data: &[u8], out: &mut Vec<u8>) {
	let mut range = 0..data.len();
	let before = range.start;
	let first = range.next();
	out.push(before as u8);
	out.push(first.map_or(0xff, |f| f as u8));
	let mut range = 0..data.len();
	loop {
		let before = range.start;
		let Some(i) = range.next() else { break };
		out.extend([before as u8, i as u8]);
	}
}
