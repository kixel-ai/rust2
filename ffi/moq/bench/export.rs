// The Rust floor: every case the bindings time, called on the stand-in's API directly, as a
// Rust caller holding a byte slice would. Futures are polled by hand with a no-op waker,
// so a woken read costs a re-poll and nothing of an executor. Each case prints its wall
// time and the process's CPU time.

use std::future::Future;
use std::hint::black_box;
use std::pin::pin;
use std::sync::Barrier;
use std::task::{Context, Poll, Waker};
use std::time::Instant;

const GROUPS: u64 = 10000;
const FRAMES: u64 = 100;
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

fn begin() -> Start {
	Start { wall: Instant::now(), cpu: cpu() }
}

/// Wall and CPU nanoseconds since `start`.
fn elapsed(start: Start) -> (f64, f64) {
	(start.wall.elapsed().as_nanos() as f64, cpu() - start.cpu)
}

/// Wall and CPU nanoseconds per call since `start`, over `n` calls.
fn per(start: Start, n: u64) -> (f64, f64) {
	let (wall, cpu) = elapsed(start);
	(wall / n as f64, cpu / n as f64)
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
		let begin = begin();
		go.wait();
		begin
	});
	let (wall, cpu) = elapsed(begin);
	(wall / n as f64, cpu / (n * THREADS as u64) as f64)
}

/// `future`'s output, which must be ready at its first poll.
fn ready<F: Future>(future: F) -> F::Output {
	match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
		Poll::Ready(output) => output,
		Poll::Pending => panic!("pending"),
	}
}

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
fn throughput(track: &moq::track::Producer, subscriber: &mut moq::track::Subscriber, at: moq::Timestamp, size: usize) -> (f64, f64) {
	let groups = ((64 << 20) / (size as u64 * FRAMES)).clamp(3, 2000);
	let frame = vec![0u8; size];
	let mut sum = 0u64;
	let start = begin();
	for _ in 0..groups {
		let mut group = track.append_group().unwrap();
		let mut reader = ready(subscriber.recv_group()).unwrap().unwrap();
		for _ in 0..FRAMES {
			group.write_frame(at, frame.as_slice()).unwrap();
			let read = ready(reader.read_frame()).unwrap().unwrap();
			sum += read.payload[0] as u64;
		}
		group.finish().unwrap();
	}
	black_box(sum);
	let bytes = (groups * FRAMES * size as u64) as f64;
	let (wall, cpu) = elapsed(start);
	(bytes * 1e3 / wall, bytes * 1e3 / cpu)
}

fn round() -> Vec<(String, (f64, f64))> {
	let broadcast = moq::broadcast::Info::new().produce();
	let info = moq::track::Info::default().with_timescale(moq::Timescale::MICRO);
	let track = broadcast.create_track("bench", info).unwrap();
	let frame = [0u8; 100];
	let n = GROUPS * FRAMES;
	let mut out = Vec::new();

	let start = begin();
	for _ in 0..n {
		black_box(black_box(&track).is_closed());
	}
	out.push(("call".into(), per(start, n)));

	let demand = track.append_group().unwrap().demand();
	let start = begin();
	for _ in 0..n {
		black_box(black_box(&demand).sequence());
	}
	out.push(("getter".into(), per(start, n)));

	// The handle call and the getter from THREADS threads: on one shared value, then on a
	// clone each, as the bindings' own-handle cases do.
	let is_closed = |track: &moq::track::Producer| {
		black_box(track.is_closed());
	};
	let sequence = |demand: &moq::group::Demand| {
		black_box(demand.sequence());
	};
	let tracks: Vec<_> = (0..THREADS).map(|_| track.clone()).collect();
	let demands: Vec<_> = (0..THREADS).map(|_| demand.clone()).collect();
	out.push(("shared:call".into(), concurrent(&[&track; THREADS], n, is_closed)));
	out.push(("own:call".into(), concurrent(&tracks.iter().collect::<Vec<_>>(), n, is_closed)));
	out.push(("shared:getter".into(), concurrent(&[&demand; THREADS], n, sequence)));
	out.push(("own:getter".into(), concurrent(&demands.iter().collect::<Vec<_>>(), n, sequence)));

	let start = begin();
	for _ in 0..n {
		black_box(black_box(&track).largest());
	}
	out.push(("record".into(), per(start, n)));

	// A borrowed `&Info` cloned, as the bindings return an owned copy.
	let start = begin();
	for _ in 0..n {
		black_box(black_box(&track).info().clone());
	}
	out.push(("object".into(), per(start, n)));

	let mut micros = 0;
	let start = begin();
	for _ in 0..GROUPS {
		let mut group = track.append_group().unwrap();
		for _ in 0..FRAMES {
			let at = moq::Timestamp::from_micros(micros).unwrap();
			micros += 1;
			group.write_frame(at, frame.as_slice()).unwrap();
		}
		group.finish().unwrap();
	}
	out.push(("stamped".into(), per(start, n)));

	let at = moq::Timestamp::from_micros(micros).unwrap();
	let start = begin();
	for _ in 0..GROUPS {
		let mut group = track.append_group().unwrap();
		for _ in 0..FRAMES {
			group.write_frame(at, frame.as_slice()).unwrap();
		}
		group.finish().unwrap();
	}
	out.push(("write".into(), per(start, n)));

	let mut subscriber = ready(track.consume().subscribe(None)).unwrap();
	let mut last = None;
	let start = begin();
	for _ in 0..GROUPS {
		let mut group = track.append_group().unwrap();
		let mut reader = ready(subscriber.recv_group()).unwrap().unwrap();
		for _ in 0..FRAMES {
			group.write_frame(at, frame.as_slice()).unwrap();
			last = ready(reader.read_frame()).unwrap();
		}
		group.finish().unwrap();
	}
	out.push(("ready".into(), per(start, n)));

	let start = begin();
	for _ in 0..GROUPS {
		let mut group = track.append_group().unwrap();
		let mut reader = ready(subscriber.recv_group()).unwrap().unwrap();
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
	out.push(("woken".into(), per(start, n)));

	// The payload as a binding hands it over: a new owner of the frame's bytes.
	let last = last.unwrap();
	let start = begin();
	for _ in 0..n {
		black_box(black_box(&last).payload.clone());
	}
	out.push(("payload".into(), per(start, n)));

	for size in SIZES {
		out.push((format!("mbps:{size}"), throughput(&track, &mut subscriber, at, size)));
	}
	out
}

fn main() {
	round();
	for (case, (wall, cpu)) in round() {
		println!("{case}\t{wall}\t{cpu}");
	}
}
