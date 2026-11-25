//! The environment a count is comparable within.
//!
//! Valgrind presents a CPU derived from the host's, and code that picks a path by CPU
//! feature, such as glibc's string functions or Rust's runtime detection, counts
//! differently on another. So does another Valgrind, and glibc's tunables, which can mask
//! features under the same CPU, and the preload every count runs with, whose code runs inside
//! what is counted. Each count names the fingerprint of all of them, so counts from two
//! environments never share a series.

use std::process::Command;

/// The inputs of a fingerprint, and its label.
pub struct Fingerprint {
	pub arch: &'static str,
	/// `valgrind --version`.
	pub valgrind: String,
	/// `GLIBC_TUNABLES` as the benchmarks inherit it, when set.
	pub tunables: Option<String>,
	/// What the CPU identifies as under Valgrind, one line a leaf.
	pub cpu: String,
	/// The hash of the library every count preloads, its seed and clock compiled in.
	pub preload: String,
}

impl Fingerprint {
	/// Probes this environment: runs this executable's [`cpu`] under the same Valgrind tool
	/// and options the benchmarks run under.
	pub fn probe() -> Result<Self, String> {
		let arch = std::env::consts::ARCH;
		if arch != "x86_64" {
			return Err(format!(
				"{arch} is not supported: only x86_64's CPU, as Valgrind presents it, is fingerprinted"
			));
		}
		let version = output(
			Command::new(crate::valgrind_path()?).arg("--version"),
			"valgrind --version",
		)?;
		let exe = std::env::current_exe().map_err(|e| format!("cannot locate rust2-bench itself: {e}"))?;
		let mut probe = crate::valgrind()?;
		probe.arg("--callgrind-out-file=/dev/null").arg(exe).arg(crate::PROBE);
		let cpu = output(&mut probe, "the CPU probe under Valgrind")?;
		Ok(Self {
			arch,
			valgrind: version.trim().to_owned(),
			tunables: std::env::var("GLIBC_TUNABLES").ok(),
			cpu,
			preload: crate::preload::hash(),
		})
	}

	/// The fingerprint as a measure names it: the architecture and a hash of every input.
	pub fn label(&self) -> String {
		let mut hash = Fnv::default();
		hash.write(self.arch);
		hash.write(&self.valgrind);
		match &self.tunables {
			Some(tunables) => hash.write(&format!("GLIBC_TUNABLES={tunables}")),
			None => hash.write("GLIBC_TUNABLES unset"),
		}
		hash.write(&self.cpu);
		hash.write(&self.preload);
		format!("{}-{:012x}", self.arch, hash.0 >> 16)
	}
}

/// Runs `command`, returning its stdout, or failing with its stderr.
fn output(command: &mut Command, what: &str) -> Result<String, String> {
	let out = command.output().map_err(|e| format!("cannot run {what}: {e}"))?;
	if !out.status.success() {
		return Err(format!(
			"{what} failed ({}): {}",
			out.status,
			String::from_utf8_lossy(&out.stderr)
		));
	}
	String::from_utf8(out.stdout).map_err(|_| format!("{what} printed something not UTF-8"))
}

/// The CPU's identification, every CPUID leaf and its first 16 subleaves, one line each,
/// with the fields that name the core it ran on masked, since those change from one run to
/// the next on a host that passes them through.
#[cfg(target_arch = "x86_64")]
pub fn cpu() -> String {
	use std::arch::x86_64::__cpuid_count;
	use std::fmt::Write;

	let mut out = String::new();
	for base in [0, 0x8000_0000u32] {
		let max = __cpuid_count(base, 0).eax;
		// A leaf range past its base is not one: on a CPU without extended leaves, the
		// extended base answers as the highest basic leaf.
		let max = if max < base { base } else { max.min(base + 0xff) };
		for leaf in base..=max {
			for sub in 0..16 {
				let mut r = __cpuid_count(leaf, sub);
				match leaf {
					// The initial APIC ID.
					1 => r.ebx &= 0x00ff_ffff,
					// The x2APIC ID.
					0xb | 0x1f => r.edx = 0,
					// A hybrid CPU's core type.
					0x1a => r.eax = 0,
					// AMD's extended APIC ID and its core and node IDs.
					0x8000_001e => (r.eax, r.ebx, r.ecx) = (0, r.ebx & !0xff, r.ecx & !0xff),
					_ => {}
				}
				writeln!(
					out,
					"{leaf:08x}.{sub:x} {:08x} {:08x} {:08x} {:08x}",
					r.eax, r.ebx, r.ecx, r.edx
				)
				.expect("a String takes every write");
			}
		}
	}
	out
}

#[cfg(not(target_arch = "x86_64"))]
pub fn cpu() -> String {
	unreachable!("probe refuses every other architecture first")
}

/// 64-bit FNV-1a, a stable hash with no dependency; a label needs no resistance to collisions
/// someone engineers.
pub struct Fnv(pub u64);

impl Default for Fnv {
	fn default() -> Self {
		Self(0xcbf2_9ce4_8422_2325)
	}
}

impl Fnv {
	/// Hashes `field`, then a separator no field holds, so no two lists of fields collide by
	/// where one ends.
	pub fn write(&mut self, field: &str) {
		self.bytes(field.as_bytes());
	}

	/// Hashes `field` as [`write`](Self::write) does, for bytes that are not text.
	pub fn bytes(&mut self, field: &[u8]) {
		for &byte in field.iter().chain(&[0]) {
			self.0 = (self.0 ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn fingerprint() -> Fingerprint {
		Fingerprint {
			arch: "x86_64",
			valgrind: "valgrind-3.27.1".to_owned(),
			tunables: None,
			cpu: "00000000.0 0000000d 756e6547 6c65746e 49656e69\n".to_owned(),
			preload: "0123456789ab".to_owned(),
		}
	}

	#[test]
	fn the_label_names_the_architecture_and_is_stable() {
		let label = fingerprint().label();
		assert_eq!(label, fingerprint().label());
		assert!(
			label.starts_with("x86_64-") && label.len() == "x86_64-".len() + 12,
			"{label}"
		);
	}

	#[test]
	fn every_input_moves_the_label() {
		let base = fingerprint().label();
		let mut tuned = fingerprint();
		tuned.tunables = Some(String::new());
		let mut other = fingerprint();
		other.valgrind = "valgrind-3.28.0".to_owned();
		let mut cpu = fingerprint();
		cpu.cpu.push('\n');
		let mut preload = fingerprint();
		preload.preload = "ba9876543210".to_owned();
		for changed in [tuned, other, cpu, preload] {
			assert_ne!(changed.label(), base);
		}
	}
}
