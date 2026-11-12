//! Strong and weak counts of `Arc`s and `Rc`s and their `Weak`s, through every way one is
//! made and dropped: cloned, downgraded, upgraded, made dangling, made holding handles to
//! itself, unwrapped by `into_inner`, and held in a `Vec`, an `Option`, a struct, and a closure, of a value and of a
//! trait object; the values `get_mut` and `make_mut` write, alone or shared, and where
//! `as_ptr` says each value is, an `Arc`'s and a `Weak`'s; and the clones `Option`'s and an iterator's `cloned` make,
//! through each type's `Clone`.

use std::cell::{Cell, RefCell};
use std::rc::{self, Rc};
use std::sync::{self, Arc, Mutex};

/// Logs its id when it drops, so the last strong drop shows where it happens.
struct Noisy {
	id: u8,
	log: Arc<Mutex<Vec<u8>>>,
}

/// A clone is told apart by its id.
impl Clone for Noisy {
	fn clone(&self) -> Self {
		Self {
			id: self.id.wrapping_add(1),
			log: self.log.clone(),
		}
	}
}

impl Drop for Noisy {
	fn drop(&mut self) {
		self.log.lock().unwrap().push(self.id);
	}
}

/// Puts what dropped since the last flush.
fn flush(log: &Mutex<Vec<u8>>, out: &mut Vec<u8>) {
	let mut log = log.lock().unwrap();
	out.extend_from_slice(&log);
	log.clear();
}

/// What a trait object of a `Noisy` reads.
trait Id {
	fn id(&self) -> u8;
}

impl Id for Noisy {
	fn id(&self) -> u8 {
		self.id
	}
}

