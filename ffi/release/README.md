# Binding releases

A crate's [rust2-ffi](/ffi/README.md) export, released as Python wheels, a Swift package,
and a Go module. Two halves, split where Cargo's knowledge ends:

- **The builder**, [`bindings.yml`](/.github/workflows/bindings.yml) over `package.py`,
  shared and read-only. It derives everything from Cargo metadata and the export: package
  name, version, description, license and its files, URLs, and the module names the
  exporter gives the crate's library. It builds, repairs, and checks the packages in clean
  consumers, and publishes nothing.
- **The release workflow**, the caller's own file, which `package.py init` writes once.
  It holds what Cargo cannot establish: who may publish to PyPI, where the XCFramework is
  hosted, which repository SwiftPM resolves the package from, the Go module's path, and
  which tag releases.

## Adopting it

From a rust2 checkout at the commit to pin, in the repository holding the crate:

```sh
python3 <rust2>/ffi/release/package.py init --manifest-path rs/moq-ffi/Cargo.toml --swift-mirror moq-dev/moq-swift-ffi --go-module moq.dev/moq-ffi
```

It writes `.github/workflows/release-<crate>.yml`, calling the builder at that commit, and
refuses to overwrite a different file. Then, outside the repository:

- register the workflow and the `pypi` environment as the PyPI project's trusted publisher;
- create the mirror repository, install a GitHub App on it with contents write, and put its
  `APP_CLIENT_ID` and `APP_PRIVATE_KEY` in a `swift-mirror` environment that only release
  tags may deploy to;
- serve the build's `<crate>-go` artifact, the module laid out as a GOPROXY serves it, from a
  module proxy: on a host you control, a `go-import` tag of type `mod` names it, but Go
  resolves a `github.com` path by git alone, so its consumers set `GOPROXY` to the proxy. No
  job publishes it: a git mirror holds no library over GitHub's 100 MiB, and a proxy is the
  caller's to run.

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
`python-check`, `swift-check`, and `go-check` programs that must print it through the
installed packages. Without a check, a consumer only imports the module.

## What the builder does

1. Finds the earlier releases among the `<crate>-v<version>` GitHub releases, and their
   `api.json` assets: the newest below the crate's version, which the changelog follows,
   and the newest below it Cargo treats as compatible (the same leftmost nonzero part,
   neither a prerelease), which each export passes as `--baseline`, so a release that
   breaks its API without a breaking version fails
   ([The API lock](/ffi/README.md#the-api-lock)). A release without the asset fails the
   build.
2. Stages each target's API on the host whose C toolchain its build scripts need.
3. On each build host, exports the binding of every target, merging the others' staged
   APIs, so an item only some targets have fails there at run time instead of being
   missing ([Targets](/ffi/README.md#targets)). A job checks that every host generated the
   same binding.
4. Builds each target with the crate's own toolchain, as rustup resolves it from the
   crate's directory; only the export runs on rust2's nightly.
5. Python: a `cp311-abi3` wheel per target, the oldest CPython the generated extension's
   stable ABI loads in, holding the module as `<lib>/__init__.py` beside its library, with
   PEP 639 license metadata and a complete hashed RECORD. Each is repaired by its platform's
   standard tool, which claims the portable tag or fails: auditwheel for
   `manylinux_2_28` on x86_64 and aarch64 (built by cargo-zigbuild against glibc 2.28, as
   [crabx](/crabx/README.md) builds), delocate for `macosx_11_0_arm64`, and delvewheel for
   `win_amd64`.
6. Swift: static libraries for macOS arm64 at macOS 13, and iOS arm64 and its simulator
   at iOS 16, the floors of the generated Swift's `Swift.Duration`, each checked against
   every object's declared minimum OS; one XCFramework of the three, zipped with `ditto`,
   its SHA-256 checked against `swift package compute-checksum`; and a `Package.swift`
   naming it at `https://github.com/<repo>/releases/download/<crate>-v<version>/<Module>FFI.xcframework.zip`.
   The package depends on swift-collections by the release tag at the exporter's pinned
   revision, since a package resolved by version cannot depend on a revision. Rust's
   native link libraries become linker settings, per platform where they differ; any
   other flag fails the build.
7. Go: one module for every target, at the module path and `v<version>`, the major
   version's `/vN` suffix checked: the export's `go/`, whose runtime's `#cgo` lines link
   `rt/lib<prefix>_<goos>_<goarch>.a` for `linux/amd64` and `linux/arm64` (built against
   glibc 2.28), `darwin/arm64` and `ios/arm64` (the Swift package's macOS and iOS device
   libraries; no consumer here runs iOS), and `windows/amd64` (an
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
8. The release's `api.json`, and `changes.md`, the items removed, changed, and added since
   the previous release's, by kind and Rust path; the caller's release publishes the first
   as an asset and the second as its notes.
9. Consumers, with no checkout, Rust, or source path: each wheel installed with
   `--no-index --no-deps` into a fresh environment on its oldest CPython, run with `-I`
   from an empty directory; and the Swift package built by its own scheme for macOS, iOS,
   and the iOS Simulator, then consumed by a new executable package that runs the check;
   and the Go module, unpacked from its zip into a new module that requires it, vetted,
   built with the newest Go and no `CGO_LDFLAGS`, and run on Linux x64 and arm64, macOS, and
   Windows. With an `example`, each must print native Rust's stdout byte for byte.

rust2 runs it on every manual Check and nightly through
[`bindings-test.yml`](/.github/workflows/bindings-test.yml): on the api fixture, whose
wheels must print its native transcript, and on moq's stand-in facade at the exporter's moq
pin, through `moq/moq.py`, `moq/moq.swift`, and `moq/moq.go`.

## Other registries

The same boundary holds for each destination: the builder makes and checks the artifact
from Cargo facts, the caller's workflow names the identity and credential. A language whose
generator rust2-ffi lacks, such as Node or C#, is unsupported, not a package without its
binding.

| Destination | Builder work | What only the caller can say |
| --- | --- | --- |
| Maven Central | An AAR with classes.jar, JNI libraries per Android ABI and the generated R8 rules; POM, sources and javadoc jars | The verified groupId namespace, publisher authorization, and signing key |
| pub.dev | The generated Dart library with a native-assets build hook and per-platform archives, pubspec, and license | Publisher authorization and the archives' download host |
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
