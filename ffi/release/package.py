"""Build and check the Python wheels, Swift package, and Go module of a crate's rust2-ffi export.

bindings.yml runs each step on the host it needs; release.yml is the caller `init` writes.
Every fact about the package comes from Cargo metadata or the export, so nothing here
names a crate: what Cargo cannot say, such as who may publish, stays in the caller.
"""

import argparse
import base64
import csv
import difflib
import hashlib
import io
import json
import os
from pathlib import Path
import plistlib
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.parse
import uuid
import zipfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
# The wheel's Linux floor: glibc 2.28 (RHEL 8, Debian 10), crabx's, and moq's manylinux_2_28.
GLIBC = "2.28"
# The macOS a wheel loads on: the oldest Rust's aarch64-apple-darwin target supports.
MACOS_WHEEL = "11.0"
# The Swift package's floors: what the generated Swift's `Swift.Duration` needs.
SWIFT_PLATFORMS = {"macos": "13.0", "ios": "16.0"}
# Each Apple slice the Swift package ships, and the platform its minimum OS is checked on.
APPLE_TARGETS = {
    "aarch64-apple-darwin": "macos",
    "aarch64-apple-ios": "ios",
    "aarch64-apple-ios-sim": "ios",
}
# Every target a wheel is built for.
WHEEL_TARGETS = [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
]
# Every target the Go module links a static library of, by its GOOS_GOARCH. Go links
# Windows through MinGW, so a -gnu library.
GO_TARGETS = {
    "x86_64-unknown-linux-gnu": "linux_amd64",
    "aarch64-unknown-linux-gnu": "linux_arm64",
    "aarch64-apple-darwin": "darwin_arm64",
    "aarch64-apple-ios": "ios_arm64",
    "x86_64-pc-windows-gnu": "windows_amd64",
}
# What the go command refuses to extract: a module zip, or its files, over 500 MiB, and a
# go.mod over 16 MiB (golang.org/x/mod/zip's MaxZipFile and MaxGoMod).
GO_ZIP_MAX = 500 << 20
GO_MOD_MAX = 16 << 20
# The sections the go command refuses in an archive a #cgo directive names: LLVM's bitcode,
# which Rust's prebuilt compiler_builtins carries.
LTO_SECTIONS = [".llvmbc", ".llvmcmd", "__LLVM,__bitcode", "__LLVM,__cmdline"]
# Cargo metadata names a library target by its crate types; a proc-macro is no library to export.
LIBRARY_KINDS = {"lib", "rlib", "dylib", "cdylib", "staticlib"}
CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"
COLLECTIONS = "https://github.com/apple/swift-collections"
# The first date a ZIP entry can hold, and so the earliest SOURCE_DATE_EPOCH a build takes.
ZIP_EPOCH = 315532800
SEMVER = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-(alpha|beta|rc)\.(0|[1-9][0-9]*))?")


def run(*args, **kwargs):
    """Run a tool, failing the build with its captured output when it fails."""
    try:
        return subprocess.run([str(a) for a in args], check=True, **kwargs)
    except subprocess.CalledProcessError as error:
        for stream in (error.stdout, error.stderr):
            if isinstance(stream, bytes):
                sys.stderr.flush()
                sys.stderr.buffer.write(stream)
            elif stream:
                sys.stderr.write(stream)
        sys.stderr.flush()
        raise SystemExit(f"{error.cmd[0]} failed with status {error.returncode}") from None


def output(*args, **kwargs):
    """A tool's standard output, failing the build when it fails."""
    return run(*args, capture_output=True, encoding="utf-8", **kwargs).stdout


def library(package):
    """The package's library target name, as Rust spells it, whatever crate types it declares."""
    libs = [t for t in package["targets"] if LIBRARY_KINDS & set(t["kind"])]
    if len(libs) != 1:
        raise SystemExit(f"{package['name']} has no library to export")
    return libs[0]["name"].replace("-", "_")


def epoch(directory):
    """The build time every file of this release carries: SOURCE_DATE_EPOCH, else the commit's."""
    raw = os.environ.get("SOURCE_DATE_EPOCH") or output("git", "-C", directory, "log", "-1", "--format=%ct").strip()
    if not raw.isdigit() or int(raw) < ZIP_EPOCH:
        raise SystemExit(f"SOURCE_DATE_EPOCH {raw!r} is no time since 1980: set it, or build from a commit")
    return int(raw)


def facts(manifest):
    """The package at `manifest`, as Cargo metadata describes it."""
    meta = json.loads(output("cargo", "metadata", "--format-version", "1", "--no-deps",
                             "--manifest-path", manifest))
    path = Path(manifest).resolve()
    package = next((p for p in meta["packages"] if Path(p["manifest_path"]).resolve() == path), None)
    if package is None:
        raise SystemExit(f"{manifest} is a virtual manifest: name the package to export")
    return {
        "name": package["name"],
        "version": package["version"],
        "description": package.get("description"),
        "license": package.get("license"),
        "license_file": package.get("license_file"),
        "repository": package.get("repository"),
        "homepage": package.get("homepage"),
        "lib": library(package),
        "dir": str(path.parent),
        "workspace": meta["workspace_root"],
        "epoch": epoch(path.parent),
        "examples": sorted(t["name"] for t in package["targets"] if "example" in t["kind"]),
    }


def pep440(version):
    """Cargo's version as PyPI spells it, refusing what has no exact spelling there."""
    match = SEMVER.fullmatch(version)
    if match is None:
        raise SystemExit(f"version {version} has no PEP 440 equivalent: use X.Y.Z or X.Y.Z-(alpha|beta|rc).N")
    major, minor, patch, pre, number = match.groups()
    suffix = {"alpha": "a", "beta": "b", "rc": "rc", None: ""}[pre]
    return f"{major}.{minor}.{patch}{suffix}{number or ''}"


def pascal(name):
    """The exporter's Swift module name for a prefix: `moq_ffi` is `MoqFfi`."""
    return "".join(part[:1].upper() + part[1:] for part in name.split("_") if part)


def python_floor(export):
    """The oldest CPython the export's stable-ABI extension loads in, from its glue's pyo3 feature."""
    found = set(re.findall(r'"abi3-py3([0-9]+)"', (Path(export) / "Cargo.toml").read_text(encoding="utf-8")))
    if len(found) != 1:
        raise SystemExit(f"{export}/Cargo.toml names no single abi3 floor: {sorted(found)}")
    return int(found.pop())


def licenses(info):
    """The license files the package names, or else those beside its manifest or workspace."""
    if info["license_file"]:
        return [Path(info["dir"]) / info["license_file"]]
    for root in (Path(info["dir"]), Path(info["workspace"])):
        found = sorted(p for p in root.iterdir() if p.is_file() and re.match(r"(LICEN[CS]E|COPYING)", p.name))
        if found:
            return found
    return []


def metadata(info, floor):
    """The wheel's core metadata, version 2.4."""
    lines = ["Metadata-Version: 2.4", f"Name: {info['name']}", f"Version: {pep440(info['version'])}"]
    if info["description"]:
        lines.append(f"Summary: {' '.join(info['description'].split())}")
    if info["license"]:
        # Cargo still takes the legacy MIT/Apache-2.0, which PyPI refuses only at upload.
        if "/" in info["license"]:
            raise SystemExit(f"license {info['license']!r} is no SPDX expression: write it with OR")
        lines.append(f"License-Expression: {info['license']}")
    lines += [f"License-File: {path.name}" for path in licenses(info)]
    for label, url in (("Homepage", info["homepage"]), ("Repository", info["repository"])):
        if url:
            lines.append(f"Project-URL: {label}, {url}")
    lines.append(f"Requires-Python: >=3.{floor}")
    return "\n".join(lines) + "\n\n"


