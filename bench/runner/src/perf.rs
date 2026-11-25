//! Counts natively, with Linux perf counters: the core PMU a count reads, and each job run on
//! its CPUs, without address randomization.

use std::path::Path;

/// The variable that has a harness target count each benchmark with perf counters of the PMU
/// of the perf type it holds: rust2-bench-harness's `PERF`.
const PERF: &str = "RUST2_BENCH_PERF";

/// What a harness target prints for each benchmark it counted so: its count, `alone` or
/// `threads`, and its name.
const COUNTED: &str = "rust2-bench counted ";

/// Where the kernel lists its PMUs.
const PMUS: &str = "/sys/bus/event_source/devices";

/// The core PMU whose counters count every job, and the CPUs the jobs run on.
pub struct Pmu {
	/// Its name, such as `cpu_atom`, or `cpu` on a host whose CPUs all share one.
	name: String,
	/// Its perf type, which an event names it by, or 0 on a host with one.
	kind: u32,
	/// The CPUs it counts on that rust2-bench may run on, which every job is pinned to.
	pub cpus: Vec<usize>,
}

impl Pmu {
	/// The core PMU of the most CPUs rust2-bench may run on, once this host has shown it lets a
	/// process count its own instructions there. On a host whose CPUs are of several kinds,
	/// such as Intel's P- and E-cores, each kind has its own PMU, which counts only on its CPUs.
	pub fn find() -> Result<Self, String> {
		let allowed = allowed()?;
		let mut cores = Vec::new();
		let entries = std::fs::read_dir(PMUS).map_err(|e| format!("cannot list {PMUS}: {e}"))?;
		for entry in entries {
			let dir = entry.map_err(|e| format!("cannot list {PMUS}: {e}"))?.path();
			// Only a core PMU of a host with several lists the CPUs it counts on; an uncore one
			// names a `cpumask` instead.
			let Ok(list) = std::fs::read_to_string(dir.join("cpus")) else {
				continue;
			};
			let name = dir.file_name().expect("listed").to_string_lossy().into_owned();
			let kind = read(&dir.join("type"))?
				.parse()
				.map_err(|_| format!("{}/type is no perf type", dir.display()))?;
			let cpus: Vec<usize> = cpus(&list)
				.map_err(|err| format!("{}/cpus: {err}", dir.display()))?
				.into_iter()
				.filter(|cpu| allowed.contains(cpu))
				.collect();
			cores.push(Pmu { name, kind, cpus });
		}
		// The most CPUs, then the first name, so a host always picks the same one.
		cores.sort_by(|a, b| b.cpus.len().cmp(&a.cpus.len()).then(a.name.cmp(&b.name)));
		let pmu = match cores.len() {
			0 => Pmu {
				name: "cpu".to_owned(),
				kind: 0,
				cpus: allowed,
			},
			// A host with one names it by its CPU's generic events.
			1 => Pmu {
				kind: 0,
				..cores.remove(0)
			},
			_ => cores.remove(0),
		};
		if pmu.cpus.is_empty() {
			return Err(format!(
				"rust2-bench may run on none of the CPUs PMU {} counts on",
				pmu.name
			));
		}
		rust2_bench_harness::__probe_perf(pmu.kind)?;
		Ok(pmu)
	}

	/// What the counts are labeled by, which a compare's two sides must share.
	pub fn label(&self) -> String {
		format!("perf {}", self.name)
	}

	/// Has a harness target's `command` count with this PMU.
	pub fn select(&self, command: &mut std::process::Command) {
		command.env(PERF, self.kind.to_string());
	}

	/// Has `command` run on this PMU's CPUs, at the addresses of its last run: without address
	/// randomization, which moves a count, as the copy loop glibc picks by alignment does.
	pub fn launch(&self, command: &mut std::process::Command) {
		#[cfg(target_os = "linux")]
		use std::os::unix::process::CommandExt;
		let mut mask = vec![0u64; self.cpus.iter().max().map_or(1, |max| max / 64 + 1)];
		for cpu in &self.cpus {
			mask[cpu / 64] |= 1 << (cpu % 64);
		}
		// SAFETY: between fork and exec it makes two system calls, which allocate nothing.
		#[cfg(target_os = "linux")]
		unsafe {
			command.pre_exec(move || {
				if sched_setaffinity(0, mask.len() * 8, mask.as_ptr()) != 0 {
					return Err(std::io::Error::last_os_error());
				}
				let persona = personality(0xffff_ffff);
				if persona == -1 || personality(persona as u64 | ADDR_NO_RANDOMIZE) == -1 {
					return Err(std::io::Error::last_os_error());
				}
				Ok(())
			});
		}
	}
}

