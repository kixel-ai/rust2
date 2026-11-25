# Binding releases

A crate's [rust2-ffi](/ffi/README.md) export, released as Python wheels and an sdist, a Swift package,
C and C++ archives, a Go module, a Kotlin library for desktop JVMs and Android on Maven
Central, a Dart package on pub.dev, a web package on npm and JSR, and a React Native package
on npm. Two halves, split where Cargo's knowledge ends:

- **The builder**, [`bindings.yml`](/.github/workflows/bindings.yml) over `package.py`,
  shared and read-only. It derives everything from Cargo metadata and the export: package
  name, version, description, license and its files, URLs, and the module names the
  exporter gives the crate's library. It builds, repairs, and checks the packages in clean
  consumers, and publishes nothing.
- **The release workflow**, the caller's own file, which `package.py init` writes once.
  It holds what Cargo cannot establish: who may publish to PyPI, pub.dev, npm, JSR, and
  Maven Central, the scope the web and React Native packages are named under, the Kotlin library's groupId,
  where the XCFramework and the Dart libraries are hosted, which repository SwiftPM resolves
  the package from, the Go module's path, and which tag releases.

## Adopting it

From a rust2 checkout at the commit to pin, in the repository holding the crate:

```sh
python3 <rust2>/ffi/release/package.py init --manifest-path rs/moq-ffi/Cargo.toml --swift-mirror moq-dev/moq-swift-ffi --go-module moq.dev/moq-ffi --scope moq --maven-group dev.moq
```

It writes `.github/workflows/release-<crate>.yml`, calling the builder at that commit, and
refuses to overwrite a different file. `--scope` ships the crate's web package as
`@<scope>/<crate>` and its React Native package as `@<scope>/<crate>-rn`, since one npm name
cannot hold both a wasm32 module and a TurboModule, and `--maven-group` its Kotlin library as
`<group>:<crate>`; without
either, the lines that would are written commented out. Then, outside the repository:

- register the workflow and the `pypi` environment as the PyPI project's trusted publisher;
- create the mirror repository, install a GitHub App on it with contents write, and put its
  `APP_CLIENT_ID` and `APP_PRIVATE_KEY` in a `swift-mirror` environment that only release
  tags may deploy to;
- serve the build's `<crate>-go` artifact, the module laid out as a GOPROXY serves it, from a
  module proxy: on a host you control, a `go-import` tag of type `mod` names it, but Go
  resolves a `github.com` path by git alone, so its consumers set `GOPROXY` to the proxy. No
  job publishes it: a git mirror holds no library over GitHub's 100 MiB, and a proxy is the
  caller's to run.
- for the Dart package, publish its first version by hand, the `<crate>-pub` artifact's
  `package/` after its libraries are on the release, as pub.dev configures automated
  publishing only for a package it holds; then, on its admin page, enable publishing from
  GitHub Actions for this repository, with the tag pattern `<crate>-v{{version}}` and the
  `pub` environment required, and make that environment one only release tags may deploy to;
- for the web package, publish its first version by hand, as npm configures a trusted
  publisher only for a package it holds, then register the workflow and the `npm`
  environment as its trusted publisher; and create the package in the JSR scope and link
  it to the repository, which JSR's publishing from GitHub Actions trusts. Make `npm` and
  `jsr` environments that only release tags may deploy to. npm's provenance needs Cargo's
  `repository` to be the publishing repository, which the release checks before any
  registry takes it.
- for the React Native package, likewise publish its first version by hand, the
  `<crate>-react-native` artifact's tarball, then register the same workflow and `npm`
  environment as its trusted publisher.
