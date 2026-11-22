// The api fixture's Dart items only wasm32 has, which the bindings serve too: on the host's
// library each throws the UnsupportedError of its target. Apart from api.dart, which compiles
// against the release builder's export too, whose targets have no wasm32.

import 'dart:io';

import 'package:api/api.dart' as api;

void check(bool ok, String what) {
  if (!ok) {
    stderr.writeln('failed: $what');
    exit(1);
  }
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

final class Log extends api.Log {
  int closes = 0;

  @override
  bool log(int level, String line) => true;

  @override
  void close() => closes++;
}

void main() {
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

  // An implementation a call this target lacks was given is not kept, nor does it keep
  // later calls off the isolate's thread.
  final log = Log();
  check(api.onFirstThread(), 'a call while Rust holds no implementation');
  lacks(() => api.wasmLog(log), 'api_wasm_log');
  check(api.onFirstThread(), "a call after one this target lacks runs on the isolate's thread");
  check(log.closes == 0, 'a call this target lacks gives Rust no implementation');
  print('targets ok');
}
