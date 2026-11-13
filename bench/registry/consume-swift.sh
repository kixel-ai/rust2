#!/usr/bin/env bash
# Build a new executable package whose only library dependency is the artifact.
set -euo pipefail
artifacts=$(cd "$1" && pwd)
mkdir consumer
cd consumer
mkdir -p Sources/Check
cp "$artifacts/consumer.swift" Sources/Check/check.swift
cat >Package.swift <<'SWIFT'
// swift-tools-version: 6.2
import PackageDescription
let package = Package(
    name: "Check", platforms: [.macOS(.v15)],
    dependencies: [
        .package(path: "../Api"),
        .package(url: "https://github.com/apple/swift-collections", revision: "f96b909e1a9e469620bc5037c1e24e00153f376f")
    ],
    targets: [.executableTarget(name: "Check", dependencies: [
        .product(name: "Api", package: "Api"),
        .product(name: "BasicContainers", package: "swift-collections")
    ])]
)
SWIFT
swift build -c release
.build/release/Check >actual.txt
cmp "$artifacts/native.txt" actual.txt
.build/release/Check futures
.build/release/Check owned
