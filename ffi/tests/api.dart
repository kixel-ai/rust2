// The api fixture's Dart test: the script fixtures/api/examples/transcript.rs runs
// natively, through the generated Dart. tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks the futures' semantics, what closing and the garbage
// collector free, what a closed object refuses, how returned bytes cross, or breaks a
// contract, which must fail.

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:api/api.dart' as api;

/// The Log callback: keeps even levels.
final class PrintLog extends api.Log {
  @override
  bool log(int level, String line) {
    print('log $level $line');
    return level % 2 == 0;
  }

  // Rust closes it once it drops the implementation.
  @override
  void close() => print('freed');
}

String bound(api.BoundU64 b) => switch (b) {
  api.BoundU64Included(:final value) => 'Included($value)',
  api.BoundU64Excluded(:final value) => 'Excluded($value)',
  api.BoundU64Unbounded() => 'Unbounded',
};

Uint8List bytes(List<int> values) => Uint8List.fromList(values);

String text(Uint8List data) => utf8.decode(data);

String quoted(List<String> values) => '[${values.map((v) => '"$v"').join(', ')}]';

/// A path's UTF-8 bytes as Rust's `escape_ascii` prints them.
String escaped(String path) {
  final out = StringBuffer();
  for (final b in utf8.encode(path)) {
    switch (b) {
      case 0x09:
        out.write(r'\t');
      case 0x0d:
        out.write(r'\r');
      case 0x0a:
        out.write(r'\n');
      case 0x5c || 0x27 || 0x22:
        out.write('\\${String.fromCharCode(b)}');
      case >= 0x20 && < 0x7f:
        out.write(String.fromCharCode(b));
      default:
        out.write('\\x${b.toRadixString(16).padLeft(2, '0')}');
    }
  }
  return out.toString();
}

void check(bool ok, String what) {
  if (!ok) {
    stderr.writeln('failed: $what');
    exit(1);
  }
}

/// `f` throws a `T`.
void throws<T extends Object>(void Function() f, String what) {
  try {
    f();
  } on T {
    return;
  }
  check(false, what);
}

/// `f` throws the StateError of a closed `type`.
void closed(void Function() f, String type, String what) {
  try {
    f();
  } on StateError catch (e) {
    check(e.message == '$type is closed', '$what: $e');
    return;
  }
  check(false, what);
}

/// Let the event loop run what is ready.
Future<void> step() => Future<void>.delayed(Duration.zero);

/// Run the event loop until `done()`, failing after a deadline.
Future<void> until(bool Function() done, String what) async {
  final deadline = DateTime.now().add(const Duration(seconds: 5));
  while (!done()) {
    check(DateTime.now().isBefore(deadline), what);
    await step();
  }
}

