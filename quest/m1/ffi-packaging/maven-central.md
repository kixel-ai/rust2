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
`just gh central-check`) takes the ID of a finished kixel-ai/rust2 run on rust2's `main`,
refusing any other so only reviewed code sees the signing key, downloads its
`moq-ffi-maven`, and runs `central.sh validate` from that run's commit in the `publish`
environment. It runs only on `release`, the one ref the environment admits that publishes
nothing by being pushed; GitHub offers a dispatch only for a workflow on the default branch,
so [moq-dev/moq#9519](https://github.com/moq-dev/moq/pull/9519) landed it on `main` and
[moq-dev/moq#9520](https://github.com/moq-dev/moq/pull/9520) on `release`. Never push a
`moq-ffi-v*` tag for it: that starts moq's real release of every `moq-ffi` package.

Dispatch it, which `publish` lets an agent do as it requires no reviewer:
`gh workflow run central-check.yml -R moq-dev/moq --ref release -f run=<id>`, with `<id>` a
finished Nightly, or a dispatched Check, on rust2's `main` since
[#990](https://github.com/kixel-ai/rust2/pull/990) landed `central.sh` and the `dev.moq`
stand-in. A failed validation is a builder fix here. Delete moq's workflow once moq releases
its Kotlin library through rust2's builder.

Decided 2026-11-24 (maintainer, /quest-run): ✅ land moq-dev/moq#9519 into `main` and
moq-dev/moq#9520 into `release` (rejected: holding them). Both landed on 2026-11-24.

## Waiting

- kixel-ai/rust2: a Check, or a Nightly, finishes on `main` with its `moq-ffi-maven` artifact.
  None has since [#990](https://github.com/kixel-ai/rust2/pull/990): the stand-in's Windows and
  `apple-darwin` build legs ran past the job's 90-minute cap
  ([Check 45378844915](https://github.com/kixel-ai/rust2/actions/runs/45378844915)), which
  [#1000](https://github.com/kixel-ai/rust2/pull/1000) raises to 150. Check: after it lands,
  `gh workflow run check.yml -R kixel-ai/rust2 --ref main`, then
  `gh api repos/kixel-ai/rust2/actions/runs/<id>/artifacts --jq '[.artifacts[].name]'` lists
  `moq-ffi-maven`. Then an agent dispatches `central-check.yml` on moq's `release` with that
  run's ID, as above. The api fixture's builds fail on `main` until
  [#993](https://github.com/kixel-ai/rust2/pull/993) lands, which leaves the run red but does
  not stop the stand-in's `maven` job.
