# Binding releases

A crate's [rust2-ffi](/ffi/README.md) export, released as Python wheels and an sdist, a Swift package,
C and C++ archives, a Go module, and a web package on npm and JSR. Two halves, split where
Cargo's knowledge ends:

- **The builder**, [`bindings.yml`](/.github/workflows/bindings.yml) over `package.py`,
  shared and read-only. It derives everything from Cargo metadata and the export: package
  name, version, description, license and its files, URLs, and the module names the
  exporter gives the crate's library. It builds, repairs, and checks the packages in clean
  consumers, and publishes nothing.
- **The release workflow**, the caller's own file, which `package.py init` writes once.
  It holds what Cargo cannot establish: who may publish to PyPI, npm, and JSR, the scope
  the web package is named under, where the XCFramework is hosted, which repository
  SwiftPM resolves the package from, the Go module's path, and which tag releases.

## Adopting it

From a rust2 checkout at the commit to pin, in the repository holding the crate:

```sh
python3 <rust2>/ffi/release/package.py init --manifest-path rs/moq-ffi/Cargo.toml --swift-mirror moq-dev/moq-swift-ffi --go-module moq.dev/moq-ffi --scope moq
```

It writes `.github/workflows/release-<crate>.yml`, calling the builder at that commit, and
refuses to overwrite a different file. `--scope` ships the crate's web package as
`@<scope>/<crate>`; without it, the lines that would are written commented out. Then,
outside the repository:

- register the workflow and the `pypi` environment as the PyPI project's trusted publisher;
- create the mirror repository, install a GitHub App on it with contents write, and put its
  `APP_CLIENT_ID` and `APP_PRIVATE_KEY` in a `swift-mirror` environment that only release
  tags may deploy to;
- serve the build's `<crate>-go` artifact, the module laid out as a GOPROXY serves it, from a
  module proxy: on a host you control, a `go-import` tag of type `mod` names it, but Go
  resolves a `github.com` path by git alone, so its consumers set `GOPROXY` to the proxy. No
  job publishes it: a git mirror holds no library over GitHub's 100 MiB, and a proxy is the
  caller's to run.
- for the web package, publish its first version by hand, as npm configures a trusted
  publisher only for a package it holds, then register the workflow and the `npm`
  environment as its trusted publisher; and create the package in the JSR scope and link
  it to the repository, which JSR's publishing from GitHub Actions trusts. Make `npm` and
  `jsr` environments that only release tags may deploy to. npm's provenance needs Cargo's
  `repository` to be the publishing repository, which the release checks before any
  registry takes it.

Releasing is pushing `<crate>-v<version>`, the crate's version, which the release checks.
Called without a tag, such as from a nightly, it runs everything up to publication.
A release that failed while publishing is recovered with **Re-run failed jobs**, which keeps
the run's successful build and its artifacts: each publishing job then finishes what the
first attempt left, uploading only what is missing. A rebuild makes other bytes, as wheels
and the XCFramework archive carry build times, so **Re-run all jobs** fails wherever a
registry already holds the first attempt's files.
A Cargo prerelease, such as `1.0.0-rc.1`, is a GitHub prerelease too.

The builder takes the crate's `Cargo.toml`, the `go-module` path, which without it is the
export's own `<prefix>` that no proxy serves, and optional inputs the caller's tests own:
`features` to ship, an `example` whose stdout is native Rust's transcript, and the
`python-check`, `swift-check`, `c-check`, and `go-check` programs that must print it through
the installed packages. Without a check, a consumer only imports the module, or links a
function of each C and C++ header. A `c-check` is C or C++ by its extension. The web package
takes `web-manifest-path`, the package exported for `wasm32-wasip1`, often the crate's own,
which no Cargo fact says builds for the web, and `scope`; and an optional `web-check`, a
TypeScript module importing the binding as `./web/<module>.ts` does beside an export,
whose imports the consumer rewrites to the package's name. Its crate's version must be
the release's. `features` and `example` apply to it when it is the crate's own; a web
crate apart from it takes `web-features` and `web-example` instead.

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
   crate's directory; only the export runs on rust2's nightly.
5. Python: a `cp311-abi3` wheel per target, the oldest CPython the generated extension's
   stable ABI loads in, holding the module as `<lib>/__init__.py` beside its library and
   PEP 561's `py.typed`, so mypy and pyright check a caller against it, with PEP 639 license
   metadata and a complete hashed RECORD. Each is repaired by its platform's
   standard tool, which claims the portable tag or fails: auditwheel for
   `manylinux_2_28` on x86_64 and aarch64 (built by cargo-zigbuild against glibc 2.28, as
   [crabx](/crabx/README.md) builds), delocate for `macosx_11_0_arm64`, and delvewheel for
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
   `build.target`; refuses a target the export lacks, and one whose architecture differs from the Python's, as an emulated x86_64
   Python on arm64 would build; tags the wheel as pip names its platform, macOS's at the
   deployment target it builds for, by default the target's oldest; and builds musl's
   library against a dynamic C runtime, as CPython links one, adding the flag to cargo's
   target rustflags, which cargo then reads in place of a config's `build.rustflags`.
