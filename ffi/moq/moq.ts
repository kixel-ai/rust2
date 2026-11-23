// moq.c's script through the React Native binding, on Hermes off a device: a broadcast's
// track, published and subscribed to in one process, with every read a promise, and an
// origin, its driver polled from the event loop, with moq-pattern, kio, moq-tokio, hang, and
// moq-mux exported into the same library, and the catalog lent to JavaScript for a call.
//
// The local reads need no runtime: each read that must wait pends until the write that
// wakes it posts its next poll to the JavaScript thread. A session over QUIC runs on
// moq-tokio's fallback runtime, whose threads post each wake there too.

import * as moq from "./react-native/index.ts";
import * as BroadcastInfo from "./react-native/BroadcastInfo.ts";
import * as BroadcastConsumer from "./react-native/BroadcastConsumer.ts";
import * as BroadcastProducer from "./react-native/BroadcastProducer.ts";
import * as Client from "./react-native/Client.ts";
import * as ClockInstant from "./react-native/ClockInstant.ts";
import * as Error_ from "./react-native/Error_.ts";
import * as FrameFrame from "./react-native/FrameFrame.ts";
import * as GroupConsumer from "./react-native/GroupConsumer.ts";
import * as GroupProducer from "./react-native/GroupProducer.ts";
import * as HangCatalogAudioConfig from "./react-native/HangCatalogAudioConfig.ts";
import * as HangCatalogExtra from "./react-native/HangCatalogExtra.ts";
import * as Json from "./react-native/Json.ts";
import * as MuxCatalogConfig from "./react-native/MuxCatalogConfig.ts";
import * as MuxCatalogHangExtra from "./react-native/MuxCatalogHangExtra.ts";
import * as MuxCatalogProducerExtra from "./react-native/MuxCatalogProducerExtra.ts";
import * as MuxError from "./react-native/MuxError.ts";
import * as MuxJsonConfig from "./react-native/MuxJsonConfig.ts";
import * as MuxJsonOptions from "./react-native/MuxJsonOptions.ts";
import * as MuxJsonSnapshotJson from "./react-native/MuxJsonSnapshotJson.ts";
import * as OriginConfig from "./react-native/OriginConfig.ts";
import * as OriginConsumer from "./react-native/OriginConsumer.ts";
import * as OriginDriver from "./react-native/OriginDriver.ts";
import * as OriginProducer from "./react-native/OriginProducer.ts";
import * as OriginRoute from "./react-native/OriginRoute.ts";
import * as OriginRouteOptions from "./react-native/OriginRouteOptions.ts";
import * as Path from "./react-native/Path.ts";
import * as PathRelative from "./react-native/PathRelative.ts";
import * as Pattern from "./react-native/Pattern.ts";
import * as Patterns from "./react-native/Patterns.ts";
import * as Timescale from "./react-native/Timescale.ts";
import * as Timestamp from "./react-native/Timestamp.ts";
import * as TokioClient from "./react-native/TokioClient.ts";
import * as TokioClientConfig from "./react-native/TokioClientConfig.ts";
import * as TokioConnectConfig from "./react-native/TokioConnectConfig.ts";
import * as TokioConnectError from "./react-native/TokioConnectError.ts";
import * as TokioConnection from "./react-native/TokioConnection.ts";
import * as TokioListenBind from "./react-native/TokioListenBind.ts";
import * as TokioListenConfig from "./react-native/TokioListenConfig.ts";
import * as TokioServer from "./react-native/TokioServer.ts";
import * as TokioServerConfig from "./react-native/TokioServerConfig.ts";
import * as TokioTlsConnect from "./react-native/TokioTlsConnect.ts";
import * as TokioTlsListen from "./react-native/TokioTlsListen.ts";
import * as TrackConsumer from "./react-native/TrackConsumer.ts";
import * as TrackInfo from "./react-native/TrackInfo.ts";
import * as TrackProducer from "./react-native/TrackProducer.ts";
import * as TrackRange from "./react-native/TrackRange.ts";
import * as TrackSubscriber from "./react-native/TrackSubscriber.ts";
import * as TrackSubscription from "./react-native/TrackSubscription.ts";
import * as Url from "./react-native/Url.ts";
import * as Versions from "./react-native/Versions.ts";
import * as Waiter from "./react-native/Waiter.ts";
import type * as rt from "./react-native/rt.ts";

