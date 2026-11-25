// The Rust floor: every case the bindings time, called on the stand-in's API directly, as a
// Rust caller holding a byte slice would. Futures are polled by hand with a no-op waker,
// so a woken read costs a re-poll and nothing of an executor. Each single-threaded case is a
// `#[bench]` function, which rust2-bench counts through rust2-bench-harness when
// RUST2_CODSPEED is set; otherwise each case prints its wall time and the process's CPU time.

use std::future::Future;
use std::hint::black_box;
use std::pin::pin;
use std::sync::Barrier;
use std::task::{Context, Poll, Waker};
use std::time::Instant;

/// The groups a counted case writes, a tenth of a timed one's: Valgrind counts every call exactly.
const GROUPS: u64 = 1000;
/// How many times a timed case runs a counted one's loop.
const REPEATS: u64 = 10;
const FRAMES: u64 = 100;
/// The calls a counted case makes.
const CALLS: u64 = GROUPS * FRAMES;
const THREADS: usize = 4;
const SIZES: [usize; 5] = [100, 1024, 16384, 65536, 1048576];

/// When a case started, on the wall clock and in the process's CPU time.
struct Start {
	wall: Instant,
	cpu: f64,
}

/// The process's CPU time in nanoseconds, on every thread.
fn cpu() -> f64 {
	let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
	// SAFETY: `ts` is a valid out pointer.
	assert_eq!(unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts) }, 0);
	ts.tv_sec as f64 * 1e9 + ts.tv_nsec as f64
}

fn now() -> Start {
	Start { wall: Instant::now(), cpu: cpu() }
}

/// Wall and CPU nanoseconds since `start`.
fn elapsed(start: Start) -> (f64, f64) {
	(start.wall.elapsed().as_nanos() as f64, cpu() - start.cpu)
}

/// Wall and CPU nanoseconds per call of `case`'s [`REPEATS`] runs of [`CALLS`] calls each.
fn per_call(mut case: impl FnMut()) -> (f64, f64) {
	let start = now();
	for _ in 0..REPEATS {
		case();
	}
	let (wall, cpu) = elapsed(start);
	let n = (REPEATS * CALLS) as f64;
	(wall / n, cpu / n)
}

/// `call` made `n` times on each of [`THREADS`] threads at once, thread `i` on `handles[i]`:
/// wall nanoseconds per call on one thread, and CPU nanoseconds per call.
fn concurrent<T: Sync>(handles: &[&T], n: u64, call: impl Fn(&T) + Sync) -> (f64, f64) {
	// Every thread waits at `ready`, then at `go`, which opens only once the span has begun.
	let (ready, go) = (Barrier::new(THREADS + 1), Barrier::new(THREADS + 1));
	// The scope joins every thread before it returns, so the span ends after the last call.
	let begin = std::thread::scope(|scope| {
		for &handle in handles {
			let (ready, go, call) = (&ready, &go, &call);
			scope.spawn(move || {
				ready.wait();
				go.wait();
				for _ in 0..n {
					call(black_box(handle));
				}
			});
		}
		ready.wait();
		let begin = now();
		go.wait();
		begin
	});
	let (wall, cpu) = elapsed(begin);
	(wall / n as f64, cpu / (n * THREADS as u64) as f64)
}

/// `future`'s output, which must be ready at its first poll.
fn resolve<F: Future>(future: F) -> F::Output {
	match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
		Poll::Ready(output) => output,
		Poll::Pending => panic!("pending"),
	}
}

/// The cases rust2-bench counts, each named `ffi/moq/bench/export.rs::rust::<case>`, as each
/// language's program names its own, and each given a broadcast of its own by its setup.
mod rust {
	use std::future::Future;
	use std::hint::black_box;
	use std::pin::pin;
	use std::task::{Context, Poll, Waker};

	use rust2_bench_harness::bench;

	use super::{CALLS, FRAMES, GROUPS, SIZES, resolve};

	/// A timed track. It caches every group written to it until it is dropped, as its
	/// broadcast's pool, which nothing polls, evicts none.
	pub struct Track {
		_broadcast: moq::broadcast::Producer,
		pub track: moq::track::Producer,
		/// The next group's start, in microseconds: groups are keyed by their start, which
		/// only moves forward on a track.
		micros: u64,
	}

	impl Track {
		/// The next microsecond.
		fn at(&mut self) -> moq::Timestamp {
			let at = moq::Timestamp::from_micros(self.micros).unwrap();
			self.micros += 1;
			at
		}
	}

	pub fn track() -> Track {
		let broadcast = moq::broadcast::Info::new().produce();
		let info = moq::track::Info::default().with_timescale(moq::Timescale::MICRO);
		let track = broadcast.create_track("bench", info).unwrap();
		Track { _broadcast: broadcast, track, micros: 0 }
	}

