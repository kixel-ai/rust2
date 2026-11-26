# rust2

Rust to other places.

- **TypeScript**: an own rustc driver that translates a [documented Rust subset](ts/README.md) to TypeScript, without WASM.
- **C, then any language**: an [exporter](ffi/README.md) that turns a crate's `pub` API into a C ABI and header with no attributes, sharing the TypeScript driver's front end, and generates bindings over it in [C++](ffi/README.md#c), [Kotlin](ffi/README.md#kotlin), [Python](ffi/README.md#python), [Swift](ffi/README.md#swift), [Go](ffi/README.md#go), [Dart](ffi/README.md#dart), and [TypeScript for the web](ffi/README.md#typescript-web). The flake's `ffi` package is the one to pin. Its [release builder](ffi/release/README.md) turns an export into checked Python wheels and a Swift package, published by the crate's own workflow.
- **Public-visibility audit**: [rust2-pub](driver/pub/README.md), on the drivers' front end, finds the `pub` items no other crate needs and validates narrowing them.
- **Faster builds**: a reproducible [build-speed benchmark](bench/build/README.md) of upstream's levers on moq.
- **Benchmarks**: [rust2-bench](bench/runner/README.md) counts each benchmark's instructions under stock Valgrind, in parallel, for Kixel Quest and for a local base-versus-head compare, on x86_64 Linux hosts. Its [harness](bench/runner/harness/README.md) replaces Criterion: benchmarks are plain functions marked `#[bench]`.

Everything here is early; the [quest tree](quest/README.md) holds the plan.

## Development

```bash
nix develop   # or direnv allow
just ci       # just check, then just test, as the merge queue runs them
```

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
