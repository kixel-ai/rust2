# Census of `reject-fixture`

- Refusals: 256
- Function instances: 292
- Generic functions no instance reached, lowered over their type parameters: 13

## By construct

| Count | Construct | First |
| ---: | --- | --- |
| 50 | a raw pointer to `_` in `_`, which the crate may put another object in place of | fixtures/reject/src/lib.rs:536:10 |
| 23 | a raw pointer to `_` in a variable assigned again while it may point there | fixtures/reject/src/lib.rs:544:10 |
| 20 | unsupported call to `_` | fixtures/reject/src/lib.rs:164:8 |
| 15 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | fixtures/reject/src/lib.rs:1259:33 |
| 15 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | fixtures/reject/src/lib.rs:1577:18 |
| 6 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | fixtures/reject/src/lib.rs:483:27 |
| 5 | `_` with a mutable reference to a value JS copies, which its mapping would copy | fixtures/reject/src/lib.rs:1157:22 |
| 5 | a raw pointer to `_` in an element, a payload, or a union field, which is not supported yet | fixtures/reject/src/lib.rs:607:10 |
| 5 | memory from `alloc` cast to pointers to `_` and to `_`, which reads the bytes of one type as another | fixtures/reject/src/lib.rs:2268:7 |
| 4 | a variant of `_`, which is not supported: either is the same JS string | fixtures/reject/src/lib.rs:260:2 |
| 4 | an assignment through a reference to `_` | fixtures/reject/src/lib.rs:83:2 |
| 3 | `_` of `_`, whose `clone_from` is its own, which is not supported yet | fixtures/reject/src/lib.rs:21:4 |
| 3 | a byte of memory from `alloc`, which JS keeps as the values a cast of its pointer types it as | fixtures/reject/src/lib.rs:496:3 |
| 3 | a copy between pointers to `_`, which JS keeps as an object that knows no array it is in | fixtures/reject/src/lib.rs:1632:3 |
| 3 | a copy of a closure that changes what it captured | fixtures/reject/src/lib.rs:182:14 |
| 3 | a slice or a cell of a mutable reference to `_` that names no place, which a whole write would replace only there | fixtures/reject/src/lib.rs:2188:2 |
| 3 | an offset of a pointer to `_`, which JS keeps as an object that knows no array it is in | fixtures/reject/src/lib.rs:1635:56 |
| 3 | replacing the value behind a reference to `_` | fixtures/reject/src/lib.rs:91:2 |
| 3 | unsupported type `_` | fixtures/reject/src/lib.rs:7:14 |
| 2 | `_` is not supported: its `None` and `Some` would both be `rt.None` | fixtures/reject/src/lib.rs:16:15 |
| 2 | `_` of `_`, whose `None` is `rt.None`, which is not supported yet | fixtures/reject/src/lib.rs:37:4 |
| 2 | `_`, a trait object of another crate's trait, which is not supported yet | fixtures/reject/src/lib.rs:11:16 |
| 2 | `_`, a trait object of the generic trait `_`, which is not supported yet | fixtures/reject/src/lib.rs:386:23 |
| 2 | `offset_from` between a pointer into a value in no array and one into a value in no array, which JS keeps in different arrays | fixtures/reject/src/lib.rs:2147:5 |
| 2 | `offset_from` between a pointer into an array of this body's own and one into an array of this body's own, which JS keeps in different arrays | fixtures/reject/src/lib.rs:2149:24 |
| 2 | a pointer reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust | fixtures/reject/src/lib.rs:2172:18 |
| 2 | a raw or mutable slice of a byte in no array of bytes | fixtures/reject/src/lib.rs:2158:12 |
| 2 | a raw pointer to a place of `()`, which has no address of its own | fixtures/reject/src/lib.rs:656:15 |
| 2 | an upcast from `_` to `_`, which is not supported yet | fixtures/reject/src/lib.rs:454:32 |
| 2 | comparing pointers to `_`, each a place JS makes of its own | fixtures/reject/src/lib.rs:524:2 |
| 2 | overriding `_`, which std's adaptors call in place of `next` or beside it | fixtures/reject/src/lib.rs:125:2 |
| 2 | unsupported type `_` from another crate | fixtures/reject/src/lib.rs:367:32 |
| 1 | `Buf::copy_to_bytes` of `_`, a value JS copies, which is not supported | fixtures/reject/src/lib.rs:689:2 |
| 1 | `BufMut::put` of `_` into a `BytesMut`, which advances it by the room left in a capacity that depends on whether the other handles of its storage dropped, which is not supported | fixtures/reject/src/lib.rs:323:2 |
| 1 | `BufMut::put` of `_` into a `BytesMut`, which picks the methods it calls by a capacity that depends on whether the other handles of its storage dropped, which is not supported | fixtures/reject/src/lib.rs:318:2 |
| 1 | `Bytes::from_owner` of `_`, which would drop with the last clone of what it makes, which is not supported | fixtures/reject/src/lib.rs:303:2 |
| 1 | `Extend` of `_` by `_`, whose `_` of `_` is not supported yet | fixtures/reject/src/lib.rs:1194:31 |
| 1 | `Pointer` formatting, as in `{:p}`, is not supported | fixtures/reject/src/lib.rs:49:2 |
| 1 | `_` is not supported: its `Pending` and `Ready` would both be `rt.OuterPending` | fixtures/reject/src/lib.rs:72:20 |
| 1 | `_` is not supported: its field `_` is `_`, typed `_`, whose `None` would be `rt.None` as `_` | fixtures/reject/src/lib.rs:31:2 |
| 1 | `_` is not supported: its field `_` is `_`, typed `_`, whose `Pending` would be `rt.OuterPending` as `_` | fixtures/reject/src/lib.rs:424:2 |
| 1 | `_` of `_` by `_`, which is not supported yet | fixtures/reject/src/lib.rs:211:6 |
| 1 | `_` of `_`, which is not supported yet | fixtures/reject/src/lib.rs:466:4 |
| 1 | `_`, which reads a capacity that depends on whether the other handles of its storage dropped, which is not supported | fixtures/reject/src/lib.rs:312:6 |
| 1 | `clone_from` of `_`, whose items' `clone_from` is their own, which is not supported yet | fixtures/reject/src/lib.rs:2040:4 |
| 1 | `collect` of `_`, which sorts by `_`'s own `Ord`, of entries that drop, which is not supported yet | fixtures/reject/src/lib.rs:877:22 |
| 1 | `fmt::Arguments` with a placeholder, or not made in place, written to `_`'s own `fmt::Write`, which is not supported yet: std splits what a placeholder writes into `write_str` calls the runtime does not make | fixtures/reject/src/lib.rs:55:10 |
| 1 | `offset_from` between a pointer into a shared slice JS copies and one into a value in no array, which JS keeps in different arrays | fixtures/reject/src/lib.rs:2152:21 |
| 1 | `offset_from` between a pointer into a shared slice JS copies and one into an array of this body's own, which JS keeps in different arrays | fixtures/reject/src/lib.rs:2151:20 |
| 1 | `offset_from` of pointers to `()`, which has no bytes | fixtures/reject/src/lib.rs:2094:13 |
| 1 | `offset_from` of pointers to `_`, which JS keeps as an object that knows no array it is in | fixtures/reject/src/lib.rs:2087:36 |
| 1 | a cast from `_` to `_` | fixtures/reject/src/lib.rs:68:2 |
| 1 | a coercion to `_` | fixtures/reject/src/lib.rs:234:2 |
| 1 | a coercion to `_`, which is not supported yet | fixtures/reject/src/lib.rs:413:30 |
| 1 | a constant of type `_` | fixtures/reject/src/lib.rs:2288:34 |
| 1 | a constant of type `_`, a pointer into memory translation can't name | fixtures/reject/src/lib.rs:706:3 |
| 1 | a copy reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust | fixtures/reject/src/lib.rs:2170:3 |
| 1 | a dangling pointer to `_`, a value of no bytes made of more than structs and tuples | fixtures/reject/src/lib.rs:665:11 |
| 1 | a future of `_`, which can be `rt.Pending`, as the future's own `Pending` is | fixtures/reject/src/lib.rs:77:34 |
| 1 | a mapped guard of `_`, which JS can't write in place | fixtures/reject/src/lib.rs:2074:10 |
| 1 | a mutable reference into a `Result`'s payload, passed on or kept | fixtures/reject/src/lib.rs:291:12 |
| 1 | a mutable static, which any code may write | fixtures/reject/src/lib.rs:61:11 |
| 1 | a pointer into `_`, which JS keeps as an object that knows no array it is in | fixtures/reject/src/lib.rs:2081:12 |
| 1 | a pointer into an array of `_`, which JS keeps as an object that knows no array it is in | fixtures/reject/src/lib.rs:2086:30 |
| 1 | a pointer reaching past a value in no array, which JS keeps apart from what holds it in Rust | fixtures/reject/src/lib.rs:2174:27 |
| 1 | a pointer's address, which JS has no counterpart of | fixtures/reject/src/lib.rs:488:17 |
| 1 | a printable type must implement `Display` or `Debug` | fixtures/reject/src/lib.rs:64:1 |
| 1 | a raw pointer to `_` behind a reference, which is the object and not where it is kept | fixtures/reject/src/lib.rs:505:10 |
| 1 | a reference through which a raw pointer may be made into `_` in an element, a payload, or a union field, which is not supported yet | fixtures/reject/src/lib.rs:1286:11 |
| 1 | a scalar constant of type `_` | fixtures/reject/src/lib.rs:8:2 |
| 1 | a slice from a pointer to `_`, which JS keeps as an object that knows no array it is in | fixtures/reject/src/lib.rs:2085:11 |
| 1 | a slice of `()`, which has no bytes, from a pointer | fixtures/reject/src/lib.rs:2094:39 |
| 1 | a slice of one byte's place, which a byte slice can view only in an array of bytes | fixtures/reject/src/lib.rs:2182:2 |
| 1 | a slice reaching past a value in no array, which JS keeps apart from what holds it in Rust | fixtures/reject/src/lib.rs:2173:6 |
| 1 | a trait object of `_`, which is not supported yet | fixtures/reject/src/lib.rs:403:26 |
| 1 | arithmetic on `_` | fixtures/reject/src/lib.rs:8:2 |
| 1 | comparing `_` | fixtures/reject/src/lib.rs:1634:41 |
| 1 | comparing `_` as a key, which is not supported yet | fixtures/reject/src/lib.rs:193:75 |
| 1 | comparing pointers to `_` in an atomic, which JS has no address of | fixtures/reject/src/lib.rs:705:15 |
| 1 | comparing pointers to `_`, which has no bytes and so no address of its own | fixtures/reject/src/lib.rs:656:70 |
| 1 | const generic types are not supported: `_` | fixtures/reject/src/lib.rs:2019:1 |
| 1 | constructing `_` | fixtures/reject/src/lib.rs:278:53 |
| 1 | hashing `_` as a key, which is not supported yet | fixtures/reject/src/lib.rs:193:75 |
| 1 | hashing `_` by its address, which JS has no counterpart of | fixtures/reject/src/lib.rs:1638:5 |
| 1 | hashing `_` through `_`'s own `Hash::hash_slice`, which is not supported yet | fixtures/reject/src/lib.rs:253:16 |
| 1 | union `_`, whose fields overlap in `_` and `f64`, which JS keeps apart | fixtures/reject/src/lib.rs:510:1 |

