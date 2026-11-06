// moq.c's script through the generated Dart: a broadcast's track, published and subscribed
// to in one process, with every read awaited, and an origin, its driver run from the
// isolate's event loop, with moq-pattern, kio, moq-tokio, hang, and moq-mux exported
// into the same library, and the catalog lent to Dart for a call.
//
// A local read that must wait completes once the write that wakes it lands, on the event
// loop the script awaits. A session over QUIC runs on moq-tokio's fallback runtime, whose
// threads wake the isolate through its port.

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:moq/moq.dart' as moq;

var failures = 0;

void expect(bool ok, String what) {
  if (!ok) {
    stderr.writeln('failed: $what');
    failures++;
  }
}

/// `f` throws a `T`, which `check` accepts.
void throws<T extends Object>(void Function() f, bool Function(T) check, String what) {
  try {
    f();
  } on T catch (e) {
    expect(check(e), '$what: $e');
    return;
  }
  expect(false, what);
}

bool same(Uint8List? a, List<int> b) => a != null && a.length == b.length && a.indexed.every((e) => e.$2 == b[e.$1]);

/// Let the event loop run what is ready.
Future<void> step() => Future<void>.delayed(Duration.zero);

/// Poll `driver` from the isolate's event loop: on each wake, and at the deadline each poll
/// returns, through a [Timer], with one waiter held across polls, since what a poll
/// registers lives only as long as its waiter. It completes once the driver finishes, with
/// its error.
Future<moq.Error> drive(moq.OriginDriver driver) {
  final finished = Completer<moq.Error>();
  Timer? deadline;
  var scheduled = false;
  late final moq.Waiter waiter;
  void poll() {
    scheduled = false;
    deadline?.cancel();
    // A wake posted before the driver finished may land after.
    if (finished.isCompleted) return;
    try {
      final next = driver.poll(moq.clockNow(), waiter);
      if (next != null) {
        deadline = Timer(next.saturatingDurationSince(moq.clockNow()), poll);
        next.close();
      }
    } on moq.Error catch (e) {
      waiter.close();
      driver.close();
      finished.complete(e);
    }
  }

  // Each wake reaches the event loop as a message, never inside a call into Rust; wakes that
  // land together poll once.
  waiter = moq.Waiter(() {
    if (!scheduled) {
      scheduled = true;
      scheduleMicrotask(poll);
    }
  });
  poll();
  return finished.future;
}

Future<void> local() async {
  final broadcast = moq.BroadcastInfo().produce();
  final track = broadcast.createTrack('video', null);
  expect(track.name() == 'video', 'a track has its name');

  // A second track of the same name is an error, printed as Rust prints it.
  throws<moq.Error>(
    () => broadcast.createTrack('video', null),
    (e) => '$e' == 'duplicate',
    "a duplicate track's error is Rust's",
  );

  final subscriber = await track.consume().subscribe(null);

  // A group's arrival wakes the subscriber's pending receive.
  final recv = subscriber.recvGroup();
  var received = false;
  recv.then((_) => received = true).ignore();
  await step();
  expect(!received, 'a receive waits for a group');
  final writer = track.appendGroup();
  expect(!received, 'a receive never completes inside the wake');
  final first = (await recv.timeout(const Duration(seconds: 5)))!;
  expect(first.index() == 0, 'the first group is 0');

  // So does a frame's.
  final read = first.readFrame();
  var ready = false;
  read.then((_) => ready = true).ignore();
  await step();
  expect(!ready, 'a read waits for a frame');
  writer.writeFrame(null, utf8.encode('hello'));
  final hello = (await read.timeout(const Duration(seconds: 5)))!;
  expect(utf8.decode(hello.payloadGet) == 'hello', 'a frame has its payload');
  expect(hello.timestampGet == null, 'an untimed frame has no timestamp');
  // When it arrived: kio's clock's `Instant`, from kio's own export.
  expect(moq.clockNow().checkedDurationSince(hello.arrived()) != null, 'a frame arrived before now');

  // A timestamp is a `Copy` value behind a handle: passing it copies it out. The track is
  // untimed, so a timestamped frame is refused.
  final at = moq.Timestamp.fromMillis(1500);
  expect(at.asSecs() == 1, '1500 ms is 1 s');
  throws<moq.Error>(
    () => writer.writeFrame(at, utf8.encode('late')),
    (e) => '$e' == "frame timestamp doesn't fit its track",
    "the late frame's error is Rust's",
  );
  expect(at.asSecs() == 1, 'a copied timestamp stays usable');

  // The last frame, bytes that are not UTF-8, then the group's end, which reads as null.
  final world = [0xff, ...utf8.encode('world')];
  writer.writeFrame(null, Uint8List.fromList(world));
  writer.finish();
  final last = await first.readFrame();
  expect(same(last?.payloadGet, world), 'the last frame reads');
  expect(await first.readFrame() == null, 'a finished group reads null');
}

