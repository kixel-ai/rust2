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

So the template's `maven` job takes moq's names: `environment: publish`, and the signing key
and passphrase from `SIGNING_KEY` and `SIGNING_PASSWORD` instead of `MAVEN_SIGNING_KEY` and
`MAVEN_SIGNING_PASSPHRASE`, with the header comment saying so. Recommended over mapping
the names in each consumer's copy, since a consumer's edit is a gap under the configless
rule. The key is the ASCII-armored one moq's Gradle plugin reads in memory, which the
job's `gpg --import` takes as it is.

Check without publishing: upload the signed bundle with `publishingType=USER_MANAGED`, wait
for `VALIDATED` from `POST /api/v1/publisher/status`, and drop it with
`DELETE /api/v1/publisher/deployment/<id>`, from a `workflow_dispatch` run on moq's
`release` branch, the one ref the policy admits that publishes nothing by being pushed.
A `moq-ffi-v*` tag is wrong for it: pushing one starts moq's `release-shared-ffi.yml`, a real
release of every `moq-ffi` package, and moq's `release tags` ruleset guards those tags. In
the template the `maven` job needs `tag`, which skips off a tag, so the run needs a path of
its own: a dispatched job that validates and drops instead of publishing. A dispatch reads
its workflow from the ref it runs on, and `release` diverges from moq's `main`, so the
workflow reaches it through a PR into `release`, which moq's `release-gate.yml` checks.
GitHub offers a `workflow_dispatch` workflow only once its file is on the default branch,
so the same file lands on moq's `main` first; the run then dispatches the `release` copy.
Build the stand-in's `dev.moq:moq-ffi` at its pin. A failed validation is a builder fix here.
