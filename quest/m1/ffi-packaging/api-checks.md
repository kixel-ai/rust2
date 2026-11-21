# [S] The bindings test's api checks with their generated transcript

## Goal

`bindings-test.yml` runs the api fixture's `ffi/tests/api.py`, `api.cpp`, and `api.ts` on
the released wheels, C archives, and npm package again, each with the script `tests/fixtures/transcript`
generates from `fixtures/api/examples/transcript.rs`, and each prints native Rust's transcript.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#909](https://github.com/kixel-ai/rust2/pull/909): ✅ kept, with a `just ffi transcripts <dir>` recipe and check inputs that may name a directory (rejected: committing the generated scripts; dropping the checks to imports; dropping the quest).

Found by [the Python packaging quest's PR](https://github.com/kixel-ai/rust2/pull/909): since
[#885](https://github.com/kixel-ai/rust2/pull/885), `api.py` imports a generated `transcript`
module, `api.cpp` includes `transcript.hpp`, and `api.ts` imports `./transcript.ts`, none of
which the builder's single-file `python-check`, `c-check`, and `web-check` carry, so the api
fixture's consumers fail from the first nightly after it. moq's stand-in checks import no
generated script and still pass.

Recommended: a `just ffi transcripts <dir>` recipe writes each language's generated script,
which `bindings-test.yml` runs before calling the builder, and each check input may name a
directory, whose other files every consumer places beside the check: on Python's path
(the consumer runs `-I`, which keeps the script's directory off it), in the C consumer's
include path, and beside the npm consumer's module. Rejected: committing the generated
scripts, which `#885` keeps out of the tree, and dropping the fixture's checks to imports,
which loses the byte-for-byte transcript.

Public API: the builder's check inputs, if they take a directory.
