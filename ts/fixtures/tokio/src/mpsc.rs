//! An mpsc channel, bounded or unbounded, driven by a script of steps: tasks that receive,
//! send, reserve, and wait for it to close, beside the steps that need no task, logging
//! each value, error, count, and drop, and which tasks each step woke.

use std::cell::RefCell;
use std::future::poll_fn;
use std::rc::Rc;

use tokio::sync::mpsc;

use crate::exec::Executor;
use crate::log::{Item, note};

/// A sender of either kind.
#[derive(Clone)]
enum Tx {
	Bounded(mpsc::Sender<Item>),
	Unbounded(mpsc::UnboundedSender<Item>),
}

/// A weak sender of either kind.
#[derive(Clone)]
enum Weak {
	Bounded(mpsc::WeakSender<Item>),
	Unbounded(mpsc::WeakUnboundedSender<Item>),
}

/// The receiver of either kind.
enum Rx {
	Bounded(mpsc::Receiver<Item>),
	Unbounded(mpsc::UnboundedReceiver<Item>),
}

impl Rx {
	async fn recv(&mut self) -> Option<Item> {
		match self {
			Rx::Bounded(rx) => rx.recv().await,
			Rx::Unbounded(rx) => rx.recv().await,
		}
	}
}

/// What the steps share. The receiver is in its slot while no task holds it.
struct State {
	exec: Executor,
	senders: Vec<Option<Tx>>,
	weaks: Vec<Option<Weak>>,
	rx: Rc<RefCell<Option<Rx>>>,
	owned: Vec<Option<mpsc::OwnedPermit<Item>>>,
	next: u8,
}

impl State {
	fn item(&mut self) -> Item {
		self.next = self.next.wrapping_add(1);
		Item(self.next)
	}

	fn sender(&self, arg: u8) -> Option<Tx> {
		let live: Vec<&Tx> = self.senders.iter().flatten().collect();
		(!live.is_empty()).then(|| live[usize::from(arg) % live.len()].clone())
	}

	/// The bounded sender `arg` picks, if it picks one, read by reference: rust2-ts can't yet
	/// type-check the drop of the rest of a temporary `if let Some(Tx::Bounded(tx))` tests.
	fn bounded(&self, arg: u8) -> Option<mpsc::Sender<Item>> {
		match self.sender(arg)? {
			Tx::Bounded(tx) => Some(tx),
			Tx::Unbounded(_) => None,
		}
	}

	/// Take the receiver from its slot for a task, which puts it back once it is done.
	fn take_rx(&self) -> Option<(Rx, Rc<RefCell<Option<Rx>>>)> {
		let rx = self.rx.borrow_mut().take()?;
		Some((rx, self.rx.clone()))
	}
}

/// The bounded channel of the capacity the first byte picks, then its script.
pub fn bounded(data: &[u8], out: &mut String) {
	let [cap, script @ ..] = data else {
		return;
	};
	let (tx, rx) = mpsc::channel(usize::from(cap % 4) + 1);
	script_run(Tx::Bounded(tx), Rx::Bounded(rx), script, out);
}

/// The unbounded channel, then its script.
pub fn unbounded(data: &[u8], out: &mut String) {
	let (tx, rx) = mpsc::unbounded_channel();
	script_run(Tx::Unbounded(tx), Rx::Unbounded(rx), data, out);
}

fn script_run(tx: Tx, rx: Rx, script: &[u8], out: &mut String) {
	let mut s = State {
		exec: Executor::default(),
		senders: vec![Some(tx)],
		weaks: Vec::new(),
		rx: Rc::new(RefCell::new(Some(rx))),
		owned: Vec::new(),
		next: 0,
	};
	for step in script.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		step_run(&mut s, op % 20, arg);
		let woken = s.exec.woken();
		if !woken.is_empty() {
			note(format!("woke {woken}"));
		}
	}
	note("end".into());
	s.exec.clear();
	s.owned.clear();
	note("owned dropped".into());
	s.weaks.clear();
	s.senders.clear();
	note("senders dropped".into());
	drop(s.rx.borrow_mut().take());
	note("receiver dropped".into());
	out.push_str(&crate::log::take());
}

