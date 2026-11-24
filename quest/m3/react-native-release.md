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
- Recommended: the package is named `@<scope>/<crate>-react-native` beside the web
  package's `@<scope>/<crate>`, since one npm name cannot hold both a wasm32 module and a
  TurboModule.

## Waiting

- Maintainer: name the package `@<scope>/<crate>-react-native` (recommended) or something
  else, such as `@<scope>/<crate>` with the web package renamed instead; an agent can't
  pick a published name. Check: this item is gone from the quest.
