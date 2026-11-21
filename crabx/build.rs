fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	// The build target is the platform whose releases crabx fetches.
	println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());
}
