# [M] String encoding without TextEncoder per call

## Goal

`&str` to bytes no longer allocates through `TextEncoder.encode` on every call
(`ts/runtime/rs.ts:3305`): a literal's UTF-8 bytes are a hoisted constant, short strings
take an ASCII path, and a `String` decoded from bytes keeps them. String encode, join, and
decode rows on `just ts bench` report before and after.

## Plan

Found by the 2026-11-18 audit: string encode runs 30 times rust2-wasm in Chrome, join 8,
and decode 6. JS String Builtins are now in every browser, Safari since 26.2, so
rust2-wasm's strings get faster too; measure against its current build.

Public API: none.
