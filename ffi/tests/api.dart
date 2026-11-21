// The api fixture's Dart test: the script fixtures/api/examples/transcript.rs runs
// natively, through the Dart tests/fixtures/transcript writes from it, which calls the
// functions here that transcript.rs defines. tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks what the script says only in Rust, the futures'
// semantics, what closing and the garbage collector free, what a closed object refuses,
// how returned bytes cross, or breaks a contract, which must fail.

import 'dart:async';
import 'dart:collection';
import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:api/api.dart' as api;
import 'package:transcript/transcript.dart' as transcript;

Uint8List bytes(List<int> values) => Uint8List.fromList(values);

String text(Uint8List data) => utf8.decode(data);

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

/// The Log callback: keeps even levels.
final class Print extends api.Log {
  @override
  bool log(int level, String line) {
    print('log $level $line');
    return level % 2 == 0;
  }

  // Rust closes it once it drops the implementation.
  @override
  void close() => print('freed');
}

/// An eraser whose methods differ only by a trailing underscore.
final class Eraser extends api.ClashEraser {
  @override
  int delete_(int new_, int new_2) => new_ * 10 + new_2;

  @override
  int delete() => 3;
}

/// The Undo callback: each method returns its own digit.
final class Edits extends api.Undo {

  @override
  int delete(int n) => n - 1;

  @override
  int free() => 2;

  @override
  int type() => 3;
}

/// Bytes in hex, two digits each.
String _hex(List<int> bytes) => bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join();

String hex(api.ArrayU8x4 bytes) => _hex(bytes.toBytes());

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

String pathHex(api.PathBuf path) => _hex(path.bytes);

/// A duration as its seconds and nine digits of nanoseconds.
String seconds(api.RustDuration d) => '${d.secs.toRadixString(10)}.${d.nanos.toString().padLeft(9, '0')}';

/// A duration in whole milliseconds, cut to a `u64` as Rust's `as` does.
api.U64 millis(api.RustDuration d) =>
    api.U64.fromBigInt((d.secs.toBigInt() * BigInt.from(1000) + BigInt.from(d.nanos ~/ 1000000)).toUnsigned(64));

/// A char's code point.
int code(String c) => c.runes.single;

List<T> reversed<T>(List<T> list) => list.reversed.toList();

/// Each float's bits.
List<api.U64> f64Bits(Float64List floats) =>
    floats.buffer.asUint64List(floats.offsetInBytes, floats.length).map(api.U64.fromBits).toList();

/// A socket address as its IP's bytes in hex, its port, flow label, and scope, each
/// IPv6's alone, apart by colons.
String socketHex(api.SocketAddr addr) => switch (addr) {
  api.SocketAddrV4(:final ip, :final port) => '${_hex(ip.octets)}:$port',
  api.SocketAddrV6(:final ip, :final port, :final flowinfo, :final scopeId) =>
    '${_hex(ip.octets)}:$port:$flowinfo:$scopeId',
};

/// An address's bytes in hex, so the length shows its family.
String ipHex(api.IpAddr ip) => _hex(ip.octets);

/// A path no Rust string spells: bytes that are not UTF-8, or on Windows, WTF-8's lone
/// surrogate.
api.PathBuf unspelled() => Platform.isWindows ? api.PathBuf.fromBytes([0xed, 0xa0, 0x80]) : api.stray();

/// Print each of `n`'s lists, its floats as their bits.
Future<void> numbers(String what, api.Numbers n) async {
  String bits32(Float32List v) => Uint32List.fromList(v.buffer.asUint32List(v.offsetInBytes, v.length)).toString();
  String digits(Iterable<api.U64> v) => '[${v.map((v) => v.toRadixString(10)).join(', ')}]';
  print('$what u16 ${n.u16s}');
  print('$what u32 ${n.u32s}');
  print('$what u64 ${digits(n.u64s)}');
  print('$what i8 ${n.i8s}');
  print('$what i16 ${n.i16s}');
  print('$what i32 ${n.i32s}');
  print('$what i64 ${n.i64s}');
  print('$what f32 ${bits32(n.f32s)}');
  print('$what f64 ${digits(f64Bits(n.f64s))}');
  final some = n.some;
  print('$what some ${some == null ? 'None' : 'Some($some)'}');
}

