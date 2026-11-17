; A C library's functions, as LLVM IR, which the toolchain's own llc compiles for wasm32
; with no C toolchain: one calling back into a Rust function the crate defines, one calling
; a function nothing defines, one reading a static nothing defines, and one calling through
; a static the crate defines.

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

@missing_data = external global i32

define i32 @needs_data() {
	%value = load i32, ptr @missing_data
	ret i32 %value
}

@LACKS_TABLE = external global ptr

define void @needs_table() {
	%f = load ptr, ptr @LACKS_TABLE
	call void %f()
	ret void
}
