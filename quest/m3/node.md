# [L] A Node-API addon

## Goal

A report, then a binding if the report earns it: the `api` fixture and moq's frame
workloads called from Node three ways, through [rust2-wasm](/quest/m2/wasm/README.md),
through Node's built-in `node:ffi` over the existing C header, and through a native
Node-API addon the crate builds with `--features node`, as Python's CPython extension is
built. The addon lands as a generator only where it beats both others on
[the binding benchmark](/bench/ffi/README.md); otherwise this quest is deleted with its
numbers in the PR.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ planned in m3 before a
consumer asks ([m3](/quest/m3/README.md#plan)).

- rust2-wasm already loads in Node, Bun, and Deno, so a native addon must earn its cost
  (a 3,000 to 5,000 line generator, and a binary per platform) in speed or in what wasm
  cannot do: threads, sockets, and the file system, which a library such as moq-tokio
  needs for native QUIC.
- `node:ffi` is experimental since Node 26.1 (2026-05), and Bun and Deno have their own
  FFI; Bun's docs call its `bun:ffi` not production-ready. If a runtime's own FFI over the
  header is fast enough, a thin generated TypeScript file over it replaces the addon.
- No tool was found that generates Node-API bindings from a C header; napi-rs is
  annotation-based.
- Node-API is ABI-stable across Node versions and is what Bun and Deno load too, so one
  addon serves all three.

Public API: the generated TypeScript API and the `node` feature, if it lands.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - the C ABI, its lossless types, and its checks are settled before a new generator copies them
- [The exporter's wasm32 target](/quest/m2/wasm/backend.md) - the route the addon is measured against, whose TypeScript API it should match
