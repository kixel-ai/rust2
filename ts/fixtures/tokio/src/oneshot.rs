//! A oneshot channel driven by a script of steps: a task awaiting the receiver, one waiting
//! for it to close, a send, a try, a close, and drops, logging each outcome and drop, and
//! which tasks each step woke.

use std::cell::RefCell;
use std::future::poll_fn;
use std::rc::Rc;

use tokio::sync::oneshot;

use crate::exec::Executor;
use crate::log::{Item, note};

/// A handle's slot, which a task takes it from and puts it back into once done.
type Slot<T> = Rc<RefCell<Option<T>>>;

pub fn script(data: &[u8], out: &mut String) {
	let (tx, rx) = oneshot::channel::<Item>();
	note(format!("{tx:?} {rx:?}"));
	let tx: Slot<oneshot::Sender<Item>> = Rc::new(RefCell::new(Some(tx)));
	let rx: Slot<oneshot::Receiver<Item>> = Rc::new(RefCell::new(Some(rx)));
	let mut exec = Executor::default();
	for step in data.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		match op % 11 {
			// Await the receiver.
			0 => {
				if let Some(mut r) = rx.borrow_mut().take() {
					let slot = rx.clone();
					exec.spawn(async move {
						let got = (&mut r).await;
						let line = format!("recv {got:?} {r:?}");
						*slot.borrow_mut() = Some(r);
						line
					});
				}
			}
			// Wait for the receiver to close, or poll for it.
			1 => {
				if let Some(mut t) = tx.borrow_mut().take() {
					let slot = tx.clone();
					exec.spawn(async move {
						match arg & 1 {
							0 => t.closed().await,
							_ => poll_fn(|cx| t.poll_closed(cx)).await,
						}
						let line = format!("closed {}", t.is_closed());
						*slot.borrow_mut() = Some(t);
						line
					});
				}
			}
			2 => {
				if let Some(t) = tx.borrow_mut().take() {
					match t.send(Item(arg)) {
						Ok(()) => note("sent".into()),
						Err(v) => note(format!("send back {v}")),
					}
				}
			}
			3 => {
				if let Some(r) = rx.borrow_mut().as_mut() {
					match r.try_recv() {
						Ok(v) => note(format!("try_recv {v}")),
						Err(e) => note(format!("try_recv {e} {e:?}")),
					}
				}
			}
			4 => {
				if let Some(r) = rx.borrow_mut().as_mut() {
					r.close();
				}
			}
			5 => {
				note("drop sender".into());
				drop(tx.borrow_mut().take());
			}
			6 => {
				note("drop receiver".into());
				drop(rx.borrow_mut().take());
			}
			7 => exec.cancel(arg),
			8 => exec.run(),
			9 => {
				if let Some(r) = rx.borrow().as_ref() {
					note(format!(
						"rx empty {} terminated {} {r:?}",
						r.is_empty(),
						r.is_terminated()
					));
				}
				if let Some(t) = tx.borrow().as_ref() {
					note(format!("tx closed {} {t:?}", t.is_closed()));
				}
			}
			// Await a receiver that is done, which panics.
			_ => {
				let done = rx.borrow().as_ref().is_some_and(|r| r.is_terminated());
				if done && arg == 0x5a {
					let mut r = rx.borrow_mut().take().expect("live");
					exec.spawn(async move { format!("{:?}", (&mut r).await) });
					exec.run();
				}
			}
		}
		let woken = exec.woken();
		if !woken.is_empty() {
			note(format!("woke {woken}"));
		}
	}
	note("end".into());
	exec.clear();
	drop(tx.borrow_mut().take());
	drop(rx.borrow_mut().take());
	note("dropped".into());
	out.push_str(&crate::log::take());
}
