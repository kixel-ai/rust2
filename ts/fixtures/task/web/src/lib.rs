//! `task-fixture`'s benchmark entry as the whole `pub` API, so rust2-ffi's export of it
//! holds what the bench's other wasm32 routes build and no more.

pub use task_fixture::bench::run;
