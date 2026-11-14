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
  on 0d258940): all six fixture builds passed, but `actions/attest` refused the
  custom `buildType`, so nothing was attested and no tag was cut.
- Maintainer decision (2026-11-13): ✅ use GitHub's
  `https://actions.github.io/buildtypes/workflow/v1` build type with its
  `externalParameters.workflow` shape, keeping crate name, version, and path in
  `externalParameters` (rejected: switch to `actions/attest-build-provenance` and
  drop the custom predicate). [#781](https://github.com/kixel-ai/rust2/pull/781)
  implements it in `provenance()`; redispatch once it lands.
- The reusable workflow and dry-run fixture are implemented by the pipeline PR.
  The dispatch cannot run before its workflow exists on the default branch.
