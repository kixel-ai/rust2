#!/usr/bin/env bash
# Build a new executable package whose only library dependency is the artifact.
set -euo pipefail
artifacts=$(cd "$1" && pwd)
cd "$artifacts"
consumer=$(mktemp -d consumer.XXXXXXXX)
cd "$consumer"
mkdir -p Sources/Check
cp "$artifacts/consumer.swift" Sources/Check/check.swift
python3 - <<'PYTHON'
from pathlib import Path
import re
revision = Path('../Api/collections-revision.txt').read_text().strip()
assert re.fullmatch(r'[a-f0-9]{40}', revision)
Path('Package.swift').write_text('''// swift-tools-version: 6.2
import PackageDescription
let package = Package(
    name: "Check", platforms: [.macOS(.v15)],
    dependencies: [
        .package(path: "../Api"),
        .package(url: "https://github.com/apple/swift-collections", revision: "''' + revision + '''")
    ],
    targets: [.executableTarget(name: "Check", dependencies: [
        .product(name: "Api", package: "Api"),
        .product(name: "BasicContainers", package: "swift-collections")
    ])]
)
''')
PYTHON
swift build -c release
.build/release/Check >actual.txt
cmp "$artifacts/native.txt" actual.txt
.build/release/Check futures
.build/release/Check owned
