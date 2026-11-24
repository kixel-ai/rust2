# [M] React Native package from the release builder

## Goal

[The release builder](/ffi/release/README.md) packs an export's React Native package,
`react-native/`, with its libraries built for each ABI it serves: Android's four in
`libs/android/<abi>/`, and iOS's device and simulator in `libs/ios/<prefix>.xcframework`.
It checks the packed package in a new React Native app built for release, as the nightly's
device tests do, and the release workflow `package.py init` writes publishes it to npm.

## Plan

- No rule of [every binding's](/ffi/README.md#every-binding) requires it: the binding's
  own PR left it out. moq, the first consumer, ships each binding it offers through the
  builder, so its React Native users would install this package from npm.
- The builder already builds Android's ABIs for the Kotlin library and an XCFramework for
  Swift: the package takes the same static libraries, so no target is new.
- Decided 2026-11-24 (maintainer, /quest-run): the package is named `@<scope>/<crate>-rn`,
  such as `@moq/moq-rn` beside the web's `@moq/moq`, since one npm name cannot hold both a
  wasm32 module and a TurboModule. Rejected: `@<scope>/<crate>-react-native`, and giving
  React Native `@<scope>/<crate>` with the web package renamed.
