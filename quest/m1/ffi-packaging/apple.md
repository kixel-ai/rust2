# [S] Apple: privacy manifest, Mac Catalyst, and dSYMs

## Goal

The XCFramework carries a privacy manifest declaring the required-reason APIs its library
links, found by `llvm-nm` rather than listed by hand, adds a Mac Catalyst slice, and ships
dSYMs, so an app embedding it passes App Store upload checks and symbolicates crashes.

## Plan

Found by the 2026-11-18 language audit: Rust's std links `stat`-family symbols and
possibly `mach_absolute_time`, which Apple's upload scan rejects without declarations
(ITMS-91053); the XCFramework holds macOS arm64, iOS arm64, and the iOS simulator only.
visionOS and tvOS need `build-std`; leave them out until a consumer asks. CocoaPods trunk
goes read-only on 2026-12-02, so SwiftPM alone stays right.

Public API: the XCFramework's slices.
