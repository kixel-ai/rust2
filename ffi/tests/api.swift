// The api fixture's Swift test: the script fixtures/api/examples/transcript.rs runs
// natively, through the generated Swift, which tests/fixtures/transcript writes as
// transcript.swift's `transcript()`. tests/fixtures.rs compares the transcripts. Each
// function transcript.rs defines is this file's, by the same name.
//
// With an argument, it instead checks what only Swift says, the futures' semantics, what
// releasing frees, or breaks a contract, which must fail.

import Api
import BasicContainers
import Foundation
#if os(Linux)
import NoDump
#endif

// The Log callback: keeps even levels.
final class Print: Log {
	func log(_ level: UInt8, _ line: String) -> Bool {
		print("log \(level) \(line)")
		return level % 2 == 0
	}

	// Swift frees it once Rust drops the implementation.
	deinit {
		print("freed")
	}
}


/// A Log that counts its frees.
final class DropLog: Log {
	let drops: Cell<Int>

	init(_ drops: Cell<Int>) {
		self.drops = drops
	}

	func log(_: UInt8, _: String) -> Bool {
		true
	}

	deinit {
		drops.value += 1
	}
}

/// A Log that cancels a task at each line, and counts them.
final class CancelLog: Log, @unchecked Sendable {
	let task: Cell<Task<Void, Swift.Error>?>
	let lines = Cell(0)

	init(_ task: Cell<Task<Void, Swift.Error>?>) {
		self.task = task
	}

	func log(_: UInt8, _: String) -> Bool {
		task.value?.cancel()
		lines.value += 1
		return true
	}
}

func text(_ data: Bytes) -> String {
	String(decoding: data, as: UTF8.self)
}

/// Check that `body` throws for the call `function` of a Refusal a move closed.
func closed(_ function: String, _ body: () throws -> Void) {
	do {
		try body()
		check(false, "\(function) of a closed Refusal throws")
	} catch let e as CheckError {
		check(e.function == function && e.message == "Refusal is closed", "\(function) threw \(e)")
	} catch {
		check(false, "\(function) threw \(error)")
	}
}

/// Check that `body` throws the CheckError of `function`, which the library's target lacks.
func lacks(_ function: String, _ body: () throws -> Void) {
	do {
		try body()
		check(false, "\(function) throws off its target")
	} catch let e as CheckError {
		check(
			e.function == function && e.message == "it is not on this library's target",
			"\(function) threw \(e)"
		)
	} catch {
		check(false, "\(function) threw \(error)")
	}
}

/// The file serves wasm32 too: what only wasm32 has throws on the host's library, wherever
/// Swift lets it.
func targets() {
	lacks("api_wasm_only") { _ = try wasmOnly() }
	lacks("api_Page_default") { _ = try Page.default() }
	let booth = Booth(mark: Mark.of(1))
	lacks("api_Booth_debug") { _ = try booth.debugDescription }
	lacks("api_Booth_eq") { _ = try booth == booth }
}

/// The refusal `refuse` throws for `reason`: a class, since Swift throws it.
func refusal(_ reason: String) -> Refusal {
	do {
		_ = try refuse(reason)
	} catch let refusal as Refusal {
		return refusal
	} catch {}
	fatalError("refuse returned")
}

/// A path's bytes as Rust's `escape_ascii` prints them.
func escaped(_ path: PathBuf) -> String {
	var out = ""
	for b in path.bytes {
		switch b {
		case 0x09: out += "\\t"
		case 0x0d: out += "\\r"
		case 0x0a: out += "\\n"
		case 0x5c, 0x27, 0x22: out += "\\" + String(UnicodeScalar(b))
		case 0x20..<0x7f: out += String(UnicodeScalar(b))
		default: out += "\\x" + String(b, radix: 16).leftPadded(2)
		}
	}
	return out
}

extension String {
	func leftPadded(_ width: Int) -> String {
		String(repeating: "0", count: max(0, width - count)) + self
	}
}

func check(_ ok: Bool, _ what: String) {
	if !ok {
		FileHandle.standardError.write(Data("failed: \(what)\n".utf8))
		exit(1)
	}
}

/// A value a task sets and the test reads, under a lock.
final class Cell<T>: @unchecked Sendable {
	private let lock = NSLock()
	private var stored: T

	init(_ value: T) {
		stored = value
	}

	var value: T {
		get {
			lock.lock()
			defer { lock.unlock() }
			return stored
		}
		set {
			lock.lock()
			defer { lock.unlock() }
			stored = newValue
		}
	}
}

/// A queue of jobs that a test runs when it chooses: the executor its tasks prefer, so a
/// call resumes on it.
/// A value that is not Send, which a test hands another thread to misuse.
final class Held: @unchecked Sendable {
	let local: Local

	init(_ local: consuming Local) {
		self.local = local
	}
}

final class Queue: TaskExecutor, @unchecked Sendable {
	private let lock = NSLock()
	private var jobs: [UnownedJob] = []

	func enqueue(_ job: UnownedJob) {
		lock.lock()
		defer { lock.unlock() }
		jobs.append(job)
	}

