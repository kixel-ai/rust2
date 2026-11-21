# [S] The api fixture's Dart, Go, and Swift packages print its transcript

## Goal

The builder's Dart, Flutter, Go, and Swift consumers of the api fixture run its checks,
`tests/api.dart`, `api.go`, and `api.swift`, through the released packages and print
native Rust's transcript byte for byte, as its Python, C++, and web consumers do since
[#930](https://github.com/kixel-ai/rust2/pull/930). Today they only load the library.

## Plan

Found by [#915](https://github.com/kixel-ai/rust2/pull/915), which adds the Dart package
and its `dart-check` input: #930 left `api.dart` to it, but each of the three checks names
wasm32-only items (`targets()` in each, `api.wasmOnly()` and the like). The builder's api
fixture export holds no wasm32 API, so Dart, Go, and Swift, which resolve names when they
compile, refuse the check; Python and C++ reach those names only at run time.

Recommended: move each check's wasm32 cases into a file of their own that the exporter's
tests compile beside the check, so the check compiles against any export, and give
`dart-check`, `go-check`, and `swift-check` #930's directory form, which
`just ffi transcripts` fills with each check and its generated script.
Rejected: exporting the builder's api fixture for wasm32 too, which would put a target no
native package ships into every package's API.
