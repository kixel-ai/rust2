# [S] The first Kotlin library Maven Central accepts

## Goal

Maven Central's Portal accepts a bundle the release's `maven` job signs from the builder's
Maven repository: its validation passes the POM, signatures, checksums, and sources and
javadoc jars, which no run has yet shown, as it needs a publisher's credentials.

## Plan

Found by [#950](https://github.com/kixel-ai/rust2/pull/950), which builds the AAR and its
Maven repository, checks them in a Gradle app on the emulator, and adds the release's `maven`
job that signs and uploads them (`ffi/release/release.yml`). The repository now holds the
desktop jar and its Gradle metadata beside the AAR, which the validation covers too.

Decided 2026-11-23 (maintainer, /quest-run): ✅ reuse moq-dev/moq's existing `publish`
environment (rejected: a new `maven` environment with a freshly minted Portal token).
As checked that day, `publish` holds `MAVEN_CENTRAL_USERNAME`, `MAVEN_CENTRAL_PASSWORD`,
`SIGNING_KEY`, and `SIGNING_PASSWORD`, which moq's `release-kt-ffi.yml` uses. Its deployment
policy admits only the tags `libmoq-v*`, `moq-boy-v*`, `moq-cli-v*`, `moq-ffi-v*`,
`moq-gst-v*`, and `moq-relay-v*`, and the branch `release`.

The template's `maven` job takes moq's names (`environment: publish`, `SIGNING_KEY`,
`SIGNING_PASSWORD`) and runs `ffi/release/central.sh publish`, rust2's script at the builder's
pin, which `central.sh validate` shares: it uploads with `publishingType=USER_MANAGED`,
waits for `VALIDATED`, and drops the deployment, failing if the drop fails. rust2's test of
moq's stand-in builds it under `dev.moq`, so its `moq-ffi-maven` artifact,
`dev.moq:moq-ffi:0.0.0` at the exporter's moq pin, is the bundle Central validates.

moq's **Central check** workflow (`.github/workflows/central-check.yml`, through
`just gh central-check`) takes a kixel-ai/rust2 run ID, refuses a fork's commit, downloads
its `moq-ffi-maven`, and runs `central.sh validate` from the run's own commit in the
`publish` environment. It runs only on `release`, the one ref the environment admits that
publishes nothing by being pushed; GitHub offers a dispatch only for a workflow on the
default branch, so [moq-dev/moq#9519](https://github.com/moq-dev/moq/pull/9519) lands it on
`main` and [moq-dev/moq#9520](https://github.com/moq-dev/moq/pull/9520) on `release`. Never
push a `moq-ffi-v*` tag for it: that starts moq's real release of every `moq-ffi` package.

Once both land, dispatch it, which `publish` lets an agent do as it requires no reviewer:
`gh workflow run central-check.yml -R moq-dev/moq --ref release -f run=<id>`, with `<id>` a
successful kixel-ai/rust2 Check or Nightly run whose commit holds `central.sh`. A failed
validation is a builder fix here. Delete moq's workflow once moq releases its Kotlin library
through rust2's builder.

## Waiting

- Maintainer: approve landing [moq-dev/moq#9519](https://github.com/moq-dev/moq/pull/9519)
  into `main` and then [moq-dev/moq#9520](https://github.com/moq-dev/moq/pull/9520) into
  `release`, which moq lands only with the maintainer's go-ahead. Check:
  `gh api repos/moq-dev/moq/contents/.github/workflows/central-check.yml?ref=release` succeeds.
