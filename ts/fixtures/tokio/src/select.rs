//! `tokio::select!`, biased as web-codecs' decoder waits for a frame or for the decoder to
//! close, and unbiased where at most one branch is ready at each poll, so the branch it
//! picks first at random shows in no log.

use tokio::sync::{mpsc, oneshot, watch};

use crate::exec::Executor;
use crate::log::{Item, note};

/// A decoder's output, as web-codecs' `video::Decoded` is.
struct Decoded {
	frames: mpsc::UnboundedReceiver<Item>,
	closed: watch::Receiver<Result<(), String>>,
}

impl Decoded {
	async fn next(&mut self) -> Result<Option<Item>, String> {
		tokio::select! {
			biased;
			frame = self.frames.recv() => Ok(frame),
			Ok(()) = self.closed.changed() => Err(self.closed.borrow().clone().err().unwrap()),
		}
	}
}

/// The decoder's frames and its close, by a script of steps, with a task reading frames
/// until none is left or it closed.
pub fn decoded(data: &[u8], out: &mut String) {
	let (frames_tx, frames) = mpsc::unbounded_channel();
	let (closed_tx, closed) = watch::channel(Ok(()));
	let mut frames_tx = Some(frames_tx);
	let mut closed_tx = Some(closed_tx);
	let mut decoded = Decoded { frames, closed };
	let mut exec = Executor::default();
	exec.spawn(async move {
		loop {
			match decoded.next().await {
				Ok(Some(frame)) => note(format!("frame {frame}")),
				Ok(None) => return "frames done".into(),
				Err(e) => return format!("closed {e}"),
			}
		}
	});
	for step in data.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		match op % 6 {
			0 => {
				if let Some(tx) = &frames_tx {
					note(format!("send {:?}", tx.send(Item(arg)).is_ok()));
				}
			}
			1 => {
				if let Some(tx) = &closed_tx {
					// An `Ok` the decoder reads as an error, which panics, only where asked.
					let value = if arg == 0x5a { Ok(()) } else { Err(format!("e{arg}")) };
					note(format!("replaced {:?}", tx.send_replace(value)));
				}
			}
			2 => {
				note("drop frames".into());
				frames_tx = None;
			}
			3 => {
				note("drop closed".into());
				closed_tx = None;
			}
			4 => exec.cancel(arg),
			_ => exec.run(),
		}
		let woken = exec.woken();
		if !woken.is_empty() {
			note(format!("woke {woken}"));
		}
	}
	note("end".into());
	exec.clear();
	drop(frames_tx);
	drop(closed_tx);
	out.push_str(&crate::log::take());
}

/// An unbiased select of an mpsc receiver's values, a oneshot's, and `else` once both are
/// done, each step followed by a run, so at most one branch is newly ready at each poll.
pub fn unbiased(data: &[u8], out: &mut String) {
	let (a_tx, mut a) = mpsc::channel::<Item>(1);
	let (b_tx, mut b) = oneshot::channel::<Item>();
	let mut a_tx = Some(a_tx);
	let mut b_tx = Some(b_tx);
	let mut exec = Executor::default();
	exec.spawn(async move {
		let mut b_done = false;
		loop {
			let line = tokio::select! {
				Some(v) = a.recv() => format!("a {v}"),
				v = &mut b, if !b_done => {
					b_done = true;
					format!("b {v:?}")
				}
				else => break "both done".into(),
			};
			note(line);
		}
	});
	exec.run();
	for step in data.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		match op % 5 {
			0 => {
				if let Some(tx) = &a_tx {
					note(format!(
						"try_send {:?}",
						tx.try_send(Item(arg)).map_err(|e| e.to_string())
					));
				}
			}
			1 => {
				if let Some(tx) = b_tx.take() {
					note(format!("send {:?}", tx.send(Item(arg)).map_err(|v| v.0)));
				}
			}
			2 => {
				note("drop a".into());
				a_tx = None;
			}
			3 => {
				note("drop b".into());
				b_tx = None;
			}
			_ => exec.cancel(arg),
		}
		exec.run();
	}
	note("end".into());
	exec.clear();
	drop(a_tx);
	drop(b_tx);
	out.push_str(&crate::log::take());
}
