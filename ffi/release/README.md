# Binding releases

A crate's [rust2-ffi](/ffi/README.md) export, released as Python wheels, a Swift package,
and C and C++ archives. Two halves, split where Cargo's knowledge ends:

- **The builder**, [`bindings.yml`](/.github/workflows/bindings.yml) over `package.py`,
  shared and read-only. It derives everything from Cargo metadata and the export: package
  name, version, description, license and its files, URLs, and the module names the
  exporter gives the crate's library. It builds, repairs, and checks the packages in clean
  consumers, and publishes nothing.
- **The release workflow**, the caller's own file, which `package.py init` writes once.
  It holds what Cargo cannot establish: who may publish to PyPI, where the XCFramework is
  hosted, which repository SwiftPM resolves the package from, and which tag releases.

## Adopting it

From a rust2 checkout at the commit to pin, in the repository holding the crate:

```sh
python3 <rust2>/ffi/release/package.py init --manifest-path rs/moq-ffi/Cargo.toml --swift-mirror moq-dev/moq-swift-ffi
```

It writes `.github/workflows/release-<crate>.yml`, calling the builder at that commit, and
refuses to overwrite a different file. Then, outside the repository:

- register the workflow and the `pypi` environment as the PyPI project's trusted publisher;
- create the mirror repository, install a GitHub App on it with contents write, and put its
  `APP_CLIENT_ID` and `APP_PRIVATE_KEY` in a `swift-mirror` environment that only release
  tags may deploy to.

Releasing is pushing `<crate>-v<version>`, the crate's version, which the release checks.
Called without a tag, such as from a nightly, it runs everything up to publication.
A release that failed while publishing is recovered with **Re-run failed jobs**, which keeps
the run's successful build and its artifacts: each publishing job then finishes what the
first attempt left, uploading only what is missing. A rebuild makes other bytes, as wheels
and the XCFramework archive carry build times, so **Re-run all jobs** fails wherever a
registry already holds the first attempt's files.
A Cargo prerelease, such as `1.0.0-rc.1`, is a GitHub prerelease too.

The builder takes the crate's `Cargo.toml` and five optional inputs the caller's tests own:
`features` to ship, an `example` whose stdout is native Rust's transcript, and the
`python-check`, `swift-check`, and `c-check` programs that must print it through the
installed packages. Without a check, a consumer only imports the module, or links a function
of each C and C++ header. A `c-check` is C or C++ by its extension, built at C11 or C++17.

## What the builder does

