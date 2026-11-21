# [S] Apple privacy declarations for the Dart package's libraries

## Goal

A Flutter app on the App Store declares every required-reason API the Dart package's
Apple libraries call, as the Swift package's `PrivacyInfo.xcprivacy` does since
[#912](https://github.com/kixel-ai/rust2/pull/912), and a library calling one whose reason
only the crate's author knows fails the release.

## Plan

Found by [#915](https://github.com/kixel-ai/rust2/pull/915)'s review: its Apple Dart
libraries reach `fstat` and `mach_absolute_time` through std, as the Swift package's
slices do, but ship no manifest. A code asset is a bare dylib that Flutter wraps in a
framework it generates, so the package has no place to put one Flutter would bundle
([llamadart-native#98](https://github.com/leehack/llamadart-native/pull/98) hits the same
wall).

Recommended: scan each Apple Dart library's imports with #912's `REQUIRED_REASON_APIS`,
failing as it does on a category with no universal reason, and ship the manifest in the
package with a README line telling the app to merge it into its own, until dart-lang/native
lets a code asset carry a resource; then bundle it. Rejected: refusing every Apple library
that calls one, which would refuse std's `fstat` everywhere.