const ADDR_NO_RANDOMIZE: u64 = 0x0040000;

#[cfg(target_os = "linux")]
unsafe extern "C" {
	fn sched_setaffinity(pid: i32, size: usize, mask: *const u64) -> i32;
	fn sched_getaffinity(pid: i32, size: usize, mask: *mut u64) -> i32;
	fn personality(persona: u64) -> i32;
}

/// The CPUs rust2-bench may run on.
#[cfg(not(target_os = "linux"))]
fn allowed() -> Result<Vec<usize>, String> {
	Err("perf counters are Linux's: count under Valgrind instead".to_owned())
}

/// The CPUs rust2-bench may run on.
#[cfg(target_os = "linux")]
fn allowed() -> Result<Vec<usize>, String> {
	let mut mask = [0u64; 64];
	// SAFETY: the mask is as large as it says.
	if unsafe { sched_getaffinity(0, size_of_val(&mask), mask.as_mut_ptr()) } != 0 {
		return Err(format!(
			"cannot read the CPUs rust2-bench may run on: {}",
			std::io::Error::last_os_error()
		));
	}
	Ok((0..mask.len() * 64)
		.filter(|cpu| mask[cpu / 64] & 1 << (cpu % 64) != 0)
		.collect())
}

fn read(path: &Path) -> Result<String, String> {
	std::fs::read_to_string(path)
		.map(|text| text.trim().to_owned())
		.map_err(|e| format!("cannot read {}: {e}", path.display()))
}

/// The CPUs a list such as `0-7,16` names.
fn cpus(list: &str) -> Result<Vec<usize>, String> {
	let mut cpus = Vec::new();
	for range in list.trim().split(',').filter(|range| !range.is_empty()) {
		let number = |n: &str| n.parse::<usize>().map_err(|_| format!("{list:?} is no CPU list"));
		match range.split_once('-') {
			Some((first, last)) => cpus.extend(number(first)?..=number(last)?),
			None => cpus.push(number(range)?),
		}
	}
	Ok(cpus)
}

/// Each benchmark a harness target's output says it counted, its count, and whether threads
/// ran beside it.
pub fn counted(stdout: &str) -> Result<Vec<(String, u64, bool)>, String> {
	stdout
		.lines()
		.filter_map(|line| line.strip_prefix(COUNTED))
		.map(|line| {
			let malformed = || format!("a harness target printed {COUNTED}{line}, which no harness prints");
			let mut fields = line.splitn(3, ' ');
			let count = fields.next().and_then(|n| n.parse().ok()).ok_or_else(malformed)?;
			let threaded = match fields.next() {
				Some("alone") => false,
				Some("threads") => true,
				_ => return Err(malformed()),
			};
			let name = fields.next().filter(|name| !name.is_empty()).ok_or_else(malformed)?;
			Ok((name.to_owned(), count, threaded))
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reads_a_cpu_list() {
		assert_eq!(cpus("0-3,8,10-11\n").unwrap(), [0, 1, 2, 3, 8, 10, 11]);
		assert_eq!(cpus("").unwrap(), Vec::<usize>::new());
		assert!(cpus("0-x").is_err());
	}

	#[test]
	fn reads_what_a_harness_counted() {
		let out = "rust2-bench counted 4096 alone a.rs::f[1 2]\nrust2-bench ran a.rs::f[1 2]\n\
		           own output\nrust2-bench counted 12 threads a.rs::g\n";
		assert_eq!(
			counted(out).unwrap(),
			[
				("a.rs::f[1 2]".to_owned(), 4096, false),
				("a.rs::g".to_owned(), 12, true)
			]
		);
		assert!(counted("rust2-bench counted 1 maybe a.rs::f\n").is_err());
		assert!(counted("rust2-bench counted x alone a.rs::f\n").is_err());
		assert!(counted("rust2-bench counted 1 alone\n").is_err());
	}
}