def write_zip(path, entries, stamp):
    """A ZIP of `(name, data, permissions)` entries in the order given, the same bytes whoever writes it and whenever.

    Every entry takes `stamp`, a name ending in `/` is a directory, and nothing else of the
    writer, such as its host or umask, is recorded.
    """
    date = time.gmtime(stamp)[:6]
    with zipfile.ZipFile(path, "w") as archive:
        for name, data, mode in entries:
            entry = zipfile.ZipInfo(name, date)
            entry.create_system = 3
            if name.endswith("/"):
                entry.external_attr = (0o040000 | mode) << 16 | 0x10
            else:
                entry.external_attr = (0o100000 | mode) << 16
                entry.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(entry, data, compresslevel=9)


def seal(path, files, stamp):
    """Write `files`, `{name: (data, permissions)}`, as a wheel: sorted with the dist-info last, and its RECORD hashing the rest."""
    meta = {name.split("/")[0] for name in files if name.split("/")[0].endswith(".dist-info")}
    if len(meta) != 1:
        raise SystemExit(f"{path} needs one .dist-info directory, has {sorted(meta)}")
    record = f"{meta.pop()}/RECORD"
    entries = sorted(files, key=lambda name: (name.split("/")[0].endswith(".dist-info"), name))
    rows = io.StringIO(newline="")
    writer = csv.writer(rows, lineterminator="\n")
    for name in entries:
        digest = base64.urlsafe_b64encode(hashlib.sha256(files[name][0]).digest()).rstrip(b"=").decode()
        writer.writerow([name, f"sha256={digest}", len(files[name][0])])
    writer.writerow([record, "", ""])
    write_zip(path, [(name, *files[name]) for name in entries] + [(record, rows.getvalue().encode(), 0o644)], stamp)


def normalize_wheel(path, stamp):
    """Rewrite a repaired wheel as `seal` writes one, whatever order, times, and modes its tool left."""
    with zipfile.ZipFile(path) as archive:
        files = {
            entry.filename: (archive.read(entry), 0o755 if entry.external_attr >> 16 & 0o111 else 0o644)
            for entry in archive.infolist()
            # Repair tools add directory entries, which RECORD never lists, and rewrite RECORD itself.
            if not entry.is_dir() and not entry.filename.endswith(".dist-info/RECORD")
        }
    seal(path, files, stamp)


def wheel_platform(target):
    """The tag a wheel for `target` starts with, before repair narrows it."""
    tags = {
        "x86_64-unknown-linux-gnu": "linux_x86_64",
        "aarch64-unknown-linux-gnu": "linux_aarch64",
        "aarch64-apple-darwin": f"macosx_{MACOS_WHEEL.replace('.', '_')}_arm64",
        "x86_64-pc-windows-msvc": "win_amd64",
    }
    if target not in tags:
        raise SystemExit(f"no wheel is built for {target}: the targets are {', '.join(tags)}")
    return tags[target]


def library_name(lib, target):
    """The file the generated module loads from beside itself."""
    if "windows" in target:
        return f"{lib}.dll"
    return f"lib{lib}.dylib" if "apple" in target else f"lib{lib}.so"


def write_wheel(info, export, library, target, out):
    """An unrepaired wheel holding the generated module as a package beside its library."""
    lib = info["lib"]
    floor = python_floor(export)
    # The binary distribution format's escaped name.
    dist = re.sub(r"[-_.]+", "_", info["name"]).lower()
    version = pep440(info["version"])
    tag = f"cp3{floor}-abi3-{wheel_platform(target)}"
    meta = f"{dist}-{version}.dist-info"
    files = {
        f"{lib}/__init__.py": ((Path(export) / "python" / f"{lib}.py").read_bytes(), 0o644),
        f"{lib}/{library_name(lib, target)}": (Path(library).read_bytes(), 0o755),
    }
    for path in licenses(info):
        files[f"{meta}/licenses/{path.name}"] = (path.read_bytes(), 0o644)
    # PEP 770: the dependencies the library was built from, where a scanner looks.
    files[f"{meta}/sboms/{info['name']}.cdx.json"] = (info["sbom"], 0o644)
    files[f"{meta}/METADATA"] = (metadata(info, floor).encode(), 0o644)
    files[f"{meta}/WHEEL"] = (
        f"Wheel-Version: 1.0\nGenerator: rust2-ffi\nRoot-Is-Purelib: false\nTag: {tag}\n".encode(), 0o644)
    path = Path(out) / f"{dist}-{version}-{tag}.whl"
    seal(path, files, info["epoch"])
    return path


def repair(wheel, target, out, stamp):
    """Hand the wheel to its platform's repair tool, which claims the portable tag or fails."""
    Path(out).mkdir(parents=True, exist_ok=True)
    if "linux" in target:
        arch = target.split("-")[0]
        run("auditwheel", "repair", "--plat", f"manylinux_{GLIBC.replace('.', '_')}_{arch}", "-w", out, wheel)
    elif "apple" in target:
        run("delocate-wheel", "--require-archs", "arm64", "--require-target-macos-version", MACOS_WHEEL,
            "-w", out, wheel)
    else:
        # The wheel's library is a .dll, which delvewheel analyzes only when asked.
        run("delvewheel", "repair", "--analyze-existing", "--wheel-dir", out, wheel)
    repaired = list(Path(out).glob("*.whl"))
    if len(repaired) != 1:
        raise SystemExit(f"repair wrote {len(repaired)} wheels into {out}")
    normalize_wheel(repaired[0], stamp)
    check_wheel(repaired[0])
    return repaired[0]


def check_wheel(path):
    """Fail unless RECORD covers every file with its true size and hash, as pip checks it."""
    with zipfile.ZipFile(path) as archive:
        # Repair tools add directory entries, which RECORD never lists.
        names = {n for n in archive.namelist() if not n.endswith("/")}
        records = [n for n in names if n.endswith(".dist-info/RECORD")]
        if len(records) != 1:
            raise SystemExit(f"{path.name} has {len(records)} RECORD files")
        rows = list(csv.reader(io.StringIO(archive.read(records[0]).decode())))
        listed = {row[0] for row in rows}
        if listed != names:
            raise SystemExit(f"{path.name}: RECORD and the archive differ: {sorted(listed ^ names)}")
        for name, digest, size in rows:
            if name == records[0]:
                continue
            data = archive.read(name)
            want = "sha256=" + base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b"=").decode()
            if digest != want or int(size) != len(data):
                raise SystemExit(f"{path.name}: RECORD's row for {name} is wrong")


def digest(export, lib):
    """A hash of the generated bindings, which every build host's export must agree on."""
    hasher = hashlib.sha256()
    export = Path(export)
    paths = [export / "python" / f"{lib}.py", export / "include" / f"{lib}.h", *sorted((export / "swift").iterdir()),
             *sorted(p for p in (export / "go").rglob("*") if p.is_file())]
    for path in paths:
        hasher.update(path.relative_to(export).as_posix().encode() + b"\0")
        # Exact bytes: each wheel ships its own host's module.
        hasher.update(path.read_bytes() + b"\0")
    return hasher.hexdigest()