fn step_run(s: &mut State, op: u8, arg: u8) {
	match op {
		// Receive one value.
		0 => {
			if let Some((mut rx, slot)) = s.take_rx() {
				s.exec.spawn(async move {
					let v = rx.recv().await;
					*slot.borrow_mut() = Some(rx);
					format!("recv {v:?}")
				});
			}
		}
		// Receive up to `arg % 4` values into a vector of a capacity of its own.
		1 => {
			if let Some((mut rx, slot)) = s.take_rx() {
				let limit = usize::from(arg % 4);
				s.exec.spawn(async move {
					let mut buffer = Vec::with_capacity(usize::from(limit > 1));
					let n = match &mut rx {
						Rx::Bounded(rx) => rx.recv_many(&mut buffer, limit).await,
						Rx::Unbounded(rx) => rx.recv_many(&mut buffer, limit).await,
					};
					*slot.borrow_mut() = Some(rx);
					format!("recv_many {n} {buffer:?} cap {}", buffer.capacity())
				});
			}
		}
		// Receive one value by `poll_recv`.
		2 => {
			if let Some((mut rx, slot)) = s.take_rx() {
				s.exec.spawn(async move {
					let v = match &mut rx {
						Rx::Bounded(rx) => poll_fn(|cx| rx.poll_recv(cx)).await,
						Rx::Unbounded(rx) => poll_fn(|cx| rx.poll_recv(cx)).await,
					};
					*slot.borrow_mut() = Some(rx);
					format!("poll_recv {v:?}")
				});
			}
		}
		// Send a value, as a task.
		3 => {
			let item = s.item();
			match s.sender(arg) {
				Some(Tx::Bounded(tx)) => s.exec.spawn(async move {
					match tx.send(item).await {
						Ok(()) => "sent".into(),
						Err(e) => format!("send {e} {e:?} {}", e.0),
					}
				}),
				Some(Tx::Unbounded(tx)) => match tx.send(item) {
					Ok(()) => note("sent".into()),
					Err(e) => note(format!("send {e} {e:?} {}", e.0)),
				},
				None => {}
			}
		}
		// Reserve a slot, then send or not.
		4 => {
			let item = s.item();
			if let Some(tx) = s.bounded(arg) {
				s.exec.spawn(async move {
					match tx.reserve().await {
						Ok(permit) if arg & 0x80 == 0 => {
							permit.send(item);
							"reserved and sent".into()
						}
						Ok(permit) => {
							drop(permit);
							"reserved and dropped".into()
						}
						Err(e) => format!("reserve {e} {e:?}"),
					}
				});
			}
		}
		// Reserve `arg % 6` slots, more than any channel holds included, and send through some.
		5 => {
			let first = s.item();
			if let Some(tx) = s.bounded(arg) {
				let n = usize::from(arg % 6);
				let sent = usize::from(arg >> 4) % 3;
				s.exec.spawn(async move {
					match tx.reserve_many(n).await {
						Ok(mut permits) => {
							let len = permits.len();
							let mut item = Some(first);
							for _ in 0..sent {
								if let (Some(permit), Some(v)) = (permits.next(), item.take()) {
									permit.send(v);
								}
							}
							format!("reserved {len}, {} left {:?}", permits.len(), permits.size_hint())
						}
						Err(e) => format!("reserve_many {e}"),
					}
				});
			}
		}
		// Reserve a slot with a sender of its own, kept to use later.
		6 => {
			if let Some(tx) = s.bounded(arg) {
				let owned = Rc::new(RefCell::new(None));
				let slot = owned.clone();
				s.exec.spawn(async move {
					match tx.reserve_owned().await {
						Ok(permit) => {
							*slot.borrow_mut() = Some(permit);
							"reserved owned".into()
						}
						Err(e) => format!("reserve_owned {e}"),
					}
				});
				s.exec.run();
				if let Some(permit) = owned.borrow_mut().take() {
					s.owned.push(Some(permit));
				}
			}
		}
		// Use or drop an owned permit.
		7 => {
			let live: Vec<usize> = (0..s.owned.len()).filter(|&i| s.owned[i].is_some()).collect();
			if live.is_empty() {
				return;
			}
			let i = live[usize::from(arg) % live.len()];
			let permit = s.owned[i].take().expect("live");
			if let Some(tx) = s.bounded(0) {
				note(format!(
					"same {} {}",
					permit.same_channel_as_sender(&tx),
					live.len() > 1 && {
						let j = live[(usize::from(arg) + 1) % live.len()];
						s.owned[j].as_ref().is_some_and(|other| permit.same_channel(other))
					}
				));
			}
			match arg % 3 {
				0 => {
					let item = s.item();
					let tx = permit.send(item);
					note(format!("owned sent, {} senders", tx.strong_count()));
					s.senders.push(Some(Tx::Bounded(tx)));
				}
				1 => {
					let tx = permit.release();
					note(format!("owned released, capacity {}", tx.capacity()));
					s.senders.push(Some(Tx::Bounded(tx)));
				}
				_ => {
					drop(permit);
					note("owned dropped".into());
				}
			}
		}
		// Wait for the receiver to close.
		8 => {
			if let Some(tx) = s.sender(arg) {
				s.exec.spawn(async move {
					match &tx {
						Tx::Bounded(tx) => tx.closed().await,
						Tx::Unbounded(tx) => tx.closed().await,
					}
					"closed".into()
				});
			}
		}
		// Send without waiting.
		9 => {
			let item = s.item();
			match s.sender(arg) {
				Some(Tx::Bounded(tx)) => match tx.try_send(item) {
					Ok(()) => note("try_send ok".into()),
					Err(e) => {
						let line = format!("try_send {e} {e:?}");
						note(format!("{line} {}", e.into_inner()));
					}
				},
				Some(Tx::Unbounded(tx)) => note(format!("send {:?}", tx.send(item).is_ok())),
				None => {}
			}
		}
		// Reserve without waiting.
		10 => {
			if let Some(tx) = s.bounded(arg) {
				match arg % 3 {
					0 => match tx.try_reserve() {
						Ok(permit) => {
							let item = s.item();
							permit.send(item);
							note("try_reserve sent".into());
						}
						Err(e) => note(format!("try_reserve {e} {e:?}")),
					},
					1 => match tx.try_reserve_many(usize::from(arg >> 4)) {
						Ok(permits) => note(format!("try_reserve_many {}", permits.len())),
						Err(e) => note(format!("try_reserve_many {e}")),
					},
					_ => match tx.clone().try_reserve_owned() {
						Ok(permit) => s.owned.push(Some(permit)),
						Err(e) => note(format!("try_reserve_owned {e}")),
					},
				}
			}
		}
		// Receive without waiting.
		11 => {
			let mut slot = s.rx.borrow_mut();
			let got = match slot.as_mut() {
				Some(Rx::Bounded(rx)) => rx.try_recv(),
				Some(Rx::Unbounded(rx)) => rx.try_recv(),
				None => return,
			};
			drop(slot);
			match got {
				Ok(v) => note(format!("try_recv {v}")),
				Err(e) => note(format!("try_recv {e} {e:?}")),
			}
		}
		12 => match s.rx.borrow_mut().as_mut() {
			Some(Rx::Bounded(rx)) => rx.close(),
			Some(Rx::Unbounded(rx)) => rx.close(),
			None => {}
		},
		// Drop a sender.
		13 => {
			let live: Vec<usize> = (0..s.senders.len()).filter(|&i| s.senders[i].is_some()).collect();
			if let Some(&i) = live.get(usize::from(arg) % live.len().max(1)) {
				note(format!("drop sender {i}"));
				s.senders[i] = None;
			}
		}
		14 => {
			if let Some(tx) = s.sender(arg) {
				s.senders.push(Some(tx));
			}
		}
		15 => {
			note("drop receiver".into());
			drop(s.rx.borrow_mut().take());
		}
		16 => s.exec.cancel(arg),
		17 => s.exec.run(),
		// Downgrade a sender, upgrade a weak one, or drop one.
		18 => match arg % 3 {
			0 => {
				if let Some(tx) = s.sender(arg) {
					s.weaks.push(Some(match &tx {
						Tx::Bounded(tx) => Weak::Bounded(tx.downgrade()),
						Tx::Unbounded(tx) => Weak::Unbounded(tx.downgrade()),
					}));
				}
			}
			1 => {
				if let Some(Some(weak)) = s.weaks.last() {
					let up = match weak.clone() {
						Weak::Bounded(w) => w.upgrade().map(Tx::Bounded),
						Weak::Unbounded(w) => w.upgrade().map(Tx::Unbounded),
					};
					note(format!("upgrade {}", up.is_some()));
					s.senders.push(up);
				}
			}
			_ => {
				s.weaks.pop();
			}
		},
		// What the handles report.
		_ => state(s),
	}
}