	var size: Int {
		lock.lock()
		defer { lock.unlock() }
		return jobs.count
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

	/// Run `body` as a task that prefers this queue.
	@discardableResult
	func launch(_ body: @escaping @Sendable () async throws -> Void) -> Task<Void, Swift.Error> {
		Task(executorPreference: self, operation: body)
	}
}

/// An actor awaiting an eager future, whose closure Rust calls before the call returns.
actor Probe {
	func count(_ n: UInt32) async throws -> UInt32 {
		try await counted(n) { _ in self.preconditionIsolated("a future runs on its caller's actor") }
	}
}

/// `body`'s result, waiting on this thread for it to finish on the global executor.
func block<T: Sendable>(_ body: @escaping @Sendable () async throws -> T) -> T {
	let done = DispatchSemaphore(value: 0)
	let result = Cell<Result<T, Swift.Error>?>(nil)
	Task {
		do {
			result.value = .success(try await body())
		} catch {
			result.value = .failure(error)
		}
		done.signal()
	}
	done.wait()
	// Moved out, since the task may release its hold on `result` only after this returns.
	let value = result.value!
	result.value = nil
	return try! value.get()
}

/// A channel's futures, on an executor the script drains, so it sees each wait and the
/// wake that queues its resume.
func channel() {
	let channel = Channel.new()
	let wakes = Cell(0)
	let q = Queue()
	let got = Cell<Bytes?>(nil)
	q.launch { got.value = try await channel.recv() }
	q.drain()
	print("recv pending \(got.value == nil)")
	try! channel.send(Bytes("hi".utf8))
	wakes.value += q.size
	print("woken \(wakes.value)")
	q.drain()
	print("recv \(text(got.value!))")
	block {
		try! channel.send(Bytes())
		do {
			print("nonempty \(try await channel.recvNonempty().count)")
		} catch {
			print("nonempty err \(error)")
		}
		var four = Batch.new(), two = Api.pair()
		let filled = try await fill4(&four, 9) + fill2(&two, 1)
		print("fill \(filled.map { Swift.String($0) }.joined(separator: " "))")
		// The send happens at the call, before the future is first polled.
		try await channel.sendLater(Bytes("eager".utf8))
		print("recv \(text(try await channel.recv()))")
		// The send happens at the call, and the future borrows the channel and the tag.
		print("tagged \(try await channel.tagged("tag"))")
		print("itemized \(try await channel.itemized([1, 2, 3]))")
		print("byte_len \(try await byteLen(Bytes("abc".utf8)))")
		try! channel.send(Bytes("sized".utf8))
		let (payload, size) = try await channel.recvSized()
		print("sized \(text(payload)) \(size)")
	}
	// The waker moves into Rust, which keeps it until the send wakes it.
	try! channel.notify { wakes.value += 1 }
	try! channel.send(Bytes("rung".utf8))
	print("woken \(wakes.value)")
	block {
		// A poll with no async twin is a future.
		print("take \(text(try await channel.pollTake()))")
		// A `&mut self` poll needs a binding of its own, which a closure cannot capture.
		var measured = Channel.new()
		try! measured.send(Bytes("measured".utf8))
		print("measure \(try await measured.pollMeasure(2))")
	}
	// A context is made of the waker, which Rust keeps until the send wakes it.
	try! channel.listen { wakes.value += 1 }
	try! channel.send(Bytes("heard".utf8))
	print("woken \(wakes.value)")
	// A returned waker wakes the waiter's task through Rust until released.
	do {
		let waiter = Waiter.new { wakes.value += 1 }
		try! waiter.waker()()
		try! waiter.context()()
	}
	print("woken \(wakes.value)")
}

/// A kept closure: called again at the instant it asks for, then once its waiter wakes.
func drive() {
	let start = Moment.new(0, 1000)!
	let calls = Cell(0)
	let stopped = block {
		try await Api.drive(start) { now, waiter in
			calls.value += 1
			switch calls.value {
			case 1: return now.after(5)
			case 2:
				print("drive \(now.millis() - start.millis())")
				try! waiter.waker()()
				return nil
			default: throw StringError(error: "stopped after \(calls.value)")
			}
		}
	}
	print("drive \(stopped)")
}

/// Readers over bytes, whose reads fail, and that seek past `Int64.max`.
func readers() {
	var memory = Memory(Data("hello world".utf8))
	printTail { try tail(&memory, 5) }
	check(memory.at == 6, "a reader's seeks stay once the call returns")
	var broken = Broken()
	printTail { try tail(&broken, 5) }
	// A seek from the start reaches the reader whole, past `Int64.max`, as `Cursor` takes one.
	var far = Far()
	do {
		print("sought \(try seekTo(&far, UInt64.max))")
	} catch let e as StringError {
		print("sought error \(e.error)")
	} catch {
		check(false, "seekTo throws its io::Error's message")
	}
}

/// Futures of a value that is not Send, made on this thread, so awaited on an executor the
/// script drains on it.
func localLater(_ local: consuming Local) {
	let held = Held(local)
	let later = Cell<UInt32>(0)
	let q = Queue()
	q.launch { later.value = try await localOrNewLater(local: held.local).bump() }
	q.drain()
	print("local later \(later.value)")
	print("local later fresh \(block { try await localOrNewLater().bump() })")
}

/// A path no Rust string spells: bytes that are not UTF-8.
func unspelled() -> PathBuf {
	stray()
}

/// A path's bytes in hex.
func pathHex(_ path: PathBuf) -> String {
	hex(path.bytes)
}

/// A duration as its seconds and nine digits of nanoseconds.
func seconds(_ d: RustDuration) -> String {
	"\(d.secs).\(String(d.nanos).leftPadded(9))"
}

/// A duration in whole milliseconds.
func millis(_ d: RustDuration) -> UInt64 {
	d.secs * 1000 + UInt64(d.nanos) / 1_000_000
}

/// A char's code point.
func code(_ c: Unicode.Scalar) -> UInt32 {
	c.value
}

/// The list reversed.
func reversed<T: ~Copyable>(_ list: consuming UniqueArray<T>) -> UniqueArray<T> {
	var out = UniqueArray<T>()
	while list.count > 0 {
		out.append(list.remove(at: list.count - 1))
	}
	return out
}

/// Each float's bits.
func f64Bits(_ floats: [Double]) -> [UInt64] {
	floats.map(\.bitPattern)
}

/// What the transcript's script says only in Rust, or Swift alone has: its own types'
/// conversions, the errors and checks it throws, and its protocols.
func checks() {
	check(U128(high: .max, low: .max).description == "340282366920938463463374607431768211455", "a u128's decimal digits")
	check(ArrayU8x4([1, 2, 3]) == nil, "3 bytes are no [u8; 4]")
	check(advance(.init_(1)) == .init_(0) && advance(.init_(0)) == .done, "a variant named init crosses")
	// Rust's seconds are a u64, past Int64's.
	let rustMax = try! RustDuration(secs: .max, nanos: 999_999_999)
	let past = RustDuration.seconds(UInt64(Int64.max) + 1)
	check(
		extended(past, try! RustDuration(secs: UInt64(Int64.max), nanos: 999_999_999)) == rustMax,
		"a duration past Int64's seconds crosses")
	check(extended(try! RustDuration(secs: .max, nanos: 999_999_996), .nanoseconds(3)) == rustMax, "Rust's longest duration crosses")
	check(extended(rustMax, .nanoseconds(1)) == nil, "Rust refuses a duration past its longest")
	durations(rustMax)
	do {
		let two = Counter.new("two", 2)
		let three = Counter.new("three", 3)
		// A list and an option holding a struct beside a class, whose handle each moves.
		var pairs = UniqueArray<TupleCounterRefusal>()
		pairs.append(TupleCounterRefusal(Counter.new("a", 1), refusal("x")))
		check(
			try! reasons(pairs, TupleCounterRefusal(Counter.new("b", 2), refusal("y")))
				== "a refused: x, b refused: y",
			"reasons moves a struct and a class out of a list and an option"
		)
		// A class a move closed throws for each later call, as a receiver or nested.
		let kept = refusal("z")
		var moved = UniqueArray<TupleCounterRefusal>()
		moved.append(TupleCounterRefusal(Counter.new("c", 3), kept))
		_ = try! reasons(moved, nil)
		closed("api_Refusal_to_string") { _ = try kept.description }
		var again = UniqueArray<TupleCounterRefusal>()
		again.append(TupleCounterRefusal(Counter.new("d", 4), kept))
		do {
			_ = try reasons(again, nil)
			check(false, "api_reasons of a closed Refusal throws")
		} catch let e as CheckError {
			check(e.function == "api_reasons" && e.message == "Refusal is closed", "api_reasons threw \(e)")
		} catch {
			check(false, "api_reasons threw \(error)")
		}
		// A call that throws before Rust runs changes none of its arguments: it closes no
		// class it would move, and writes back no value Rust would.
		let live = refusal("live")
		var mixed = UniqueArray<TupleCounterRefusal>()
		mixed.append(TupleCounterRefusal(Counter.new("e", 5), live))
		mixed.append(TupleCounterRefusal(Counter.new("f", 6), kept))
		check((try? reasons(mixed, nil)) == nil, "api_reasons of a closed Refusal beside a live one throws")
		check(try! live.description == "refused: live", "a refused call leaves the class it would move open")
		// Nor does it keep a callback it gave before the throw.
		let drops = Cell(0)
		check((try? relay(DropLog(drops), [kept])) == nil, "api_relay of a closed Refusal throws")
		check(drops.value == 1, "a call refused before Rust runs frees the callback it lowered")
		// Nor the buffers it gave Rust before the throw, alone or beside the closed class.
		check(try! excuse(tokenBytes(1, 1025), (tokenBytes(2, 1025), refusal("x"))) == 2051, "api_excuse sums its lengths")
		let given = dropped()
		do {
			let note = tokenBytes(1, 1025)
			let nested = tokenBytes(2, 1025)
			check((try? excuse(note, (nested, kept))) == nil, "api_excuse of a closed Refusal throws")
		}
		check(dropped() == given + 2, "a call refused before Rust runs frees the buffers it gave")
		var at = Point(x: 1, y: 2)
		check((try? nudgeBy(&at, kept)) == nil, "api_nudge_by of a closed Refusal throws")
		check(at == Point(x: 1, y: 2), "a refused call writes nothing back")
		try! nudgeBy(&at, live)
		check(at == Point(x: 5, y: 6), "a call writes back what Rust left")
		// Each optional borrowed parameter is an overload with and without its label.
		let one = Counter.new("one", 1)
		check(
			try! two.clamped() == 2 && two.clamped(low: three) == 3 && two.clamped(high: one) == 1
				&& two.clamped(low: one, high: three) == 2,
			"clamped overloads each optional borrow"
		)
	}
	do {
		// A draft borrows its ledger exclusively until it is consumed: the ledger refuses any
		// other use meanwhile.
		var ledger = Ledger.new()
		let draft = try! ledger.edit()
		do {
			_ = try ledger.published()
			check(false, "a lent ledger refuses a read")
		} catch let e as CheckError {
			check(
				e.function == "api_Ledger_published" && e.message == "a handle is borrowed exclusively",
				"a lent ledger threw \(e)"
			)
		} catch {
			check(false, "a lent ledger threw \(error)")
		}
		_ = consume draft
	}
	// A kept closure, called on another thread after the call that took it returned.
	check(try! Spawned.new { 7 }.join() == 7, "a kept closure runs on another thread")
	// A zero, which Swift's integer holds and Rust's `NonZero` does not, makes none, and a
	// record holding one keeps the protocols its traits give.
	check(refused("NonZero(_:)") { _ = try NonZero<UInt64>(0) }, "a zero makes no NonZero")
	check(Set([try! NonZero<Int8>(-1), try! NonZero(-1), try! NonZero(.min)]).count == 2, "a set of NonZero")
	check(try! NonZero<Int8>(.min) < NonZero(1), "a NonZero compares as its integer")
	let debuggable: any CustomDebugStringConvertible = nonZeros()
	check(debuggable.debugDescription == describeNonZeros(nonZeros()), "a record of NonZero is Debug")
	lossless()
	targets()
	print("checks ok")
}

/// Print each of `n`'s lists, its floats as their bits, as transcript.rs does.
func numbers(_ what: String, _ n: Numbers) {
	print("\(what) u16 \(n.u16s)")
	print("\(what) u32 \(n.u32s)")
	print("\(what) u64 \(n.u64s)")
	print("\(what) i8 \(n.i8s)")
	print("\(what) i16 \(n.i16s)")
	print("\(what) i32 \(n.i32s)")
	print("\(what) i64 \(n.i64s)")
	print("\(what) f32 \(n.f32s.map(\.bitPattern))")
	print("\(what) f64 \(n.f64s.map(\.bitPattern))")
	print("\(what) some \(n.some.map { "Some(\($0))" } ?? "None")")
}

func printTail(_ read: () throws -> (Bytes, UInt64)) {
	do {
		let (bytes, at) = try read()
		print("tail \(String(decoding: bytes, as: UTF8.self)) \(at)")
	} catch let e as StringError {
		print("tail error \(e.error)")
	} catch {
		check(false, "tail throws its io::Error's message")
	}
}

/// Bytes in memory, read and seeked as a file is.
struct Memory: SeekableReader {
	let bytes: Data
	var at = 0

