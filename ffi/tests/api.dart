// The api fixture's Dart test: the script fixtures/api/examples/transcript.rs runs
// natively, through the generated Dart. tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks the futures' semantics, what closing and the garbage
// collector free, what a closed object refuses, how returned bytes cross, or breaks a
// contract, which must fail.

import 'dart:async';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:typed_data';

import 'package:api/api.dart' as api;

/// The unsigned value of a `u64`'s bits, which Dart's `int` reads as signed.
String u64(int v) => BigInt.from(v).toUnsigned(64).toString();

/// The `f64` of `bits`.
double f64(int bits) => (Uint64List(1)..[0] = bits).buffer.asFloat64List()[0];

/// Each float's bits, as transcript.rs prints them.
String bits32(Float32List v) => Uint32List.fromList(v.buffer.asUint32List(v.offsetInBytes, v.length)).toString();

String bits64(Float64List v) => '[${v.buffer.asUint64List(v.offsetInBytes, v.length).map(u64).join(', ')}]';

/// Print each of `n`'s lists, its floats as their bits, as transcript.rs does.
void numbers(String what, api.Numbers n) {
  print('$what u16 ${n.u16s}');
  print('$what u32 ${n.u32s}');
  print('$what u64 [${n.u64s.map((v) => v.toRadixString(10)).join(', ')}]');
  print('$what i8 ${n.i8s}');
  print('$what i16 ${n.i16s}');
  print('$what i32 ${n.i32s}');
  print('$what i64 ${n.i64s}');
  print('$what f32 ${bits32(n.f32s)}');
  print('$what f64 ${bits64(n.f64s)}');
  final some = n.some;
  print('$what some ${some == null ? 'None' : 'Some($some)'}');
}

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

/// Bytes in hex, as transcript.rs prints them.
String hex(List<int> bytes) => bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join();

/// A socket address as its IP's bytes in hex, its port, flow label, and scope, each
/// IPv6's alone, apart by colons, as transcript.rs prints it.
String socketHex(api.SocketAddr addr) => switch (addr) {
  api.SocketAddrV4(:final ip, :final port) => '${hex(ip.octets)}:$port',
  api.SocketAddrV6(:final ip, :final port, :final flowinfo, :final scopeId) =>
    '${hex(ip.octets)}:$port:$flowinfo:$scopeId',
};

String text(Uint8List data) => utf8.decode(data);

String quoted(List<String> values) => '[${values.map((v) => '"$v"').join(', ')}]';

