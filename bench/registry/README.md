# Registry packaging spike

Recommendation: generate release templates for moq. A single configless publisher
cannot infer registry ownership from Cargo metadata. Shared package builders are
still useful. This spike publishes nothing and changes no binding semantics.

`build.sh` exports the existing api fixture (including its existing exclusion file),
builds a CPython stable-ABI library, writes a wheel, and on macOS wraps a separate
C static library in an XCFramework with a Swift source package. The Swift wrapper
stays source so the consumer compiles it with its compiler; the Rust library is
binary. The XCFramework currently has only the build host's macOS slice. This is
not an iOS or universal distribution claim.

Run from the repository root through `quest gate nix develop --command
bench/registry/build.sh`. Output is in `ffi/target/registry`. Install the wheel in a
fresh Python environment, then run `consume.py <artifact-directory>` with that
environment's Python from outside the checkout. The consumer compares stdout
byte for byte with the fixture's native Rust transcript, then exercises futures,
ownership, integer values and calls. The wheel contains the generated module as
`api/__init__.py` and its library beside it, with a complete hashed RECORD.

On macOS, unpack `Api.zip` beside a fresh consumer directory and run
`consume-swift.sh <artifact-directory>` from there. The script uses SwiftPM to
resolve the package, runs the same transcript against Rust, then futures and
ownership checks. Swift 6.2 or newer and macOS 15 are required for the fixture's
executor APIs. BasicContainers is pinned to the exporter's swift-collections
revision. No registry credentials, signing keys or publishing permissions are used.

## Scope and evidence

The fixture is intentional: this measures packaging an existing export, not finding
an arbitrary facade in a workspace. Name, version and description come from its
Cargo manifest; the library's distinct `api` name is respected. The fixture is
`publish = false`; this spike does not upload it anywhere. Existing binding
fixtures own semantic differential tests; these consumers check that packaging
preserves the generated API and runtime loading.

The native wheel tag only claims the build host's platform. Linux packaging removes
RUNPATH and checks that the only dependencies are glibc, its loader and libgcc_s,
with no symbol newer than Ubuntu 24.04's glibc 2.39. Anything else fails packaging.
This is checked on each actual architecture; it is not a manylinux claim. macOS
uses a major_0 tag (the form pip accepts on macOS 11+) and the host architecture. A release needs
manylinux builds and auditwheel repair on Linux, delocate on macOS, and DLL
inspection on Windows, including transitive native dependencies. Stable ABI alone
does not establish OS portability. The generated extension requests abi3-py311,
so the wheel uses cp311-abi3 and Requires-Python >=3.11.

Linux x86_64 execution passed on 2026-11-13 with CPython 3.14.10: the built
`api_fixture-0.0.0-cp311-abi3-linux_x86_64.whl` installed into a fresh uv-created
virtual environment with no index or dependencies. Running from a clean directory
with `-I` matched the native Rust transcript byte for byte and passed futures,
ownership, values and calls. A separate ZIP inspection checked RECORD coverage,
file sizes and SHA-256 hashes. The full `quest gate nix develop --command just ci`
passed, including the exporter's existing Python and Swift differential tests.

The export includes host and wasm32 API descriptions, as the existing fixture
suite does: its values test checks that wasm32-only calls are refused on the host.
Only the host native library is packaged. Initial export attempts exposed missing
CACHEDIR.TAG markers in imported local build artifacts; restoring those markers
allowed the unchanged exporter to run. No cache tools or checks were disabled.

