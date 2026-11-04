# rust2

Rust to other places.

- **TypeScript**: an own rustc driver that translates a [documented Rust subset](ts/README.md) to readable TypeScript, without WASM.
- **C, then any language**: an [exporter](ffi/README.md) that turns a crate's `pub` API into a C ABI and header with no attributes, sharing the TypeScript driver's front end.
- **Any language**: [a hard fork](uniffi/README.md) of [UniFFI](https://github.com/mozilla/uniffi-rs) with every generator [moq](https://github.com/moq-dev/moq) uses.
- **Faster builds**: a reproducible [build-speed benchmark](bench/build/README.md), then a pure-Rust subset (no build scripts, no C linking) that builds faster, and [rust2-lint](driver/src/bin/rust2-lint/main.rs), which fails a build whose generics' copies cost its codegen more than a budget (`nix run github:moq-dev/rust2#lint -- --help`).

Everything here is early; the [quest tree](quest/README.md) holds the plan.

## Development

```bash
nix develop   # or direnv allow
just ci       # just check, then just test, as the merge queue runs them

# The UniFFI fixture suites need every foreign toolchain.
nix develop .#uniffi --command just uniffi fixtures
```

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option, unless a directory's own license says otherwise (the UniFFI fork keeps its upstream licenses: MPL-2.0, and MIT for the Dart generator).
