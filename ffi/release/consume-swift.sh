#!/usr/bin/env bash
# Consume a staged Swift package as a user would, before any upload: its own scheme built
# for macOS, iOS, and the iOS Simulator at the floors it declares, then a new executable
# package whose only library is it, run and compared with native Rust.
# usage: consume-swift.sh <bindings-swift artifact> <bindings-build-apple artifact>
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
swift=$(cd "$1" && pwd)
apple=$(cd "$2" && pwd)
module=$(python3 -I -c 'import json, sys; print(json.load(open(sys.argv[1]))["module"])' "$swift/swift.json")
work=$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/swift-consumer.XXXXXXXX")
trap 'rm -rf "$work"' EXIT

# The ZIP a release hosts, unpacked as SwiftPM would after checking its checksum.
[[ "$(shasum -a 256 "$swift/${module}FFI.xcframework.zip" | cut -d' ' -f1)" == "$(cat "$swift/checksum.txt")" ]]
cp -R "$swift/package" "$work/$module"
# SwiftPM takes a local binary target only from inside the package.
(cd "$work/$module" && unzip -q "$swift/${module}FFI.xcframework.zip")
python3 -I "$here/package.py" swift-local --state "$swift/swift.json" --package "$work/$module" --framework "$work/$module/${module}FFI.xcframework"

(
    cd "$work/$module"
    for destination in 'generic/platform=macOS' 'generic/platform=iOS' 'generic/platform=iOS Simulator'; do
        xcodebuild -quiet -scheme "$module" -destination "$destination" -derivedDataPath "$work/derived" \
            CODE_SIGNING_ALLOWED=NO build
    done
)

mkdir -p "$work/consumer/Sources/Check"
if [[ -f "$apple/checks/main.swift" ]]; then
    cp "$apple/checks/main.swift" "$work/consumer/Sources/Check/Check.swift"
else
    printf 'import %s\n\n@main\nstruct Check {\n\tstatic func main() {}\n}\n' "$module" >"$work/consumer/Sources/Check/Check.swift"
fi
collections=$(python3 -I -c 'import json, sys; print(json.load(open(sys.argv[1]))["collections"])' "$swift/swift.json")
# The check program may use newer APIs than the package needs, such as an executor's, and
# builds in Swift 5's language mode, as rust2's own tests build it; the package builds in 6.
cat >"$work/consumer/Package.swift" <<SWIFT
// swift-tools-version: 6.2
import PackageDescription

let package = Package(
    name: "Check",
    platforms: [.macOS(.v15)],
    dependencies: [
        .package(path: "../$module"),
        .package(url: "https://github.com/apple/swift-collections", exact: "$collections"),
    ],
    targets: [
        .executableTarget(
            name: "Check",
            dependencies: [
                .product(name: "$module", package: "$module"),
                .product(name: "BasicContainers", package: "swift-collections"),
            ],
            swiftSettings: [.swiftLanguageMode(.v5)]
        ),
    ]
)
SWIFT
cd "$work/consumer"
swift build -c release
.build/release/Check >"$work/actual.txt"
if [[ -f "$apple/native.txt" ]]; then
    python3 -I "$here/package.py" compare "$apple/native.txt" "$work/actual.txt"
fi
echo "installed Swift package: builds for macOS, iOS and the iOS Simulator, and its check passes"