/// A path's bytes as Rust's `escape_ascii` prints them.
String escaped(api.PathBuf path) {
  final out = StringBuffer();
  for (final b in path.bytes) {
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

/// `f` throws the StateError of a closed `type`: Dart's own, for a value whose calls only
/// borrow it, or else Rust's check of a moved value, naming the function.
void closed(void Function() f, String type, String what) {
  try {
    f();
  } on StateError catch (e) {
    final moved = e.message.startsWith('api_') && e.message.endsWith(": a handle's value was moved out");
    check(e.message == '$type is closed' || moved, '$what: $e');
    return;
  }
  check(false, what);
}

/// `f` throws the StateError `message` of a close under a borrow.
void borrowed(void Function() f, String message, String what) {
  try {
    f();
  } on StateError catch (e) {
    check(e.message == message, '$what: $e');
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

  final c = api.Counter('c', api.U64(5));
  print('get ${c.get()}');
  print('name ${c.name()}');
  print('add ok ${c.add(api.U64(10))}');
  try {
    c.add(api.U64(2000));
  } on api.Error catch (e) {
    print('add err $e ${e.debug()}');
  }
  try {
    c.sub(api.U64(20));
  } on api.Error catch (e) {
    print('sub err $e ${e.debug()}');
  }
  c.rename('d');
  print('name ${c.name()}');
  print('is d ${c.is_('d')}');
  final copy = c.clone();
  copy.add(api.U64(1));
  print('clone ${copy.get()} original ${c.get()}');
  copy.close();
  print('debug ${c.debug()}');
  final label = c.intoLabel(p);
  print('label ${label.debug()}');
  print('note ${api.hasText(const api.Note(text: ''))} ${api.hasText(const api.Note(text: null))}');

  final (below, above) = api.Counter('s', api.U64(7)).split(api.U64(5));
  print('split ${below.name()} ${below.get()} ${above.name()} ${above.get()}');
  final (only,) = api.only(api.Counter('o', api.U64(3)));
  print('only ${only.name()} ${only.get()}');
  final (w8, w16, w32, w64) = api.widths(api.U64(98305));
  print('widths $w8 $w16 $w32 $w64');
  for (final b in [2, 0]) {
    try {
      final (q, r) = api.divide(api.U64(7), api.U64(b));
      print('divide $q $r');
    } on api.Error catch (e) {
      print('divide err $e');
    }
  }
  for (final n in [98305, 0]) {
    final spread = api.spread(api.U64(n));
    print(spread == null ? 'spread none' : 'spread ${spread.$1} ${spread.$2} ${spread.$3} ${spread.$4}');
  }
  for (final needle in ['lo', 'x']) {
    final found = api.find(utf8.encode('hello'), utf8.encode(needle));
    print(found == null ? 'find none' : 'find ${found.$1} ${found.$2}');
  }
  final statRows = api.rows([const api.Point(x: 1, y: 2), const api.Point(x: 3, y: 3)]);
  print('rows ${statRows.map((r) => '${r.$1} ${r.$2.debug()} ${r.$3.x} ${r.$3.y}').join(', ')}');
  // Each handle in a tuple moves in, as one taken by value does.
  final board = api.Board([('low', api.Counter('l', api.U64(1))), ('high', api.Counter('h', api.U64(9)))]);
  final ranked = board.ranked();
  print('ranked ${ranked.map((r) => '${r.$1} ${r.$2.name()} ${r.$2.get()}').join(', ')}');
  final best = board.best();
  print('best ${best.name()} ${best.get()}');
  best.close();
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
  final registry = api.Registry(api.Counter('first', api.U64(1)));
  print('last ${registry.last() != null}');
  registry.push(api.Counter('last', api.U64(2)));
  final first = registry.firstGet;
  first.add(api.U64(10));
  print('first ${registry.firstGet.get()} lent ${first.get()}');
  final last = registry.last()!;
  last.add(api.U64(10));
  print('last ${registry.last()!.get()} lent ${last.get()}');
  // A closure lent for the call edits the first counter through a handle.
  const by = 5;
  final edited = registry.edit((counter) {
    counter.add(api.U64(by));
    return counter.edited() && counter.get() == 6;
  });
  print('edit $edited ${registry.firstGet.get()}');
  // A setter moves an object in, as assignment does, closing it.
  registry.firstSet(api.Counter('second', api.U64(3)));
  print('set first ${registry.firstGet.get()} ${registry.firstGet.name()}');
  print('walk ${api.walk(api.U64(3), (cursor) => api.U64(cursor.at().toInt() * 2))}');
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
  final lit = api.Name.borrowed('lit');
  print('borrowed ${lit.asStr()} ${lit.shares(api.Name.borrowed('lit'))} ${lit.shares(api.Name.borrowed('other'))}');

  print('total ${api.total(api.RangeU64(api.U64(2), api.U64(5)))}');
  print('reach ${api.reach(2, 5)}');
  final scaled = [
    api.scaled((BigInt.one << 128) - BigInt.one, api.U64(1)),
    api.scaled((BigInt.one << 64) + BigInt.one, api.U64.fromBits(1 << 63)),
    api.scaled(BigInt.one << 127, api.U64(2)),
  ];
  print('scaled ${scaled.map((v) => v?.toRadixString(16).padLeft(32, '0') ?? 'none').join(' ')}');
  final digest = api.Digest();
  digest.sumSet(api.fingerprint('deadbeef'));
  var unparsed = '';
  try {
    api.fingerprint('zz');
  } on api.StringException catch (e) {
    unparsed = e.error;
  }
  print('digest ${digest.sumGet} ${digest.flipped(api.ArrayU8x4([0xff, 0, 0xff, 0]))} $unparsed');
  digest.close();
  final batch = api.Batch(), batch2 = api.pair();
  final accepted = [for (var v = 1; v <= 5; v++) batch.push(v)].where((p) => p).length;
  final paired = [for (var v = 1; v <= 3; v++) batch2.push(v)].where((p) => p).length;
  final emptied = api.drain4(batch);
  print('batch ${batch.capacity()} $accepted $emptied ${api.drain4(batch)} ${batch2.capacity()} $paired ${api.drain2(batch2)}');
  print('capacities ${api.capacities42(batch, batch2)} ${api.capacities22(batch2, batch2)}');
  batch.close();
  batch2.close();
  final wide = api.widen(api.RangeU64(api.U64(2), api.U64(5)), api.U64(1));
  print('widen ${wide.start} ${wide.end}');
  final counts = [
    api.BoundsU64(api.BoundU64Included(api.U64(2)), api.BoundU64Excluded(api.U64(5))),
    api.BoundsU64(api.BoundU64Included(api.U64(2)), api.BoundU64Included(api.U64(5))),
    api.BoundsU64(api.BoundU64Included(api.U64(3)), api.BoundU64Unbounded()),
    api.BoundsU64(api.BoundU64Unbounded(), api.BoundU64Unbounded()),
    api.BoundsU64(api.BoundU64Excluded(api.U64(2)), api.BoundU64Included(api.U64(4))),
  ].map(api.count);
  print('count ${counts.join(' ')}');
  print('end ${bound(api.end(api.U64(3)))} ${bound(api.end(api.U64(0)))}');
  final (endsStart, endsEnd) = api.ends(api.U64(3));
  print('ends ${bound(endsStart)} ${bound(endsEnd)}');
  for (final n in [2, 5]) {
    try {
      print('below ${api.below(api.U64(n), api.U64(3))}');
    } on api.BoundsU64Exception catch (e) {
      print('below err ${bound(e.error.start)} ${bound(e.error.end)}');
    }
  }
  final window = api.Window(range: api.RangeU64(api.U64(2), api.U64(8)), end: api.end(api.U64(5)));
  print('window ${window.contains(api.U64(4))} ${window.contains(api.U64(6))} ${window.debug()}');

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
  final stamp = api.Stamp.fromMillis(api.U64(2500));
  print('secs ${stamp.secs()}');
  print('later ${api.Stamp.fromMillis(api.U64(3000)).later(stamp)}');
  print('secs ${stamp.secs()}');
  print('whole ${stamp.whole().debug()}');
  final message = api.Message(stamp, utf8.encode('body'));
  print('payload ${text(message.payloadGet)}');
  print('stamp ${message.stampGet!.debug()}');
  print('sent ${message.sent()}');
  // Setters assign in place: the bytes are copied in, and a `Copy` handle copied.
  final reset = api.Stamp.fromMillis(api.U64(4000));
  message.stampSet(reset);
  message.payloadSet(utf8.encode('edited'));
  print('payload ${text(message.payloadGet)}');
  print('stamp ${message.stampGet!.debug()}');
  print('secs ${reset.secs()}');
  message.stampSet(null);
  print('stamp ${message.stampGet != null}');

  // A value past a word, copied across: a closure takes and returns one by value.
  final moment = api.Moment.new_(api.U64(1500), api.U64(1000))!;
  print('moment ${moment.millis()} ${api.Moment.new_(api.U64(1), api.U64(0)) == null}');
  final shifted = api.shifted(moment, (m) => m.after(api.U64(500)));
  print('shifted ${shifted.millis()} ${shifted == moment.after(api.U64(500))}');
  print('wrapped ${moment.after(api.U64.max).millis()} ${moment == shifted}');

  // A value with padding, or bytes that vary, is a handle, as no table says where padding is.
  final gap = api.Gap(api.U64(7), 3);
  print('gap ${gap.sum()} ${gap == api.Gap(api.U64(7), 3)}');
  final berth = api.Berth(4);
  print('berth Some(${berth.at()}) ${api.Berth(null).at() == null ? 'None' : 'Some'}');

  // A `Copy` type changed in place is a value, which a `&mut self` method returns as the
  // call left it, Rust's own return beside it: a change reaches only the copy it replaces.
  var score = api.Score.of(api.U64(2));
  final increment = api.Score.of(api.U64(3));
  score = score.add(increment);
  score = score.add(increment);
  print('score ${score.count()} ${increment.doubled().count()} ${increment.count()}');
  var (scoreCopy, taken) = score.take();
  print('taken $taken ${scoreCopy.count()} ${score.count()}');
  final otherScore = api.Mut(increment);
  final (absorbed, before0, before1) = scoreCopy.absorb(otherScore);
  scoreCopy = absorbed;
  print('absorbed $before0 $before1 ${scoreCopy.count()} ${otherScore.value.count()} ${increment.count()}');
  var hits = api.Hits.of(api.U64(1), api.U64(2));
  final keptHits = hits;
  hits = hits.add(api.Hits.of(api.U64(10), api.U64(20)));
  hits = hits.hitsSet(api.U64(7));
  print('hits ${hits.hitsGet} ${hits.missesGet} ${keptHits.hitsGet} ${keptHits.missesGet} ${hits == keptHits}');
  final (counted, countedSame) = keptHits.eq(keptHits);
  print('counted $countedSame ${counted.hitsGet}');
  var extent = api.Extent(api.U64(5), api.U64(3));
  final extentStart = extent;
  final (slid, old) = extent.slide(api.U64(10));
  extent = slid;
  extent = extent.startSet(api.U64(extent.startGet.toInt() + 1));
  print('extent $old ${extent.startGet} ${extent.end()} ${extentStart.startGet} ${extentStart.end()}');
  final weighted = api.Weighted(score: increment, weight: 4);
  print('weighted ${weighted.total()} ${weighted.score.count().toInt() + increment.count().toInt()}');
  // A checked mark beside bytes in a tuple and in a record lent, and bytes beside a move.
  final mark = api.Mark.of(api.U64(1));
  mark.add(api.U64(2));
  final parcel = api.Parcel(mark: mark, payload: bytes([97, 98, 99]));
  print('heft ${api.heft((mark, bytes([97, 98])), parcel, (api.Counter('w', api.U64(1)), bytes([97, 98, 99, 100])))}');
  // A value's `&mut self` beside a checked handle.
  final (noted, notedBefore) = api.Score.of(api.U64(2)).note(api.Mark.of(api.U64(5)));
  print('noted $notedBefore ${noted.count()}');

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
  final fill4 = api.Batch(), fill2 = api.pair();
  print('fill ${[...await api.fill4(fill4, 9), ...await api.fill2(fill2, 1)].join(' ')}');
  fill4.close();
  fill2.close();
  // The send happens at the call: the future is dropped unawaited.
  channel.sendLater(utf8.encode('eager'));
  print('recv ${text(await channel.recv())}');
  // The send happens at the call, and the future borrows the channel and the tag.
  print('tagged ${await channel.tagged('tag')}');
  print('itemized ${await channel.itemized(Uint32List.fromList([1, 2, 3]))}');
  print('byte_len ${await api.byteLen(utf8.encode('abc'))}');
  channel.send(utf8.encode('sized'));
  final (payload, size) = await channel.recvSized();
  print('sized ${text(payload)} $size');
  // The waker moves into Rust, which keeps it until the send wakes it.
  channel.notify(() => wakes++);
  channel.send(utf8.encode('rung'));
  await until(() => wakes == 2, 'the send wakes the waker Rust keeps');
  print('woken $wakes');
  // A poll with no async twin is a future.
  print('take ${text(await channel.pollTake())}');
  channel.send(utf8.encode('measured'));
  print('measure ${await channel.pollMeasure(api.U64(2))}');
  // A context is made of the waker, which Rust keeps until the send wakes it.
  channel.listen(() => wakes++);
  channel.send(utf8.encode('heard'));
  await until(() => wakes == 3, 'the send wakes the context Rust keeps');
  print('woken $wakes');
  // A returned waker wakes the waiter's task through Rust until it is closed, after which
  // a wake throws.
  final waiter = api.Waiter(() => wakes++);
  final waker = waiter.waker(), context = waiter.context();
  waker();
  context();
  waiter.close();
  waker.close();
  context.close();
  context.close();
  throws<StateError>(waker.call, 'a closed waker throws');
  await until(() => wakes == 5, 'a returned waker wakes the waiter');
  print('woken $wakes');

  final nodes = api.Node(api.U64(1), null);
  nodes.push(api.U64(2));
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
    stack.push(api.Counter(n, api.U64(i + 1)));
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
  // Names that clash in some language, each kept apart there.
  const clashSpan = api.ClashSpan(width: 4);
  final clashStore = api.ClashStore(7);
  print('clash ${const api.ClashTone(hz: 440).debug()} ${const api.ClashTone_(db: 3).debug()} '
      '${const api.Cancelled_(step: 2).debug()} span ${clashSpan.width_()} ${clashSpan.toString_()} $clashSpan '
      'store ${clashStore.free()} ${clashStore.close_()} '
      'audio ${api.clashAudioEncoderFinish()} ${api.clashAudioEncoderFinish_()} '
      'nested ${api.clashNested(((1, 2), 3), ((1, 2, 3),))}');
  clashStore.close();
  // Instances that name each other, and one made only by an impl of it.
  final cross = api.cross(
    const api.LeftRightU8(value: api.RightU8(value: 1)),
    const api.RightLeftU8(value: api.LeftU8(value: 2)),
  );
  print('cross $cross outer ${api.OuterInnerU8(4).get()}');
  // A bound that outlives the call takes a value, and an instance only a constant names.
  print('kept ${api.keptPoint(const api.Point(x: 1, y: -2))} boxed ${api.BOXED.value}');
  print('keep ${api.keepPoint(const api.Point(x: 1, y: -2))}');
  const five = api.Point(x: 5, y: 2);
  print('kept or ${api.keptOrPoint(five)} ${api.keptOrPoint(null)} keeper ${api.keeperPoint(five)}');
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
  final v4 = api.SocketAddrV4(api.Ipv4Addr.from(InternetAddress('127.0.0.1')), 8080);
  final v6 = api.SocketAddrV6(api.Ipv6Addr.from(InternetAddress('::1')), 443, scopeId: 3);
  print('ports ${api.ports([v4, v6])}');
  final peer = api.Peer(addr: v6, names: ['a', 'b']).next('c');
  print('peer ${peer.debug()} ${quoted(peer.aliases())}');
  final file = api.file(api.PathBuf('dir'), 'f\\\'"\t');
  print('file ${escaped(file)}');
  print('parents ${api.parents([file, api.PathBuf('/')]).map((p) => p == null ? 'none' : escaped(p)).join(' ')}');
  final words = api.words(' one two  three ');
  print('words ${words.length} ${quoted(words)}');
  print('sentence ${api.sentence(words)} ${api.sentence(null)}');
  print('sentence ${api.sentence([])}|');
  print('either ${api.either('abc')} ${api.either(null)}');
  print('prefixed ${utf8.decode(api.prefixed(utf8.encode('cd'), utf8.encode('ab')))} '
      '${utf8.decode(api.prefixed(utf8.encode('cd'), null))}');
  print('coordinates ${api.coordinates(const api.Point(x: 2, y: 3))} ${api.coordinates(null)}');
  final two = api.Counter('two', api.U64(2));
  final three = api.Counter('three', api.U64(3));
  print('plus ${two.plus(three)} ${two.plus(null)}');
  print('add_to ${two.addTo(three)} ${two.addTo(null)} ${three.get()}');
  two.close();
  three.close();
  final encoded = api.Mut(utf8.encode('0,'));
  api.putNumber(1, encoded);
  api.putNumber(23, encoded);
  print('put_number ${utf8.decode(encoded.value)}');
  final entry = api.Mut((utf8.encode('ab'), 1));
  final stamps = api.pushCounted(entry, 0x63);
  print('push_counted $stamps ${utf8.decode(entry.value.$1)} ${entry.value.$2}');
  final digits = api.Mut(utf8.encode('7x9'));
  final firstDigit = api.digit(digits);
  var skipped = false;
  try {
    api.digit(digits);
  } on api.Error {
    skipped = true;
  }
  print('digit $firstDigit $skipped ${api.digit(digits)} ${digits.value.length}');
  final nudged = api.Mut(const api.Point(x: 1, y: 2));
  api.nudge(nudged, 3);
  print('nudge ${nudged.value.x} ${nudged.value.y}');
  final config = api.secure('h3');
  print('secure ${api.alpnLent(config)} ${api.alpn(config.clone())}');
  final cert = api.certify(utf8.encode('der'));
  final certLen = api.certLen(cert);
  print('cert $certLen ${text(api.certDer(cert))} ${api.chainLen([api.certify(utf8.encode('a')), api.certify(utf8.encode('bc'))])}');
  config.close();
  // Another crate's trait objects: an `Arc`, taken back, and `'static` ones, held by value.
  print('resolver ${api.resolved(api.resolver('r1'))}');
  final codecs = api.Codecs();
  print('codecs ${codecs.ids()}');
  codecs.listSet(codecs.listGet.reversed.toList());
  print('codecs reversed ${codecs.ids()}');
  codecs.close();
  // A standard library value no type maps, in a handle.
  final failure = api.failure('boom');
  print('failure ${api.failureMessage(failure)} ${api.failureKind(failure)}');
  // Iterators, passed as the arrays of their items.
  print('sum_all ${api.sumAll([1, 2, 3])}');
  print('weigh_all ${api.weighAllCounter([api.Counter('w', api.U64(3))])}');
  print('offered_len ${api.offeredLen([utf8.encode('h3'), utf8.encode('moq-00')])}');
  print('join_words ${api.joinWords(['a', 'b', 'c'])}');
  // Bytes that must be `'static`, interned.
  for (final label in ['der', 'der again']) {
    final der = api.Der.fromSlice(utf8.encode('der'));
    print('$label ${text(der.bytes())}');
    der.close();
  }
  final session = api.resume('h3', 60);
  print('resume ${session.alpn()} ${session.lifetime().seconds}');
  session.close();
  try {
    api.resume('', 60);
  } on api.StringException catch (e) {
    print('resume err ${e.error}');
  }
  final meters = api.Meters();
  final lentMeter = meters.first();
  lentMeter.add(api.U64(2));
  print('lent ${lentMeter.add(api.U64(3))} ${meters.first().count()}');
  lentMeter.close();
  meters.clear();
  print('cleared ${meters.first().count()}');
  final lastMeter = meters.lastGet;
  lastMeter.add(api.U64(4));
  print('field lent ${lastMeter.add(api.U64(1))}');
  lastMeter.close();
  // An optional field that is not Clone, lent as `Some` of a handle.
  print('spare ${meters.spareGet != null}');
  meters.spareSet(api.Meter.default_());
  final spare = meters.spareGet!;
  spare.add(api.U64(5));
  print('spare lent ${spare.add(api.U64(1))}');
  spare.close();
  meters.close();
  final tallies = api.Tallies();
  tallies.first().bump();
  print('tallied ${tallies.first().bump()}');
  tallies.close();
  print('chunks ${api.chunks(utf8.encode('abcde'), api.U64(2))}');
  final counters = api.counters(['x', 'y']);
  print('counters ${counters.map((c) => '${c.name()} ${c.get()}').join(', ')}');
  // Each counter moves in, so a later call throws.
  print('worth ${api.worth([counters[0]], [counters[1]])} ${api.worth(null, [])}');
  for (final counter in counters) {
    closed(counter.get, 'Counter', 'a counter moved in is closed');
  }
  final held = [api.heldCount((api.Counter('h', api.U64(4)),)), api.heldCount(null)];
  print('held ${held.map((n) => n == null ? 'None' : 'Some($n)').join(' ')}');
  print(
    'boxed Some("${api.boxedName(api.boxedCounter('b'))}") '
    '${api.boxedName(null) == null ? 'None' : 'Some'} ${api.boxedCounter(null) == null}',
  );
  print('less ${api.less(5, 2)}');
  print(
    'sums ${api.sums([
      Uint32List.fromList([1, 2]),
      Uint32List(0),
      Uint32List.fromList([3]),
    ])}',
  );
  print('least ${api.least([[3, 1], [], [2]]).join(' ')}');
  numbers('edges', api.Numbers.edges());
  numbers('reversed', api.Numbers.edges().reversed());
  final edges = api.Numbers.edges();
  final nan = api.Measure.new_(double.nan, false);
  print('float equal ${edges == edges} ${edges == api.Numbers.edges()} ${api.Numbers.zero(true) == api.Numbers.zero(false)} ${nan == nan} ${api.Measure.new_(-0.0, true) == api.Measure.new_(0.0, false)}');
  print('sum u32 ${api.sumU32(Uint32List(0))} ${api.sumU32(Uint32List.fromList([0xffffffff, 0xffffffff, 2]))}');
  print('doubled i64 ${api.doubledI64(Int64List.fromList([-0x8000000000000000, -1, 0, 0x7fffffffffffffff]))}');
  final floats = api.boxedFloats(Float64List.fromList([-0.0, f64(0x7ff8000000000001)]));
  print('boxed floats ${bits64(floats)}');
  final summed = api.summed([const api.Point(x: 1, y: 2), const api.Point(x: -4, y: 6)]);
  final noneSummed = api.summed([]);
  print('summed ${summed.x} ${summed.y} ${noneSummed.x} ${noneSummed.y}');
  print('longest ${api.longest(['a', 'abc'])} ${api.longest([])}');
  print('first of Some("${api.firstOf(['a', 'b'])}") ${api.firstOf([]) ?? 'None'}');
  final limits = api.byteLimits(utf8.encode('a'), utf8.encode('bc'));
  final waits = api.breadth(
    api.LimitsDuration(
      low: api.RustDuration.from(const Duration(milliseconds: 5)),
      high: api.RustDuration.from(const Duration(seconds: 2)),
    ),
  );
  print('limits ${utf8.decode(limits.low)} ${utf8.decode(limits.high)} breadth ${waits.toDuration().inMilliseconds}');
  final parts = utf8.decode(api.joined([utf8.encode('ab'), Uint8List(0), utf8.encode('c')]));
  print('joined $parts ${api.joined([]).length} read ${api.readLen(utf8.encode('hello'))}');
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
  print('both ${debugs(api.Shape.BOTH)} primes ${api.PRIMES}');
  final trail = api.Trail([const api.Point(x: 1, y: 2), const api.Point(x: 3, y: 4)], ['a', 'b']);
  print('trail ${debugs(trail.points())} ${debugs(trail.labels())}');
  // Returned iterators, each collected at the call: drained labels are gone after.
  print('iterated ${debugs(trail.iter())} ${quoted(trail.names())} ${trail.bytes()}');
  print('drained ${debugs(trail.drain())} then ${debugs(trail.labels())}');
  // A filter, collected at the call.
  print('beyond ${trail.beyond(1).map((p) => '${p.x} ${p.y}').join(', ')}');
  trail.close();
  final numbered = api.numbered(['p', 'q']);
  print('numbered ${numbered.map((c) => '${c.name()} ${c.get()}').join(', ')}');
  final (next4, next6) = api.nexts(v4, v6);
  print('nexts ${next4.ip} ${next4.port} ${next6.port} ${next6.scopeId}');
  final (nextIp4, nextIp6) = api.nextIps(
    api.Ipv4Addr.from(InternetAddress('10.0.0.255')),
    api.Ipv6Addr.from(InternetAddress('::ffff')),
  );
  final mapped = api.flip(api.IpAddr.from(InternetAddress('127.0.0.1')));
  final ips = [nextIp4, nextIp6, mapped, api.flip(mapped), api.flip(api.IpAddr.from(InternetAddress('::1')))];
  print('ips ${ips.map((ip) => hex(ip.octets)).join(' ')}');

  // Lossless round trips: each type's edge values, passed through Rust and back.
  final durations = [
    api.RustDuration.zero,
    api.RustDuration(api.U64(0), 1),
    api.RustDuration(api.U64(9223372036), 854775808),
    api.RustDuration(api.U64.max, 999999999),
  ].map(api.sameDuration);
  print('lossless duration ${durations.map((d) => '${d.secs.toRadixString(10)}.${d.nanos.toString().padLeft(9, '0')}').join(' ')}');
  final u64s = [api.U64(0), api.U64.fromBigInt(BigInt.one << 63), api.U64.max];
  print('lossless u64 ${u64s.map((v) => api.sameU64(v).toRadixString(10)).join(' ')}');
  print('lossless usize ${[api.U64(0), api.U64.max].map((v) => api.sameUsize(v).toRadixString(10)).join(' ')}');
  print('lossless isize ${api.sameIsize(-0x8000000000000000)} ${api.sameIsize(0x7fffffffffffffff)}');
  final u128s = [BigInt.zero, BigInt.one << 64, (BigInt.one << 128) - BigInt.one];
  print('lossless u128 ${u128s.map((v) => api.sameU128(v).toRadixString(16).padLeft(32, '0')).join(' ')}');
  final chars = ['\u{0}', '\u{d7ff}', '\u{e000}', '\u{10ffff}'];
  print('lossless char ${chars.map((c) => api.sameChar(c).runes.single.toRadixString(16)).join(' ')}');
  // A path no Rust string spells: bytes that are not UTF-8, or on Windows, WTF-8's lone surrogate.
  final unspelled = api.PathBuf.fromBytes(Platform.isWindows ? [0xed, 0xa0, 0x80] : [0xff]);
  print('lossless path ${[api.PathBuf('a/é'), unspelled].map((p) => hex(api.samePath(p).bytes)).join(' ')}');
  final sockV4 = api.sameSocketAddrV4(api.SocketAddrV4(api.Ipv4Addr.fromOctets([255, 255, 255, 255]), 65535));
  final sockV6 = api.sameSocketAddrV6(
    api.SocketAddrV6(
      api.Ipv6Addr.fromOctets([0, 1, for (var i = 0; i < 13; i++) 0, 2]),
      3,
      flowinfo: 0xffffffff,
      scopeId: 0xffffffff,
    ),
  );
  final socks = [sockV4, sockV6].map(api.sameSocketAddr);
  print('lossless socket ${socketHex(sockV4)} ${socketHex(sockV6)} ${socks.map(socketHex).join(' ')}');
  // A key hashes by Rust's `Hash`: equal keys alike, and these two apart.
  final key = api.Key('a');
  print('key ${key.hashCode == api.Key('a').hashCode} ${key.hashCode == api.Key('b').hashCode}');
  final sameIps = [
    api.sameIpAddr(api.Ipv4Addr.fromOctets([255, 255, 255, 255])),
    api.sameIpAddr(api.Ipv6Addr.fromOctets([0xff, 0xff, for (var i = 0; i < 13; i++) 0, 1])),
  ];
  print('lossless ip ${sameIps.map((ip) => hex(ip.octets)).join(' ')}');

  // serde_json's map iterator, collected as pairs of a key and its document.
  final sections = api.sections([('b', api.Json.parse('[1]')), ('a', api.Json.parse('2'))]);
  print('sections ${sections.map((e) => '${e.$1} ${e.$2.toText()}').join(', ')}');
  for (final section in sections) {
    section.$2.close();
  }
  // An instant only Rust makes, which each call copies out of its handle, and compares.
  final began = api.now();
  final ended = api.later(began, api.RustDuration.from(const Duration(milliseconds: 1500)));
  final same = api.later(began, api.RustDuration.zero);
  print(
    'instants ${api.since(ended, began).toDuration().inMilliseconds} '
    '${api.since(began, ended).toDuration().inMilliseconds} ${began == same} ${began == ended}',
  );
  print('has_instant ${api.hasInstant(began)} ${api.hasInstant(null)}');
  // tracing's level, and its filter as an optional level whose null is `OFF`.
  final verbosity = api.Verbosity(api.Level.warn);
  print('verbosity ${api.spell(verbosity.filter())} ${api.spell(api.louder(verbosity.filter()))}');
  verbosity.levelSet(api.Level.trace);
  print('verbosity ${verbosity.levelGet == api.Level.trace} ${api.spell(verbosity.filter())} ${api.spell(api.louder(verbosity.filter()))} ${api.spell(api.louder(null))}');
  verbosity.close();

  // A closure's string, which Rust takes.
  print('made ${api.made(3, (n) => '$n times')}');
  // A closure taking and returning `Copy` values copies them, never running their `Clone`.
  final nine = api.Ticket(9);
  final renewed = api.reissue(api.Ticket(7), (t) => api.Ticket(t.number() + 1)).number();
  final copied = api.reissue(api.Ticket(7), (_) => nine).number();
  print('reissued $renewed $copied, cloned ${api.Ticket.clones()}');
  // A record's traits borrow the checked handle it holds.
  final nines = api.Mark.of(api.U64(9));
  final front = api.seat(nines, 1);
  final back = api.seat(nines, 2);
  print('seat ${front.debug()} ${front == api.seat(nines, 1)} ${front == back}');
  // A kept closure, called on another thread after the call that took it returned.
  final spawned = api.Spawned(() => 7);
  print('spawned ${spawned.join()}');
  // A kept closure: called again at the instant it asks for, then once its waiter wakes,
  // failing with the exception the binding throws for Rust's error.
  final start = api.Moment.new_(api.U64(0), api.U64(1000))!;
  var calls = 0;
  final stopped = await within(
    api.drive(start, (now, waiter) {
      calls++;
      switch (calls) {
        case 1:
          return now.after(api.U64(5));
        case 2:
          print('drive ${now.millis().toInt() - start.millis().toInt()}');
          final waker = waiter.waker();
          waker();
          waker.close();
          return null;
        default:
          throw api.StringException('stopped after $calls');
      }
    }),
    'a kept closure wakes its future',
  );
  print('drive $stopped');
  // A reader the caller implements: dart:io's file, then one whose reads fail.
  final dir = Directory.systemTemp.createTempSync('api');
  final opened = (File('${dir.path}/tail')..writeAsStringSync('hello world')).openSync();
  for (final reader in [api.SeekableReader.file(opened), Broken()]) {
    try {
      final (tail, at) = api.tail(reader, api.U64(5));
      print('tail ${text(tail)} $at');
    } on api.StringException catch (e) {
      print('tail error $e');
    }
  }
  opened.closeSync();
  dir.deleteSync(recursive: true);
  // A seek from the start reaches the reader whole, past `i64::MAX`, as `Cursor` takes one.
  try {
    print('sought ${api.seekTo(Far(), api.U64.max).toRadixString(10)}');
  } on api.StringException catch (e) {
    print('sought error $e');
  }
  print('non zeros ${api.describeNonZeros(api.nonZeros())}');
  for (final hop in [null, api.U64(1), api.U64.max]) {
    final next = api.nextHop(hop);
    print(next == null ? 'hop None' : 'hop Some(${next.toRadixString(10)})');
  }
  print('farther hop ${api.fartherHop(const api.PairNonZeroU16(left: 3, right: 0xFFFF))}');
}

