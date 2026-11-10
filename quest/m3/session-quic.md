# [M] Sessions on sans-IO QUIC

## Goal

A binding runs moq sessions over moq-quic's sans-IO QUIC with no runtime in the library:
the library owns QUIC and TLS, and the binding moves UDP datagrams and drives the session's
poll from its own event loop, as driving decided for the origin's
`Driver`. Proven from C in `ffi/moq/moq.c` against a local server.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ m3, out of
[the exporter epic](/quest/m1/ffi-export/README.md), which it would have held open until
moq's m3 driver lands. Rejected: m2; a top-level m1 quest.

Decided with the maintainer on 2026-10-27: the full-control path, after sessions on
moq-tokio, which `ffi/moq/moq.c` proves with moq-tokio's fallback runtime. It suits consumers
with their own event loop or sockets.

Decided with the maintainer on 2026-10-28, from
[#215](https://github.com/moq-dev/rust2/pull/215): at moq `30f25c04`, only moq-tokio's
and moq-uring's drivers join moq-quic to moq-net's `poll::Session`, and each brings a
runtime. The runtime-free driver that joins them is moq's change, not the exporter's.
Datagrams move by pull: the binding passes in each one it receives, polls on each wake and
at the deadline the poll returns, and drains the owned datagrams to send. No callback
crosses the ABI, and no buffer is lent mutably.

Then bump the moq pin and export the driver from its crate's `pub` API, with no wrapper.
At `30f25c04`, the census exports 60% of moq-quic's types and 86% of its methods, and
none of its traits. It refuses `Endpoint::handle`, `connection::Connection`, `Transmit`,
and `poll_transmit`'s `&mut Vec<u8>`. Those figures predate the pin at `ffcb7cd9`; measure
them again at the start, and report the driver crate's coverage before and after.

Public API: the C names moq's bindings get.

## Waiting

- moq-dev/moq: the [runtime-free QUIC driver](https://github.com/moq-dev/moq/blob/main/quest/m3/quic-sansio-driver.md),
  planned in [moq-dev/moq#7197](https://github.com/moq-dev/moq/pull/7197), lands. moq ranks
  it in m3, after its next breaking release. Done once that quest is gone from moq's `main`
  and the driver's connections implement moq-net's `transport::poll::Session`.
