// The api fixture's Swift test: the script fixtures/api/examples/transcript.rs runs
// natively, through the generated Swift. tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks the futures' semantics, what releasing frees, or
// breaks a contract, which must fail.

import Api
import BasicContainers
import Foundation
#if os(Linux)
import NoDump
#endif

// The Log callback: keeps even levels.
final class PrintLog: Log {
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

func bound(_ b: BoundU64) -> String {
	switch b {
	case .included(let n): return "Included(\(n))"
	case .excluded(let n): return "Excluded(\(n))"
	case .unbounded: return "Unbounded"
	}
}

func debug<T>(_ value: T) -> String {
	String(reflecting: value)
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

/// Each of `items` as `each` prints it, joined by commas.
func listed<T: ~Copyable>(_ items: borrowing UniqueArray<T>, _ each: (borrowing T) -> String) -> String {
	var out: [String] = []
	for i in 0..<items.count {
		out.append(each(items[i]))
	}
	return out.joined(separator: ", ")
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

func script() {
	print("limit \(LIMIT)")

	let p = Point(x: 1, y: 2).offset(2, 3)
	print("offset \(p.x) \(p.y)")
	print("debug \(debug(p))")
	print("eq \(p == Point(x: 3, y: 5))")
	let d = Point.default()
	print("default \(d.x) \(d.y)")

	var c = Counter.new("c", 5)
	print("get \(try! c.get())")
	print("name \(try! c.name())")
	print("add ok \(try! c.add(10))")
	do {
		_ = try c.add(2000)
	} catch let e as Api.Error {
		print("add err \(e) \(debug(e))")
	} catch {
		check(false, "add throws an Error")
	}
	do {
		_ = try c.sub(20)
	} catch let e as Api.Error {
		print("sub err \(e) \(debug(e))")
	} catch {
		check(false, "sub throws an Error")
	}
	try! c.rename("d")
	print("name \(try! c.name())")
	print("is d \(try! c.`is`("d"))")
	var copy = try! c.clone()
	_ = try! copy.add(1)
	print("clone \(try! copy.get()) original \(try! c.get())")
	print("debug \(try! c.debugDescription)")
	let label = try! c.intoLabel(p)
	print("label \(debug(label))")
	print("note \(hasText(Note(text: ""))) \(hasText(Note(text: nil)))")

	// A tuple holding a handle is a struct of its elements.
	let split = try! Counter.new("s", 7).split(5)
	print("split \(try! split._0.name()) \(try! split._0.get()) \(try! split._1.name()) \(try! split._1.get())")
	let one = try! only(Counter.new("o", 3))
	print("only \(try! one.name()) \(try! one.get())")
	let (w8, w16, w32, w64) = widths(98305)
	print("widths \(w8) \(w16) \(w32) \(w64)")
	for b: UInt64 in [2, 0] {
		do {
			let (q, r) = try divide(7, b)
			print("divide \(q) \(r)")
		} catch {
			print("divide err \(error)")
		}
	}
	for n: UInt64 in [98305, 0] {
		if let s = spread(n) {
			print("spread \(s.0) \(s.1) \(s.2) \(s.3)")
		} else {
			print("spread none")
		}
	}
	for needle in ["lo", "x"] {
		if let found = find(Bytes("hello".utf8), Bytes(needle.utf8)) {
			print("find \(found.0) \(found.1)")
		} else {
			print("find none")
		}
	}
	let statRows = rows([Point(x: 1, y: 2), Point(x: 3, y: 3)])
	print("rows \(statRows.map { "\($0.0) \(debug($0.1)) \($0.2.x) \($0.2.y)" }.joined(separator: ", "))")
	// Each handle in a tuple moves in, as one taken by value does, and an array of them is a
	// `UniqueArray`.
	var ranks = UniqueArray<TupleStringCounter>()
	ranks.append(TupleStringCounter("low", Counter.new("l", 1)))
	ranks.append(TupleStringCounter("high", Counter.new("h", 9)))
	let board = try! Board.new(ranks)
	let ranked = try! board.ranked()
	print("ranked \(listed(ranked) { "\($0._0) \(try! $0._1.name()) \(try! $0._1.get())" })")
	let best = try! board.best()
	print("best \(try! best.name()) \(try! best.get())")
	// A map crosses as its pairs and a set as its items, which Rust collects in order, each
	// key once, the last kept.
	let atlas = Atlas(points: [("b", Point(x: 1, y: 1)), ("ab", Point(x: 2, y: 3)), ("b", Point(x: 4, y: 4))], ids: [3, 1, 3])
		.with("c", Point(x: 5, y: 6))
	let atlasPoints = atlas.points.map { "\($0.0) \($0.1.x) \($0.1.y)" }.joined(separator: ", ")
	print("atlas \(atlasPoints); ids \(atlas.ids.map { "\($0)" }.joined(separator: " "))")
	// Another crate's error crosses as its message.
	for text in ["80", "x"] {
		do {
			print("port \(try port(text))")
		} catch let e as StringError {
			print("port err \(e.error)")
		} catch {
			check(false, "port throws its message")
		}
	}
	let span = Span(bounds: (2, 10), named: ("s", .square))
	let (lowerHalf, upperHalf) = halves(span)
	print("halves \(lowerHalf.0) \(lowerHalf.1) \(upperHalf.0) \(upperHalf.1) \(debug(span))")
	let (swappedN, swappedText) = swapped(("t", 4))
	print("swapped \(swappedN) \(swappedText)")
	// A tuple of one is its element, so `Option<(Option<u32>,)>` nests two optionals.
	for value: UInt32?? in [.none, .some(nil), .some(3)] {
		switch doubled(value) {
		case .none: print("doubled None")
		case .some(let inner): print("doubled Some((\(inner.map { "Some(\($0))" } ?? "None"),))")
		}
	}
	print("lent back \(Array(lentBack(Bytes([1, 2])))) boxed \(Array(boxedTuple(Bytes([3]))))")
	// A borrowed string and an owned one share a tuple struct.
	print("title Some((\"\(title(true)!)\",)) \(title(false) == nil ? "None" : "Some") named Some((\"\(named("x")!)\",))")

	// Each lent counter is a new handle to a clone.
	var registry = try! Registry.new(Counter.new("first", 1))
	print("last \(try! registry.last() != nil)")
	try! registry.push(Counter.new("last", 2))
	var first = try! registry.firstGet()
	_ = try! first.add(10)
	print("first \(try! registry.firstGet().get()) lent \(try! first.get())")
	var last = try! registry.last()!
	_ = try! last.add(10)
	print("last \(try! registry.last()!.get()) lent \(try! last.get())")
	// A closure lent for the call edits the first counter through a handle.
	let by: UInt64 = 5
	let edited = try! registry.edit { counter in
		_ = try! counter.add(by)
		return try! counter.edited() && counter.get() == 6
	}
	print("edit \(edited) \(try! registry.firstGet().get())")
	// A setter moves an object in, as assignment does, closing its handle.
	try! registry.firstSet(Counter.new("second", 3))
	print("set first \(try! registry.firstGet().get()) \(try! registry.firstGet().name())")
	print("walk \(walk(3) { cursor in try! cursor.at() * 2 })")
	print("each \(each(2) { at in print("visit \(at)") })")
	print("offset \(offset({ 7 }, 3))")
	print("shift \(shift({ 7 }, 4))")

	let name = Name.owned("a")
	var joined = try! name.join(Name.owned("b"))
	print("joined \(try! joined.asStr()) \(try! joined.debugDescription) \(try! joined == name)")
	try! joined.rename("c")
	print("renamed \(try! joined.asStr())")
	let long = try! longer(joined, Name.owned("dd"))
	print("longer \(try! long.asStr())")
	print("unnamed [\(try! unnamed().asStr())]")
	let lit = Name.borrowed("lit")
	print("borrowed \(try! lit.asStr()) \(try! lit.shares(Name.borrowed("lit"))) \(try! lit.shares(Name.borrowed("other")))")

	print("total \(total(RangeU64(start: 2, end: 5)))")
	print("reach \(reach(2, 5))")
	let scaledHex = [
		scaled(U128(high: .max, low: .max), 1), scaled(U128(high: 1, low: 1), 1 << 63), scaled(U128(high: 1 << 63, low: 0), 2),
	].map { v in
		v.map { v in [v.high, v.low].map { h in Swift.String(repeating: "0", count: 16 - Swift.String(h, radix: 16).count) + Swift.String(h, radix: 16) }.joined() } ?? "none"
	}
	print("scaled \(scaledHex.joined(separator: " "))")
	check(U128(high: .max, low: .max).description == "340282366920938463463374607431768211455", "a u128's decimal digits")
	let sumHex = { (sum: ArrayU8x4) in sum.map { Swift.String(format: "%02x", $0) }.joined() }
	var digest = Digest.new()
	try! digest.sumSet(try! fingerprint("deadbeef"))
	var unparsed = ""
	do {
		_ = try fingerprint("zz")
	} catch let e as StringError {
		unparsed = e.error
	} catch {
		check(false, "fingerprint throws its message")
	}
	print("digest \(sumHex(try! digest.sumGet)) \(sumHex(try! digest.flipped(ArrayU8x4([0xff, 0, 0xff, 0])!))) \(unparsed)")
	check(ArrayU8x4([1, 2, 3]) == nil, "3 bytes are no [u8; 4]")
	var batch = Batch.new(), batch2 = pair()
	let accepted = (1...5).filter { try! batch.push($0) }.count
	let paired = (1...3).filter { try! batch2.push($0) }.count
	let emptied = try! drain4(&batch)
	print("batch \(try! batch.capacity()) \(accepted) \(emptied) \(try! drain4(&batch)) \(try! batch2.capacity()) \(paired) \(try! drain2(&batch2))")
	print("capacities \(try! capacities42(batch, batch2)) \(try! capacities22(batch2, batch2))")
	let wide = widen(RangeU64(start: 2, end: 5), 1)
	print("widen \(wide.start) \(wide.end)")
	let counts = [
		BoundsU64(start: .included(2), end: .excluded(5)),
		BoundsU64(start: .included(2), end: .included(5)),
		BoundsU64(start: .included(3), end: .unbounded),
		BoundsU64(start: .unbounded, end: .unbounded),
		BoundsU64(start: .excluded(2), end: .included(4)),
	].map { count($0) }
	print("count \(counts.map(String.init).joined(separator: " "))")
	print("end \(bound(end(3))) \(bound(end(0)))")
	let (endsStart, endsEnd) = ends(3)
	print("ends \(bound(endsStart)) \(bound(endsEnd))")
	for n: UInt64 in [2, 5] {
		do {
			print("below \(try Api.below(n, 3))")
		} catch let e as BoundsU64Error {
			print("below err \(bound(e.error.start)) \(bound(e.error.end))")
		} catch {
			check(false, "below throws its bounds")
		}
	}
	let window = Window(range: RangeU64(start: 2, end: 8), end: end(5))
	print("window \(window.contains(4)) \(window.contains(6)) \(debug(window))")

	print(greet("x", .square))
	print("len \(len(Bytes("abc".utf8)))")
	let binary = Bytes([0xff, 0, 0x61])
	print("hex \(hex(binary))")
	print("timed hex \(timedHex(binary))")
	print("sample hex \(sampleHex(binary))")
	print("queued hex \(queuedHex(binary))")
	print("tick \(tickName(Label(name: "tick", at: nil, ttl: nil)))")
	print("rooted \(rootedName(Label(name: "rooted", at: nil, ttl: nil)))")
	print("stored \(storedText("stored"))")
	print("layer \(layerName(Label(name: "layer", at: nil, ttl: nil)))")
	for event: Event in [.moved(p), .named(name: "n"), .done] {
		print(describe(event))
	}
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

	// A `Copy` handle passed by value is copied, so it stays usable.
	let stamp = Stamp.fromMillis(2500)
	print("secs \(stamp.secs())")
	print("later \(Stamp.fromMillis(3000).later(stamp))")
	print("secs \(stamp.secs())")
	print("whole \(debug(stamp.whole()))")
	var message = Message.new(stamp, Bytes("body".utf8))
	print("payload \(text(try! message.payloadGet))")
	print("stamp \(debug(try! message.stampGet!))")
	print("sent \(try! message.sent())")
	// Setters assign in place: the bytes are copied in, and a `Copy` handle copied.
	let reset = Stamp.fromMillis(4000)
	try! message.stampSet(reset)
	try! message.payloadSet(Bytes("edited".utf8))
	print("payload \(text(try! message.payloadGet))")
	print("stamp \(debug(try! message.stampGet!))")
	print("secs \(reset.secs())")
	try! message.stampSet(nil)
	print("stamp \(try! message.stampGet != nil)")

	// A value past a word, copied across: a closure takes and returns one by value.
	let moment = Moment.new(1500, 1000)!
	print("moment \(moment.millis()) \(Moment.new(1, 0) == nil)")
	let shifted = shifted(moment) { $0.after(500) }
	print("shifted \(shifted.millis()) \(shifted == moment.after(500))")
	print("wrapped \(moment.after(UInt64.max).millis()) \(moment == shifted)")

	// A value with padding, or bytes that vary, is a handle, as no table says where padding is.
	let gap = Gap.new(7, 3)
	print("gap \(gap.sum()) \(gap == Gap.new(7, 3))")
	let berth = Berth.new(4)
	print("berth Some(\(berth.at()!)) \(Berth.new(nil).at() == nil ? "None" : "Some")")

	// A `Copy` type changed in place is a value, whose change reaches only the copy changed.
	var score = Score.of(2)
	let step = Score.of(3)
	score.add(step)
	score.add(step)
	print("score \(score.count()) \(step.doubled().count()) \(step.count())")
	var scoreCopy = score
	let taken = scoreCopy.take()
	print("taken \(taken) \(scoreCopy.count()) \(score.count())")
	var otherScore = step
	let before = scoreCopy.absorb(&otherScore)
	print("absorbed \(before.0) \(before.1) \(scoreCopy.count()) \(otherScore.count()) \(step.count())")
	var hits = Hits.of(1, 2)
	let keptHits = hits
	hits.add(Hits.of(10, 20))
	hits.hitsSet(7)
	print("hits \(hits.hitsGet) \(hits.missesGet) \(keptHits.hitsGet) \(keptHits.missesGet) \(hits == keptHits)")
	var counted = keptHits
	let countedSame = counted.eq(keptHits)
	print("counted \(countedSame) \(counted.hitsGet)")
	var extent = Extent.new(5, 3)
	let extentStart = extent
	let old = extent.slide(10)
	extent.startSet(extent.startGet + 1)
	print("extent \(old) \(extent.startGet) \(extent.end()) \(extentStart.startGet) \(extentStart.end())")
	let weighted = Weighted(score: step, weight: 4)
	print("weighted \(weighted.total()) \(weighted.score.count() + step.count())")
	// A checked mark beside bytes in a tuple and in a record lent, and bytes beside a move.
	let mark = Mark.of(1)
	try! mark.add(2)
	let parcel = Parcel(mark: mark, payload: Bytes("abc".utf8))
	print("heft \(try! heft((mark, Bytes("ab".utf8)), parcel, TupleCounterBytes(Counter.new("w", 1), Bytes("abcd".utf8))))")
	// A value's `&mut self` beside a checked handle.
	var noted = Score.of(2)
	let notedBefore = try! noted.note(Mark.of(5))
	print("noted \(notedBefore) \(noted.count())")

	// Bound first, so Swift's reference is gone before the print.
	let kept = emit(PrintLog(), Bytes([1, 2, 3, 4]))
	print("kept \(kept)")
	do {
		let logger = Logger.new(PrintLog())
		print("write \(logger.write("hi"))")
	}
	print("dropped")
	// Rust calls it on another thread, before the call returns.
	let threshold: UInt8 = 2
	let filtered = filter(Bytes([1, 2, 3, 4])) { level, line in
		print("filter \(level) \(line)")
		return level > threshold
	}
	print("filtered \(filtered)")

	let channel = Channel.new()
	let wakes = Cell(0)
	// On an executor the script drains, so it sees the receive wait and its wake queue
	// the resume.
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

	var list = try! Node.new(1, nil)
	try! list.push(2)
	print("node \(try! list.sum())")

	let tally = Tally.new()
	_ = try! tally.bump()
	print("tally \(try! tally.bump()) \(try! tally == tally)")
	print("both \(try! bumpBoth(tally, tally))")
	print("peek \(try! peek(tally) { lent in try! lent.bump() })")

	let grown = grow(Area.new(Size(width: 2, height: 3)), 1)
	let size = grown.size()
	print("area \(size.width) \(size.height) \(grown.pixels())")

	// Generic types, each at the instances it infers, named by their arguments.
	let pair = PairU32(left: 2, right: 5)
	let swappedPair = pair.swap()
	print("pair \(swappedPair.left) \(swappedPair.right) sum \(pair.sum()) \(debug(pair)) \(pair == swappedPair)")
	print("even \(pair.even()) \(PairU32(left: 7, right: 7).even())")
	// An instance renamed alone, beside another of `Pair` that keeps its name.
	let sized: PairSizeU8 = tag(Size(width: 1, height: 2), 3)
	let flipped: PairU8Size = sized.swap()
	print("sized \(flipped.left) \(flipped.right.width) \(debug(flipped))")
	let beside = PairU32(left: 258, right: 9).besideSize(sized.left)
	print("beside \(beside.left) \(debug(beside))")
	// At its default, named as the type itself.
	var stack = Stack.new()
	for (i, n) in ["a", "b"].enumerated() {
		try! stack.push(Counter.new(n, UInt64(i) + 1))
	}
	let top = try! stack.pop()!.get()
	print("stack \(try! stack.depth()) Some(\(top)) \(try! stack.pop() != nil) \(try! stack.pop() == nil)")
	// Generic functions and methods, at each implementation of the trait bounding them, by
	// reference, which leaves Swift its handle.
	let fromPoint = Counter.seededPoint("p", Point(x: -3, y: 4))
	let fromSize = Counter.seededSize("s", Size(width: 5, height: 6))
	let fromCounter = try! Counter.seededCounter("c", fromPoint)
	let fromPolar = Counter.seededPolarPoint("q", PolarPoint(turns: 1, distance: 2))
	print(
		"seeded \(try! fromPoint.get()) \(try! fromSize.get()) \(try! fromCounter.get()) \(try! fromPolar.get()) "
			+ "past \(try! stack.depthPastPoint(Point(x: 1, y: 2))) seeds \(seedsPointSize(Size(width: 2, height: 3)))")

	// A trait implemented for `Point` and `&Point` apart: one instance.
	print("weigh \(weighPoint(Point(x: -1, y: 2)))")
	// A function named as the type it returns, and one named as a type it does not.
	print("polar \(polarPoint(2)!.distance) \(polarPoint(0) == nil) area \(area(Size(width: 2, height: 3)))")
	// Two types whose paths join alike, the second kept apart where its path splits.
	print("wind \(wind(PolarTurn(whole: 2), Polar_Turn(part: 3)))")
	// Names that clash in some language, each kept apart there.
	let clashSpan = ClashSpan(width: 4)
	let clashStore = ClashStore.new(7)
	print(
		"clash \(debug(ClashTone(hz: 440))) \(debug(ClashTone_(db: 3))) \(debug(Cancelled(step: 2))) "
			+ "span \(clashSpan.width_()) \(clashSpan.toString()) \(clashSpan) "
			+ "store \(clashStore.free()) \(clashStore.close()) "
			+ "audio \(clashAudioEncoderFinish()) \(clashAudioEncoderFinish_()) "
			+ "nested \(clashNested(((1, 2), 3), (1, 2, 3)))"
	)
	// Instances that name each other, and one made only by an impl of it.
	let crossed = cross(LeftRightU8(value: RightU8(value: 1)), RightLeftU8(value: LeftU8(value: 2)))
	print("cross \(crossed) outer \(OuterInnerU8.new(4).get())")
	// A bound that outlives the call takes a value, and an instance only a constant names.
	print("kept \(keptPoint(Point(x: 1, y: -2))) boxed \(BOXED.value)")
	print("keep \(keepPoint(Point(x: 1, y: -2)))")
	let five = Point(x: 5, y: 2)
	print("kept or \(keptOrPoint(five)) \(keptOrPoint(nil)) keeper \(keeperPoint(five))")
	// Inferred instances taking a lent closure and a foreign implementation, and one only
	// its impl of a trait names.
	let sides = PairU32(left: 2, right: 5)
	print("apply \(applyU32(sides) { side in side * 3 })")
	let report = reportU32(sides, PrintLog())
	print("report \(report)")
	print("wrapped \(keptWrapU32(WrapU32(value: 4)))")
	let keptLog = keeps(PrintLog())
	print("keeps \(keptLog.value)")

	// Its Drop runs when Swift releases the last reference, not before.
	do {
		let token = Token.new(7)
		withExtendedLifetime(token) { print("token \(try! token.idGet) dropped \(dropped())") }
	}
	print("dropped \(dropped())")

	// A field and the method of its name, each its own member.
	var retry = Retry.new(5)
	let limit = try! retry.limitGet
	print("retry \(limit != nil ? 1 : 0) \(limit ?? 0) \(try! retry.limit()) \(try! retry.delay())")
	// The renamed field's setter, named as its getter.
	try! retry.limitSet(nil)
	print("retry \(try! retry.limit())")
	print("slot \(SlotU8.new(200).value())")

	// Foreign types: a URL parsed from text, socket addresses, OS paths, and `Vec`s.
	let base = try! Url.parse("https://example.com/a/b")
	print("url \(try! base.description) host \(try! host(base).map { "Some(\"\($0)\")" } ?? "None")")
	let url = try! resolve(try! base.clone(), "c?d")
	print("resolve \(try! url.description) \(try! url == base)")
	do {
		_ = try Url.parse("no scheme")
	} catch let e as StringError {
		print("parse err \(e.error)")
	} catch {
		check(false, "parse throws its message")
	}
	let v4 = SocketAddr.v4(SocketAddrV4(ip: Ipv4Addr(127, 0, 0, 1), port: 8080))
	let loopback = Ipv6Addr(bits: U128(1))
	let v6 = SocketAddr.v6(SocketAddrV6(ip: loopback, port: 443, scopeId: 3))
	print("ports \(ports([v4, v6]))")
	let peer = Peer(addr: v6, names: ["a", "b"]).next("c")
	print("peer \(debug(peer)) \(peer.aliases())")
	let file = file("dir", "f\\'\"\t")
	print("file \(escaped(file))")
	print("parents \(parents([file, "/"]).map { $0.map(escaped) ?? "none" }.joined(separator: " "))")
	let words = words(" one two  three ")
	print("words \(words.count) \(words)")
	print("sentence \(sentence(words)) \(sentence(nil))")
	print("sentence \(sentence([]))|")
	print("either \(either("abc")) \(either(nil))")
	print("prefixed \(String(decoding: prefixed(Bytes("cd".utf8), Bytes("ab".utf8)), as: UTF8.self)) \(String(decoding: prefixed(Bytes("cd".utf8), nil), as: UTF8.self))")
	print("coordinates \(coordinates(Point(x: 2, y: 3))) \(coordinates(nil))")
	do {
		// An optional borrowed handle is an overload with it and one without; one borrowed
		// mutably is an optional `inout`.
		let two = Counter.new("two", 2)
		var three: Counter? = Counter.new("three", 3)
		var none: Counter? = nil
		print("plus \(try! two.plus(other: three!)) \(try! two.plus())")
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
			try! two.clamped() == 2 && two.clamped(low: three!) == 3 && two.clamped(high: one) == 1
				&& two.clamped(low: one, high: three!) == 2,
			"clamped overloads each optional borrow"
		)
		print("add_to \(try! two.addTo(&three)) \(try! two.addTo(&none)) \(try! three!.get())")
	}
	do {
		var encoded = Bytes("0,".utf8)
		putNumber(1, &encoded)
		putNumber(23, &encoded)
		print("put_number \(String(decoding: encoded, as: UTF8.self))")
		var entry = (Bytes("ab".utf8), UInt32(1))
		let stamps = pushCounted(&entry, UInt8(ascii: "c"))
		print("push_counted \(stamps) \(String(decoding: entry.0, as: UTF8.self)) \(entry.1)")
		var digits = Bytes("7x9".utf8)
		let firstDigit = try! digit(&digits)
		let skipped = (try? digit(&digits)) == nil
		let lastDigit = try! digit(&digits)
		print("digit \(firstDigit) \(skipped) \(lastDigit) \(digits.count)")
		var nudged = Point(x: 1, y: 2)
		nudge(&nudged, 3)
		print("nudge \(nudged.x) \(nudged.y)")
	}
	do {
		let config = secure("h3")
		print("secure \(try! alpnLent(config)) \(try! alpn(config.clone()))")
		let cert = certify(Bytes("der".utf8))
		let certLength = try! certLen(cert)
		let der = text(try! certDer(cert))
		var chain = UniqueArray<TlsCert>()
		chain.append(certify(Bytes("a".utf8)))
		chain.append(certify(Bytes("bc".utf8)))
		print("cert \(certLength) \(der) \(try! chainLen(chain))")
		// Another crate's trait objects: an `Arc`, taken back, and `'static` ones, held by value.
		print("resolver \(try! resolved(resolver("r1")))")
		var codecs = Codecs.new()
		print("codecs \(try! codecs.ids())")
		var listed = try! codecs.listGet()
		var reversed = UniqueArray<TlsCodec>()
		reversed.append(listed.remove(at: 1))
		reversed.append(listed.remove(at: 0))
		try! codecs.listSet(reversed)
		print("codecs reversed \(try! codecs.ids())")
		// A standard library value no type maps, in a handle.
		let failed = failure("boom")
		let message = try! failureMessage(failed)
		print("failure \(message) \(try! failureKind(failed))")
		// Iterators, passed as the arrays of their items.
		print("sum_all \(sumAll([1, 2, 3]))")
		var weighed = UniqueArray<Counter>()
		weighed.append(Counter.new("w", 3))
		print("weigh_all \(try! weighAllCounter(weighed))")
		print("offered_len \(offeredLen([Bytes("h3".utf8), Bytes("moq-00".utf8)]))")
		print("join_words \(joinWords(["a", "b", "c"]))")
		// Bytes that must be `'static`, interned.
		print("der \(text(Der.fromSlice(Bytes("der".utf8)).bytes()))")
		print("der again \(text(Der.fromSlice(Bytes("der".utf8)).bytes()))")
		let session = try! resume("h3", 60)
		print("resume \(session.alpn()) \(session.lifetime().seconds)")
		do {
			_ = try resume("", 60)
		} catch let e as StringError {
			print("resume err \(e.error)")
		} catch {
			check(false, "resume throws its message")
		}
		var meters = Meters.new()
		let first = try! meters.first()
		_ = try! first.add(2)
		print("lent \(try! first.add(3)) \(try! meters.first().count())")
		try! meters.clear()
		print("cleared \(try! meters.first().count())")
		let last = try! meters.lastGet()
		_ = try! last.add(4)
		print("field lent \(try! last.add(1))")
		// An optional field that is not Clone, lent as `Some` of a handle.
		var hasSpare = false
		if let _ = try! meters.spareGet() {
			hasSpare = true
		}
		print("spare \(hasSpare)")
		try! meters.spareSet(Meter.default())
		if let spare = try! meters.spareGet() {
			_ = try! spare.add(5)
			print("spare lent \(try! spare.add(1))")
		}
		let tallies = Tallies.new()
		_ = try! tallies.first().bump()
		print("tallied \(try! tallies.first().bump())")
	}
	do {
		// A draft borrowing its ledger exclusively until it is consumed, through which its
		// sheet's members are reached by Deref and DerefMut: the ledger refuses any other use
		// meanwhile.
		var ledger = Ledger.new()
		var draft = try! ledger.edit()
		try! draft.titleSet("minutes")
		try! draft.write("first")
		print("draft \(try! draft.titleGet) \(try! draft.lines()) \(try! draft.describe()) \(try! draft.edited())")
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
		print("committed \(try! draft.commit())")
		print("ledger \(try! ledger.title()) \(try! ledger.published())")
		let read = try! ledger.edit()
		print("read \(try! read.lines()) \(try! read.edited())")
		_ = consume read
		var second = try! ledger.edit()
		try! second.write("second")
		_ = consume second
		print("dropped \(try! ledger.published())")
		print("receipt \(try! ledger.receipt().count())")
		try! ledger.seal()
		do {
			_ = try ledger.edit()
			check(false, "a sealed ledger refuses a draft")
		} catch let e as Api.Error {
			print("sealed \(e)")
		} catch {
			check(false, "a sealed ledger throws an Error")
		}
	}
	print("chunks \(chunks(Bytes("abcde".utf8), 2).map { Array($0) })")
	var counters = counters(["x", "y"])
	print("counters \(listed(counters) { "\(try! $0.name()) \(try! $0.get())" })")
	// Each counter moves in.
	var some = UniqueArray<Counter>()
	some.append(counters.remove(at: 0))
	var boxed = UniqueArray<Counter>()
	boxed.append(counters.remove(at: 0))
	print("worth \(try! worth(some, boxed)) \(try! worth(nil, UniqueArray()))")
	print("held \([try! heldCount(Counter.new("h", 4)), try! heldCount(nil)].map { $0.map { "Some(\($0))" } ?? "None" }.joined(separator: " "))")
	print("boxed \(try! boxedName(boxedCounter("b")).map { "Some(\"\($0)\")" } ?? "None") \(try! boxedName(nil) ?? "None") \(boxedCounter(nil) == nil)")
	print("less \(less(5, 2))")
	print("sums \(sums([[1, 2], [], [3]]))")
	print("least \(least([[3, 1], [], [2]]).map { "\($0)" }.joined(separator: " "))")
	numbers("edges", Numbers.edges())
	numbers("reversed", Numbers.edges().reversed())
	let edges = Numbers.edges()
	let nan = Measure.new(Double.nan, false)
	print("float equal \(edges == edges) \(edges == Numbers.edges()) \(Numbers.zero(true) == Numbers.zero(false)) \(nan == nan) \(Measure.new(-0.0, true) == Measure.new(0.0, false))")
	print("sum u32 \(sumU32([])) \(sumU32([UInt32.max, UInt32.max, 2]))")
	print("doubled i64 \(doubledI64([Int64.min, -1, 0, Int64.max]))")
	print("boxed floats \(boxedFloats([-0.0, Double(bitPattern: 0x7ff8_0000_0000_0001)]).map(\.bitPattern))")
	let sum = summed([Point(x: 1, y: 2), Point(x: -4, y: 6)])
	let noneSum = summed([])
	print("summed \(sum.x) \(sum.y) \(noneSum.x) \(noneSum.y)")
	print("longest \(longest(["a", "abc"])) \(longest([]))")
	print("first of Some(\"\(firstOf(["a", "b"])!)\") \(firstOf([]) ?? "None")")
	let limits = byteLimits(Bytes("a".utf8), Bytes("bc".utf8))
	let waits = breadth(LimitsDuration(low: .milliseconds(5), high: .seconds(2)))
	let low = String(decoding: limits.low, as: UTF8.self)
	let high = String(decoding: limits.high, as: UTF8.self)
	print("limits \(low) \(high) breadth \(waits.secs * 1000 + UInt64(waits.nanos) / 1_000_000)")
	let parts = String(decoding: Api.joined([Bytes("ab".utf8), Bytes(), Bytes("c".utf8)]), as: UTF8.self)
	print("joined \(parts) \(Api.joined([]).count) read \(readLen(Bytes("hello".utf8)))")
	let doc = try! Json.parse(#"{"b": [1, 2.5, null], "a": "é"}"#)
	print("json \(try! encode(doc))")
	print("timed text \(try! timedText(doc))")
	let tagged = try! tagged(doc, "t")
	print("tagged \(try! tagged.toText())")
	// A JSON object's entries cross as pairs of a key and its document, each moved in.
	var pairs = UniqueArray<TupleStringJson>()
	pairs.append(TupleStringJson("b", try! Json.parse("[1]")))
	pairs.append(TupleStringJson("a", try! Json.parse("2")))
	let jsonEntries = try! entries(pairs, "t")
	print("entries \(listed(jsonEntries) { "\($0._0) \(try! $0._1.toText())" })")
	do {
		_ = try Json.parse("{\"a\":")
	} catch let e as StringError {
		print("json parse err \(e.error)")
	} catch {
		check(false, "parse throws its message")
	}
	let site = try! Site.new(base)
	print("site \(try! site.homeGet().description) \(try! site.homeGet().description)")

	// Slices, passed in and returned as clones, and the socket address of each family.
	print("shapes \(Shape.ALL) greetings \(GREETINGS)")
	print("both \(Shape.BOTH) primes \(PRIMES)")
	var trail = Trail.new([Point(x: 1, y: 2), Point(x: 3, y: 4)], ["a", "b"])
	print("trail \(try! trail.points()) \(try! trail.labels())")
	// Returned iterators, each collected at the call: drained labels are gone after.
	print("iterated \(try! trail.iter()) \(try! trail.names()) \(Array(try! trail.bytes()))")
	print("drained \(try! trail.drain()) then \(try! trail.labels())")
	// A filter, collected at the call.
	print("beyond \(try! trail.beyond(1).map { "\($0.x) \($0.y)" }.joined(separator: ", "))")
	let numbered = numbered(["p", "q"])
	print("numbered \(listed(numbered) { "\(try! $0.name()) \(try! $0.get())" })")
	let (next4, next6) = nexts(SocketAddrV4(ip: Ipv4Addr(127, 0, 0, 1), port: 8080), SocketAddrV6(ip: loopback, port: 443, scopeId: 3))
	print("nexts \(next4.ip.octets.map(String.init).joined(separator: ".")) \(next4.port) \(next6.port) \(next6.scopeId)")
	let (nextIp4, nextIp6) = nextIps(Ipv4Addr(10, 0, 0, 255), Ipv6Addr(bits: U128(0xffff)))
	let mapped = flip(.v4(Ipv4Addr(127, 0, 0, 1)))
	let ips = [IpAddr.v4(nextIp4), .v6(nextIp6), mapped, flip(mapped), flip(.v6(loopback))]
	print("ips \(ips.map(ipHex).joined(separator: " "))")
	lossless()

	// serde_json's map iterator, collected as pairs of a key and its document.
	var sectionPairs = UniqueArray<TupleStringJson>()
	sectionPairs.append(TupleStringJson("b", try! Json.parse("[1]")))
	sectionPairs.append(TupleStringJson("a", try! Json.parse("2")))
	let jsonSections = try! sections(sectionPairs)
	print("sections \(listed(jsonSections) { "\($0._0) \(try! $0._1.toText())" })")
	// An instant only Rust makes, which each call copies out of its handle, and compares.
	let millis = { (d: RustDuration) in d.secs * 1000 + UInt64(d.nanos) / 1_000_000 }
	let began = now()
	let ended = later(began, .milliseconds(1500))
	print("instants \(millis(since(ended, began))) \(millis(since(began, ended))) \(began == later(began, .seconds(0))) \(began == ended)")
	print("has_instant \(hasInstant(began)) \(hasInstant(nil))")
	// tracing's level, and its filter as an optional level whose nil is `OFF`.
	var verbosity = Verbosity.new(.warn)
	print("verbosity \(spell(try! verbosity.filter())) \(spell(louder(try! verbosity.filter())))")
	try! verbosity.levelSet(.trace)
	print("verbosity \(try! verbosity.levelGet == .trace) \(spell(try! verbosity.filter())) \(spell(louder(try! verbosity.filter()))) \(spell(louder(nil)))")

	print("made \(made(3) { "\($0) times" })")
	// A closure taking and returning `Copy` objects copies them, never running their `Clone`.
	let nine = Ticket.new(9)
	let given = reissue(Ticket.new(7)) { Ticket.new($0.number() + 1) }.number()
	let held = reissue(Ticket.new(7)) { _ in nine }.number()
	print("reissued \(given) \(held), cloned \(Ticket.clones())")
	// A record's traits borrow the checked handle it holds.
	let nines = Mark.of(9)
	let front = try! seat(nines, 1)
	let back = try! seat(nines, 2)
	print("seat \(try! front.debugDescription) \(try! front == front) \(try! front == back)")
	// A kept closure, called on another thread after the call that took it returned.
	print("spawned \(try! Spawned.new { 7 }.join())")
	// A kept closure: called again at the instant it asks for, then once its waiter wakes.
	let start = Moment.new(0, 1000)!
	let calls = Cell(0)
	let stopped = block {
		try await drive(start) { now, waiter in
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
	print("non zeros \(describeNonZeros(nonZeros()))")
	for hop in [nil, 1, UInt64.max] as [UInt64?] {
		print("hop \(nextHop(hop.map { try! NonZero($0) }).map { "Some(\($0))" } ?? "None")")
	}
	print("farther hop \(fartherHop(PairNonZeroU16(left: try! NonZero(3), right: try! NonZero(.max))))")
	do {
		let local = Local.new()
		let shared = try! localOrNew(local: local)
		let bumped = try! local.bump()
		print("local \(bumped) \(try! shared.bump())")
		print("local fresh \(try! localOrNew().bump())")
		print("local either \(try! bumpEither(shared))")
		// Made on this thread, so awaited on the executor the script drains on it.
		let held = Held(local)
		let later = Cell<UInt32>(0)
		q.launch { later.value = try await localOrNewLater(local: held.local).bump() }
		q.drain()
		print("local later \(later.value)")
		print("local later fresh \(block { try await localOrNewLater().bump() })")
	}
	// A zero, which Swift's integer holds and Rust's `NonZero` does not, makes none, and a
	// record holding one keeps the protocols its traits give.
	check(refused("NonZero(_:)") { _ = try NonZero<UInt64>(0) }, "a zero makes no NonZero")
	check(Set([try! NonZero<Int8>(-1), try! NonZero(-1), try! NonZero(.min)]).count == 2, "a set of NonZero")
	check(try! NonZero<Int8>(.min) < NonZero(1), "a NonZero compares as its integer")
	let debug: any CustomDebugStringConvertible = nonZeros()
	check(debug.debugDescription == describeNonZeros(nonZeros()), "a record of NonZero is Debug")
	targets()
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
		let spawned = Spawned.new { [held = Released(freed)] in
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

/// `bytes` in hex, two digits each.
func hexBytes(_ bytes: some Sequence<UInt8>) -> String {
	bytes.map { String($0, radix: 16).leftPadded(2) }.joined()
}

/// An IP address's octets in hex, so their length shows its family.
func ipHex(_ ip: IpAddr) -> String {
	switch ip {
	case .v4(let ip): return hexBytes(ip.octets)
	case .v6(let ip): return hexBytes(ip.octets)
	}
}

/// A socket address as its IP's octets in hex, its port, flow label, and scope, each
/// IPv6's alone, apart by colons.
func socketHex(_ addr: SocketAddr) -> String {
	switch addr {
	case .v4(let v4): return "\(hexBytes(v4.ip.octets)):\(v4.port)"
	case .v6(let v6): return "\(hexBytes(v6.ip.octets)):\(v6.port):\(v6.flowinfo):\(v6.scopeId)"
	}
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

/// Each type's edge values, passed through Rust and back, as transcript.rs prints them,
/// and the conversions to and from each runtime type, which throw or return nil for a
/// value Rust's type cannot hold.
func lossless() {
	let spans = [
		RustDuration.seconds(0), .nanoseconds(1), .nanoseconds(UInt64(Int64.max) + 1),
		try! RustDuration(secs: .max, nanos: 999_999_999),
	].map { sameDuration($0) }
	print("lossless duration \(spans.map { "\($0.secs).\(String($0.nanos).leftPadded(9))" }.joined(separator: " "))")
	print("lossless u64 \(sameU64(0)) \(sameU64(1 << 63)) \(sameU64(.max))")
	print("lossless usize \(sameUsize(0)) \(sameUsize(.max))")
	print("lossless isize \(sameIsize(.min)) \(sameIsize(.max))")
	let wides = [U128(0), U128(high: 1, low: 0), U128(high: .max, low: .max)].map { sameU128($0) }
	print("lossless u128 \(wides.map { hexBytes(withUnsafeBytes(of: ($0.high.bigEndian, $0.low.bigEndian)) { Array($0) }) }.joined(separator: " "))")
	let chars: [Unicode.Scalar] = ["\0", "\u{d7ff}", "\u{e000}", "\u{10ffff}"]
	print("lossless char \(chars.map { String(sameChar($0).value, radix: 16) }.joined(separator: " "))")
	check(sameString("a\u{0}é\u{10ffff}") == "a\u{0}é\u{10ffff}", "text crosses")
	// A path that is not UTF-8, which no String holds, keeps its bytes.
	let strayPath = try! stray()
	check(strayPath.string == nil && PathBuf("a/é").string == "a/é", "a path's text is nil unless UTF-8")
	print("lossless path \(hexBytes(samePath("a/é").bytes)) \(hexBytes(samePath(strayPath).bytes))")
	check(samePath(strayPath) == strayPath, "a path that is not UTF-8 crosses back")
	try! visitStray { path in check(path == strayPath, "a lent path that is not UTF-8 keeps its bytes") }
	check(samePath(PathBuf(bytes: Bytes([0xff]))) == strayPath, "a caller's bytes make a path")
	let v4 = sameSocketAddrV4(SocketAddrV4(ip: Ipv4Addr(bits: .max), port: .max))
	let ip6 = Ipv6Addr(bits: U128(high: 1 << 48, low: 2))
	let v6 = sameSocketAddrV6(SocketAddrV6(ip: ip6, port: 3, flowinfo: .max, scopeId: .max))
	let addrs = [SocketAddr.v4(v4), .v6(v6)]
	print("lossless socket \(addrs.map(socketHex).joined(separator: " ")) \(addrs.map { socketHex(sameSocketAddr($0)) }.joined(separator: " "))")
	// A key hashes by Rust's `Hash`, which agrees with its `==`.
	let a = Key.new("a")
	let b = Key.new("b")
	print("key \(hashed(a) == hashed(Key.new("a"))) \(hashed(a) == hashed(b))")
	let edge6 = Ipv6Addr(octets: [0xff, 0xff] + Array(repeating: 0, count: 13) + [1])!
	print("lossless ip \(ipHex(sameIpAddr(.v4(Ipv4Addr(255, 255, 255, 255))))) \(ipHex(sameIpAddr(.v6(edge6))))")
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
		case nil: script()
		case "futures": futures()
		case "owned": owned()
		case "shared": shared()
		case let which?: broken(which)
		}
	}
}
