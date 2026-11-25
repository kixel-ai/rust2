// bench/export.c's cases through the generated Swift, so the two differ by what the Swift
// wrapper adds. Each round runs twice, the first to warm up. Each case prints its wall time
// and the process's CPU time, the global executor's threads included, or under Valgrind,
// runs once for rust2-bench to count.

import Foundation
import Hooks
import Moq

/// Whether Valgrind counts this run, which then skips the warm-up and the threaded cases.
let measured = hooks_init("ffi/moq/bench/export.swift::swift")
/// A tenth under Valgrind, which counts every call exactly.
let groups = measured ? 1000 : 10000
let frames = 100
let n = groups * frames
let threads = 4
let sizes = [100, 1024, 16384, 65536, 1_048_576]

/// Wall and CPU nanoseconds: a point, or a span per call.
struct Span: Sendable {
	var wall: Double
	var cpu: Double
}

/// `clock` in nanoseconds.
func nanoseconds(_ clock: clockid_t) -> Double {
	var ts = timespec()
	clock_gettime(clock, &ts)
	return Double(ts.tv_sec) * 1e9 + Double(ts.tv_nsec)
}

func now() -> Span {
	Span(wall: nanoseconds(CLOCK_MONOTONIC), cpu: nanoseconds(CLOCK_PROCESS_CPUTIME_ID))
}

/// Nanoseconds per call since `start`, over `calls` calls.
func per(_ start: Span, _ calls: Double = Double(n)) -> Span {
	let end = now()
	return Span(wall: (end.wall - start.wall) / calls, cpu: (end.cpu - start.cpu) / calls)
}

/// A case's start: now, and Valgrind counting from here.
func begin() -> Span {
	let start = now()
	hooks_start()
	return start
}

/// The case `name`'s nanoseconds per call since `start`, over `calls` calls.
func end(_ start: Span, _ name: String, _ calls: Double = Double(n)) -> Span {
	hooks_stop(name)
	return per(start, calls)
}

/// A value a task sets and this thread takes once the task signals.
final class Cell<T: ~Copyable>: @unchecked Sendable {
	var value: T?
}

/// `body`'s result, waiting on this thread for it to finish on the global executor.
func block<T: ~Copyable>(_ body: @escaping @Sendable () async throws -> T) -> T {
	let done = DispatchSemaphore(value: 0)
	let result = Cell<T>()
	Task {
		result.value = try! await body()
		done.signal()
	}
	done.wait()
	return result.value.take()!
}

/// A queue of jobs that a test runs when it chooses: the executor its tasks prefer, so a
/// call resumes on it.
final class Queue: TaskExecutor, @unchecked Sendable {
	private let lock = NSLock()
	private var jobs: [UnownedJob] = []

	func enqueue(_ job: UnownedJob) {
		lock.lock()
		defer { lock.unlock() }
		jobs.append(job)
	}

	func drain() {
		while true {
			lock.lock()
			let job = jobs.isEmpty ? nil : jobs.removeFirst()
			lock.unlock()
			guard let job else { return }
			job.runSynchronously(on: asUnownedTaskExecutor())
		}
	}

}

/// A value threads share: a noncopyable object a closure can only borrow through a class.
final class Shared<T: ~Copyable & Sendable>: Sendable {
	let value: T
	init(_ value: consuming T) {
		self.value = value
	}
}

/// Each of `calls` made n times on a thread of its own, all at once: wall nanoseconds per
/// call on one thread, and CPU nanoseconds per call.
func concurrent(_ calls: [@Sendable () -> Int]) -> Span {
	let ready = DispatchSemaphore(value: 0)
	let go = DispatchSemaphore(value: 0)
	let done = DispatchSemaphore(value: 0)
	for call in calls {
		Thread {
			var sum = 0
			ready.signal()
			go.wait()
			for _ in 0..<n {
				sum &+= call()
			}
			blackHole(sum)
			done.signal()
		}.start()
	}
	for _ in calls {
		ready.wait()
	}
	let start = now()
	for _ in calls {
		go.signal()
	}
	for _ in calls {
		done.wait()
	}
	let span = per(start)
	return Span(wall: span.wall, cpu: span.cpu / Double(calls.count))
}

/// Keeps `value` from being optimized away.
@inline(never)
func blackHole(_ value: Int) {
	if value == Int.min {
		print("")
	}
}

/// The async cases' spans, the last frame read, their reads' sum, and the round's next
/// microsecond, which a tuple cannot hold.
struct Reads: ~Copyable {
	var out: [(String, Span)]
	var last: FrameFrame
	var sum: Int
	var micros: UInt64
}