Future<void> script() async {
  print('limit ${api.LIMIT}');

  final p = const api.Point(x: 1, y: 2).offset(2, 3);
  print('offset ${p.x} ${p.y}');
  print('debug ${p.debug()}');
  print('eq ${p == const api.Point(x: 3, y: 5)}');
  final d = api.Point.default_();
  print('default ${d.x} ${d.y}');

  final c = api.Counter('c', 5);
  print('get ${c.get()}');
  print('name ${c.name()}');
  print('add ok ${c.add(10)}');
  try {
    c.add(2000);
  } on api.Error catch (e) {
    print('add err $e ${e.debug()}');
  }
  try {
    c.sub(20);
  } on api.Error catch (e) {
    print('sub err $e ${e.debug()}');
  }
  c.rename('d');
  print('name ${c.name()}');
  print('is d ${c.is_('d')}');
  final copy = c.clone();
  copy.add(1);
  print('clone ${copy.get()} original ${c.get()}');
  copy.close();
  print('debug ${c.debug()}');
  final label = c.intoLabel(p);
  print('label ${label.debug()}');
  print('note ${api.hasText(const api.Note(text: ''))} ${api.hasText(const api.Note(text: null))}');

  final (below, above) = api.Counter('s', 7).split(5);
  print('split ${below.name()} ${below.get()} ${above.name()} ${above.get()}');
  final (only,) = api.only(api.Counter('o', 3));
  print('only ${only.name()} ${only.get()}');
  final (w8, w16, w32, w64) = api.widths(98305);
  print('widths $w8 $w16 $w32 $w64');
  for (final b in [2, 0]) {
    try {
      final (q, r) = api.divide(7, b);
      print('divide $q $r');
    } on api.Error catch (e) {
      print('divide err $e');
    }
  }
  for (final n in [98305, 0]) {
    final spread = api.spread(n);
    print(spread == null ? 'spread none' : 'spread ${spread.$1} ${spread.$2} ${spread.$3} ${spread.$4}');
  }
  for (final needle in ['lo', 'x']) {
    final found = api.find(utf8.encode('hello'), utf8.encode(needle));
    print(found == null ? 'find none' : 'find ${found.$1} ${found.$2}');
  }
  final statRows = api.rows([const api.Point(x: 1, y: 2), const api.Point(x: 3, y: 3)]);
  print('rows ${statRows.map((r) => '${r.$1} ${r.$2.debug()} ${r.$3.x} ${r.$3.y}').join(', ')}');
  // Each handle in a tuple moves in, as one taken by value does.
  final board = api.Board([('low', api.Counter('l', 1)), ('high', api.Counter('h', 9))]);
  final ranked = board.ranked();
  print('ranked ${ranked.map((r) => '${r.$1} ${r.$2.name()} ${r.$2.get()}').join(', ')}');
  board.close();
  // A map crosses as its pairs and a set as its items, which Rust collects in order, each
  // key once, the last kept.
  final atlas = const api.Atlas(
    points: [('b', api.Point(x: 1, y: 1)), ('ab', api.Point(x: 2, y: 3)), ('b', api.Point(x: 4, y: 4))],
    ids: [3, 1, 3],
  ).with_('c', const api.Point(x: 5, y: 6));
  final atlasPoints = atlas.points.map((p) => '${p.$1} ${p.$2.x} ${p.$2.y}').join(', ');
  print('atlas $atlasPoints; ids ${atlas.ids.join(' ')}');
  // Another crate's error crosses as its message.
  for (final text in ['80', 'x']) {
    try {
      print('port ${api.port(text)}');
    } on api.StringException catch (e) {
      print('port err ${e.error}');
    }
  }
  const span = api.Span(bounds: (2, 10), named: ('s', api.Shape.square));
  final (lowerHalf, upperHalf) = api.halves(span);
  print('halves ${lowerHalf.$1} ${lowerHalf.$2} ${upperHalf.$1} ${upperHalf.$2} ${span.debug()}');
  final (swappedN, swappedText) = api.swapped(('t', 4));
  print('swapped $swappedN $swappedText');
  for (final value in <(int?,)?>[null, (null,), (3,)]) {
    final doubled = api.doubled(value);
    if (doubled == null) {
      print('doubled None');
    } else {
      print('doubled Some((${doubled.$1 == null ? 'None' : 'Some(${doubled.$1})'},))');
    }
  }
  // A borrowed or boxed tuple of one is a tuple, not its element.
  final (lentOne,) = api.lentBack((bytes([1, 2]),));
  final (boxedOne,) = api.boxedTuple(bytes([3]));
  print('lent back $lentOne boxed $boxedOne');
  // A borrowed string and an owned one share a tuple struct.
  print(
    'title Some(("${api.title(true)!.$1}",)) ${api.title(false) == null ? 'None' : 'Some'} '
    'named Some(("${api.named('x')!.$1}",))',
  );

  // Each lent counter is a new handle to a clone.
  final registry = api.Registry(api.Counter('first', 1));
  print('last ${registry.last() != null}');
  registry.push(api.Counter('last', 2));
  final first = registry.firstGet;
  first.add(10);
  print('first ${registry.firstGet.get()} lent ${first.get()}');
  final last = registry.last()!;
  last.add(10);
  print('last ${registry.last()!.get()} lent ${last.get()}');
  // A closure lent for the call edits the first counter through a handle.
  const by = 5;
  final edited = registry.edit((counter) {
    counter.add(by);
    return counter.edited() && counter.get() == 6;
  });
  print('edit $edited ${registry.firstGet.get()}');
  // A setter moves an object in, as assignment does, closing it.
  registry.firstSet(api.Counter('second', 3));
  print('set first ${registry.firstGet.get()} ${registry.firstGet.name()}');
  print('walk ${api.walk(3, (cursor) => cursor.at() * 2)}');
  print('each ${api.each(2, (at) => print('visit $at'))}');
  print('offset ${api.offset(() => 7, 3)}');
  print('shift ${api.shift(() => 7, 4)}');

  final name = api.Name.owned('a');
  final joined = name.join(api.Name.owned('b'));
  print('joined ${joined.asStr()} ${joined.debug()} ${joined == name}');
  joined.rename('c');
  print('renamed ${joined.asStr()}');
  final long = api.longer(joined, api.Name.owned('dd'));
  print('longer ${long.asStr()}');
  print('unnamed [${api.unnamed().asStr()}]');

  print('total ${api.total(const api.RangeU64(2, 5))}');
  final wide = api.widen(const api.RangeU64(2, 5), 1);
  print('widen ${wide.start} ${wide.end}');
  final counts = [
    const api.BoundsU64(api.BoundU64Included(2), api.BoundU64Excluded(5)),
    const api.BoundsU64(api.BoundU64Included(2), api.BoundU64Included(5)),
    const api.BoundsU64(api.BoundU64Included(3), api.BoundU64Unbounded()),
    const api.BoundsU64(api.BoundU64Unbounded(), api.BoundU64Unbounded()),
    const api.BoundsU64(api.BoundU64Excluded(2), api.BoundU64Included(4)),
  ].map(api.count);
  print('count ${counts.join(' ')}');
  print('end ${bound(api.end(3))} ${bound(api.end(0))}');
  final (endsStart, endsEnd) = api.ends(3);
  print('ends ${bound(endsStart)} ${bound(endsEnd)}');
  for (final n in [2, 5]) {
    try {
      print('below ${api.below(n, 3)}');
    } on api.BoundsU64Exception catch (e) {
      print('below err ${bound(e.error.start)} ${bound(e.error.end)}');
    }
  }
  final window = api.Window(range: const api.RangeU64(2, 8), end: api.end(5));
  print('window ${window.contains(4)} ${window.contains(6)} ${window.debug()}');

  print(api.greet('x', api.Shape.square));
  print('len ${api.len(utf8.encode('abc'))}');
  final binary = bytes([0xff, 0, 0x61]);
  print('hex ${api.hex(binary)}');
  print('timed hex ${api.timedHex(binary)}');
  print('sample hex ${api.sampleHex(binary)}');
  print('queued hex ${api.queuedHex(binary)}');
  print('tick ${api.tickName(const api.Label(name: 'tick', at: null, ttl: null))}');
  print('rooted ${api.rootedName(const api.Label(name: 'rooted', at: null, ttl: null))}');
  print('stored ${api.storedText('stored')}');
  print('layer ${api.layerName(const api.Label(name: 'layer', at: null, ttl: null))}');
  for (final event in [api.EventMoved(p), const api.EventNamed(name: 'n'), const api.EventDone()]) {
    print(api.describe(event));
  }

  // A `Copy` handle passed by value is copied, so it stays usable.
  final stamp = api.Stamp.fromMillis(2500);
  print('secs ${stamp.secs()}');
  print('later ${api.Stamp.fromMillis(3000).later(stamp)}');
  print('secs ${stamp.secs()}');
  final message = api.Message(stamp, utf8.encode('body'));
  print('payload ${text(message.payloadGet)}');
  print('stamp ${message.stampGet!.debug()}');
  print('sent ${message.sent()}');
  // Setters assign in place: the bytes are copied in, and a `Copy` handle copied.
  final reset = api.Stamp.fromMillis(4000);
  message.stampSet(reset);
  message.payloadSet(utf8.encode('edited'));
  print('payload ${text(message.payloadGet)}');
  print('stamp ${message.stampGet!.debug()}');
  print('secs ${reset.secs()}');
  message.stampSet(null);
  print('stamp ${message.stampGet != null}');

  print('kept ${api.emit(PrintLog(), bytes([1, 2, 3, 4]))}');
  final logger = api.Logger(PrintLog());
  print('write ${logger.write('hi')}');
  logger.close();
  print('dropped');
  // Rust calls it on another thread, before the call returns.
  const threshold = 2;
  final filtered = api.filter(bytes([1, 2, 3, 4]), (level, line) {
    print('filter $level $line');
    return level > threshold;
  });
  print('filtered $filtered');

  final channel = api.Channel();
  var wakes = 0;
  // Polled at the call, it pends, and the wake the send posts resumes it from the event loop.
  Uint8List? got;
  final recv = channel.recv()..then((payload) => got = payload);
  await step();
  print('recv pending ${got == null}');
  channel.send(utf8.encode('hi'));
  check(got == null, 'recv never resumes inside the wake');
  await recv;
  wakes++;
  print('woken $wakes');
  print('recv ${text(got!)}');
  channel.send(Uint8List(0));
  try {
    print('nonempty ${(await channel.recvNonempty()).length}');
  } on api.Error catch (e) {
    print('nonempty err $e');
  }
  // The send happens at the call: the future is dropped unawaited.
  channel.sendLater(utf8.encode('eager'));
  print('recv ${text(await channel.recv())}');
  channel.send(utf8.encode('sized'));
  final (payload, size) = await channel.recvSized();
  print('sized ${text(payload)} $size');
  // The waker moves into Rust, which keeps it until the send wakes it.
  channel.notify(() => wakes++);
  channel.send(utf8.encode('rung'));
  await until(() => wakes == 2, 'the send wakes the waker Rust keeps');
  print('woken $wakes');

  final nodes = api.Node(1, null);
  nodes.push(2);
  print('node ${nodes.sum()}');

  final tally = api.Tally();
  tally.bump();
  print('tally ${tally.bump()} ${tally == tally}');
  print('both ${api.bumpBoth(tally, tally)}');
  print('peek ${api.peek(tally, (lent) => lent.bump())}');

  final area = api.grow(api.Area(const api.Size(width: 2, height: 3)), 1);
  final areaSize = area.size();
  print('area ${areaSize.width} ${areaSize.height} ${area.pixels()}');

  // Generic types, each at the instances it infers, named by their arguments.
  const pair = api.PairU32(left: 2, right: 5);
  final swapped = pair.swap();
  print('pair ${swapped.left} ${swapped.right} sum ${pair.sum()} ${pair.debug()} ${pair == swapped}');
  print('even ${pair.even()} ${const api.PairU32(left: 7, right: 7).even()}');
  // An instance renamed alone, beside another of `Pair` that keeps its name.
  final sized = api.tag(const api.Size(width: 1, height: 2), 3);
  final flipped = sized.swap();
  print('sized ${flipped.left} ${flipped.right.width} ${flipped.debug()}');
  final beside = const api.PairU32(left: 258, right: 9).besideSize(sized.left);
  print('beside ${beside.left} ${beside.debug()}');
  // At its default, named as the type itself.
  final stack = api.Stack();
  for (final (i, n) in ['a', 'b'].indexed) {
    stack.push(api.Counter(n, i + 1));
  }
  final top = stack.pop()!.get();
  print('stack ${stack.depth()} Some($top) ${stack.pop() != null} ${stack.pop() == null}');
  // Generic functions and methods, at each implementation of the trait bounding them, by
  // reference, which leaves Dart its object.
  final fromPoint = api.Counter.seededPoint('p', const api.Point(x: -3, y: 4));
  final fromSize = api.Counter.seededSize('s', const api.Size(width: 5, height: 6));
  final fromCounter = api.Counter.seededCounter('c', fromPoint);
  final fromPolar = api.Counter.seededPolarPoint('q', const api.PolarPoint(turns: 1, distance: 2));
  print(
    'seeded ${fromPoint.get()} ${fromSize.get()} ${fromCounter.get()} ${fromPolar.get()} '
    'past ${stack.depthPastPoint(const api.Point(x: 1, y: 2))} '
    'seeds ${api.seedsPointSize(const api.Size(width: 2, height: 3))}',
  );

  // A trait implemented for `Point` and `&Point` apart: one instance.
  print('weigh ${api.weighPoint(const api.Point(x: -1, y: 2))}');
  // A function named as the type it returns, and one named as a type it does not.
  print('polar ${api.polarPoint(2)!.distance} ${api.polarPoint(0) == null} '
      'area ${api.area(const api.Size(width: 2, height: 3))}');
  // Two types whose paths join alike, the second kept apart where its path splits.
  print('wind ${api.wind(const api.PolarTurn(whole: 2), const api.Polar_Turn(part: 3))}');
  // Instances that name each other, and one made only by an impl of it.
  final cross = api.cross(
    const api.LeftRightU8(value: api.RightU8(value: 1)),
    const api.RightLeftU8(value: api.LeftU8(value: 2)),
  );
  print('cross $cross outer ${api.OuterInnerU8(4).get()}');
  // A bound that outlives the call takes a value, and an instance only a constant names.
  print('kept ${api.keptPoint(const api.Point(x: 1, y: -2))} boxed ${api.BOXED.value}');
  print('keep ${api.keepPoint(const api.Point(x: 1, y: -2))}');
  // Inferred instances taking a lent closure and a foreign implementation, and one only
  // its impl of a trait names.
  const sides = api.PairU32(left: 2, right: 5);
  print('apply ${api.applyU32(sides, (side) => side * 3)}');
  print('report ${api.reportU32(sides, PrintLog())}');
  print('wrapped ${api.keptWrapU32(const api.WrapU32(value: 4))}');
  print('keeps ${api.keeps(PrintLog()).value}');

  // Its Drop runs when Dart closes it, not before.
  final token = api.Token(7);
  print('token ${token.idGet} dropped ${api.dropped()}');
  token.close();
  print('dropped ${api.dropped()}');

  // A field and the method of its name, each its own member.
  final retry = api.Retry(5);
  final limit = retry.limitGet;
  print('retry ${limit == null ? 0 : 1} ${limit ?? 0} ${retry.limit()} ${retry.delay()}');
  // The renamed field's setter, named as its getter.
  retry.limitSet(null);
  print('retry ${retry.limit()}');
  print('slot ${api.SlotU8(200).value()}');

  // Foreign types: a URL parsed from text, socket addresses, OS paths, and lists.
  final base = api.Url.parse('https://example.com/a/b');
  final host = api.host(base);
  print('url $base host ${host == null ? 'None' : 'Some("$host")'}');
  final url = api.resolve(base.clone(), 'c?d');
  print('resolve $url ${url == base}');
  url.close();
  try {
    api.Url.parse('no scheme').close();
  } on api.StringException catch (e) {
    print('parse err ${e.error}');
  }
  final v4 = api.SocketAddr(InternetAddress('127.0.0.1'), 8080);
  final v6 = api.SocketAddr(InternetAddress('::1'), 443, scopeId: 3);
  print('ports ${api.ports([v4, v6])}');
  final peer = api.Peer(addr: v6, names: ['a', 'b']).next('c');
  print('peer ${peer.debug()} ${quoted(peer.aliases())}');
  final file = api.file('dir', 'f\\\'"\t');
  print('file ${escaped(file)}');
  print('parents ${api.parents([file, '/']).map((p) => p == null ? 'none' : escaped(p)).join(' ')}');
  final words = api.words(' one two  three ');
  print('words ${words.length} ${quoted(words)}');
  print('sentence ${api.sentence(words)} ${api.sentence(null)}');
  print('sentence ${api.sentence([])}|');
  print('chunks ${api.chunks(utf8.encode('abcde'), 2)}');
  final counters = api.counters(['x', 'y']);
  print('counters ${counters.map((c) => '${c.name()} ${c.get()}').join(', ')}');
  // Each counter moves in, so Dart closes it.
  print('worth ${api.worth([counters[0]], [counters[1]])} ${api.worth(null, [])}');
  for (final counter in counters) {
    closed(counter.get, 'Counter', 'a counter moved in is closed');
  }
  final held = [api.heldCount((api.Counter('h', 4),)), api.heldCount(null)];
  print('held ${held.map((n) => n == null ? 'None' : 'Some($n)').join(' ')}');
  print(
    'boxed Some("${api.boxedName(api.boxedCounter('b'))}") '
    '${api.boxedName(null) == null ? 'None' : 'Some'} ${api.boxedCounter(null) == null}',
  );
  print('less ${api.less(5, 2)}');
  print(
    'sums ${api.sums([
      [1, 2],
      [],
      [3],
    ])}',
  );
  final summed = api.summed([const api.Point(x: 1, y: 2), const api.Point(x: -4, y: 6)]);
  final noneSummed = api.summed([]);
  print('summed ${summed.x} ${summed.y} ${noneSummed.x} ${noneSummed.y}');
  print('longest ${api.longest(['a', 'abc'])} ${api.longest([])}');
  final doc = api.Json.parse('{"b": [1, 2.5, null], "a": "é"}');
  print('json ${api.encode(doc)}');
  print('timed text ${api.timedText(doc)}');
  final tagged = api.tagged(doc, 't');
  print('tagged ${tagged.toText()}');
  tagged.close();
  // A JSON object's entries cross as pairs of a key and its document, each moved in.
  final entries = api.entries([('b', api.Json.parse('[1]')), ('a', api.Json.parse('2'))], 't');
  print('entries ${entries.map((e) => '${e.$1} ${e.$2.toText()}').join(', ')}');
  for (final entry in entries) {
    entry.$2.close();
  }
  try {
    api.Json.parse('{"a":').close();
  } on api.StringException catch (e) {
    print('json parse err ${e.error}');
  }
  final site = api.Site(base);
  print('site ${site.homeGet} ${site.homeGet}');

  // Slices, passed in and returned as clones, and the socket address of each family.
  String debugs(List<Object> values) =>
      '[${values.map((v) => switch (v) {
        api.Shape s => s.debug(),
        api.Point p => p.debug(),
        api.Label l => l.debug(),
        _ => throw ArgumentError(v),
      }).join(', ')}]';
  print('shapes ${debugs(api.Shape.ALL)} greetings ${quoted(api.GREETINGS)}');
  final trail = api.Trail([const api.Point(x: 1, y: 2), const api.Point(x: 3, y: 4)], ['a', 'b']);
  print('trail ${debugs(trail.points())} ${debugs(trail.labels())}');
  // Returned iterators, each collected at the call: drained labels are gone after.
  print('iterated ${debugs(trail.iter())} ${quoted(trail.names())} ${trail.bytes()}');
  print('drained ${debugs(trail.drain())} then ${debugs(trail.labels())}');
  trail.close();
  final numbered = api.numbered(['p', 'q']);
  print('numbered ${numbered.map((c) => '${c.name()} ${c.get()}').join(', ')}');
  final (next4, next6) = api.nexts(v4, v6);
  print('nexts ${next4.address.address} ${next4.port} ${next6.port} ${next6.scopeId}');
}

