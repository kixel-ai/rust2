# [M] tokio's sync channels mapped into the runtime

## Goal

tokio's `sync::mpsc` (bounded and unbounded), `sync::watch`, and `sync::oneshot`, and
`tokio::select!`'s support, are mapped into the [rust2-ts](/ts/README.md) runtime, as
`hashbrown` and `bytes` are, so a crate passing values through them translates with no
`ts.toml` binding. Differential tests against native Rust on tokio's current-thread
runtime prove it: values, ordering, closed and lagged errors, and when each future
becomes ready.

## Plan

Suggested on [#638](https://github.com/moq-dev/rust2/pull/638): web-rs's web-codecs hands
its encoders' and decoders' frames out through `mpsc::unbounded_channel`, and their errors
through `watch::channel`, read in a `tokio::select!` with `biased;`. Its spike's census
counts about 50 refusals in tokio's `sync`.

Decided with the maintainer on 2026-11-07: ✅ map the channels into the runtime (rejected:
translating tokio's source, whose intrusive waiter lists, raw pointers into `UnsafeCell`s,
atomics, and `process::abort` mean nothing in single-threaded JS; and mapping only
web-codecs' methods, which the next consumer would reopen). Unsafe Rust may stay refused
where a wrapper over a common library does the job.

- Each channel's public API maps, waiters kept in a queue the runtime wakes, as
  `rs.Future`s on its executor. Methods that block a thread (`blocking_recv`,
  `blocking_send`) are refused, as no JS thread blocks.
- `select!` expands to code that already translates, apart from its support module:
  `thread_rng_n` picks the first branch polled, uniform as tokio's is, so a `select!`
  without `biased;` maps too.
- `Notify`, `Mutex`, `RwLock`, and `Semaphore` stay refused where reached, until a
  consumer needs them.

Public API: the translator's output for a crate using tokio's sync channels.
