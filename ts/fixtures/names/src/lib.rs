//! Names that stay as translation reaches more: `stable_names` translates `run`, then `added`
//! and `run`, and checks that every line of the first stays, in order.

mod bytes;
mod dyns;
mod shapes;
mod tally;
mod wrap;

use shapes::Shape;
use wrap::{Pair, Wrap};

/// One instance of each generic item it reaches.
pub fn run(data: &[u8]) -> Vec<u8> {
	let first = data.first().copied().unwrap_or(0);
	let pair = Wrap::new(Pair(first, 1)).into_inner();
	let mut out = vec![Wrap::new(pair).map(|p| p.0).into_inner()];
	out.push(data.len().partial_cmp(&4).map_or(9, |o| o as i8 as u8));
	out.extend_from_slice(format!("{}", Wrap::new(pair)).as_bytes());
	let shape = if first == 0 { Shape::Dot } else { Shape::Line };
	out.push(shape.sides());
	out.extend_from_slice(&tally::run(data).to_be_bytes());
	out.extend_from_slice(bytes::read(data).unwrap_or(&[]));
	out
}

/// Another instance of what `run` reaches one of, each sorting before it: a generic method's,
/// a closure's, an impl's of another trait, a std function's that merges with `run`'s, and an
/// impl's over a deeper `&mut`, and a function's that shares the slot of `Ok` parts of one
/// `run` reaches. Its trait object's module shares its name with a type's this module
/// imports, and sorts first.
pub fn added(out: &mut Vec<u8>) {
	out.push(Wrap::new(1u8).into_inner());
	out.push(1u8.partial_cmp(&2).map_or(7, |o| o as i8 as u8));
	let mapped = Wrap::new(2u8).map(|b| Pair(b, 0));
	out.extend_from_slice(format!("{mapped:?}").as_bytes());
	let shape: &dyn dyns::Shape = &3u32;
	out.push(shape.sides());
	out.extend_from_slice(&tally::deeper(4).to_be_bytes());
	out.extend_from_slice(&bytes::lead(out).unwrap_or(&[]).to_vec());
}
