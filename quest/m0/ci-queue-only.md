# [M] The merge queue is the only CI build, on the self-hosted runner

## Goal

Each tree is built once in CI: the merge queue's `Queue build / Build`, on
the self-hosted `rust2-queue` runner. Pull requests run no CI, and a push to
`main` reruns nothing the queue already tested. Asynchronous work (nightly,
valgrind, releases) stays on GitHub's runners, and the manual benchmark keeps
its exclusive `rust2-bench` host.

## Plan

Found 2026-11-27 (host CPU audit, kixel-ai/dotfiles#403): `ci.yml` runs
`build.yml` on every PR push and again on every push to `main`, the same tree
the queue already built. Over two days the `main` reruns took 14.1 job-hours
against the queue's 14.2. They exist only because a push to `main` is the one
run allowed to save the build cache the others restore. All three run on
GitHub's runners since kixel-ai/rust2#991, while nixos's `rust2-ci` container
sits idle (25 CPU-seconds in 39 hours).

Decided 2026-11-27 (maintainer, `/quest-plan`):

- ✅ Merge queues, and anything else that blocks a merge, run on self-hosted
  runners; asynchronous work (nightly, releases, benches) runs on GitHub's
  (rejected: keeping the queue on GitHub's runners as an exception to the
  maintainer's rule that a public repository's merge gates run self-hosted).
  This reverses the hosting half of #991, whose reason was a heavily loaded
  host: on 2026-11-27 nixos averaged 6.4 of its 32 cores busy over 24 hours.
- ✅ No CI on pull requests, as kixel-ai/quest decided for every repository
  on 2026-11-27 (`quest/m0/ci/workflows.md`). Agents run the checks locally
  before pushing, and the merge queue catches what they miss.
- ✅ No rerun on push to `main`.

Recommendations:

- Delete `ci.yml`. `queue.yml` keeps reporting the skipped required context
  on a PR's head.
- The `rust2-queue` runner group admits only
  `queue-build.yml@refs/heads/main`, while `queue.yml` today calls `build.yml`
  from the merge group's ref, so changing the runner label alone leaves the
  next merge group with no runner. Restore a queue-only reusable workflow
  called at `main`, as before #991, keeping the required
  `Queue build / Build` context and the skipped report on a PR's head.
- Build through the host's mbx store (`backend: local`, as rind's and moq's
  queues do), so the queue warms itself and nothing needs a `main` push to
  save a cache. Keep mbx at 1.31.0 or newer, which `build.yml` documents as its
  minimum (older ones bypass the cache for the FFI export's MIR flags or seed
  target dirs `cargo clean --target-dir` rejects); dotfiles'
  `quest/m0/nixos/ci-shared-mbx.md` moves rust2's fixer to the same version.
- Keep the shard split from #991 only if a single self-hosted job is slower;
  on one host, shards share its cores.
- Measure one merge group's wall time and the runner's CPU before and after.

## Related

- [Pre-push hook runs the diff's CI](/quest/m0/pre-push-check.md) - the local side of
  the same decision
