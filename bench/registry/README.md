# Registry packaging spike

Accepted direction: generate release templates for moq. A single configless publisher
cannot infer registry ownership from Cargo metadata. Shared package builders are
still useful. This spike publishes nothing and changes no binding semantics.

The maintainer [accepted the no-go on 2026-11-16](https://github.com/kixel-ai/rust2/pull/761#issuecomment-6930429582):
release templates with shared builders are the full version (rejected: keep
investigating a configless publisher; stop the spike with no full version).
The [follow-up quest](/quest/m2/release-templates.md) starts with moq's existing
Python and Swift releases and preserves their identities and platform policy.

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

The export includes host and wasm32-wasip1 API descriptions, as the existing fixture
suite does: its values test checks that wasm32-only calls are refused on the host.
Only the host native library is packaged. Initial export attempts exposed missing
CACHEDIR.TAG markers in imported local build artifacts; restoring those markers
allowed the unchanged exporter to run. No cache tools or checks were disabled.

macOS 26.6.2 arm64 execution passed on kixtop on 2026-11-16, with Xcode 26.6 and
Apple Swift 6.3.3. In an isolated checkout, `nix develop --command quest gate
bench/registry/build.sh` built `Api.zip`. From a fresh artifact directory containing
only the ZIP, native transcript, Swift consumer and verifier, `unzip -q Api.zip`
and the verifier built a release SwiftPM consumer with Apple's compiler. It ran
through `nix develop --command quest gate env -i HOME=/Users/kixelated
PATH=/usr/bin:/bin:/usr/sbin:/sbin /bin/bash <artifact-directory>/verify-swift.sh
<artifact-directory>`, so the consumer inherited only the host home and Apple
toolchain search path. The transcript matched byte for byte, followed by
`futures ok` and `owned ok`. Both transcripts had SHA-256
`729ffddcec8928c564d29fd1ee3c4fc34ccaecf18900d34be63c4e6fd0a50714`.
`plutil -p Api/ApiFFI.xcframework/Info.plist` and `lipo -info
Api/ApiFFI.xcframework/macos-arm64/libapi.a` independently confirmed one macOS arm64
slice.

The macOS builder selects the installed Xcode and its SDK explicitly: Nix's SDK
bundle lacks `libz.tbd` and cannot serve as Xcode's developer directory. The current
spike exports the same WASI target as the fixture suite, so target availability
agrees, and the Swift fixture avoids shadowing its `debug()` helper in Swift 6.
The original external-call pin, commit `901f7baa`, also passed Swift validation; that
older fixture needs an explicit `@Sendable` type on its duration-conversion helper.
After merging main to resolve the fixture conflict, the current source passed
again with the transcript hash above. Apple Swift still warns about a pointer
capture in the fixture's separate shared-call stress test; the consumer exercises
the transcript, futures and ownership, not that stress test.
The first GitHub reusable invocation,
[run 43968322384](https://github.com/kixel-ai/rust2/actions/runs/43968322384),
built both platforms and passed the clean Linux consumer. The macOS wheel
installed but could not import: Nix's linker wrapper embedded a Nix-store
libiconv dependency absent from the consumer runner. The builder now uses
Apple's linker, and macOS wheel packaging inspects `otool -L` and refuses any
dependency outside system libraries and frameworks. Its dylib ID is relative.
The next GitHub invocation passed both artifact consumers, as recorded below.
Do not delete moq's release workflows on this evidence.

## GitHub invocation and artifact handoff

On 2026-11-16, ran `gh workflow run check.yml --repo kixel-ai/rust2 --ref
quest/m2/registry`. The registered manual Check workflow called
`./.github/workflows/registry.yml` at
`13710f7b7f1dcc649fa5cdabff348b497bc835ff` in
[run 43971814703](https://github.com/kixel-ai/rust2/actions/runs/43971814703).
Direct dispatch of `registry.yml` returned 404 because that new workflow was not
yet registered on the default branch. The manual caller also runs the existing
full Check job; nightly's report now includes the reusable job's result.

Both builders passed and uploaded `registry-ubuntu-24.04-arm` and
`registry-macos-26`. Separate jobs downloaded these artifacts without checkout,
Nix, Rust, exporter or source paths. They created a venv, installed the wheel
with `pip install --no-index --no-deps artifacts/*.whl`, and ran the copied
verifier with Python's `-I` from a clean directory.
[Linux arm64](https://github.com/kixel-ai/rust2/actions/runs/43971814703/job/133671565856)
and [macOS arm64](https://github.com/kixel-ai/rust2/actions/runs/43971814703/job/133671565931)
each printed `installed wheel: transcript matches Rust; futures, ownership,
values and calls pass`. The macOS 26.7.2 consumer then unpacked `Api.zip` and ran
`bash verify-swift.sh .`: SwiftPM's release build passed, its transcript matched
Rust byte for byte, then `futures ok` and `owned ok` passed. Both consumer jobs
removed their workspaces successfully.

The Mac builder's independent `otool -L` check reported only the relative dylib
ID, `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`. The external-call
fixture pin now uses this validated revision, including the Apple linker fix.
Downloaded both uploaded artifacts with `gh run download 43971814703 --repo
kixel-ai/rust2 --dir .tmp/registry-verified-artifacts`: independent ZIP inspection
verified both wheels' complete RECORD coverage, sizes and SHA-256 hashes, and
the XCFramework plist described exactly one macOS arm64 slice. Both packaged
native transcripts had SHA-256
`729ffddcec8928c564d29fd1ee3c4fc34ccaecf18900d34be63c4e6fd0a50714`.
This completes the fixture-packaging spike and establishes the artifact handoff,
not a configless publisher or moq release migration.

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