fn state(s: &State) {
	let mut line = String::from("state");
	if let Some(tx) = s.sender(0) {
		match &tx {
			Tx::Bounded(tx) => {
				line += &format!(
					" tx closed {} cap {}/{} strong {} weak {}",
					tx.is_closed(),
					tx.capacity(),
					tx.max_capacity(),
					tx.strong_count(),
					tx.weak_count()
				)
			}
			Tx::Unbounded(tx) => {
				line += &format!(
					" tx closed {} strong {} weak {}",
					tx.is_closed(),
					tx.strong_count(),
					tx.weak_count()
				)
			}
		}
		if let Some(other) = s.bounded(1)
			&& let Tx::Bounded(tx) = &tx
		{
			line += &format!(" same {}", tx.same_channel(&other));
		}
	}
	if let Some(Some(weak)) = s.weaks.last() {
		match weak {
			Weak::Bounded(w) => line += &format!(" weak {} {} {w:?}", w.strong_count(), w.weak_count()),
			Weak::Unbounded(w) => line += &format!(" weak {} {} {w:?}", w.strong_count(), w.weak_count()),
		}
	}
	match s.rx.borrow().as_ref() {
		Some(Rx::Bounded(rx)) => {
			line += &format!(
				" rx closed {} empty {} len {} cap {}/{} senders {} {}",
				rx.is_closed(),
				rx.is_empty(),
				rx.len(),
				rx.capacity(),
				rx.max_capacity(),
				rx.sender_strong_count(),
				rx.sender_weak_count()
			)
		}
		Some(Rx::Unbounded(rx)) => {
			line += &format!(
				" rx closed {} empty {} len {} senders {} {}",
				rx.is_closed(),
				rx.is_empty(),
				rx.len(),
				rx.sender_strong_count(),
				rx.sender_weak_count()
			)
		}
		None => {}
	}
	note(line);
}
