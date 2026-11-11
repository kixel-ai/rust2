# [M] Kotlin records without the byte-array codec

## Goal

A [rust2-ffi](/ffi/README.md) Kotlin binding returns and passes a record without encoding
it through a byte array. [The binding benchmark](/bench/ffi/README.md) measures a record
return at 113 ns wall in Kotlin against C's 31 and Swift's 37, which reads the C struct.

## Plan

- Read and write the C struct's fields directly through JNI into the Kotlin data class,
  as Swift reads the C struct, generating one JNI accessor per record. Recommended over a
  direct `ByteBuffer` over the struct, which still copies field by field in Kotlin and adds
  a buffer allocation per call.
- Records nesting a string, a list, or an object keep their existing conversion for that
  field only.
- Prove it with the api fixture's Kotlin differential test and re-measure the bench.