// The host's globals, which the binding's types leave out.
declare const process: { exit(code: number): never; stderr: { write(text: string): boolean } };
declare function setTimeout(f: () => void, ms: number): number;
declare function clearTimeout(id: number): void;

let failures = 0;

function expect(ok: boolean, what: string): void {
	if (!ok) {
		process.stderr.write(`failed: ${what}\n`);
		failures++;
	}
}

/** The value of `r`, which must be `Ok`. */
function ok<T, E>(r: rt.Result<T, E>, what: string): T {
	if (!r.ok) throw new Error(`${what} failed`);
	return r.value;
}

/** The error of `r`, which must be `Err`. */
function err<T, E>(r: rt.Result<T, E>, what: string): E | undefined {
	expect(!r.ok, what);
	return r.ok ? undefined : r.error;
}

/** `v`, which must be some. */
function some<T>(v: T | undefined, what: string): T {
	if (v === undefined) throw new Error(`${what} returned none`);
	return v;
}

const text = (bytes: rt.Bytes | undefined): string | undefined =>
	bytes === undefined ? undefined : String.fromCharCode(...bytes.slice());

const ascii = (s: string): Uint8Array => Uint8Array.from(s, (c) => c.charCodeAt(0));

const same = (a: Uint8Array | undefined, b: Uint8Array): boolean =>
	a !== undefined && a.length === b.length && a.every((x, i) => x === b[i]);

/** Whether `p` has settled, once what is already posted has run. */
async function settled(p: Promise<unknown>): Promise<boolean> {
	let done = false;
	p.then(
		() => (done = true),
		() => (done = true),
	);
	await new Promise<void>((resolve) => setTimeout(resolve, 0));
	return done;
}

/** `p`, or a rejection naming `what` once `ms` pass first. */
function within<T>(p: Promise<T>, ms: number, what: string): Promise<T> {
	let timer = 0;
	const late = new Promise<never>((_, reject) => {
		timer = setTimeout(() => reject(new Error(`${what} took over ${ms} ms`)), ms);
	});
	return Promise.race([p, late]).finally(() => clearTimeout(timer));
}

/**
 * Poll `driver` from the event loop: on each wake, posted rather than run inside a call into
 * Rust, and at the deadline each poll returns, through a timer, with one waiter held across
 * polls, since what a poll registers lives only as long as its waiter. It resolves once the
 * driver finishes, with its error, having freed both.
 */
function drive(driver: OriginDriver.OriginDriver): Promise<Error_.Error_> {
	return new Promise((resolve) => {
		let deadline: number | undefined;
		let scheduled = false;
		let finished = false;
		const poll = (): void => {
			scheduled = false;
			if (deadline !== undefined) clearTimeout(deadline);
			deadline = undefined;
			// A wake posted before the driver finished may land after.
			if (finished) return;
			const next = OriginDriver.poll(driver, moq.clockNow(), waiter);
			if (!next.ok) {
				finished = true;
				waiter[Symbol.dispose]();
				driver[Symbol.dispose]();
				resolve(next.error);
				return;
			}
			if (next.value !== undefined) {
				const left = ClockInstant.saturatingDurationSince(next.value, moq.clockNow());
				// A timer waits at most 2^31 - 1 ms; a later deadline polls then and waits again.
				const ms = typeof left.secs === "number" ? left.secs * 1e3 + Math.ceil(left.nanos / 1e6) : Infinity;
				deadline = setTimeout(poll, Math.min(ms, 2 ** 31 - 1));
			}
		};
		// Wakes that land together poll once.
		const waiter = Waiter.new$(() => {
			if (scheduled) return;
			scheduled = true;
			setTimeout(poll, 0);
		});
		poll();
	});
}

