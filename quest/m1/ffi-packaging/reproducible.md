# [S] Reproducible, attested releases

## Goal

A rebuild of a release makes the same bytes: wheels, archives, and the XCFramework zip
take `SOURCE_DATE_EPOCH` timestamps, a deterministic file order, and remapped paths, so
**Re-run all jobs** succeeds. Every artifact gets a build provenance attestation
(`actions/attest-build-provenance`) and an SBOM.

## Plan

Found by the 2026-11-18 language audit: `package.py:208`'s `writestr` stamps the current
time and the XCFramework is zipped with `ditto`, which `ffi/release/README.md` admits
makes rebuilds differ. PyPI's trusted publishing already uploads PEP 740 attestations; the
other artifacts have none.

Public API: none.