## By API

The type or module each refused type or path belongs to.

| Count | API | First |
| ---: | --- | --- |
| 39 | `std::cell::UnsafeCell` | fixtures/reject/src/lib.rs:615:15 |
| 27 | `(u32, u32)` | fixtures/reject/src/lib.rs:536:10 |
| 15 | `u32` | fixtures/reject/src/lib.rs:483:27 |
| 11 | `(u8, u8)` | fixtures/reject/src/lib.rs:1194:31 |
| 9 | `[u8; 4]` | fixtures/reject/src/lib.rs:403:26 |
| 8 | `u8` | fixtures/reject/src/lib.rs:211:6 |
| 6 | `f16` | fixtures/reject/src/lib.rs:7:14 |
| 6 | `std::borrow::Cow` | fixtures/reject/src/lib.rs:260:2 |
| 5 | `Gap` | fixtures/reject/src/lib.rs:830:10 |
| 5 | `Option` | fixtures/reject/src/lib.rs:16:15 |
| 5 | `i32` | fixtures/reject/src/lib.rs:2268:7 |
| 4 | `std::pin::Pin` | fixtures/reject/src/lib.rs:91:2 |
| 4 | `std::str` | fixtures/reject/src/lib.rs:368:4 |
| 3 | `Iterator` | fixtures/reject/src/lib.rs:37:4 |
| 3 | `Mid` | fixtures/reject/src/lib.rs:1277:34 |
| 3 | `Outer` | fixtures/reject/src/lib.rs:1277:34 |
| 3 | `Reused` | fixtures/reject/src/lib.rs:21:4 |
| 3 | `dyn std::any::Any` | fixtures/reject/src/lib.rs:454:32 |
| 3 | `std::mem::ManuallyDrop` | fixtures/reject/src/lib.rs:2288:2 |
| 3 | `std::vec::Vec` | fixtures/reject/src/lib.rs:253:16 |
| 2 | `(u16, u16)` | fixtures/reject/src/lib.rs:1098:31 |
| 2 | `Counted` | fixtures/reject/src/lib.rs:318:2 |
| 2 | `InArray` | fixtures/reject/src/lib.rs:1224:9 |
| 2 | `InPayload` | fixtures/reject/src/lib.rs:1236:9 |
| 2 | `InTuple` | fixtures/reject/src/lib.rs:1228:9 |
| 2 | `InWritten` | fixtures/reject/src/lib.rs:1248:9 |
| 2 | `Inside` | fixtures/reject/src/lib.rs:1767:20 |
| 2 | `Poll` | fixtures/reject/src/lib.rs:72:20 |
| 2 | `Scaled` | fixtures/reject/src/lib.rs:386:23 |
| 2 | `[u32; 3]` | fixtures/reject/src/lib.rs:2112:10 |
| 2 | `[u8; 2]` | fixtures/reject/src/lib.rs:1648:37 |
| 2 | `[u8; 3]` | fixtures/reject/src/lib.rs:1648:15 |
| 2 | `dyn Keyed` | fixtures/reject/src/lib.rs:454:32 |
| 2 | `dyn Scaled` | fixtures/reject/src/lib.rs:386:23 |
| 2 | `dyn std::iter::Iterator` | fixtures/reject/src/lib.rs:11:16 |
| 2 | `slice` | fixtures/reject/src/lib.rs:21:4 |
| 2 | `std::any::TypeId` | fixtures/reject/src/lib.rs:466:2 |
| 2 | `std::collections::BTreeSet` | fixtures/reject/src/lib.rs:193:75 |
| 2 | `std::collections::LinkedList` | fixtures/reject/src/lib.rs:2039:4 |
| 2 | `std::mem::MaybeUninit` | fixtures/reject/src/lib.rs:844:19 |
| 2 | `std::ops::Range` | fixtures/reject/src/lib.rs:164:8 |
| 2 | `std::option::Option` | fixtures/reject/src/lib.rs:298:2 |
| 2 | `std::sync::Arc` | fixtures/reject/src/lib.rs:234:2 |
| 2 | `std::sync::MutexGuard` | fixtures/reject/src/lib.rs:83:2 |
| 2 | `std::vec::IntoIter` | fixtures/reject/src/lib.rs:363:17 |
| 1 | `(Dropped, (u8, u8))` | fixtures/reject/src/lib.rs:2333:2 |
| 1 | `([Cloned; 2], (u8, u8))` | fixtures/reject/src/lib.rs:2317:2 |
| 1 | `([Greatest; 2], (u8, u8))` | fixtures/reject/src/lib.rs:2351:13 |
| 1 | `([Touched; 2], (u8, u8))` | fixtures/reject/src/lib.rs:2382:2 |
| 1 | `(std::vec::Vec, Tens)` | fixtures/reject/src/lib.rs:1194:31 |
| 1 | `(u16, ByKeyReplace)` | fixtures/reject/src/lib.rs:1543:55 |
| 1 | `(u16, ByKeySet)` | fixtures/reject/src/lib.rs:1535:51 |
| 1 | `(u16, ByKeyTake)` | fixtures/reject/src/lib.rs:1550:52 |
| 1 | `(u16, ByRefReplace)` | fixtures/reject/src/lib.rs:1519:32 |
| 1 | `(u16, ByRefTake)` | fixtures/reject/src/lib.rs:1527:32 |
| 1 | `(u16, InTuple)` | fixtures/reject/src/lib.rs:1268:33 |
| 1 | `*const [u8; 4]` | fixtures/reject/src/lib.rs:1638:5 |
| 1 | `*mut [u8; 4]` | fixtures/reject/src/lib.rs:1634:41 |
| 1 | `Aliased` | fixtures/reject/src/lib.rs:1042:29 |
| 1 | `Assigned` | fixtures/reject/src/lib.rs:1934:20 |
| 1 | `Bare` | fixtures/reject/src/lib.rs:656:70 |
| 1 | `Batched` | fixtures/reject/src/lib.rs:253:16 |
| 1 | `Behind` | fixtures/reject/src/lib.rs:1673:22 |
| 1 | `Bits` | fixtures/reject/src/lib.rs:510:1 |
| 1 | `Blank` | fixtures/reject/src/lib.rs:689:2 |
| 1 | `Block` | fixtures/reject/src/lib.rs:2019:1 |
| 1 | `ByBox` | fixtures/reject/src/lib.rs:1426:35 |
| 1 | `ByClosure` | fixtures/reject/src/lib.rs:1441:35 |
| 1 | `ByExchanged` | fixtures/reject/src/lib.rs:1487:30 |
| 1 | `ByInstance` | fixtures/reject/src/lib.rs:1467:35 |
| 1 | `ByKeyReplace` | fixtures/reject/src/lib.rs:1543:55 |
| 1 | `ByKeySet` | fixtures/reject/src/lib.rs:1535:51 |
| 1 | `ByKeyTake` | fixtures/reject/src/lib.rs:1550:52 |
| 1 | `ByObject` | fixtures/reject/src/lib.rs:1454:35 |
| 1 | `ByRead` | fixtures/reject/src/lib.rs:1377:35 |
| 1 | `ByRefReplace` | fixtures/reject/src/lib.rs:1519:32 |
| 1 | `ByRefTake` | fixtures/reject/src/lib.rs:1527:32 |
| 1 | `ByReplace` | fixtures/reject/src/lib.rs:1415:35 |
| 1 | `BySwap` | fixtures/reject/src/lib.rs:1403:35 |
| 1 | `BySwapped` | fixtures/reject/src/lib.rs:1476:76 |
| 1 | `ByWrite` | fixtures/reject/src/lib.rs:1390:35 |
| 1 | `BytesMut` | fixtures/reject/src/lib.rs:312:6 |
| 1 | `Called` | fixtures/reject/src/lib.rs:1008:28 |
| 1 | `Chosen` | fixtures/reject/src/lib.rs:1828:17 |
| 1 | `Clone` | fixtures/reject/src/lib.rs:2193:4 |
| 1 | `Closed` | fixtures/reject/src/lib.rs:2000:15 |
| 1 | `Compared` | fixtures/reject/src/lib.rs:1795:22 |
| 1 | `Copied` | fixtures/reject/src/lib.rs:1098:31 |
| 1 | `Deep` | fixtures/reject/src/lib.rs:1709:18 |
| 1 | `Deeper` | fixtures/reject/src/lib.rs:1960:22 |
| 1 | `DoubleEndedIterator` | fixtures/reject/src/lib.rs:150:2 |
| 1 | `Dynamic` | fixtures/reject/src/lib.rs:1984:20 |
| 1 | `Enclosing` | fixtures/reject/src/lib.rs:1812:22 |
| 1 | `Escaped` | fixtures/reject/src/lib.rs:994:43 |
| 1 | `Halves` | fixtures/reject/src/lib.rs:2255:16 |
| 1 | `HashMap` | fixtures/reject/src/lib.rs:211:6 |
| 1 | `Hits` | fixtures/reject/src/lib.rs:648:21 |
| 1 | `InField` | fixtures/reject/src/lib.rs:1240:9 |
| 1 | `InGeneric` | fixtures/reject/src/lib.rs:1244:9 |
| 1 | `InNested` | fixtures/reject/src/lib.rs:1232:9 |
| 1 | `Lane` | fixtures/reject/src/lib.rs:794:19 |
| 1 | `Lanes` | fixtures/reject/src/lib.rs:2114:10 |
| 1 | `Loaded` | fixtures/reject/src/lib.rs:1064:29 |
| 1 | `Looped` | fixtures/reject/src/lib.rs:1032:26 |
| 1 | `Mapped` | fixtures/reject/src/lib.rs:1109:39 |
| 1 | `Maybe` | fixtures/reject/src/lib.rs:31:2 |
| 1 | `Onced` | fixtures/reject/src/lib.rs:1155:46 |
| 1 | `Only` | fixtures/reject/src/lib.rs:665:11 |
| 1 | `Parked` | fixtures/reject/src/lib.rs:2059:20 |
| 1 | `Parking` | fixtures/reject/src/lib.rs:2059:20 |
| 1 | `Plain` | fixtures/reject/src/lib.rs:748:10 |
| 1 | `Pushed` | fixtures/reject/src/lib.rs:1170:22 |
| 1 | `Repeated` | fixtures/reject/src/lib.rs:1143:18 |
| 1 | `Replaced` | fixtures/reject/src/lib.rs:1879:20 |
| 1 | `Rewritten` | fixtures/reject/src/lib.rs:1052:47 |
| 1 | `Shared` | fixtures/reject/src/lib.rs:1750:20 |
| 1 | `Sink` | fixtures/reject/src/lib.rs:55:10 |
| 1 | `Slotted` | fixtures/reject/src/lib.rs:1731:21 |
| 1 | `Span2` | fixtures/reject/src/lib.rs:536:10 |
| 1 | `Swapped` | fixtures/reject/src/lib.rs:1897:20 |
| 1 | `T \| undefined` | fixtures/reject/src/lib.rs:31:2 |
| 1 | `Tail` | fixtures/reject/src/lib.rs:413:30 |
| 1 | `Taken` | fixtures/reject/src/lib.rs:1915:20 |
| 1 | `Target` | fixtures/reject/src/lib.rs:1591:10 |
| 1 | `Tens` | fixtures/reject/src/lib.rs:1194:31 |
| 1 | `Track` | fixtures/reject/src/lib.rs:774:19 |
| 1 | `Unboxed` | fixtures/reject/src/lib.rs:1688:16 |
| 1 | `Waiting` | fixtures/reject/src/lib.rs:424:2 |
| 1 | `Window` | fixtures/reject/src/lib.rs:597:2 |
| 1 | `Wrapped` | fixtures/reject/src/lib.rs:1856:21 |
| 1 | `[InArray; 2]` | fixtures/reject/src/lib.rs:1259:33 |
| 1 | `[InWritten; 1]` | fixtures/reject/src/lib.rs:1320:35 |
| 1 | `[std::cell::UnsafeCell; 2]` | fixtures/reject/src/lib.rs:2013:20 |
| 1 | `[u32; 2]` | fixtures/reject/src/lib.rs:2099:13 |
| 1 | `poll` | fixtures/reject/src/lib.rs:424:2 |
| 1 | `rt.Poll` | fixtures/reject/src/lib.rs:424:2 |
| 1 | `std::any::Any` | fixtures/reject/src/lib.rs:466:4 |
| 1 | `std::cell::Cell` | fixtures/reject/src/lib.rs:877:22 |
| 1 | `std::cell::LazyCell` | fixtures/reject/src/lib.rs:2230:3 |
| 1 | `std::collections::BTreeMap` | fixtures/reject/src/lib.rs:877:22 |
| 1 | `std::collections::HashMap` | fixtures/reject/src/lib.rs:211:6 |
| 1 | `std::default::Default` | fixtures/reject/src/lib.rs:2041:2 |
| 1 | `std::iter` | fixtures/reject/src/lib.rs:1157:22 |
| 1 | `std::mem` | fixtures/reject/src/lib.rs:2198:2 |
| 1 | `std::net::AddrParseError` | fixtures/reject/src/lib.rs:367:32 |
| 1 | `std::net::Ipv4Addr` | fixtures/reject/src/lib.rs:367:32 |
| 1 | `std::num` | fixtures/reject/src/lib.rs:1199:4 |
| 1 | `std::ops::RangeFrom` | fixtures/reject/src/lib.rs:906:32 |
| 1 | `std::ops::RangeTo` | fixtures/reject/src/lib.rs:906:11 |
| 1 | `std::ptr` | fixtures/reject/src/lib.rs:1634:14 |
| 1 | `std::slice::IterMut` | fixtures/reject/src/lib.rs:694:15 |
| 1 | `std::std` | fixtures/reject/src/lib.rs:2282:13 |
| 1 | `std::sync::LazyLock` | fixtures/reject/src/lib.rs:2235:2 |
| 1 | `std::task::Poll` | fixtures/reject/src/lib.rs:77:34 |
| 1 | `std::thread::LocalKey` | fixtures/reject/src/lib.rs:2241:2 |
| 1 | `u64` | fixtures/reject/src/lib.rs:68:2 |
| 1 | `value` | fixtures/reject/src/lib.rs:31:2 |
| 1 | `{closure}` | fixtures/reject/src/lib.rs:2074:10 |

