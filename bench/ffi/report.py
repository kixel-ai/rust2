"""The binding benchmark's tables, in Markdown: the median of each case's rounds per language,
in CPU and wall time, each beside the median 1-minute load average over its rounds, read
from `just ffi bench-run`'s raw.tsv; the median time and peak RSS of the export and its
crate's build, beside moq's own, from its build.tsv; and the sizes of the export's generated bindings and
native libraries, and of the wasm32 stand-in's TypeScript, module, and npm package, and of
one caller's bundle. `just ffi bench-android`'s raw.tsv alone prints only the time tables.

Usage: report.py <raw.tsv> [<build.tsv> <export dir> <web export dir> <caller.ts>]
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
	("kotlin-art", "Kotlin (ART)"),
	("python", "Python"),
	("swift", "Swift"),
	("go", "Go"),
	("dart", "Dart"),
	("ts-node", "TS (Node)"),
	("ts-chromium", "TS (Chromium)"),
	("ts-firefox", "TS (Firefox)"),
]

# The TypeScript programs, which run Rust on one JavaScript thread.
TS = ["ts-node", "ts-chromium", "ts-firefox"]

CALLS = [
	("call", "`track.is_closed()`, a handle call"),
	("getter", "`demand.start()`, a u64 getter"),
	("shared:call", "the handle call on 4 threads, one shared handle"),
	("own:call", "the same, a handle per thread"),
	("shared:getter", "the u64 getter on 4 threads, one shared handle"),
	("own:getter", "the same, a handle per thread"),
	("record", "`track.largest()`, a record return"),
	("object", "`track.info()`, an object return"),
	("objects", "the same from 4 threads at once, Kotlin only"),
	("field", "`info.priority`, a primitive `pub` field: a leaf"),
	("hop:call", "Dart only: the handle call while Rust holds a closure, on a worker"),
	("hop:field", "Dart only: the field then, a leaf call on the isolate's thread"),
	("write", "`group.write_frame(at, frame)`, 100 B"),
	("stamped", "the same, with a new `Timestamp` each"),
	("ready", "a 100 B frame written and read, ready"),
	("woken", "the same, read after a wake"),
	("loop", "one managed reader and writer, including their handoff; Go writes only after the read pends"),
	("drained", "Swift only: one reader on a drained executor, no writer handoff"),
	("payload", "`frame.payload`, 100 B"),
]

# What a language does in place of a case it cannot run, printed in that case's cell.
THREADED = ["shared:call", "own:call", "shared:getter", "own:getter"]
INSTEAD = {("dart", case): "no: an isolate takes no other's object" for case in THREADED} | {
	(lang, case): "no: one thread" for lang in TS for case in THREADED
}

# Each step `just ffi bench-run` times from an emptied target, and what it is.
BUILDS = [
	("export", "rust2-ffi's export, a `cargo check` of moq under its driver"),
	("check", "moq's own `cargo check` of the same crates"),
	("generated", "the generated crate's release build, Kotlin and Dart glue included"),
	("moq", "moq's own release build of the same crates"),
]

SIZES = [100, 1024, 16384, 65536, 1048576]

LABELS = {16: "16 B", 64: "64 B", 100: "100 B", 256: "256 B", 1024: "1 KiB", 4096: "4 KiB", 16384: "16 KiB"}
LABELS |= {65536: "64 KiB", 262144: "256 KiB", 1048576: "1 MiB"}

# Each binding's generated files, and its Rust glue in the export's crate, under the export.
BINDINGS = [
	("C", ["include/moq.h"], ["src/lib.rs", "src/rt.rs"]),
	("C++", ["include/moq.hpp", "include/moq"], []),
	("Kotlin", ["kotlin/moq.kt", "kotlin/moq.pro"], ["src/jni"]),
	("Python", ["python/moq.py"], ["src/python"]),
	("Swift", ["swift"], []),
	("Go", ["go"], []),
	("Dart", ["dart/lib/moq.dart"], ["src/dart"]),
]

# The TypeScript, under the wasm32 stand-in's export, and the glue its module builds with.
WEB_BINDING = ("TypeScript", ["web"], ["src/lib.rs", "src/rt.rs", "src/web"])

# Brotli at its densest, as a host precompresses what it serves; Python's standard library
# lacks it, so Node's.
BROTLI = (
	"const zlib = require('node:zlib');"
	"const data = require('node:fs').readFileSync(0);"
	"process.stdout.write(String(zlib.brotliCompressSync(data, {params: {[zlib.constants.BROTLI_PARAM_QUALITY]: 11}}).length));"
)
WASM = ("`moq.wasm`, the wasm32 stand-in's module, with the web glue", "target/wasm32-wasip1/release/moq.wasm")

# The shared library's suffix, as `just ffi moq-export` names it on each platform it builds on.
SHARED = "dylib" if sys.platform == "darwin" else "so"

LIBRARIES = [
	("`libmoq.a`, the C ABI that C, C++, Swift, and Go link", "target/release/libmoq.a"),
	(f"`libmoq.{SHARED}`, with the Kotlin and Dart glue", f"target/release/libmoq.{SHARED}"),
	(f"`python/libmoq.{SHARED}`, the CPython extension", f"python/libmoq.{SHARED}"),
]

# What `strip --strip-all` drops, or on macOS, whose strip lacks it, every local symbol.
STRIP = ["strip", "-x"] if sys.platform == "darwin" else ["strip", "--strip-all"]


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


def typescript(path):
	"""A TypeScript module the web binding writes, not its JavaScript or declarations."""
	return path.endswith(".ts") and not path.endswith(".d.ts")


def compressed(data):
	"""The raw, gzipped, and brotli'd bytes of `data`."""
	brotli = subprocess.run(["node", "-e", BROTLI], input=data, capture_output=True, check=True).stdout
	return len(data), len(gzip.compress(data, compresslevel=6, mtime=0)), int(brotli)


