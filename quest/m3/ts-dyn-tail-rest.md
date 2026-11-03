# [M] The rest of a trait object in a struct's last field

## Goal

[rust2-ts](/ts/README.md) translates the pointers to a struct whose last field unsizes to a
trait object that it still refuses: a `Cell`, an `UnsafeCell`, or a `ManuallyDrop` as the
struct, a `dyn Future` in the last field polled through a pin, and a raw pointer unsized to
a trait object or a struct of one. Each keeps Rust's dispatch, aliasing, and drop order,
proven by a differential test against native Rust.

## Plan

[#493](https://github.com/moq-dev/rust2/pull/493) translates `Rc<RefCell<dyn Trait>>`,
`Arc<Mutex<dyn Trait>>`, `Arc<RwLock<dyn Trait>>`, and the crate's own structs as an
`rt.tailDyn` view of the struct each was made from, whose last field is the object a
`&mut` to that field would unsize to. It refuses the rest with a span, and the reject
fixture keeps `&Cell<dyn Reading>`. No consumer reaches these yet; they are std gaps.

- `Cell`, `UnsafeCell`, and `ManuallyDrop` each have an `rt` value of their own, so a view
  of one needs the field it holds its value in, as `RefCell` and `Mutex` name `value`.
- A `dyn Future` in the last field, such as `Mutex<dyn Future<Output = T> + Unpin>` polled
  by `Pin::new(&mut *guard)`, would make a new `rt.Future` at each read of the field. Check
  that polling it through the view advances the one future, and drops it once.
- A raw pointer unsized to a trait object or a struct of one is refused in `object_of`,
  for a plain `*const dyn Trait` as well. [Unsafe code](/quest/m3/ts-unsafe.md) decides
  what a raw pointer is; a raw pointer to a view is the view, whose equality `rt.tailSame`
  already compares.

Public API: the translator's output.
