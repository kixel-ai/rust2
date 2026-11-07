# [XL] Every public item exports

## Goal

[rust2-ffi](/ffi/README.md) exports every `pub` item of moq's crates, so `just ffi census`
is empty at the moq pin and `ffi.toml` is deleted: the exporter reads no config, and an
item it cannot export fails the export with a span.

## Plan

Decided with the maintainer on 2026-11-04: configless, and an unsupported public item
fails the export rather than being excluded. ✅ Split: the naming rule
first, then this. Rejected: one quest that waits for the census to reach zero.

Today `ffi/moq/ffi.toml` excludes about 300 refused items across moq-net, moq-pattern,
moq-clock, moq-tokio, hang, moq-mux, and kio. ✅ Plan the children later: the first agent runs the
census and splits it by refusal kind into an epic through `/quest-plan`, ranked by what
moq-ffi's hand-written wrapper still covers. This quest's own remaining work, deleting
`ffi.toml` and the config reader, stays in that epic's README.

Public API: the exporter's command line and the C API of every item that exports.