def sizes(paths):
	"""The raw, gzipped, and brotli'd bytes of `paths`, each compressed alone as a host serves it."""
	total = [0, 0, 0]
	for path in paths:
		with open(path, "rb") as f:
			for i, n in enumerate(compressed(f.read())):
				total[i] += n
	return total


def bundle(caller):
	"""`caller` and what it imports of the binding, bundled for the browser, minified and
	tree-shaken, as an app's build ships it."""
	with tempfile.TemporaryDirectory() as tmp:
		out = os.path.join(tmp, "bundle.js")
		subprocess.run(["esbuild", caller, "--bundle", "--minify", "--format=esm", "--platform=browser",
			"--target=es2024", "--log-level=warning", f"--outfile={out}"], check=True)
		with open(out, "rb") as f:
			return f.read()


def wasm_stripped(data):
	"""A wasm module without its custom sections, its names and debug info among them."""
	out = bytearray(data[:8])
	at = 8
	while at < len(data):
		section = data[at]
		size = shift = 0
		end = at + 1
		while True:
			byte = data[end]
			size |= (byte & 0x7F) << shift
			shift += 7
			end += 1
			if byte < 0x80:
				break
		if section != 0:
			out += data[at : end + size]
		at = end + size
	return bytes(out)


def kib(n):
	return f"{n / 1024:,.0f} KiB"


def number(value):
	return f"{value:,.1f}" if value < 10 else f"{value:,.0f}"


def table(header, rows):
	lines = ["| " + " | ".join(header) + " |", "|" + "---|" * len(header)]
	lines += ["| " + " | ".join(row) + " |" for row in rows]
	return "\n".join(lines)


def builds(path):
	"""The Build table: each step's median wall and CPU seconds and peak RSS, and load."""
	rounds = collections.defaultdict(list)
	with open(path) as f:
		for line in f:
			step, _, wall, cpu, rss, load = line.rstrip("\n").split("\t")
			rounds[step].append((float(wall), float(cpu), int(rss), float(load)))
	rows = []
	for step, label in BUILDS:
		if step not in rounds:
			continue
		wall, cpu, rss, load = (statistics.median(s[i] for s in rounds[step]) for i in range(4))
		rows.append([label, f"{wall:,.1f}", f"{cpu:,.1f}", f"{rss / 1048576:,.0f} MiB", f"{load:.0f}"])
	return table(["step", "wall s", "CPU s", "peak RSS", "load average"], rows)


