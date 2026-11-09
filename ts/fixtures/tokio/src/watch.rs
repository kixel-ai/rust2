//! A watch channel driven by a script of steps: tasks that wait for a change, for a value,
//! or for every receiver to drop, beside sends, borrows, and version marks, logging each
//! value, error, count, and drop, and which tasks each step woke.

use std::cell::RefCell;
use std::rc::Rc;

use tokio::sync::watch;

use crate::exec::Executor;
use crate::log::{Item, note};

/// A receiver's slot, which a task takes it from and puts it back into once done.
type Slot = Rc<RefCell<Option<watch::Receiver<Item>>>>;

struct State {
	exec: Executor,
	senders: Vec<Option<watch::Sender<Item>>>,
	receivers: Vec<Slot>,
	next: u8,
}

impl State {
	fn item(&mut self) -> Item {
		self.next = self.next.wrapping_add(1);
		Item(self.next)
	}

	fn sender(&self, arg: u8) -> Option<&watch::Sender<Item>> {
		let live: Vec<&watch::Sender<Item>> = self.senders.iter().flatten().collect();
		(!live.is_empty()).then(|| live[usize::from(arg) % live.len()])
	}

	fn slot(&self, arg: u8) -> Option<Slot> {
		let live: Vec<&Slot> = self.receivers.iter().filter(|s| s.borrow().is_some()).collect();
		(!live.is_empty()).then(|| live[usize::from(arg) % live.len()].clone())
	}
}

/// The channel, then its script.
pub fn script(data: &[u8], out: &mut String) {
	let (tx, rx) = watch::channel(Item(0));
	let mut s = State {
		exec: Executor::default(),
		senders: vec![Some(tx)],
		receivers: vec![Rc::new(RefCell::new(Some(rx)))],
		next: 0,
	};
	for step in data.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		step_run(&mut s, op % 16, arg);
		let woken = s.exec.woken();
		if !woken.is_empty() {
			note(format!("woke {woken}"));
		}
	}
	note("end".into());
	s.exec.clear();
	s.senders.clear();
	note("senders dropped".into());
	for slot in &s.receivers {
		drop(slot.borrow_mut().take());
	}
	note("receivers dropped".into());
	out.push_str(&crate::log::take());
}

