# [S] React Native's JSI module trusts no kit but its binding's

## Goal

No JavaScript an app runs can make React Native's JSI module read memory it does not own
through the kit `install` takes: a second install into a runtime is an error, and each value
the module takes back from a kit function it calls is checked before it is read as the type
it expects, so a forged kit is a `TypeError`, never undefined behavior.

## Plan

- Found 2026-11-23 auditing review round 2 of React Native's binding: `install` is reachable
  from any JavaScript, through `TurboModuleRegistry` or the host's global, and takes `kit`
  whole. `ffi/runtime/rt.jsi.hpp` then reads kit results unchecked: `halves`' and
  `u128`'s `getObject(rt).getArray(rt)`, `pend`'s `made` array and its `asFunction`, and
  `returned`'s `getObject`. JSI checks none of these but in a debug build, as it does
  `getNativeState`.
- Refuse an install into a runtime that holds the module's `teardown` already, as
  `Runtime::getRuntimeData` shows; `rt.rn.ts`'s `load` installs once per runtime.
- Check each kit result's type where it is read, throwing as `obj` and `arr` do, and give
  `tests/react-native.ts` a case that installs a forged kit into a fresh global's installer
  and calls a function through it.
