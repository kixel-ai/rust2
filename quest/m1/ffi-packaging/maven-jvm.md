# [M] The Kotlin binding for desktop JVMs on Maven Central

## Goal

`<group>:<crate>` serves desktop JVMs as well as Android: a jar of the generated Kotlin with
each desktop target's library, which the binding loads from it, chosen over the AAR by
Gradle on a JVM. A clean Gradle JVM app on Linux, macOS, and Windows prints the transcript.

## Plan

Found by [#950](https://github.com/kixel-ai/rust2/pull/950), whose AAR serves Android alone:
moq's hand-built `dev.moq:moq-ffi` is a Kotlin Multiplatform publication with JVM and Android
variants (`kt/moq-ffi/build.gradle.kts` in moq-dev/moq), so publishing the AAR alone under
those coordinates would drop the JVM one moq ships today.

The generated Kotlin loads its library with `System.loadLibrary`, which finds none inside a
jar: the runtime extracts the host's library from the jar's resources to a file it loads,
before the contract check, as JNA's and UniFFI's loaders do. The jar's libraries are the
wheels' targets, built with `--features kotlin` on each build host.

Recommended: one set of coordinates, with Gradle module metadata (`.module`) naming the AAR
for Android and the jar for the JVM, as moq's publication does, so an app keeps one
dependency; Maven, which reads no `.module`, gets the jar through the POM's packaging.

## Waiting

- Maintainer: one set of coordinates with Gradle module metadata choosing the AAR or the jar
  (recommended: moq's users keep the one dependency they have today), or a separate
  `<crate>-jvm` artifact (simpler, no `.module`, but each app names its platform's).