/// `future`'s value, failing past a deadline rather than waiting for a wake that never lands.
Future<T> within<T>(Future<T> future, String what) => future.timeout(
  const Duration(seconds: 5),
  onTimeout: () {
    check(false, what);
    throw StateError('unreachable');
  },
);

/// `future` completes with [api.Cancelled].
Future<void> cancelled(Future<Object?> future, String what) async {
  try {
    await within(future, what);
  } on api.Cancelled {
    return;
  }
  check(false, what);
}

/// A Log that counts the lines it keeps and its closes, running `then` on each line.
final class CountLog extends api.Log {
  final void Function() then;
  int lines = 0;
  int closes = 0;

  CountLog([void Function()? then]) : then = then ?? (() {});

  @override
  bool log(int level, String line) {
    lines++;
    then();
    return true;
  }

  @override
  void close() => closes++;
}

/// The futures' semantics beyond the transcript, each checked by an assertion.
Future<void> futures() async {
  final channel = api.Channel();

  // The send lands from another thread while the receive waits, and so does the wake of a
  // waker Rust keeps.
  final threaded = channel.recv();
  channel.sendFromThread(utf8.encode('threaded'));
  check(
    text(await within(threaded, 'a wake from another thread')) == 'threaded',
    'recv returns what another thread sent',
  );
  var wakes = 0;
  channel.notify(() => wakes++);
  channel.sendFromThread(utf8.encode('rung'));
  await until(() => wakes == 1, 'a waker woken from another thread calls back');
  channel.clear();

  // An implementation Rust calls from another thread, which the call waits on, runs on the
  // isolate's thread meanwhile.
  final threadLog = CountLog();
  check(api.emitFromThread(threadLog, bytes([1, 2, 3])) == 3, 'a call into Dart from a thread the call joins');
  check(threadLog.lines == 3, 'each line logged');
  // Dropped on that thread, so closed from the event loop.
  await until(() => threadLog.closes == 1, 'an implementation dropped on another thread is closed');
  final laterLog = CountLog();
  check(await api.emitLater(laterLog, bytes([1, 2])) == 4, 'an async call and its poll into Dart from a joined thread');
  check(laterLog.lines == 4, 'each line logged, eagerly and once polled');

  // An implementation Rust holds, which a later call that gives none waits on another
  // thread to call, runs on the isolate's thread meanwhile: from a method, and from a
  // cancel whose drop calls it.
  final heldLog = CountLog();
  final logger = api.Logger(heldLog);
  check(logger.writeFromThread('held'), 'a call into a held implementation from a joined thread');
  check(heldLog.lines == 1, 'the held implementation logged');
  logger.close();
  check(heldLog.closes == 1, 'closing the object closes the implementation it held');
  // So does a call the thread made before the isolate waited, which its event loop would
  // have run: `sleep` holds the isolate's thread without running its event loop meanwhile,
  // unless the thread made it while its constructor waited.
  final postedLog = CountLog();
  final posting = api.LogThread(postedLog, 'posted');
  sleep(const Duration(milliseconds: 100));
  check(posting.join(), 'a call into Dart made before the isolate waited on its thread');
  check(postedLog.lines == 1, 'the posted call logged');
  await until(() => postedLog.closes == 1, 'an implementation its thread dropped is closed');
  final dropLog = CountLog();
  final dropping = api.logOnDrop(dropLog);
  await step();
  dropping.cancel();
  check(dropLog.lines == 1, "a cancel calls into the implementation its future's drop holds");
  check(dropLog.closes == 1, 'a cancel closes the implementation its future held');
  await cancelled(dropping, 'a cancelled future that never finishes completes with Cancelled');

  // A wake that lands while the future polls, before it returns pending, as each of
  // `yielded`'s does, is kept: the future polls again rather than waiting for a wake that
  // already happened.
  check(await within(api.yielded(7), 'a wake during the poll polls again') == 7, 'yielded returns its value');

  // A future's output of None is null, and its error is thrown.
  check(await api.halved(3) == null, 'a future ready with None returns null');
  check(await api.halved(4) == 2, 'a future ready with Some returns its value');
  check(await api.cornerLater() == (1, 1), "a future's borrowed tuple returns whole");
  channel.send(Uint8List(0));
  try {
    await channel.recvNonempty();
    check(false, "a future's error throws");
  } on api.ErrorUnderflow catch (e) {
    check(e.by == 1, "a future's error reads back");
  }

  // Cancelled between its wake and its poll, a receive loses nothing: the payload stays
  // for the next.
  final woken = channel.recv();
  channel.send(utf8.encode('kept'));
  woken.cancel();
  await cancelled(woken, 'a receive cancelled after its wake completes with Cancelled');
  // Its wake, still in the port, finds it gone.
  await step();
  check(text(await within(channel.recv(), 'the next receive')) == 'kept', 'a cancelled receive loses nothing');

  // Cancelling a receive that waits for no wake completes it.
  final waiting = channel.recv();
  await step();
  waiting.cancel();
  waiting.cancel();
  await cancelled(waiting, 'a waiting receive cancelled completes with Cancelled');

  // Cancelling a finished future does nothing.
  final done = api.halved(4);
  check(await done == 2, 'halved returns its value');
  done.cancel();
  check(await done == 2, 'a finished future cancelled keeps its value');

  // A handle closed while a call awaits on it stays alive until the call ends.
  final other = api.Channel();
  final held = other.recv();
  final sender = other.clone();
  other.close();
  sender.send(utf8.encode('held'));
  check(
    text(await within(held, 'a receive on a closed handle')) == 'held',
    'a closed handle stays alive in the call awaiting on it',
  );
  sender.close();

  // A future holding an implementation drops it as it finishes, closing it.
  final log = CountLog();
  check(await api.logLater(log), 'the future returns what its log kept');
  check(log.lines == 1 && log.closes == 1, 'a finished future closes the implementation it held');
  print('futures ok');
}