6. Swift: static libraries for macOS arm64 at macOS 13, and iOS arm64 and its simulator
   at iOS 16, the floors of the generated Swift's `Swift.Duration`, each checked against
   every object's declared minimum OS; one XCFramework of the three, zipped with `ditto`,
   its SHA-256 checked against `swift package compute-checksum`; and a `Package.swift`
   naming it at `https://github.com/<repo>/releases/download/<crate>-v<version>/<Module>FFI.xcframework.zip`.
   The package depends on swift-collections by the release tag at the exporter's pinned
   revision, since a package resolved by version cannot depend on a revision. Rust's
   native link libraries become linker settings, per platform where they differ; any
   other flag fails the build.
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

   The static library is one object prelinked from Rust's archive (`ld -r`, then `objcopy`
   on ELF and the linker's exported-symbols list on Mach-O) whose only global symbols are
   the C ABI's, `<lib>_*`. So its std and compiler-builtins, with unmangled symbols such as
   `__addvdi3`, cannot clash with another Rust static library in the same program, which
   each build checks by linking one beside it. MSVC's linker has no relocatable link, so the
   Windows static library keeps every symbol: a program with two Rust static libraries
   there links the DLL instead.
8. Go: one module for every target, at the module path and `v<version>`, the major
   version's `/vN` suffix checked: the export's `go/`, whose runtime's `#cgo` lines link
   `rt/lib<prefix>_<goos>_<goarch>.a` for `linux/amd64` and `linux/arm64` (built against
   glibc 2.28), `darwin/amd64`, `darwin/arm64` and `ios/arm64` (the latter two the Swift
   package's macOS and iOS device libraries; no consumer here runs iOS), and `windows/amd64` (an
   `x86_64-pc-windows-gnu` library, for Go's MinGW linker), and the license files. Each
   library must need exactly the system libraries the export's line names, as rustc prints
   them building it, and loses its debug info and the LLVM bitcode Go refuses. The module's
   `.info`, `.mod`, and `.zip`, and a version `list` naming this release alone, which a proxy
   serving earlier releases appends to its own, are laid out as a GOPROXY serves them, under Go's 500 MiB
   limit on the zip and on its files, whose sizes the run's summary reports. One module
   holds every platform, so a release is one version of one module. Should it near the
   limit, each platform's library moves to a module of its own, whose one package's `#cgo`
   line links it and which the runtime imports under that platform's build constraint, so
   `go get` downloads only the platform it builds for.
9. The release's `api.json`, and `changes.md`, the items removed, changed, and added since
   the previous release's, by kind and Rust path; the caller's release publishes the first
   as an asset and the second as its notes.
10. Consumers, with no checkout, Rust, or source path: each wheel installed with
    `--no-index --no-deps` into a fresh environment on its oldest CPython, run with `-I`
    from an empty directory, and on Linux x64 checked by `mypy --strict` as installed; the
    sdist installed the same way, built by pip with rustup's Rust, in Alpine's musllinux, on
    Windows on arm64, and on Intel macOS; and the Swift package built by its own scheme for macOS, iOS,
    and the iOS Simulator, then consumed by a new executable package that runs the check;
    and each C archive unpacked alone, built by a new CMake project through its CPS file and
    through its config package, and on Linux and macOS by pkg-config, linking each library,
    where a shared consumer must record the versioned name; and the Go module, unpacked from
    its zip into a new module that requires it, vetted, built with the newest Go and no
    `CGO_LDFLAGS`, and run on Linux x64 and arm64, macOS on arm64 and x86_64, and Windows.
    With an `example`, each must print native Rust's stdout byte for byte.
11. Web: `web-manifest-path` exported for `wasm32-wasip1` alone, whose `web/` holds the
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

rust2 runs it on every manual Check and nightly through
[`bindings-test.yml`](/.github/workflows/bindings-test.yml): on the api fixture, whose
wheels, C++ consumers, and web package must print its native transcript, and on moq's
stand-in facade at the exporter's moq pin, through `moq/moq.py`, `moq/moq.swift`,
`moq/moq.cpp`, and `moq/moq.go`, with its wasm32 stand-in as the web package.

## Other registries

The same boundary holds for each destination: the builder makes and checks the artifact
from Cargo facts, the caller's workflow names the identity and credential. A language whose
generator rust2-ffi lacks, such as C#, is unsupported, not a package without its
binding.

| Destination | Builder work | What only the caller can say |
| --- | --- | --- |
| Maven Central | An AAR with classes.jar, JNI libraries per Android ABI and the generated R8 rules; POM, sources and javadoc jars | The verified groupId namespace, publisher authorization, and signing key |
| pub.dev | The generated Dart library with a native-assets build hook and per-platform archives, pubspec, and license | Publisher authorization and the archives' download host |
| NuGet | Needs a C# generator first: the managed assembly and RID-specific native assets | Package ID ownership, credentials, and target frameworks |

Cargo's `repository` can suggest a Go module path or a Maven namespace but cannot prove
either is the publisher's; a Cargo name need not be the existing PyPI or npm name. Where
a registry requires a field Cargo leaves optional, such as a license, its absence fails
the build.

Prior art: [maturin](https://www.maturin.rs/distribution.html) and
[cibuildwheel](https://cibuildwheel.pypa.io) own Python wheel matrices, and
[BoltFFI](https://github.com/boltffi/boltffi) packages annotated crates locally; none
establishes registry ownership, which is why publication stays in the caller.
