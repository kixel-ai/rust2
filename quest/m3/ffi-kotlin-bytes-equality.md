# [S] Kotlin: bytes compare by content

## Goal

Kotlin's `Bytes` compares and hashes by its bytes, as Rust's `Bytes` and `Vec<u8>` do, so
a generated data class holding bytes, such as a record with a payload, equals another
record with the same fields, and works as a map key.

## Plan

Found by [#432](https://github.com/moq-dev/rust2/pull/432): `Bytes` has no `equals` or
`hashCode`, so it compares by identity and a record holding bytes equals only itself.
This predates #432.

Content equality must read lent bytes through the same JNI calls that keep their buffer
alive during a read, and cost no more crossings than the comparison needs: compare sizes
first, and copy or compare in bulk rather than per byte. Cache the hash only if a
measurement shows it matters. Prove it with a test comparing copied, lent, sliced, and
Kotlin-held bytes, and records holding each.

Public API: breaking for code that relied on identity; it gets a `MIGRATE.md` entry.