async function local(): Promise<void> {
	using broadcast = BroadcastInfo.produce(BroadcastInfo.new$());
	using track = ok(BroadcastProducer.createTrack(broadcast, "video", undefined), "createTrack");
	expect(TrackProducer.name(track) === "video", "a track has its name");

	// A second track of the same name is an error, printed as Rust prints it.
	const duplicate = err(BroadcastProducer.createTrack(broadcast, "video", undefined), "a duplicate track is an error");
	expect(duplicate !== undefined && Error_.toString(duplicate) === "duplicate", "a duplicate track's error is Rust's");

	using consumer = TrackProducer.consume(track);
	using subscriber = ok(await TrackConsumer.subscribe(consumer, undefined), "subscribe");

	// A group's arrival wakes the subscriber's pending receive.
	const receiving = TrackSubscriber.recvGroup(subscriber);
	expect(!(await settled(receiving)), "a receive waits for a group");
	// The track is untimed, so its group's start is a bare key.
	using writer = ok(TrackProducer.appendGroup(track, Timestamp.unscaled(0)), "appendGroup");
	using first = some(ok(await receiving, "recvGroup"), "recvGroup");
	expect(GroupConsumer.index(first) === 0, "the first group is 0");

	// So does a frame's.
	const reading = GroupConsumer.readFrame(first);
	expect(!(await settled(reading)), "a read waits for a frame");
	ok(GroupProducer.writeFrame(writer, Timestamp.unscaled(0), ascii("hello")), "writeFrame");
	using hello = some(ok(await reading, "readFrame"), "readFrame");
	{
		using payload = FrameFrame.payloadGet(hello);
		expect(text(payload) === "hello", "a frame has its payload");
	}
	expect(Timestamp.scale(FrameFrame.timestampGet(hello)) === undefined, "an untimed frame's timestamp has no scale");
	// When it arrived: kio's clock's `Instant`, from kio's own export.
	expect(
		ClockInstant.checkedDurationSince(moq.clockNow(), FrameFrame.arrived(hello)) !== undefined,
		"a frame arrived before now",
	);

	// A timestamp is a `Copy` value copied across, which allocates nothing. The track is
	// untimed, so a timed frame is refused.
	const at = ok(Timestamp.fromMillis(1500), "fromMillis");
	expect(ok(Timestamp.asSecs(at), "asSecs") === 1, "1500 ms is 1 s");
	// Timestamps order in time across scales, and a scaled one and an unscaled key not at all.
	const second = ok(Timestamp.fromSecs(1), "fromSecs");
	const key = Timestamp.unscaled(0);
	expect(Timestamp.partialCmp(second, at) === -1 && Timestamp.partialCmp(at, second) === 1, "1 s is before 1500 ms");
	expect(Timestamp.partialCmp(key, at) === undefined, "an unscaled key is unordered against a time");
	const late = err(GroupProducer.writeFrame(writer, at, ascii("late")), "a timed frame on an untimed track is an error");
	expect(
		late !== undefined && Error_.toString(late) === "frame timestamp doesn't fit its track",
		"the late frame's error is Rust's",
	);
	expect(ok(Timestamp.asSecs(at), "asSecs") === 1, "a copied timestamp stays usable");

	// The last frame, bytes that are not UTF-8, then the group's end, which reads as none.
	const world = Uint8Array.of(0xff, ...ascii("world"));
	ok(GroupProducer.writeFrame(writer, Timestamp.unscaled(0), world), "writeFrame");
	ok(GroupProducer.finish(writer), "finish")[Symbol.dispose]();
	{
		using last = some(ok(await GroupConsumer.readFrame(first), "readFrame"), "readFrame");
		using payload = FrameFrame.payloadGet(last);
		expect(same(payload.slice(), world), "the last frame reads");
	}
	expect(ok(await GroupConsumer.readFrame(first), "readFrame") === undefined, "a finished group reads none");
}