macOS execution is unverified: this session's host is Linux and `ssh kixtop uname
-a` timed out. Do not delete moq's release workflows on this evidence.

## What each registry still needs

| Destination | Package work | Information Cargo cannot establish |
| --- | --- | --- |
| PyPI | Wheel matrix, dependency repair, name/version normalization, license metadata, sdist policy | Ownership of the distribution name and trusted-publisher registration |
| SwiftPM | macOS/iOS device/simulator slices, merged exports for cfg differences, minimum OS versions, checksummed hosted zip and release Package.swift | Artifact hosting URL and repository/tag policy; code signing identity if signing is required |
| Maven Central | AAR with classes.jar, JNI libraries per Android ABI, generated R8 rules, POM, sources/javadoc artifacts and signatures | Verified groupId namespace, publisher authorization and signing key |
| npm | A Node binding backend (not currently generated), platform packages and JS loader, TypeScript declarations | Scope/name ownership and trusted-publisher registration |
| NuGet | A C# binding backend (not currently generated), managed assembly plus RID-specific native assets | Package ID ownership, credentials and desired managed target frameworks |
| pub.dev | Generated Dart library plus native asset hooks and platform archives, pubspec and license files | Publisher authorization and the distribution/download host |
| Go | Generated cgo packages, module-relative imports and native library delivery for supported targets | Canonical module path, repository/subdirectory and release tags (including major-version suffixes) |

Cargo's repository URL can suggest a Go module path or Maven namespace. It cannot
prove either is the publisher's intended existing identity. Cargo package names
also need not equal existing PyPI names or npm scopes. A manifest's license and
repository fields are optional; absence must produce a packaging error where the
registry requires them. Authentication setup is an external platform boundary,
not something a workflow can manufacture from a crate.

For moq, the existing release-py-ffi, release-swift-ffi and release-swift-lib
workflows additionally encode their current names, hosting and platform policy.
Replacing those workflows requires preserving those contracts and testing moq,
not just this fixture. The quest's go condition is therefore not established;
templates are the recommended no-go outcome even if both artifact consumers pass.

## Prior art and primary sources

- [BoltFFI](https://github.com/boltffi/boltffi): annotation-based generation and local packaging are adjacent to this problem; do not treat it as evidence of registry ownership automation.
- [maturin](https://www.maturin.rs/distribution.html): Python distribution tooling can own wheel repair/build concerns but does not solve the other registries.
- [Wheel format](https://packaging.python.org/en/latest/specifications/binary-distribution-format/): tags, metadata and RECORD.
- [Apple binary packages](https://developer.apple.com/documentation/Xcode/distributing-binary-frameworks-as-swift-packages): XCFramework binary targets and hosted artifact checksums.
- [Maven Central requirements](https://central.sonatype.org/publish/requirements/) and [namespace registration](https://central.sonatype.org/register/namespace/).
- [npm trusted publishing](https://docs.npmjs.com/trusted-publishers/) and [scopes](https://docs.npmjs.com/about-scopes/).
- [NuGet native assets](https://learn.microsoft.com/nuget/create-packages/native-files-in-net-packages).
- [Dart hooks](https://dart.dev/tools/hooks) and [publishing](https://dart.dev/tools/pub/publishing).
- [Go module publishing](https://go.dev/doc/modules/developing).

The nightly workflow calls the same reusable workflow, outside the merge gate.
Consumer workspaces are unique per run attempt and removed in an always-run
cleanup step, including on self-hosted runners.
Builders replace their own generated output; Swift consumers create a fresh package
beside the artifact regardless of the invoking directory. Rust's reported native
link libraries become SwiftPM linker settings; unrecognized flags fail packaging.
The collections revision is read from ffi/justfile and carried inside the package.

Registry requirements checked on 2026-11-13. The table is a design assessment,
not a claim that any of those unimplemented publishers was exercised.

## Reusable workflow

`.github/workflows/registry.yml` is callable with no inputs or secrets. It builds
this fixed fixture on Linux arm64 and macOS arm64 and passes only artifacts to
fresh consumer jobs. A caller needs just a job with
`uses: kixel-ai/rust2/.github/workflows/registry.yml@<reviewed-commit>`.
External calls pin the fixture, exporter and packaging scripts together to an
immutable rust2 revision, independently of the caller's checkout. Calls within
rust2 test the triggering commit so nightly catches changes on main. It is a spike
runner, not a publisher of the caller's crate. Private callers select self-hosted
runners with OS/architecture labels. Public callers use hosted runners.

These two targets overlap crabx's planned matrix. The pipeline quest has not
landed a shared matrix to reuse. Extending this to stock crates requires facade
discovery and packaging of moq's existing identity; neither is claimed here.