/// A channel's futures, on the isolate's event loop.
Future<void> channel() async {
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
}

/// A kept closure: called again at the instant it asks for, then once its waiter wakes,
/// failing with the exception the binding throws for Rust's error.
Future<void> drive() async {
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
}

/// Readers the caller implements: dart:io's file, one whose reads fail, and one that seeks
/// past `i64::MAX`.
Future<void> readers() async {
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
}

/// Streams, each iterated as a Dart stream: to its end, left early, and to an error, past
/// which the stream goes on.
Future<void> streams() async {
  final counts = <int>[];
  final countdown = api.Countdown(3);
  await for (final count in countdown.asStream()) {
    counts.add(count);
  }
  print('countdown ${counts.join(' ')}');
  countdown.close();
  // A loop left early takes nothing past the item it stopped at.
  final rest = api.Countdown(5);
  await for (final count in rest.asStream()) {
    if (count == 4) break;
  }
  print('countdown after break ${await rest.next()}');
  rest.close();
  // Each payload sends the next, which `!` fails and an empty one ends.
  final channel = api.Channel();
  final feed = channel.feed();
  for (final first in ['a', 'c']) {
    channel.send(utf8.encode(first));
    try {
      await for (final payload in feed.asStream()) {
        print('feed ${text(payload)}');
        channel.send(utf8.encode(switch (text(payload)) {
          'a' => 'b',
          'b' => '!',
          _ => '',
        }));
      }
      print('feed end');
    } on api.Error catch (e) {
      print('feed error $e');
    }
  }
  feed.close();
  channel.close();
}

/// A stream's semantics beyond the transcript: what a cancel or pause leaves of its pending
/// `next`, which the stream holds exclusively, so a direct `next` throws while one pends.
Future<void> streamChecks() async {
  final channel = api.Channel();
  final feed = channel.feed();
  final got = <String>[];

  // Cancelled while its `next` waits, it drops that call: the payload sent after is the
  // next call's.
  final cancelled = feed.asStream().listen((payload) => got.add(text(payload)));
  await step();
  borrowed(() => feed.next(), 'api_Feed_next: a handle is already borrowed', 'a stream pends one next');
  await cancelled.cancel();
  channel.send(utf8.encode('after cancel'));
  await step();
  check(got.isEmpty, 'a cancelled stream adds nothing');
  check(text((await within(feed.next(), 'a next after a cancel'))!) == 'after cancel', 'a cancelled stream loses nothing');

  // Paused, it drops its pending call and pulls nothing until it resumes.
  final paused = feed.asStream().listen((payload) => got.add(text(payload)));
  await step();
  paused.pause();
  channel.send(utf8.encode('while paused'));
  await step();
  check(got.isEmpty, 'a paused stream adds nothing');
  check(text((await within(feed.next(), 'a next while paused'))!) == 'while paused', 'a paused stream pulls nothing');
  channel.send(utf8.encode('resumed'));
  paused.resume();
  await until(() => got.length == 1, 'a resumed stream pulls again');
  check(got.single == 'resumed', 'a resumed stream adds what it pulls');
  await paused.cancel();
  got.clear();

  // A cancel or pause in the listener's handler, of an item ready at once, lands before
  // the next call: nothing pends after it.
  for (final stop in ['cancel', 'pause']) {
    channel.send(utf8.encode(stop));
    late final StreamSubscription<Uint8List> sub;
    sub = feed.asStream().listen((payload) {
      got.add(text(payload));
      stop == 'cancel' ? sub.cancel().ignore() : sub.pause();
    });
    await until(() => got.length == 1, 'a stream adds an item ready at once');
    channel.send(utf8.encode('after $stop'));
    await step();
    check(text((await within(feed.next(), 'a next after a $stop'))!) == 'after $stop', 'a $stop in the handler pulls nothing');
    await sub.cancel();
    got.clear();
  }

  // A pause from a callback Rust runs in the poll that finishes the call keeps its item
  // for the resume.
  final counts = <int>[];
  var told = false;
  late final StreamSubscription<int> tellingSub;
  final telling = api.Countdown.told(
    3,
    CountLog(() {
      if (!told) tellingSub.pause();
      told = true;
    }),
  );
  tellingSub = telling.asStream().listen(counts.add);
  await until(() => told, 'a told countdown tells its count');
  await step();
  check(tellingSub.isPaused && counts.isEmpty, 'a stream paused in its poll adds nothing');
  tellingSub.resume();
  await until(() => counts.length == 3, 'a stream paused in its poll resumes');
  check(counts.join(' ') == '3 2 1', 'a stream paused in its poll keeps its item: $counts');
  await tellingSub.cancel();
  telling.close();

  // An error is the last event; a check refusing the call is one too.
  channel.send(utf8.encode('!'));
  final events = <Object>[];
  final done = Completer<void>();
  feed.asStream().listen(
    (payload) => events.add(text(payload)),
    onError: (Object e) => events.add(e),
    onDone: done.complete,
  );
  await within(done.future, 'a stream that fails ends');
  check(events.length == 1 && events.single is api.ErrorOverflow, 'an error ends a stream: $events');
  feed.close();
  try {
    await feed.asStream().first;
    check(false, 'a closed stream fails');
  } on StateError catch (e) {
    check(e.message == 'Feed is closed' || e.message.endsWith(": a handle's value was moved out"), 'a closed stream fails with its check: $e');
  }
  channel.close();
}