/// A reader whose reads fail, as the transcript's: each seek lands at 0.
/// A reader of nothing, which seeks anywhere from its start, as Rust's `Cursor` does: a
/// `u64` offset is its bits, so -1 is `u64::MAX`.
final class Far implements api.SeekableReader {
  @override
  int readInto(Uint8List buffer) => 0;

  @override
  int seek(int offset, api.SeekFrom from) => offset;
}

final class Broken implements api.SeekableReader {
  @override
  int readInto(Uint8List buffer) => throw const Failure('broken');

  @override
  int seek(int offset, api.SeekFrom from) => 0;
}

/// An I/O failure of a reader, whose text Rust's error holds.
final class Failure implements Exception {
  final String message;

  const Failure(this.message);

  @override
  String toString() => message;
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
  // So does a returned waker, from its wake and from the close that drops it.
  final wakeLog = CountLog();
  final logWaker = api.logWaker(wakeLog);
  logWaker();
  check(wakeLog.lines == 1, 'a wake calls into Dart from a joined thread');
  logWaker.close();
  check(wakeLog.lines == 2, "a returned waker's close calls into Dart from a joined thread");
  await until(() => wakeLog.closes == 1, 'the implementation a closed waker held is closed');
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
  // A future's error borrowing its argument crosses as its message, as `parseCert`'s does.
  check(await api.parseLater('cert') == 4, 'a future with a borrowing error returns Ok');
  try {
    await api.parseLater('');
    check(false, "a future's borrowing error throws");
  } on api.StringException catch (e) {
    check(e.error == 'malformed: ', "a future's borrowing error is its message");
  }
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

