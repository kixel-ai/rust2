# [M] An Android library on Maven Central

## Goal

The builder releases an AAR to Maven Central: the Kotlin binding, `jniLibs` for
arm64-v8a, armeabi-v7a, and x86_64, the generated `.pro` as consumer rules, sources and
javadoc jars, and unstripped libraries for Play's native crash symbolication. A clean
Gradle app consumes it on the emulator and prints the transcript.

## Plan

Found by the 2026-11-18 language audit: moq ships an Android library, and the builder has
no Maven row.

The artifactId and version come from the crate; the groupId is the caller's, since Cargo's
`repository` can only suggest a Maven namespace it cannot prove the publisher owns
(`ffi/release/README.md`'s "Other registries" table).

Public API: the Maven coordinates.

## Required

- [16 KB pages](/quest/m1/ffi-android.md) - the libraries the AAR ships are aligned for Play