async function origin(): Promise<void> {
	// An origin and the driver that runs it, a pair. Its root, which it lends, is returned
	// as a new handle to a clone.
	const [origin, driver] = OriginProducer.new$(OriginConfig.default$());
	{
		using root = OriginProducer.root(origin);
		expect(Path.asStr(root) === "", "an origin's root is empty");
	}

	// Scoped beneath a root to patterns, which moq-pattern's own export builds.
	{
		using patterns = Patterns.new$();
		using alice = ok(Pattern.subtree("alice"), "subtree");
		expect(Patterns.insert(patterns, alice), "a pattern inserts");
		using scoped = ok(OriginProducer.scope(origin, "room", patterns), "scope");
		using root = OriginProducer.root(scoped);
		expect(Path.asStr(root) === "room", "a scope has its root");
		using absolute = ok(OriginProducer.absolute(scoped, "alice"), "absolute");
		expect(Path.asStr(absolute) === "room/alice", "a scope's paths are absolute");
		using allowed = OriginProducer.allowed(scoped);
		expect(Patterns.matches(allowed, "alice/cam"), "a scope allows its patterns");
		expect(!Patterns.matches(allowed, "bob"), "a scope allows nothing else");
		using consumer = OriginProducer.consume(scoped);
		using bob = ok(OriginConsumer.absolute(consumer, "bob"), "absolute");
		expect(Path.asStr(bob) === "room/bob", "a scope's consumer shares its root");
	}

	// A broadcast the origin routes is stamped with its path, which its info lends. Each
	// handle to it is disposed, since an open one keeps it, and so the origin, alive.
	{
		using publication = ok(OriginProducer.route(origin, "room/alice", OriginRouteOptions.default$()), "route");
		using broadcast = OriginRoute.broadcast(publication);
		using info = BroadcastProducer.info(broadcast);
		using path = BroadcastInfo.pathGet(info);
		expect(Path.asStr(path) === "room/alice", "a broadcast has its path");

		// Paths own their segments, so JavaScript builds them, and relative ones, from a string.
		expect(err(Path.new$("/cam/"), "a path with an empty segment is an error") === "EmptySegment", "an empty segment is its own error");
		using cam = ok(Path.new$("cam"), "Path.new");
		expect(Path.asStr(cam) === "cam", "a path builds from its segments");
		using joined = ok(Path.join(path, cam), "join");
		expect(Path.asStr(joined) === "room/alice/cam", "paths join");
		{
			using stripped = some(Path.stripPrefix(joined, path), "stripPrefix");
			expect(Path.eq(stripped, cam), "a path strips its prefix");
		}
		expect(Path.stripPrefix(path, joined) === undefined, "a path strips no longer prefix");
		using relative = ok(PathRelative.new$("../bob"), "PathRelative.new");
		using resolved = ok(Path.resolve(joined, relative), "resolve");
		expect(Path.asStr(resolved) === "room/bob", "a relative path resolves");
	}

	// The driver, polled from the event loop until every owner of the origin is gone.
	let done = false;
	const driving = drive(driver).then((e) => {
		done = true;
		return e;
	});
	await settled(driving);
	// A client publishes the origin through moq-net's generic `with_publisher`, at each
	// type that implements its bound, by reference, so JavaScript keeps its handle.
	Client.withPublisherProducer(Client.new$(), origin)[Symbol.dispose]();
	expect(!done, "a driver runs while its origin has an owner");
	origin[Symbol.dispose]();
	// The last owner's disposal wakes the driver, which then finishes, cleanly.
	using error = await within(driving, 5000, "the driver");
	expect(Error_.toString(error) === "closed", "a finished driver's error is Rust's");
}