	/// A track holding one group, and that group's demand.
	pub struct Appended {
		pub track: Track,
		pub demand: moq::group::Demand,
	}

	pub fn appended() -> Appended {
		let mut track = track();
		let at = track.at();
		let demand = track.track.append_group(at).unwrap().demand();
		Appended { track, demand }
	}

	/// A track and a subscriber to it.
	pub struct Subscribed {
		track: Track,
		subscriber: moq::track::Subscriber,
	}

	pub fn subscribed() -> Subscribed {
		let track = track();
		let subscriber = resolve(track.track.consume().subscribe(None)).unwrap();
		Subscribed { track, subscriber }
	}

	/// A 100 B frame written and read back.
	pub struct Read {
		_subscribed: Subscribed,
		pub frame: moq::frame::Frame,
	}

	pub fn read() -> Read {
		let mut subscribed = subscribed();
		let frame = ready_one(&mut subscribed).unwrap();
		Read { _subscribed: subscribed, frame }
	}

	/// The `throughput` case of one frame size: its subscribed track, a frame, and the groups
	/// it writes, about 64 MiB and at least 300 frames.
	pub struct Piped {
		subscribed: Subscribed,
		frame: Vec<u8>,
		pub groups: u64,
	}

	pub fn piped(size: usize) -> Piped {
		Piped {
			subscribed: subscribed(),
			frame: vec![0u8; size],
			groups: ((64 << 20) / (size as u64 * FRAMES)).clamp(3, 2000),
		}
	}

	#[bench(setup = track)]
	pub fn call(track: &Track) {
		for _ in 0..CALLS {
			black_box(black_box(&track.track).is_closed());
		}
	}

	#[bench(setup = appended)]
	pub fn getter(appended: &Appended) {
		for _ in 0..CALLS {
			black_box(black_box(&appended.demand).start());
		}
	}

	#[bench(setup = appended)]
	pub fn record(appended: &Appended) {
		for _ in 0..CALLS {
			black_box(black_box(&appended.track.track).largest());
		}
	}

	/// A borrowed `&Info` cloned, as the bindings return an owned copy.
	#[bench(setup = appended)]
	pub fn object(appended: &Appended) {
		for _ in 0..CALLS {
			black_box(black_box(&appended.track.track).info().clone());
		}
	}

	#[bench(setup = track)]
	pub fn stamped(track: &mut Track) {
		let frame = [0u8; 100];
		for _ in 0..GROUPS {
			let start = track.micros;
			let mut group = track.track.append_group(moq::Timestamp::from_micros(start).unwrap()).unwrap();
			for _ in 0..FRAMES {
				group.write_frame(track.at(), frame.as_slice()).unwrap();
			}
			group.finish().unwrap();
		}
	}

	/// One timestamp per group, every frame of it stamped alike.
	#[bench(setup = track)]
	pub fn write(track: &mut Track) {
		let frame = [0u8; 100];
		for _ in 0..GROUPS {
			let at = track.at();
			let mut group = track.track.append_group(at).unwrap();
			for _ in 0..FRAMES {
				group.write_frame(at, frame.as_slice()).unwrap();
			}
			group.finish().unwrap();
		}
	}

	/// A group of [`FRAMES`] 100 B frames, each written then read through a new future ready
	/// at its first poll, and the last frame read.
	fn ready_one(subscribed: &mut Subscribed) -> Option<moq::frame::Frame> {
		let frame = [0u8; 100];
		let at = subscribed.track.at();
		let mut group = subscribed.track.track.append_group(at).unwrap();
		let mut reader = resolve(subscribed.subscriber.recv_group()).unwrap().unwrap();
		let mut last = None;
		for _ in 0..FRAMES {
			group.write_frame(at, frame.as_slice()).unwrap();
			last = resolve(reader.read_frame()).unwrap();
		}
		group.finish().unwrap();
		last
	}

	#[bench(setup = subscribed)]
	pub fn ready(subscribed: &mut Subscribed) -> Option<moq::frame::Frame> {
		let mut last = None;
		for _ in 0..GROUPS {
			last = ready_one(subscribed);
		}
		last
	}

	#[bench(setup = subscribed)]
	pub fn woken(subscribed: &mut Subscribed) {
		let frame = [0u8; 100];
		for _ in 0..GROUPS {
			let at = subscribed.track.at();
			let mut group = subscribed.track.track.append_group(at).unwrap();
			let mut reader = resolve(subscribed.subscriber.recv_group()).unwrap().unwrap();
			for _ in 0..FRAMES {
				let mut reading = pin!(reader.read_frame());
				let mut cx = Context::from_waker(Waker::noop());
				assert!(reading.as_mut().poll(&mut cx).is_pending());
				group.write_frame(at, frame.as_slice()).unwrap();
				let Poll::Ready(read) = reading.poll(&mut cx) else { panic!("pending") };
				black_box(read.unwrap());
			}
			group.finish().unwrap();
		}
	}