/// Allocate until `done()`, so the garbage collector runs, failing past a deadline.
Future<void> collect(bool Function() done, String what) async {
  final deadline = DateTime.now().add(const Duration(seconds: 10));
  while (!done()) {
    check(DateTime.now().isBefore(deadline), what);
    pressure();
    await step();
  }
}

/// Garbage, young and old, enough that the garbage collector runs.
void pressure() {
  var junk = <Object>[];
  for (var i = 0; i < 64; i++) {
    junk = [Uint8List(1 << 20), for (var j = 0; j < 256; j++) List<int>.filled(16, j)];
  }
  check(junk.isNotEmpty, 'junk');
}

/// What frees an object nobody closed: closing at once, or the garbage collector once it
/// is unreachable, and only what closing did not free already.
Future<void> owned() async {
  for (var n = 65535; n <= 65537; n++) {
    final payload = Uint8List.fromList([for (var i = 0; i < n; i++) i % 256]);
    final packet = api.nextPacket(api.Packet(seq: 1, payload: payload));
    check(
      packet.seq == 2 && packet.payload.length == n && packet.payload.indexed.every((e) => e.$2 == e.$1 % 256),
      "a record's $n bytes cross both ways",
    );
  }
  final before = api.dropped();
  final token = api.Token(1);
  token.close();
  check(api.dropped() == before + 1, 'closing frees at once');
  token.close();
  check(api.dropped() == before + 1, 'closing again frees nothing');
  closed(() => token.idGet, 'Token', 'a closed token refuses a read');

  unclosed();
  await collect(() => api.dropped() >= before + 2, 'the garbage collector frees what nobody closed');
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  check(api.dropped() == before + 2, 'the garbage collector frees each once');

  // An implementation an object the garbage collector frees held is closed from the event
  // loop, as no drop in the collector may call into Dart.
  final log = CountLog();
  unclosedLogger(log);
  await collect(() => log.closes == 1, 'the garbage collector frees what an object it frees holds');
  print('owned ok');
}

