//! A deterministic executor, translated with the rest: each run polls the woken tasks in
//! the order they were spawned, not woken, so the log is the same whichever order a
//! channel wakes its waiters in, as a watch channel's are at random.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Wake, Waker};

use crate::log::note;

/// Whether a task was woken since its last poll.
struct Flag(AtomicBool);

impl Wake for Flag {
	fn wake(self: Arc<Self>) {
		self.0.store(true, Ordering::Relaxed);
	}

	fn wake_by_ref(self: &Arc<Self>) {
		self.0.store(true, Ordering::Relaxed);
	}
}

struct Task {
	future: Pin<Box<dyn Future<Output = String>>>,
	flag: Arc<Flag>,
	waker: Waker,
}

/// Tasks by id, `None` once finished or dropped.
#[derive(Default)]
pub struct Executor {
	tasks: Vec<Option<Task>>,
}

impl Executor {
	/// Spawn a task, polled at the next run.
	pub fn spawn(&mut self, future: impl Future<Output = String> + 'static) {
		let flag = Arc::new(Flag(AtomicBool::new(true)));
		note(format!("t{} spawn", self.tasks.len()));
		self.tasks.push(Some(Task {
			future: Box::pin(future),
			waker: Waker::from(flag.clone()),
			flag,
		}));
	}

	/// Poll woken tasks, oldest first, until none is woken. A task polls at most `limit`
	/// times, so one that wakes itself forever stops.
	pub fn run(&mut self) {
		for _ in 0..16 {
			let mut polled = false;
			for id in 0..self.tasks.len() {
				let Some(task) = self.tasks[id].as_mut() else {
					continue;
				};
				if !task.flag.0.swap(false, Ordering::Relaxed) {
					continue;
				}
				polled = true;
				let mut cx = Context::from_waker(&task.waker);
				match task.future.as_mut().poll(&mut cx) {
					Poll::Ready(out) => {
						note(format!("t{id} ready {out}"));
						self.tasks[id] = None;
					}
					Poll::Pending => note(format!("t{id} pending")),
				}
			}
			if !polled {
				return;
			}
		}
	}

	/// Which live tasks are woken and waiting for a poll.
	pub fn woken(&self) -> String {
		let ids: Vec<String> = (self.tasks.iter().enumerate())
			.filter(|(_, t)| t.as_ref().is_some_and(|t| t.flag.0.load(Ordering::Relaxed)))
			.map(|(id, _)| format!("t{id}"))
			.collect();
		ids.join(",")
	}

	/// Drop the live task `arg` picks, if any.
	pub fn cancel(&mut self, arg: u8) {
		let live: Vec<usize> = (0..self.tasks.len()).filter(|&id| self.tasks[id].is_some()).collect();
		if live.is_empty() {
			return;
		}
		let id = live[usize::from(arg) % live.len()];
		note(format!("t{id} cancel"));
		self.tasks[id] = None;
	}

	/// Drop every live task, oldest first.
	pub fn clear(&mut self) {
		for id in 0..self.tasks.len() {
			if self.tasks[id].take().is_some() {
				note(format!("t{id} drop"));
			}
		}
	}
}
