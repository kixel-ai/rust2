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
- The reusable workflow and dry-run fixture are implemented by the pipeline PR.
  The dispatch cannot run before its workflow exists on the default branch.