## By type or path

| Count | Construct | Type or path | First |
| ---: | --- | --- | --- |
| 17 | a raw pointer to `_` in a variable assigned again while it may point there | `(u32, u32)` | fixtures/reject/src/lib.rs:544:10 |
| 4 | a variant of `_`, which is not supported: either is the same JS string | `std::borrow::Cow` | fixtures/reject/src/lib.rs:260:2 |
| 3 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `Gap` | fixtures/reject/src/lib.rs:830:10 |
| 3 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `Mid`, `Outer` | fixtures/reject/src/lib.rs:1277:34 |
| 3 | memory from `alloc` cast to pointers to `_` and to `_`, which reads the bytes of one type as another | `i32`, `u32` | fixtures/reject/src/lib.rs:2269:11 |
| 3 | unsupported type `_` | `f16` | fixtures/reject/src/lib.rs:7:14 |
| 2 | `_` is not supported: its `None` and `Some` would both be `rt.None` | `Option` | fixtures/reject/src/lib.rs:16:15 |
| 2 | `_` of `_`, whose `None` is `rt.None`, which is not supported yet | `Iterator::scan`, `Option` | fixtures/reject/src/lib.rs:37:4 |
| 2 | `_`, a trait object of another crate's trait, which is not supported yet | `dyn std::iter::Iterator` | fixtures/reject/src/lib.rs:11:16 |
| 2 | `_`, a trait object of the generic trait `_`, which is not supported yet | `dyn Scaled`, `Scaled` | fixtures/reject/src/lib.rs:386:23 |
| 2 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | `u32`, `u8` | fixtures/reject/src/lib.rs:483:27 |
| 2 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | `u8`, `u32` | fixtures/reject/src/lib.rs:2254:15 |
| 2 | a copy between pointers to `_`, which JS keeps as an object that knows no array it is in | `[u8; 4]` | fixtures/reject/src/lib.rs:1632:3 |
| 2 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::mem::MaybeUninit`, `Gap` | fixtures/reject/src/lib.rs:844:19 |
| 2 | a raw pointer to `_` in a variable assigned again while it may point there | `[u8; 4]` | fixtures/reject/src/lib.rs:1655:26 |
| 2 | a raw pointer to `_` in a variable assigned again while it may point there | `std::cell::UnsafeCell` | fixtures/reject/src/lib.rs:624:15 |
| 2 | a raw pointer to `_` in an element, a payload, or a union field, which is not supported yet | `[u8; 2]` | fixtures/reject/src/lib.rs:1648:37 |
| 2 | a raw pointer to `_` in an element, a payload, or a union field, which is not supported yet | `[u8; 3]` | fixtures/reject/src/lib.rs:1648:15 |
| 2 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Inside` | fixtures/reject/src/lib.rs:1767:20 |
| 2 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `[u8; 4]` | fixtures/reject/src/lib.rs:1655:26 |
| 2 | an assignment through a reference to `_` | `std::pin::Pin` | fixtures/reject/src/lib.rs:107:2 |
| 2 | an offset of a pointer to `_`, which JS keeps as an object that knows no array it is in | `[u8; 4]` | fixtures/reject/src/lib.rs:1635:56 |
| 2 | an upcast from `_` to `_`, which is not supported yet | `dyn Keyed`, `dyn std::any::Any` | fixtures/reject/src/lib.rs:454:32 |
| 2 | comparing pointers to `_`, each a place JS makes of its own | `u32` | fixtures/reject/src/lib.rs:524:2 |
| 2 | memory from `alloc` cast to pointers to `_` and to `_`, which reads the bytes of one type as another | `u32`, `i32` | fixtures/reject/src/lib.rs:2268:7 |
| 2 | replacing the value behind a reference to `_` | `std::pin::Pin` | fixtures/reject/src/lib.rs:91:2 |
| 2 | unsupported call to `_` | `<std::vec::IntoIter as std::iter::Iterator>::sum` | fixtures/reject/src/lib.rs:363:17 |
| 1 | `Buf::copy_to_bytes` of `_`, a value JS copies, which is not supported | `Blank` | fixtures/reject/src/lib.rs:689:2 |
| 1 | `BufMut::put` of `_` into a `BytesMut`, which advances it by the room left in a capacity that depends on whether the other handles of its storage dropped, which is not supported | `Counted` | fixtures/reject/src/lib.rs:323:2 |
| 1 | `BufMut::put` of `_` into a `BytesMut`, which picks the methods it calls by a capacity that depends on whether the other handles of its storage dropped, which is not supported | `Counted` | fixtures/reject/src/lib.rs:318:2 |
| 1 | `Bytes::from_owner` of `_`, which would drop with the last clone of what it makes, which is not supported | `std::sync::Arc` | fixtures/reject/src/lib.rs:303:2 |
| 1 | `Extend` of `_` by `_`, whose `_` of `_` is not supported yet | `(std::vec::Vec, Tens)`, `(u8, u8)`, `Tens`, `u8` | fixtures/reject/src/lib.rs:1194:31 |
| 1 | `_` is not supported: its `Pending` and `Ready` would both be `rt.OuterPending` | `Poll` | fixtures/reject/src/lib.rs:72:20 |
| 1 | `_` is not supported: its field `_` is `_`, typed `_`, whose `None` would be `rt.None` as `_` | `Maybe`, `value`, `Option`, `T \| undefined`, `Option` | fixtures/reject/src/lib.rs:31:2 |
| 1 | `_` is not supported: its field `_` is `_`, typed `_`, whose `Pending` would be `rt.OuterPending` as `_` | `Waiting`, `poll`, `Poll`, `rt.Poll`, `Poll` | fixtures/reject/src/lib.rs:424:2 |
| 1 | `_` of `_` by `_`, which is not supported yet | `HashMap::contains_key`, `std::collections::HashMap`, `u8` | fixtures/reject/src/lib.rs:211:6 |
| 1 | `_` of `_`, which is not supported yet | `std::any::Any::type_id`, `dyn std::any::Any` | fixtures/reject/src/lib.rs:466:4 |
| 1 | `_` of `_`, whose `clone_from` is its own, which is not supported yet | `Clone::clone_from`, `std::vec::Vec` | fixtures/reject/src/lib.rs:2193:4 |
| 1 | `_` of `_`, whose `clone_from` is its own, which is not supported yet | `slice::clone_from_slice`, `Reused` | fixtures/reject/src/lib.rs:229:4 |
| 1 | `_` of `_`, whose `clone_from` is its own, which is not supported yet | `slice::fill`, `Reused` | fixtures/reject/src/lib.rs:21:4 |
| 1 | `_` with a mutable reference to a value JS copies, which its mapping would copy | `std::collections::LinkedList::retain` | fixtures/reject/src/lib.rs:2039:4 |
| 1 | `_` with a mutable reference to a value JS copies, which its mapping would copy | `std::default::Default::default` | fixtures/reject/src/lib.rs:2041:2 |
| 1 | `_` with a mutable reference to a value JS copies, which its mapping would copy | `std::iter::once` | fixtures/reject/src/lib.rs:1157:22 |
| 1 | `_` with a mutable reference to a value JS copies, which its mapping would copy | `std::str::from_utf8_mut` | fixtures/reject/src/lib.rs:1564:20 |
| 1 | `_` with a mutable reference to a value JS copies, which its mapping would copy | `std::str::get_unchecked_mut` | fixtures/reject/src/lib.rs:1559:13 |
| 1 | `_`, which reads a capacity that depends on whether the other handles of its storage dropped, which is not supported | `BytesMut::capacity` | fixtures/reject/src/lib.rs:312:6 |
| 1 | `clone_from` of `_`, whose items' `clone_from` is their own, which is not supported yet | `std::collections::LinkedList` | fixtures/reject/src/lib.rs:2040:4 |
| 1 | `collect` of `_`, which sorts by `_`'s own `Ord`, of entries that drop, which is not supported yet | `std::collections::BTreeMap`, `std::cell::Cell` | fixtures/reject/src/lib.rs:877:22 |
| 1 | `fmt::Arguments` with a placeholder, or not made in place, written to `_`'s own `fmt::Write`, which is not supported yet: std splits what a placeholder writes into `write_str` calls the runtime does not make | `Sink` | fixtures/reject/src/lib.rs:55:10 |
| 1 | `offset_from` of pointers to `_`, which JS keeps as an object that knows no array it is in | `(u8, u8)` | fixtures/reject/src/lib.rs:2087:36 |
| 1 | a cast from `_` to `_` | `u64`, `f16` | fixtures/reject/src/lib.rs:68:2 |
| 1 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | `u32`, `[u32; 2]` | fixtures/reject/src/lib.rs:2099:13 |
| 1 | a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another | `u8`, `Halves` | fixtures/reject/src/lib.rs:2255:16 |
| 1 | a coercion to `_` | `std::sync::Arc` | fixtures/reject/src/lib.rs:234:2 |
| 1 | a coercion to `_`, which is not supported yet | `&Tail` | fixtures/reject/src/lib.rs:413:30 |
| 1 | a constant of type `_` | `std::mem::ManuallyDrop` | fixtures/reject/src/lib.rs:2288:34 |
| 1 | a constant of type `_`, a pointer into memory translation can't name | `&u32` | fixtures/reject/src/lib.rs:706:3 |
| 1 | a copy between pointers to `_`, which JS keeps as an object that knows no array it is in | `(u8, u8)` | fixtures/reject/src/lib.rs:2084:3 |
| 1 | a dangling pointer to `_`, a value of no bytes made of more than structs and tuples | `Only` | fixtures/reject/src/lib.rs:665:11 |
| 1 | a future of `_`, which can be `rt.Pending`, as the future's own `Pending` is | `std::task::Poll` | fixtures/reject/src/lib.rs:77:34 |
| 1 | a mapped guard of `_`, which JS can't write in place | `{closure}` | fixtures/reject/src/lib.rs:2074:10 |
| 1 | a pointer into `_`, which JS keeps as an object that knows no array it is in | `(u8, u8)` | fixtures/reject/src/lib.rs:2081:12 |
| 1 | a pointer into an array of `_`, which JS keeps as an object that knows no array it is in | `(u8, u8)` | fixtures/reject/src/lib.rs:2086:30 |
| 1 | a raw pointer to `_` behind a reference, which is the object and not where it is kept | `std::vec::Vec` | fixtures/reject/src/lib.rs:505:10 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u16, u16)`, `Copied` | fixtures/reject/src/lib.rs:1098:31 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u16, u16)`, `Pushed` | fixtures/reject/src/lib.rs:1170:22 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `Lane` | fixtures/reject/src/lib.rs:794:19 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `Plain` | fixtures/reject/src/lib.rs:748:10 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `Span2` | fixtures/reject/src/lib.rs:536:10 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `Track` | fixtures/reject/src/lib.rs:774:19 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `Window` | fixtures/reject/src/lib.rs:597:2 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u32, u32)`, `std::cell::UnsafeCell` | fixtures/reject/src/lib.rs:615:15 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u8, u8)`, `(Dropped, (u8, u8))` | fixtures/reject/src/lib.rs:2333:2 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u8, u8)`, `([Cloned; 2], (u8, u8))` | fixtures/reject/src/lib.rs:2317:2 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u8, u8)`, `([Greatest; 2], (u8, u8))` | fixtures/reject/src/lib.rs:2351:13 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `(u8, u8)`, `([Touched; 2], (u8, u8))` | fixtures/reject/src/lib.rs:2382:2 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `ByKeyReplace`, `(u16, ByKeyReplace)` | fixtures/reject/src/lib.rs:1543:55 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `ByKeySet`, `(u16, ByKeySet)` | fixtures/reject/src/lib.rs:1535:51 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `ByKeyTake`, `(u16, ByKeyTake)` | fixtures/reject/src/lib.rs:1550:52 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `ByRefReplace`, `(u16, ByRefReplace)` | fixtures/reject/src/lib.rs:1519:32 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `ByRefTake`, `(u16, ByRefTake)` | fixtures/reject/src/lib.rs:1527:32 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `[u32; 3]`, `Lanes` | fixtures/reject/src/lib.rs:2114:10 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Aliased` | fixtures/reject/src/lib.rs:1042:29 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByBox` | fixtures/reject/src/lib.rs:1426:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByClosure` | fixtures/reject/src/lib.rs:1441:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByExchanged` | fixtures/reject/src/lib.rs:1487:30 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByInstance` | fixtures/reject/src/lib.rs:1467:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByObject` | fixtures/reject/src/lib.rs:1454:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByRead` | fixtures/reject/src/lib.rs:1377:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByReplace` | fixtures/reject/src/lib.rs:1415:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `BySwap` | fixtures/reject/src/lib.rs:1403:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `BySwapped` | fixtures/reject/src/lib.rs:1476:76 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `ByWrite` | fixtures/reject/src/lib.rs:1390:35 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Called` | fixtures/reject/src/lib.rs:1008:28 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Escaped` | fixtures/reject/src/lib.rs:994:43 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Hits` | fixtures/reject/src/lib.rs:648:21 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InArray` | fixtures/reject/src/lib.rs:1224:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InField` | fixtures/reject/src/lib.rs:1240:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InGeneric` | fixtures/reject/src/lib.rs:1244:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InNested` | fixtures/reject/src/lib.rs:1232:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InPayload` | fixtures/reject/src/lib.rs:1236:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InTuple` | fixtures/reject/src/lib.rs:1228:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `InWritten` | fixtures/reject/src/lib.rs:1248:9 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Loaded` | fixtures/reject/src/lib.rs:1064:29 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Looped` | fixtures/reject/src/lib.rs:1032:26 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Mapped` | fixtures/reject/src/lib.rs:1109:39 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Onced` | fixtures/reject/src/lib.rs:1155:46 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Repeated` | fixtures/reject/src/lib.rs:1143:18 |
| 1 | a raw pointer to `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Rewritten` | fixtures/reject/src/lib.rs:1052:47 |
| 1 | a raw pointer to `_` in a variable assigned again while it may point there | `Target` | fixtures/reject/src/lib.rs:1591:10 |
| 1 | a raw pointer to `_` in a variable assigned again while it may point there | `[u32; 3]` | fixtures/reject/src/lib.rs:2112:10 |
| 1 | a raw pointer to `_` in an element, a payload, or a union field, which is not supported yet | `(u32, u32)` | fixtures/reject/src/lib.rs:607:10 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `InArray`, `[InArray; 2]` | fixtures/reject/src/lib.rs:1259:33 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `InTuple`, `(u16, InTuple)` | fixtures/reject/src/lib.rs:1268:33 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `InWritten`, `[InWritten; 1]` | fixtures/reject/src/lib.rs:1320:35 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `Parked`, `Parking` | fixtures/reject/src/lib.rs:2059:20 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Assigned` | fixtures/reject/src/lib.rs:1934:20 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Closed` | fixtures/reject/src/lib.rs:2000:15 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Deeper` | fixtures/reject/src/lib.rs:1960:22 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Dynamic` | fixtures/reject/src/lib.rs:1984:20 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Replaced` | fixtures/reject/src/lib.rs:1879:20 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Swapped` | fixtures/reject/src/lib.rs:1897:20 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `Taken` | fixtures/reject/src/lib.rs:1915:20 |
| 1 | a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of | `std::cell::UnsafeCell`, `[std::cell::UnsafeCell; 2]` | fixtures/reject/src/lib.rs:2013:20 |
| 1 | a reference through which a raw pointer may be made into `_` in an element, a payload, or a union field, which is not supported yet | `InPayload` | fixtures/reject/src/lib.rs:1286:11 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Behind` | fixtures/reject/src/lib.rs:1673:22 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Chosen` | fixtures/reject/src/lib.rs:1828:17 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Compared` | fixtures/reject/src/lib.rs:1795:22 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Deep` | fixtures/reject/src/lib.rs:1709:18 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Enclosing` | fixtures/reject/src/lib.rs:1812:22 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Shared` | fixtures/reject/src/lib.rs:1750:20 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Slotted` | fixtures/reject/src/lib.rs:1731:21 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Unboxed` | fixtures/reject/src/lib.rs:1688:16 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `Wrapped` | fixtures/reject/src/lib.rs:1856:21 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `std::cell::UnsafeCell` | fixtures/reject/src/lib.rs:1577:18 |
| 1 | a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there | `std::option::Option` | fixtures/reject/src/lib.rs:2030:13 |
| 1 | a scalar constant of type `_` | `f16` | fixtures/reject/src/lib.rs:8:2 |
| 1 | a slice from a pointer to `_`, which JS keeps as an object that knows no array it is in | `(u8, u8)` | fixtures/reject/src/lib.rs:2085:11 |
| 1 | a slice or a cell of a mutable reference to `_` that names no place, which a whole write would replace only there | `Reused` | fixtures/reject/src/lib.rs:2188:2 |
| 1 | a slice or a cell of a mutable reference to `_` that names no place, which a whole write would replace only there | `std::cell::LazyCell` | fixtures/reject/src/lib.rs:2230:3 |
| 1 | a slice or a cell of a mutable reference to `_` that names no place, which a whole write would replace only there | `std::thread::LocalKey` | fixtures/reject/src/lib.rs:2241:2 |
| 1 | a trait object of `_`, which is not supported yet | `[u8; 4]` | fixtures/reject/src/lib.rs:403:26 |
| 1 | an assignment through a reference to `_` | `std::sync::LazyLock` | fixtures/reject/src/lib.rs:2235:2 |
| 1 | an assignment through a reference to `_` | `std::sync::MutexGuard` | fixtures/reject/src/lib.rs:83:2 |
| 1 | an offset of a pointer to `_`, which JS keeps as an object that knows no array it is in | `(u8, u8)` | fixtures/reject/src/lib.rs:2083:13 |
| 1 | arithmetic on `_` | `f16` | fixtures/reject/src/lib.rs:8:2 |
| 1 | comparing `_` | `*mut [u8; 4]` | fixtures/reject/src/lib.rs:1634:41 |
| 1 | comparing `_` as a key, which is not supported yet | `std::collections::BTreeSet` | fixtures/reject/src/lib.rs:193:75 |
| 1 | comparing pointers to `_` in an atomic, which JS has no address of | `u32` | fixtures/reject/src/lib.rs:705:15 |
| 1 | comparing pointers to `_`, which has no bytes and so no address of its own | `Bare` | fixtures/reject/src/lib.rs:656:70 |
| 1 | const generic types are not supported: `_` | `Block` | fixtures/reject/src/lib.rs:2019:1 |
| 1 | constructing `_` | `std::borrow::Cow` | fixtures/reject/src/lib.rs:278:53 |
| 1 | hashing `_` as a key, which is not supported yet | `std::collections::BTreeSet` | fixtures/reject/src/lib.rs:193:75 |
| 1 | hashing `_` by its address, which JS has no counterpart of | `*const [u8; 4]` | fixtures/reject/src/lib.rs:1638:5 |
| 1 | hashing `_` through `_`'s own `Hash::hash_slice`, which is not supported yet | `std::vec::Vec`, `Batched` | fixtures/reject/src/lib.rs:253:16 |
| 1 | overriding `_`, which std's adaptors call in place of `next` or beside it | `DoubleEndedIterator::nth_back` | fixtures/reject/src/lib.rs:150:2 |
| 1 | overriding `_`, which std's adaptors call in place of `next` or beside it | `Iterator::fold` | fixtures/reject/src/lib.rs:125:2 |
| 1 | replacing the value behind a reference to `_` | `std::sync::MutexGuard` | fixtures/reject/src/lib.rs:112:2 |
| 1 | union `_`, whose fields overlap in `_` and `f64`, which JS keeps apart | `Bits`, `u32` | fixtures/reject/src/lib.rs:510:1 |
| 1 | unsupported call to `_` | `<std::any::TypeId as std::cmp::PartialEq>::eq` | fixtures/reject/src/lib.rs:466:2 |
| 1 | unsupported call to `_` | `<std::mem::ManuallyDrop as std::cmp::PartialEq>::eq` | fixtures/reject/src/lib.rs:2288:25 |
| 1 | unsupported call to `_` | `<std::mem::ManuallyDrop as std::fmt::Debug>::fmt` | fixtures/reject/src/lib.rs:2288:2 |
| 1 | unsupported call to `_` | `<std::ops::RangeFrom as std::ops::RangeBounds>::contains` | fixtures/reject/src/lib.rs:906:32 |
| 1 | unsupported call to `_` | `<std::option::Option as std::cmp::PartialEq>::eq` | fixtures/reject/src/lib.rs:298:2 |
| 1 | unsupported call to `_` | `<std::slice::IterMut as std::iter::Iterator>::is_sorted` | fixtures/reject/src/lib.rs:694:15 |
| 1 | unsupported call to `_` | `<u8 as std::hash::Hash>::hash` | fixtures/reject/src/lib.rs:244:10 |
| 1 | unsupported call to `_` | `std::any::TypeId::of` | fixtures/reject/src/lib.rs:466:17 |
| 1 | unsupported call to `_` | `std::borrow::Cow::is_borrowed` | fixtures/reject/src/lib.rs:272:2 |
| 1 | unsupported call to `_` | `std::mem::size_of_val` | fixtures/reject/src/lib.rs:2198:2 |
| 1 | unsupported call to `_` | `std::num::is_ascii_octdigit` | fixtures/reject/src/lib.rs:1199:4 |
| 1 | unsupported call to `_` | `std::ops::Range::contains` | fixtures/reject/src/lib.rs:164:8 |
| 1 | unsupported call to `_` | `std::ops::Range::is_empty` | fixtures/reject/src/lib.rs:911:9 |
| 1 | unsupported call to `_` | `std::ops::RangeTo::contains` | fixtures/reject/src/lib.rs:906:11 |
| 1 | unsupported call to `_` | `std::ptr::addr_eq` | fixtures/reject/src/lib.rs:1634:14 |
| 1 | unsupported call to `_` | `std::std::alloc_zeroed` | fixtures/reject/src/lib.rs:2282:13 |
| 1 | unsupported call to `_` | `std::str::as_bytes_mut` | fixtures/reject/src/lib.rs:1558:13 |
| 1 | unsupported call to `_` | `std::str::parse` | fixtures/reject/src/lib.rs:368:4 |
| 1 | unsupported type `_` from another crate | `std::net::AddrParseError` | fixtures/reject/src/lib.rs:367:32 |
| 1 | unsupported type `_` from another crate | `std::net::Ipv4Addr` | fixtures/reject/src/lib.rs:367:32 |

