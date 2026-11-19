//! The model's protocol scripts, `tests/protocol/model.rs`, run through the api fixture's
//! `extern "C"` functions under Miri, as `tests/protocol.c` runs them natively: Miri fails
//! on a leak, a use after free, or a race any step's interleaving with the worker makes.
//! `api_miri` mounts this file beside `tests/miri/api.rs`.

use super::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};

#[path = "../protocol/model.rs"]
#[allow(dead_code, reason = "writing scripts uses what running them does not")]
mod model;

use model::{Kind, Op, Outcome, Script, Status, Step};

/// The scripts Miri runs: few and short, as each step costs it thousands of native ones.
const SEEDS: u64 = 12;
const STEPS: usize = 40;

/// A script's handles, the kind of each, and its waker's counts, which both threads reach.
struct Driver {
	slots: Vec<AtomicPtr<c_void>>,
	kinds: Vec<Kind>,
	wakes: AtomicUsize,
	drops: AtomicUsize,
	/// The waker's functions, reified once: Miri may give each reification its own address,
	/// so a waker made anew per poll would never match the one a future keeps.
	wake: unsafe extern "C" fn(*mut c_void),
	drop: unsafe extern "C" fn(*mut c_void),
}

unsafe extern "C" fn wake(data: *mut c_void) {
	unsafe { &*data.cast::<Driver>() }.wakes.fetch_add(1, Ordering::SeqCst);
}

unsafe extern "C" fn drop_waker(data: *mut c_void) {
	unsafe { &*data.cast::<Driver>() }.drops.fetch_add(1, Ordering::SeqCst);
}

fn view(s: &'static str) -> rt::Buf {
	rt::Buf {
		ptr: s.as_ptr(),
		len: s.len(),
		owner: std::ptr::null_mut(),
	}
}

impl Driver {
	fn at(&self, h: usize) -> *const c_void {
		self.slots[h].load(Ordering::SeqCst)
	}

	fn opt(&self, h: Option<usize>) -> *const c_void {
		h.map_or(std::ptr::null(), |h| self.at(h))
	}

	fn set(&self, h: usize, handle: *const c_void) {
		self.slots[h].store(handle.cast_mut(), Ordering::SeqCst);
	}

	fn waker(&self) -> rt::Waker {
		rt::Waker {
			wake: Some(self.wake),
			drop: Some(self.drop),
			data: std::ptr::from_ref(self).cast_mut().cast(),
		}
	}

