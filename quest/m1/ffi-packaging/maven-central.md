# [S] The first Android library Maven Central accepts

## Goal

Maven Central's Portal accepts a bundle the release's `maven` job signs from the builder's
Maven repository: its validation passes the POM, signatures, checksums, and sources and
javadoc jars, which no run has yet shown, as it needs a publisher's credentials.

## Plan

Found by [#950](https://github.com/kixel-ai/rust2/pull/950), which builds the AAR and its
Maven repository, checks them in a Gradle app on the emulator, and adds the release's `maven`
job that signs and uploads them (`ffi/release/release.yml`).

Check without publishing: upload the signed bundle with `publishingType=USER_MANAGED`, wait
for `VALIDATED` from `POST /api/v1/publisher/status`, and drop it with
`DELETE /api/v1/publisher/deployment/<id>`, from a workflow run in the repository holding the
credentials. Recommended: moq-dev/moq, which holds the `dev.moq` namespace and already
publishes `dev.moq:moq-ffi` with a Portal token and signing key, over the stand-in's
`dev.moq:moq-ffi` built at its pin. A failed validation is a builder fix here.

## Waiting

- Maintainer: in the repository that releases, moq-dev/moq for `dev.moq`, make a `maven`
  environment only release tags may deploy to, holding a Central Portal user token as
  `MAVEN_CENTRAL_USERNAME` and `MAVEN_CENTRAL_PASSWORD` and the armored signing key and its
  passphrase as `MAVEN_SIGNING_KEY` and `MAVEN_SIGNING_PASSPHRASE` (the ones moq's
  `release-kt-ffi.yml` uses serve). No agent can: the token and key are the publisher's,
  and Central has no trusted publishing to stand in for them. Check:
  `gh api repos/moq-dev/moq/environments/maven/secrets --jq '[.secrets[].name]'` lists the four.

## Related

- [The JVM library](/quest/m1/ffi-packaging/maven-jvm.md) - publishes beside the AAR under the same coordinates