## Spans

<details><summary>a raw pointer to `_` in `_`, which the crate may put another object in place of (50)</summary>

- fixtures/reject/src/lib.rs:536:10: a raw pointer to `(u32, u32)` in `Span2`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:597:2: a raw pointer to `(u32, u32)` in `Window`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:615:15: a raw pointer to `(u32, u32)` in `std::cell::UnsafeCell<(u32, u32)>`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:648:21: a raw pointer to `std::cell::UnsafeCell<u32>` in `Hits`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:748:10: a raw pointer to `(u32, u32)` in `Plain`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:774:19: a raw pointer to `(u32, u32)` in `Track`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:794:19: a raw pointer to `(u32, u32)` in `Lane`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:830:10: a raw pointer to `(u32, u32)` in `Gap`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:831:10: a raw pointer to `(u32, u32)` in `Gap`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:832:58: a raw pointer to `(u32, u32)` in `Gap`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:844:19: a raw pointer to `std::mem::MaybeUninit<(u32, u32)>` in `Gap`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:845:19: a raw pointer to `std::mem::MaybeUninit<(u32, u32)>` in `Gap`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:994:43: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Escaped`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1008:28: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Called`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1032:26: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Looped`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1042:29: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Aliased`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1052:47: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Rewritten`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1064:29: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Loaded`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1098:31: a raw pointer to `(u16, u16)` in `Copied`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1109:39: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Mapped`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1143:18: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Repeated`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1155:46: a raw pointer to `std::cell::UnsafeCell<(u16, u16)>` in `Onced`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1170:22: a raw pointer to `(u16, u16)` in `Pushed`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1224:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InArray`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1228:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InTuple`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1232:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InNested`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1236:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InPayload`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1240:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InField`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1244:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InGeneric`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1248:9: a raw pointer to `std::cell::UnsafeCell<u16>` in `InWritten`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1377:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByRead`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1390:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByWrite`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1403:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `BySwap`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1415:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByReplace`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1426:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByBox`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1441:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByClosure`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1454:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByObject`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1467:35: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByInstance`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1476:76: a raw pointer to `std::cell::UnsafeCell<u16>` in `BySwapped`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1487:30: a raw pointer to `std::cell::UnsafeCell<u16>` in `ByExchanged`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1519:32: a raw pointer to `ByRefReplace` in `(u16, ByRefReplace)`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1527:32: a raw pointer to `ByRefTake` in `(u16, ByRefTake)`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1535:51: a raw pointer to `ByKeySet` in `(u16, ByKeySet)`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1543:55: a raw pointer to `ByKeyReplace` in `(u16, ByKeyReplace)`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1550:52: a raw pointer to `ByKeyTake` in `(u16, ByKeyTake)`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2114:10: a raw pointer to `[u32; 3]` in `Lanes`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2317:2: a raw pointer to `(u8, u8)` in `([Cloned; 2], (u8, u8))`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2333:2: a raw pointer to `(u8, u8)` in `(Dropped, (u8, u8))`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2351:13: a raw pointer to `(u8, u8)` in `([Greatest; 2], (u8, u8))`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2382:2: a raw pointer to `(u8, u8)` in `([Touched; 2], (u8, u8))`, which the crate may put another object in place of

</details>

<details><summary>a raw pointer to `_` in a variable assigned again while it may point there (23)</summary>

- fixtures/reject/src/lib.rs:544:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:552:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:561:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:569:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:578:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:586:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:624:15: a raw pointer to `std::cell::UnsafeCell<u32>` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:637:15: a raw pointer to `std::cell::UnsafeCell<u32>` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:923:26: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:936:22: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:946:12: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:955:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:965:17: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:970:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:1591:10: a raw pointer to `Target` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:1604:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:1619:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:1655:26: a raw pointer to `[u8; 4]` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:1656:10: a raw pointer to `[u8; 4]` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:2112:10: a raw pointer to `[u32; 3]` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:2204:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:2212:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there
- fixtures/reject/src/lib.rs:2220:10: a raw pointer to `(u32, u32)` in a variable assigned again while it may point there

</details>

<details><summary>unsupported call to `_` (20)</summary>

- fixtures/reject/src/lib.rs:164:8: unsupported call to `std::ops::Range::<Key>::contains::<Key>`
- fixtures/reject/src/lib.rs:244:10: unsupported call to `<u8 as std::hash::Hash>::hash::<H>`
- fixtures/reject/src/lib.rs:272:2: unsupported call to `std::borrow::Cow::<'_, [u8]>::is_borrowed`
- fixtures/reject/src/lib.rs:298:2: unsupported call to `<std::option::Option<&mut u32> as std::cmp::PartialEq>::eq`
- fixtures/reject/src/lib.rs:363:17: unsupported call to `<std::vec::IntoIter<Part> as std::iter::Iterator>::sum::<std::time::Duration>`
- fixtures/reject/src/lib.rs:363:38: unsupported call to `<std::vec::IntoIter<Part> as std::iter::Iterator>::sum::<u64>`
- fixtures/reject/src/lib.rs:368:4: unsupported call to `core::str::<impl str>::parse::<std::net::Ipv4Addr>`
- fixtures/reject/src/lib.rs:466:2: unsupported call to `<std::any::TypeId as std::cmp::PartialEq>::eq`
- fixtures/reject/src/lib.rs:466:17: unsupported call to `std::any::TypeId::of::<u8>`
- fixtures/reject/src/lib.rs:694:15: unsupported call to `<std::slice::IterMut<'_, u8> as std::iter::Iterator>::is_sorted`
- fixtures/reject/src/lib.rs:906:11: unsupported call to `std::ops::RangeTo::<u8>::contains::<Key>`
- fixtures/reject/src/lib.rs:906:32: unsupported call to `<std::ops::RangeFrom<u8> as std::ops::RangeBounds<u8>>::contains::<Key>`
- fixtures/reject/src/lib.rs:911:9: unsupported call to `std::ops::Range::<&mut u8>::is_empty`
- fixtures/reject/src/lib.rs:1199:4: unsupported call to `core::num::<impl u8>::is_ascii_octdigit`
- fixtures/reject/src/lib.rs:1558:13: unsupported call to `core::str::<impl str>::as_bytes_mut`
- fixtures/reject/src/lib.rs:1634:14: unsupported call to `std::ptr::addr_eq::<[u8; 4], [u8; 4]>`
- fixtures/reject/src/lib.rs:2198:2: unsupported call to `std::mem::size_of_val::<dyn std::fmt::Debug>`
- fixtures/reject/src/lib.rs:2282:13: unsupported call to `std::alloc::alloc_zeroed`
- fixtures/reject/src/lib.rs:2288:2: unsupported call to `<std::mem::ManuallyDrop<u32> as std::fmt::Debug>::fmt`
- fixtures/reject/src/lib.rs:2288:25: unsupported call to `<std::mem::ManuallyDrop<u32> as std::cmp::PartialEq>::eq`

