# [S] An adaptor of the crate's own iterator held as a value

## Goal

[rust2-ts](/ts/README.md) translates an adaptor taking by value an iterator of the
crate's own whose JS value is copied, as a fieldless enum's is, such as
`Toggle::Ready.map(f)`, so its `next` reads and writes the iterator's place. Proven by
differential tests against native Rust.

## Plan

Found while translating `Debug` of std's adaptors: `toggle.map(|x| x)` of a fieldless
enum that implements `Iterator` fails to type-check, as `rt.iterFrom` is given the enum's
value where its `next` takes a `rt.Place`; `(&mut toggle).map(|x| x)` translates. Find
where `into_iter` of an owned iterator of the crate's own makes its source, and give it
the place its `next` reads through.