1. Stages each target's API on the host whose C toolchain its build scripts need.
2. On each build host, exports the binding of every target, merging the others' staged
   APIs, so an item only some targets have fails there at run time instead of being
   missing ([Targets](/ffi/README.md#targets)). A job checks that every host generated the
   same binding.
3. Builds each target with the crate's own toolchain, as rustup resolves it from the
   crate's directory; only the export runs on rust2's nightly.
4. Python: a `cp311-abi3` wheel per target, the oldest CPython the generated extension's
   stable ABI loads in, holding the module as `<lib>/__init__.py` beside its library, with
   PEP 639 license metadata and a complete hashed RECORD. Each is repaired by its platform's
   standard tool, which claims the portable tag or fails: auditwheel for
   `manylinux_2_28` on x86_64 and aarch64 (built by cargo-zigbuild against glibc 2.28, as
   [crabx](/crabx/README.md) builds), delocate for `macosx_11_0_arm64`, and delvewheel for
   `win_amd64`.
5. Swift: static libraries for macOS arm64 at macOS 13, and iOS arm64 and its simulator
   at iOS 16, the floors of the generated Swift's `Swift.Duration`, each checked against
   every object's declared minimum OS; one XCFramework of the three, zipped with `ditto`,
   its SHA-256 checked against `swift package compute-checksum`; and a `Package.swift`
   naming it at `https://github.com/<repo>/releases/download/<crate>-v<version>/<Module>FFI.xcframework.zip`.
   The package depends on swift-collections by the release tag at the exporter's pinned
   revision, since a package resolved by version cannot depend on a revision. Rust's
   native link libraries become linker settings, per platform where they differ; any
   other flag fails the build.
6. C and C++: an archive per wheel target, `<crate>-<version>-<target>.tar.gz`, or `.zip`
   on Windows, built like the wheels, Linux's against glibc 2.28 and macOS's at 11.0. It
   holds `include/` with both headers, the static and shared libraries, and three ways to
   find them, each naming the native libraries the static library needs, from rustc's
   `--print native-static-libs`:
   - `lib/cmake/<lib>/<lib>-config.cmake`, whose `find_package(<lib>)` gives `<lib>::<lib>`,
     the static library, and `<lib>::shared`;
   - `lib/cps/<lib>.cps`, the [Common Package Specification](https://cps-org.github.io/cps/)
     of the same two components, which CMake 4.3 and newer prefers;
   - `lib/pkgconfig/<lib>.pc`, the shared library by default and the static one with
     `--static`; none on Windows, whose MSVC has no pkg-config convention.

   The shared library's name holds Cargo's compatible line, `lib<lib>.so.1` for 1.x,
   `.so.0.4` for 0.4.x, the whole version for a 0.0.x or prerelease, as its SONAME, or its
   `@rpath` install name on macOS (`lib<lib>.1.dylib`); the [API lock](/ffi/README.md#the-api-lock)
   is what keeps a line's ABI. The CMake and CPS versions accept a request within that line.
   On Windows it is `bin/<lib>.dll` with `lib/<lib>.dll.lib`, beside the static `lib/<lib>.lib`.

   The static library is one object prelinked from Rust's archive (`ld -r`, then `objcopy`
   on ELF and the linker's exported-symbols list on Mach-O) whose only global symbols are
   the C ABI's, `<lib>_*`. So its std and compiler-builtins, with unmangled symbols such as
   `__addvdi3`, cannot clash with another Rust static library in the same program, which
   each build checks by linking one beside it. MSVC's linker has no relocatable link, so the
   Windows static library keeps every symbol: a program with two Rust static libraries
   there links the DLL instead.
7. Consumers, with no checkout, Rust, or source path: each wheel installed with
   `--no-index --no-deps` into a fresh environment on its oldest CPython, run with `-I`
   from an empty directory; and the Swift package built by its own scheme for macOS, iOS,
   and the iOS Simulator, then consumed by a new executable package that runs the check;
   and each C archive unpacked alone, built by a new CMake project through its CPS file and
   through its config package, and on Linux and macOS by pkg-config, linking each library,
   where a shared consumer must record the versioned name.
   With an `example`, each must print native Rust's stdout byte for byte.

rust2 runs it on every manual Check and nightly through
[`bindings-test.yml`](/.github/workflows/bindings-test.yml): on the api fixture, whose
wheels and C++ consumers must print its native transcript, and on moq's stand-in facade at
the exporter's moq pin, through `moq/moq.py`, `moq/moq.swift`, and `moq/moq.cpp`.

## Other registries

The same boundary holds for each destination: the builder makes and checks the artifact
from Cargo facts, the caller's workflow names the identity and credential. A language whose
generator rust2-ffi lacks, such as Node or C#, is unsupported, not a package without its
binding.

| Destination | Builder work | What only the caller can say |
| --- | --- | --- |
| Maven Central | An AAR with classes.jar, JNI libraries per Android ABI and the generated R8 rules; POM, sources and javadoc jars | The verified groupId namespace, publisher authorization, and signing key |
| pub.dev | The generated Dart library with a native-assets build hook and per-platform archives, pubspec, and license | Publisher authorization and the archives' download host |
| Go | The generated cgo packages with module-relative imports, and native libraries per target | The module path, repository subdirectory, and release tags, major-version suffixes included |
| npm | Needs a Node generator first: platform packages and a JS loader | Scope ownership and trusted-publisher registration |
| NuGet | Needs a C# generator first: the managed assembly and RID-specific native assets | Package ID ownership, credentials, and target frameworks |

Cargo's `repository` can suggest a Go module path or a Maven namespace but cannot prove
either is the publisher's; a Cargo name need not be the existing PyPI or npm name. Where
a registry requires a field Cargo leaves optional, such as a license, its absence fails
the build.

Prior art: [maturin](https://www.maturin.rs/distribution.html) and
[cibuildwheel](https://cibuildwheel.pypa.io) own Python wheel matrices, and
[BoltFFI](https://github.com/boltffi/boltffi) packages annotated crates locally; none
establishes registry ownership, which is why publication stays in the caller.