</details>

<details><summary>a reference through which a raw pointer may be made into `_` in `_`, which the crate may put another object in place of (15)</summary>

- fixtures/reject/src/lib.rs:1259:33: a reference through which a raw pointer may be made into `InArray` in `[InArray; 2]`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1268:33: a reference through which a raw pointer may be made into `InTuple` in `(u16, InTuple)`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1277:34: a reference through which a raw pointer may be made into `Mid<InNested>` in `Outer<InNested>`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1298:33: a reference through which a raw pointer may be made into `Mid<InField>` in `Outer<InField>`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1311:35: a reference through which a raw pointer may be made into `Mid<InGeneric>` in `Outer<InGeneric>`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1320:35: a reference through which a raw pointer may be made into `InWritten` in `[InWritten; 1]`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1879:20: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Replaced`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1897:20: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Swapped`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1915:20: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Taken`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1934:20: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Assigned`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1960:22: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Deeper`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:1984:20: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Dynamic`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2000:15: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `Closed`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2013:20: a reference through which a raw pointer may be made into `std::cell::UnsafeCell<u16>` in `[std::cell::UnsafeCell<u16>; 2]`, which the crate may put another object in place of
- fixtures/reject/src/lib.rs:2059:20: a reference through which a raw pointer may be made into `Parked` in `Parking<T>`, which the crate may put another object in place of