  // Closing a handle a call awaiting on it borrows throws, as Rust refuses a drop under a
  // borrow, and changes nothing: the call ends as it would, and then the close passes.
  final other = api.Channel();
  final held = other.recv();
  final sender = other.clone();
  try {
    other.close();
    check(false, 'a close under an awaiting call throws');
  } on StateError catch (e) {
    check(e.message == 'api_Channel_drop: a handle is already borrowed', 'a close under an awaiting call: $e');
  }
  sender.send(utf8.encode('held'));
  check(text(await within(held, 'a receive the close left')) == 'held', 'the awaiting call ends as it would');
  other.close();
  closed(other.clear, 'Channel', 'a call after the close');
  sender.close();

  // A future holding an implementation drops it as it finishes, closing it.
  final log = CountLog();
  check(await api.logLater(log), 'the future returns what its log kept');
  check(log.lines == 1 && log.closes == 1, 'a finished future closes the implementation it held');

  // Dart futures Rust polls, whose completion wakes it.
  var ran = false;
  check(await api.after(Future(() => ran = true), 4) == 8 && ran, 'Rust awaits a Dart future');
  final gate = Completer<void>();
  var after = 0;
  final doubled = api.after(gate.future, 5).then((value) => after = value);
  await step();
  await step();
  check(after == 0, 'Rust waits on a Dart future');
  gate.complete();
  await within(doubled, "a Dart future's completion wakes Rust");
  check(after == 10, "a Dart future's completion wakes Rust");