	init(_ bytes: Data) {
		self.bytes = bytes
	}

	mutating func read(into buffer: UnsafeMutableRawBufferPointer) throws -> Int {
		let n = min(buffer.count, bytes.count - at)
		buffer.copyBytes(from: bytes[at..<at + n])
		at += n
		return n
	}

	mutating func seek(offset: Int64, from origin: SeekOrigin) throws -> UInt64 {
		let base = switch origin {
		case .start: Int64(0)
		case .current: Int64(at)
		case .end: Int64(bytes.count)
		}
		at = Int(base + offset)
		return UInt64(at)
	}
}

/// An error whose description is its message, as Rust reads a Swift error's.
struct Failure: Swift.Error, CustomStringConvertible {
	let description: String
}

/// A reader whose reads fail, as the native transcript's does: each seek lands at 0.
/// A reader of nothing, which seeks anywhere from its start, as Rust's `Cursor` does.
struct Far: SeekableReader {
	func read(into buffer: UnsafeMutableRawBufferPointer) throws -> Int {
		0
	}

	func seek(offset: Int64, from origin: SeekOrigin) throws -> UInt64 {
		UInt64(offset)
	}

	func seek(start offset: UInt64) throws -> UInt64 {
		offset
	}
}

struct Broken: SeekableReader {
	func read(into buffer: UnsafeMutableRawBufferPointer) throws -> Int {
		throw Failure(description: "broken")
	}