</details>

<details><summary>a reference to `_`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there (15)</summary>

- fixtures/reject/src/lib.rs:1577:18: a reference to `std::cell::UnsafeCell<u64>`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1655:26: a reference to `[u8; 4]`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1673:22: a reference to `Behind`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1688:16: a reference to `Unboxed`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1709:18: a reference to `Deep`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1731:21: a reference to `Slotted`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1750:20: a reference to `Shared`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1767:20: a reference to `Inside`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1780:26: a reference to `Inside`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1795:22: a reference to `Compared`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1812:22: a reference to `Enclosing`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1828:17: a reference to `Chosen`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1841:19: a reference to `[u8; 4]`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:1856:21: a reference to `Wrapped<std::cell::Cell<u32>>`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there
- fixtures/reject/src/lib.rs:2030:13: a reference to `std::option::Option<Block<2>>`, through which a raw pointer may be made, in a variable assigned again while that pointer may point there

</details>

<details><summary>a cast from a pointer to `_` to one to `_`, which reads the bytes of one type as another (6)</summary>

- fixtures/reject/src/lib.rs:483:27: a cast from a pointer to `u32` to one to `u8`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2099:13: a cast from a pointer to `u32` to one to `[u32; 2]`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2254:15: a cast from a pointer to `u8` to one to `u32`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2255:16: a cast from a pointer to `u8` to one to `Halves`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2257:15: a cast from a pointer to `u32` to one to `u8`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2282:12: a cast from a pointer to `u8` to one to `u32`, which reads the bytes of one type as another

</details>

<details><summary>`_` with a mutable reference to a value JS copies, which its mapping would copy (5)</summary>

- fixtures/reject/src/lib.rs:1157:22: `std::iter::once` with a mutable reference to a value JS copies, which its mapping would copy
- fixtures/reject/src/lib.rs:1559:13: `core::str::<impl str>::get_unchecked_mut` with a mutable reference to a value JS copies, which its mapping would copy
- fixtures/reject/src/lib.rs:1564:20: `std::str::from_utf8_mut` with a mutable reference to a value JS copies, which its mapping would copy
- fixtures/reject/src/lib.rs:2039:4: `std::collections::LinkedList::<T, A>::retain` with a mutable reference to a value JS copies, which its mapping would copy
- fixtures/reject/src/lib.rs:2041:2: `std::default::Default::default` with a mutable reference to a value JS copies, which its mapping would copy

</details>

<details><summary>a raw pointer to `_` in an element, a payload, or a union field, which is not supported yet (5)</summary>

- fixtures/reject/src/lib.rs:607:10: a raw pointer to `(u32, u32)` in an element, a payload, or a union field, which is not supported yet
- fixtures/reject/src/lib.rs:1648:15: a raw pointer to `[u8; 3]` in an element, a payload, or a union field, which is not supported yet
- fixtures/reject/src/lib.rs:1648:18: a raw pointer to `[u8; 3]` in an element, a payload, or a union field, which is not supported yet
- fixtures/reject/src/lib.rs:1648:37: a raw pointer to `[u8; 2]` in an element, a payload, or a union field, which is not supported yet
- fixtures/reject/src/lib.rs:1648:44: a raw pointer to `[u8; 2]` in an element, a payload, or a union field, which is not supported yet

</details>

<details><summary>memory from `alloc` cast to pointers to `_` and to `_`, which reads the bytes of one type as another (5)</summary>

- fixtures/reject/src/lib.rs:2268:7: memory from `alloc` cast to pointers to `u32` and to `i32`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2269:11: memory from `alloc` cast to pointers to `i32` and to `u32`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2271:17: memory from `alloc` cast to pointers to `i32` and to `u32`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2272:21: memory from `alloc` cast to pointers to `u32` and to `i32`, which reads the bytes of one type as another
- fixtures/reject/src/lib.rs:2274:17: memory from `alloc` cast to pointers to `i32` and to `u32`, which reads the bytes of one type as another

</details>

<details><summary>a variant of `_`, which is not supported: either is the same JS string (4)</summary>

- fixtures/reject/src/lib.rs:260:2: a variant of `std::borrow::Cow<'_, str>`, which is not supported: either is the same JS string
- fixtures/reject/src/lib.rs:261:30: a variant of `std::borrow::Cow<'_, str>`, which is not supported: either is the same JS string
- fixtures/reject/src/lib.rs:262:27: a variant of `std::borrow::Cow<'_, str>`, which is not supported: either is the same JS string
- fixtures/reject/src/lib.rs:267:2: a variant of `std::borrow::Cow<'_, str>`, which is not supported: either is the same JS string

</details>

<details><summary>an assignment through a reference to `_` (4)</summary>

- fixtures/reject/src/lib.rs:83:2: an assignment through a reference to `std::sync::MutexGuard<'_, u8>`
- fixtures/reject/src/lib.rs:107:2: an assignment through a reference to `std::pin::Pin<std::boxed::Box<dyn std::future::Future<Output = u8>>>`
- fixtures/reject/src/lib.rs:172:2: an assignment through a reference to `std::pin::Pin<&mut Noisy>`
- fixtures/reject/src/lib.rs:2235:2: an assignment through a reference to `std::sync::LazyLock<u32>`

</details>

<details><summary>`_` of `_`, whose `clone_from` is its own, which is not supported yet (3)</summary>

- fixtures/reject/src/lib.rs:21:4: `slice::fill` of `Reused`, whose `clone_from` is its own, which is not supported yet
- fixtures/reject/src/lib.rs:229:4: `slice::clone_from_slice` of `Reused`, whose `clone_from` is its own, which is not supported yet
- fixtures/reject/src/lib.rs:2193:4: `Clone::clone_from` of `std::vec::Vec<Reused>`, whose `clone_from` is its own, which is not supported yet

</details>

<details><summary>a byte of memory from `alloc`, which JS keeps as the values a cast of its pointer types it as (3)</summary>

