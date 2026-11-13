The attestation and trust root are Sigstore's real conda provenance fixture and its
pinned public-good trust root, from sigstore/sigstore-rust 0.16.0:

- `crates/sigstore-verify/test_data/bundles/conda-attestation.sigstore.json`
- `crates/sigstore-verify/test_data/trusted_roots/public-good.json`
- `crates/sigstore-verify/test_data/bundles/github-private-attestation.sigstore.json`
  (GitHub's timestamp-only release attestation, with the library's embedded GitHub root)

Source: https://github.com/sigstore/sigstore-rust/tree/55227ff848a69d2d81ac36607d6fcfa25b97553a,
Apache-2.0.
The test changes its artifact digest, repository, signature, and transparency-log time
independently and requires each change to fail verification.
The GitHub fixture also checks the RFC 3161 signature, signed release tag, and
rejection under the public-good trust domain.
