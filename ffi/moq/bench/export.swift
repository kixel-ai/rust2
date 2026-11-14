// bench/export.c's cases through the generated Swift, so the two differ by what the Swift
// wrapper adds. Each round runs twice, the first to warm up. Each case prints its wall time
// and the process's CPU time, the global executor's threads included.

import Foundation
import Moq

let groups = 10000
let frames = 100
let n = groups * frames
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

/// The async cases' spans, the last frame read, and their reads' sum, which a tuple cannot
/// hold.
struct Reads: ~Copyable {
	var out: [(String, Span)]
	var last: FrameFrame
	var sum: Int
}

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
func throughput(_ track: TrackProducer, _ subscriber: inout TrackSubscriber, _ at: Timestamp, _ size: Int) async throws -> (Span, Int) {
	let count = min(max((64 << 20) / (size * frames), 3), 2000)
	let frame = Bytes(Data(count: size))
	var sum = 0
	let start = now()
	for _ in 0..<count {
		var group = try track.appendGroup()
		var reader = try await subscriber.recvGroup()!
		for _ in 0..<frames {
			try group.writeFrame(at, frame)
			sum += Int(try await reader.readFrame()!.payloadGet[0])
		}
		try group.finish()
	}
	let ns = per(start, Double(count * frames * size))
	return (Span(wall: 1e3 / ns.wall, cpu: 1e3 / ns.cpu), sum)
}

func round() throws -> ([(String, Span)], Int) {
	var out: [(String, Span)] = []
	let broadcast = try BroadcastInfo.new().produce()
	let track = try broadcast.createTrack("bench", TrackInfo.default().withTimescale(Timescale.MICRO))
	let frame = Bytes(Data(count: 100))
	var sum = 0

	var start = now()
	for _ in 0..<n where try track.isClosed() {
		sum += 1
	}
	out.append(("call", per(start)))

	var demanded = try track.appendGroup()
	let demand = try demanded.demand()
	start = now()
	for _ in 0..<n {
		sum &+= Int(try demand.sequence())
	}
	out.append(("getter", per(start)))
	try demanded.finish()

	start = now()
	for _ in 0..<n where try track.largest() != nil {
		sum += 1
	}
	out.append(("record", per(start)))

	start = now()
	for _ in 0..<n {
		_ = try track.info()
	}
	out.append(("object", per(start)))

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	var micros: UInt64 = 0
	start = now()
	for _ in 0..<groups {
		var group = try track.appendGroup()
		for _ in 0..<frames {
			let at = try Timestamp.fromMicros(micros)
			micros += 1
			try group.writeFrame(at, frame)
		}
		try group.finish()
	}
	out.append(("stamped", per(start)))

	let at = try Timestamp.fromMicros(micros)
	start = now()
	for _ in 0..<groups {
		var group = try track.appendGroup()
		for _ in 0..<frames {
			try group.writeFrame(at, frame)
		}
		try group.finish()
	}
	out.append(("write", per(start)))

	let reads = block {
		var out: [(String, Span)] = []
		var subscriber = try await track.consume().subscribe(nil)

		// Each frame written, then read through an async func whose future is ready at its
		// first poll, so it never suspends.
		var start = now()
		var last: FrameFrame?
		for _ in 0..<groups {
			var group = try track.appendGroup()
			var reader = try await subscriber.recvGroup()!
			for _ in 0..<frames {
				try group.writeFrame(at, frame)
				last = try await reader.readFrame()!
			}
			try group.finish()
		}
		out.append(("ready", per(start)))

		// Each read started in a task before its frame is written, which the yield lets
		// suspend, and the write's wake resumes it on the global executor.
		start = now()
		for _ in 0..<groups {
			var group = try track.appendGroup()
			var reader = try await subscriber.recvGroup()!
			for _ in 0..<frames {
				let pending = Task { try await reader.readFrame()! }
				await Task.yield()
				try group.writeFrame(at, frame)
				_ = try await pending.value
			}
			try group.finish()
		}
		out.append(("woken", per(start)))

		// A single writer task serves the calling reader for the whole case.
		// Only one permit is outstanding. The write can race the first poll.
		start = now()
		let (permits, send) = AsyncStream<Void>.makeStream()
		let writer = Task {
			var next = permits.makeAsyncIterator()
			for _ in 0..<groups {
				var group = try track.appendGroup()
				for _ in 0..<frames {
					let permit: Void? = await next.next()
					precondition(permit != nil)
					try group.writeFrame(at, frame)
				}
				let consumed: Void? = await next.next()
				precondition(consumed != nil)
				try group.finish()
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
		out.append(("loop", per(start)))

		var sum = 0
		for size in sizes {
			let (mbps, read) = try await throughput(track, &subscriber, at, size)
			out.append(("mbps:\(size)", mbps))
			sum += read
		}
		return Reads(out: out, last: last.take()!, sum: sum)
	}
	out += reads.out
	sum += reads.sum
	let last = reads.last

	// Drain one reader on its preferred executor until it suspends before every
	// write. Unlike a yield on the global pool, an empty queue proves it parked.
	let wakeTrack = try broadcast.createTrack("wake-loop", TrackInfo.default().withTimescale(Timescale.MICRO))
	let queue = Queue()
	let consumed = Cell<Int>()
	consumed.value = 0
	let done = Cell<Bool>()
	done.value = false
	start = now()
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
		var group = try wakeTrack.appendGroup()
		queue.drain()
		for f in 0..<frames {
			precondition(consumed.value == g * frames + f)
			try group.writeFrame(at, frame)
			queue.drain()
			precondition(consumed.value == g * frames + f + 1)
		}
		try group.finish()
	}
	precondition(done.value == true)
	out.append(("drained", per(start)))

	// The last frame's 100 bytes, returned as Bytes, and one of them read.
	start = now()
	for _ in 0..<n {
		sum += try Int(last.payloadGet[0])
	}
	out.append(("payload", per(start)))
	return (out, sum)
}

var sink = 0
sink += try round().1
let (out, sum) = try round()
sink += sum
for (key, value) in out {
	print("\(key)\t\(value.wall)\t\(value.cpu)")
}
if sink == Int.min {
	print("!")
}