/// Futures of a value that is not `Send`, awaited on the isolate's thread.
Future<void> localLater(api.Local local) async {
  print('local later ${(await api.localOrNewLater(local)).bump()}');
  print('local later fresh ${(await api.localOrNewLater(null)).bump()}');
}

/// What the transcript's script says only in Rust: Dart's own errors.
void checks() {
  // A draft borrows its ledger exclusively until it closes: the ledger refuses any other
  // use meanwhile.
  final ledger = api.Ledger();
  final draft = ledger.edit();
  try {
    ledger.published();
    check(false, 'a lent ledger refuses a read');
  } on StateError catch (e) {
    check(e.message == 'api_Ledger_published: a handle is borrowed exclusively', 'a lent ledger: $e');
  }
  draft.close();
  // Each counter moves in, so a later call throws.
  final counters = api.counters(['x', 'y']);
  api.worth([counters[0]], [counters[1]]);
  for (final counter in counters) {
    closed(counter.get, 'Counter', 'a counter moved in is closed');
  }
  // A kept closure, called on another thread after the call that took it returned.
  check(api.Spawned(() => 7).join() == 7, 'a kept closure runs on another thread');
  // Names apart from what Dart gives every enum and what a trait gives a type, each
  // reaching its own Rust.
  check(api.ClashLevel.default_() == api.ClashLevel.default__, 'a value named as Default\'s');
  check(api.ClashLevel.debug_.debug() == 'Debug', 'a value named as Debug\'s');
  check(api.ClashLevel.debug_.index_() == 2 && api.ClashLevel.debug_.index == 1, 'a method named as an enum\'s index');
  final tag = api.ClashTagPrivate(number: 3);
  check(tag.number == 3 && tag.number_() == 13, 'a method named as its variants\' field');
  // Names that differ only by trailing underscores, each reaching its own Rust: a name Dart
  // spells as Rust does keeps it, whichever comes first.
  final underscored = api.ClashUnderscored(new_: 1, new2_: 2, debug_: 3, default_: 4);
  check(
    api.clashUnderscored(underscored, api.ClashOp.delete, api.ClashChangeSet(new_: 5, new2_: 6)) == 2561234,
    'names apart by trailing underscores cross',
  );
  check(
    api.clashUnderscoredNext(underscored, api.ClashChangeSet(new_: 5, new2_: 6)) ==
        (api.ClashUnderscored(new_: 2, new2_: 3, debug_: 4, default_: 5), api.ClashChangeSet(new_: 6, new2_: 5)),
    'names apart by trailing underscores cross back',
  );
  check(api.clashErased(Eraser()) == 123, 'a callback\'s methods apart by a trailing underscore');
  check(
    api.ClashEscaped(new_: 3).delete() == 3 && api.ClashEscaped(new_: 3).delete_(1, 2) == 213,
    'methods apart by a trailing underscore',
  );
  check(api.clashErase_() == 2 && api.clashErase() == 1, 'functions apart by a trailing underscore');
  print('checks ok');
}

/// A reader of nothing, which seeks anywhere from its start, as Rust's `Cursor` does: a
/// `u64` offset is its bits, so -1 is `u64::MAX`.
final class Far implements api.SeekableReader {
  @override
  int readInto(Uint8List buffer) => 0;