Future<void> origin() async {
  // An origin and the driver that runs it, a pair. Its root, which it lends, is returned
  // as a new object of a clone.
  final (origin, driver) = moq.OriginProducer.new_(moq.OriginConfig.default_());
  expect(origin.root().asStr() == '', "an origin's root is empty");

  // Scoped beneath a root to patterns, which moq-pattern's own export builds. Each object
  // of the origin is closed, since an open one keeps it alive.
  final patterns = moq.Patterns();
  expect(patterns.insert(moq.Pattern.subtree('alice')), 'a pattern inserts');
  final scoped = origin.scope('room', patterns);
  expect(scoped.root().asStr() == 'room', 'a scope has its root');
  expect(scoped.absolute('alice').asStr() == 'room/alice', "a scope's paths are absolute");
  final allowed = scoped.allowed();
  expect(allowed.matches('alice/cam'), 'a scope allows its patterns');
  expect(!allowed.matches('bob'), 'a scope allows nothing else');
  final consumer = scoped.consume();
  expect(consumer.absolute('bob').asStr() == 'room/bob', "a scope's consumer shares its root");
  consumer.close();
  scoped.close();
  patterns.close();

  // A broadcast the origin creates is stamped with its path, which its info lends.
  final publication = origin.createBroadcast('room/alice');
  final created = publication.broadcast();
  final path = created.info().pathGet;
  created.close();
  publication.close();
  expect(path.asStr() == 'room/alice', 'a broadcast has its path');

  // Paths own their segments, so Dart builds them, and relative ones, from a string.
  throws<moq.PathError>(
    () => moq.Path('/cam/'),
    (e) => e == moq.PathError.emptySegment,
    'an empty segment is its own error',
  );
  final cam = moq.Path('cam');
  expect(cam.asStr() == 'cam', 'a path builds from its segments');
  final joined = path.join(cam);
  expect(joined.asStr() == 'room/alice/cam', 'paths join');
  expect(joined.stripPrefix(path) == cam, 'a path strips its prefix');
  expect(path.stripPrefix(joined) == null, 'a path strips no longer prefix');
  expect(joined.resolve(moq.PathRelative('../bob')).asStr() == 'room/bob', 'a relative path resolves');

  // The driver, polled from the event loop until every owner of the origin is gone.
  final driving = drive(driver);
  var done = false;
  driving.then((_) => done = true).ignore();
  await step();
  // A client publishes the origin through moq-net's generic `with_publisher`, at each
  // type that implements its bound, by reference, so Dart keeps its object.
  moq.Client().withPublisherProducer(origin).close();
  expect(!done, 'a driver runs while its origin has an owner');
  origin.close();
  // The last owner's close wakes the driver, which then finishes, cleanly.
  final error = await driving.timeout(const Duration(seconds: 5));
  expect('$error' == 'closed', "a finished driver's error is Rust's");
}