def collections():
    """swift-collections' release whose `BasicContainers` the generated Swift was tested with.

    rust2 pins it by revision; a package resolved by version may depend only on a version, so
    the tag at that revision stands in, and its absence is an error.
    """
    revision = re.search(r'swift_collections_rev := "([0-9a-f]{40})"', (ROOT / "ffi/justfile").read_text(encoding="utf-8"))
    if revision is None:
        raise SystemExit("ffi/justfile pins no swift_collections_rev")
    tags = output("git", "ls-remote", "--tags", COLLECTIONS)
    for line in tags.splitlines():
        sha, ref = line.split("\t")
        name = ref.removeprefix("refs/tags/").removesuffix("^{}")
        if sha == revision.group(1) and re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", name):
            return name
    raise SystemExit(f"swift-collections has no release at {revision.group(1)}")


def linker_settings(flags_by_platform):
    """SwiftPM linker settings for the native libraries rustc reports on each platform."""
    seen = {}
    for apple, flags in flags_by_platform.items():
        args = iter(shlex.split(flags))
        for flag in args:
            if flag.startswith("-l") and len(flag) > 2:
                setting = f".linkedLibrary({json.dumps(flag[2:])}"
            elif flag == "-framework":
                setting = f".linkedFramework({json.dumps(next(args))}"
            else:
                raise SystemExit(f"unsupported native linker flag on {apple}: {flag}")
            seen.setdefault(setting, set()).add(apple)
    swift = {"macos": ".macOS", "ios": ".iOS"}
    out = []
    for setting, platforms in seen.items():
        if platforms == set(flags_by_platform):
            out.append(setting + ")")
        else:
            out.append(setting + ", .when(platforms: [" + ", ".join(swift[p] for p in sorted(platforms)) + "]))")
    return out


def package_swift(module, version, settings, binary):
    """The package manifest: the Swift source over the XCFramework, either by URL or by path."""
    platforms = f'.macOS("{SWIFT_PLATFORMS["macos"]}"), .iOS("{SWIFT_PLATFORMS["ios"]}")'
    links = "".join(f"\n                {s}," for s in settings)
    return f'''// swift-tools-version: 6.2
// Generated by rust2-ffi's release builder.
import PackageDescription

let package = Package(
    name: "{module}",
    platforms: [{platforms}],
    products: [.library(name: "{module}", targets: ["{module}"])],
    dependencies: [.package(url: "{COLLECTIONS}", exact: "{version}")],
    targets: [
        {binary},
        .target(
            name: "{module}",
            dependencies: ["{module}FFI", .product(name: "BasicContainers", package: "swift-collections")],
            linkerSettings: [{links}
            ]
        ),
    ]
)
'''


def min_os(archive):
    """The newest minimum OS any object in a static library declares."""
    versions = set()
    for line in output("/usr/bin/otool", "-l", archive).splitlines():
        match = re.match(r"\s*minos ([0-9.]+)$", line)
        if match:
            versions.add(tuple(int(x) for x in match.group(1).split(".")))
    if not versions:
        raise SystemExit(f"{archive} declares no minimum OS")
    return max(versions)


def sort_plist(path):
    """Rewrite an XCFramework's Info.plist with its libraries in name order, which xcodebuild lists in a random one."""
    with open(path, "rb") as source:
        plist = plistlib.load(source)
    plist["AvailableLibraries"].sort(key=lambda library: library["LibraryIdentifier"])
    with open(path, "wb") as sink:
        plistlib.dump(plist, sink, sort_keys=True)


def zip_tree(root, path, stamp):
    """The directory `root` as a ZIP holding it as its top-level directory, as `ditto --keepParent` does."""
    entries = [(f"{root.name}/", b"", 0o755)]
    for found in sorted(root.rglob("*"), key=lambda p: p.relative_to(root).as_posix()):
        name = f"{root.name}/{found.relative_to(root).as_posix()}"
        if found.is_symlink():
            raise SystemExit(f"{found} is a symlink: the XCFramework ZIP holds only files")
        if found.is_dir():
            entries.append((f"{name}/", b"", 0o755))
        else:
            entries.append((name, found.read_bytes(), 0o755 if found.stat().st_mode & 0o111 else 0o644))
    write_zip(path, entries, stamp)


def swift(info, export, libs, natives, out):
    """The XCFramework of every Apple slice, its checksummed ZIP, and the package around it."""
    if sys.platform != "darwin":
        raise SystemExit("XCFramework packaging needs Apple's xcodebuild, on macOS")
    if set(libs) != set(APPLE_TARGETS):
        raise SystemExit(f"the Swift package ships {sorted(APPLE_TARGETS)}, given {sorted(libs)}")
    lib, export, out = info["lib"], Path(export), Path(out)
    module = pascal(lib)
    for target, path in libs.items():
        floor = tuple(int(x) for x in SWIFT_PLATFORMS[APPLE_TARGETS[target]].split("."))
        if min_os(path) > floor:
            raise SystemExit(f"{path} needs a newer OS than {SWIFT_PLATFORMS[APPLE_TARGETS[target]]}: build it with the deployment target set")
    if out.exists():
        shutil.rmtree(out)
    headers = out / "headers"
    headers.mkdir(parents=True)
    shutil.copy(export / "include" / f"{lib}.h", headers)
    shutil.copy(export / "swift" / f"{lib}_atomics.h", headers)
    modulemap = (export / "swift/module.modulemap").read_text(encoding="utf-8")
    if f"../include/{lib}.h" not in modulemap:
        raise SystemExit("the module map no longer names ../include: update the packager")
    (headers / "module.modulemap").write_text(modulemap.replace(f"../include/{lib}.h", f"{lib}.h"), encoding="utf-8")
    framework = out / f"{module}FFI.xcframework"
    archive = out / f"{module}FFI.xcframework.zip"
    again = out / "again"
    # The XCFramework twice, which must zip to the same bytes: the checksum a consumer
    # resolves is of this ZIP, and a rebuild must make it again.
    for tree, zipped in ((framework, archive), (again / framework.name, again / archive.name)):
        args = ["xcodebuild", "-create-xcframework"]
        for target in APPLE_TARGETS:
            args += ["-library", libs[target], "-headers", headers]
        # libtool, which xcodebuild runs on each library, stamps the time unless told not to.
        run(*args, "-output", tree, env={**os.environ, "ZERO_AR_DATE": "1"})
        sort_plist(tree / "Info.plist")
        zip_tree(tree, zipped, info["epoch"])
    if archive.read_bytes() != (again / archive.name).read_bytes():
        raise SystemExit("xcodebuild made different XCFrameworks from the same libraries:\n"
                         + "\n".join(zip_diff(archive, again / archive.name)))
    shutil.rmtree(headers)
    shutil.rmtree(again)
    checksum = hashlib.sha256(archive.read_bytes()).hexdigest()
    # SwiftPM's own computation, as a consumer resolving the URL runs it.
    swift_sum = output("swift", "package", "compute-checksum", archive).strip()
    if swift_sum != checksum:
        raise SystemExit(f"swift computes {swift_sum}, not {checksum}")
    (out / "checksum.txt").write_text(checksum + "\n", encoding="utf-8")
    package = out / "package"
    sources = package / "Sources" / module
    sources.mkdir(parents=True)
    shutil.copy(export / "swift" / f"{lib}.swift", sources)
    for path in licenses(info):
        shutil.copy(path, package)
    by_platform = {}
    for target, apple in APPLE_TARGETS.items():
        by_platform.setdefault(apple, set()).add(Path(natives[target]).read_text(encoding="utf-8").strip())
    flags = {}
    for apple, found in by_platform.items():
        if len(found) != 1:
            raise SystemExit(f"{apple}'s slices link different native libraries: {sorted(found)}")
        flags[apple] = found.pop()
    state = {"collections": collections(), "settings": linker_settings(flags), "module": module}
    (out / "swift.json").write_text(json.dumps(state, indent=2) + "\n", encoding="utf-8")
    shutil.rmtree(framework)
    return state


