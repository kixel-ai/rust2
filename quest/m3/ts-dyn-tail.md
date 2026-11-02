# [M] A trait object in a struct's last field

## Goal

[rust2-ts](/ts/README.md) translates a pointer to a struct whose last field unsizes to a
trait object, such as `Rc<RefCell<dyn Trait>>`, `Arc<Mutex<dyn Trait>>`, or the crate's
own `&Tail<dyn Trait>`, with Rust's dispatch, borrow checks, and drop order, proven by a
differential test against native Rust.

## Plan

Found writing the `Rc`, `Weak`, and `Pin` trait objects on 2026-11-02: these fail as "a
coercion to `Rc<RefCell<dyn Shape>>`, which is not supported yet", and the reject fixture
keeps `tail_read`. `Rc<RefCell<dyn Trait>>` is the common shared, mutable object in Rust.

One shape: the struct's other fields stay as they are, and the last one is the object a box of
its value would be, so a `RefCell`'s and a `Mutex`'s guards lend the object.

Public API: the translator's output.
