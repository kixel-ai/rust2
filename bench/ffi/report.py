"""The binding benchmark's tables, in Markdown: the median of each case's rounds per language,
read from `just ffi bench-run`'s raw.tsv, and the sizes of the export's generated bindings
and native libraries.

Usage: report.py <raw.tsv> <export dir>
"""

import collections
import gzip
import os
import shutil
import statistics
import subprocess
import sys
import tempfile

LANGS = [
	("rust", "Rust"),
	("c", "C"),
	("cpp", "C++"),
	("moq-c", "moq-c"),
	("kotlin", "Kotlin"),
	("python", "Python"),
	("swift", "Swift"),
	("go", "Go"),
	("dart", "Dart"),
]

CALLS = [
	("call", "`track.is_closed()`, a handle call"),
	("getter", "`demand.sequence()`, a u64 getter"),
	("record", "`track.largest()`, a record return"),
	("object", "`track.info()`, an object return"),
	("write", "`group.write_frame(at, frame)`, 100 B"),
	("stamped", "the same, with a new `Timestamp` each"),
	("ready", "a 100 B frame written and read, ready"),
	("woken", "the same, read after a wake"),
	("payload", "`frame.payload`, 100 B"),
]

SIZES = [100, 1024, 16384, 65536, 1048576]

# Each binding's generated files, and its Rust glue in the export's crate, under the export.
BINDINGS = [
	("C", ["include/moq.h"], ["src/lib.rs", "src/rt.rs"]),
	("C++", ["include/moq.hpp"], []),
	("Kotlin", ["kotlin/moq.kt", "kotlin/moq.pro"], ["src/jni"]),
	("Python", ["python/moq.py"], ["src/python"]),
	("Swift", ["swift"], []),
	("Go", ["go"], []),
	("Dart", ["dart/moq.dart"], ["src/dart"]),
]

LIBRARIES = [
	("`libmoq.a`, the C ABI that C, C++, Swift, and Go link", "target/release/libmoq.a"),
	("`libmoq.so`, with the Kotlin and Dart glue", "target/release/libmoq.so"),
	("`python/libmoq.so`, the CPython extension", "python/libmoq.so"),
]


def files(root, paths):
	"""Every file under each of `paths`, from `root`, sorted."""
	out = []
	for path in paths:
		full = os.path.join(root, path)
		if os.path.isdir(full):
			for base, _, names in os.walk(full):
				out += [os.path.join(base, name) for name in names if not name.endswith(".pyc")]
		else:
			out.append(full)
	return sorted(out)


def size(paths):
	"""The raw and gzipped bytes of `paths`, each gzipped alone as a package ships it."""
	raw = zipped = 0
	for path in paths:
		with open(path, "rb") as f:
			data = f.read()
		raw += len(data)
		zipped += len(gzip.compress(data, compresslevel=6, mtime=0))
	return raw, zipped


def kib(n):
	return f"{n / 1024:,.0f} KiB"


def ns(value):
	return f"{value:.1f}" if value < 10 else f"{value:,.0f}"


def mbps(value):
	return f"{value:,.1f}" if value < 10 else f"{value:,.0f}"


def table(header, rows):
	lines = ["| " + " | ".join(header) + " |", "|" + "---|" * len(header)]
	lines += ["| " + " | ".join(row) + " |" for row in rows]
	return "\n".join(lines)


def main(raw_path, export):
	rounds = collections.defaultdict(list)
	with open(raw_path) as f:
		for line in f:
			lang, _, case, value = line.rstrip("\n").split("\t")
			rounds[(lang, case)].append(float(value))
	median = {key: statistics.median(values) for key, values in rounds.items()}
	langs = [(key, name) for key, name in LANGS if any(lang == key for lang, _ in median)]

	def cell(lang, case, fmt):
		return fmt(median[(lang, case)]) if (lang, case) in median else ""

	print("### Calls (ns per call)\n")
	header = ["call"] + [name for _, name in langs]
	print(table(header, [[label] + [cell(lang, case, ns) for lang, _ in langs] for case, label in CALLS]))

	print("\n### Throughput (MB/s, write plus ready read)\n")
	thru = [(key, name) for key, name in langs if (key, f"mbps:{SIZES[0]}") in median]
	header = ["frame"] + [name for _, name in thru]
	labels = {100: "100 B", 1024: "1 KiB", 16384: "16 KiB", 65536: "64 KiB", 1048576: "1 MiB"}
	print(table(header, [[labels[n]] + [cell(lang, f"mbps:{n}", mbps) for lang, _ in thru] for n in SIZES]))

	print("\n### Size\n")
	rows = []
	for name, generated, glue in BINDINGS:
		raw, zipped = size(files(export, generated))
		glue_raw, glue_zipped = size(files(export, glue))
		count = len(files(export, generated))
		glue_cell = f"{kib(glue_raw)} / {kib(glue_zipped)}" if glue else "none"
		rows.append([name, str(count), kib(raw), kib(zipped), glue_cell])
	print(table(["binding", "files", "raw", "gzip", "Rust glue, raw / gzip"], rows))

	print()
	rows = []
	for label, path in LIBRARIES:
		full = os.path.join(export, path)
		with tempfile.TemporaryDirectory() as tmp:
			copy = os.path.join(tmp, os.path.basename(path))
			shutil.copyfile(full, copy)
			subprocess.run(["strip", "--strip-all", copy], check=True)
			rows.append([label, kib(os.path.getsize(full)), kib(os.path.getsize(copy))])
	print(table(["native library", "as built", "stripped"], rows))


if __name__ == "__main__":
	if len(sys.argv) != 3:
		sys.exit(__doc__)
	main(sys.argv[1], sys.argv[2])