def release_manifest(out, url):
    """Package.swift naming the hosted ZIP at `url` by its checksum."""
    out = Path(out)
    state = json.loads((out / "swift.json").read_text(encoding="utf-8"))
    checksum = (out / "checksum.txt").read_text(encoding="utf-8").strip()
    binary = f'.binaryTarget(name: "{state["module"]}FFI", url: {json.dumps(url)}, checksum: "{checksum}")'
    text = package_swift(state["module"], state["collections"], state["settings"], binary)
    (out / "package" / "Package.swift").write_text(text, encoding="utf-8")


def local_manifest(state, package, framework):
    """Package.swift naming an unpacked XCFramework by path, for a consumer before any upload."""
    package = Path(package)
    state = json.loads(Path(state).read_text(encoding="utf-8"))
    binary = f'.binaryTarget(name: "{state["module"]}FFI", path: {json.dumps(os.path.relpath(framework, package))})'
    (package / "Package.swift").write_text(package_swift(state["module"], state["collections"], state["settings"], binary), encoding="utf-8")


def compare(native, actual, what):
    """Fail with a diff unless a consumer printed exactly what native Rust printed."""
    if native == actual:
        return
    sys.stderr.buffer.writelines(difflib.diff_bytes(
        difflib.unified_diff, native.splitlines(keepends=True), actual.splitlines(keepends=True),
        fromfile=b"native Rust", tofile=what.encode()))
    raise SystemExit(f"{what} differs from native Rust")


def consume_python(check, native, module):
    """Run a check program, or else an import, on the installed wheel from an empty directory."""
    program = [str(Path(check).resolve())] if check else ["-c", f"import {module}"]
    with tempfile.TemporaryDirectory() as clean:
        result = subprocess.run([sys.executable, "-I", "-X", "dev", "-W", "error", *program],
                                capture_output=True, cwd=clean)
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{check} failed on the installed wheel with status {result.returncode}")
    if native:
        # Windows' CPython writes text-mode stdout with CRLF, where Rust's println! writes LF.
        actual = result.stdout.replace(b"\r\n", b"\n") if sys.platform == "win32" else result.stdout
        compare(Path(native).read_bytes(), actual, "installed wheel")


def exporter(manifest, features, *args):
    """Run rust2-ffi from this checkout on the package, with its ffi.toml if it keeps one."""
    manifest = Path(manifest).resolve()
    config = manifest.parent / "ffi.toml"
    if not config.exists():
        # The exporter still requires a config file; an empty one leaves nothing out.
        config = Path(tempfile.mkdtemp()) / "ffi.toml"
        config.write_text("", encoding="utf-8")
    cargo = ["--", "--features", features] if features.strip() else []
    run("cargo", "run", "--locked", "--release", "--quiet", "--", "--config", config, "--manifest-path", manifest,
        *args, *cargo, cwd=ROOT / "ffi")


def stage_api(args):
    """Each target's API, checked on this host, for the build hosts' exports to merge."""
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)
    for target in args.targets:
        exporter(args.manifest_path, args.features, "--target", target, "--api", out / f"{target}.json")


def export(args):
    """The binding of every target: this host's checked here, the others' staged elsewhere."""
    staged = sorted(Path(args.api).resolve().glob("*.json"))
    others = [p for p in staged if p.stem not in args.targets]
    if len(others) + len(args.targets) != len(staged):
        raise SystemExit(f"{args.api} lacks a staged API for one of {args.targets}")
    targets = [arg for target in args.targets for arg in ("--target", target)]
    withs = [arg for path in others for arg in ("--with", path)]
    out = Path(args.out).resolve()
    if args.go_module:
        # The export keeps a go.mod in go/, whose module its packages import each other by.
        (out / "go").mkdir(parents=True, exist_ok=True)
        (out / "go" / "go.mod").write_text(f"module {args.go_module}\n\ngo 1.24\n", newline="\n", encoding="utf-8")
    exporter(args.manifest_path, args.features, *targets, *withs, "--out", out)


def rustflags_config(directory, target):
    """What sets rustflags for cargo where a RUSTFLAGS override would silently replace it."""
    here = Path(directory).resolve()
    home = Path(os.environ.get("CARGO_HOME") or Path.home() / ".cargo")
    for root in [home, *(r / ".cargo" for r in [here, *here.parents])]:
        for name in ("config.toml", "config"):
            found = root / name
            if not found.is_file():
                continue
            config = tomllib.loads(found.read_text(encoding="utf-8"))
            tables = {key: table for key, table in config.get("target", {}).items() if key == target or key.startswith("cfg(")}
            if "rustflags" in config.get("build", {}) or any("rustflags" in table for table in tables.values()):
                yield str(found)
    for key in os.environ:
        if key == "CARGO_BUILD_RUSTFLAGS" or re.fullmatch(r"CARGO_TARGET_.*_RUSTFLAGS", key):
            yield key


def reproducible(info, export_dir, target):
    """The environment that makes a build's bytes the same on any host, at any path.

    Release time is the commit's, and each directory the build reads from, which rustc and
    C compilers write into panic messages and debug info, is spelled the same.
    """
    pairs = {
        info["workspace"]: "/source",
        str(export_dir): "/export",
        os.environ.get("CARGO_HOME") or str(Path.home() / ".cargo"): "/cargo",
    }
    # The deepest last, which rustc prefers where one directory holds another.
    pairs = sorted(pairs.items(), key=lambda pair: len(pair[0]))
    for found in rustflags_config(info["dir"], target):
        raise SystemExit(f"{found} sets rustflags, which the reproducible build's RUSTFLAGS would replace: export RUSTFLAGS instead")
    encoded = os.environ.get("CARGO_ENCODED_RUSTFLAGS")
    if encoded is None:
        flags = os.environ.get("RUSTFLAGS", "").split()
    else:
        # Cargo reads an empty value as no flags, not as one empty flag.
        flags = encoded.split("\x1f") if encoded else []
    flags += [f"--remap-path-prefix={old}={new}" for old, new in pairs]
    env = {"SOURCE_DATE_EPOCH": str(info["epoch"])}
    if "msvc" in target:
        # link.exe stamps the build time into the DLL unless told to hash its content.
        flags += ["-C", "link-arg=/Brepro"]
    else:
        # cc splits CFLAGS on whitespace, and quoting them would change how a caller's own flags read.
        for old, _ in pairs:
            if any(c.isspace() for c in old):
                raise SystemExit(f"{old} holds whitespace, which would split its -ffile-prefix-map in CFLAGS: build from a path without it")
        # cc reads CXXFLAGS, not CFLAGS, for C++.
        for var in ("CFLAGS", "CXXFLAGS"):
            env[var] = " ".join(filter(None, [os.environ.get(var), *(f"-ffile-prefix-map={old}={new}" for old, new in pairs)]))
    if "apple" in target:
        # Apple's ar and libtool stamp archive members with the time unless told not to, and
        # the linker names a dylib after the path it wrote it to, in the export's build directory.
        env["ZERO_AR_DATE"] = "1"
        flags += ["-C", f"link-arg=-Wl,-install_name,@rpath/{library_name(info['lib'], target)}"]
    env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(flags)
    return env