- fixtures/reject/src/lib.rs:496:3: a byte of memory from `alloc`, which JS keeps as the values a cast of its pointer types it as
- fixtures/reject/src/lib.rs:497:11: a byte of memory from `alloc`, which JS keeps as the values a cast of its pointer types it as
- fixtures/reject/src/lib.rs:2253:3: a byte of memory from `alloc`, which JS keeps as the values a cast of its pointer types it as

</details>

<details><summary>a copy between pointers to `_`, which JS keeps as an object that knows no array it is in (3)</summary>

- fixtures/reject/src/lib.rs:1632:3: a copy between pointers to `[u8; 4]`, which JS keeps as an object that knows no array it is in
- fixtures/reject/src/lib.rs:1633:3: a copy between pointers to `[u8; 4]`, which JS keeps as an object that knows no array it is in
- fixtures/reject/src/lib.rs:2084:3: a copy between pointers to `(u8, u8)`, which JS keeps as an object that knows no array it is in

</details>

<details><summary>a copy of a closure that changes what it captured (3)</summary>

- fixtures/reject/src/lib.rs:182:14: a copy of a closure that changes what it captured
- fixtures/reject/src/lib.rs:436:14: a copy of a closure that changes what it captured
- fixtures/reject/src/lib.rs:437:14: a copy of a closure that changes what it captured

</details>

<details><summary>a slice or a cell of a mutable reference to `_` that names no place, which a whole write would replace only there (3)</summary>

- fixtures/reject/src/lib.rs:2188:2: a slice or a cell of a mutable reference to `Reused` that names no place, which a whole write would replace only there
- fixtures/reject/src/lib.rs:2230:3: a slice or a cell of a mutable reference to `std::cell::LazyCell<u32>` that names no place, which a whole write would replace only there
- fixtures/reject/src/lib.rs:2241:2: a slice or a cell of a mutable reference to `std::thread::LocalKey<u32>` that names no place, which a whole write would replace only there

</details>

<details><summary>an offset of a pointer to `_`, which JS keeps as an object that knows no array it is in (3)</summary>

- fixtures/reject/src/lib.rs:1635:56: an offset of a pointer to `[u8; 4]`, which JS keeps as an object that knows no array it is in
- fixtures/reject/src/lib.rs:1635:73: an offset of a pointer to `[u8; 4]`, which JS keeps as an object that knows no array it is in
- fixtures/reject/src/lib.rs:2083:13: an offset of a pointer to `(u8, u8)`, which JS keeps as an object that knows no array it is in

</details>

<details><summary>replacing the value behind a reference to `_` (3)</summary>

- fixtures/reject/src/lib.rs:91:2: replacing the value behind a reference to `std::pin::Pin<std::boxed::Box<std::future::PollFn<for<'a, 'b> fn(&'a mut std::task::Context<'b>) -> std::task::Poll<u8>>>>`
- fixtures/reject/src/lib.rs:99:2: replacing the value behind a reference to `std::pin::Pin<std::boxed::Box<[u8; 2]>>`
- fixtures/reject/src/lib.rs:112:2: replacing the value behind a reference to `std::sync::MutexGuard<'_, u8>`

</details>

<details><summary>unsupported type `_` (3)</summary>

- fixtures/reject/src/lib.rs:7:14: unsupported type `f16`
- fixtures/reject/src/lib.rs:7:25: unsupported type `f16`
- fixtures/reject/src/lib.rs:66:51: unsupported type `f16`

</details>

<details><summary>`_` is not supported: its `None` and `Some` would both be `rt.None` (2)</summary>

- fixtures/reject/src/lib.rs:16:15: `Option<std::option::Option<std::option::Option<u8>>>` is not supported: its `None` and `Some` would both be `rt.None`
- fixtures/reject/src/lib.rs:17:2: `Option<std::option::Option<std::option::Option<u8>>>` is not supported: its `None` and `Some` would both be `rt.None`

</details>

<details><summary>`_` of `_`, whose `None` is `rt.None`, which is not supported yet (2)</summary>

- fixtures/reject/src/lib.rs:37:4: `Iterator::scan` of `Option<std::option::Option<u8>>`, whose `None` is `rt.None`, which is not supported yet
- fixtures/reject/src/lib.rs:188:11: `Iterator::scan` of `Option<()>`, whose `None` is `rt.None`, which is not supported yet

</details>

<details><summary>`_`, a trait object of another crate's trait, which is not supported yet (2)</summary>

- fixtures/reject/src/lib.rs:11:16: `dyn std::iter::Iterator<Item = u8>`, a trait object of another crate's trait, which is not supported yet
- fixtures/reject/src/lib.rs:12:4: `dyn std::iter::Iterator<Item = u8>`, a trait object of another crate's trait, which is not supported yet

</details>

<details><summary>`_`, a trait object of the generic trait `_`, which is not supported yet (2)</summary>

- fixtures/reject/src/lib.rs:386:23: `dyn Scaled<u8>`, a trait object of the generic trait `Scaled`, which is not supported yet
- fixtures/reject/src/lib.rs:387:4: `dyn Scaled<u8>`, a trait object of the generic trait `Scaled`, which is not supported yet

</details>

<details><summary>`offset_from` between a pointer into a value in no array and one into a value in no array, which JS keeps in different arrays (2)</summary>

- fixtures/reject/src/lib.rs:2147:5: `offset_from` between a pointer into a value in no array and one into a value in no array, which JS keeps in different arrays
- fixtures/reject/src/lib.rs:2148:26: `offset_from` between a pointer into a value in no array and one into a value in no array, which JS keeps in different arrays

</details>

<details><summary>`offset_from` between a pointer into an array of this body's own and one into an array of this body's own, which JS keeps in different arrays (2)</summary>

- fixtures/reject/src/lib.rs:2149:24: `offset_from` between a pointer into an array of this body's own and one into an array of this body's own, which JS keeps in different arrays
- fixtures/reject/src/lib.rs:2150:23: `offset_from` between a pointer into an array of this body's own and one into an array of this body's own, which JS keeps in different arrays

</details>

<details><summary>a pointer reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust (2)</summary>

- fixtures/reject/src/lib.rs:2172:18: a pointer reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust
- fixtures/reject/src/lib.rs:2175:21: a pointer reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust

</details>

<details><summary>a raw or mutable slice of a byte in no array of bytes (2)</summary>

- fixtures/reject/src/lib.rs:2158:12: a raw or mutable slice of a byte in no array of bytes
- fixtures/reject/src/lib.rs:2159:19: a raw or mutable slice of a byte in no array of bytes

</details>

<details><summary>a raw pointer to a place of `()`, which has no address of its own (2)</summary>

- fixtures/reject/src/lib.rs:656:15: a raw pointer to a place of `()`, which has no address of its own
- fixtures/reject/src/lib.rs:656:29: a raw pointer to a place of `()`, which has no address of its own

</details>

<details><summary>an upcast from `_` to `_`, which is not supported yet (2)</summary>

- fixtures/reject/src/lib.rs:454:32: an upcast from `dyn Keyed` to `dyn std::any::Any`, which is not supported yet
- fixtures/reject/src/lib.rs:460:46: an upcast from `dyn Keyed` to `dyn std::any::Any`, which is not supported yet

</details>

<details><summary>comparing pointers to `_`, each a place JS makes of its own (2)</summary>

- fixtures/reject/src/lib.rs:524:2: comparing pointers to `u32`, each a place JS makes of its own
- fixtures/reject/src/lib.rs:2067:2: comparing pointers to `u32`, each a place JS makes of its own

</details>

<details><summary>overriding `_`, which std's adaptors call in place of `next` or beside it (2)</summary>

- fixtures/reject/src/lib.rs:125:2: overriding `Iterator::fold`, which std's adaptors call in place of `next` or beside it
- fixtures/reject/src/lib.rs:150:2: overriding `DoubleEndedIterator::nth_back`, which std's adaptors call in place of `next` or beside it

</details>

<details><summary>unsupported type `_` from another crate (2)</summary>

- fixtures/reject/src/lib.rs:367:32: unsupported type `std::net::AddrParseError` from another crate
- fixtures/reject/src/lib.rs:367:32: unsupported type `std::net::Ipv4Addr` from another crate

</details>

<details><summary>`Buf::copy_to_bytes` of `_`, a value JS copies, which is not supported (1)</summary>

- fixtures/reject/src/lib.rs:689:2: `Buf::copy_to_bytes` of `Blank`, a value JS copies, which is not supported

</details>

<details><summary>`BufMut::put` of `_` into a `BytesMut`, which advances it by the room left in a capacity that depends on whether the other handles of its storage dropped, which is not supported (1)</summary>

- fixtures/reject/src/lib.rs:323:2: `BufMut::put` of `Counted` into a `BytesMut`, which advances it by the room left in a capacity that depends on whether the other handles of its storage dropped, which is not supported

</details>

<details><summary>`BufMut::put` of `_` into a `BytesMut`, which picks the methods it calls by a capacity that depends on whether the other handles of its storage dropped, which is not supported (1)</summary>

- fixtures/reject/src/lib.rs:318:2: `BufMut::put` of `Counted` into a `BytesMut`, which picks the methods it calls by a capacity that depends on whether the other handles of its storage dropped, which is not supported

</details>

<details><summary>`Bytes::from_owner` of `_`, which would drop with the last clone of what it makes, which is not supported (1)</summary>

- fixtures/reject/src/lib.rs:303:2: `Bytes::from_owner` of `std::sync::Arc<[u8]>`, which would drop with the last clone of what it makes, which is not supported

</details>

<details><summary>`Extend` of `_` by `_`, whose `_` of `_` is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:1194:31: `Extend` of `(std::vec::Vec<u8>, Tens)` by `(u8, u8)`, whose `Tens` of `u8` is not supported yet

</details>

<details><summary>`Pointer` formatting, as in `{:p}`, is not supported (1)</summary>

- fixtures/reject/src/lib.rs:49:2: `Pointer` formatting, as in `{:p}`, is not supported

</details>

<details><summary>`_` is not supported: its `Pending` and `Ready` would both be `rt.OuterPending` (1)</summary>

- fixtures/reject/src/lib.rs:72:20: `Poll<std::option::Option<std::task::Poll<std::task::Poll<u8>>>>` is not supported: its `Pending` and `Ready` would both be `rt.OuterPending`

</details>

<details><summary>`_` is not supported: its field `_` is `_`, typed `_`, whose `None` would be `rt.None` as `_` (1)</summary>

- fixtures/reject/src/lib.rs:31:2: `Maybe<()>` is not supported: its field `value` is `Option<T>`, typed `T | undefined`, whose `None` would be `rt.None` as `Option<()>`

