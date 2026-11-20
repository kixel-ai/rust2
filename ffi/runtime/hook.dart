// The build hook of a Dart package rust2-ffi exports, written by it. It bundles the
// package's Rust library as the native asset its `@Native` functions find. A release's
// hook/release.json names each target's library by URL, size, and SHA-256, which this
// downloads; an export holds none, so this builds the crate rust2-ffi exported the package
// beside, ../Cargo.toml, with Cargo.

import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:code_assets/code_assets.dart';
import 'package:crypto/crypto.dart';
import 'package:hooks/hooks.dart';

Future<void> main(List<String> args) async {
  await build(args, (input, output) async {
    if (!input.config.buildCodeAssets) return;
    final code = input.config.code;
    final package = input.packageName;
    // Dart links no static library yet, so a build that takes only one has no use for this.
    if (code.linkModePreference == LinkModePreference.static) {
      throw UnsupportedError('$package ships a dynamic library, which a static-only build cannot take');
    }
    final target = _target(code);
    final release = File.fromUri(input.packageRoot.resolve('hook/release.json'));
    final File library;
    if (release.existsSync()) {
      library = await _download(input, code, release, target);
      output.dependencies.add(release.uri);
    } else {
      final (built, inputs) = await _build(input, code, target);
      library = built;
      output.dependencies.addAll(inputs);
    }
    // A copy, as Cargo may rebuild its own and the shared cache serves other builds.
    final bundled = input.outputDirectory.resolve(code.targetOS.dylibFileName(package));
    await library.copy(bundled.toFilePath());
    output.assets.code.add(
      CodeAsset(package: package, name: '$package.dart', linkMode: DynamicLoadingBundled(), file: bundled),
    );
  });
}

/// The Rust target of what the build is for.
String _target(CodeConfig code) {
  final simulator = code.targetOS == OS.iOS && code.iOS.targetSdk == IOSSdk.iPhoneSimulator;
  return switch ((code.targetOS, code.targetArchitecture)) {
    (OS.android, Architecture.arm64) => 'aarch64-linux-android',
    (OS.android, Architecture.arm) => 'armv7-linux-androideabi',
    (OS.android, Architecture.x64) => 'x86_64-linux-android',
    (OS.iOS, Architecture.arm64) => simulator ? 'aarch64-apple-ios-sim' : 'aarch64-apple-ios',
    (OS.iOS, Architecture.x64) when simulator => 'x86_64-apple-ios',
    (OS.macOS, Architecture.arm64) => 'aarch64-apple-darwin',
    (OS.macOS, Architecture.x64) => 'x86_64-apple-darwin',
    (OS.linux, Architecture.arm64) => 'aarch64-unknown-linux-gnu',
    (OS.linux, Architecture.x64) => 'x86_64-unknown-linux-gnu',
    (OS.windows, Architecture.arm64) => 'aarch64-pc-windows-msvc',
    (OS.windows, Architecture.x64) => 'x86_64-pc-windows-msvc',
    (final os, final arch) => throw UnsupportedError('no Rust target builds for $os on $arch'),
  };
}

/// The release's library for `target`, from the shared cache when it holds the same bytes,
/// else downloaded and checked against the size and hash the release names.
Future<File> _download(BuildInput input, CodeConfig code, File release, String target) async {
  final manifest = jsonDecode(await release.readAsString()) as Map<String, Object?>;
  final libraries = manifest['libraries']! as Map<String, Object?>;
  final entry = libraries[target] as Map<String, Object?>?;
  if (entry == null) {
    throw UnsupportedError('${input.packageName} ships no library for $target, only ${libraries.keys.join(', ')}');
  }
  _floor(input.packageName, code, entry['minimum'] as String?);
  final name = entry['file']! as String;
  final size = entry['size']! as int;
  final hash = entry['sha256']! as String;
  final cached = File.fromUri(input.outputDirectoryShared.resolve(name));
  if (cached.existsSync() && sha256.convert(await cached.readAsBytes()).toString() == hash) return cached;
  final url = Uri.parse(manifest['url']! as String).resolve(name);
  final client = HttpClient()..findProxy = HttpClient.findProxyFromEnvironment;
  try {
    final response = await (await client.getUrl(url)).close();
    if (response.statusCode != HttpStatus.ok) {
      throw HttpException('$url answered ${response.statusCode}', uri: url);
    }
    // Never more than the release names, whatever the host sends.
    final bytes = BytesBuilder(copy: false);
    await for (final chunk in response) {
      bytes.add(chunk);
      if (bytes.length > size) throw StateError('$url is longer than the $size bytes the release names');
    }
    final data = bytes.takeBytes();
    final actual = sha256.convert(data).toString();
    if (data.length != size || actual != hash) {
      throw StateError('$url is ${data.length} bytes of SHA-256 $actual, not the $size of $hash the release names');
    }
    // Renamed into place, so no build sees part of it.
    await cached.parent.create(recursive: true);
    final part = File('${cached.path}.$pid.part');
    await part.writeAsBytes(data, flush: true);
    await part.rename(cached.path);
    return cached;
  } finally {
    client.close(force: true);
  }
}