def purl(package):
    """The crate's package URL, qualified by its source unless that is crates.io, so two sources of one version stay two components."""
    base = f"pkg:cargo/{package['name']}@{package['version']}"
    source = package.get("source") or ""
    # An alternate registry may speak the git or the sparse protocol; crates.io reports the former either way.
    if source.startswith(("registry+", "sparse+")) and source != CRATES_IO:
        return f"{base}?repository_url={urllib.parse.quote(source.split('+', 1)[1], safe='')}"
    if source.startswith("git+"):
        return f"{base}?vcs_url={urllib.parse.quote(source, safe='')}"
    return base


def sbom(info, crate):
    """A CycloneDX SBOM of the crates the binding is built from, the same for every host and target."""
    meta = json.loads(output("cargo", "metadata", "--locked", "--format-version", "1", "--features", "python",
                             "--manifest-path", crate))
    lock = tomllib.loads((Path(crate).parent / "Cargo.lock").read_text(encoding="utf-8"))
    checksums = {(p["name"], p["version"], p.get("source")): p["checksum"] for p in lock["package"] if "checksum" in p}
    packages = {p["id"]: p for p in meta["packages"]}
    nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
    root = f"pkg:cargo/{info['name']}@{info['version']}"
    # The generated crate is the package it exports: its dependencies are the package's.
    ref = lambda i: root if i == meta["resolve"]["root"] or purl(packages[i]) == root else purl(packages[i])  # noqa: E731
    seen, queue, depends = set(), [meta["resolve"]["root"]], {}
    while queue:
        node = queue.pop()
        if node in seen:
            continue
        seen.add(node)
        # What the library links is its normal dependencies; a proc-macro or build script only runs at build time.
        shipped = [d["pkg"] for d in nodes[node]["deps"]
                   if any(k["kind"] is None for k in d["dep_kinds"])
                   and not any("proc-macro" in t["kind"] for t in packages[d["pkg"]]["targets"])]
        depends.setdefault(ref(node), set()).update(ref(i) for i in shipped)
        queue += shipped
    components = {ref(i): packages[i] for i in sorted(seen) if ref(i) != root}
    def component(name, package):
        out = {"type": "library", "bom-ref": name, "name": package["name"], "version": package["version"], "purl": name}
        if package.get("license"):
            # Cargo still takes the legacy MIT/Apache-2.0, which is no SPDX expression.
            out["licenses"] = [{"license": {"name": package["license"]}} if "/" in package["license"]
                               else {"expression": package["license"]}]
        if package.get("repository"):
            out["externalReferences"] = [{"type": "vcs", "url": package["repository"]}]
        checksum = checksums.get((package["name"], package["version"], package.get("source")))
        if checksum:
            out["hashes"] = [{"alg": "SHA-256", "content": checksum}]
        return out
    document = {
        "bomFormat": "CycloneDX",
        "specVersion": "1.6",
        "version": 1,
        "serialNumber": "",
        "metadata": {
            "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(info["epoch"])),
            "component": {"type": "library", "bom-ref": root, "name": info["name"], "version": info["version"], "purl": root},
        },
        "components": [component(name, components[name]) for name in sorted(components)],
        "dependencies": [{"ref": name, "dependsOn": sorted(depends[name] - {name})} for name in sorted(depends)],
    }
    # actions/attest takes a CycloneDX file only with a serial number: derived from the content, so every host writes the same.
    document["serialNumber"] = f"urn:uuid:{uuid.uuid5(uuid.NAMESPACE_URL, json.dumps(document, sort_keys=True))}"
    return (json.dumps(document, indent=2) + "\n").encode()


def zip_diff(one, two):
    """The entries two ZIPs hold differently, each with its content hash, time, and attributes."""
    with zipfile.ZipFile(one) as a, zipfile.ZipFile(two) as b:
        entries = lambda z: {i.filename: (hashlib.sha256(z.read(i)).hexdigest(), i.date_time, i.external_attr) for i in z.infolist()}  # noqa: E731
        left, right = entries(a), entries(b)
    return [f"  {name}: {left.get(name)} against {right.get(name)}"
            for name in sorted(left.keys() | right.keys()) if left.get(name) != right.get(name)]


def ar_entries(data, label="the archive"):
    """An archive's `(name, header fields, start, end)` per member, its symbol table and names table by their own."""
    if data[:8] != b"!<arch>\n":
        raise SystemExit(f"{label} is no archive")
    entries, long_names, offset = [], b"", 8
    while offset + 60 <= len(data):
        name, date, uid, gid, mode, size = (data[offset + a:offset + b].decode("ascii", "replace").strip()
                                            for a, b in ((0, 16), (16, 28), (28, 34), (34, 40), (40, 48), (48, 58)))
        start, end = offset + 60, offset + 60 + int(size)
        offset = end + int(size) % 2
        if name == "//":
            long_names = data[start:end]
        elif name.startswith("/") and name[1:].isdigit():
            first = int(name[1:])
            name = long_names[first:long_names.index(b"\n", first)].decode("utf-8", "replace").rstrip("/")
        else:
            name = name.rstrip("/") or "/"
        entries.append((name, (date, uid, gid, mode), start, end))
    return entries


def ar_members(path):
    """An archive's members as `{name: (content, time, owner, group, mode)}`."""
    data = Path(path).read_bytes()
    members = {}
    for name, fields, start, end in ar_entries(data, path):
        # A name twice, as an archive may hold, is told apart by its place.
        while name in members:
            name += "'"
        members[name] = (data[start:end], *fields)
    return members


def ar_diff(one, two):
    """The members two archives hold differently, each with where its content first differs."""
    left, right = ar_members(one), ar_members(two)
    lines = []
    for name in sorted(left.keys() | right.keys()):
        a, b = left.get(name), right.get(name)
        if a == b:
            continue
        if a is None or b is None:
            lines.append(f"  {name}: only in the {'first' if b is None else 'second'}")
            continue
        at = next((i for i, (x, y) in enumerate(zip(a[0], b[0])) if x != y), min(len(a[0]), len(b[0])))
        lines.append(f"  {name}: {len(a[0])} against {len(b[0])} bytes, {a[1:]} against {b[1:]}; first differing byte {at}: "
                     f"{a[0][max(at - 24, 0):at + 40]!r} against {b[0][max(at - 24, 0):at + 40]!r}")
    return lines


def same(first, second):
    """Fail unless two builds' wheels and static libraries, the Go module's included, are the same bytes, naming what differs."""
    found = {}
    for root in (Path(first), Path(second)):
        found[root] = {p.relative_to(root).as_posix(): p for pattern in ("wheels/*.whl", "static/*/*.a", "go/*.a") for p in root.glob(pattern)}
    names = sorted(found[Path(first)])
    if not names or names != sorted(found[Path(second)]):
        raise SystemExit(f"the builds hold different packages: {names} against {sorted(found[Path(second)])}")
    problems = []
    for name in names:
        one, two = (found[Path(root)][name] for root in (first, second))
        if one.read_bytes() == two.read_bytes():
            continue
        problems.append(name)
        if name.endswith(".whl"):
            problems += zip_diff(one, two)
        elif name.endswith(".a"):
            problems += ar_diff(one, two)
    if problems:
        raise SystemExit("a rebuild made other bytes:\n" + "\n".join(problems))
    print(f"{len(names)} files rebuilt byte for byte")


