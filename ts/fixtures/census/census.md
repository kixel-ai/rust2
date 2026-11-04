# Census of `census-fixture`

- Refusals: 12
- Function instances: 6
- Generic functions no instance reached, lowered over their type parameters: 1

## By construct

| Count | Construct | First |
| ---: | --- | --- |
| 3 | a raw pointer to `_` behind a reference, which is the object and not where it is kept | fixtures/census/src/lib.rs:9:2 |
| 2 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | fixtures/census/src/lib.rs:19:12 |
| 2 | a mutable static, which any code may write | fixtures/census/src/lib.rs:38:3 |
| 1 | `Bytes::from_owner` of `_`, which would drop with the last clone of what it makes, which is not supported | fixtures/census/src/lib.rs:45:2 |
| 1 | `Pointer` formatting, as in `{:p}`, is not supported | fixtures/census/src/lib.rs:24:2 |
| 1 | a pointer made of an address, which JS has no memory at | fixtures/census/src/lib.rs:14:2 |
| 1 | the address of a pointer to `_`, each a place JS makes of its own | fixtures/census/src/lib.rs:9:2 |
| 1 | union `_`, whose fields overlap in `_` and `f64`, which JS keeps apart | fixtures/census/src/lib.rs:28:1 |

## By API

The type or module each refused type or path belongs to.

| Count | API | First |
| ---: | --- | --- |
| 5 | `u32` | fixtures/census/src/lib.rs:9:2 |
| 2 | `u64` | fixtures/census/src/lib.rs:19:47 |
| 2 | `u8` | fixtures/census/src/lib.rs:19:12 |
| 1 | `Bits` | fixtures/census/src/lib.rs:28:1 |
| 1 | `std::sync::Arc` | fixtures/census/src/lib.rs:45:2 |

## By type or path

| Count | Construct | Type or path | First |
| ---: | --- | --- | --- |
| 2 | a raw pointer to `_` behind a reference, which is the object and not where it is kept | `u32` | fixtures/census/src/lib.rs:9:2 |
| 1 | `Bytes::from_owner` of `_`, which would drop with the last clone of what it makes, which is not supported | `std::sync::Arc` | fixtures/census/src/lib.rs:45:2 |
| 1 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | `u32`, `u8` | fixtures/census/src/lib.rs:19:12 |
| 1 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | `u64`, `u8` | fixtures/census/src/lib.rs:19:47 |
| 1 | a raw pointer to `_` behind a reference, which is the object and not where it is kept | `u64` | fixtures/census/src/lib.rs:19:48 |
| 1 | the address of a pointer to `_`, each a place JS makes of its own | `u32` | fixtures/census/src/lib.rs:9:2 |
| 1 | union `_`, whose fields overlap in `_` and `f64`, which JS keeps apart | `Bits`, `u32` | fixtures/census/src/lib.rs:28:1 |

## Spans

<details><summary>a raw pointer to `_` behind a reference, which is the object and not where it is kept (3)</summary>

- fixtures/census/src/lib.rs:9:2: a raw pointer to `u32` behind a reference, which is the object and not where it is kept
- fixtures/census/src/lib.rs:19:13: a raw pointer to `u32` behind a reference, which is the object and not where it is kept
- fixtures/census/src/lib.rs:19:48: a raw pointer to `u64` behind a reference, which is the object and not where it is kept

</details>

<details><summary>a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another (2)</summary>

- fixtures/census/src/lib.rs:19:12: a cast from a pointer to `u32` to one to `u8`, which reads the bytes of one type as another
- fixtures/census/src/lib.rs:19:47: a cast from a pointer to `u64` to one to `u8`, which reads the bytes of one type as another

</details>

<details><summary>a mutable static, which any code may write (2)</summary>

- fixtures/census/src/lib.rs:38:3: a mutable static, which any code may write
- fixtures/census/src/lib.rs:39:3: a mutable static, which any code may write

</details>

<details><summary>`Bytes::from_owner` of `_`, which would drop with the last clone of what it makes, which is not supported (1)</summary>

- fixtures/census/src/lib.rs:45:2: `Bytes::from_owner` of `std::sync::Arc<[u8]>`, which would drop with the last clone of what it makes, which is not supported

</details>

<details><summary>`Pointer` formatting, as in `{:p}`, is not supported (1)</summary>

- fixtures/census/src/lib.rs:24:2: `Pointer` formatting, as in `{:p}`, is not supported

</details>

<details><summary>a pointer made of an address, which JS has no memory at (1)</summary>

- fixtures/census/src/lib.rs:14:2: a pointer made of an address, which JS has no memory at

</details>

<details><summary>the address of a pointer to `_`, each a place JS makes of its own (1)</summary>

- fixtures/census/src/lib.rs:9:2: the address of a pointer to `u32`, each a place JS makes of its own

</details>

<details><summary>union `_`, whose fields overlap in `_` and `f64`, which JS keeps apart (1)</summary>

- fixtures/census/src/lib.rs:28:1: union `Bits`, whose fields overlap in `u32` and `f64`, which JS keeps apart

</details>