  @override
  int seek(int offset, api.SeekFrom from) => offset;
}

/// A reader whose reads fail, as the transcript's: each seek lands at 0.
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

/// A [CountLog] whose close, which Rust's drop of it runs, then runs `closed`.
final class ClosingLog extends CountLog {
  final void Function() closed;

  ClosingLog(this.closed);

  @override
  void close() {
    super.close();
    closed();
  }
}

/// A list of objects whose length, read as a call lowers it, first runs `read`.
final class ReadingList extends ListBase<api.Counter> {
  final void Function() read;

  ReadingList(this.read);

  @override
  int get length {
    read();
    return 0;
  }

  @override
  set length(int n) => throw UnsupportedError('a fixed list');

  @override
  api.Counter operator [](int i) => throw RangeError.index(i, this);

  @override
  void operator []=(int i, api.Counter value) => throw UnsupportedError('a fixed list');
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

  // A value that is not Send lives on a home, a thread Rust owns, which runs each call on
  // it: the isolate's thread may change between turns of its event loop, and a call runs
  // on a worker while Rust holds an implementation. A close drops it there.
  final local = api.Local();
  final shared = local.share();
  local.bump();
  check(shared.bump() == 2, 'a shared local counts together');
  check(await local.later() == 3, "a local's future runs on its home");
  check(await api.localLater(5) == 10, 'a future that is not Send runs on a home');
  await step();
  check(local.onOwnThread(), 'a call after a suspension runs on its home');
  final relayLog = CountLog();
  final relayed = api.logOnDrop(relayLog);
  check(local.bump() == 4 && local.onOwnThread(), 'a call through the relay runs on its home');
  check(local.visit(() => local.bump()) == 5, 'a callback on a home calls back into its local');
  relayed.cancel();
  await cancelled(relayed, 'a cancelled future that held a relay completes with Cancelled');
  final localsBefore = api.localsDropped();
  local.close();
  check(api.localsDropped() == localsBefore + 1, 'a closed local drops on its home');
  shared.close();

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
  await streamChecks();
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

  // Freed only once later returns push it out of the nursery.
  unclosed();
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  check(api.dropped() == before + 1, 'the nursery keeps what nobody closed');
  fill();
  await collect(() => api.dropped() >= before + 2, 'the garbage collector frees what nobody closed');
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  check(api.dropped() == before + 2, 'the garbage collector frees each once');

  // Closed once pushed out of the nursery, which gave the garbage collector its hold: the
  // close takes it back, so the collector frees nothing more.
  final kept = api.Token(4);
  fill();
  kept.close();
  check(api.dropped() == before + 3, 'closing an object pushed out of the nursery frees it at once');
  for (var i = 0; i < 3; i++) {
    pressure();
  }
  await step();
  check(api.dropped() == before + 3, 'the garbage collector frees nothing a close freed');

  // An isolate's nursery frees what it still holds once the isolate exits.
  final exited = api.dropped();
  await Isolate.run(unclosed);
  await collect(() => api.dropped() > exited, "an isolate's exit frees what its nursery held");