def cargo(manifest, *args, env=None):
    """Cargo with the package's own toolchain, as rustup finds it from the package's directory."""
    return run("cargo", *args, cwd=Path(manifest).resolve().parent, env={**os.environ, **(env or {})},
               capture_output=True, encoding="utf-8")


def staticlib(manifest, crate, lib, target, env=None):
    """The C library alone, apart from the Python extension, whose CPython symbols it would
    leave undefined, and the system libraries rustc says it needs beside it."""
    # Its own target directory, since cargo does not rebuild for a changed deployment target.
    target_dir = crate.parent / ("target-swift" if target in APPLE_TARGETS else "target-go")
    # cargo-zigbuild links Linux against the glibc floor's symbols, whichever glibc the host has.
    linux = "linux" in target
    # Uncolored, so the note below reads as written whatever the caller's terminal.
    built = run(*(["cargo-zigbuild"] if linux else ["cargo"]), "rustc", "--color", "never", "--locked", "--release",
                "--manifest-path", crate, "--target", f"{target}.{GLIBC}" if linux else target,
                "--target-dir", target_dir, "--lib", "--crate-type", "staticlib", "--", "--print=native-static-libs",
                cwd=Path(manifest).resolve().parent, env={**os.environ, **(env or {})},
                capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    notes = re.findall(r"note: native-static-libs: (.*)", built.stderr)
    if len(notes) != 1:
        raise SystemExit(f"rustc reported no native libraries for {target}")
    return target_dir / target / "release" / f"lib{lib}.a", notes[0].strip()


def go_links(export, lib, target):
    """The system libraries the export's Go links beside `target`'s library, from its #cgo line."""
    platform = GO_TARGETS[target]
    rt = (Path(export) / "go" / "rt" / "rt.go").read_text(encoding="utf-8")
    match = re.search(rf"^#cgo \S+ LDFLAGS: \$\{{SRCDIR\}}/lib{lib}_{platform}\.a(.*)$", rt, re.M)
    if match is None:
        raise SystemExit(f"the export's Go links no library for {target}")
    return match.group(1).strip()


def llvm_tool(manifest, name):
    """One of LLVM's tools from the toolchain building the crate, which reads every target's objects."""
    cwd = Path(manifest).resolve().parent
    run("rustup", "component", "add", "llvm-tools", cwd=cwd, capture_output=True)
    sysroot = output("rustc", "--print", "sysroot", cwd=cwd).strip()
    host = re.search(r"^host: (\S+)$", output("rustc", "-vV", cwd=cwd), re.M).group(1)
    exe = ".exe" if sys.platform == "win32" else ""
    return Path(sysroot) / "lib" / "rustlib" / host / "bin" / f"{name}{exe}"


# The random directory rustc imports a Windows GNU library's raw-dylib symbols through, which
# dlltool names them after, as `<path>_out_rustc<6 random characters>_kernel32_dll_imports_lib`.
RUSTC_TEMP = re.compile(rb"(?<=_out_rustc)[0-9A-Za-z]{6}(?=_[0-9a-z_]+_dll_imports_lib)")
IMPORT_MEMBER = re.compile(r".+\.dll(h|t|s[0-9]+)\.o")


def fix_import_names(path):
    """Replace that random part, at the same length, in the import library's members and the archive's symbol table alone, so the archive is the same on every build and no other byte changes."""
    data = bytearray(Path(path).read_bytes())
    for name, _, start, end in ar_entries(bytes(data)):
        if name == "/" or IMPORT_MEMBER.fullmatch(name):
            data[start:end] = RUSTC_TEMP.sub(b"000000", bytes(data[start:end]))
    Path(path).write_bytes(bytes(data))


def go_library(args, export_dir, lib, target, library, native, out):
    """The Go module's copy of `target`'s library: checked against the export's #cgo line,
    without debug info, which only bloats the module, or the bitcode the go command refuses."""
    want = go_links(export_dir, lib, target)
    if native != want:
        raise SystemExit(f"rustc links {target}'s library with `{native}`, but the export's Go with `{want}`")
    objcopy = llvm_tool(args.manifest_path, "llvm-objcopy")
    dest = out / "go" / f"lib{lib}_{GO_TARGETS[target]}.a"
    dest.parent.mkdir(parents=True, exist_ok=True)
    run(objcopy, "--strip-debug", *(f"--remove-section={s}" for s in LTO_SECTIONS), library, dest)
    if "windows-gnu" in target:
        fix_import_names(dest)


def build(args):
    """This host's wheels, Apple and Go static libraries, the export's sources, and native output."""
    info = facts(args.manifest_path)
    lib, export_dir, out = info["lib"], Path(args.export).resolve(), Path(args.out).resolve()
    crate = export_dir / "Cargo.toml"
    out.mkdir(parents=True, exist_ok=True)
    info["sbom"] = sbom(info, crate)
    (out / "sbom.cdx.json").write_bytes(info["sbom"])
    for target in args.targets:
        if target not in WHEEL_TARGETS and target not in APPLE_TARGETS and target not in GO_TARGETS:
            raise SystemExit(f"no package ships {target}")
        run("rustup", "target", "add", target, cwd=Path(args.manifest_path).resolve().parent)
        if target in WHEEL_TARGETS:
            env = reproducible(info, export_dir, target)
            if "apple" in target:
                env["MACOSX_DEPLOYMENT_TARGET"] = MACOS_WHEEL
            linux = "linux" in target
            # cargo-zigbuild links Linux against the glibc floor's symbols, whichever glibc the host has.
            built = cargo(args.manifest_path, *(["zigbuild"] if linux else ["build"]), "--locked", "--release",
                          "--manifest-path", crate, "--target", f"{target}.{GLIBC}" if linux else target,
                          "--target-dir", export_dir / "target-python", "--features", "python", env=env)
            sys.stderr.write(built.stderr)
            library = export_dir / "target-python" / target / "release" / library_name(lib, target)
            with tempfile.TemporaryDirectory() as raw:
                repair(write_wheel(info, export_dir, library, target, raw), target, out / "wheels", info["epoch"])
        if target in APPLE_TARGETS:
            apple = APPLE_TARGETS[target]
            name = "MACOSX_DEPLOYMENT_TARGET" if apple == "macos" else "IPHONEOS_DEPLOYMENT_TARGET"
            library, native = staticlib(args.manifest_path, crate, lib, target,
                                        env={**reproducible(info, export_dir, target), name: SWIFT_PLATFORMS[apple]})
            static = out / "static" / target
            static.mkdir(parents=True)
            shutil.copy(library, static)
            (static / "native.txt").write_text(native + "\n", encoding="utf-8")
            # The Go module links the Swift package's macOS library, at its floor.
            if target in GO_TARGETS:
                go_library(args, export_dir, lib, target, library, native, out)
        elif target in GO_TARGETS:
            library, native = staticlib(args.manifest_path, crate, lib, target, env=reproducible(info, export_dir, target))
            go_library(args, export_dir, lib, target, library, native, out)
    # The generated sources, for the Swift package, the Go module, and the hosts' agreement.
    # Text this writes is LF on every host, as the jobs reading it compare bytes.
    for part in ("include", "swift", "python", "go"):
        shutil.copytree(export_dir / part, out / "export" / part, dirs_exist_ok=True)
    shutil.copy(crate, out / "export")
    (out / "digest.txt").write_text(digest(export_dir, lib) + "\n", newline="\n", encoding="utf-8")
    (out / "lib.txt").write_text(lib + "\n", newline="\n", encoding="utf-8")
    # Native output for a host whose wheel or Go module a consumer runs; an iOS slice runs nowhere here.
    if args.example and any(target in WHEEL_TARGETS for target in args.targets):
        if args.example not in info["examples"]:
            raise SystemExit(f"{info['name']} has no example {args.example}: it has {info['examples']}")
        features = ["--features", args.features] if args.features.strip() else []
        manifest = Path(args.manifest_path).resolve()
        # Bytes, as the consumers' output is compared, with no newline translation.
        native = run("cargo", "run", "--locked", "--release", "--quiet", "--manifest-path", manifest,
                     "--example", args.example, *features, cwd=manifest.parent, capture_output=True)
        (out / "native.txt").write_bytes(native.stdout)


def go_version(version):
    """Cargo's version as a Go module's: semver with a `v`, refusing build metadata, which Go has no spelling of."""
    if SEMVER.fullmatch(version) is None:
        raise SystemExit(f"version {version} is no Go module version: use X.Y.Z or X.Y.Z-(alpha|beta|rc).N")
    return f"v{version}"


def go_module_path(export):
    """The module the export's go.mod declares, as the go command reads it."""
    path = Path(export) / "go" / "go.mod"
    module = json.loads(output("go", "mod", "edit", "-json", path, env={**os.environ, "GOTOOLCHAIN": "local"})).get("Module")
    if not module:
        raise SystemExit(f"{path} declares no module")
    return module["Path"]


def check_major(module, version):
    """Fail unless the module path carries the suffix Go requires of its major version."""
    major = int(version[1:].split(".")[0])
    suffix = re.search(r"/v([0-9]+)$", module)
    if major >= 2 and (suffix is None or int(suffix.group(1)) != major):
        raise SystemExit(f"Go requires version {version} of {module} to be at a module path ending /v{major}")
    if major < 2 and suffix is not None:
        raise SystemExit(f"Go refuses version {version} of {module}, whose path names major version {suffix.group(1)}")


def escape(module):
    """A module path as a proxy's URL spells it, each capital letter as `!` and its lower case."""
    return re.sub(r"[A-Z]", lambda m: "!" + m.group(0).lower(), module)


def go_files(info, export, libs):
    """The module's files by path: the export's Go, each library its runtime links from the
    directory `libs` beside it, and the license files, at the root where
    pkg.go.dev looks for them."""
    root = Path(export) / "go"
    files = {p.relative_to(root).as_posix(): p.read_bytes() for p in sorted(root.rglob("*")) if p.is_file()}
    # Exactly the libraries the runtime's #cgo lines name.
    want = set(re.findall(r"^#cgo \S+ LDFLAGS: \$\{SRCDIR\}/(\S+\.a)", files["rt/rt.go"].decode(), re.M))
    if want != {f"lib{info['lib']}_{platform}.a" for platform in GO_TARGETS.values()}:
        raise SystemExit(f"the export's Go links {sorted(want)}, not each of {sorted(GO_TARGETS)}")
    found = {p.name for p in Path(libs).iterdir()}
    if found != want:
        raise SystemExit(f"the Go module links {sorted(want)}, given {sorted(found)}")
    for name in sorted(want):
        files[f"rt/{name}"] = (Path(libs) / name).read_bytes()
    for path in licenses(info):
        files[path.name] = path.read_bytes()
    # Two files a case-insensitive file system would make one, which the go command refuses.
    folded = {}
    for name in files:
        if folded.setdefault(name.lower(), name) != name:
            raise SystemExit(f"the Go module holds both {folded[name.lower()]} and {name}")
    return files


def go_module(info, export, libs, out):
    """The module's .info, .mod, and .zip, laid out as a GOPROXY serves them, under `out`."""
    module, version = go_module_path(export), go_version(info["version"])
    check_major(module, version)
    files = go_files(info, export, libs)
    if len(files["go.mod"]) > GO_MOD_MAX:
        raise SystemExit(f"go.mod is {len(files['go.mod'])} bytes, over Go's {GO_MOD_MAX}")
    size = sum(len(data) for data in files.values())
    if size > GO_ZIP_MAX:
        raise SystemExit(f"the Go module's files are {size >> 20} MiB, over Go's {GO_ZIP_MAX >> 20} MiB")
    versions = Path(out) / escape(module) / "@v"
    versions.mkdir(parents=True)
    (versions / f"{version}.info").write_text(json.dumps({"Version": version}) + "\n", encoding="utf-8")
    (versions / f"{version}.mod").write_bytes(files["go.mod"])
    # What `go get <module>` lists to pick a version: this release's alone, which a proxy
    # serving earlier ones appends to its own list.
    (versions / "list").write_text(version + "\n", encoding="utf-8")
    archive = versions / f"{version}.zip"
    # Stamped as every other ZIP here, so the zip is a function of its files and the commit.
    write_zip(archive, [(f"{module}@{version}/{name}", data, 0o644) for name, data in files.items()], info["epoch"])
    if archive.stat().st_size > GO_ZIP_MAX:
        raise SystemExit(f"the Go module's zip is {archive.stat().st_size >> 20} MiB, over Go's {GO_ZIP_MAX >> 20} MiB")
    rows = [f"| `{name}` | {len(data) / (1 << 20):.1f} MiB |" for name, data in files.items() if name.endswith(".a")]
    return "\n".join([
        f"Go module `{module}` {version}: {size / (1 << 20):.1f} MiB of files, a "
        f"{archive.stat().st_size / (1 << 20):.1f} MiB zip, of Go's {GO_ZIP_MAX >> 20} MiB limit on each.",
        "", "| Library | Size |", "| --- | --- |", *rows, "",
    ])


def zip_root(name):
    """The module and version a file of a module zip, `<module>@<version>/<path>`, is under."""
    match = re.match(r"([^@]+)@(v[^/]+)/", name)
    if match is None:
        raise SystemExit(f"{name} is in no module's zip")
    return match.groups()


def consume_go(proxy, check, native):
    """Build and run a check program, or else an import of the runtime, against the module
    unpacked from its zip, in a new module of its own: no checkout, Rust, or CGO_LDFLAGS."""
    zips = list(Path(proxy).rglob("*.zip"))
    if len(zips) != 1:
        raise SystemExit(f"expected one module zip in {proxy}, found {zips}")
    with tempfile.TemporaryDirectory() as work, zipfile.ZipFile(zips[0]) as z:
        work = Path(work)
        module, version = zip_root(z.namelist()[0])
        prefix = f"{module}@{version}"
        z.extractall(work / "mod")
        consumer = work / "consumer"
        consumer.mkdir()
        (consumer / "go.mod").write_text(
            f"module consumer\n\ngo 1.24\n\nrequire {module} {version}\n\nreplace {module} => ../mod/{prefix}\n",
            encoding="utf-8")
        main = Path(check).read_text(encoding="utf-8") if check else f'package main\n\nimport _ "{module}/rt"\n\nfunc main() {{}}\n'
        (consumer / "main.go").write_text(main, encoding="utf-8")
        env = {k: v for k, v in os.environ.items() if not k.startswith("CGO_") or k == "CGO_ENABLED"}
        env.update(CGO_ENABLED="1", GOFLAGS="-mod=mod", GOPROXY="off", GOTOOLCHAIN="local", GOWORK="off")
        run("go", "vet", f"{module}/...", ".", cwd=consumer, env=env)
        exe = consumer / ("check.exe" if sys.platform == "win32" else "check")
        run("go", "build", "-o", exe, ".", cwd=consumer, env=env)
        result = subprocess.run([exe], capture_output=True, cwd=work)
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{check or 'the import'} failed on the Go module with status {result.returncode}")
    if native and check:
        compare(Path(native).read_bytes(), result.stdout, "Go module")


def template(info, manifest, rust2, mirror, go_module):
    """The caller workflow, with every identity Cargo cannot establish spelled out in it."""
    if not re.fullmatch(r"[0-9a-f]{40}", rust2):
        raise SystemExit("pin rust2 by a full commit SHA")
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", mirror):
        raise SystemExit(f"the Swift mirror must be OWNER/REPO, not {mirror!r}")
    # A proxy serves only a path whose first element is a host name.
    if not re.fullmatch(r"[a-z0-9-]+(\.[a-z0-9-]+)+(/[A-Za-z0-9._~-]+)*", go_module):
        raise SystemExit(f"the Go module must be a path a proxy serves, such as example.com/name, not {go_module!r}")
    check_major(go_module, go_version(info["version"]))
    values = {
        "GO_MODULE": go_module,
        "NAME": info["name"],
        "MANIFEST": manifest,
        "RUST2": rust2,
        "MODULE": pascal(info["lib"]),
        "MIRROR": mirror,
        "MIRROR_OWNER": mirror.split("/")[0],
        "MIRROR_REPO": mirror.split("/")[1],
    }
    text = (HERE / "release.yml").read_text(encoding="utf-8")
    for key, value in values.items():
        text = text.replace(f"@{key}@", value)
    leftover = sorted(set(re.findall(r"@[A-Z0-9_]+@", text)))
    if leftover:
        raise SystemExit(f"release.yml has unfilled placeholders: {leftover}")
    return text


def init(args):
    """Write .github/workflows/release-<crate>.yml into the repository holding the crate."""
    manifest = Path(args.manifest_path)
    root = Path(output("git", "-C", manifest.resolve().parent, "rev-parse", "--show-toplevel").strip())
    info = facts(manifest)
    rust2 = args.rust2 or output("git", "-C", HERE, "rev-parse", "HEAD").strip()
    relative = manifest.resolve().relative_to(root).as_posix()
    path = root / ".github" / "workflows" / f"release-{info['name']}.yml"
    text = template(info, relative, rust2, args.swift_mirror, args.go_module)
    if path.exists() and path.read_text(encoding="utf-8") != text:
        raise SystemExit(f"{path} exists; delete it to write the release workflow")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, newline="\n", encoding="utf-8")
    print(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("facts", help="print the package's Cargo facts as GitHub outputs")
    p.add_argument("--manifest-path", required=True)
    for name, helptext in (("api", "stage each target's API on this host"),
                           ("export", "export the binding of every staged target"),
                           ("build", "build this host's wheels and Apple libraries")):
        p = sub.add_parser(name, help=helptext)
        p.add_argument("--manifest-path", required=True)
        p.add_argument("--features", default="")
        p.add_argument("--out", required=True)
        p.add_argument("targets", nargs="+")
        if name == "export":
            p.add_argument("--api", required=True)
            p.add_argument("--go-module", default="", help="the Go module path; the export's own, the prefix, by default")
        if name == "build":
            p.add_argument("--export", required=True)
            p.add_argument("--example", default="")
    p = sub.add_parser("same", help="fail unless two builds made the same wheels and libraries")
    p.add_argument("first")
    p.add_argument("second")
    p = sub.add_parser("digest", help="print the hash of the generated bindings")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p = sub.add_parser("swift", help="write the XCFramework ZIP and the package around it")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p.add_argument("--lib", action="append", default=[], metavar="TARGET=PATH")
    p.add_argument("--native", action="append", default=[], metavar="TARGET=PATH")
    p.add_argument("--url", required=True, help="where the release hosts the ZIP")
    p.add_argument("--out", required=True)
    p = sub.add_parser("swift-local", help="point an unpacked package at a local XCFramework")
    p.add_argument("--state", required=True, help="the swift.json `swift` wrote")
    p.add_argument("--package", required=True)
    p.add_argument("--framework", required=True)
    p = sub.add_parser("go", help="write the Go module as a GOPROXY serves it")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p.add_argument("--libs", required=True, help="a directory of every Go target's library, as build names them")
    p.add_argument("--out", required=True)
    p = sub.add_parser("consume-go", help="run a check program, or an import, on the unpacked Go module")
    p.add_argument("--proxy", required=True, help="the directory `go` wrote")
    p.add_argument("--check")
    p.add_argument("--native")
    p = sub.add_parser("consume-python", help="run a check program, or an import, on the installed wheel")
    group = p.add_mutually_exclusive_group(required=True)
    group.add_argument("--check")
    group.add_argument("--import", dest="module")
    p.add_argument("--native")
    p = sub.add_parser("compare", help="compare a consumer's output with native Rust's")
    p.add_argument("native")
    p.add_argument("actual")
    p = sub.add_parser("init", help="write the release workflow into the crate's repository")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--swift-mirror", required=True, metavar="OWNER/REPO",
                   help="the repository SwiftPM resolves the package from, whose root holds its Package.swift")
    p.add_argument("--go-module", required=True, metavar="PATH",
                   help="the Go module's path, which a proxy you run serves, /vN suffix included")
    p.add_argument("--rust2", help="the rust2 commit to pin; this checkout's by default")
    args = parser.parse_args()
    # winapi links MinGW import libraries of its own, from cargo's registry, where nothing
    # linking the Go module's static library finds them: the system's serve every check and
    # build, so the export's #cgo lines and rustc agree on them.
    os.environ["WINAPI_NO_BUNDLED_LIBRARIES"] = "1"

    if args.command == "facts":
        info = facts(args.manifest_path)
        lines = [f"{key}={info[key]}" for key in ("name", "version", "lib")]
        lines += [f"module={pascal(info['lib'])}", f"tag={info['name']}-v{info['version']}",
                  f"examples={json.dumps(info['examples'])}"]
        print("\n".join(lines))
    elif args.command == "api":
        stage_api(args)
    elif args.command == "export":
        export(args)
    elif args.command == "build":
        build(args)
    elif args.command == "same":
        same(args.first, args.second)
    elif args.command == "digest":
        print(digest(args.export, facts(args.manifest_path)["lib"]))
    elif args.command == "swift":
        info = facts(args.manifest_path)
        pairs = lambda items: dict(item.split("=", 1) for item in items)  # noqa: E731
        swift(info, args.export, pairs(args.lib), pairs(args.native), args.out)
        release_manifest(args.out, args.url)
    elif args.command == "swift-local":
        local_manifest(args.state, args.package, args.framework)
    elif args.command == "go":
        print(go_module(facts(args.manifest_path), args.export, args.libs, args.out))
    elif args.command == "consume-go":
        consume_go(args.proxy, args.check, args.native)
    elif args.command == "consume-python":
        consume_python(args.check, args.native, args.module)
    elif args.command == "compare":
        compare(Path(args.native).read_bytes(), Path(args.actual).read_bytes(), "installed Swift package")
    elif args.command == "init":
        init(args)


if __name__ == "__main__":
    main()