  // Given up on, a call into Rust passed back as a Dart future is cancelled.
  final kept = api.Channel();
  final recv = kept.recv();
  try {
    await api.within(0, recv);
    check(false, 'Rust gives up on a Dart future');
  } on api.Error catch (e) {
    check(e is api.ErrorOverflow, 'Rust gives up on a Dart future');
  }
  await cancelled(recv, "Rust's drop of a call into Rust it was given cancels it");
  kept.close();

  // A Dart future that fails once Rust dropped it, before the isolate hears of the drop,
  // aborts nothing.
  final late = Completer<void>();
  try {
    await api.within(0, late.future);
    check(false, 'Rust gives up on a Dart future');
  } on api.Error catch (e) {
    check(e is api.ErrorOverflow, 'Rust gives up on a Dart future');
  }
  late.completeError(StateError('failed after Rust dropped it'));
  await step();

  // A poll of a waiter, of a Dart future, which its future owns.
  final gated = Completer<void>();
  var polled = false;
  final polling = api.Waiter.pollFuture(gated.future).then((_) => polled = true);
  await step();
  await step();
  check(!polled, 'a poll of a pending Dart future pends');
  gated.complete();
  await within(polling, "a Dart future's completion wakes the poll of it");
  // Cancelled before it finishes, it drops the future it owns: here a call into Rust,
  // which is cancelled in turn.
  final inner = api.Waiter.pollFuture(Completer<void>().future);
  final outer = api.Waiter.pollFuture(inner);
  await step();
  outer.cancel();
  await cancelled(outer, 'a poll of a Dart future cancels');
  await cancelled(inner, 'a cancelled poll drops the future it owns');
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
  targets();
  print('owned ok');
}