@pragma('vm:never-inline')
void unclosed() {
  api.Token(3);
}

@pragma('vm:never-inline')
void unclosedLogger(api.Log log) {
  api.Logger(log);
}

/// Each call taking a closed object throws, however it was closed, and moves nothing.
void closedObjects() {
  final c = api.Counter('c', 1);
  c.close();
  closed(c.get, 'Counter', 'a method of a closed object');
  closed(c.clone, 'Counter', 'clone of a closed object');
  closed(c.debug, 'Counter', 'debug of a closed object');
  final url = api.Url.parse('https://example.com');
  url.close();
  closed(url.toString, 'Url', 'toString of a closed object');
  final tally = api.Tally();
  tally.close();
  closed(() => tally == api.Tally(), 'Tally', '== of a closed object');
  closed(() => api.Tally() == tally, 'Tally', '== with a closed object');
  final registry = api.Registry(api.Counter('first', 1));
  registry.close();
  closed(() => registry.firstGet, 'Registry', "a field's getter of a closed object");
  final second = api.Counter('second', 2);
  closed(() => registry.firstSet(second), 'Registry', "a field's setter of a closed object");
  check(second.get() == 2, 'a call that throws moves nothing in');

  // An argument, alone, or in a list, beside others the call then moves none of.
  closed(() => api.bumpBoth(api.Tally(), tally), 'Tally', 'a closed argument');
  final open = api.Counter('open', 3);
  closed(() => api.worth([open, c], []), 'Counter', 'a closed object in a list');
  closed(() => api.worth([open], [c]), 'Counter', 'a closed object in a later list');
  closed(() => api.Board([('open', open), ('closed', c)]), 'Counter', 'a closed object in a tuple');
  check(open.get() == 3, 'a call that throws moves none of its other arguments');

  // Nor gives Rust an implementation beside an argument that throws, which would keep it
  // unclosed and force each later call through a thread of its own.
  check(api.onFirstThread(), 'a call while Rust holds no implementation');
  final spared = CountLog();
  throws<RangeError>(() => api.logCounts(spared, 256, open, []), 'an integer out of range after an implementation');
  closed(() => api.logCounts(spared, 0, c, []), 'Counter', 'a closed object after an implementation');
  closed(() => api.logCounts(spared, 0, open, [c]), 'Counter', 'a closed object in a list after an implementation');
  check(spared.closes == 0, 'a call that throws gives Rust no implementation');
  check(api.onFirstThread(), "a call after one that threw runs on the isolate's thread");
  check(api.logCounts(spared, 0, open, []) == 1, 'an implementation a call that threw gave none of');
  check(spared.lines == 1 && spared.closes == 1, 'an implementation given once is closed once');

  // An async call.
  final channel = api.Channel();
  channel.close();
  closed(channel.recv, 'Channel', 'an async call of a closed object');

  // Closed by a call that moved it, plain or boxed, or once the closure it was lent to
  // returned.
  final moved = api.Counter('m', 0);
  moved.intoLabel(null);
  closed(moved.get, 'Counter', 'a method of a moved object');
  final tail = api.Node(1, null);
  api.Node(2, tail);
  closed(tail.sum, 'Node', 'a method of a boxed moved object');
  late api.Counter kept;
  api.Registry(api.Counter('c', 0)).edit((counter) {
    kept = counter;
    return false;
  });
  closed(kept.get, 'Counter', 'a lent object kept past its closure');

  // Closed while a call holds it, from the closure the call lends, the object stays alive
  // until the call returns, and is freed then.
  final editing = api.Registry(api.Counter('e', 4));
  final edited = editing.edit((counter) {
    editing.close();
    closed(() => editing.firstGet, 'Registry', 'an object closed in a closure refuses calls');
    return counter.get() == 4;
  });
  check(edited, 'the call holding a closed object still reaches it');
  closed(() => editing.firstGet, 'Registry', 'an object closed in a closure stays closed');
  // As the drop of what it holds shows: once, after the call.
  late api.Logger logger;
  late CountLog log;
  log = CountLog(() {
    logger.close();
    check(log.closes == 0, 'an object closed during a call it is held by outlives the call');
  });
  logger = api.Logger(log);
  check(logger.write('x'), 'the call holding a closed object returns');
  check(log.closes == 1, 'an object closed during a call it is held by is freed as it returns');
  logger.close();
  check(log.closes == 1, 'closing it again frees nothing');
  print('closed ok');
}

