# [XS] The Android legs' kotlinc download never fails a release

## Goal

The builder's `android` build legs get their pinned kotlinc without depending on an
anonymous download from GitHub's release assets succeeding, so a release never loses its
Android libraries to a rate limit.

## Plan

Found by [#950](https://github.com/kixel-ai/rust2/pull/950): one builder dispatch lost both
`android` build legs when `Android tools` in `.github/workflows/bindings.yml` fetched
`kotlin-compiler-$KOTLIN.zip` from `github.com/JetBrains/kotlin/releases` and got HTTP 429
through all five of curl's retries; the rerun passed. A caller's release runs the same job.

Recommended: restore the checked zip from `actions/cache`, keyed on `KOTLIN_SHA256`, so only
a cache miss reaches the release download, and on a miss fetch it with the job's token if
GitHub's asset redirect honors it for a higher limit (check first; if it doesn't, the cache
alone). Rejected: more retries or a longer backoff, which only narrow the window.
