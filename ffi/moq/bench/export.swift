// bench/export.c's cases through the generated Swift, so the two differ by what the Swift
// wrapper adds. Each round runs twice, the first to warm up.

import Foundation
import Moq

let groups = 10000
let frames = 100
let n = groups * frames
let sizes = [100, 1024, 16384, 65536, 1_048_576]

/// Nanoseconds now.
func now() -> UInt64 {
	DispatchTime.now().uptimeNanoseconds
}

/// Nanoseconds per call since `start`, over `calls` calls.
func per(_ start: UInt64, _ calls: Int = n) -> Double {
	Double(now() - start) / Double(calls)
}

/// A value a task sets and this thread reads once the task signals.
final class Cell<T>: @unchecked Sendable {
	var value: T?
}

/// `body`'s result, waiting on this thread for it to finish on the global executor.
func block<T: Sendable>(_ body: @escaping @Sendable () async throws -> T) -> T {
	let done = DispatchSemaphore(value: 0)
	let result = Cell<T>()
	Task {
		result.value = try! await body()
		done.signal()
	}
	done.wait()
	return result.value!
}

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a second.
func throughput(_ track: TrackProducer, _ subscriber: TrackSubscriber, _ at: Timestamp, _ size: Int) async throws -> (Double, Int) {
	let count = min(max((64 << 20) / (size * frames), 3), 2000)
	let frame = Data(count: size)
	var sum = 0
	let start = now()
	for _ in 0..<count {
		let group = try track.appendGroup()
		let reader = try await subscriber.recvGroup()!
		for _ in 0..<frames {
			try group.writeFrame(at, frame)
			sum += Int(try await reader.readFrame()!.payloadGet[0])
		}
		try group.finish()
	}
	return (Double(count * frames * size) * 1e3 / Double(now() - start), sum)
}

func round() throws -> ([(String, Double)], Int) {
	var out: [(String, Double)] = []
	let broadcast = BroadcastInfo.new().produce()
	let track = try broadcast.createTrack("bench", TrackInfo.default().withTimescale(Timescale.MICRO))
	let frame = Data(count: 100)
	var sum = 0

	var start = now()
	for _ in 0..<n where track.isClosed() {
		sum += 1
	}
	out.append(("call", per(start)))

	let demanded = try track.appendGroup()
	let demand = demanded.demand()
	start = now()
	for _ in 0..<n {
		sum &+= Int(demand.sequence())
	}
	out.append(("getter", per(start)))
	try demanded.finish()

	start = now()
	for _ in 0..<n where track.largest() != nil {
		sum += 1
	}
	out.append(("record", per(start)))

	start = now()
	for _ in 0..<n {
		_ = track.info()
	}
	out.append(("object", per(start)))

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	var micros: UInt64 = 0
	start = now()
	for _ in 0..<groups {
		let group = try track.appendGroup()
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
		let group = try track.appendGroup()
		for _ in 0..<frames {
			try group.writeFrame(at, frame)
		}
		try group.finish()
	}
	out.append(("write", per(start)))

	let (reads, last, readSum) = block {
		var out: [(String, Double)] = []
		let subscriber = try await track.consume().subscribe(nil)

		// Each frame written, then read through an async func whose future is ready at its
		// first poll, so it never suspends.
		var start = now()
		var last: FrameFrame?
		for _ in 0..<groups {
			let group = try track.appendGroup()
			let reader = try await subscriber.recvGroup()!
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
			let group = try track.appendGroup()
			let reader = try await subscriber.recvGroup()!
			for _ in 0..<frames {
				let pending = Task { try await reader.readFrame()! }
				await Task.yield()
				try group.writeFrame(at, frame)
				_ = try await pending.value
			}
			try group.finish()
		}
		out.append(("woken", per(start)))

		var sum = 0
		for size in sizes {
			let (mbps, read) = try await throughput(track, subscriber, at, size)
			out.append(("mbps:\(size)", mbps))
			sum += read
		}
		return (out, last!, sum)
	}
	out += reads
	sum += readSum

	// The last frame's 100 bytes, copied into a Data, and one of them read.
	start = now()
	for _ in 0..<n {
		sum += Int(last.payloadGet[0])
	}
	out.append(("payload", per(start)))
	return (out, sum)
}

var sink = 0
sink += try round().1
let (out, sum) = try round()
sink += sum
for (key, value) in out {
	print("\(key)\t\(value)")
}
if sink == Int.min {
	print("!")
}
