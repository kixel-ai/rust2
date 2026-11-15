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
- Second dispatch (2026-11-14, [run 43573016004](https://github.com/kixel-ai/rust2/actions/runs/43573016004)
  on main 6eef5a4f, after #781): all six builds passed, but the attestation API refused
  the provenance because its first `resolvedDependencies` entry, the crate source, lacked
  the `@refs/heads/main` ref that the signing certificate carries. The fix puts the caller
  workflow's `repo@ref` entry, with its commit, first.
- Third dispatch (2026-11-14, [run 43573196018](https://github.com/kixel-ai/rust2/actions/runs/43573196018)
  on the fix's branch, 5f405aae): all jobs passed. All six target archives hold both
  `crabx-pipeline-fixture` and `second`, and `gh attestation verify <archive> -R kixel-ai/rust2
  --signer-workflow kixel-ai/rust2/.github/workflows/crabx.yml` accepts every one.
- The reusable workflow and dry-run fixture are implemented by the pipeline PR.
  The dispatch cannot run before its workflow exists on the default branch.

## Waiting

- Maintainer: once this fix lands, run `gh workflow run crabx-test.yml -R kixel-ai/rust2 --ref main`
  and confirm it passes on the landed commit `<sha>`. Then tag that commit and publish:
  `git tag pipeline-v1.0.0 <sha> && git push origin pipeline-v1.0.0 && gh release create
  pipeline-v1.0.0 -R kixel-ai/rust2 --verify-tag --title pipeline-v1.0.0 --generate-notes`.
  Recommended: do it. The only change since the verified run is this reorder. An agent
  could run these steps, but a pushed release tag is public and permanent, so it waits on
  the maintainer's go. Done when `gh release view pipeline-v1.0.0 -R kixel-ai/rust2` succeeds.