def main(raw_path, build_path=None, export=None, web=None, caller=None):
	rounds = collections.defaultdict(list)
	with open(raw_path) as f:
		for line in f:
			lang, _, case, wall, cpu, load = line.rstrip("\n").split("\t")
			rounds[(lang, case)].append((float(wall), float(cpu), float(load)))
	median = {
		key: tuple(statistics.median(values[i] for values in samples) for i in range(3))
		for key, samples in rounds.items()
	}
	langs = [(key, name) for key, name in LANGS if any(lang == key for lang, _ in median)]

	def cell(lang, case, i):
		if (lang, case) in INSTEAD:
			return INSTEAD[(lang, case)]
		if (lang, case) not in median:
			return ""
		values = median[(lang, case)]
		# A browser has no clock of its CPU time.
		if values[i] != values[i]:
			return ""
		return f"{number(values[i])} ({values[2]:.0f})"

	thru = [(key, name) for key, name in langs if (key, f"mbps:{SIZES[0]}") in median]
	payloads = sorted({int(case.split(":")[1]) for _, case in median if case.startswith("payload:")})
	sized = [(key, name) for key, name in langs if any((key, f"payload:{n}") in median for n in payloads)]
	for i, clock, rate in [(1, "CPU", "MB per CPU second"), (0, "wall", "MB/s")]:
		print(f"### Calls (ns per call, {clock} time; load average)\n")
		header = ["call"] + [name for _, name in langs]
		rows = [[label] + [cell(lang, case, i) for lang, _ in langs] for case, label in CALLS]
		# A case no language here runs, as in bench-android's Kotlin alone.
		print(table(header, [row for row in rows if any(row[1:])]))
		print(f"\n### Throughput ({rate}, write plus ready read; load average)\n")
		header = ["frame"] + [name for _, name in thru]
		rows = [[LABELS[n]] + [cell(lang, f"mbps:{n}", i) for lang, _ in thru] for n in SIZES]
		print(table(header, rows))
		print()
		if sized:
			print(f"### `frame.payload` by size (ns per call, {clock} time; load average)\n")
			header = ["payload"] + [name for _, name in sized]
			rows = [[LABELS[n]] + [cell(lang, f"payload:{n}", i) for lang, _ in sized] for n in payloads]
			print(table(header, rows))
			print()

	if build_path is None:
		return

	print("### Build (median, from an emptied target)\n")
	print(builds(build_path))
	print()

	print("### Size\n")
	rows = []
	for root, (name, generated, glue) in [(export, b) for b in BINDINGS] + [(web, WEB_BINDING)]:
		ours = [p for p in files(root, generated) if root != web or typescript(p)]
		raw, zipped = size(ours)
		glue_raw, glue_zipped = size(files(root, glue))
		count = len(ours)
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
			subprocess.run([*STRIP, copy], check=True)
			rows.append([label, kib(os.path.getsize(full)), kib(os.path.getsize(copy))])
	label, path = WASM
	with open(os.path.join(web, path), "rb") as f:
		data = f.read()
	rows.append([label, kib(len(data)), kib(len(wasm_stripped(data)))])
	print(table(["native library", "as built", "stripped"], rows))

	print()
	everything = files(web, ["web"])
	modules = [p for p in everything if typescript(p)]
	package = [p for p in everything if p.endswith(".js") or p.endswith(".d.ts")]
	rows = [
		["`moq.wasm`, stripped", *map(kib, compressed(wasm_stripped(data)))],
		[f"the TypeScript, {len(modules)} modules", *map(kib, sizes(modules))],
		["the npm package's JavaScript and declarations", *map(kib, sizes(package))],
		[f"`{os.path.basename(caller)}` bundled by esbuild, minified", *map(kib, compressed(bundle(caller)))],
	]
	print(table(["web binding", "raw", "gzip", "brotli"], rows))


if __name__ == "__main__":
	if len(sys.argv) not in (2, 6):
		sys.exit(__doc__)
	main(*sys.argv[1:])
