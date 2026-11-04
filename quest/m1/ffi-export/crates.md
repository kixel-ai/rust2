# [S] moq-clock's C names once it moves into kio

## Goal

moq-clock's items export with C names that say they are the clock's: today `now()` and
`origin()` are `moq_now` and `moq_origin`, which reads like moq-net's origin types.

This waits on moq's [kio::clock quest](https://github.com/moq-dev/moq/blob/main/quest/m1/kio-clock.md),
which moves moq-clock into `kio::clock`. It has landed once `rs/moq-clock` is gone from
moq's `main`.

## Plan

Then bump the moq pin and re-export `kio::clock` in moq-clock's place in `just ffi
standin`, so the naming rule names its items `moq_clock_*`.

Public API: the C names moq's export emits.