/// `f` throws the UnsupportedError of `what`, which the library's target lacks.
void lacks(void Function() f, String what) {
  try {
    f();
  } on UnsupportedError catch (e) {
    check(e.message == "$what: it is not on this library's target", '$what: $e');
    return;
  }
  check(false, what);
}

/// The bindings serve wasm32 too, whose items the host's library lacks.
void targets() {
  check(api.pointerBits(api.platform()) == 64, "the host's variant crosses");
  check(api.isFamily(api.Family.native), "the host's plain variant crosses");
  lacks(() => api.wasmOnly(), 'api_wasm_only');
  lacks(() => api.pointerBits(const api.PlatformWasm(bits: 32)), 'api_Platform.Wasm');
  lacks(() => api.isFamily(api.Family.wasm), 'api_Family.Wasm');
  lacks(() => api.Page.default_(), 'api_Page_default');
  lacks(() => const api.Page(size: 1).debug(), 'api_Page_debug');
  lacks(() => const api.Page(size: 1) == const api.Page(size: 2), 'api_Page_eq');
  final booth = api.Booth(mark: api.Mark.of(api.U64(1)));
  lacks(() => booth.debug(), 'api_Booth_debug');
  lacks(() => booth == api.Booth(mark: booth.mark), 'api_Booth_eq');
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
  final c = api.Counter('c', api.U64(1));
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
  final registry = api.Registry(api.Counter('first', api.U64(1)));
  registry.close();
  closed(() => registry.firstGet, 'Registry', "a field's getter of a closed object");
  final second = api.Counter('second', api.U64(2));
  closed(() => registry.firstSet(second), 'Registry', "a field's setter of a closed object");
  check(second.get() == 2, 'a call that throws moves nothing in');

  // An argument, alone, or in a list, beside others the call then moves none of.
  closed(() => api.bumpBoth(api.Tally(), tally), 'Tally', 'a closed argument');
  final open = api.Counter('open', api.U64(3));
  closed(() => api.worth([open, c], []), 'Counter', 'a closed object in a list');
  closed(() => api.worth([open], [c]), 'Counter', 'a closed object in a later list');
  closed(() => api.Board([('open', open), ('closed', c)]), 'Counter', 'a closed object in a tuple');
  check(open.get() == 3, 'a call that throws moves none of its other arguments');

  // Nor keeps an implementation beside an argument that throws, which would keep it unclosed
  // and force each later call through a thread of its own: one Dart refuses Rust never
  // takes, and one Rust's check refuses Rust drops, as it takes what it was given first.
  check(api.onFirstThread(), 'a call while Rust holds no implementation');
  final spared = CountLog();
  throws<ArgumentError>(() => api.ArrayU8x4([1, 2, 3]), '3 bytes are no [u8; 4]');
  throws<ArgumentError>(() => api.ArrayU8x4([1, 2, 3, 256]), '256 is no byte');
  throws<RangeError>(() => api.logCounts(spared, 256, open, []), 'an integer out of range after an implementation');
  closed(() => api.logCounts(spared, 0, c, []), 'Counter', 'a closed object after an implementation');
  closed(() => api.logCounts(spared, 0, open, [c]), 'Counter', 'a closed object in a list after an implementation');
  check(spared.closes == 2, 'a call whose check throws drops the implementation it gave Rust');
  check(api.onFirstThread(), "a call after one that threw runs on the isolate's thread");
  lacks(() => api.wasmLog(spared), 'api_wasm_log');
  check(api.onFirstThread(), "a call after one this target lacks runs on the isolate's thread");
  check(api.logCounts(spared, 0, open, []) == 1, 'an implementation a call that threw gave none of');
  check(spared.lines == 1 && spared.closes == 3, 'an implementation given once is closed once');

  // An async call.
  final channel = api.Channel();
  channel.close();
  closed(channel.recv, 'Channel', 'an async call of a closed object');

  // Closed by a call that moved it, plain or boxed, or once the closure it was lent to
  // returned.
  final moved = api.Counter('m', api.U64(0));
  moved.intoLabel(null);
  closed(moved.get, 'Counter', 'a method of a moved object');
  final tail = api.Node(api.U64(1), null);
  api.Node(api.U64(2), tail);
  closed(tail.sum, 'Node', 'a method of a boxed moved object');
  late api.Counter kept;
  api.Registry(api.Counter('c', api.U64(0))).edit((counter) {
    kept = counter;
    return false;
  });
  closed(kept.get, 'Counter', 'a lent object kept past its closure');

  // Closed from the closure a call lends while the call borrows it, an object throws, as
  // Rust refuses a drop under a borrow, and changes nothing; closed after, it closes.
  final editing = api.Registry(api.Counter('e', api.U64(4)));
  final edited = editing.edit((counter) {
    borrowed(editing.close, 'api_Registry_drop: a handle is already borrowed', 'a close in a closure the call lends');
    return counter.get() == 4;
  });
  check(edited, 'the call whose object a close failed on runs on');
  editing.close();
  closed(() => editing.firstGet, 'Registry', 'an object closed after the call');
  // The same for one Dart counts, as the drop of what it holds shows: once, at the close.
  late api.Logger logger;
  late CountLog log;
  log = CountLog(() {
    borrowed(logger.close, 'Logger: a handle is already borrowed', 'a close during a call it is held by');
  });
  logger = api.Logger(log);
  check(logger.write('x'), 'the call whose object a close failed on returns');
  check(log.closes == 0, 'a close that failed frees nothing');
  logger.close();
  check(log.closes == 1, 'a close after the call frees it');
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
  // Read after, so `big` stays reachable and its drop counts in none of forwarded's checks.
  await forwarded();
  check(big[65536] == 1, 'lent bytes read back while reachable');
  print('bytes ok');
}

