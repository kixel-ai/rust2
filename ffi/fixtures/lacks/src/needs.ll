; A C library's two functions, as LLVM IR, which the toolchain's own llc compiles for
; wasm32 with no C toolchain: one calling back into a Rust function the crate defines, and
; one calling a function nothing defines.

declare void @lacks_callback()

define void @needs_callback() {
	call void @lacks_callback()
	ret void
}

declare void @missing()

define void @needs_missing() {
	call void @missing()
	ret void
}