macro_rules! counts {
	(
		$name:ident,
		$holder:ident,
		$cyclic:ident,
		$node:ident,
		$unique:ident,
		$releasing:ident,
		$cloned:ident,
		$shared:ident,
		$addressed:ident,
		$pointed:ident,
		$entry:ident,
		$Ptr:ident,
		$weak:ident
	) => {
		/// Notes as it drops whether the `Weak` in its slot points at it, as an index keyed
		/// by identity tells its own entry from another's.
		struct $entry {
			id: u8,
			slot: Arc<Mutex<Option<$weak::Weak<$entry>>>>,
			log: Arc<Mutex<Vec<u8>>>,
		}

		impl Drop for $entry {
			fn drop(&mut self) {
				let own = self
					.slot
					.lock()
					.unwrap()
					.as_ref()
					.is_some_and(|owner| std::ptr::eq(owner.as_ptr(), self));
				self.log.lock().unwrap().extend([self.id, u8::from(own)]);
			}
		}

		/// Each byte an operation on pools of strong and weak handles of entries and on the
		/// slot their drops read, the rest of it the handle it picks: where `Weak::as_ptr`
		/// points, against `as_ptr` and references of live and dropped values and other
		/// `Weak`s, `Weak::new`'s included; and of a byte JS copies, which `make_mut` moves
		/// away from its `Weak`, and of an array, read through it.
		pub fn $pointed(data: &[u8], out: &mut Vec<u8>) {
			let log = Arc::new(Mutex::new(Vec::new()));
			let slot: Arc<Mutex<Option<$weak::Weak<$entry>>>> = Arc::default();
			let entry = |id: u8| {
				$Ptr::new($entry {
					id,
					slot: slot.clone(),
					log: log.clone(),
				})
			};
			let mut strong = vec![entry(0)];
			let mut weak = vec![$weak::Weak::new()];
			// Where each dropped value was, which a `Weak` keeps allocated so no other value
			// is put there.
			let mut dropped: Vec<(*const $entry, $weak::Weak<$entry>)> = Vec::new();
			let mut number = $Ptr::new(1u8);
			let mut kept = $Ptr::downgrade(&number);
			let pair = $Ptr::new([2u8, 3]);
			let pair_weak = $Ptr::downgrade(&pair);
			let units = ($weak::Weak::<()>::new(), $weak::Weak::<[u8; 2]>::new());
			for &op in data {
				let pick = usize::from(op >> 3);
				match op % 8 {
					0 => strong.push(entry(op)),
					1 => weak.push($Ptr::downgrade(&strong[pick % strong.len()])),
					2 => weak.push($weak::Weak::new()),
					3 => *slot.lock().unwrap() = Some(weak[pick % weak.len()].clone()),
					4 if strong.len() > 1 => {
						let gone = strong.swap_remove(pick % strong.len());
						let (at, kept) = ($Ptr::as_ptr(&gone), $Ptr::downgrade(&gone));
						// The last strong one's value, moved out of the allocation its `Weak`
						// keeps, is at an address of its own as it drops.
						if op & 0x80 != 0
							&& let Some(value) = $Ptr::into_inner(gone)
						{
							out.extend([
								u8::from(std::ptr::eq(kept.as_ptr(), &value)),
								u8::from(std::ptr::eq(at, &value)),
								u8::from(kept.upgrade().is_none()),
							]);
						}
						dropped.push((at, kept));
					}
					4 => *slot.lock().unwrap() = None,
					5 => {
						let owner = &weak[pick % weak.len()];
						let p = owner.as_ptr();
						out.push(u8::from(std::ptr::eq(p, $weak::Weak::<$entry>::new().as_ptr())));
						out.extend(strong.iter().map(|s| u8::from(std::ptr::eq(p, $Ptr::as_ptr(s)))));
						out.extend(strong.iter().map(|s| u8::from(std::ptr::eq(p, &**s))));
						out.extend(dropped.iter().map(|(at, _)| u8::from(std::ptr::eq(p, *at))));
						out.extend(weak.iter().map(|other| u8::from(std::ptr::eq(p, other.as_ptr()))));
						out.push(u8::from(p.is_null()));
						out.push(owner.upgrade().map_or(0xfe, |e| e.id));
					}
					6 => {
						if op & 0x80 != 0 {
							kept = $Ptr::downgrade(&number);
						}
						*$Ptr::make_mut(&mut number) ^= op;
						let at = kept.as_ptr().cast::<()>();
						out.push(u8::from(std::ptr::eq(at, $Ptr::as_ptr(&number).cast::<()>())));
						let none = $weak::Weak::<u16>::new().as_ptr().cast::<()>();
						out.push(u8::from(std::ptr::eq(at, none)));
						out.push(u8::from(std::ptr::eq(
							$weak::Weak::<u8>::new().as_ptr().cast(),
							none,
						)));
						if kept.strong_count() > 0 {
							out.push(unsafe { *kept.as_ptr() });
						}
						out.push(*number);
					}
					_ => {
						out.extend_from_slice(unsafe { &*pair_weak.as_ptr() });
						out.push(pair[0]);
						// `Weak::new`'s, at one address whatever it points at.
						let none = $weak::Weak::<u8>::new().as_ptr().cast::<()>();
						out.push(u8::from(std::ptr::eq(units.0.as_ptr(), none)));
						let past = units.1.as_ptr().cast::<u8>().wrapping_add(usize::from(op >> 7));
						out.push(u8::from(past.is_null()));
					}
				}
				flush(&log, out);
			}
			*slot.lock().unwrap() = None;
			drop((strong, weak, dropped));
			flush(&log, out);
		}

		/// Holds a handle that doesn't keep its value.
		struct $holder<T> {
			weak: $weak::Weak<T>,
		}

		/// Holds handles to itself, which `new_cyclic` gives it before it exists, and what
		/// the first read then.
		struct $node {
			this: $weak::Weak<$node>,
			more: Vec<$weak::Weak<$node>>,
			noisy: Noisy,
			seen: [u8; 3],
		}

		impl $node {
			fn new(this: &$weak::Weak<$node>, noisy: Noisy, more: u8) -> Self {
				Self {
					this: this.clone(),
					more: (0..more).map(|_| this.clone()).collect(),
					noisy,
					seen: [
						this.upgrade().is_none() as u8,
						$weak::Weak::strong_count(this) as u8,
						$weak::Weak::weak_count(this) as u8,
					],
				}
			}
		}

		/// Each byte a value made by `new_cyclic`, from a closure that moves what it owns,
		/// reads it, reads it as an `Fn`, or owns nothing that drops, holding up to 3 more
		/// handles to itself; then kept, dropped, or dropped with a handle outside. Each puts what it read and what
		/// dropped.
		pub fn $cyclic(data: &[u8], out: &mut Vec<u8>) {
			let log = Arc::new(Mutex::new(Vec::new()));
			let mut kept: Vec<$Ptr<$node>> = Vec::new();
			let mut outside: Vec<$weak::Weak<$node>> = Vec::new();
			for &op in data {
				let noisy = Noisy {
					id: op,
					log: log.clone(),
				};
				let more = (op >> 2) % 4;
				let node = match op % 4 {
					0 => $Ptr::new_cyclic(move |this| $node::new(this, noisy, more)),
					// Its capture drops once it is called.
					1 => $Ptr::new_cyclic(move |this| {
						let copy = Noisy {
							id: noisy.id ^ 0x80,
							log: noisy.log.clone(),
						};
						$node::new(this, copy, more)
					}),
					// An `Fn`, made before the call, so `new_cyclic` drops it after.
					2 => {
						let make = move |this: &$weak::Weak<$node>| {
							let copy = Noisy {
								id: noisy.id ^ 0x40,
								log: noisy.log.clone(),
							};
							$node::new(this, copy, more)
						};
						$Ptr::new_cyclic(make)
					}
					_ => $Ptr::new_cyclic(|this| {
						let fresh = Noisy {
							id: 0xee,
							log: log.clone(),
						};
						$node::new(this, fresh, more)
					}),
				};
				out.extend(node.seen);
				out.push($Ptr::strong_count(&node) as u8);
				out.push($Ptr::weak_count(&node) as u8);
				out.push(node.more.len() as u8);
				out.push(node.this.upgrade().map_or(0xfd, |n| n.noisy.id));
				out.push($weak::Weak::ptr_eq(&node.this, &$Ptr::downgrade(&node)) as u8);
				out.push($Ptr::weak_count(&node) as u8);
				match op >> 6 {
					0 => kept.push(node),
					1 => {
						outside.push($Ptr::downgrade(&node));
						drop(node);
					}
					_ => drop(node),
				}
				flush(&log, out);
			}
			drop(kept);
			out.extend(outside.iter().map(|w| $weak::Weak::strong_count(w) as u8));
			flush(&log, out);
		}

		/// Each byte an operation on pools of strong and weak handles, the rest of it the
		/// handle it picks; each puts what it read.
		pub fn $name(data: &[u8], out: &mut Vec<u8>) {
			let log = Arc::new(Mutex::new(Vec::new()));
			let mut strong: Vec<$Ptr<Noisy>> = Vec::new();
			let mut weak: Vec<$weak::Weak<Noisy>> = Vec::new();
			for &op in data {
				let pick = usize::from(op >> 4);
				match op % 12 {
					0 => strong.push($Ptr::new(Noisy {
						id: op,
						log: log.clone(),
					})),
					1 if !strong.is_empty() => {
						let again = strong[pick % strong.len()].clone();
						strong.push(again);
					}
					2 if !strong.is_empty() && op & 0x80 == 0 => drop(strong.swap_remove(pick % strong.len())),
					// The value from the last strong handle alone, which its `Weak`s no longer
					// upgrade after; another's share only drops.
					2 if !strong.is_empty() => match $Ptr::into_inner(strong.swap_remove(pick % strong.len())) {
						Some(noisy) => out.push(noisy.id),
						None => out.push(0xfb),
					},
					3 if !strong.is_empty() => weak.push($Ptr::downgrade(&strong[pick % strong.len()])),
					4 if !weak.is_empty() => {
						let again = weak[pick % weak.len()].clone();
						weak.push(again);
					}
					5 if !weak.is_empty() => drop(weak.swap_remove(pick % weak.len())),
					6 if !weak.is_empty() => match weak[pick % weak.len()].upgrade() {
						Some(up) => {
							out.push(up.id);
							strong.push(up);
						}
						None => out.push(0xfe),
					},
					7 => weak.push($weak::Weak::new()),
					// A weak handle in an `Option`, a struct, and a closure, each dropped
					// before the next byte.
					8 if !strong.is_empty() => {
						let picked = &strong[pick % strong.len()];
						let held = Some($Ptr::downgrade(picked));
						let kept = $holder {
							weak: $Ptr::downgrade(picked),
						};
						let captured = $Ptr::downgrade(picked);
						let read = move || captured.upgrade().map_or(0xfd, |s| s.id);
						out.push($Ptr::weak_count(picked) as u8);
						out.push(read());
						drop(held);
						out.push($Ptr::weak_count(picked) as u8);
						drop(kept);
						drop(read);
						out.push($Ptr::weak_count(picked) as u8);
					}
					// The value no other handle shares, which `get_mut` lends.
					9 if !strong.is_empty() => {
						let len = strong.len();
						match $Ptr::get_mut(&mut strong[pick % len]) {
							Some(noisy) => {
								noisy.id ^= 0x10;
								out.push(noisy.id);
							}
							None => out.push(0xfc),
						}
					}
					// A clone while another strong handle shares it, whose share drops, or the
					// value moved away from the weak ones, which dangle.
					10 if !strong.is_empty() => {
						let len = strong.len();
						let picked = &mut strong[pick % len];
						let noisy = $Ptr::make_mut(picked);
						noisy.id ^= 0x20;
						out.push(noisy.id);
						out.push($Ptr::strong_count(picked) as u8);
						out.push($Ptr::weak_count(picked) as u8);
					}
					// Every count, of each strong handle and each weak one.
					_ => {
						for s in &strong {
							out.push($Ptr::strong_count(s) as u8);
							out.push($Ptr::weak_count(s) as u8);
						}
						for w in &weak {
							out.push($weak::Weak::strong_count(w) as u8);
							out.push($weak::Weak::weak_count(w) as u8);
						}
					}
				}
			}
			// Trait objects of the last strong handle share its counts.
			if let Some(last) = strong.last() {
				let object: $Ptr<dyn Id> = last.clone();
				let weak_object = $Ptr::downgrade(&object);
				out.push($Ptr::weak_count(last) as u8);
				out.push($weak::Weak::weak_count(&weak_object) as u8);
				out.push(weak_object.upgrade().map_or(0xfd, |o| o.id()));
				drop(object);
				out.push($weak::Weak::strong_count(&weak_object) as u8);
			}
			// One pinned, and one read through an `Option`.
			drop($Ptr::pin(Noisy {
				id: 0xe0,
				log: log.clone(),
			}));
			out.push(
				strong
					.first()
					.map($Ptr::clone)
					.as_deref()
					.map_or(0xfd, |noisy| noisy.id),
			);
			drop(strong);
			out.extend(weak.iter().map(|w| w.upgrade().is_none() as u8));
			out.extend(weak.iter().map(|w| $weak::Weak::weak_count(w) as u8));
			drop(weak);
			out.extend_from_slice(&log.lock().unwrap());
		}

		/// Releases the other handles to it when cloned, as its id says it was, and logs its
		/// id when it drops.
		struct $releasing {
			id: u8,
			others: Rc<RefCell<Vec<$Ptr<$releasing>>>>,
			log: Rc<RefCell<Vec<u8>>>,
		}

		impl Clone for $releasing {
			fn clone(&self) -> Self {
				self.others.borrow_mut().clear();
				Self {
					id: self.id.wrapping_add(1),
					others: self.others.clone(),
					log: self.log.clone(),
				}
			}
		}

		impl Drop for $releasing {
			fn drop(&mut self) {
				self.log.borrow_mut().push(self.id);
			}
		}

		/// Each byte an operation on pools of strong and weak handles of a byte JS copies,
		/// the rest of it the handle it picks; each puts what it read and the picked
		/// handle's counts.
		pub fn $unique(data: &[u8], out: &mut Vec<u8>) {
			let get_mut: fn(&mut $Ptr<u8>) -> Option<&mut u8> = $Ptr::get_mut;
			let make_mut: fn(&mut $Ptr<u8>) -> &mut u8 = $Ptr::make_mut;
			let mut strong: Vec<$Ptr<u8>> = vec![$Ptr::new(0)];
			let mut weak: Vec<$weak::Weak<u8>> = Vec::new();
			for &op in data {
				let pick = usize::from(op >> 3) % strong.len();
				match op % 7 {
					0 => strong.push($Ptr::new(op)),
					1 => {
						let again = strong[pick].clone();
						strong.push(again);
					}
					2 => weak.push($Ptr::downgrade(&strong[pick])),
					3 if strong.len() > 1 && op & 0x80 == 0 => drop(strong.swap_remove(pick)),
					3 if strong.len() > 1 => out.push($Ptr::into_inner(strong.swap_remove(pick)).unwrap_or(0xfb)),
					3 => weak.clear(),
					// Called directly, and through a function pointer.
					4 => match if op & 0x80 == 0 {
						$Ptr::get_mut(&mut strong[pick])
					} else {
						get_mut(&mut strong[pick])
					} {
						Some(value) => {
							*value = value.wrapping_add(op);
							out.push(*value);
						}
						None => out.push(0xfc),
					},
					5 => {
						let value = if op & 0x80 == 0 {
							$Ptr::make_mut(&mut strong[pick])
						} else {
							make_mut(&mut strong[pick])
						};
						*value ^= op;
						out.push(*value);
					}
					_ => {
						let pinned = $Ptr::pin(op);
						let held = Some($Ptr::clone(&strong[pick]));
						out.extend_from_slice(&[*pinned, held.as_deref().copied().unwrap_or(0xfb)]);
					}
				}
				let picked = &strong[pick % strong.len()];
				out.extend_from_slice(&[
					**picked,
					$Ptr::strong_count(picked) as u8,
					$Ptr::weak_count(picked) as u8,
				]);
			}
			for w in &weak {
				out.push(w.upgrade().map_or(0xfd, |s| *s));
				out.push($weak::Weak::strong_count(w) as u8);
				out.push($weak::Weak::weak_count(w) as u8);
			}
			out.extend(strong.iter().map(|s| **s));
			// A clone that releases the other handles leaves `make_mut` the last, which drops.
			let others = Rc::new(RefCell::new(Vec::new()));
			let log = Rc::new(RefCell::new(Vec::new()));
			let mut first = $Ptr::new($releasing {
				id: data.len() as u8,
				others: others.clone(),
				log: log.clone(),
			});
			let shared = data.first().map_or(1, |b| b % 3 + 1);
			others
				.borrow_mut()
				.extend((0..shared).map(|_| $Ptr::clone(&first)));
			$Ptr::make_mut(&mut first).id ^= 0x40;
			out.extend_from_slice(&[first.id, $Ptr::strong_count(&first) as u8]);
			out.extend_from_slice(&log.borrow());
			drop(first);
			out.extend_from_slice(&log.borrow());
		}

		/// Holds a handle, and clones it with a derived `Clone`.
		#[derive(Clone)]
		struct $shared {
			noisy: $Ptr<Noisy>,
			tag: u8,
		}

		/// Each byte clones what `Option`'s or an iterator's `cloned` lends, the rest of it
		/// the one it picks: a handle, through `&` and `&mut`, a struct holding one, a
		/// `Noisy` whose own `Clone` runs, or a `Vec` of handles. Each clone is kept or
		/// dropped, and each byte puts the picked handle's count and what dropped.
		pub fn $cloned(data: &[u8], out: &mut Vec<u8>) {
			let log = Arc::new(Mutex::new(Vec::new()));
			let mut handles: Vec<$Ptr<Noisy>> = (0..3)
				.map(|id| {
					$Ptr::new(Noisy {
						id: id * 0x10,
						log: log.clone(),
					})
				})
				.collect();
			let mut kept: Vec<$Ptr<Noisy>> = Vec::new();
			let mut shared: Vec<$shared> = Vec::new();
			let mut noisy: Vec<Noisy> = Vec::new();
			let mut lists: Vec<Vec<$Ptr<Noisy>>> = Vec::new();
			for &op in data {
				let pick = usize::from(op >> 4);
				let handle = &handles[pick % handles.len()];
				let keep = op & 0x08 != 0;
				match op % 8 {
					0 => {
						let again = handles.get(pick % 4).cloned();
						out.push(again.as_ref().map_or(0xfd, |h| h.id));
						kept.extend(again.filter(|_| keep));
					}
					1 => {
						let len = handles.len();
						let again = handles.get_mut(pick % len).cloned();
						kept.extend(again.filter(|_| keep));
					}
					2 => {
						let made = $shared {
							noisy: handle.clone(),
							tag: op,
						};
						let one = [made].first().cloned();
						out.push(one.as_ref().map_or(0xfd, |s| s.tag ^ s.noisy.id));
						shared.extend(one.filter(|_| keep));
					}
					3 => {
						let made = Noisy {
							id: op,
							log: log.clone(),
						};
						let one = Some(&made).cloned();
						out.push(one.as_ref().map_or(0xfd, |n| n.id));
						noisy.extend(one.filter(|_| keep));
					}
					4 => {
						let all: Vec<$Ptr<Noisy>> = handles.iter().cloned().collect();
						out.push($Ptr::strong_count(handle) as u8);
						if keep {
							lists.push(all);
						}
					}
					5 => {
						let again: Vec<$shared> = shared.iter().cloned().collect();
						let ids: Vec<Noisy> = noisy.iter().cloned().collect();
						out.extend(ids.iter().map(|n| n.id));
						if keep {
							shared.extend(again);
						}
					}
					6 => {
						let list = lists.first().cloned();
						out.push(list.as_ref().map_or(0xfd, |l| l.len() as u8));
						lists.extend(list.filter(|_| keep));
					}
					// Drop the oldest of what is kept, or a handle itself.
					_ => match op >> 6 {
						0 if !kept.is_empty() => drop(kept.remove(0)),
						1 if !shared.is_empty() => drop(shared.remove(0)),
						2 if !noisy.is_empty() => drop(noisy.remove(0)),
						3 if handles.len() > 1 => drop(handles.remove(pick % handles.len())),
						_ => drop(lists.pop()),
					},
				}
				let handle = &handles[pick % handles.len()];
				out.push($Ptr::strong_count(handle) as u8);
				flush(&log, out);
			}
			out.extend(handles.iter().map(|h| $Ptr::strong_count(h) as u8));
			drop((handles, kept, shared, noisy, lists));
			flush(&log, out);
		}

		/// Each byte an operation on pools of strong and weak handles of a pair kept as an
		/// object, the rest of it the handle it picks, and on one of bytes, of a byte JS
		/// copies, of an `Option` of a pair, of a pair nothing writes, and of `()`: `make_mut`
		/// keeps the allocation `as_ptr` points at only when no other handle, strong or weak,
		/// shares it, and a read through `as_ptr` sees each write.
		pub fn $addressed(data: &[u8], out: &mut Vec<u8>) {
			let mut strong: Vec<$Ptr<(u8, u8)>> = vec![$Ptr::new((0, 1))];
			let mut weak: Vec<$weak::Weak<(u8, u8)>> = Vec::new();
			let mut bytes = $Ptr::new([2u8, 3]);
			let mut number = $Ptr::new(4u8);
			let mut maybe = $Ptr::new(Some((5u8, 6u8)));
			let mut unread: $Ptr<(u16, u8)> = $Ptr::new((7, 8));
			let mut unit = $Ptr::new(());
			for &op in data {
				let pick = usize::from(op >> 3) % strong.len();
				match op % 6 {
					0 => strong.push($Ptr::new((op, 0))),
					1 => {
						let again = strong[pick].clone();
						strong.push(again);
					}
					2 => weak.push($Ptr::downgrade(&strong[pick])),
					3 if strong.len() > 1 && op & 0x80 == 0 => drop(strong.swap_remove(pick)),
					3 => weak.clear(),
					4 => {
						let before = $Ptr::as_ptr(&strong[pick]);
						$Ptr::make_mut(&mut strong[pick]).1 ^= op;
						let after = $Ptr::as_ptr(&strong[pick]);
						out.push(u8::from(std::ptr::addr_eq(before, after)));
						out.push(u8::from(std::ptr::eq(after, &*strong[pick])));
						out.push(unsafe { (*after).1 });
					}
					_ => {
						let shadow = (op & 0x80 != 0).then(|| $Ptr::clone(&bytes));
						let before = $Ptr::as_ptr(&bytes);
						$Ptr::make_mut(&mut bytes)[0] ^= op;
						let after = $Ptr::as_ptr(&bytes);
						out.push(u8::from(std::ptr::addr_eq(before, after)));
						out.extend_from_slice(unsafe { &*after });
						drop(shadow);
						let at = $Ptr::as_ptr(&number);
						if let Some(n) = $Ptr::get_mut(&mut number) {
							*n ^= op;
						}
						out.push(unsafe { *at });
						let kept = (op & 0x40 != 0).then(|| $Ptr::downgrade(&maybe));
						if let Some(pair) = $Ptr::make_mut(&mut maybe).as_mut() {
							pair.0 ^= op;
						}
						out.push(maybe.map_or(0xfd, |(a, _)| a));
						drop(kept);
						// Moved or cloned by a `make_mut` that never writes it, of a type nothing
						// writes.
						let kept = (op & 0x40 != 0).then(|| $Ptr::downgrade(&unread));
						let shadow = (op & 0x20 != 0).then(|| $Ptr::clone(&unread));
						let before = $Ptr::as_ptr(&unread);
						let pair = *$Ptr::make_mut(&mut unread);
						out.extend([u8::from(std::ptr::addr_eq(before, $Ptr::as_ptr(&unread))), pair.1]);
						drop((kept, shadow));
						let other = $Ptr::new(());
						let before = $Ptr::as_ptr(&unit);
						let shadow = (op & 0x80 != 0).then(|| $Ptr::clone(&unit));
						$Ptr::make_mut(&mut unit);
						out.extend([
							u8::from(std::ptr::eq(before, $Ptr::as_ptr(&unit))),
							u8::from(std::ptr::eq($Ptr::as_ptr(&other), $Ptr::as_ptr(&unit))),
						]);
						drop(shadow);
					}
				}
				let picked = &strong[pick % strong.len()];
				out.extend_from_slice(&[picked.0, picked.1, $Ptr::strong_count(picked) as u8]);
			}
		}
	};
}