/// Refuse an app older than the oldest system the released library loads on.
void _floor(String package, CodeConfig code, String? minimum) {
  if (minimum == null) return;
  final parts = minimum.split('.').map(int.parse).toList();
  if (code.targetOS == OS.android) {
    if (code.android.targetNdkApi < parts[0]) {
      throw UnsupportedError(
        "$package's Android library needs API $minimum, above the app's ${code.android.targetNdkApi}: raise its minSdk",
      );
    }
    return;
  }
  final floor = AppleVersion(parts[0], minor: parts.length > 1 ? parts[1] : 0);
  final (system, version) = switch (code.targetOS) {
    OS.iOS => ('iOS', code.iOS.deploymentVersion),
    OS.macOS => ('macOS', code.macOS.deploymentVersion),
    final os => throw UnsupportedError('$package names a minimum $minimum for $os'),
  };
  if (version < floor) {
    throw UnsupportedError(
      "$package's $system library needs $system $minimum, above the app's ${version.major}.${version.minor}: raise its deployment target",
    );
  }
}

/// The library Cargo builds of the exported crate for `target`, and the files it read.
Future<(File, List<Uri>)> _build(BuildInput input, CodeConfig code, String target) async {
  final manifest = File.fromUri(input.packageRoot.resolve('../Cargo.toml'));
  if (!manifest.existsSync()) {
    throw StateError(
      '${input.packageName} holds neither hook/release.json, which its release writes, '
      'nor ../Cargo.toml, the crate rust2-ffi exported it with',
    );
  }
  final environment = <String, String>{};
  final key = target.replaceAll('-', '_');
  switch (code.targetOS) {
    case OS.android:
      // The NDK's clang for the app's API level, which Flutter passes beside its own.
      final compiler = code.cCompiler;
      if (compiler == null) throw StateError('building $target needs the Android NDK, which this build names none of');
      final triple = target == 'armv7-linux-androideabi' ? 'armv7a-linux-androideabi' : target;
      final cmd = Platform.isWindows ? '.cmd' : '';
      final clang = compiler.compiler.resolve('$triple${code.android.targetNdkApi}-clang');
      environment['CARGO_TARGET_${key.toUpperCase()}_LINKER'] = '${clang.toFilePath()}$cmd';
      environment['CC_$key'] = '${clang.toFilePath()}$cmd';
      environment['CXX_$key'] = '${clang.toFilePath()}++$cmd';
      environment['AR_$key'] = compiler.archiver.toFilePath();
    case OS.iOS:
      final version = code.iOS.deploymentVersion;
      environment['IPHONEOS_DEPLOYMENT_TARGET'] = '${version.major}.${version.minor}';
    case OS.macOS:
      final version = code.macOS.deploymentVersion;
      environment['MACOSX_DEPLOYMENT_TARGET'] = '${version.major}.${version.minor}';
  }
  final args = [
    'build', '--release', '--lib', '--features', 'dart', '--target', target, //
    '--manifest-path', manifest.path, '--message-format=json-render-diagnostics',
  ];
  // From the crate's directory, whose toolchain rustup picks.
  final result = await Process.run('cargo', args, workingDirectory: manifest.parent.path, environment: environment);
  if (result.exitCode != 0) {
    throw ProcessException('cargo', args, '${result.stderr}', result.exitCode);
  }
  final name = code.targetOS.dylibFileName(input.packageName);
  final built = <String>{
    for (final line in LineSplitter.split(result.stdout as String))
      if (jsonDecode(line) case {'reason': 'compiler-artifact', 'filenames': final List<Object?> files})
        for (final file in files.cast<String>())
          if (File(file).uri.pathSegments.last == name) file,
  };
  if (built.length != 1) throw StateError('cargo built ${built.isEmpty ? 'no' : built.length} $name for $target');
  final library = File(built.single);
  // Cargo's dep-info beside it lists the sources of every path package the build read.
  final info = File('${library.path.substring(0, library.path.length - name.length)}${name.split('.').first}.d');
  final inputs = [manifest.uri, for (final path in _depInfo(await info.readAsString())) File(path).uri];
  return (library, inputs);
}

/// The files a Cargo dep-info file names its artifact reading: after each line's
/// `<artifact>: `, paths split on spaces, a space in one escaped as `\ `.
List<String> _depInfo(String text) => [
  for (final line in LineSplitter.split(text))
    if (line.indexOf(': ') case final at when at >= 0)
      for (final match in RegExp(r'(?:\\ |[^ ])+').allMatches(line.substring(at + 2)))
        match[0]!.replaceAll(r'\ ', ' '),
];
