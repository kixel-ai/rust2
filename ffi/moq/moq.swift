// moq.c's script through the generated Swift: a broadcast's track, published and
// subscribed to in one process, with every read an async func, and an origin, with
// moq-pattern, kio, moq-tokio, hang, and moq-mux exported into the same library,
// and the catalog lent to Swift for a call.
//
// The local reads need no runtime: each read that must wait resumes on an executor the
// script drains, after the write that wakes it. A session over QUIC runs on moq-tokio's
// fallback runtime, whose threads resume Swift's.

import BasicContainers
import Foundation
import Moq

nonisolated(unsafe) var failures = 0

func expect(_ ok: Bool, _ what: String) {
	if !ok {
		FileHandle.standardError.write(Data("failed: \(what)\n".utf8))
		failures += 1
	}
}

func text(_ data: Bytes?) -> String? {
	data.map { String(decoding: $0, as: UTF8.self) }
}

/// A value a task sets and the script reads, under a lock.
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

/// A handle a task calls `mutating` methods on: an escaping closure cannot capture a
/// noncopyable `var`, but may a class holding one.
final class Held<T: ~Copyable>: @unchecked Sendable {
	var value: T

	init(_ value: consuming T) {
		self.value = value
	}
}

/// A handle a task takes and the script reads, under a lock: a struct Swift cannot copy, so
/// it moves in and out.
final class Slot<T: ~Copyable>: @unchecked Sendable {
	private let lock = NSLock()
	private var stored: T? = nil

	func put(_ value: consuming T) {
		lock.lock()
		defer { lock.unlock() }
		stored = consume value
	}

	func take() -> T? {
		lock.lock()
		defer { lock.unlock() }
		return stored.take()
	}

	var isEmpty: Bool {
		lock.lock()
		defer { lock.unlock() }
		return stored == nil
	}
}

/// The jobs tasks resume with, run when the script drains them.
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

	func launch(_ body: @escaping @Sendable () async throws -> Void) {
		_ = Task(executorPreference: self, operation: body)
	}
}

/// `body`'s result, waiting on this thread for it to finish on the global executor.
func block<T: ~Copyable>(_ body: @escaping @Sendable () async throws -> T) throws -> T {
	let done = DispatchSemaphore(value: 0)
	let result = Slot<T>()
	let failure = Cell<Swift.Error?>(nil)
	Task {
		do {
			result.put(try await body())
		} catch {
			failure.value = error
		}
		done.signal()
	}
	done.wait()
	if let error = failure.value {
		throw error
	}
	return result.take()!
}

/// Whether `path` is `other`: `==` takes a `Path`, which an optional one holds.
func same(_ path: borrowing Path?, _ other: borrowing Path) -> Bool {
	switch path {
	case .some(let path): return path == other
	case .none: return false
	}
}

/// An origin, scoped, published, and polled once by `waiter`, returning its driver, with
/// `before` the wakes counted before the origin, along with every object of it, is
/// released on return.
func origin(_ waiter: borrowing Waiter, _ wakes: Cell<Int>, _ before: inout Int) throws -> OriginDriver {
	// An origin and the driver that runs it, a pair. Its root, which it lends, is returned
	// as a new handle to a clone.
	let pair = OriginProducer.new(OriginConfig.default())
	let origin = pair._0
	var driver = pair._1
	expect(origin.root().asStr() == "", "an origin's root is empty")

	// Scoped beneath a root to patterns, which moq-pattern's own export builds.
	var patterns = Patterns.new()
	expect(try patterns.insert(Pattern.subtree("alice")), "a pattern inserts")
	let scoped = try origin.scope("room", patterns)
	expect(scoped.root().asStr() == "room", "a scope has its root")
	expect(try scoped.absolute("alice").asStr() == "room/alice", "a scope's paths are absolute")
	let allowed = scoped.allowed()
	expect(allowed.matches("alice/cam"), "a scope allows its patterns")
	expect(!allowed.matches("bob"), "a scope allows nothing else")
	let consumer = scoped.consume()
	expect(try consumer.absolute("bob").asStr() == "room/bob", "a scope's consumer shares its root")

	// A broadcast the origin creates is stamped with its path, which its info lends.
	do {
		let publication = try origin.createBroadcast("room/alice")
		let path = publication.broadcast().info().pathGet
		expect(path.asStr() == "room/alice", "a broadcast has its path")

		// Paths own their segments, so Swift builds them, and relative ones, from a string.
		do {
			_ = try Path.new("/cam/")
			expect(false, "a path with an empty segment is an error")
		} catch let e as PathError {
			expect(e == .emptySegment, "an empty segment is its own error")
		}
		let cam = try Path.new("cam")
		expect(cam.asStr() == "cam", "a path builds from its segments")
		let joined = try path.join(cam)
		expect(joined.asStr() == "room/alice/cam", "paths join")
		expect(same(joined.stripPrefix(path), cam), "a path strips its prefix")
		expect(path.stripPrefix(joined) == nil, "a path strips no longer prefix")
		expect(try joined.resolve(PathRelative.new("../bob")).asStr() == "room/bob", "a relative path resolves")
	}

	_ = try driver.poll(clockNow(), waiter)
	// A client publishes the origin through moq-net's generic `with_publisher`, at each
	// type that implements its bound, by reference, so Swift keeps its handle.
	_ = Client.new().withPublisherProducer(origin)
	before = wakes.value
	return driver
}

