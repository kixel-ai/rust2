// Runs scripts tests/protocol/model.rs writes through the api fixture's Dart, each step on
// the isolate, writing what it returned a line to the script's path with `.out` added.
// Each script's transcript from the model must match it line for line.
//
// Steps run without yielding to the event loop but to await a future, so a future the
// step that made it left pending holds its borrows until the step awaiting it.

import 'dart:io';

import 'package:api/api.dart' as api;

/// What a refused check's message ends with, after the function's name, by the name the
/// model prints.
const statuses = {
  "a handle's value was moved out": 'moved',
  'a handle is borrowed exclusively': 'exclusive',
  'a handle is already borrowed': 'borrowed',
  'a handle is borrowed on another thread': 'thread',
  'a handle a call lent is borrowed exclusively or moved out': 'lent',
  'a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed': 'outlived',
  'a value that is not Send is used on another thread than the one that made it': 'not_send',
};

/// The line for what a step threw: Rust's `Err`, a refused check, or a closed handle, which
/// a moved-out one is too.
String failure(Object e) {
  if (e is api.Error) return 'error';
  if (e is StateError) {
    if (e.message.endsWith(' is closed')) return 'moved';
    for (final MapEntry(:key, :value) in statuses.entries) {
      if (e.message.endsWith(': $key')) return value;
    }
  }
  throw e;
}

final class Script {
  final List<Object?> slots;
  final List<String> steps;

  Script(List<String> lines)
    : slots = List.filled(lines.first.split(' ').length, null),
      steps = lines.skip(1).where((line) => line.isNotEmpty).toList();

  T at<T>(String h) => slots[int.parse(h)] as T;

  api.Counter? opt(String h) => h == '-' ? null : at<api.Counter>(h);

  String made(String h, Object value) {
    slots[int.parse(h)] = value;
    return 'ok';
  }

  String u64(api.U64 v) => v.toBigInt().toString();

  /// A ledger's edit into its slot: true, as the model prints it.
  String edit(List<String> a) {
    made(a[0], at<api.Ledger>(a[1]).edit());
    return '1';
  }

  /// Run a call returning nothing.
  String ok(void Function() call) {
    call();
    return 'ok';
  }

  /// Run one step: what it returned.
  Future<String> run(String op, List<String> a) async {
    try {
      return switch (op) {
        'counter_new' => made(a[0], api.Counter('c', api.U64(int.parse(a[1])))),
        'counter_get' => u64(at<api.Counter>(a[0]).get()),
        'counter_add' => u64(at<api.Counter>(a[0]).add(api.U64(int.parse(a[1])))),
        'counter_into_sum' => u64(at<api.Counter>(a[0]).intoSum(api.U64(int.parse(a[1])))),
        'counter_plus' => u64(at<api.Counter>(a[0]).plus(opt(a[1]))),
        'counter_add_to' => at<api.Counter>(a[0]).addTo(opt(a[1])) ? '1' : '0',
        'counter_clone' => made(a[0], at<api.Counter>(a[1]).clone()),
        'plus_later' => made(a[0], at<api.Counter>(a[1]).plusLater(opt(a[2])).then(u64)),
        'ledger_new' => made(a[0], api.Ledger()),
        'ledger_edit' => edit(a),
        'ledger_published' => '${at<api.Ledger>(a[0]).published()}',
        'ledger_seal' => ok(() => at<api.Ledger>(a[0]).seal()),
        'ledger_receipt' => made(a[0], at<api.Ledger>(a[1]).receipt()),
        'draft_write' => ok(() => at<api.Draft>(a[0]).write('line')),
        'draft_lines' => '${at<api.Draft>(a[0]).lines()}',
        'draft_commit' => '${at<api.Draft>(a[0]).commit()}',
        'tallied' => '${api.tallied(at<api.Receipt>(a[0]))}',
        'stored' => '${api.stored(at<api.Receipt>(a[0]))}',
        'receipt_count' => '${at<api.Receipt>(a[0]).count()}',
        'receipt_clone' => made(a[0], at<api.Receipt>(a[1]).clone()),
        'tally_new' => made(a[0], api.Tally()),
        'tally_bump' => '${at<api.Tally>(a[0]).bump()}',
        'bump_later' => made(a[0], at<api.Tally>(a[1]).bumpLater().then((v) => '$v')),
        // Every local is on a home, as Dart makes each value that is not Send.
        'local_new' => made(a[0], api.Local()),
        'local_bump' => '${at<api.Local>(a[0]).bump()}',
        'local_share' => made(a[0], at<api.Local>(a[1]).share()),
        'local_later' => made(a[0], at<api.Local>(a[1]).later().then((v) => '$v')),
        'meters_new' => made(a[0], api.Meters()),
        'meters_first' => made(a[0], at<api.Meters>(a[1]).first()),
        'meters_clear' => ok(() => at<api.Meters>(a[0]).clear()),
        'meter_new' => made(a[0], api.Meter.default_()),
        'meter_add' => u64(at<api.Meter>(a[0]).add(api.U64(int.parse(a[1])))),
        'meter_reset' => ok(() => at<api.Meter>(a[0]).reset()),
        'drop' => ok(() => (slots[int.parse(a[0])] as dynamic).close()),
        'await' => await at<Future<String>>(a[0]),
        _ => throw UnsupportedError('an op Dart does not run: $op'),
      };
    } catch (e) {
      return failure(e);
    }
  }

  Future<String> runAll() async {
    final out = StringBuffer();
    for (final line in steps) {
      final words = line.split(' ');
      if (words[0] != '0') throw UnsupportedError('Dart runs every step on its isolate');
      out.writeln(await run(words[1], words.sublist(2)));
    }
    return out.toString();
  }
}

Future<void> main(List<String> paths) async {
  for (final path in paths) {
    File('$path.out').writeAsStringSync(await Script(File(path).readAsLinesSync()).runAll());
  }
}