</details>

<details><summary>`_` is not supported: its field `_` is `_`, typed `_`, whose `Pending` would be `rt.OuterPending` as `_` (1)</summary>

- fixtures/reject/src/lib.rs:424:2: `Waiting<std::task::Poll<u8>>` is not supported: its field `poll` is `Poll<T>`, typed `rt.Poll<T>`, whose `Pending` would be `rt.OuterPending` as `Poll<std::task::Poll<u8>>`

</details>

<details><summary>`_` of `_` by `_`, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:211:6: `HashMap::contains_key` of `std::collections::HashMap<Id, u8>` by `u8`, which is not supported yet

</details>

<details><summary>`_` of `_`, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:466:4: `std::any::Any::type_id` of `dyn std::any::Any`, which is not supported yet

</details>

<details><summary>`_`, which reads a capacity that depends on whether the other handles of its storage dropped, which is not supported (1)</summary>

- fixtures/reject/src/lib.rs:312:6: `BytesMut::capacity`, which reads a capacity that depends on whether the other handles of its storage dropped, which is not supported

</details>

<details><summary>`clone_from` of `_`, whose items' `clone_from` is their own, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:2040:4: `clone_from` of `std::collections::LinkedList<Reused>`, whose items' `clone_from` is their own, which is not supported yet

</details>

<details><summary>`collect` of `_`, which sorts by `_`'s own `Ord`, of entries that drop, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:877:22: `collect` of `std::collections::BTreeMap<std::cell::Cell<Ranked>, Noisy>`, which sorts by `std::cell::Cell<Ranked>`'s own `Ord`, of entries that drop, which is not supported yet

</details>

<details><summary>`fmt::Arguments` with a placeholder, or not made in place, written to `_`'s own `fmt::Write`, which is not supported yet: std splits what a placeholder writes into `write_str` calls the runtime does not make (1)</summary>

- fixtures/reject/src/lib.rs:55:10: `fmt::Arguments` with a placeholder, or not made in place, written to `Sink`'s own `fmt::Write`, which is not supported yet: std splits what a placeholder writes into `write_str` calls the runtime does not make

</details>

<details><summary>`offset_from` between a pointer into a shared slice JS copies and one into a value in no array, which JS keeps in different arrays (1)</summary>

- fixtures/reject/src/lib.rs:2152:21: `offset_from` between a pointer into a shared slice JS copies and one into a value in no array, which JS keeps in different arrays

</details>

<details><summary>`offset_from` between a pointer into a shared slice JS copies and one into an array of this body's own, which JS keeps in different arrays (1)</summary>

- fixtures/reject/src/lib.rs:2151:20: `offset_from` between a pointer into a shared slice JS copies and one into an array of this body's own, which JS keeps in different arrays

</details>

<details><summary>`offset_from` of pointers to `()`, which has no bytes (1)</summary>

- fixtures/reject/src/lib.rs:2094:13: `offset_from` of pointers to `()`, which has no bytes

</details>

<details><summary>`offset_from` of pointers to `_`, which JS keeps as an object that knows no array it is in (1)</summary>

- fixtures/reject/src/lib.rs:2087:36: `offset_from` of pointers to `(u8, u8)`, which JS keeps as an object that knows no array it is in

</details>

<details><summary>a cast from `_` to `_` (1)</summary>

- fixtures/reject/src/lib.rs:68:2: a cast from `u64` to `f16`

</details>

<details><summary>a coercion to `_` (1)</summary>

- fixtures/reject/src/lib.rs:234:2: a coercion to `std::sync::Arc<[u8]>`

</details>

<details><summary>a coercion to `_`, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:413:30: a coercion to `&Tail<dyn Reading>`, which is not supported yet

</details>

<details><summary>a constant of type `_` (1)</summary>

- fixtures/reject/src/lib.rs:2288:34: a constant of type `std::mem::ManuallyDrop<u32>`

</details>

<details><summary>a constant of type `_`, a pointer into memory translation can't name (1)</summary>

- fixtures/reject/src/lib.rs:706:3: a constant of type `&u32`, a pointer into memory translation can't name

</details>

<details><summary>a copy reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust (1)</summary>

- fixtures/reject/src/lib.rs:2170:3: a copy reaching past a shared slice JS copies, which JS keeps apart from what holds it in Rust

</details>

<details><summary>a dangling pointer to `_`, a value of no bytes made of more than structs and tuples (1)</summary>

- fixtures/reject/src/lib.rs:665:11: a dangling pointer to `Only`, a value of no bytes made of more than structs and tuples

</details>

<details><summary>a future of `_`, which can be `rt.Pending`, as the future's own `Pending` is (1)</summary>

- fixtures/reject/src/lib.rs:77:34: a future of `std::task::Poll<u8>`, which can be `rt.Pending`, as the future's own `Pending` is

</details>

<details><summary>a mapped guard of `_`, which JS can't write in place (1)</summary>

- fixtures/reject/src/lib.rs:2074:10: a mapped guard of `{closure@fixtures/reject/src/lib.rs:2073:38: 2073:50}`, which JS can't write in place

</details>

<details><summary>a mutable reference into a `Result`'s payload, passed on or kept (1)</summary>

- fixtures/reject/src/lib.rs:291:12: a mutable reference into a `Result`'s payload, passed on or kept

</details>

<details><summary>a mutable static, which any code may write (1)</summary>

- fixtures/reject/src/lib.rs:61:11: a mutable static, which any code may write

</details>

<details><summary>a pointer into `_`, which JS keeps as an object that knows no array it is in (1)</summary>

- fixtures/reject/src/lib.rs:2081:12: a pointer into `(u8, u8)`, which JS keeps as an object that knows no array it is in

</details>

<details><summary>a pointer into an array of `_`, which JS keeps as an object that knows no array it is in (1)</summary>

- fixtures/reject/src/lib.rs:2086:30: a pointer into an array of `(u8, u8)`, which JS keeps as an object that knows no array it is in

</details>

<details><summary>a pointer reaching past a value in no array, which JS keeps apart from what holds it in Rust (1)</summary>

- fixtures/reject/src/lib.rs:2174:27: a pointer reaching past a value in no array, which JS keeps apart from what holds it in Rust

</details>

<details><summary>a pointer's address, which JS has no counterpart of (1)</summary>

- fixtures/reject/src/lib.rs:488:17: a pointer's address, which JS has no counterpart of

</details>

<details><summary>a printable type must implement `Display` or `Debug` (1)</summary>

- fixtures/reject/src/lib.rs:64:1: a printable type must implement `Display` or `Debug`

</details>

<details><summary>a raw pointer to `_` behind a reference, which is the object and not where it is kept (1)</summary>

- fixtures/reject/src/lib.rs:505:10: a raw pointer to `std::vec::Vec<u8>` behind a reference, which is the object and not where it is kept

</details>

<details><summary>a reference through which a raw pointer may be made into `_` in an element, a payload, or a union field, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:1286:11: a reference through which a raw pointer may be made into `InPayload` in an element, a payload, or a union field, which is not supported yet

</details>

<details><summary>a scalar constant of type `_` (1)</summary>

- fixtures/reject/src/lib.rs:8:2: a scalar constant of type `f16`

</details>

<details><summary>a slice from a pointer to `_`, which JS keeps as an object that knows no array it is in (1)</summary>

- fixtures/reject/src/lib.rs:2085:11: a slice from a pointer to `(u8, u8)`, which JS keeps as an object that knows no array it is in

</details>

<details><summary>a slice of `()`, which has no bytes, from a pointer (1)</summary>

- fixtures/reject/src/lib.rs:2094:39: a slice of `()`, which has no bytes, from a pointer

</details>

<details><summary>a slice of one byte's place, which a byte slice can view only in an array of bytes (1)</summary>

- fixtures/reject/src/lib.rs:2182:2: a slice of one byte's place, which a byte slice can view only in an array of bytes

</details>

<details><summary>a slice reaching past a value in no array, which JS keeps apart from what holds it in Rust (1)</summary>

- fixtures/reject/src/lib.rs:2173:6: a slice reaching past a value in no array, which JS keeps apart from what holds it in Rust

</details>

<details><summary>a trait object of `_`, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:403:26: a trait object of `[u8; 4]`, which is not supported yet

</details>

<details><summary>arithmetic on `_` (1)</summary>

- fixtures/reject/src/lib.rs:8:2: arithmetic on `f16`

</details>

<details><summary>comparing `_` (1)</summary>

- fixtures/reject/src/lib.rs:1634:41: comparing `*mut [u8; 4]`

</details>

<details><summary>comparing `_` as a key, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:193:75: comparing `std::collections::BTreeSet<u8>` as a key, which is not supported yet

</details>

<details><summary>comparing pointers to `_` in an atomic, which JS has no address of (1)</summary>

- fixtures/reject/src/lib.rs:705:15: comparing pointers to `u32` in an atomic, which JS has no address of

</details>

<details><summary>comparing pointers to `_`, which has no bytes and so no address of its own (1)</summary>

- fixtures/reject/src/lib.rs:656:70: comparing pointers to `Bare`, which has no bytes and so no address of its own

</details>

<details><summary>const generic types are not supported: `_` (1)</summary>

- fixtures/reject/src/lib.rs:2019:1: const generic types are not supported: `Block`

</details>

<details><summary>constructing `_` (1)</summary>

- fixtures/reject/src/lib.rs:278:53: constructing `std::borrow::Cow<'_, std::sync::Arc<u8>>`

</details>

<details><summary>hashing `_` as a key, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:193:75: hashing `std::collections::BTreeSet<u8>` as a key, which is not supported yet

</details>

<details><summary>hashing `_` by its address, which JS has no counterpart of (1)</summary>

- fixtures/reject/src/lib.rs:1638:5: hashing `*const [u8; 4]` by its address, which JS has no counterpart of

</details>

<details><summary>hashing `_` through `_`'s own `Hash::hash_slice`, which is not supported yet (1)</summary>

- fixtures/reject/src/lib.rs:253:16: hashing `std::vec::Vec<Batched>` through `Batched`'s own `Hash::hash_slice`, which is not supported yet

</details>

<details><summary>union `_`, whose fields overlap in `_` and `f64`, which JS keeps apart (1)</summary>

- fixtures/reject/src/lib.rs:510:1: union `Bits`, whose fields overlap in `u32` and `f64`, which JS keeps apart

</details>
