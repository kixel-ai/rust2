# [S] The Flutter consumer's Gradle build never fails resolving the Kotlin plugin

## Goal

The builder's `flutter-consumer` job passes or fails on the Dart package alone: its
`flutter build apk` never fails resolving `org.jetbrains.kotlin:kotlin-stdlib` or the Kotlin
Gradle plugin for the Flutter SDK's own `dev.flutter.flutter-plugin-loader` build.

## Plan

Found by [#962](https://github.com/kixel-ai/rust2/pull/962)'s repro runs
([45025889521](https://github.com/kixel-ai/rust2/actions/runs/45025889521),
[45026469300](https://github.com/kixel-ai/rust2/actions/runs/45026469300)): 7 of 40 runs
of `consume-flutter` on the api fixture failed in 6 to 20 seconds with `Could not find
org.jetbrains.kotlin:kotlin-stdlib:2.3.20`, searched only at
`https://plugins.gradle.org/m2/`, configuring the `:gradle` project Flutter 3.50.1's
`settings.gradle.kts` includes. All 7 were api-fixture jobs; the moq stand-in's 20 never
failed so, though both resolve the same classpath, so timing against the portal is the
likelier cause than the fixture.

Reproduce it with `--info` to see the portal's response, then fix the cause: the portal
redirecting to a Maven Central mirror that 404s, which a `mavenCentral()` repository for
that build would settle, is one candidate. No retry in its place.