- for the Kotlin library, verify the group's namespace on the
  [Central Portal](https://central.sonatype.com), and make a `publish` environment that only
  release tags may deploy to, holding a Portal user token as `MAVEN_CENTRAL_USERNAME` and
  `MAVEN_CENTRAL_PASSWORD`, and an ASCII-armored private OpenPGP key whose public half a key
  server holds as `SIGNING_KEY`, with its passphrase, if any, as `SIGNING_PASSWORD`. Central
  has no trusted publishing, so these are long-lived. Before the first tag,
  `<rust2>/ffi/release/central.sh validate <repository>`, given the build's `<crate>-maven` artifact
  and the same secrets, the Portal token as `USERNAME` and `PASSWORD`, has the Portal
  validate the signed bundle and drops it unpublished.

Releasing is pushing `<crate>-v<version>`, the crate's version, which the release checks.
Called without a tag, such as from a nightly, it runs everything up to publication.
A release that failed while publishing is recovered with **Re-run failed jobs**, which keeps
the run's successful build and its artifacts: each publishing job then finishes what the
first attempt left, uploading only what is missing. A rebuild makes the same bytes
([Reproducible builds](#reproducible-builds)), so **Re-run all jobs** works as well.
A Cargo prerelease, such as `1.0.0-rc.1`, is a GitHub prerelease too.

The builder takes the crate's `Cargo.toml`, the `go-module` path, which without it is the
export's own `<prefix>` that no proxy serves, and optional inputs the caller's tests own:
`features` to ship, an `example` whose stdout is native Rust's transcript, and the
`python-check`, `swift-check`, `c-check`, `go-check`, `kotlin-check`, `dart-check`, and
`react-native-check` programs that must print it through the installed packages, and `reproduce`, which builds
everything twice, from two checkouts. Without a check, a consumer only imports the module,
links a function of each C and C++ header, or loads the Kotlin library. A `c-check` is C
or C++ by its extension. A `dart-check` prints through `print`, which a Flutter app on
Android passes to its log, not through `stdout`. The Kotlin library takes `maven-group`,
its groupId, which no Cargo fact proves the publisher holds; without it, the builder makes
none. A `kotlin-check` is a Kotlin file whose top-level `main` prints the transcript,
importing the binding's package; each consumer runs the class the compiler made that `main`
a member of, whatever its package or `@file:JvmName`. The web package takes `web-manifest-path`, the package exported for
`wasm32-wasip1`, often the crate's own, which no Cargo fact says builds for the web, and
`scope`; and an optional `web-check`, a
TypeScript module importing the binding as `./web/<module>.ts` does beside an export,
whose imports the consumer rewrites to the package's name. Its crate's version must be
the release's. `features` and `example` apply to it when it is the crate's own; a web
crate apart from it takes `web-features` and `web-example` instead. The React Native
package takes `scope` alone, and a `react-native-check` is a TypeScript module importing
the binding as `./react-native/<module>.ts` does beside an export, whose top-level await
the consumer runs, and which prints through `console.log` and may end through
`process.exit`.
Every check may instead name a directory holding the program as `check.py`, `check.c` or
`check.cpp`, `check.kt`, `check.ts`, `check.swift`, `check.go`, or `check.dart`, beside the files it
reads, which each consumer places beside it: a check that imports a module, includes a header, or
calls a function of files of its own. A Dart check's `main` takes no arguments or a list of
them, a Go check's directory is its package, built beside the consumer's own `go.mod`, and a
Kotlin check's directory holds Kotlin sources alone, as the Android app it runs in reads no file
beside it.
A check its tests generate comes through `checks-artifact`, an artifact of the caller's run
the builder unpacks over its checkout before reading the checks.

## What the builder does

1. Finds the earlier releases among the `<crate>-v<version>` GitHub releases, and their
   `api.json` assets: the newest below the crate's version, which the changelog follows,
   and the newest below it Cargo treats as compatible (the same leftmost nonzero part,
   neither a prerelease), which each export passes as `--baseline`, so a release that
   breaks its API without a breaking version fails
   ([The API lock](/ffi/README.md#the-api-lock)). A release without the asset fails the
   build.
2. Stages each target's API on the host whose C toolchain its build scripts need: each
   package's below, and those the sdist builds on, `x86_64-unknown-linux-musl`,
   `aarch64-unknown-linux-musl`, `x86_64-apple-darwin`, and `aarch64-pc-windows-msvc`, since
   the generated crate builds for no target it was not exported for.
3. On each build host, exports the binding of every target, merging the others' staged
   APIs, so an item only some targets have fails there at run time instead of being
   missing ([Targets](/ffi/README.md#targets)). A job checks that every host generated the
   same binding.
4. Builds each target with the crate's own toolchain, as rustup resolves it from the
   crate's directory; only the export runs on rust2's nightly. Android's targets build, and
   stage their APIs, on Linux through the NDK Flutter defaults to, r28: with a `scope`,
   React Native's fourth, x86 (`i686-linux-android`), too.
5. Python: two wheels per target, built without asking a CPython (`PYO3_NO_PYTHON`): a
   `cp311-abi3` one, the oldest CPython the generated extension's stable ABI loads in, and
   a `cp315-abi3t` one for free-threaded CPython 3.15 and later (PEP 803), which leaves the
   GIL off. The `abi3t` one is tagged for free-threaded builds alone, where `abi3.abi3t`
   would move GIL builds from 3.15 onto it, whose Windows library links the free-threaded
   build's `python3t.dll`.
   Both carry the same metadata, `Requires-Python: >=3.11`, as a resolver reads one wheel's
   for all. Each holds the module as `<lib>/__init__.py` beside its library and
   PEP 561's `py.typed`, so mypy and pyright check a caller against it, with PEP 639 license
   metadata and a complete hashed RECORD. Each is repaired by its platform's
   standard tool, which claims the portable tag or fails: auditwheel for
   `manylinux_2_28` on x86_64 and aarch64 (built by cargo-zigbuild against glibc 2.28, as
   [crabx's pipeline](https://github.com/kixel-ai/crabx/blob/main/pipeline/README.md) builds), delocate for `macosx_11_0_arm64`, and delvewheel for
   `win_amd64`.

   And an sdist, `<name>-<version>.tar.gz`, from which pip builds the same wheel on any
   target the export names, such as musllinux, Intel macOS, and Windows on arm64, with Rust
   and no Python package: its `pyproject.toml` names `rust2/backend.py`
   ([`backend.py`](backend.py), which writes the prebuilt wheels too) as its PEP 517
   backend, which runs `cargo rustc --locked --release` on the generated crate in
   `binding/`. The crate's own path dependencies lie in `sources/`, each as `cargo package`
   lists it, at the places they hold relative to each other, so their relative paths still
   resolve and their absolute ones are rewritten; a workspace root they inherit from keeps
   its `[workspace]` table alone. A `rust-toolchain.toml` pins the channel rustup resolves
   from the crate's directory, without its components. The backend builds for
   `CARGO_BUILD_TARGET`, or else rustc's host, passed as `--target` over a cargo config's
   `build.target`; builds the `abi3t` extension on a free-threaded CPython, refusing one
   before 3.15, and the `abi3` one otherwise; refuses a target the export lacks, and one whose architecture differs from the Python's, as an emulated x86_64
   Python on arm64 would build; tags the wheel as pip names its platform, macOS's at the
   deployment target it builds for, by default the target's oldest; and builds musl's
   library against a dynamic C runtime, as CPython links one, adding the flag to cargo's
   target rustflags, which cargo then reads in place of a config's `build.rustflags`.
6. Swift: static libraries for macOS arm64 at macOS 13 and Mac Catalyst arm64 at iOS 16,
   the floors of the generated Swift's `Swift.Duration`, and iOS arm64 and its simulator at
   React Native's 15.1, below the package's 16, as React Native's package takes them too,
   each checked against every object's declared minimum OS and hidden as the C archive's
   static library is; one XCFramework of the four, zipped by the builder itself,
   its SHA-256 checked against `swift package compute-checksum`; and a `Package.swift`
   naming it at `https://github.com/<repo>/releases/download/<crate>-v<version>/<Module>FFI.xcframework.zip`.
   The package depends on swift-collections by the release tag at the exporter's pinned
   revision, since a package resolved by version cannot depend on a revision. Rust's
   native link libraries become linker settings, per platform where they differ; any
   other flag fails the build.

   Each slice is linked as an app links it, keeping only what the C ABI reaches, and
   `nm` lists the [required-reason APIs](https://developer.apple.com/documentation/bundleresources/describing-use-of-required-reason-api)
   that link imports, which the App Store's upload scan refuses undeclared (ITMS-91053).
   The package's `PrivacyInfo.xcprivacy` declares them, a resource of its own
   `<Module>Privacy` target, which SwiftPM bundles into the app, since a static library holds
   none: file metadata (`stat` and its kin, and Foundation's date keys such as
   `NSURLCreationDateKey`) of the app's containers, `C617.1`, and of files
   the user granted it, `3B52.1`; and the boot clock
   (`mach_absolute_time`) for elapsed time, `35F9.1`. Disk space (`statfs` and its kin, and
   keys such as `NSFileSystemFreeSize`), `getattrlist`, user defaults, and active keyboards have no reason
   that fits every library, so a library reaching them fails the build. An Objective-C call
   by selector, or to a class looked up at run time, imports no symbol, and the scan misses it.
   A sandboxed macOS app may also stat a world-readable system path, such as `/etc/localtime`,
   which no reason Apple defines covers, so neither these two nor a hand-written manifest
   can declare it.

   The slices carry line tables, `debug = "line-tables-only"` unless the crate's release
   profile sets a level holding them: a static library has no dSYM of its own, and the app's dSYM is made
   from the debug info of what it links, so a crash in Rust symbolicates to its source
   line, which the build checks by `atos` on a dSYM of each slice's link.
7. C and C++: an archive per wheel target, `<crate>-<version>-<target>.tar.gz`, or `.zip`
   on Windows, built like the wheels, Linux's against glibc 2.28 and macOS's at 11.0. It
   holds `include/` with both headers, the static and shared libraries, and three ways to
   find them, each naming the native libraries the static library needs, from rustc's
   `--print native-static-libs`:
   - `lib/cmake/<lib>/<lib>-config.cmake`, whose `find_package(<lib>)` gives `<lib>::<lib>`,
     the static library, and `<lib>::<lib>_shared`;
   - `lib/cps/<lib>.cps`, the [Common Package Specification](https://cps-org.github.io/cps/)
     of the same two components, which CMake 4.3 and newer prefers;
   - `lib/pkgconfig/<lib>.pc`, which links the shared library, as the linker prefers it to
     the static one beside it; `--static` adds the native libraries, for a prefix that holds
     only the static library. None on Windows, whose MSVC has no pkg-config convention.

   The shared library's SONAME, or `@rpath` install name on macOS, holds the whole version,
   `lib<lib>.so.0.4.12` or `lib<lib>.0.4.12.dylib`: every binding checks at load that the
   library's [contract](/ffi/README.md#the-c-abi) is its header's, so no two releases'
   libraries can stand in for each other. A build-time request is another matter, as header
   and library come together: the CMake and CPS versions accept one within Cargo's
   compatible line, 0.4 for 0.4.12, and a prerelease only by a request naming no version.
   Both targets require C11 and C++17 of what links them, the headers' floor.
   On Windows it is `bin/<lib>.dll` with `lib/<lib>.dll.lib`, beside the static `lib/<lib>.lib`.

   The static library is hidden, as every static library the builder ships but Windows'
   is: `llvm-objcopy` renames every global symbol it defines but the C ABI's, the `<lib>_*`
   symbols the generated crate's own objects define, to `<name>.<lib>.<digest>`, the
   digest of the archive's bytes, a suffix Rust's and C++'s demanglers skip; on ELF the
   renamed symbols are hidden too, so a shared library linking it exports only the C ABI.
   So its std and compiler-builtins, with unmangled symbols such as `rust_eh_personality`
   and `__addvdi3`, cannot clash with another Rust static library in the same program,
   built by any toolchain, nor with another version of its own. Each object keeps its
   sections whole: a relocatable link would drop Mach-O's DWARF, and with `--gc-sections`
   lld drops ELF's unwind tables. Each build checks it by linking one beside it, built by
   a released stable toolchain two releases older (three from a nightly) without the
   crate's compiler flags, in either order, and running the program where the host can:
   one the same toolchain built would link even unhidden, since the linker takes std's
   members from the first archive alone. A Windows static library keeps every symbol
   ([its quest](/quest/m1/ffi-packaging/static-isolation.md)): a C program with two Rust
   static libraries there links the DLL instead.
8. Go: one module for every target, at the module path and `v<version>`, the major
   version's `/vN` suffix checked: the export's `go/`, whose runtime's `#cgo` lines link
   `rt/lib<prefix>_<goos>_<goarch>.a` for `linux/amd64` and `linux/arm64` (built against
   glibc 2.28), `darwin/amd64`, `darwin/arm64` and `ios/arm64` (the latter two the Swift
   package's macOS and iOS device libraries; no consumer here runs iOS), `android/arm64` and
   `android/amd64` (at the Dart package's and the AAR's API 24; no consumer here runs Go on
   Android), and `windows/amd64` (an
   `x86_64-pc-windows-gnu` library, for Go's MinGW linker), and the license files. Each
   library must need exactly the system libraries the export's line names, as rustc prints
   them building it, is hidden as the C archive's static library is, and loses its debug
   info and the LLVM bitcode Go refuses. The Windows library is not hidden, so a Go program
   on Windows cannot also link a Rust static library another toolchain built. The module's
   `.info`, `.mod`, and `.zip`, and a version `list` naming this release alone, which a proxy
   serving earlier releases appends to its own, are laid out as a GOPROXY serves them, under Go's 500 MiB
   limit on the zip and on its files, whose sizes the run's summary reports. One module
   holds every platform, so a release is one version of one module. Should it near the
   limit, each platform's library moves to a module of its own, whose one package's `#cgo`
   line links it and which the runtime imports under that platform's build constraint, so
   `go get` downloads only the platform it builds for.
9. Dart: the export's `dart/`, a package `package:<lib>/<lib>.dart` names, whose
   `hook/build.dart` bundles its library as the native asset its `@Native` functions find
   ([Dart](/ffi/README.md#dart)). Each target's library, the C ABI with its Dart glue,
   `<crate>-<version>-dart-<target>.so`, `.dylib`, or `.dll`: Linux x64 and arm64 (against
   glibc 2.28), macOS arm64 and x64 (at 10.15), iOS arm64 and its arm64 simulator (at 13.0),
   Windows x64, and Android arm64, armv7, and x86_64 (at API 24), Flutter's floors. The
   package adds `hook/release.json`, naming each at
   `https://github.com/<repo>/releases/download/<crate>-v<version>/` by size and SHA-256,
   and the oldest system it loads on, where an app could target an older one; the hook
   downloads its target's into the build's shared cache, refusing other bytes, and refuses
   an app targeting an older system. Its pubspec takes Cargo's description, repository,
   and homepage; `LICENSE` holds each license file's text, or else Cargo's SPDX expression,
   as pub reads no other name; `README.md` is the crate's readme, or else a line naming
   the crate; and `CHANGELOG.md` is `changes.md` under the version. `PrivacyInfo.xcprivacy`
   declares the required-reason APIs each Apple library imports, as step 6 finds them for
   the Swift package, failing on the same categories; an app on Apple's stores merges it
   into its own manifest, as Flutter wraps a code asset in a framework it generates, which
   carries none. `dart pub publish --dry-run` must find no warning, and the analyzer no
   issue with the lints pub.dev scores by.
10. Kotlin, with a `maven-group`: a library per desktop target, the wheels', built with
    `--features kotlin` as the wheels are, and a library per ABI the AAR holds, `arm64-v8a`,
    `armeabi-v7a`, and `x86_64`, built with `--features kotlin` by the NDK for API 24, moq's floor, each checked to align
    its segments to the 16 KB pages Google Play requires of a 64-bit library from 2027-02-01,
    and to need no library an Android 24 device lacks, the NDK's `libc++_shared.so` included,
    which an AAR cannot ship, as an app holding another AAR's copy packs only one. Each keeps its symbol table and line tables, `debug =
    "line-tables-only"` unless the crate's release profile or the environment sets a level
    holding them, over any level or `strip` a `.cargo/config.toml` or the profile sets: an app's build strips them from its APK and gives
    them to Play, which symbolicates a crash in Rust to its source line.

    The generated Kotlin is compiled once, against API 35's `android.jar`, whose `@FastNative`
    serves, and kotlinx-coroutines, for Kotlin 2.2 and JVM 1.8, so a consumer's Kotlin 2.1
    reads its metadata. A desktop JVM runs the same classes, which name no Android API, and
    never loads `@FastNative`, an annotation only ART reads.
    The jar, `<crate>-<version>.jar`, holds the classes and each wheel target's desktop library
    (glibc Linux on x86_64 and aarch64, macOS on arm64, Windows on x86_64) at
    `<lib>/native/<target>/`, beside the index `<lib>/<lib>.natives` naming them, from which
    the binding loads its host's ([Kotlin](/ffi/README.md#kotlin)). The AAR,
    `<crate>-<version>.aar`, holds the Android libraries in `jni/`, the classes as
    `classes.jar`, the generated R8 rules as `proguard.txt`, which each app that
    shrinks it applies, and a manifest naming the floor and the namespace `<group>.<lib>`,
    spelled as the Java package the Android Gradle Plugin requires (`io.7mind` as `io._7mind`).
    Beside them in a Maven repository, `<crate>-maven`, as Central requires: one POM, whose
    packaging is the jar, so Maven, which reads no Gradle metadata, resolves the jar; Gradle
    module metadata (`.module`) choosing the AAR for Android and the jar for the JVM, by the
    attributes Kotlin Multiplatform publishes them with, so an app names one dependency
    either way; the POM with the crate's name, Cargo's description, homepage or else repository, each of its license's
    alternatives, its authors or else its repository's owner, and source control, each a
    field whose absence fails the build, and the Kotlin standard library and
    kotlinx-coroutines as dependencies; a sources jar, a javadoc jar pointing at it, as the
    KDoc an IDE shows is the sources'; a CycloneDX SBOM of the JNI build's crates; and each
    file's MD5 and SHA-1. The caller's release signs each and uploads the bundle.
11. The release's `api.json`, and `changes.md`, the items removed, changed, and added since
   the previous release's, by kind and Rust path; the caller's release publishes the first
   as an asset and the second as its notes.
12. Consumers, with no checkout, Rust, or source path: each wheel installed with
    `--no-index --no-deps` into a fresh environment on its oldest CPython, run with `-I`
    from an empty directory, and on Linux x64 checked by `mypy --strict` as installed; the
    sdist installed the same way, built by pip with rustup's Rust, in Alpine's musllinux, on
    Windows on arm64, and on Intel macOS; and the Swift package built by its own scheme for macOS, iOS,
    the iOS Simulator, and Mac Catalyst, then consumed by a new executable package that runs
    the check, whose bundled privacy manifest must declare what the executable imports;
    and each C archive unpacked alone, built by a new CMake project through its CPS file and
    through its config package, and on Linux and macOS by pkg-config, linking each library,
    where a shared consumer must record the versioned name; and the Go module, unpacked from
    its zip into a new module that requires it, vetted, built with the newest Go and no
    `CGO_LDFLAGS`, and run on Linux x64 and arm64, macOS on arm64 and x86_64, and Windows;
    and the Dart package, its libraries served from the loopback, in a new Dart app by
    `dart run` and from `dart build cli`'s bundle on Linux x64 and arm64, macOS on arm64
    and x86_64, and Windows, on the newest Dart and on Linux on 3.10, the oldest the package takes, after a
    hook served a library of other bytes refused it; and in a new Flutter app on an Android
    emulator at API 24, run in debug, then again after a hot restart, which keeps the library
    and Rust's statics but not the isolate;
    and a new Gradle JVM app, Kotlin's Gradle plugin compiling the check, depending on the
    library through its Maven repository as on Central, which must resolve the jar, run with
    the classpath Gradle resolved on Linux x64 and arm64, macOS on arm64, and Windows;
    and a new Gradle app, its Android Gradle Plugin's Kotlin compiling the check, depending
    on the library through its Maven repository as on Central, which resolves the AAR, shrunk by R8 with the AAR's rules,
    its libraries stripped by the NDK they were built with,
    whose bundle must hold each library's line tables for Play, run on an x86_64 emulator at
    API 35: an instrumentation in the app's own process and class loader runs the check, or
    loads the library, and reports its output and how the process ended, as a check may end
    it with `exitProcess`.
    With an `example`, each must print native Rust's stdout byte for byte, the Android app
    the example's built for the emulator and run there, which is built only for a
    `kotlin-check` to match.
    Beside them, on Linux x64, a new Dart app on the export's own package, whose hook builds
    the crate with Cargo, as in a checkout.
13. Web: `web-manifest-path` exported for `wasm32-wasip1` alone, whose `web/` holds the
    TypeScript, its JavaScript and declarations, and a `package.json`
    ([TypeScript (web)](/ffi/README.md#typescript-web)), and its module built at
    `opt-level = "s"` unless the crate's release profile sets a level, as a page downloads
    it. The npm package is the JavaScript, declarations, and module, named
    `@<scope>/<crate>` with Cargo's description, license, and URLs, packed by `npm pack`;
    the JSR package is the TypeScript and module with a `jsr.json` exporting each module by
    name, refused past JSR's 20 MiB. The example runs for `wasm32-wasip1` under Node's WASI,
    native Rust on the binding's target. A new npm project installs the tarball, as an
    app would, and the `web-check`, or else a `load()`, runs on Node, Deno, and Bun, and
    bundled by Vite and by webpack into a browser's script that Node runs, each finding the
    module where `load` looks by default; and `deno publish --dry-run` checks the JSR
    package, its types against Deno's included.
14. React Native, with a `scope`: the export's `react-native/`
    ([React Native](/ffi/README.md#react-native)), with the libraries its app builds link:
    a static library of the C ABI for each ABI React Native's template builds, `arm64-v8a`,
    `armeabi-v7a`, `x86`, and `x86_64`, at API 24, its floor, hidden as the C archive's
    static library is, and an XCFramework of the
    Swift package's iOS device and simulator libraries, at its iOS floor, 15.1. Each is
    stripped of debug info and LTO bitcode, as the Go module's: an unlinked library's DWARF
    is most of its bytes, and with it moq's tarball would be about 400 MB, twice what npm
    takes, so a crash in Rust symbolicates to the function. `android/CMakeLists.txt`
    links the system libraries rustc names beside the module's own, and the podspec links
    its frameworks and libraries and bundles `ios/PrivacyInfo.xcprivacy`, which declares
    the required-reason APIs a link of the iOS libraries imports, as step 6 finds them,
    failing on the same categories, since a static library holds no resources. The slices
    of each platform must link the same libraries. The XCFramework is made twice, which
    must be the same bytes. The npm package, `@<scope>/<crate>-rn`, takes Cargo's
    description, license, and URLs, and is packed by `npm pack`; its SBOM is the C ABI's
    crates'. A new app at React Native 0.88.1, the package's floor, made by the community
    CLI's `init`, installs the tarball, as an app would; esbuild bundles the
    `react-native-check`, or else a `load()`, leaving its imports of React Native and the
    package to Metro, and it runs as the app starts, logging what it printed. It is built
    for release for each of the four ABIs and run on an x86_64 emulator at API 35, and
    built for release for an iOS device and for the simulator and run on a simulator. With
    an `example`, each must print native Rust's stdout byte for byte, the example's built
    for the emulator and the simulator and run there.

rust2 runs it on every manual Check and nightly through
[`bindings-test.yml`](/.github/workflows/bindings-test.yml): on the api fixture, whose
wheels, C++ consumers, web package, Go module, Swift package, and Dart package must print
its native transcript through the checks `just ffi transcripts` writes, each test beside the
script generated from it, and on moq's
stand-in facade at the exporter's moq pin, through `moq/moq.py`, `moq/moq.swift`,
`moq/moq.cpp`, `moq/moq.go`, `moq/moq.kt`, `moq/moq.dart`, and through React Native's
package `moq/moq.ts`, with its wasm32 stand-in as the web package.

## Reproducible builds

A rebuild of a commit makes the same bytes, so **Re-run all jobs** finds each registry
holding exactly the files it would upload, and anyone with the commit can check them.

- **Time.** Every file takes `SOURCE_DATE_EPOCH`, or else the commit's time, whatever its
  archive format: the builder writes each ZIP itself, with sorted entries, fixed modes, and
  no host, umask, or time of its own, and rewrites each repaired wheel the same way, its
  RECORD rebuilt. Zlib's own output is the one thing a host could still change.
- **Paths.** A checkout at any path builds the same bytes, as long as the export keeps its
  place beside the crate: the exporter writes the crate's facade, and the patches and crates
  it resolves with, relative to the export, and `build` and `web` run cargo through
  `/tmp/rust2-ffi` (`C:\rust2-ffi` on Windows), a link to the deepest directory holding the
  export, those crates, and their workspaces. Cargo hashes a path crate outside the
  generated crate's workspace, the facade, by its absolute path into every symbol, and
  dlltool names a Windows GNU library's import symbols after its target directory, so each
  sees one path wherever the checkout is. One build at a time holds the link: a second fails
  while it exists, and one a killed build left must be removed. An export at another depth
  from the crate writes another relative path, so other symbols; one whose own path holds a
  symlink, or on another drive from the crate, keeps the absolute path, so builds the same
  only at the same path, as does a crate whose manifest names any path absolutely, or a
  build whose `.cargo/config.toml` names a path package (a path `[patch]` or `paths`), which
  cargo joins from the config's real directory: cargo would read it through the link as a
  second package, so such a build runs without the link. The crate's workspace, the exported crate, and `CARGO_HOME`, by
  each spelling, are remapped to fixed names for rustc and the C compilers, so panic
  messages and debug info name no checkout. This takes `RUSTFLAGS` over a
  `.cargo/config.toml`'s `rustflags`, so a config that sets any fails the build: export them
  as `RUSTFLAGS`, which the builder keeps. Apple builds also stamp no archive time
  (`ZERO_AR_DATE`), and Windows links with `/Brepro`. rustc's random temporary directory in
  a Windows GNU import symbol's name is replaced by zeros.
- **Checked.** The Swift job builds the XCFramework twice and fails unless both ZIPs match.
  With `reproduce: true`, every build host builds a second time, from a second checkout and
  export a level deeper, and `package.py same` fails unless each wheel, static library, C
  archive, Dart library, and Kotlin library, the Go module's included, matches byte for
  byte; the Maven job builds its repository a second time from a copy of the export a level
  deeper, which must match too, the web job builds its npm tarball from the second checkout,
  and the React Native job packs its tarball again; rust2
  turns it on for its api fixture. It doubles the build and shares the same 120-minute
  build-job cap as ordinary runs, including in nightlies. Use it only when both builds
  fit that cap; slow legs of a large crate can be cancelled before comparison.

**Attestations and SBOM.** `package.py build` writes `sbom.cdx.json`, a CycloneDX SBOM of
the crates the binding links (normal dependencies, platform-neutral, with each registry
crate's checksum from `Cargo.lock`), which every wheel also carries in
`.dist-info/sboms/` (PEP 770) and the build hosts must agree on. The caller's `attest` job
runs on a tag, before anything is published, and gives every wheel, the XCFramework, the Go
module's `.zip` and `.mod`, each C archive, each Dart library, the jar, the AAR, their POM and Gradle metadata, and the
npm tarballs a build provenance attestation and an SBOM attestation (each tarball's from its
own `sbom.cdx.json`, the Kotlin library's from its CycloneDX file), which
`gh attestation verify <file> --repo <owner>/<repo>` checks. Only that job holds `id-token` and `attestations`
write; the builder stays read-only. PyPI's trusted publishing uploads its own PEP 740
attestations for the wheels as well.

## Other registries

The same boundary holds for each destination: the builder makes and checks the artifact
from Cargo facts, the caller's workflow names the identity and credential. A language whose
generator rust2-ffi lacks, such as C#, is unsupported, not a package without its
binding.

| Destination | Builder work | What only the caller can say |
| --- | --- | --- |
| NuGet | Needs a C# generator first: the managed assembly and RID-specific native assets | Package ID ownership, credentials, and target frameworks |

Cargo's `repository` can suggest a Go module path or a Maven namespace but cannot prove
either is the publisher's; a Cargo name need not be the existing PyPI or npm name. Where
a registry requires a field Cargo leaves optional, such as a license, its absence fails
the build.

Prior art: [maturin](https://www.maturin.rs/distribution.html) and
[cibuildwheel](https://cibuildwheel.pypa.io) own Python wheel matrices, and
[BoltFFI](https://github.com/boltffi/boltffi) packages annotated crates locally; none
establishes registry ownership, which is why publication stays in the caller.