	/// The payload as a binding hands it over: a new owner of the frame's bytes.
	#[bench(setup = read)]
	pub fn payload(read: &Read) {
		for _ in 0..CALLS {
			black_box(black_box(&read.frame).payload.clone());
		}
	}

	/// Groups of [`FRAMES`] frames written and each read back, its payload's first byte
	/// touched: the sum of those bytes.
	#[bench(args = SIZES, setup = piped)]
	pub fn throughput(piped: &mut Piped) -> u64 {
		let mut sum = 0u64;
		for _ in 0..piped.groups {
			let at = piped.subscribed.track.at();
			let mut group = piped.subscribed.track.track.append_group(at).unwrap();
			let mut reader = resolve(piped.subscribed.subscriber.recv_group()).unwrap().unwrap();
			for _ in 0..FRAMES {
				group.write_frame(at, piped.frame.as_slice()).unwrap();
				let read = resolve(reader.read_frame()).unwrap().unwrap();
				sum += read.payload[0] as u64;
			}
			group.finish().unwrap();
		}
		sum
	}
}

/// Each case's wall and CPU nanoseconds per call, or for throughput, megabytes a wall and a
/// CPU second.
fn round() -> Vec<(String, (f64, f64))> {
	let mut out = Vec::new();

	let track = rust::track();
	out.push(("call".into(), per_call(|| rust::call(&track))));
	drop(track);

	let appended = rust::appended();
	out.push(("getter".into(), per_call(|| rust::getter(&appended))));

	// The handle call and the getter from THREADS threads: on one shared value, then on a
	// clone each, as the bindings' own-handle cases do. Valgrind runs one thread at a time,
	// which contends for nothing, so only this timed round has them.
	let n = REPEATS * CALLS;
	let is_closed = |track: &moq::track::Producer| {
		black_box(track.is_closed());
	};
	let sequence = |demand: &moq::group::Demand| {
		black_box(demand.start());
	};
	let (track, demand) = (&appended.track.track, &appended.demand);
	let tracks: Vec<_> = (0..THREADS).map(|_| track.clone()).collect();
	let demands: Vec<_> = (0..THREADS).map(|_| demand.clone()).collect();
	out.push(("shared:call".into(), concurrent(&[track; THREADS], n, is_closed)));
	out.push(("own:call".into(), concurrent(&tracks.iter().collect::<Vec<_>>(), n, is_closed)));
	out.push(("shared:getter".into(), concurrent(&[demand; THREADS], n, sequence)));
	out.push(("own:getter".into(), concurrent(&demands.iter().collect::<Vec<_>>(), n, sequence)));
	drop((tracks, demands));

	out.push(("record".into(), per_call(|| rust::record(&appended))));
	out.push(("object".into(), per_call(|| rust::object(&appended))));
	drop(appended);

	// Each case that writes groups writes them to a track of its own, dropped once it ends:
	// a round holds one case's frames at a time instead of every case's, about 2 GB.
	let mut track = rust::track();
	out.push(("stamped".into(), per_call(|| rust::stamped(&mut track))));
	drop(track);

	let mut track = rust::track();
	out.push(("write".into(), per_call(|| rust::write(&mut track))));
	drop(track);

	let mut subscribed = rust::subscribed();
	out.push(("ready".into(), per_call(|| drop(rust::ready(&mut subscribed)))));
	drop(subscribed);

	let mut subscribed = rust::subscribed();
	out.push(("woken".into(), per_call(|| rust::woken(&mut subscribed))));
	drop(subscribed);

	let read = rust::read();
	out.push(("payload".into(), per_call(|| rust::payload(&read))));
	drop(read);

	for size in SIZES {
		let mut piped = rust::piped(size);
		let start = now();
		black_box(rust::throughput(&mut piped));
		let (wall, cpu) = elapsed(start);
		let bytes = (piped.groups * FRAMES * size as u64) as f64;
		out.push((format!("mbps:{size}"), (bytes * 1e3 / wall, bytes * 1e3 / cpu)));
	}
	out
}

fn main() {
	// `just ffi count` sets it: each case runs once, under Valgrind, for rust2-bench to count.
	if std::env::var_os("RUST2_CODSPEED").is_some() {
		rust2_bench_harness::main();
		return;
	}
	// A round to warm up, then one to print.
	round();
	for (case, (wall, cpu)) in round() {
		println!("{case}\t{wall}\t{cpu}");
	}
}