async function catalog(): Promise<void> {
	// moq-mux reads an OpusHead into hang's catalog config: 44.1 kHz in, stereo, which the
	// catalog lists at the decoder's 48 kHz, keeping the head as its description.
	const head = Uint8Array.of(...ascii("OpusHead"), 1, 2, 0x38, 0x01, 0x44, 0xac, 0, 0, 0, 0, 0);
	using audio = ok(moq.muxCodecOpusConfig(head), "muxCodecOpusConfig");
	expect(HangCatalogAudioConfig.sampleRateGet(audio) === 48000, "Opus decodes at 48 kHz");
	expect(HangCatalogAudioConfig.channelCountGet(audio) === 2, "the head is stereo");
	expect(HangCatalogAudioConfig.codecGet(audio).tag === "Opus", "the codec is Opus");
	{
		using description = HangCatalogAudioConfig.descriptionGet(audio);
		expect(same(description?.slice(), head), "the head is the description");
	}
	// A truncated head is moq-mux's error.
	const truncated = err(moq.muxCodecOpusConfig(head.subarray(0, 8)), "a truncated head is an error");
	expect(truncated !== undefined && MuxError.toString(truncated) !== "", "a truncated head's error has a message");
	truncated?.[Symbol.dispose]();

	// A subscription's ranges, from any iterable of them: an array.
	const times: TrackRange.TrackRange = { start: { tag: "Time", _0: 2 }, end: { tag: "Time", _0: 6 } };
	using subscription = TrackSubscription.withRanges(TrackSubscription.default$(), [TrackRange.live(), times]);
	const ranged = TrackSubscription.rangesGet(subscription);
	expect(
		ranged.length === 2 && TrackRange.eq(ranged[0]!, TrackRange.live()) && TrackRange.eq(ranged[1]!, times),
		"a subscription keeps its ranges",
	);
	expect(ranged[1]!.start.tag === "Time" && ranged[1]!.start._0 === 2, "a range starts at its first time");
	expect(ranged[1]!.end?.tag === "Time" && ranged[1]!.end._0 === 6, "a range ends at its last time, included");

	// A catalog at moq-mux's untyped extension, from the config JavaScript builds, whose
	// generic `is_claimed` is named by the last segment of its argument, and whose `mutate`
	// lends the catalog to JavaScript.
	using publisher = BroadcastInfo.produce(BroadcastInfo.new$());
	const extra = MuxCatalogConfig.withCatalogExtra(MuxCatalogConfig.default$(), HangCatalogExtra.default$());
	using catalog = ok(MuxCatalogProducerExtra.new$(publisher, extra), "MuxCatalogProducerExtra.new");
	expect(!MuxCatalogProducerExtra.isClaimedBinaryConfig(catalog, "state"), "an unclaimed name is not claimed");
	let read = 0;
	ok(
		MuxCatalogProducerExtra.mutate(catalog, (lent) => {
			if (ok(HangCatalogExtra.toJson(lent), "toJson").startsWith("{")) read++;
		}),
		"mutate",
	);
	expect(read === 1, "mutate lends the catalog once");

	// A JSON track the catalog lists, published through moq-mux's generic `json_snapshot` at
	// a JSON document, and a section JavaScript parses from text, set in the lent catalog and
	// read back as text.
	const statusTrack = ok(
		BroadcastProducer.createTrack(publisher, "status", TrackInfo.withTimescale(TrackInfo.default$(), Timescale.MILLI())),
		"createTrack",
	);
	using status = ok(
		MuxCatalogProducerExtra.jsonSnapshotConfig(catalog, statusTrack, MuxJsonConfig.default$(), MuxJsonOptions.default$()),
		"jsonSnapshotConfig",
	);
	expect(MuxJsonSnapshotJson.name(status) === "status", "a JSON snapshot has its track's name");
	expect(MuxCatalogProducerExtra.isClaimedJsonConfig(catalog, "status"), "the JSON snapshot claims its name");
	// A value JavaScript parses is published as the track's latest, which a subscriber reads
	// back as JSON text.
	{
		using on = ok(Json.parse(`{"on": true}`), "Json.parse");
		ok(MuxJsonSnapshotJson.update(status, on, ok(Timestamp.fromMillis(1), "fromMillis")), "update");
		using subscriber = MuxJsonSnapshotJson.consume(status);
		using group = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
		using frame = some(ok(await GroupConsumer.readFrame(group), "readFrame"), "readFrame");
		using payload = FrameFrame.payloadGet(frame);
		expect(text(payload) === `{"on":true}`, "a subscriber reads the latest value");
	}
	ok(
		MuxCatalogProducerExtra.mutate(catalog, (lent) => {
			using sections = HangCatalogExtra.extGet(lent);
			ok(MuxCatalogHangExtra.set(sections, "app", ok(Json.parse(`{"v": [1, true]}`), "Json.parse")), "set");
			HangCatalogExtra.extSet(lent, sections);
		}),
		"mutate",
	);
	let app: string | undefined;
	ok(
		MuxCatalogProducerExtra.mutate(catalog, (lent) => {
			expect(ok(HangCatalogExtra.toJson(lent), "toJson").includes(`"status"`), "the catalog lists the JSON track");
			using sections = HangCatalogExtra.extGet(lent);
			using section = MuxCatalogHangExtra.get(sections, "app");
			app = section === undefined ? undefined : Json.toText(section);
		}),
		"mutate",
	);
	expect(app === `{"v":[1,true]}`, "the catalog reads its section back as text");
	// Malformed text fails to parse, and a reserved section is moq-mux's error.
	expect(
		err(Json.parse(`{"v":`), "malformed JSON fails to parse") === "EOF while parsing a value at line 1 column 5",
		"a parse error is serde_json's",
	);
	{
		using sections = MuxCatalogHangExtra.default$();
		using reserved = err(MuxCatalogHangExtra.set(sections, "json", ok(Json.parse("1"), "Json.parse")), "a reserved section is an error");
		expect(reserved !== undefined && MuxError.toString(reserved) === "reserved catalog section: json", "a reserved section is moq-mux's error");
	}
}