fn step_run(s: &mut State, op: u8, arg: u8) {
	match op {
		// Wait for a change.
		0 => {
			if let Some(slot) = s.slot(arg) {
				let mut rx = slot.borrow_mut().take().expect("live");
				s.exec.spawn(async move {
					let changed = rx.changed().await;
					let line = format!("changed {changed:?} to {}", *rx.borrow_and_update());
					*slot.borrow_mut() = Some(rx);
					line
				});
			}
		}
		// Wait for a value divisible by `arg % 3 + 1`, by a closure that owns what it logs,
		// which drops with the future.
		1 => {
			if let Some(slot) = s.slot(arg) {
				let mut rx = slot.borrow_mut().take().expect("live");
				let by = arg % 3 + 1;
				let seen = Item(100 + by);
				s.exec.spawn(async move {
					let line = match rx
						.wait_for(move |v| {
							note(format!("wait_for sees {v} beside {seen}"));
							v.0 % by == 0
						})
						.await
					{
						Ok(r) => format!("wait_for {} changed {}", *r, r.has_changed()),
						Err(e) => format!("wait_for {e} {e:?}"),
					};
					*slot.borrow_mut() = Some(rx);
					line
				});
			}
		}
		// Wait for every receiver to drop.
		2 => {
			if let Some(tx) = s.sender(arg).cloned() {
				s.exec.spawn(async move {
					tx.closed().await;
					"closed".into()
				});
			}
		}
		3 => {
			let item = s.item();
			if let Some(tx) = s.sender(arg) {
				match tx.send(item) {
					Ok(()) => note("sent".into()),
					Err(e) => note(format!("send {e} {e:?} {}", e.0)),
				}
			}
		}
		4 => {
			let item = s.item();
			if let Some(tx) = s.sender(arg) {
				let old = tx.send_replace(item);
				note(format!("replaced {old}"));
			}
		}
		5 => {
			if let Some(tx) = s.sender(arg) {
				let owned = Item(200);
				tx.send_modify(move |v| {
					v.0 = v.0.wrapping_add(owned.0);
				});
			}
		}
		6 => {
			if let Some(tx) = s.sender(arg) {
				let modified = tx.send_if_modified(|v| {
					if arg & 0x10 == 0 {
						return false;
					}
					*v = Item(v.0 / 2);
					true
				});
				note(format!("modified {modified}"));
			}
		}
		// Borrow, and maybe update.
		7 => {
			if let Some(slot) = s.slot(arg) {
				let mut slot = slot.borrow_mut();
				let rx = slot.as_mut().expect("live");
				let r = if arg & 0x10 == 0 {
					rx.borrow()
				} else {
					rx.borrow_and_update()
				};
				note(format!("borrow {} changed {} {r:?}", *r, r.has_changed()));
			}
		}
		// Read and set what a receiver saw.
		8 => {
			if let Some(slot) = s.slot(arg) {
				let mut slot = slot.borrow_mut();
				let rx = slot.as_mut().expect("live");
				note(format!("has_changed {:?}", rx.has_changed()));
				match arg % 3 {
					0 => rx.mark_changed(),
					1 => rx.mark_unchanged(),
					_ => {}
				}
				note(format!(
					"has_changed {:?} closed {} {rx:?}",
					rx.has_changed(),
					rx.is_closed()
				));
			}
		}
		// Subscribe, or clone a receiver.
		9 => {
			let rx = match arg & 1 {
				0 => s.sender(arg).map(|tx| tx.subscribe()),
				_ => s.slot(arg).and_then(|slot| slot.borrow().clone()),
			};
			if let Some(rx) = rx {
				s.receivers.push(Rc::new(RefCell::new(Some(rx))));
			}
		}
		10 => {
			if let Some(slot) = s.slot(arg) {
				note("drop receiver".into());
				drop(slot.borrow_mut().take());
			}
		}
		11 => match arg & 1 {
			0 => {
				if let Some(tx) = s.sender(arg).cloned() {
					s.senders.push(Some(tx));
				}
			}
			_ => {
				if let Some(i) = s.senders.iter().position(Option::is_some) {
					note("drop sender".into());
					s.senders[i] = None;
				}
			}
		},
		12 => s.exec.cancel(arg),
		13 => s.exec.run(),
		14 => {
			if let Some(tx) = s.sender(arg) {
				let same = s
					.sender(arg.wrapping_add(1))
					.is_some_and(|other| other.same_channel(tx));
				note(format!(
					"sender closed {} receivers {} senders {} same {same} {:?} {tx:?}",
					tx.is_closed(),
					tx.receiver_count(),
					tx.sender_count(),
					*tx.borrow()
				));
			}
		}
		_ => {
			if let (Some(a), Some(b)) = (s.slot(arg), s.slot(arg.wrapping_add(1))) {
				let same = match (a.borrow().as_ref(), b.borrow().as_ref()) {
					(Some(a), Some(b)) => a.same_channel(b),
					_ => true,
				};
				note(format!("receivers same {same}"));
			}
		}
	}
}

/// A watch of a value JS copies, which `send_modify` writes in place, and a sender made
/// alone and by `Default`, whose receivers dropped.
pub fn copied(data: &[u8], out: &mut String) {
	let (tx, mut rx) = watch::channel(0u64);
	for &b in data {
		tx.send_modify(|v| *v = v.wrapping_mul(31).wrapping_add(u64::from(b)));
		if b & 1 == 0 {
			let changed = tx.send_if_modified(|v| {
				*v += 1;
				b & 2 == 0
			});
			note(format!("modified {changed}"));
		}
	}
	let seen = *rx.borrow_and_update();
	note(format!("{seen} {:?} {rx:?}", rx.has_changed()));
	let alone = watch::Sender::new(5u8);
	note(format!("{alone:?} {:?}", alone.send(6)));
	let made: watch::Sender<String> = watch::Sender::default();
	note(format!("{made:?} {}", made.is_closed()));
	drop(tx);
	note(format!("{:?} {rx:?}", rx.has_changed()));
	out.push_str(&crate::log::take());
}
