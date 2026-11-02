# Migrating

## Unreleased

- `uniffi_macros`: `generate_and_include_scaffolding!` (the `trybuild` feature) resolves a
  relative path from the calling file, as `include_str!` does, not from rustc's working directory.
  A trybuild case at `tests/ui/x.rs` names `src/x.udl` as `../../src/x.udl`.
- `uniffi_bindgen`: a name no longer identifies a type across crates.
  `ComponentInterface::is_name_used_as_error(name)` is now `is_used_as_error(&ty)`,
  `ComponentInterface::get_type(name)` finds only the crate's own types, and
  `VisitMut::visit_error_name` takes the module path of the crate that defines the error.
- `uniffi-bindgen-cpp`: the `--scaffolding` flag is gone. It only stubbed out generation.
- Generated C++: async needs `error_style = "expected"`, and the default exceptions style refuses
  an async function, constructor, method, or callback interface method. Its `uniffi::Future<T>`,
  `FutureResult`, `ForeignFuture`, `AsyncCancelledError`, and `AsyncDispatcherError`, and the
  `<Trait>Foreign` interface of a trait with async methods, are gone. Switch to the expected
  style, where an async call returns `uniffi::Future<T, E>`.
- Generated C: every type and function name starts with the crate's namespace, so a crate
  whose items did not already start with it gets renamed C symbols.
- Generated C: type names are CamelCase, as in GLib: `app_my_object *` is now `AppMyObject *`,
  `app_task` is `AppTask`, enum tags are `<Enum>Tag`, variant structs `<Enum><Variant>`, and
  async callback types `<Function>Callback` (`app_my_object_do_thing_callback` is now
  `AppMyObjectDoThingCallback`). Functions and enum constants keep their names.
- Generated C: every async callback takes `bool cancelled` after `user_data`, and freeing a task
  whose callback has not started runs that callback once with `cancelled` set, on the dispatcher,
  instead of never. Release `user_data` in the callback rather than after `<namespace>_task_free`,
  which no longer waits for a callback running on another thread.