function config(): void {
	// moq-tokio's terminal connect errors, which moq-net's `Error` shares no names with.
	expect(TokioConnectError.isAuth("Unauthorized"), "unauthorized is an auth error");
	expect(TokioConnectError.toString("Unauthorized") === "unauthorized", "a connect error prints as Rust's");

	// Session config's foreign types. A URL is parsed from text, failing with why.
	expect(err(Url.parse("relay.example"), "a relative URL is an error") === "relative URL without a base", "a URL's parse error is Rust's");
	// A session's config, its URL and pinned fingerprints set as Rust assigns them.
	using connect = TokioConnectConfig.default$();
	{
		using tls = TokioConnectConfig.tlsGet(connect);
		TokioTlsConnect.fingerprintSet(tls, ["ab:cd"]);
		TokioConnectConfig.tlsSet(connect, tls);
	}
	TokioConnectConfig.urlSet(connect, ok(Url.parse("https://relay.example:4443/demo"), "Url.parse"));
	{
		using tls = TokioConnectConfig.tlsGet(connect);
		const fingerprints = TokioTlsConnect.fingerprintGet(tls);
		expect(fingerprints.length === 1 && fingerprints[0] === "ab:cd", "a TLS config keeps its fingerprints");
	}
	using url = some(TokioConnectConfig.urlGet(connect), "urlGet");
	expect(Url.toString(url) === "https://relay.example:4443/demo", "a URL prints as Rust's");
	// A `Vec<String>`, as an array.
	expect(moq.ALPNS().length > 0, "a constant slice crosses as an array");
	using versions = Versions.default$();
	const alpns = Versions.alpns(versions);
	expect(alpns.length > 0 && alpns.every((alpn) => alpn.length > 0), "each version has an ALPN");
	// A socket address, crossing in a listener's bind.
	const loopback = new Uint8Array(16);
	loopback[15] = 1;
	const bind: TokioListenBind.TokioListenBind = {
		tag: "Addr",
		_0: { tag: "V6", _0: { ip: { octets: loopback }, port: 4443, flowinfo: 0, scopeId: 0 } },
	};
	expect(TokioListenBind.toString(bind) === "[::1]:4443", "an IPv6 bind prints as Rust's");
	const v4: TokioListenBind.TokioListenBind = {
		tag: "Addr",
		_0: { tag: "V4", _0: { ip: { octets: Uint8Array.of(127, 0, 0, 1) }, port: 0 } },
	};
	expect(TokioListenBind.toString(v4) === "127.0.0.1:0", "an IPv4 bind prints as Rust's");
	{
		using listen = TokioListenConfig.default$();
		TokioListenConfig.bindSet(listen, bind);
		const kept = TokioListenConfig.bindGet(listen);
		expect(kept !== undefined && TokioListenBind.eq(kept, bind), "a listener's config keeps its bind");
	}
	// TLS files, as OS paths, and fingerprints, as strings: none by default.
	{
		using tls = TokioTlsConnect.default$();
		expect(TokioTlsConnect.rootGet(tls).length === 0 && TokioTlsConnect.fingerprintGet(tls).length === 0, "a default TLS config lists none");
	}
}