	func seek(offset: Int64, from origin: SeekOrigin) throws -> UInt64 {
		0
	}
}

/// Sets its cell once released, as a closure Rust kept and released holds it.
final class Released: @unchecked Sendable {
	let cell: Cell<Bool>

	init(_ cell: Cell<Bool>) {
		self.cell = cell
	}

	deinit {
		cell.value = true
	}
}

// The futures' semantics beyond the transcript, each checked by an assertion.
func futures() {
	let channel = Channel.new()

	// The send lands from another thread while the receive waits.
	let payload = block {
		Thread.detachNewThread { try! channel.send(Bytes("blocked".utf8)) }
		return try await channel.recv()
	}
	check(text(payload) == "blocked", "recv returns the payload another thread sent")

	let q = Queue()

	// Resumed on its executor, never inside the wake.
	do {
		let got = Cell<String?>(nil)
		q.launch { got.value = text(try await channel.recv()) }
		q.drain()
		check(got.value == nil, "recv waits for a send")
		try! channel.send(Bytes("then".utf8))
		check(got.value == nil, "recv never resumes inside the wake")
		q.drain()
		check(got.value == "then", "recv resumes with the payload")
	}

	// A wake that lands while the future polls, before it returns pending, is kept: the
	// future polls again rather than waiting for a wake that already happened.
	do {
		let got = Cell<UInt32>(0)
		q.launch { got.value = try await yielded(7) }
		q.drain()
		check(got.value == 7, "a wake during the poll polls again")
	}

	// A value that is not Send stays on the thread that made it: a use elsewhere throws, and
	// a release elsewhere drops it on its own thread at its next call. A future that is not
	// Send made from none lives on a home, so any executor awaits it.
	do {
		let held = Cell<Held?>(Held(Local.new()))
		let shared = try! held.value!.local.share()
		_ = try! held.value!.local.bump()
		check(try! shared.bump() == 2, "a shared local counts together")
		let got = Cell<UInt32>(0)
		q.launch { got.value = try await held.value!.local.later() }
		q.drain()
		check(got.value == 3, "a local's future runs on its thread")
		check(block { try await localLater(5) } == 10, "a future that is not Send made from none awaits anywhere")
		let thrown = Cell<String?>(nil)
		let done = DispatchSemaphore(value: 0)
		Thread.detachNewThread {
			do {
				_ = try held.value!.local.bump()
			} catch let e as CheckError {
				thrown.value = "\(e.function): \(e.message)"
			} catch {}
			done.signal()
		}
		done.wait()
		check(
			thrown.value == "api_Local_bump: a value that is not Send is used on another thread than the one that made it",
			"a local used elsewhere throws: \(String(describing: thrown.value))"
		)
		let before = localsDropped()
		Thread.detachNewThread {
			held.value = nil
			done.signal()
		}
		done.wait()
		check(localsDropped() == before, "a local released elsewhere waits for its thread")
		check(try! shared.bump() == 4 && localsDropped() == before + 1, "a local released elsewhere drops on its thread")
	}

	// A future whose output is `None` is ready, not pending.
	do {
		let got = Cell<UInt32??>(nil)
		q.launch { got.value = .some(try await halved(3)) }
		q.drain()
		check(got.value == .some(nil), "a future ready with None returns nil")
		check(block { try await halved(4) } == 2, "a future ready with Some returns its value")
	}

	// A future runs on its caller's actor, never hopping off it and back.
	check(block { try await Probe().count(2) } == 4, "a future on an actor returns its value")

	// Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	// for the next.
	do {
		let called = Cell(false)
		let threw = Cell(false)
		let task = q.launch {
			do {
				_ = try await channel.recv()
				called.value = true
			} catch is CancellationError {
				threw.value = true
			}
		}
		q.drain()
		try! channel.send(Bytes("kept".utf8))
		task.cancel()
		q.drain()
		check(!called.value && threw.value, "a cancelled receive throws CancellationError")
		let payload = block { try await channel.recv() }
		check(text(payload) == "kept", "a cancelled receive loses nothing")
	}

	// A task cancelled before its call takes nothing, even a payload ready at the first poll.
	do {
		let threw = Cell(false)
		try! channel.send(Bytes("ready".utf8))
		let task = q.launch {
			do {
				_ = try await channel.recv()
			} catch is CancellationError {
				threw.value = true
			}
		}
		task.cancel()
		q.drain()
		check(threw.value, "a cancelled task's receive throws")
		let payload = block { try await channel.recv() }
		check(text(payload) == "ready", "a cancelled task's receive takes nothing")
	}

	// An eager call whose callback cancels its task while it makes its future is never
	// polled: only the eager half logs.
	do {
		let task = Cell<Task<Void, Swift.Error>?>(nil)
		let log = CancelLog(task)
		let threw = Cell(false)
		task.value = q.launch {
			do {
				_ = try await emitLater(log, Bytes([1, 2]))
			} catch is CancellationError {
				threw.value = true
			}
		}
		q.drain()
		check(threw.value, "an eager call cancelled as it makes its future throws")
		check(log.lines.value == 2, "an eager call cancelled as it makes its future is never polled")
	}

	// Cancelling a task that waits for no wake resumes it.
	do {
		let threw = Cell(false)
		let task = q.launch {
			do {
				_ = try await channel.recv()
			} catch is CancellationError {
				threw.value = true
			}
		}
		q.drain()
		check(!threw.value, "recv waits for a send")
		task.cancel()
		q.drain()
		check(threw.value, "cancelling a waiting receive resumes it, throwing")
	}

	// An object whose last reference goes while a call awaits on it stays alive until the
	// call ends.
	do {
		let got = Cell<String?>(nil)
		let sender: Channel
		do {
			let other = Channel.new()
			sender = try! other.clone()
			q.launch { got.value = text(try await other.recv()) }
			q.drain()
		}
		try! sender.send(Bytes("held".utf8))
		q.drain()
		check(got.value == "held", "the call awaiting on an object holds it")
	}

	// Async closures Rust polls as futures, each run as a task of its own.
	do {
		let ran = Cell(false)
		check(block { try await after({ ran.value = true }, 4) } == 8 && ran.value, "Rust awaits an async closure")
	}
	do {
		let (stream, gate) = AsyncStream<Void>.makeStream()
		let got = Cell<UInt32>(0)
		q.launch { got.value = try await after({ for await _ in stream { break } }, 5) }
		q.drain()
		check(got.value == 0, "Rust waits on an async closure that waits")
		gate.yield(())
		until { q.drain(); return got.value != 0 }
		check(got.value == 10, "an async closure's end wakes Rust")
	}
	// The closure waits, so it pends whether or not its task has run yet, and the check
	// waits for the throw, wherever the task resumes.
	do {
		let (stream, gate) = AsyncStream<Void>.makeStream()
		let threw = Cell(false)
		q.launch {
			do {
				try await within(0) { for await _ in stream { break } }
			} catch is Api.Error {
				threw.value = true
			}
		}
		until { q.drain(); return threw.value }
		check(threw.value, "Rust gives up on an async closure")
		gate.finish()
	}
	do {
		let started = Cell(false)
		let cancelled = Cell(false)
		let task = q.launch {
			try await within(1000) {
				started.value = true
				do {
					try await Task.sleep(nanoseconds: 60_000_000_000)
				} catch {
					cancelled.value = true
				}
			}
		}
		q.drain()
		until { started.value }
		task.cancel()
		q.drain()
		until { cancelled.value }
		check(cancelled.value, "cancelling the call cancels the async closure's task")
	}
	// A waiter's poll of an async closure, which its future owns.
	do {
		let (stream, gate) = AsyncStream<Void>.makeStream()
		let done = Cell(false)
		q.launch {
			try await Waiter.pollFuture({ for await _ in stream { break } })
			done.value = true
		}
		q.drain()
		check(!done.value, "a waiter's poll waits on an async closure that waits")
		gate.yield(())
		until { q.drain(); return done.value }
		check(done.value, "an async closure's end wakes a waiter's poll")
	}
	do {
		let started = Cell(false)
		let cancelled = Cell(false)
		let task = q.launch {
			try await Waiter.pollFuture {
				started.value = true
				do {
					try await Task.sleep(nanoseconds: 60_000_000_000)
				} catch {
					cancelled.value = true
				}
			}
		}
		q.drain()
		until { started.value }
		task.cancel()
		q.drain()
		until { cancelled.value }
		check(cancelled.value, "cancelling a waiter's poll cancels the async closure it owns")
	}
	// Closures Rust keeps, released once Rust drops them, holding the only reference.
	do {
		let freed = Cell(false)
		let spawned = try! Spawned.new { [held = Released(freed)] in
			_ = held
			return 1
		}
		check(try! spawned.join() == 1, "a spawned closure runs")
		until { freed.value }
		check(freed.value, "Rust releases a kept closure once it drops it")
	}
	do {
		let freed = Cell(false)
		// Any Swift error crosses as its description, which Rust returns as its error.
		let stopped = block {
			try await drive(Moment.new(0, 1000)!) { [held = Released(freed)] _, _ in
				_ = held
				throw Failure(description: "failed")
			}
		}
		check(stopped == "failed", "a closure's error crosses as its description")
		until { freed.value }
		check(freed.value, "a future releases the closure it kept once it finishes")
	}
	print("futures ok")
}

/// Wait up to five seconds for `done`, which another thread makes true.
func until(_ done: () -> Bool) {
	let deadline = Date().addingTimeInterval(5)
	while !done() && Date() < deadline {
		usleep(1000)
	}
}

// Threads calling one class at once: they contend for its hold count, so later calls hold
// it in the runtime's table instead, and its release, once they are done, still frees it.
func shared() {
	let gap = Gap.new(7, 3)
	let threads = 8
	let calls = 100_000
	let wrong = UnsafeMutablePointer<Int>.allocate(capacity: threads)
	wrong.initialize(repeating: 0, count: threads)
	DispatchQueue.concurrentPerform(iterations: threads) { t in
		for _ in 0..<calls where gap.sum() != 10 || gap.clone().sum() != 10 {
			wrong[t] += 1
		}
	}
	check((0..<threads).allSatisfy { wrong[$0] == 0 }, "calls sharing a class read its value")
	wrong.deallocate()
}

// What frees an object or bytes: Swift's release of the last reference, at once. Returned
// bytes lend Rust's buffer until it and every slice of it are released.
func owned() {
	for n in 1023...1025 {
		let payload = Bytes((0..<n).map { UInt8(truncatingIfNeeded: $0) })
		let packet = nextPacket(Packet(seq: 1, payload: payload))
		check(packet.seq == 2 && packet.payload == payload, "a record's \(n) bytes cross both ways")
	}
	let before = dropped()
	do {
		let bytes = tokenBytes(1, 1025)
		check(dropped() == before, "returned bytes lend Rust's buffer")
		check(bytes == Bytes(repeatElement(UInt8(1), count: 1025)), "returned bytes read back")
	}
	check(dropped() == before + 1, "releasing returned bytes frees Rust's")
	do {
		let token = Token.new(2)
		check(try! token.idGet == 2, "a token reads back")
	}
	check(dropped() == before + 2, "releasing an object frees its handle")
	do {
		let part = tokenBytes(3, 1025).dropFirst()
		check(dropped() == before + 2, "a slice keeps Rust's buffer")
		check(part == Bytes(repeatElement(UInt8(3), count: 1024)), "a slice outlives the bytes it was cut from")
	}
	check(dropped() == before + 3, "releasing the last slice frees Rust's buffer")
	forward()
	check(dropped() == before + 5, "each forwarded payload drops once, with its last owner")
	print("owned ok")
}

/// Where `bytes` start.
func address(_ bytes: Bytes) -> UnsafeRawPointer? {
	bytes.withUnsafeBytes { $0.baseAddress }
}

// A payload Rust returned, read and written back, whole and in a record: Rust takes a
// second owner of its own buffer each time, so the bytes stay where Rust put them.
func forward() {
	let channel = Channel.new()
	let payload = tokenBytes(4, 64)
	let at = address(payload)
	try! channel.send(payload)
	let got = block { try await channel.recv() }
	check(address(got) == at && got.count == 64, "a payload sent back is Rust's own")
	let packet = nextPacket(Packet(seq: 1, payload: got))
	check(address(packet.payload) == at && packet.payload.count == 64, "a record carries a payload back as Rust's own")
	let part = packet.payload.dropFirst(8)
	try! channel.send(part)
	let sent = block { try await channel.recv() }
	check(address(sent) == at.map { $0 + 8 } && sent.count == 56, "a slice sent back is Rust's own")
	check(Bytes(sent.data) == sent && sent.data.count == 56, "Rust's bytes copy into a Data")
	// A few bytes `Data` holds inline, which pass back as Swift's.
	try! channel.send(tokenBytes(6, 3))
	check(block { try await channel.recv() } == Bytes([6, 6, 6]), "a short payload sent back reads back")
}

// Each breaks a contract Swift's compiler checks, so it fails to compile, built alone with
// its flag.
#if MOVED
func moved() {
	let c = Counter.new("c", 0)
	_ = try! c.intoLabel(nil)
	_ = try! c.get()
}
#endif

#if BOXED_MOVED
func boxedMoved() {
	let tail = try! Node.new(1, nil)
	_ = try! Node.new(2, tail)
	_ = try! tail.sum()
}
#endif

#if LENT_MOVED
func lentMoved() {
	var registry = try! Registry.new(Counter.new("c", 0))
	_ = try! registry.edit { counter in
		_ = try! counter.intoLabel(nil)
		return false
	}
}
#endif

#if LENT_KEPT
/// Holds a lent tally past the call it was lent for.
final class Kept: @unchecked Sendable {
	var tally: Tally?
}

func lentKept() {
	let kept = Kept()
	_ = try! peek(Tally.new()) { lent in
		kept.tally = lent
		return 0
	}
}
#endif

/// `bytes` in lowercase hex, two digits each.
func hex(_ bytes: some Sequence<UInt8>) -> String {
	bytes.map { String($0, radix: 16).leftPadded(2) }.joined()
}

/// An IP address's octets in hex, so their length shows its family.
func ipHex(_ ip: IpAddr) -> String {
	switch ip {
	case .v4(let ip): return ipHex(ip)
	case .v6(let ip): return ipHex(ip)
	}
}

func ipHex(_ ip: Ipv4Addr) -> String {
	hex(ip.octets)
}

func ipHex(_ ip: Ipv6Addr) -> String {
	hex(ip.octets)
}

/// A socket address as its IP's octets in hex, its port, flow label, and scope, each
/// IPv6's alone, apart by colons.
func socketHex(_ addr: SocketAddr) -> String {
	switch addr {
	case .v4(let v4): return socketHex(v4)
	case .v6(let v6): return socketHex(v6)
	}
}

func socketHex(_ v4: SocketAddrV4) -> String {
	"\(hex(v4.ip.octets)):\(v4.port)"
}

func socketHex(_ v6: SocketAddrV6) -> String {
	"\(hex(v6.ip.octets)):\(v6.port):\(v6.flowinfo):\(v6.scopeId)"
}

/// Whether `body` throws a `CheckError` from `function`.
func refused(_ function: String, _ body: () throws -> Void) -> Bool {
	do {
		try body()
		return false
	} catch let e as CheckError {
		return e.function == function
	} catch {
		return false
	}
}

/// A `RustDuration`'s conversions to and from `Swift.Duration`, which holds every one, and
/// those it refuses, each throwing.
func durations(_ rustMax: RustDuration) {
	let swiftMax = Duration.seconds(Int64.max) * 2 + .seconds(1) + .nanoseconds(999_999_999)
	check(rustMax.duration == swiftMax, "Rust's longest duration is a Swift.Duration")
	check(try! RustDuration(swiftMax) == rustMax, "Rust's longest Swift.Duration is a RustDuration")
	check(try! RustDuration(.nanoseconds(1_500_000_001)) == RustDuration(secs: 1, nanos: 500_000_001), "a duration converts")
	check(try! RustDuration(.zero) == .seconds(0) && RustDuration.seconds(0).duration == .zero, "zero converts")
	check(RustDuration.milliseconds(.max) == (try! RustDuration(secs: UInt64.max / 1000, nanos: 615_000_000)), "milliseconds are total")
	check(RustDuration.microseconds(1_000_001) == (try! RustDuration(secs: 1, nanos: 1000)), "microseconds are total")
	check(RustDuration.nanoseconds(.max).duration == .nanoseconds(Int64.max) * 2 + .nanoseconds(1), "nanoseconds are total")
	check(RustDuration.seconds(1) < (try! RustDuration(secs: 1, nanos: 1)), "durations order as Rust's")
	check(refused("RustDuration(secs:nanos:)") { _ = try RustDuration(secs: 0, nanos: 1_000_000_000) }, "a second's nanoseconds throw")
	check(refused("RustDuration(_:)") { _ = try RustDuration(.nanoseconds(-1)) }, "a negative duration throws")
	check(refused("RustDuration(_:)") { _ = try RustDuration(swiftMax + .nanoseconds(1)) }, "a duration past Rust's longest throws")
	check(refused("RustDuration(_:)") { _ = try RustDuration(.nanoseconds(1) / 2) }, "a part below a nanosecond throws")
	check(refused("RustDuration(_:)") { _ = try RustDuration(swiftMax + .nanoseconds(1) / 2) }, "past the longest by less than a nanosecond throws")
}


/// The conversions to and from each runtime type, which throw or return nil for a value
/// Rust's type cannot hold, and its protocols, which agree with Rust's.
func lossless() {
	check(sameString("a\u{0}é\u{10ffff}") == "a\u{0}é\u{10ffff}", "text crosses")
	// A path that is not UTF-8, which no String holds, keeps its bytes.
	let strayPath = stray()
	check(strayPath.string == nil && PathBuf("a/é").string == "a/é", "a path's text is nil unless UTF-8")
	check(samePath(strayPath) == strayPath, "a path that is not UTF-8 crosses back")
	visitStray { path in check(path == strayPath, "a lent path that is not UTF-8 keeps its bytes") }
	check(samePath(PathBuf(bytes: Bytes([0xff]))) == strayPath, "a caller's bytes make a path")
	let ip6 = Ipv6Addr(bits: U128(high: 1 << 48, low: 2))
	let addrs = [SocketAddr.v4(SocketAddrV4(ip: Ipv4Addr(bits: .max), port: .max)), .v6(SocketAddrV6(ip: ip6, port: 3))]
	// A key hashes by Rust's `Hash`, which agrees with its `==`.
	let a = Key.new("a")
	let b = Key.new("b")
	check(hashed(a) == hashed(Key.new("a")) && hashed(a) != hashed(b), "a key hashes as Rust's Hash does")
	let edge6 = Ipv6Addr(octets: [0xff, 0xff] + Array(repeating: 0, count: 13) + [1])!
	// Octets convert both ways, and only exactly as many as the family holds.
	check(Ipv4Addr(octets: [1, 2, 3, 4]) == Ipv4Addr(1, 2, 3, 4) && Ipv4Addr(1, 2, 3, 4).bits == 0x0102_0304, "IPv4 octets convert")
	check(Ipv6Addr(octets: edge6.octets) == edge6 && edge6.bits == U128(high: 0xffff << 48, low: 1), "IPv6 octets convert")
	check(Ipv4Addr(octets: [1, 2, 3]) == nil && Ipv6Addr(octets: [UInt8](repeating: 0, count: 4)) == nil, "octets of another length convert to nil")
	check(SocketAddr(ip: .v6(ip6), port: 3) == .v6(SocketAddrV6(ip: ip6, port: 3)) && addrs[1].ip == .v6(ip6) && addrs[0].port == .max, "a socket address is its IP and port")
}


/// `key`'s hash by Swift's own hasher, which `Hashable` uses.
func hashed(_ key: borrowing Key) -> Int {
	var hasher = Swift.Hasher()
	key.hash(into: &hasher)
	return hasher.finalize()
}

// Each breaks a contract, which must fail with a message naming it.
func broken(_ which: String) {
	#if os(Linux)
	if nodump() != 0 {
		perror("prctl")
		exit(1)
	}
	#endif
	switch which {
	// A lent return outliving its owner's change, which Rust checks.
	case "outlived":
		var meters = Meters.new()
		let first = try! meters.first()
		try! meters.clear()
		do {
			_ = try first.count()
		} catch let e as CheckError {
			// Rust ran nothing, so its meters still clear.
			check(e.function == "api_Meter_count", "a check names its function")
			try! meters.clear()
			fatalError(e.description)
		} catch {
			check(false, "a failed check throws a CheckError")
		}
	// The file serves wasm32 too, whose items the host's library lacks, where Swift cannot
	// throw.
	case "variant":
		_ = pointerBits(.wasm(bits: 32))
	case "plain variant":
		_ = isFamily(.wasm)
	default:
		check(false, "no case \(which)")
	}
}

@main
struct Main {
	static func main() {
		let args = CommandLine.arguments.dropFirst()
		switch args.first {
		case nil: transcript()
		case "checks": checks()
		case "futures": futures()
		case "owned": owned()
		case "shared": shared()
		case let which?: broken(which)
		}
	}
}
