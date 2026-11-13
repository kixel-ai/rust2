# Crate release pipeline

The reusable workflow builds published binary crates for x86_64 and aarch64 GNU
Linux, macOS, and Windows. Linux uses cargo-zigbuild with glibc 2.28. Each archive
contains every binary, under binstall's default directory and filename. Builds
use all crate features so binaries with required features are included; mutually
exclusive features fail the build rather than silently omit binaries.

Add `.github/workflows/binaries.yml` to a public crate repository:

```yaml
name: Binaries
on:
  push:
    branches: [main]
    tags: ['v*', '*-v*']
  schedule:
    - cron: '23 7 * * *'
  workflow_dispatch:
permissions:
  contents: read
jobs:
  release:
    permissions:
      contents: write
      id-token: write
      attestations: write
    uses: kixel-ai/rust2/.github/workflows/crabx.yml@pipeline-v1.0.0
```

Use your default branch in the push filter. The first pipeline release tag must
be published before this example can run. A caller pins a pipeline release tag
or its immutable commit. The pipeline checks out its own scripts at the exact
commit of the called workflow, independently of the caller's checkout.

Each run discovers tracked crate manifests, looks up crates.io versions published
since the caller workflow was added, and fills missing target archives. Yanked
versions, library-only crates, and crates whose crates.io or published manifest
names another repository are skipped; the log names each skipped crate. Missing
or dirty VCS metadata, invalid paths, and a mismatched `name-vVERSION` tag stop
the run; yanking such a version skips it. A failed build fails the run and holds
back its crate, while crates whose archives all built still publish. An existing
annotated or lightweight `name-vVERSION` tag must point to Cargo's recorded
commit; a `vVERSION` tag is used when it points there and otherwise left to
another crate. When neither is used, publishing creates `name-vVERSION` there.
Existing release assets are preserved. Publishing is serialized per release tag
across trigger refs. Each run publishes
one crate at a time, so crates sharing a tag cannot replace pending jobs within
that run. GitHub retains only one pending publisher per tag across runs; a newer
run backfills the same missing archives if it supersedes an older pending job.
Unrelated assets are preserved, and no release is made latest merely because it
was backfilled.

Build jobs have read-only tokens and no OIDC permission. Only the publishing job
attests the downloaded archives and writes releases; it never runs crate code.
Callers must grant its three permissions. Public GitHub.com repositories are the
supported boundary; private repositories and GitHub Enterprise Server are not.

Tags made with `GITHUB_TOKEN` do not trigger workflows, which is why the default
branch push, daily schedule, and manual dispatch are included. GitHub disables
public schedules after 60 inactive days; the next push or dispatch backfills
versions published during that pause. A backfill exceeding GitHub's 256 matrix
jobs fails explicitly.

`.github/workflows/crabx-test.yml` builds the two-binary fixture on all six targets
on PRs with read-only permissions. A manual dispatch also attests the archives
without creating tags or releases, and bypasses crates.io discovery. Local tests
exercise discovery, archiving, and publishing with recorded API and Cargo
responses; the fixture workflow builds the real archives on all six targets. The pipeline release quest owns the first hosted attestation dispatch and pipeline
release tag; the shipped crabx quest owns the first real crates.io consumer.

References: [binstall archive defaults](https://github.com/cargo-bins/cargo-binstall/blob/main/SUPPORT.md),
[reusable job identity](https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#job-context),
and [attestation predicates](https://github.com/actions/attest).