/// Bytes Rust lent pass back to Rust in place, sent and carried in a record, so a payload
/// read and written on holds Rust's one buffer, freed once with its last owner: a copy
/// anywhere would free the token with the first list.
Future<void> forwarded() async {
  final before = api.dropped();
  final channel = api.Channel();
  Uint8List? held = await relay(channel);
  final canary = [api.Token(9)];
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  canary.clear();
  await collect(() => api.dropped() > before, 'the garbage collector frees an old object');
  // Collections enough to free every list the relay left, had one held a copy.
  for (var i = 0; i < 3; i++) {
    pressure();
    await step();
  }
  check(api.dropped() == before + 1, 'bytes forwarded hold the buffer Rust lent');
  check(held.length == 65537 && held.every((b) => b == 5), 'forwarded bytes read back');
  held = null;
  await collect(() => api.dropped() >= before + 2, 'the garbage collector frees forwarded bytes');
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  check(api.dropped() == before + 2, 'forwarded bytes free once');
  channel.close();
}

/// Lent bytes sent through `channel`, received, and carried on in a packet, whose lists
/// nothing else keeps.
@pragma('vm:never-inline')
Future<Uint8List> relay(api.Channel channel) async {
  send(channel);
  return carry(await channel.recv());
}

@pragma('vm:never-inline')
void send(api.Channel channel) => channel.send(api.tokenBytes(5, 65537));

@pragma('vm:never-inline')
Uint8List carry(Uint8List payload) => api.nextPacket(api.Packet(seq: 1, payload: payload)).payload;

/// A view of returned bytes, whose list nothing else keeps.
@pragma('vm:never-inline')
Uint8List subView(int id) => Uint8List.sublistView(api.tokenBytes(id, 65537), 65527);