/// Returned bytes up to 64 KiB are copied, and longer ones lend Rust's buffer, read-only,
/// kept alive by any view of it and freed by the garbage collector once none is reachable.
Future<void> lent() async {
  var before = api.dropped();
  final small = api.tokenBytes(1, 65536);
  check(api.dropped() == before + 1, "64 KiB are copied and Rust's freed at the call");
  check(small.length == 65536 && small.every((b) => b == 1), 'copied bytes read back');
  before++;
  final big = api.tokenBytes(1, 65537);
  check(api.dropped() == before, "64 KiB and 1 byte lend Rust's buffer");
  check(big.length == 65537 && big.every((b) => b == 1), 'lent bytes read back');
  throws<UnsupportedError>(() => big[0] = 2, 'lent bytes are read-only');
  throws<UnsupportedError>(() => Uint8List.sublistView(big, 1)[0] = 2, 'a view of lent bytes is read-only');

  // Kept alive by a view alone, through collections that free an old object beside it.
  final canary = [api.Token(9)];
  Uint8List? view = subView(2);
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  canary.clear();
  await collect(() => api.dropped() > before, 'the garbage collector frees an old object');
  check(api.dropped() == before + 1, "a view keeps Rust's buffer");
  check(view.length == 10 && view.every((b) => b == 2), "a view reads Rust's buffer after its list is gone");
  view = null;
  await collect(() => api.dropped() >= before + 2, 'the garbage collector frees bytes no view reaches');
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  check(api.dropped() == before + 2, 'the garbage collector frees bytes once');
  check(big[65536] == 1, 'lent bytes read back while reachable');
  print('bytes ok');
}

