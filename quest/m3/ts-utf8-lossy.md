# [S] Lossy and chunked UTF-8 decoding

## Goal

[rust2-ts](/ts/README.md) translates std's stable lossy and chunked UTF-8 decoding:
`String::from_utf8_lossy`, `<[u8]>::utf8_chunks` with its `Utf8Chunks` and `Utf8Chunk`,
and `FromUtf8Error`'s `as_bytes`, `into_bytes`, and `utf8_error`, each replacing or
splitting at exactly the invalid sequences std's does, proven by a differential test
against native Rust.
