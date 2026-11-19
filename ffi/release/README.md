# Binding releases

A crate's [rust2-ffi](/ffi/README.md) export, released as Python wheels and a Swift
package. Two halves, split where Cargo's knowledge ends:

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
first attempt left, uploading only what is missing. A rebuild makes the same bytes
([Reproducible builds](#reproducible-builds)), so **Re-run all jobs** works as well.
A Cargo prerelease, such as `1.0.0-rc.1`, is a GitHub prerelease too.

The builder takes the crate's `Cargo.toml` and five optional inputs the caller's tests own:
`features` to ship, an `example` whose stdout is native Rust's transcript, the
`python-check` and `swift-check` programs that must print it through the installed
packages, and `reproduce`, which builds everything twice. Without a check, a consumer only
imports the module.

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
   every object's declared minimum OS; one XCFramework of the three, zipped by the builder itself,
   its SHA-256 checked against `swift package compute-checksum`; and a `Package.swift`
   naming it at `https://github.com/<repo>/releases/download/<crate>-v<version>/<Module>FFI.xcframework.zip`.
   The package depends on swift-collections by the release tag at the exporter's pinned
   revision, since a package resolved by version cannot depend on a revision. Rust's
   native link libraries become linker settings, per platform where they differ; any
   other flag fails the build.
6. Consumers, with no checkout, Rust, or source path: each wheel installed with
   `--no-index --no-deps` into a fresh environment on its oldest CPython, run with `-I`
   from an empty directory; and the Swift package built by its own scheme for macOS, iOS,
   and the iOS Simulator, then consumed by a new executable package that runs the check.
   With an `example`, each must print native Rust's stdout byte for byte.

rust2 runs it on every manual Check and nightly through
[`bindings-test.yml`](/.github/workflows/bindings-test.yml): on the api fixture, whose
wheels must print its native transcript, and on moq's stand-in facade at the exporter's moq
pin, through `moq/moq.py` and `moq/moq.swift`.

## Reproducible builds

A rebuild of a commit makes the same bytes, so **Re-run all jobs** finds each registry
holding exactly the files it would upload, and anyone with the commit can check them.

- **Time.** Every file takes `SOURCE_DATE_EPOCH`, or else the commit's time, whatever its
  archive format: the builder writes each ZIP itself, with sorted entries, fixed modes, and
  no host, umask, or time of its own, and rewrites each repaired wheel the same way, its
  RECORD rebuilt. Zlib's own output is the one thing a host could still change.
- **Paths.** The crate's workspace, the exported crate, and `CARGO_HOME` are remapped to
  fixed names for rustc and the C compilers, so panic messages and debug info name no
  checkout. This takes `RUSTFLAGS` over a `.cargo/config.toml`'s `rustflags`, so a config
  that sets any fails the build: export them as `RUSTFLAGS`, which the builder keeps. Linux
  and macOS builds also stamp no archive time (`ZERO_AR_DATE`), and Windows links with
  `/Brepro`.
- **Checked.** The Swift job builds the XCFramework twice and fails unless both ZIPs match.
  With `reproduce: true`, every build host builds a second time, from a new export, and
  `package.py same` fails unless each wheel and static library matches byte for byte; rust2
  turns it on for its api fixture. It doubles the build, so a large crate may leave it to a
  nightly.
- **Not yet.** The exporter writes the crate's absolute path into the generated crate's
  version, which cargo hashes into every symbol, so a rebuild must keep the crate at the
  same path, as a rerun does
  ([planned](/quest/m1/ffi-packaging/relocatable.md)).

**Attestations and SBOM.** `package.py build` writes `sbom.cdx.json`, a CycloneDX SBOM of
the crates the binding links (normal dependencies, platform-neutral, with each registry
crate's checksum from `Cargo.lock`), which every wheel also carries in
`.dist-info/sboms/` (PEP 770) and the build hosts must agree on. The caller's `attest` job
runs on a tag, before anything is published, and gives every wheel and the XCFramework a
build provenance attestation and an SBOM attestation, which `gh attestation verify
<file> --repo <owner>/<repo>` checks. Only that job holds `id-token` and `attestations`
write; the builder stays read-only. PyPI's trusted publishing uploads its own PEP 740
attestations for the wheels as well.

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