/// Values at the edges of what Rust takes and returns, and each value Rust's type lacks,
/// which throws the language's own error rather than reaching Rust.
void bounds() {
  // A lone surrogate is one rune but no Unicode scalar.
  check(api.nextChar('a') == 'b' && api.nextChar('\u{1f600}') == '\u{1f601}', 'a char');
  throws<ArgumentError>(() => api.nextChar(String.fromCharCode(0xd800)), 'a low surrogate');
  throws<ArgumentError>(() => api.nextChar(String.fromCharCode(0xdfff)), 'a high surrogate');
  throws<ArgumentError>(() => api.nextChar('ab'), 'two chars');
  check(api.nextChar('\u{d7ff}') == null, 'the scalar before the surrogates');
  throws<ArgumentError>(() => api.scaled(BigInt.one << 128, api.U64(1)), 'an int past u128');
  throws<ArgumentError>(() => api.scaled(-BigInt.one, api.U64(1)), 'a negative int for a u128');

  // Text is UTF-8 in Rust, which a lone surrogate has no form in: Dart's encoder would
  // replace it.
  check(api.sameString('\u{1f600}é\u{10ffff}') == '\u{1f600}é\u{10ffff}', 'a pair of surrogates');
  for (final lone in ['\u{d800}', 'a\u{dc00}', '\u{dbff}\u{d800}', '\u{dc00}\u{d800}', 'é\u{d800}']) {
    throws<ArgumentError>(() => api.sameString(lone), 'a lone surrogate in ${lone.codeUnits}');
  }
  throws<ArgumentError>(() => api.PathBuf('\u{dc00}'), 'a path of a lone surrogate');
  throws<ArgumentError>(() => api.nextHop(api.U64(0)), 'a zero NonZero');
  throws<ArgumentError>(() => api.fartherHop(const api.PairNonZeroU16(left: 0, right: 1)), 'a zero NonZero field');

  // A `u64` is its bits read unsigned, checked into and out of an `int` or a `BigInt`.
  check(api.U64.max.toBigInt() == (BigInt.one << 64) - BigInt.one, 'u64::MAX as a BigInt');
  check(api.U64.max.toRadixString(10) == '18446744073709551615', 'u64::MAX in decimal');
  check(api.U64.max.toRadixString(16) == 'ffffffffffffffff', 'u64::MAX in hex');
  check(api.U64(0x7fffffffffffffff).toInt() == 0x7fffffffffffffff, 'i64::MAX as an int');
  check(api.U64(1) < api.U64.max && api.U64.max > api.U64(0x7fffffffffffffff), 'a u64 compares unsigned');
  check(api.U64.fromBigInt(BigInt.one << 63).bits == -0x8000000000000000, 'i64::MAX + 1 as its bits');
  throws<RangeError>(() => api.U64(-1), 'a negative int for a u64');
  throws<RangeError>(() => api.U64.fromBigInt(BigInt.one << 64), 'a BigInt past u64');
  throws<RangeError>(() => api.U64.fromBigInt(-BigInt.one), 'a negative BigInt for a u64');
  throws<RangeError>(() => api.U64.max.toInt(), 'a u64 past int');

  // A Rust Duration is its seconds and nanoseconds, checked into and out of a Duration,
  // whose longest is 9223372036854 seconds and 775807 microseconds.
  const longest = Duration(microseconds: 0x7fffffffffffffff);
  for (final d in [Duration.zero, const Duration(microseconds: 9223372036854000000), longest]) {
    final same = api.extended(api.RustDuration.from(d), api.RustDuration.zero)!;
    check(same.toDuration() == d, 'a Duration crosses whole, $d');
  }
  check(api.extended(api.RustDuration(api.U64.max, 999999999), api.RustDuration(api.U64(0), 1)) == null, 'past the longest');
  throws<RangeError>(() => api.RustDuration(api.U64(0), 1000000000), 'a second of nanoseconds');
  throws<RangeError>(() => api.RustDuration(api.U64(0), -1), 'negative nanoseconds');
  throws<ArgumentError>(() => api.RustDuration.from(const Duration(microseconds: -1)), 'a negative Duration');
  throws<RangeError>(() => api.RustDuration(api.U64(0), 1).toDuration(), 'a nanosecond as a Duration');
  throws<RangeError>(() => api.RustDuration(api.U64(9223372036854), 775808000).toDuration(), 'past the longest Duration');
  check(api.RustDuration(api.U64(1)).compareTo(api.RustDuration(api.U64.max)) < 0, 'a Duration compares unsigned');

  // A path is its bytes, which a String may not hold.
  final stray = api.stray();
  check(api.samePath(stray) == stray && stray.bytes.single == 0xff, 'a path that is not UTF-8');
  check('$stray' == '\u{fffd}', "a path's display replaces what is not UTF-8");
  throws<FormatException>(stray.toFilePath, 'a path that is not UTF-8 as a String');
  api.PathBuf? lent;
  api.visitStray((path) => lent = path);
  check(lent == stray, 'a lent path that is not UTF-8');
  check(api.PathBuf('a/é').toFilePath() == 'a/é', 'a UTF-8 path as a String');
  final given = [0xff];
  final made = api.PathBuf.fromBytes(given);
  given[0] = 0;
  check(made == stray && api.samePath(made) == stray, "a caller's bytes make a path, copied");
  throws<ArgumentError>(() => api.PathBuf.fromBytes([0x100]), '256 is no path byte');
  throws<ArgumentError>(() => api.PathBuf.fromBytes([-1]), '-1 is no path byte');

  // An address is its family's, checked out of an InternetAddress.
  final loopback = InternetAddress('::1');
  check(api.IpAddr.from(loopback).toInternetAddress() == loopback, 'an IPv6 address both ways');
  check(api.IpAddr.from(InternetAddress('10.0.0.1')) is api.Ipv4Addr, 'an IPv4 address');
  throws<ArgumentError>(() => api.Ipv4Addr.from(loopback), 'an IPv6 address for an Ipv4Addr');
  throws<ArgumentError>(() => api.Ipv6Addr.from(InternetAddress('10.0.0.1')), 'an IPv4 address for an Ipv6Addr');
  throws<ArgumentError>(() => api.Ipv6Addr.from(InternetAddress('fe80::1%1')), 'an IPv6 address with a scope');
  throws<ArgumentError>(
    () => api.IpAddr.from(InternetAddress('/tmp/api', type: InternetAddressType.unix)),
    'a Unix address',
  );
  throws<ArgumentError>(() => api.Ipv4Addr.fromOctets([1, 2, 3]), 'three octets');
  throws<ArgumentError>(() => api.Ipv4Addr.fromOctets([256, 0, 0, 0]), 'an octet past 255');
  final ip = api.Ipv6Addr.from(loopback);
  check(api.ports([api.SocketAddrV6(ip, 65535)]).single == 65535, 'the last port');
  throws<RangeError>(() => api.SocketAddrV6(ip, 65536), 'a port past u16');
  throws<RangeError>(() => api.SocketAddrV6(ip, 1, scopeId: -1), 'a negative scope');
  throws<RangeError>(() => api.SocketAddrV6(ip, 1, flowinfo: 0x100000000), 'a flow label past u32');

  // A key hashes by Rust's Hash, so a Set holds each once.
  check({api.Key('a'), api.Key('a'), api.Key('b')}.length == 2, 'a set of keys');

  // A block crosses whole: an empty one stays apart from none, and a u64 keeps its bits.
  api.Numbers blocks(Int32List? some) => api.Numbers(
    u16s: Uint16List(0),
    u32s: Uint32List(0),
    u64s: [api.U64.max],
    i8s: Int8List(0),
    i16s: Int16List(0),
    i32s: Int32List(0),
    i64s: Int64List(0),
    f32s: Float32List(0),
    f64s: Float64List(0),
    some: some,
  );
  check(blocks(null).reversed().some == null, 'no block crosses as none');
  check(blocks(Int32List(0)).reversed().some?.isEmpty == true, 'an empty block crosses as itself');
  check(blocks(null).reversed().u64s.single == api.U64.max, 'u64::MAX crosses whole');
  final big = Uint32List.fromList(List.generate(1000, (i) => i));
  check(api.sumU32(big) == 499500, 'a thousand u32s cross whole');
  print('ok');
}

/// A reader whose seek throws an Error, which no reader may.
final class Thrower implements api.SeekableReader {
  @override
  int readInto(Uint8List buffer) => 0;

  @override
  int seek(int offset, api.SeekFrom from) => throw StateError('a seek failed');
}

/// Each breaks a contract, which must fail with a message naming it.
void broken(String which) {
  if (Platform.isLinux) {
    // Non-dumpable, an expected abort writes no core and never starts the host's crash
    // handler. PR_SET_DUMPABLE is 4.
    final prctl = ffi.DynamicLibrary.process().lookupFunction<
        ffi.Int Function(ffi.Int,
            ffi.VarArgs<(ffi.UnsignedLong, ffi.UnsignedLong, ffi.UnsignedLong, ffi.UnsignedLong)>),
        int Function(int, int, int, int, int)>('prctl');
    check(prctl(4, 0, 0, 0, 0) == 0, 'prctl');
  }
  switch (which) {
    case 'moved':
      final c = api.Counter('c', api.U64(0));
      c.intoLabel(null);
      c.get();
    case 'boxed moved':
      final tail = api.Node(api.U64(1), null);
      api.Node(api.U64(2), tail);
      tail.sum();
    case 'outlived':
      final meters = api.Meters();
      final first = meters.first();
      meters.clear();
      first.count();
    case 'lent moved':
      api.Registry(api.Counter('c', api.U64(0))).edit((counter) {
        counter.intoLabel(null);
        return false;
      });
    case 'lent kept':
      late api.Counter kept;
      api.Registry(api.Counter('c', api.U64(0))).edit((counter) {
        kept = counter;
        return false;
      });
      kept.get();
    case 'callback threw':
      api.emit(CountLog(() => throw StateError('a log failed')), bytes([1]));
    // Only the exception of Rust's error fails a closure.
    case 'closure threw':
      api.drive(api.Moment.new_(api.U64(0), api.U64(1000))!, (_, _) => throw StateError('a poll failed'));
    // Only an Exception fails a reader.
    case 'reader threw':
      api.tail(Thrower(), api.U64(1));
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