  // An implementation an object the garbage collector frees held is closed from the event
  // loop, as no drop in the collector may call into Dart.
  final log = CountLog();
  unclosedLogger(log);
  fill();
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

/// Return a nursery's worth of objects, each closed, pushing out every earlier one.
void fill() {
  for (var i = 0; i < 64; i++) {
    api.Tally().close();
  }
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
  check(spared.closes == 0, 'a call Dart refuses gives Rust no implementation');
  // Moved out by a call, which only Rust's check sees.
  final gone = api.Counter('g', api.U64(0));
  gone.intoLabel(null);
  closed(() => api.logCounts(spared, 0, gone, []), 'Counter', 'a moved object after an implementation');
  closed(() => api.logCounts(spared, 0, open, [gone]), 'Counter', 'a moved object in a list after an implementation');
  check(spared.closes == 2, 'a call whose check throws drops the implementation it gave Rust');
  check(api.onFirstThread(), "a call after one that threw runs on the isolate's thread");
  lacks(() => api.wasmLog(spared), 'api_wasm_log');
  check(api.onFirstThread(), "a call after one this target lacks runs on the isolate's thread");
  check(api.logCounts(spared, 0, open, []) == 1, 'an implementation a call that threw gave none of');
  check(spared.lines == 1 && spared.closes == 3, 'an implementation given once is closed once');

  // Closed by Dart a call runs as it lowers its arguments, after it held the object: the
  // handle outlives the call's hold, so Rust's check refuses the value the close dropped.
  final early = api.Counter('early', api.U64(1));
  closed(
    () => api.worth([early], ReadingList(early.close)),
    'Counter',
    'an object closed while its call lowers the rest',
  );
  closed(early.get, 'Counter', 'an object closed during a call, after it');

  // Closed again by Dart its drop runs, which finds it closed, so it is freed once.
  late api.Journal journal;
  var during = '';
  final nested = ClosingLog(() {
    try {
      journal.write('x');
    } on StateError catch (e) {
      during = e.message;
    }
    journal.close();
  });
  journal = api.Journal(nested);
  journal.close();
  check(nested.closes == 1, 'a close from inside its own drop closes nothing more');
  check(during == 'Journal is closed', 'an object is closed in Dart while Rust drops it: $during');

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
  fill();
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
  fill();
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

/// Calls proven to reach no user code are leaf calls on the isolate's thread; one that only
/// looks like them is not, which a hook Rust calls from another thread proves: run in a leaf
/// call, it would wait on the isolate's thread, which waits on the call.
Future<void> leaf() async {
  final plain = api.Plain(api.U64(3), true);
  plain.nSet(api.U64(4));
  check(plain.nGet == api.U64(4) && plain.onGet, 'a leaf getter reads what a leaf setter wrote');
  check(plain == api.Plain(api.U64(4), true) && plain != api.Plain(api.U64(4), false), 'a leaf equality');
  check(plain.hashCode == api.Plain(api.U64(4), true).hashCode, 'a leaf hash');
  plain.recount(api.U64(5));
  check(plain.count() == api.U64(5), 'a leaf method reads what a leaf method wrote');
  check(plain.steps() == api.U64(5), 'a method that loops');
  final wrapped = api.Wrapped(api.U64(3));
  wrapped.nSet(api.U64(4));
  check(wrapped.nGet == api.U64(4) && wrapped.count() == api.U64(4), 'a leaf through a Deref');
  wrapped.close();
  // A walk of a string the caller sized, which a leaf call would make the collector wait on.
  final labeled = api.Labeled('a' * 100);
  check(labeled == api.Labeled('a' * 100) && labeled != api.Labeled('b'), 'a served equality over a string');
  check(labeled.hashCode == api.Labeled('a' * 100).hashCode, 'a served hash over a string');
  labeled.close();
  final buffer = api.Buffer(100);
  buffer.blank();
  check(buffer.zeroed(), 'a fill the caller sized');
  buffer.close();
  plain.close();
  throws<StateError>(() => plain.nGet, 'a leaf getter of a closed object throws');

  var hooked = 0;
  api.hook(() => hooked++);
  // A derived equality over a field whose own is written by hand.
  check(api.Hooked(api.U64(1)) == api.Hooked(api.U64(1)), 'a hooked equality');
  check(hooked == 1, 'a hooked equality serves the isolate');
  // A hand-written equality that claims to be derived.
  check(api.Forged(api.U64(1)) == api.Forged(api.U64(1)), 'a forged equality');
  check(hooked == 2, 'a forged equality serves the isolate');
  // A value kept on its thread lives on a home, whose getter and drop run there.
  final pinned = api.Pinned(api.U64(5));
  check(pinned.valueGet == api.U64(5), 'a getter of a value kept on its thread');
  pinned.close();
  check(hooked == 3, 'a drop on a home serves the isolate');
  check(api.Hooked(api.U64(1)).count() == api.U64(1), 'a hooked method');
  check(hooked == 4, 'a hooked method serves the isolate');
  final guarded = api.Guarded(api.U64(2));
  check(guarded.nGet == api.U64(2) && guarded.count() == api.U64(2), 'a hooked Deref');
  check(hooked == 6, 'a hooked Deref serves the isolate');
  guarded.close();
  api.unhook();
  print('leaf ok');
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
      await transcript.script();
    case ['checks']:
      checks();
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
    case ['leaf']:
      await leaf();
    case [final which]:
      broken(which);
    default:
      check(false, 'no case ${args.join(' ')}');
  }
}