	/// Run `op` on this thread: what it returned.
	unsafe fn run(&self, op: Op) -> Outcome {
		let mut status = 0;
		let st = &raw mut status;
		let mut made = std::ptr::null::<c_void>();
		let mut into = 0;
		let value = |v: u64| Outcome::Value(v);
		let outcome = unsafe {
			match op {
				Op::CounterNew(h, n) => {
					self.set(h, api_Counter_new(view("c"), n));
					Outcome::Ok
				}
				Op::CounterGet(h) => value(api_Counter_get(self.at(h), st)),
				Op::CounterAdd(h, n) => {
					let mut out = 0;
					let mut err = std::mem::MaybeUninit::<api_Error>::uninit();
					match api_Counter_add(self.at(h), n, &mut out, err.as_mut_ptr(), st) {
						true => value(out),
						false if status == 0 => {
							api_Error_free(err.as_mut_ptr());
							Outcome::Error
						}
						false => Outcome::Ok,
					}
				}
				Op::CounterIntoSum(h, n) => value(api_Counter_into_sum(self.at(h), n, st)),
				Op::CounterPlus(h, o) => value(api_Counter_plus(self.at(h), self.opt(o), st)),
				Op::CounterAddTo(h, o) => value(api_Counter_add_to(self.at(h), self.opt(o), st).into()),
				Op::CounterClone(to, h) => {
					(into, made) = (to, api_Counter_clone(self.at(h), st));
					Outcome::Ok
				}
				Op::PlusLater(to, h, o) => {
					(into, made) = (to, api_Counter_plus_later(self.at(h), self.opt(o), st).cast_const().cast());
					Outcome::Ok
				}
				Op::LedgerNew(h) => {
					self.set(h, api_Ledger_new());
					Outcome::Ok
				}
				Op::LedgerEdit(to, h) => {
					let mut draft = std::ptr::null();
					let mut err = std::mem::MaybeUninit::<api_Error>::uninit();
					match api_Ledger_edit(self.at(h), &mut draft, err.as_mut_ptr(), st) {
						true => {
							(into, made) = (to, draft);
							value(1)
						}
						false if status == 0 => {
							api_Error_free(err.as_mut_ptr());
							Outcome::Error
						}
						false => Outcome::Ok,
					}
				}
				Op::LedgerPublished(h) => value(api_Ledger_published(self.at(h), st).into()),
				Op::LedgerSeal(h) => {
					api_Ledger_seal(self.at(h), st);
					Outcome::Ok
				}
				Op::LedgerReceipt(to, h) => {
					(into, made) = (to, api_Ledger_receipt(self.at(h), st));
					Outcome::Ok
				}
				Op::DraftWrite(h) => {
					api_Draft_write(self.at(h), view("line"), st);
					Outcome::Ok
				}
				Op::DraftLines(h) => value(api_Draft_lines(self.at(h), st).into()),
				Op::DraftCommit(h) => value(api_Draft_commit(self.at(h), st).into()),
				Op::ReceiptCount(h) => value(api_Receipt_count(self.at(h), st).into()),
				Op::ReceiptClone(to, h) => {
					(into, made) = (to, api_Receipt_clone(self.at(h), st));
					Outcome::Ok
				}
				Op::TallyNew(h) => {
					self.set(h, api_Tally_new());
					Outcome::Ok
				}
				Op::TallyBump(h) => value(api_Tally_bump(self.at(h), st).into()),
				Op::BumpLater(to, h) => {
					(into, made) = (to, api_Tally_bump_later(self.at(h), st).cast_const().cast());
					Outcome::Ok
				}
				Op::LocalNew(h, homed) => {
					assert!(!homed, "Miri's scripts make nothing on a home");
					self.set(h, api_Local_new());
					Outcome::Ok
				}
				Op::LocalBump(h) => value(api_Local_bump(self.at(h), st).into()),
				Op::LocalShare(to, h) => {
					(into, made) = (to, api_Local_share(self.at(h), st));
					Outcome::Ok
				}
				Op::LocalLater(to, h) => {
					(into, made) = (to, api_Local_later(self.at(h), st).cast_const().cast());
					Outcome::Ok
				}
				Op::MetersNew(h) => {
					self.set(h, api_Meters_new());
					Outcome::Ok
				}
				Op::MetersFirst(to, h) => {
					(into, made) = (to, api_Meters_first(self.at(h), st));
					Outcome::Ok
				}
				Op::MetersClear(h) => {
					api_Meters_clear(self.at(h), st);
					Outcome::Ok
				}
				Op::MeterNew(h) => {
					self.set(h, api_Meter_default());
					Outcome::Ok
				}
				Op::MeterAdd(h, n) => value(api_Meter_add(self.at(h), n, st)),
				Op::MeterReset(h) => {
					api_Meter_reset(self.at(h), st);
					Outcome::Ok
				}
				Op::Drop(h) => {
					self.release(h, Some(st));
					Outcome::Ok
				}
				Op::Free(h) => {
					self.release(h, None);
					Outcome::Ok
				}
				Op::Poll(f) => self.poll(f, st),
				Op::Drain => {
					api_thread_drain();
					Outcome::Ok
				}
			}
		};
		if let Some(failed) = model_status(status) {
			return Outcome::Failed(failed);
		}
		if !made.is_null() {
			self.set(into, made);
		}
		outcome
	}