/// A view of returned bytes, whose list nothing else keeps.
@pragma('vm:never-inline')
Uint8List subView(int id) => Uint8List.sublistView(api.tokenBytes(id, 65537), 65527);

/// Values at the edges of what Rust takes and returns.
void bounds() {
  // A lone surrogate is one rune but no Unicode scalar.
  check(api.nextChar('a') == 'b' && api.nextChar('\u{1f600}') == '\u{1f601}', 'a char');
  throws<ArgumentError>(() => api.nextChar(String.fromCharCode(0xd800)), 'a low surrogate');
  throws<ArgumentError>(() => api.nextChar(String.fromCharCode(0xdfff)), 'a high surrogate');
  check(api.nextChar('\u{d7ff}') == null, 'the scalar before the surrogates');

  // Dart's longest Duration is 9223372036854 seconds and 775807 microseconds.
  for (final us in [9223372036854000000, 9223372036854775806]) {
    final d = Duration(microseconds: us);
    check(api.extended(d, Duration.zero) == d, 'a Duration near the longest, $us');
  }
  const longest = Duration(microseconds: 0x7fffffffffffffff);
  check(api.extended(longest, Duration.zero) == longest, 'the longest Duration');

  check(api.ports([api.SocketAddr(InternetAddress('::1'), 65535)]).single == 65535, 'the last port');
  throws<RangeError>(() => api.ports([api.SocketAddr(InternetAddress('::1'), 65536)]), 'a port past u16');
  throws<RangeError>(() => api.ports([api.SocketAddr(InternetAddress('::1'), 1, scopeId: -1)]), 'a negative scope');
  throws<RangeError>(
    () => api.ports([api.SocketAddr(InternetAddress('::1'), 1, flowinfo: 0x100000000)]),
    'a flow label past u32',
  );
  print('ok');
}

