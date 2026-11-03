# [S] Translating moq-net panics on a dyn Buf

## Goal

`just ts moq` and `just ts bench` translate moq-net at the pinned revision again. Today
rust2-ts panics in rustc while it reads moq-net's writes:
``<dyn bytes::Buf as std::marker::Sized>` has escaping bound vars``, from
`copy::BodyWrites::replace`, which `pointees` calls, asking whether a type under a binder
is sized. A fixture reaches the same shape, so `just ci` catches it from then on.

## Plan

Found on [moq-dev/rust2#529](https://github.com/moq-dev/rust2/pull/529), whose change
doesn't touch the analysis, so `main` likely has it too, perhaps since a trait object of `Buf`
began to translate, as in [moq-dev/rust2#519](https://github.com/moq-dev/rust2/pull/519)
or [#521](https://github.com/moq-dev/rust2/pull/521). Erase or instantiate the bound
vars before asking, rather than skipping the type.

Public API: none.