async function session(): Promise<void> {
	// A session, as moq.c runs it: a server bound to port 0 reads back the port it got and
	// serves with one call, and a client dials it with one call and reads a frame back, each
	// promise settling once moq-tokio's fallback runtime wakes it from its threads.
	using served = moq.tokioOriginSpawn();
	using live = ok(OriginProducer.route(served, "live", OriginRouteOptions.default$()), "route");
	using liveBroadcast = OriginRoute.broadcast(live);
	using liveTrack = ok(BroadcastProducer.createTrack(liveBroadcast, "video", undefined), "createTrack");
	using liveGroup = ok(TrackProducer.appendGroup(liveTrack, Timestamp.unscaled(0)), "appendGroup");
	ok(GroupProducer.writeFrame(liveGroup, Timestamp.unscaled(0), ascii("hello")), "writeFrame");
	ok(GroupProducer.finish(liveGroup), "finish")[Symbol.dispose]();

	using serverConfig = TokioServerConfig.default$();
	{
		using listen = TokioServerConfig.listenGet(serverConfig);
		TokioListenConfig.bindSet(listen, {
			tag: "Addr",
			_0: { tag: "V6", _0: { ip: { octets: new Uint8Array(16) }, port: 0, flowinfo: 0, scopeId: 0 } },
		});
		using tls = TokioListenConfig.tlsGet(listen);
		TokioTlsListen.generateSet(tls, ["localhost"]);
		TokioListenConfig.tlsSet(listen, tls);
		TokioServerConfig.listenSet(serverConfig, listen);
	}
	using server = ok(TokioServerConfig.init(serverConfig), "init");
	const local = ok(TokioServer.localAddr(server), "localAddr");
	const port = local._0.port;
	expect(port !== 0, "a server bound to port 0 reads back the port it got");
	using servedConsumer = OriginProducer.consume(served);
	const serving = TokioServer.servePublish(server, servedConsumer);

	using connect = TokioConnectConfig.default$();
	TokioConnectConfig.urlSet(connect, ok(Url.parse(`moqt://localhost:${port}`), "Url.parse"));
	{
		using insecure = TokioConnectConfig.tlsGet(connect);
		TokioTlsConnect.insecureSet(insecure, true);
		TokioConnectConfig.tlsSet(connect, insecure);
	}
	using clientConfig = TokioClientConfig.default$();
	TokioClientConfig.connectSet(clientConfig, connect);
	using client = ok(TokioClientConfig.init(clientConfig), "init");
	using received = moq.tokioOriginSpawn();
	using remote = OriginProducer.consume(received);
	using connection = TokioClient.consume(TokioClient.withReconnect(TokioClient.clone(client), false), received);
	expect(connection !== undefined, "a client with a URL dials it");
	using established = ok(await TokioConnection.established(some(connection, "consume")), "established");

	using broadcast = ok(await OriginConsumer.request(remote, "live", undefined), "request");
	using track = ok(BroadcastConsumer.track(broadcast, "video"), "track");
	using subscriber = ok(await TrackConsumer.subscribe(track, undefined), "subscribe");
	using group = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
	using frame = some(ok(await GroupConsumer.readFrame(group), "readFrame"), "readFrame");
	{
		using payload = FrameFrame.payloadGet(frame);
		expect(text(payload) === "hello", "a session reads the frame the server serves");
	}

	ok(await TokioConnection.close(established), "close");
	await TokioClient.close(client);
	serving.cancel();
	await serving.catch(() => undefined);
}

async function main(): Promise<void> {
	await moq.load();
	await local();
	await origin();
	await catalog();
	config();
	// A stall fails with the timeout's error.
	await within(session(), 30000, "the session");
	if (failures !== 0) process.exit(1);
	console.log("ok");
}

await main();
