; A C object whose constructor calls back into a Rust function the crate defines, which the
; module's start runs once any call pulls the object into the link.

declare void @lacks_constructed()

define internal void @construct() {
	call void @lacks_constructed()
	ret void
}

@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 65535, ptr @construct, ptr null }]

define void @needs_ctor() {
	ret void
}
