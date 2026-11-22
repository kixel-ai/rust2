# [S] The Flutter consumer's install on the emulator never stalls

## Goal

The builder's `flutter-consumer` job passes or fails on the Dart package alone: `flutter run`
never sits silent after `Installing build/app/outputs/flutter-apk/app-debug.apk...` until
`consume_flutter`'s 15-minute silence deadline fails the job.

## Plan

Found by [#950](https://github.com/kixel-ai/rust2/pull/950): on its head, the api fixture's
`flutter-consumer` built the APK, printed `Installing`, and then nothing until the deadline
([run 44993636561](https://github.com/kixel-ai/rust2/actions/runs/44993636561)), while the
moq stand-in's passed in the same run. [#915](https://github.com/kixel-ai/rust2/pull/915)
saw the same stall once on the moq stand-in, and it passed on a later run, so it is
intermittent and predates #950, whose diff leaves the job and `consume_flutter` unchanged.

Reproduce it first, such as by rerunning the job until it stalls with `adb logcat` and
`flutter run -v` captured, then fix the cause: an install racing the API 24 emulator's boot
(package manager not yet up although `sys.boot_completed` is set) is one candidate, which
waiting for `pm path android` or installing with `adb install` before `flutter run
--use-application-binary` would settle. No retry or longer deadline in its place.
