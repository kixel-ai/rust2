# [M] Every refusal at moq's main has a rule

## Goal

`just ffi moq-cpp <checkout>` at moq's `main` exports with no error and no new exclude:
each construct the export refuses there, beyond
[`NonZero`](/quest/m1/ffi-refusals/nonzero.md), exports by a rule or is left out as what C
can never hold. Measured on 2026-11-12 at moq `6936f0a`
([the pin bump](/quest/m1/ffi-refusals/moq-pin.md)): single refusals such as
`qmux::Session` and `CertificateDer`, third-party types taken by value; a borrowing
future from the non-async `Server::accept`; and borrowed returns that are not `Clone`.

## Plan

[The epic](/quest/m1/ffi-refusals/README.md)'s decisions apply: rust2 asks moq for no
source change, a third-party type behind an `Arc` crosses as an opaque handle, and what C
can never hold is left out and listed.

- Start from a census at a moq `main` checkout, and list every refusal it still has by
  construct in the PR. A construct that needs a decision the epic does not already make
  becomes its own quest here, with the options and a recommendation, rather than a guess.
- Recommended for a third-party type by value: an opaque handle with none of its API, as
  the `Arc` form already is, since the value moves into the handle exactly.
- Recommended for the borrowing future of a non-async fn: the rule async fns' borrowing
  futures already follow ([#710](https://github.com/kixel-ai/rust2/pull/710)).
- No exclude is added to `ffi/moq/ffi.toml` to pass.

Public API: the generated API of each item that newly exports.