Future<void> catalog() async {
  // moq-mux reads an OpusHead into hang's catalog config: 44.1 kHz in, stereo, which the
  // catalog lists at the decoder's 48 kHz, keeping the head as its description.
  final head = Uint8List.fromList([...utf8.encode('OpusHead'), 1, 2, 0x38, 0x01, 0x44, 0xac, 0, 0, 0, 0, 0]);
  final audio = moq.muxCodecOpusConfig(head);
  expect(audio.sampleRateGet == 48000, 'Opus decodes at 48 kHz');
  expect(audio.channelCountGet == 2, 'the head is stereo');
  expect(audio.codecGet == const moq.HangCatalogAudioCodecOpus(), 'the codec is Opus');
  expect(same(audio.descriptionGet, head), 'the head is the description');
  // A truncated head is moq-mux's error.
  throws<moq.MuxError>(
    () => moq.muxCodecOpusConfig(Uint8List.sublistView(head, 0, 8)),
    (e) => '$e'.isNotEmpty,
    "a truncated head's error has a message",
  );

  // A subscription's ranges, from any iterable of them: a list.
  final groups = moq.TrackRange.groups(const moq.BoundsU64(moq.BoundU64Included(2), moq.BoundU64Excluded(6)));
  final ranges = [moq.TrackRange.live(), groups];
  final ranged = moq.TrackSubscription.default_().withRanges(ranges).rangesGet;
  expect(ranged.length == 2 && ranged[0] == ranges[0] && ranged[1] == ranges[1], 'a subscription keeps its ranges');
  expect(groups.start == const moq.TrackAnchorAbsolute(2), 'groups start at their first');
  expect(groups.end == const moq.TrackAnchorAbsolute(6), 'groups end before their end');

  // A catalog at moq-mux's untyped extension, from the config Dart builds, whose generic
  // `is_claimed` is named by the last segment of its argument, and whose `mutate` lends
  // the catalog to Dart.
  final publisher = moq.BroadcastInfo().produce();
  final extra = moq.MuxCatalogConfig.default_().withCatalogExtra(moq.HangCatalogExtra.default_());
  final producer = moq.MuxCatalogProducerExtra(publisher, extra);
  expect(!producer.isClaimedBinaryConfig('state'), 'an unclaimed name is not claimed');
  var read = 0;
  producer.mutate((lent) {
    if (lent.toJson().startsWith('{')) read++;
  });
  expect(read == 1, 'mutate lends the catalog once');

  // A JSON track the catalog lists, published through moq-mux's generic `json_snapshot` at
  // a JSON document, and a section Dart parses from text, set in the lent catalog and read
  // back as text.
  final status = producer.jsonSnapshotConfig(
    publisher.createTrack('status', moq.TrackInfo.default_().withTimescale(moq.Timescale.MILLI)),
    moq.MuxJsonConfig.default_(),
    moq.MuxJsonOptions.default_(),
  );
  expect(status.name() == 'status', "a JSON snapshot has its track's name");
  expect(producer.isClaimedJsonConfig('status'), 'the JSON snapshot claims its name');
  // A value Dart parses is published as the track's latest, which a subscriber reads back
  // as JSON text.
  final on = moq.Json.parse('{"on": true}');
  status.update(on, moq.Timestamp.now());
  on.close();
  final latest = await (await status.consume().recvGroup())?.readFrame();
  expect(latest != null && utf8.decode(latest.payloadGet) == '{"on":true}', 'a subscriber reads the latest value');
  producer.mutate((lent) {
    final sections = lent.extGet;
    sections.set('app', moq.Json.parse('{"v": [1, true]}'));
    lent.extSet(sections);
  });
  String? app;
  producer.mutate((lent) {
    expect(lent.toJson().contains('"status"'), 'the catalog lists the JSON track');
    app = lent.extGet.get('app')?.toText();
  });
  expect(app == '{"v":[1,true]}', 'the catalog reads its section back as text');
  // Malformed text fails to parse, and a reserved section is moq-mux's error.
  throws<moq.StringException>(
    () => moq.Json.parse('{"v":'),
    (e) => e.error == 'EOF while parsing a value at line 1 column 5',
    "a parse error is serde_json's",
  );
  throws<moq.MuxError>(
    () => moq.MuxCatalogHangExtra.default_().set('json', moq.Json.parse('1')),
    (e) => '$e' == 'reserved catalog section: json',
    "a reserved section is moq-mux's error",
  );
}