func script() throws {
	let q = Queue()

	let broadcast = BroadcastInfo.new().produce()
	let track = try broadcast.createTrack("video", nil)
	expect(track.name() == "video", "a track has its name")

	// A second track of the same name is an error, printed as Rust prints it.
	do {
		_ = try broadcast.createTrack("video", nil)
		expect(false, "a duplicate track is an error")
	} catch let e as Moq.Error {
		expect("\(e)" == "duplicate", "a duplicate track's error is Rust's")
	}

	let subscriber = Held(try block { try await track.consume().subscribe(nil) })

	// A group's arrival wakes the subscriber's pending receive.
	let group = Slot<GroupConsumer>()
	q.launch {
		if let received = try await subscriber.value.recvGroup() {
			group.put(received)
		}
	}
	q.drain()
	expect(group.isEmpty, "a receive waits for a group")
	var writer = try track.appendGroup()
	expect(q.size == 1, "a group's arrival wakes the receive")
	q.drain()
	let first = Held(group.take()!)
	expect(first.value.index() == 0, "the first group is 0")

	// So does a frame's.
	let frame = Slot<FrameFrame>()
	q.launch {
		if let read = try await first.value.readFrame() {
			frame.put(read)
		}
	}
	q.drain()
	expect(frame.isEmpty, "a read waits for a frame")
	try writer.writeFrame(nil, Bytes("hello".utf8))
	expect(q.size == 1, "a frame's arrival wakes the read")
	q.drain()
	let hello = frame.take()!
	expect(text(hello.payloadGet) == "hello", "a frame has its payload")
	expect(hello.timestampGet == nil, "an untimed frame has no timestamp")
	// When it arrived: kio's clock's `Instant`, from kio's own export.
	expect(clockNow().checkedDurationSince(hello.arrived()) != nil, "a frame arrived before now")

	// A timestamp is a `Copy` value behind a handle: passing it copies it out. The track is
	// untimed, so a timestamped frame is refused.
	let at = try Timestamp.fromMillis(1500)
	expect(at.asSecs() == 1, "1500 ms is 1 s")
	do {
		try writer.writeFrame(at, Bytes("late".utf8))
		expect(false, "a timestamped frame on an untimed track is an error")
	} catch let e as Moq.Error {
		expect("\(e)" == "frame timestamp doesn't fit its track", "the late frame's error is Rust's")
	}
	expect(at.asSecs() == 1, "a copied timestamp stays usable")

	// The last frame, bytes that are not UTF-8, then the group's end, which reads as nil.
	let world = Bytes(Data([0xff]) + Data("world".utf8))
	try writer.writeFrame(nil, world)
	try writer.finish()
	try block {
		let last = try await first.value.readFrame()
		expect(last?.payloadGet == world, "the last frame reads")
		expect(try await first.value.readFrame() == nil, "a finished group reads nil")
	}

	// The driver, polled as a language's event loop would: at the time, and with a waiter
	// made from the waker and held across polls, since what a poll registers lives only as
	// long as its waiter. It runs until every owner of the origin is gone, which a function
	// of its own ends by returning.
	let wakes = Cell(0)
	let waiter = Waiter.new { wakes.value += 1 }
	var before = 0
	var driver = try origin(waiter, wakes, &before)
	expect(wakes.value > before, "the last owner's release wakes the driver")
	do {
		_ = try driver.poll(clockNow(), waiter)
		expect(false, "a driver whose origin is gone finishes")
	} catch let e as Moq.Error {
		expect("\(e)" == "closed", "a finished driver's error is Rust's")
	}

	// moq-mux reads an OpusHead into hang's catalog config: 44.1 kHz in, stereo, which the
	// catalog lists at the decoder's 48 kHz, keeping the head as its description.
	let head = Bytes(Data("OpusHead".utf8) + Data([1, 2, 0x38, 0x01, 0x44, 0xac, 0, 0, 0, 0, 0]))
	let audio = try muxCodecOpusConfig(head)
	expect(audio.sampleRateGet == 48000, "Opus decodes at 48 kHz")
	expect(audio.channelCountGet == 2, "the head is stereo")
	expect(audio.codecGet == .opus, "the codec is Opus")
	expect(audio.descriptionGet == head, "the head is the description")
	// A truncated head is moq-mux's error.
	do {
		_ = try muxCodecOpusConfig(head.prefix(8))
		expect(false, "a truncated head is an error")
	} catch let e as MuxError {
		expect(!"\(e)".isEmpty, "a truncated head's error has a message")
	}

	// A subscription's ranges, from any iterable of them: an array.
	let groups = TrackRange.groups(BoundsU64(start: .included(2), end: .excluded(6)))
	let ranged = TrackSubscription.default().withRanges([TrackRange.live(), groups]).rangesGet
	expect(ranged == [TrackRange.live(), groups], "a subscription keeps its ranges")
	expect(groups.start == .absolute(2), "groups start at their first")
	expect(groups.end == .absolute(6), "groups end before their end")

	// A catalog at moq-mux's untyped extension, from the config Swift builds, whose generic
	// `is_claimed` is named by the last segment of its argument, and whose `mutate` lends
	// the catalog to Swift.
	var publisher = BroadcastInfo.new().produce()
	let extra = MuxCatalogConfig.default().withCatalogExtra(HangCatalogExtra.default())
	var catalog = try MuxCatalogProducerExtra.new(&publisher, extra)
	expect(!catalog.isClaimedBinaryConfig("state"), "an unclaimed name is not claimed")
	let read = Cell(0)
	try catalog.mutate { lent in
		if try! lent.toJson().hasPrefix("{") {
			read.value += 1
		}
	}
	expect(read.value == 1, "mutate lends the catalog once")

	// A JSON track the catalog lists, published through moq-mux's generic `json_snapshot` at
	// a JSON document, and a section Swift parses from text, set in the lent catalog and
	// read back as text.
	var status = try catalog.jsonSnapshotConfig(
		try publisher.createTrack("status", TrackInfo.default().withTimescale(Timescale.MILLI)), MuxJsonConfig.default(), MuxJsonOptions.default())
	expect(status.name() == "status", "a JSON snapshot has its track's name")
	expect(catalog.isClaimedJsonConfig("status"), "the JSON snapshot claims its name")
	// A value Swift parses is published as the track's latest, which a subscriber reads
	// back as JSON text.
	let on = try Json.parse(#"{"on": true}"#)
	try status.update(on, Timestamp.now())
	let latest = Held(status.consume())
	try block {
		var group = try await latest.value.recvGroup()
		let frame = try await group?.readFrame()
		expect(text(frame?.payloadGet) == #"{"on":true}"#, "a subscriber reads the latest value")
	}
	try catalog.mutate { lent in
		var sections = lent.extGet
		try! sections.set("app", Json.parse(#"{"v": [1, true]}"#))
		lent.extSet(sections)
	}
	let app = Cell<String?>(nil)
	try catalog.mutate { lent in
		expect(try! lent.toJson().contains("\"status\""), "the catalog lists the JSON track")
		app.value = lent.extGet.get("app")?.toText()
	}
	expect(app.value == #"{"v":[1,true]}"#, "the catalog reads its section back as text")
	// Malformed text fails to parse, and a reserved section is moq-mux's error.
	do {
		_ = try Json.parse(#"{"v":"#)
		expect(false, "malformed JSON fails to parse")
	} catch let e as StringError {
		expect(e.error == "EOF while parsing a value at line 1 column 5", "a parse error is serde_json's")
	}
	do {
		var hang = MuxCatalogHangExtra.default()
		try hang.set("json", Json.parse("1"))
		expect(false, "a reserved section is an error")
	} catch let e as MuxError {
		expect("\(e)" == "reserved catalog section: json", "a reserved section is moq-mux's error")
	}

	// moq-tokio's terminal connect errors, which moq-net's `Error` shares no names with.
	expect(TokioConnectError.unauthorized.isAuth(), "unauthorized is an auth error")
	expect("\(TokioConnectError.unauthorized)" == "unauthorized", "a connect error prints as Rust's")

	// Session config's foreign types. A URL is parsed from text, failing with why.
	do {
		_ = try Url.parse("relay.example")
		expect(false, "a relative URL is an error")
	} catch let e as StringError {
		expect(e.error == "relative URL without a base", "a URL's parse error is Rust's")
	}
	// A session's config, its URL and pinned fingerprints set as Rust assigns them.
	var connect = TokioConnectConfig.default()
	var tls = connect.tlsGet
	tls.fingerprintSet(["ab:cd"])
	connect.tlsSet(tls)
	connect.urlSet(try Url.parse("https://relay.example:4443/demo"))
	expect(connect.tlsGet.fingerprintGet == ["ab:cd"], "a TLS config keeps its fingerprints")
	let url = connect.urlGet!
	let versions = Versions.default()
	let dialed = AuthTokens.new(UniqueArray()).dial(url, versions).urlGet
	expect(dialed == url, "a dial keeps its URL")
	expect(dialed.description == "https://relay.example:4443/demo", "a URL prints as Rust's")
	// A `Vec<String>`, as an array.
	expect(!ALPNS.isEmpty, "a constant slice crosses as an array")
	let alpns = versions.alpns()
	expect(!alpns.isEmpty && alpns.allSatisfy { !$0.isEmpty }, "each version has an ALPN")
	// A socket address, crossing in a listener's bind.
	let loopback: [UInt8] = Array(repeating: 0, count: 15) + [1]
	let bind = TokioListenBind.addr(SocketAddr(ip: loopback, port: 4443))
	expect("\(bind)" == "[::1]:4443", "an IPv6 bind prints as Rust's")
	let v4 = SocketAddr(ip: [127, 0, 0, 1], port: 0)
	expect("\(TokioListenBind.addr(v4))" == "127.0.0.1:0", "an IPv4 bind prints as Rust's")
	var listen = TokioListenConfig.default()
	listen.bindSet(bind)
	// The preferred address of each family, which takes only that family.
	listen.preferredV4Set(v4)
	expect(listen.preferredV4Get == v4 && listen.preferredV6Get == nil, "a listener keeps its preferred IPv4 address")
	expect(listen.bindGet == bind, "a listener's config keeps its bind")
	// TLS files, as OS paths, and fingerprints, as strings: none by default.
	let defaults = TokioTlsConnect.default()
	expect(defaults.rootGet.isEmpty && defaults.fingerprintGet.isEmpty, "a default TLS config lists none")

	// A session, as moq.c runs it: a server bound to port 0 reads back the port it got and
	// serves with one call, in a task of its own, and a client dials it with one call and
	// reads a frame back, each async func resuming once moq-tokio's fallback runtime wakes
	// it from its threads. A stall fails the script.
	DispatchQueue.global().asyncAfter(deadline: .now() + 30) {
		FileHandle.standardError.write(Data("failed: the session stalled\n".utf8))
		exit(1)
	}
	try block {
		let served = tokioOriginSpawn()
		let live = try served.createBroadcast("live")
		try live.announce(OriginRoute.default())
		let liveTrack = try live.broadcast().createTrack("video", nil)
		var liveGroup = try liveTrack.appendGroup()
		try liveGroup.writeFrame(nil, Bytes("hello".utf8))
		try liveGroup.finish()

		var serverConfig = TokioServerConfig.default()
		var listen = serverConfig.listenGet
		listen.bindSet(.addr(SocketAddr(ip: Array(repeating: 0, count: 16), port: 0)))
		var tls = listen.tlsGet
		tls.generateSet(["localhost"])
		listen.tlsSet(tls)
		serverConfig.listenSet(listen)
		let server = try serverConfig.init_()
		let local = try server.localAddr()
		expect(local.port != 0, "a server bound to port 0 reads back the port it got")
		// Serving consumes the server, which the task takes.
		let owned = Slot<TokioServer>()
		owned.put(server)
		let serving = Task { try await owned.take()!.servePublish(served.consume()) }

		var connect = TokioConnectConfig.default()
		connect.urlSet(try Url.parse("moqt://localhost:\(local.port)"))
		var insecure = connect.tlsGet
		insecure.insecureSet(true)
		connect.tlsSet(insecure)
		var clientConfig = TokioClientConfig.default()
		clientConfig.connectSet(connect)
		let client = try clientConfig.init_()
		let received = tokioOriginSpawn()
		let remote = received.consume()
		let connection = client.clone().withReconnect(false).consume(received)
		expect(connection != nil, "a client with a URL dials it")
		let established = try await connection!.established()

		var subscriber = try await remote.request("live", nil).track("video").subscribe(nil)
		var group = try await subscriber.recvGroup()
		let frame = try await group?.readFrame()
		expect(text(frame?.payloadGet) == "hello", "a session reads the frame the server serves")

		try await established.close()
		try await client.close()
		serving.cancel()
	}
}

@main
struct Main {
	static func main() {
		do {
			try script()
		} catch {
			expect(false, "the script threw \(error)")
		}
		if failures == 0 {
			print("ok")
		}
		exit(failures == 0 ? 0 : 1)
	}
}
