# [S] Release the pipeline

## Goal

Publish the first `pipeline-v1.0.0` tag on a landed pipeline commit, after the
six-target fixture builds and GitHub attestations have been exercised on GitHub.
The documented caller and crabx init can then pin a real pipeline release.

## Plan

- Maintainer decision (2026-11-13): ✅ keep this follow-up (rejected: drop).

- Dispatch `.github/workflows/crabx-test.yml` on the landed commit. Verify all six
  target archives contain both fixture binaries and that `gh attestation verify`
  accepts each archive with the reusable workflow as signer. PR fixture runs stay
  read-only; this dispatch is the first exercise of the OIDC publishing job.
- Tag that verified commit `pipeline-v1.0.0`, and publish the pipeline release.
- First dispatch (2026-11-13, [run 43527031591](https://github.com/kixel-ai/rust2/actions/runs/43527031591)
  on 0d258940): all six fixture builds passed, but the publish job's `actions/attest`
  failed with `Invalid Argument - unsupported build type:
  https://github.com/kixel-ai/rust2/tree/main/.github/crabx`. GitHub's attestation
  API refuses the custom `buildType` that `provenance()` in
  `.github/crabx/pipeline.py` writes, so nothing was attested and no tag was cut.
  Fix the predicate first, land it, then redispatch.
- The reusable workflow and dry-run fixture are implemented by the pipeline PR.
  The dispatch cannot run before its workflow exists on the default branch.

## Waiting

- Maintainer: pick the provenance fix. (a) Recommended: emit GitHub's own
  `https://actions.github.io/buildtypes/workflow/v1` build type with its
  `externalParameters.workflow` shape, keeping crate name, version, and path in
  `externalParameters`, since the API is known to accept it. (b) Drop the custom
  predicate and use `actions/attest-build-provenance`, losing the crate and
  pipeline-commit fields. Check: a redispatch of `crabx-test.yml` attests all six
  archives.
