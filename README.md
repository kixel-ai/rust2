# rust2

Rust to other places.

- **TypeScript**: an own rustc driver that translates a [documented Rust subset](ts/README.md) to TypeScript, without WASM.
- **C, then any language**: an [exporter](ffi/README.md) that turns a crate's `pub` API into a C ABI and header with no attributes, sharing the TypeScript driver's front end, and generates bindings over it in [C++](ffi/README.md#c), [Kotlin](ffi/README.md#kotlin), [Python](ffi/README.md#python), [Swift](ffi/README.md#swift), [Go](ffi/README.md#go), [Dart](ffi/README.md#dart), and [TypeScript for the web](ffi/README.md#typescript-web). The flake's `ffi` package is the one to pin. Its [release builder](ffi/release/README.md) turns an export into checked Python wheels and a Swift package, published by the crate's own workflow.
- **Lean 4**: [rust2-lean](driver/lean/README.md), a spike on the drivers' front end, translates a pure Rust subset to Lean 4 for proofs.
- **Public-visibility audit**: [rust2-pub](driver/pub/README.md), on the drivers' front end, finds the `pub` items no other crate needs and validates narrowing them.
- **Bounded `cargo test`**: [rust2-watchdog](watchdog/README.md) runs `cargo test` or `cross test` under the workspace's nextest bounds, killing the run when one test passes its bound and naming that test, for targets where nextest's process per test is dear, such as the iOS simulator.
- **Faster builds**: a reproducible [build-speed benchmark](bench/build/README.md) of upstream's levers on moq.
- **Benchmarks**: [swole](https://github.com/kixel-ai/swole), which counts the [binding benchmark](bench/ffi/README.md)'s instructions, lives in its own repository; `just ffi bench` times it.

Everything here is early; the [quest tree](quest/README.md) holds the plan.

## Development

```bash
nix develop   # or direnv allow
just ci       # just check, then just test, as the merge queue runs them
```

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
