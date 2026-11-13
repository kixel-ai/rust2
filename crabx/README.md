# crabx

Run a crates.io CLI from its author's release assets:

```sh
crabx moq-cli --help
crabx moq-cli@0.14.16 --help
crabx --bin moq-relay moq-relay --help
```

Options go before the crate. Everything after it, including flags, spaces, and
non-UTF-8 arguments on Unix, goes to the downloaded binary. A crate with several
binaries uses the one named after the crate, or its only binary. Otherwise choose
`--bin NAME`.

crabx uses binstalk's release discovery, archive layouts, and signature verification.
It never installs into `~/.cargo/bin`. Missing prebuilt assets are an error. Pass
`--build` explicitly to run `cargo install --locked` into crabx's cache instead.
`--allow-quickinstall` permits cargo-quickinstall's third-party builds after the
author's releases; those have a separate cache from the default trust policy.

For GitHub repositories, crabx checks GitHub's attestations API for the downloaded
archive's SHA-256 before installing anything. Public Sigstore bundles verify their
certificate, SCT, signature, artifact digest, and transparency log against Sigstore's
production trust root. GitHub's timestamp-only bundles use GitHub's separate CA and
timestamp authority, verifying the certificate, signature, digest, and RFC 3161 time.
Both roots refresh through TUF. A malformed bundle, wrong repository, or verification/API
failure is an error. No published attestation is allowed, as are author releases hosted
outside GitHub. Binstalk's configured minisign checks remain enabled. Optional
`GH_TOKEN` or `GITHUB_TOKEN` credentials raise GitHub's API rate limit; they are sent
only to GitHub's attestation API.

Workflow certificates must name the crate's repository and either that repository's
workflow or rust2's reusable pipeline. Source refs must be tags or branches, excluding
pull-request refs. A tag must match the asset's release tag or version. Branch-triggered
builds are accepted so the pipeline can backfill published crates. GitHub release
attestations must carry GitHub's release-service identity and sign the crate's
repository and matching release tag. The verified result
stays with the cached entry; a pinned, cached invocation makes no network request and
requires neither Cargo nor GitHub's CLI.

Caches live in the operating system's user cache directory, under
`crabx/v1/<host>/<trust-policy>/<crate>/<version>`. Entries are staged, verified, and
published atomically under a per-crate lock. An incomplete or corrupt entry fails
loudly. Build and quickinstall entries cannot satisfy the default author's-release
policy.

An unpinned invocation runs its cached version immediately and starts a detached
sparse-index refresh and installation for the next invocation. The refresh retains the newest stable,
non-yanked version, never a prerelease. A yank can select an older version. Only one
refresh per crate and policy runs at a time. Network errors leave the cached version
available and are reported on the next invocation. A newly discovered version becomes current only after installation succeeds.

Unix uses `exec`, preserving the PID, arguments, standard streams, signals, and exit
status. Windows starts the program with inherited console and standard streams and
returns its exit code. Prebuilt Linux downloads require glibc; musl builds of crabx
direct users to `--build`.

Develop with `just crabx check`, `just crabx test`, and `just crabx run <crate> [args]`.
crabx is GPL-3.0-only because it links binstalk. The other rust2 projects keep their
existing licenses.