/// Each breaks a contract, which must fail with a message naming it.
void broken(String which) {
  switch (which) {
    case 'moved':
      final c = api.Counter('c', 0);
      c.intoLabel(null);
      c.get();
    case 'boxed moved':
      final tail = api.Node(1, null);
      api.Node(2, tail);
      tail.sum();
    case 'lent moved':
      api.Registry(api.Counter('c', 0)).edit((counter) {
        counter.intoLabel(null);
        return false;
      });
    case 'lent kept':
      late api.Counter kept;
      api.Registry(api.Counter('c', 0)).edit((counter) {
        kept = counter;
        return false;
      });
      kept.get();
    case 'callback threw':
      api.emit(CountLog(() => throw StateError('a log failed')), bytes([1]));
    // An IPv6 address where Rust takes `SocketAddrV4`.
    case 'family':
      final any = api.SocketAddr(InternetAddress('::'), 1);
      api.nexts(any, any);
    // No String holds a path that is not UTF-8.
    case 'stray':
      api.stray();
    case 'lent stray':
      api.visitStray((_) {});
    default:
      check(false, 'no case $which');
  }
}

Future<void> main(List<String> args) async {
  switch (args) {
    case []:
      await script();
    case ['futures']:
      await futures();
    case ['owned']:
      await owned();
    case ['closed']:
      closedObjects();
    case ['bytes']:
      await lent();
    case ['bounds']:
      bounds();
    case [final which]:
      broken(which);
    default:
      check(false, 'no case ${args.join(' ')}');
  }
}