/// A timed track named `name` on `broadcast`. It caches every group written to it until it is
/// freed, as the broadcast's pool, which nothing polls, evicts none, so each case that writes
/// groups writes them to a track of its own, freed once it ends: a round holds one case's
/// frames at a time instead of every case's, about 2 GB.
func createTrack(_ broadcast: borrowing BroadcastProducer, _ name: String) throws -> TrackProducer {
	try broadcast.createTrack(name, TrackInfo.default().withTimescale(Timescale.MICRO))
}

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
func throughput(_ broadcast: borrowing BroadcastProducer, _ micros: inout UInt64, _ size: Int) async throws -> (Span, Int) {
	let count = min(max((64 << 20) / (size * frames), 3), 2000)
	let frame = Bytes(Data(count: size))
	let name = "throughput[\(size)]"
	let track = try createTrack(broadcast, name)
	var subscriber = try await track.consume().subscribe(nil)
	var sum = 0
	let start = begin()
	for _ in 0..<count {
		let at = try Timestamp.fromMicros(micros)
		micros += 1
		var group = try track.appendGroup(at)
		var reader = try await subscriber.recvGroup()!
		for _ in 0..<frames {
			try frame.withSpan { try group.writeFrame(at, $0) }
			sum += Int(try await reader.readFrame()!.payloadGet[0])
		}
		_ = try group.finish()
	}
	let ns = end(start, name, Double(count * frames * size))
	return (Span(wall: 1e3 / ns.wall, cpu: 1e3 / ns.cpu), sum)
}