	/// `_drop` with a status, or `_free` without.
	unsafe fn release(&self, h: usize, drop: Option<*mut i32>) {
		let handle = self.at(h);
		let future = handle.cast_mut().cast::<rt::Future>();
		macro_rules! object {
			($drop:ident, $free:ident) => {
				match drop {
					Some(st) => $drop(handle, st),
					None => $free(handle),
				}
			};
		}
		unsafe {
			match self.kinds[h] {
				Kind::Counter => object!(api_Counter_drop, api_Counter_free),
				Kind::Ledger => object!(api_Ledger_drop, api_Ledger_free),
				Kind::Draft => object!(api_Draft_drop, api_Draft_free),
				Kind::Receipt => object!(api_Receipt_drop, api_Receipt_free),
				Kind::Tally => object!(api_Tally_drop, api_Tally_free),
				Kind::Local => object!(api_Local_drop, api_Local_free),
				Kind::Meters => object!(api_Meters_drop, api_Meters_free),
				Kind::Meter => object!(api_Meter_drop, api_Meter_free),
				Kind::PlusLater => api_Counter_plus_later_future_free(future),
				Kind::BumpLater => api_Tally_bump_later_future_free(future),
				Kind::LocalLater => api_Local_later_future_free(future),
			}
		}
	}

	unsafe fn poll(&self, f: usize, st: *mut i32) -> Outcome {
		let future = self.at(f).cast_mut().cast::<rt::Future>();
		let mut out = 0u64;
		let mut out32 = 0u32;
		let polled = unsafe {
			match self.kinds[f] {
				Kind::PlusLater => api_Counter_plus_later_poll(future, self.waker(), &mut out),
				Kind::BumpLater => api_Tally_bump_later_poll(future, self.waker(), &mut out32, st),
				Kind::LocalLater => api_Local_later_poll(future, self.waker(), &mut out32, st),
				kind => unreachable!("{kind:?} is no future"),
			}
		};
		match polled {
			rt::PENDING => Outcome::Pending,
			_ if self.kinds[f] == Kind::PlusLater => Outcome::Value(out),
			_ => Outcome::Value(out32.into()),
		}
	}
}

/// The model's status for a C status number, if any failed.
fn model_status(code: i32) -> Option<Status> {
	[
		Status::Moved,
		Status::Exclusive,
		Status::Borrowed,
		Status::Lent,
		Status::Outlived,
		Status::NotSend,
	]
	.into_iter()
	.find(|status| status.code() == code)
	.or_else(|| (code != 0).then(|| panic!("status {code} is none the model knows")))
}

/// A pointer the worker thread reads only while the main thread waits for it.
struct Shared(Arc<Driver>);

// SAFETY: the driver's handles are only C's pointers, used on one thread at a time.
unsafe impl Send for Shared {}

fn run(script: &Script) {
	let driver = Arc::new(Driver {
		slots: (0..script.kinds.len()).map(|_| AtomicPtr::new(std::ptr::null_mut())).collect(),
		kinds: script.kinds.clone(),
		wakes: AtomicUsize::new(0),
		drops: AtomicUsize::new(0),
		wake,
		drop: drop_waker,
	});
	let (to_worker, steps) = mpsc::channel::<Step>();
	let (done, outcomes) = mpsc::channel::<Outcome>();
	let shared = Shared(driver.clone());
	let worker = std::thread::spawn(move || {
		let shared = shared;
		for step in steps {
			// SAFETY: each step names live handles, as the model writes it.
			done.send(unsafe { shared.0.run(step.op) }).unwrap();
		}
	});
	for (i, step) in script.steps.iter().enumerate() {
		let got = match step.thread {
			// SAFETY: as above.
			0 => unsafe { driver.run(step.op) },
			_ => {
				to_worker.send(*step).unwrap();
				outcomes.recv().unwrap()
			}
		};
		assert_eq!(got, step.expect, "script {} step {i}: {}", script.seed, step.line());
	}
	drop(to_worker);
	worker.join().unwrap();
	assert_eq!(
		(driver.wakes.load(Ordering::SeqCst), driver.drops.load(Ordering::SeqCst)),
		(script.wakes as usize, script.drops as usize),
		"script {}'s waker counts",
		script.seed
	);
}

#[test]
fn scripts() {
	for seed in 0..SEEDS {
		run(&model::script(seed, STEPS, false));
	}
}
