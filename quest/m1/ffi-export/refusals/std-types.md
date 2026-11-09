# [M] Standard and common types export by role

## Goal

The census at the moq pin refuses no item of moq's own for naming a standard or common
foreign type: 77 items at moq `ffcb7cd9`, less the 23 `Poll` and `Context` ones, which
export as futures and wakers. Each exports by its role, as
`Duration` and `String` already do: `BTreeMap` (20) and `BTreeSet`, `anyhow::Error` (12)
and `io::Error` (5), `NonZero` (5), `IpAddr`, `Instant`, `UdpSocket`, `tracing::Level`,
`serde_json::Map`, and iterator adapters such as `Audio::enabled`'s `Filter`.

## Plan

Ranked by moq-ffi's wrapper: hang's `renditions` maps, which moq-ffi's `media.rs` reads, and
moq-mux's `ts::Import` errors first. A map crosses as the sequence of pairs each language
builds its map from; an error crosses as its message, as `Display` already gives other
errors; an iterator adapter crosses as the sequence it yields.