void config() {
  // moq-tokio's terminal connect errors, which moq-net's `Error` shares no names with.
  expect(moq.TokioConnectError.unauthorized.isAuth(), 'unauthorized is an auth error');
  expect('${moq.TokioConnectError.unauthorized}' == 'unauthorized', "a connect error prints as Rust's");

  // Session config's foreign types. A URL is parsed from text, failing with why.
  throws<moq.StringException>(
    () => moq.Url.parse('relay.example'),
    (e) => e.error == 'relative URL without a base',
    "a URL's parse error is Rust's",
  );
  // A session's config, its URL and pinned fingerprints set as Rust assigns them.
  final connect = moq.TokioConnectConfig.default_();
  final tls = connect.tlsGet;
  tls.fingerprintSet(['ab:cd']);
  connect.tlsSet(tls);
  connect.urlSet(moq.Url.parse('https://relay.example:4443/demo'));
  final kept = connect.tlsGet;
  expect(kept.fingerprintGet.length == 1 && kept.fingerprintGet[0] == 'ab:cd', 'a TLS config keeps its fingerprints');
  kept.close();
  final url = connect.urlGet!;
  connect.close();
  final versions = moq.Versions.default_();
  final tokens = moq.AuthTokens([]);
  final dial = tokens.dial(url, versions);
  final dialed = dial.urlGet;
  expect(dialed == url, 'a dial keeps its URL');
  expect('$dialed' == 'https://relay.example:4443/demo', "a URL prints as Rust's");
  dialed.close();
  dial.close();
  tokens.close();
  url.close();
  // A `Vec<String>`, as a list.
  expect(moq.ALPNS.isNotEmpty, 'a constant slice crosses as a list');
  final alpns = versions.alpns();
  expect(alpns.isNotEmpty && alpns.every((a) => a.isNotEmpty), 'each version has an ALPN');
  versions.close();
  // A socket address, crossing in a listener's bind.
  final bind = moq.TokioListenBindAddr(moq.SocketAddr(InternetAddress('::1'), 4443));
  expect('$bind' == '[::1]:4443', "an IPv6 bind prints as Rust's");
  final v4 = moq.SocketAddr(InternetAddress('127.0.0.1'), 0);
  expect('${moq.TokioListenBindAddr(v4)}' == '127.0.0.1:0', "an IPv4 bind prints as Rust's");
  final listen = moq.TokioListenConfig.default_();
  listen.bindSet(bind);
  // The preferred address of each family, which takes only that family.
  listen.preferredV4Set(v4);
  expect(listen.preferredV4Get == v4 && listen.preferredV6Get == null, 'a listener keeps its preferred IPv4 address');
  // A variant without Rust's `PartialEq` compares by identity, so by its field.
  final listening = listen.bindGet;
  expect(listening is moq.TokioListenBindAddr && listening.$0 == bind.$0, "a listener's config keeps its bind");
  listen.close();
  // TLS files, as OS paths, and fingerprints, as strings: none by default.
  final defaults = moq.TokioTlsConnect.default_();
  expect(defaults.rootGet.isEmpty && defaults.fingerprintGet.isEmpty, 'a default TLS config lists none');
  defaults.close();
}

/// A session, as moq.c runs it: a server bound to port 0 reads back the port it got and
/// serves with one call, its future left pending, and a client dials it with one call and
/// reads a frame back, each await completing once moq-tokio's fallback runtime wakes it
/// from its threads.
Future<void> session() async {
  final served = moq.tokioOriginSpawn();
  final live = served.createBroadcast('live');
  live.announce(moq.OriginRoute.default_());
  final liveTrack = live.broadcast().createTrack('video', null);
  final liveGroup = liveTrack.appendGroup();
  liveGroup.writeFrame(null, utf8.encode('hello'));
  liveGroup.finish();

  final serverConfig = moq.TokioServerConfig.default_();
  final listen = serverConfig.listenGet;
  listen.bindSet(moq.TokioListenBindAddr(moq.SocketAddr(InternetAddress('::'), 0)));
  final tls = listen.tlsGet;
  tls.generateSet(['localhost']);
  listen.tlsSet(tls);
  serverConfig.listenSet(listen);
  final server = serverConfig.init();
  final local = server.localAddr();
  expect(local.port != 0, 'a server bound to port 0 reads back the port it got');
  final serving = server.servePublish(served.consume());

  final connect = moq.TokioConnectConfig.default_();
  connect.urlSet(moq.Url.parse('moqt://localhost:${local.port}'));
  final insecure = connect.tlsGet;
  insecure.insecureSet(true);
  connect.tlsSet(insecure);
  final clientConfig = moq.TokioClientConfig.default_();
  clientConfig.connectSet(connect);
  final client = clientConfig.init();
  final received = moq.tokioOriginSpawn();
  final remote = received.consume();
  final connection = client.clone().withReconnect(false).consume(received);
  expect(connection != null, 'a client with a URL dials it');
  final established = await connection!.established();

  final broadcast = await remote.request('live', null);
  final subscriber = await broadcast.track('video').subscribe(null);
  final group = await subscriber.recvGroup();
  final frame = await group?.readFrame();
  expect(frame != null && utf8.decode(frame.payloadGet) == 'hello', 'a session reads the frame the server serves');

  await established.close_();
  await client.close_();
  serving.cancel();
}

Future<void> main() async {
  await local();
  await origin();
  await catalog();
  config();
  // A stall fails with the timeout's exception.
  await session().timeout(const Duration(seconds: 30));
  if (failures == 0) print('ok');
  exitCode = failures == 0 ? 0 : 1;
}