func round(_ measured: Bool) throws -> ([(String, Span)], Int) {
	var out: [(String, Span)] = []
	// Shared, so the async cases make their tracks on it from their closure.
	let broadcast = Shared(try BroadcastInfo.new().produce())
	let track = try createTrack(broadcast.value, "bench")
	let frame = Bytes(Data(count: 100))
	var sum = 0
	// A group starts at its first frame's time, and a track's starts must increase, so
	// every case's groups take theirs from one running count.
	var micros: UInt64 = 0

	var start = begin()
	for _ in 0..<n where try track.isClosed() {
		sum += 1
	}
	out.append(("call", end(start, "call")))

	let demanded = try track.appendGroup(Timestamp.fromMicros(micros))
	micros += 1
	let demand = try demanded.demand()
	start = begin()
	for _ in 0..<n {
		sum &+= Int(try demand.start())
	}
	out.append(("getter", end(start, "getter")))

	// The handle call and the getter from `threads` threads: on one shared handle, then on a
	// clone each, a handle of its own to the same value. Valgrind runs one thread
	// at a time, which contends for nothing, so it skips them.
	if !measured {
		let isClosed = { (t: TrackProducer) -> @Sendable () -> Int in { try! t.isClosed() ? 1 : 0 } }
		let groupStart = { (d: Shared<GroupDemand>) -> @Sendable () -> Int in { Int(truncatingIfNeeded: try! d.value.start()) } }
		let shared = Shared(try demand.clone())
		out.append(("shared:call", concurrent((0..<threads).map { _ in isClosed(track) })))
		out.append(("own:call", concurrent((0..<threads).map { _ in isClosed(try! track.clone()) })))
		out.append(("shared:getter", concurrent((0..<threads).map { _ in groupStart(shared) })))
		out.append(("own:getter", concurrent((0..<threads).map { _ in groupStart(Shared(try! demand.clone())) })))
	}
	_ = try demanded.finish()

	start = begin()
	for _ in 0..<n where try track.largest() != nil {
		sum += 1
	}
	out.append(("record", end(start, "record")))

	start = begin()
	for _ in 0..<n {
		_ = try track.info()
	}
	out.append(("object", end(start, "object")))

	// Each frame stamped through a fresh timestamp, as a caller holding a u64 must.
	do {
		let track = try createTrack(broadcast.value, "stamped")
		start = begin()
		for _ in 0..<groups {
			var group = try track.appendGroup(Timestamp.fromMicros(micros))
			for _ in 0..<frames {
				let at = try Timestamp.fromMicros(micros)
				micros += 1
				try frame.withSpan { try group.writeFrame(at, $0) }
			}
			_ = try group.finish()
		}
		out.append(("stamped", end(start, "stamped")))
	}

	// The rest stamp a group's frames with its start, converted once a group.
	do {
		let track = try createTrack(broadcast.value, "write")
		start = begin()
		for _ in 0..<groups {
			let at = try Timestamp.fromMicros(micros)
			micros += 1
			var group = try track.appendGroup(at)
			for _ in 0..<frames {
				try frame.withSpan { try group.writeFrame(at, $0) }
			}
			_ = try group.finish()
		}
		out.append(("write", end(start, "write")))
	}

	let base = micros
	let reads = block {
		var out: [(String, Span)] = []
		var micros = base

		// Each frame written, then read through an async func whose future is ready at its
		// first poll, so it never suspends.
		var start: Span
		var last: FrameFrame?
		do {
			let track = try createTrack(broadcast.value, "ready")
			var subscriber = try await track.consume().subscribe(nil)
			start = begin()
			for _ in 0..<groups {
				let at = try Timestamp.fromMicros(micros)
				micros += 1
				var group = try track.appendGroup(at)
				var reader = try await subscriber.recvGroup()!
				for _ in 0..<frames {
					try frame.withSpan { try group.writeFrame(at, $0) }
					last = try await reader.readFrame()!
				}
				_ = try group.finish()
			}
			out.append(("ready", end(start, "ready")))
		}

		// Each read started in a task before its frame is written, which the yield lets
		// suspend, and the write's wake resumes it on the global executor.
		do {
			let track = try createTrack(broadcast.value, "woken")
			var subscriber = try await track.consume().subscribe(nil)
			start = begin()
			for _ in 0..<groups {
				let at = try Timestamp.fromMicros(micros)
				micros += 1
				var group = try track.appendGroup(at)
				var reader = try await subscriber.recvGroup()!
				for _ in 0..<frames {
					let pending = Task { _ = try await reader.readFrame()! }
					await Task.yield()
					try frame.withSpan { try group.writeFrame(at, $0) }
					try await pending.value
				}
				_ = try group.finish()
			}
			out.append(("woken", end(start, "woken")))
		}

		// A single writer task serves the calling reader for the whole case.
		// Only one permit is outstanding. The write can race the first poll.
		do {
			let track = try createTrack(broadcast.value, "loop")
			var subscriber = try await track.consume().subscribe(nil)
			start = begin()
			let (permits, send) = AsyncStream<Void>.makeStream()
			// The writer task takes its groups' starts from a copy, which it cannot share.
			let first = micros
			micros += UInt64(groups)
			let writer = Task {
				var next = permits.makeAsyncIterator()
				for g in 0..<groups {
					let at = try Timestamp.fromMicros(first + UInt64(g))
					var group = try track.appendGroup(at)
					for _ in 0..<frames {
						let permit: Void? = await next.next()
						precondition(permit != nil)
						try frame.withSpan { try group.writeFrame(at, $0) }
					}
					let consumed: Void? = await next.next()
					precondition(consumed != nil)
					_ = try group.finish()
				}
			}
			for _ in 0..<groups {
				var reader = try await subscriber.recvGroup()!
				for _ in 0..<frames {
					send.yield(())
					_ = try await reader.readFrame()!
				}
				send.yield(())
			}
			send.finish()
			try await writer.value
			out.append(("loop", end(start, "loop")))
		}

		var sum = 0
		for size in sizes {
			let (mbps, read) = try await throughput(broadcast.value, &micros, size)
			out.append(("mbps:\(size)", mbps))
			sum += read
		}
		return Reads(out: out, last: last.take()!, sum: sum, micros: micros)
	}
	out += reads.out
	sum += reads.sum
	micros = reads.micros
	let last = reads.last

	// Drain one reader on its preferred executor until it suspends before every
	// write. Unlike a yield on the global pool, an empty queue proves it parked.
	let wakeTrack = try createTrack(broadcast.value, "wake-loop")
	let queue = Queue()
	let consumed = Cell<Int>()
	consumed.value = 0
	let done = Cell<Bool>()
	done.value = false
	start = begin()
	Task(executorPreference: queue) {
		var subscriber = try! await wakeTrack.consume().subscribe(nil)
		for _ in 0..<groups {
			var reader = try! await subscriber.recvGroup()!
			for _ in 0..<frames {
				_ = try! await reader.readFrame()!
				consumed.value! += 1
			}
		}
		done.value = true
	}
	queue.drain()
	for g in 0..<groups {
		let at = try Timestamp.fromMicros(micros)
		micros += 1
		var group = try wakeTrack.appendGroup(at)
		queue.drain()
		for f in 0..<frames {
			precondition(consumed.value == g * frames + f)
			try frame.withSpan { try group.writeFrame(at, $0) }
			queue.drain()
			precondition(consumed.value == g * frames + f + 1)
		}
		_ = try group.finish()
	}
	precondition(done.value == true)
	out.append(("drained", end(start, "drained")))

	// The last frame's 100 bytes, returned as Bytes, and one of them read.
	start = begin()
	for _ in 0..<n {
		sum += try Int(last.payloadGet[0])
	}
	out.append(("payload", end(start, "payload")))
	return (out, sum)
}

// Under Valgrind, one round, which prints nothing: rust2-bench counts each case.
var sink = 0
if !measured {
	sink += try round(false).1
}
let (out, sum) = try round(measured)
sink += sum
for (key, value) in out where !measured {
	print("\(key)\t\(value.wall)\t\(value.cpu)")
}
if sink == Int.min {
	print("!")
}