counts!(
	arcs,
	ArcHolder,
	cyclic_arcs,
	ArcNode,
	unique_arcs,
	ArcReleasing,
	cloned_arcs,
	ArcShared,
	addressed_arcs,
	pointed_arcs,
	ArcEntry,
	Arc,
	sync
);
counts!(
	rcs,
	RcHolder,
	cyclic_rcs,
	RcNode,
	unique_rcs,
	RcReleasing,
	cloned_rcs,
	RcShared,
	addressed_rcs,
	pointed_rcs,
	RcEntry,
	Rc,
	rc
);

/// A `Copy` value whose `Clone`, written by hand, counts each clone.
#[derive(Copy)]
struct Counted<'a> {
	id: u8,
	clones: &'a Cell<u8>,
}

impl Clone for Counted<'_> {
	fn clone(&self) -> Self {
		self.clones.set(self.clones.get().wrapping_add(1));
		Counted {
			id: self.id.wrapping_add(1),
			clones: self.clones,
		}
	}
}

/// A `Copy` value with a derived `Clone`.
#[derive(Clone, Copy)]
struct Twin(u8, u8);

/// Each byte copies or clones, through `Option` and an iterator, a byte, a `Twin`, and a
/// `Counted`, and through `Option<&mut>` a byte and an `Option`; each puts what it made
/// and how many clones `Counted` counted.
pub fn copies(data: &[u8], out: &mut Vec<u8>) {
	let clones = Cell::new(0);
	let counted: Vec<Counted<'_>> = data.iter().map(|&id| Counted { id, clones: &clones }).collect();
	let twins: Vec<Twin> = data.iter().map(|&b| Twin(b, !b)).collect();
	let mut bytes = data.to_vec();
	let mut maybes: Vec<Option<u8>> = data.iter().map(|&b| (b % 3 != 0).then_some(b)).collect();
	for (i, &op) in data.iter().enumerate() {
		let pick = usize::from(op >> 3) % data.len();
		match op % 8 {
			6 => {
				let byte = bytes.get_mut(pick).cloned();
				out.push(bytes.get_mut(i + pick).copied().or(byte).unwrap_or(0xfd));
				if let Some(b) = bytes.get_mut(i) {
					*b ^= op;
				}
			}
			7 => {
				let maybe = maybes.get_mut(pick).copied();
				out.push(maybe.map_or(0xfd, |m| m.unwrap_or(0xfc)));
				let again = maybes.get_mut(i + pick).cloned();
				out.push(again.map_or(0xfd, |m| m.unwrap_or(0xfc)));
				if let Some(m) = maybes.get_mut(pick) {
					*m = m.map_or(Some(op), |_| None);
				}
			}
			0 => {
				out.push(data.get(pick).cloned().unwrap_or(0xfd));
				out.push(data.get(i).copied().unwrap_or(0xfd));
			}
			1 => {
				let twin = twins.get(pick).cloned().unwrap_or(Twin(0, 0));
				out.extend([twin.0, twin.1]);
			}
			2 => out.push(counted.get(pick).cloned().map_or(0xfd, |c| c.id)),
			3 => out.push(counted.get(pick).copied().map_or(0xfd, |c| c.id)),
			4 => out.extend(counted.iter().cloned().map(|c| c.id)),
			_ => {
				out.extend(counted.iter().copied().map(|c| c.id));
				out.extend(twins.iter().cloned().map(|t| t.0 ^ t.1));
			}
		}
		out.push(clones.get());
	}
}
