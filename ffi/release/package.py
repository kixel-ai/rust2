"""Build and check the Python wheels, Swift package, C archives, Go module, Kotlin jar and Android library, Dart package, web package, and React Native package of a crate's rust2-ffi export.

bindings.yml runs each step on the host it needs; release.yml is the caller `init` writes.
Every fact about the package comes from Cargo metadata or the export, so nothing here
names a crate: what Cargo cannot say, such as who may publish, stays in the caller.
"""

import argparse
import base64
import contextlib
import csv
import difflib
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import posixpath
import plistlib
import queue
import re
import shlex
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
import tomllib
import urllib.parse
import uuid
import zipfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
# The sdist's PEP 517 backend, which writes both distributions' wheels.
sys.path.insert(0, str(HERE))
import backend  # noqa: E402
from backend import seal, write_zip  # noqa: E402,F401
# The wheel's Linux floor: glibc 2.28 (RHEL 8, Debian 10), matching moq's manylinux_2_28.
GLIBC = "2.28"
# The macOS a wheel loads on: the oldest Rust's aarch64-apple-darwin target supports.
MACOS_WHEEL = "11.0"
# The Swift package's floors: what the generated Swift's `Swift.Duration` needs. Mac
# Catalyst counts in iOS versions, as its objects declare them.
SWIFT_PLATFORMS = {"macos": "13.0", "ios": "16.0", "maccatalyst": "16.0"}
# Each platform's name in a SwiftPM manifest.
SWIFTPM_PLATFORMS = {"macos": "macOS", "ios": "iOS", "maccatalyst": "macCatalyst"}
# Each Apple slice the Swift package ships, and the platform its minimum OS is checked on.
APPLE_TARGETS = {
    "aarch64-apple-darwin": "macos",
    "aarch64-apple-ios": "ios",
    "aarch64-apple-ios-sim": "ios",
    "aarch64-apple-ios-macabi": "maccatalyst",
}
# The OS each Apple slice is built for: the Swift package's floors, but iOS's device and
# simulator at React Native's, `min_ios_version_supported`, as its package takes them too.
APPLE_BUILD = {**{target: SWIFT_PLATFORMS[platform] for target, platform in APPLE_TARGETS.items()},
               "aarch64-apple-ios": "15.1", "aarch64-apple-ios-sim": "15.1"}
# Each slice's SDK and clang target, at the floor, for the link that finds what it calls.
APPLE_LINKS = {
    "aarch64-apple-darwin": ("macosx", "arm64-apple-macos{}"),
    "aarch64-apple-ios": ("iphoneos", "arm64-apple-ios{}"),
    "aarch64-apple-ios-sim": ("iphonesimulator", "arm64-apple-ios{}-simulator"),
    "aarch64-apple-ios-macabi": ("macosx", "arm64-apple-ios{}-macabi"),
}
# Apple's required-reason APIs a symbol names, by the privacy manifest category declaring them
# (https://developer.apple.com/documentation/bundleresources/describing-use-of-required-reason-api).
# `getattrlist` and its kin are in two categories, as what they read depends on their arguments.
# Foundation's attribute keys are constants a caller imports, so the scan sees them too.
REQUIRED_REASON_APIS = {
    **dict.fromkeys(["stat", "fstat", "fstatat", "lstat", "NSFileCreationDate", "NSFileModificationDate",
                     "NSURLContentModificationDateKey", "NSURLCreationDateKey"], "FileTimestamp"),
    **dict.fromkeys(["getattrlist", "getattrlistbulk", "fgetattrlist", "getattrlistat"], "FileTimestamp or DiskSpace"),
    **dict.fromkeys(["statfs", "statvfs", "fstatfs", "fstatvfs", "NSFileSystemFreeSize", "NSFileSystemSize",
                     "NSURLVolumeAvailableCapacityKey", "NSURLVolumeAvailableCapacityForImportantUsageKey",
                     "NSURLVolumeAvailableCapacityForOpportunisticUsageKey", "NSURLVolumeTotalCapacityKey"],
                    "DiskSpace"),
    "mach_absolute_time": "SystemBootTime",
    "OBJC_CLASS_$_NSUserDefaults": "UserDefaults",
    "OBJC_CLASS_$_UITextInputMode": "ActiveKeyboards",
}
# The reasons a library's own use of a category declares, where they hold for every library:
# a sandboxed app, as the App Store requires, stats files in its containers (C617.1) or ones
# the user granted it (3B52.1), and a clock read measures time within the app. A sandboxed
# macOS app may also read a world-readable system path, such as /etc/localtime, which no
# reason Apple defines covers, so no manifest can declare it; these two stand for the rest.
# Another category's reason only the crate's author knows, so its call fails the build.
REQUIRED_REASONS = {
    "FileTimestamp": ["C617.1", "3B52.1"],
    "SystemBootTime": ["35F9.1"],
}
# Every target a wheel and a C archive are built for, each of 8-byte pointers, as the CMake version file checks.
WHEEL_TARGETS = [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
]
# The targets no prebuilt wheel covers on which the sdist must build one. The generated crate
# builds for no target it was not exported for, so the export merges their APIs too.
SDIST_TARGETS = [
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "x86_64-apple-darwin",
    "aarch64-pc-windows-msvc",
]
# Every target the Go module links a static library of, by its GOOS_GOARCH. Go links
# Windows through MinGW, so a -gnu library.
GO_TARGETS = {
    "x86_64-unknown-linux-gnu": "linux_amd64",
    "aarch64-unknown-linux-gnu": "linux_arm64",
    "x86_64-apple-darwin": "darwin_amd64",
    "aarch64-apple-darwin": "darwin_arm64",
    "aarch64-apple-ios": "ios_arm64",
    "x86_64-pc-windows-gnu": "windows_amd64",
    "x86_64-linux-android": "android_amd64",
    "aarch64-linux-android": "android_arm64",
}
# Every target the Dart package ships a library for, by the oldest system it loads on, which
# its hook checks an app's against: Flutter's own floors, Android API 24, iOS 13, and macOS
# 10.15. Every arm64 Mac runs macOS 11, Rust's floor there, and Linux's and Windows' floors
# are the wheels'.
DART_TARGETS = {
    "x86_64-unknown-linux-gnu": None,
    "aarch64-unknown-linux-gnu": None,
    "aarch64-apple-darwin": None,
    "x86_64-apple-darwin": "10.15",
    "aarch64-apple-ios": "13.0",
    "aarch64-apple-ios-sim": "13.0",
    "x86_64-pc-windows-msvc": None,
    "aarch64-linux-android": "24",
    "armv7-linux-androideabi": "24",
    "x86_64-linux-android": "24",
}
# Every Android ABI the AAR holds a library of, by the target built for it.
ANDROID_TARGETS = {
    "aarch64-linux-android": "arm64-v8a",
    "armv7-linux-androideabi": "armeabi-v7a",
    "x86_64-linux-android": "x86_64",
}
# Every target the desktop jar bundles a library for, which its Kotlin loads on that host:
# the wheels', as moq's jar ships.
JVM_TARGETS = WHEEL_TARGETS
# Every Android ABI React Native's package holds a static library of, by the target built for
# it: its template's `reactNativeArchitectures`, each of which an app builds by default.
RN_ANDROID_TARGETS = {
    "aarch64-linux-android": "arm64-v8a",
    "armv7-linux-androideabi": "armeabi-v7a",
    "i686-linux-android": "x86",
    "x86_64-linux-android": "x86_64",
}
# React Native's iOS slices, the device's and the simulator's: the Swift package's, at React
# Native's floor.
RN_IOS_TARGETS = ("aarch64-apple-ios", "aarch64-apple-ios-sim")
# The simulator React Native's consumer runs on, as macOS runners are arm64.
IOS_SIMULATOR = "aarch64-apple-ios-sim"
# The React Native release the package's consumer app is made with: the generator's floor.
REACT_NATIVE = "0.88.1"
# The community CLI whose `init` makes that app, and the esbuild that bundles its check.
RN_CLI = "21.0.0"
ESBUILD = "0.28.2"
# The ABI the AAR's consumer runs on, in the emulator GitHub's x64 runners run at speed.
ANDROID_EMULATOR = "x86_64-linux-android"
# The oldest Android the AAR loads on: moq's floor, which the exporter's nightly emulator runs.
ANDROID_API = 24
# The SDK the classes of the AAR and the jar compile against: 34 or later, whose android.jar
# holds the @FastNative the generated Kotlin names, so the build leaves FastNative.kt out.
ANDROID_COMPILE_SDK = 35
# The Kotlin the classes are written for: the oldest kotlinc 2.4 does not deprecate,
# whose metadata a consumer's Kotlin 2.1 or later reads, and the standard library they need.
KOTLIN_API = "2.2"
# The bytecode the classes hold, which every D8 reads and no consumer's JVM target is below.
JVM_TARGET = "1.8"
# What a Maven groupId may be: dotted segments, as Central's namespaces are.
MAVEN_GROUP = re.compile(r"[A-Za-z0-9_-]+(\.[A-Za-z0-9_-]+)+")
# The words Java reserves, which no segment of a package name may be.
JAVA_RESERVED = frozenset("""abstract assert boolean break byte case catch char class const continue default do double
else enum extends false final finally float for goto if implements import instanceof int interface long native new
null package private protected public return short static strictfp super switch synchronized this throw throws
transient true try void volatile while _""".split())
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
ZIP_EPOCH = backend.ZIP_EPOCH
# The fixed directory every build runs cargo through, linked to the checkout's (`linked`), so
# cargo hashes a path package outside the generated crate's workspace, the facade, by one path,
# and dlltool names a Windows GNU library's import symbols after one target directory, from any checkout.
LINK = Path("C:/rust2-ffi") if sys.platform == "win32" else Path("/tmp/rust2-ffi")
# The target directory every build of a target shares unless one of its settings keeps it
# apart: the wheels', the C, Dart, and JVM libraries', and the Go module's and React Native's
# static libraries. Cargo reuses each dependency built with the same features, profile, and
# flags, and relinks each generated crate after the previous build has taken its library.
# The Swift and Android libraries' line tables change the profile, which shares nothing, and
# an Apple build at a deployment target of its own builds apart, since cargo does not rebuild
# for a changed one.
SHARED_DIR = "target-python"
# The target the web binding serves, whose module the npm and JSR packages hold.
WEB_TARGET = "wasm32-wasip1"
# JSR's cap on a version's files, unpacked.
JSR_LIMIT = 20 * 1024 * 1024
# The license file names JSR reads, in any case.
JSR_LICENSES = {"license", "license.md", "license.txt", "licence", "licence.md", "licence.txt"}
# The bundlers the web package's consumers build with, as an app would take them from npm.
BUNDLERS = {"vite": "8.3.8", "webpack": "5.112.2", "webpack-cli": "7.3.1"}
# Runs a WASI command module on Node: native Rust's output on the web binding's target.
WASI_RUN = """import { WASI } from "node:wasi";
import { readFileSync } from "node:fs";
const wasi = new WASI({ version: "preview1", args: [], env: {}, returnOnExit: false });
const module = await WebAssembly.compile(readFileSync(process.argv[1]));
wasi.start(await WebAssembly.instantiate(module, wasi.getImportObject()));
"""
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
        "authors": package.get("authors") or [],
        "readme": package.get("readme"),
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


def python_floor(export, abi="abi3"):
    """The oldest CPython the export's extension on the stable ABI `abi` loads in, from its glue's pyo3 feature."""
    return backend.python_floor((Path(export) / "Cargo.toml").read_text(encoding="utf-8"), abi)


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


library_name = backend.library_name


def write_wheel(info, export, library, target, out, abi):
    """An unrepaired wheel holding the generated module as a typed package beside its library on the stable ABI `abi`."""
    # Every wheel's metadata is the sdist's, as a resolver reads one wheel's for all; the tag
    # alone narrows where an `abi3t` wheel installs.
    lib, tag = info["lib"], f"cp3{python_floor(export, abi)}-{abi}-{wheel_platform(target)}"
    return backend.write_wheel(out, metadata(info, python_floor(export)), tag, lib,
                               (Path(export) / "python" / f"{lib}.py").read_bytes(), Path(library).read_bytes(),
                               target, {path.name: path.read_bytes() for path in licenses(info)}, info["epoch"],
                               info["sbom"])


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


def vendored(wheel):
    """The shared libraries a repair tool copied into `wheel` beside its own, which each puts in a
    `.libs` or `.dylibs` directory."""
    with zipfile.ZipFile(wheel) as archive:
        return sorted(Path(n).name for n in archive.namelist()
                      if not n.endswith("/") and any(d.endswith((".libs", ".dylibs")) for d in Path(n).parts[:-1]))


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


def toml_key(key):
    """A TOML key, bare where TOML allows, so `relocate` finds each `path = ...` it writes."""
    return key if re.fullmatch(r"[A-Za-z0-9_-]+", key) else json.dumps(key, ensure_ascii=False)


def toml_value(value):
    """A TOML value as one line, which `workspace_manifest` reads back to check."""
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, list):
        return "[" + ", ".join(map(toml_value, value)) + "]"
    if isinstance(value, dict):
        return "{ " + ", ".join(f"{toml_key(k)} = {toml_value(v)}" for k, v in value.items()) + " }"
    if isinstance(value, (int, float)):
        return repr(value)
    return value.isoformat()


def workspace_manifest(path):
    """The workspace root at `path` as its members read it: its `[workspace]` table alone,
    since a root that is a package fails to load without that package's sources."""
    root = tomllib.loads(Path(path).read_text(encoding="utf-8"))
    kept = {key: root[key] for key in ("cargo-features", "workspace") if key in root}
    lines = [f"cargo-features = {toml_value(kept['cargo-features'])}"] if "cargo-features" in kept else []
    lines.append("[workspace]")
    lines += [f"{toml_key(k)} = {toml_value(v)}" for k, v in kept["workspace"].items()]
    text = "# The workspace root's [workspace] table, which its members inherit from.\n" + "\n".join(lines) + "\n"
    if tomllib.loads(text) != kept:
        raise SystemExit(f"{path}'s [workspace] table does not survive rewriting")
    return text


def manifest_paths(value):
    """Every `path` a parsed manifest names, in any table. A dependency may itself be named
    `path`, so a `path` key holding a table is searched like any other table."""
    if isinstance(value, dict):
        return [p for k, v in value.items() for p in ([v] if k == "path" and isinstance(v, str) else manifest_paths(v))]
    return [p for v in value for p in manifest_paths(v)] if isinstance(value, list) else []


def relocate(manifest, moves, strict=False):
    """A manifest with each local package's absolute path moved, as `moves` maps them, checked
    by reading it back: nothing else changed, and when `strict`, every `path` is one moved."""
    # One pass, so a path already moved is never matched again by another package's key.
    spelled = {f"path = {json.dumps(old, ensure_ascii=False)}": f"path = {json.dumps(new)}" for old, new in moves.items()}
    text = manifest
    if spelled:
        text = re.sub("|".join(map(re.escape, spelled)), lambda m: spelled[m.group(0)], manifest)

    def moved(value):
        if isinstance(value, dict):
            return {k: moves.get(v, v) if k == "path" and isinstance(v, str) else moved(v) for k, v in value.items()}
        return [moved(v) for v in value] if isinstance(value, list) else value

    want = moved(tomllib.loads(manifest))
    if strict and (stray := [p for p in manifest_paths(want) if p not in moves.values()]):
        raise SystemExit(f"the generated manifest names {stray}, where no package it builds lies")
    if tomllib.loads(text) != want:
        raise SystemExit(f"a manifest's paths do not survive moving them to {sorted(moves.values())}")
    return text


def toolchain(start):
    """The sdist's rust-toolchain.toml: the channel rustup resolves from the crate's directory,
    without the components and targets its developers add, or None for no file."""
    for directory in [Path(start), *Path(start).parents]:
        for name in ("rust-toolchain", "rust-toolchain.toml"):
            path = directory / name
            if not path.is_file():
                continue
            text = path.read_text(encoding="utf-8")
            try:
                channel = tomllib.loads(text).get("toolchain", {}).get("channel")
            except tomllib.TOMLDecodeError:
                # The legacy file's one line.
                channel = text.strip()
            if not channel:
                raise SystemExit(f"{path} names no channel, which the sdist could pin")
            return f'[toolchain]\nchannel = {json.dumps(channel)}\nprofile = "minimal"\n'
    return None


def pyproject(targets):
    """The sdist's pyproject.toml: the backend beside it, and the targets its crate builds for."""
    listed = "".join(f"    {json.dumps(t)},\n" for t in targets)
    return ("# A rust2-ffi binding's sources, which the backend in rust2/ builds into a wheel with cargo.\n"
            "[build-system]\nrequires = []\nbuild-backend = \"backend\"\nbackend-path = [\"rust2\"]\n\n"
            "[tool.rust2-ffi]\n# The targets the binding was exported for, the only ones its crate builds for.\n"
            f"targets = [\n{listed}]\n")


def sdist(info, manifest, export, out):
    """The sdist: the export's crate in binding/, every local package it builds with in sources/,
    laid out as they lie relative to each other, with their workspace roots, and the backend."""
    export = Path(export).resolve()
    targets = json.loads((export / "api.json").read_text(encoding="utf-8"))["api"]["targets"]
    if missing := [t for t in SDIST_TARGETS if t not in targets]:
        raise SystemExit(f"the export lacks {missing}, on which the sdist must build: stage their APIs")
    local, roots = local_packages(manifest, export, "--features", "python")
    base = Path(os.path.commonpath([*local, *(r.parent for r in roots)]))
    def place(directory):
        """Where a directory lies in the sdist."""
        return posixpath.normpath(posixpath.join("sources", Path(directory).relative_to(base).as_posix()))

    placed = {d: place(d) for d in local}

    def moves(at):
        """Each local package's path from the sdist's directory `at`, by its absolute one."""
        return {str(d): posixpath.relpath(there, at) for d, there in placed.items()}

    files = {}
    for directory, at in placed.items():
        # What cargo would publish of it, with the original manifest; what cargo generates in
        # a package, such as Cargo.toml.orig, is on no disk.
        listed = output("cargo", "package", "--list", "--allow-dirty", "--manifest-path", directory / "Cargo.toml",
                        cwd=directory)
        for name in listed.splitlines():
            path = directory / name
            if path.is_file():
                files[f"{at}/{name}"] = (path.read_bytes(), os.access(path, os.X_OK))
        # Relative paths hold, as the packages keep their places; absolute ones move with them.
        manifest = relocate((directory / "Cargo.toml").read_text(encoding="utf-8"), moves(at))
        files[f"{at}/Cargo.toml"] = (manifest.encode(), False)
    for root in roots:
        at = place(root.parent)
        files[f"{at}/Cargo.toml"] = (relocate(workspace_manifest(root), moves(at)).encode(), False)
    # The generated manifest names each package relative to the export, where a relative path
    # reads right, and absolute elsewhere.
    binding = moves("binding")
    for d in placed:
        try:
            binding[Path(os.path.relpath(d, export)).as_posix()] = binding[str(d)]
        except ValueError:
            # On another Windows drive than the export, no relative path reaches it; only the
            # absolute one names it.
            pass
    generated = relocate((export / "Cargo.toml").read_text(encoding="utf-8"), binding, strict=True)
    files["binding/Cargo.toml"] = (generated.encode(), False)
    for path in [export / "Cargo.lock", export / "build.rs", export / "python" / f"{info['lib']}.py",
                 *sorted(p for p in (export / "src").rglob("*") if p.is_file())]:
        files[f"binding/{path.relative_to(export).as_posix()}"] = (path.read_bytes(), False)
    floor = python_floor(export)
    files["PKG-INFO"] = (metadata(info, floor).encode(), False)
    files["pyproject.toml"] = (pyproject(targets).encode(), False)
    files["rust2/backend.py"] = ((HERE / "backend.py").read_bytes(), False)
    # What the prebuilt wheels embed, which a wheel built from the sdist embeds too: the same lockfile's crates.
    files["rust2/sbom.cdx.json"] = (sbom(info, export / "Cargo.toml"), False)
    for path in licenses(info):
        files[path.name] = (path.read_bytes(), False)
    if pin := toolchain(info["dir"]):
        files["rust-toolchain.toml"] = (pin.encode(), False)
    (Path(out) / "sdist").mkdir(parents=True, exist_ok=True)
    return backend.write_sdist(Path(out) / "sdist", info["name"], pep440(info["version"]), files, info["epoch"])


def digest(export, lib):
    """A hash of the generated bindings, which every build host's export must agree on."""
    hasher = hashlib.sha256()
    export = Path(export)
    # In code point order on every host: Windows compares its paths ignoring case, which
    # would put `FastNative.kt` after `api.kt` there alone.
    def files(part):
        return sorted((p for p in (export / part).rglob("*") if p.is_file()), key=lambda p: p.relative_to(export).as_posix())
    paths = [export / "python" / f"{lib}.py", *files("include"), *files("swift"), *files("go"), *files("kotlin"),
             *files("dart"), *files("csharp"), *files("react-native")]
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
    out = []
    for setting, platforms in seen.items():
        if platforms == set(flags_by_platform):
            out.append(setting + ")")
        else:
            out.append(setting + ", .when(platforms: [" + ", ".join("." + SWIFTPM_PLATFORMS[p] for p in sorted(platforms)) + "]))")
    return out


def package_swift(module, version, settings, binary):
    """The package manifest: the Swift source over the XCFramework, either by URL or by path."""
    platforms = ", ".join(f'.{SWIFTPM_PLATFORMS[p]}("{v}")' for p, v in SWIFT_PLATFORMS.items())
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
        .target(name: "{module}Privacy", resources: [.copy("PrivacyInfo.xcprivacy")]),
        .target(
            name: "{module}",
            dependencies: ["{module}FFI", "{module}Privacy", .product(name: "BasicContainers", package: "swift-collections")],
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


def api_name(symbol):
    """The C function or Objective-C class a Mach-O symbol names, without its `$INODE64`-style variant."""
    name = symbol.removeprefix("_")
    return name if name.startswith("OBJC_CLASS_$_") else name.split("$")[0]


def required_apis(binary):
    """Each required-reason API a linked Mach-O binary imports, by its category."""
    found = {}
    for symbol in output("nm", "--no-llvm-bc", "-u", "-j", binary).split():
        category = REQUIRED_REASON_APIS.get(api_name(symbol))
        if category:
            found.setdefault(category, set()).add(api_name(symbol))
    return found


def privacy_manifest(lib, found):
    """The PrivacyInfo.xcprivacy declaring the APIs found, or the build's failure for one no reason fits every library."""
    unknown = {category: apis for category, apis in found.items() if category not in REQUIRED_REASONS}
    if unknown:
        calls = "; ".join(f"{', '.join(sorted(apis))} ({category})" for category, apis in sorted(unknown.items()))
        raise SystemExit(f"{lib} calls Apple's required-reason APIs whose reason only its author knows: {calls}\n"
                         "help: the App Store refuses an app calling them undeclared; remove the call, or the "
                         "dependency making it, from what the C ABI reaches")
    types = [{"NSPrivacyAccessedAPIType": f"NSPrivacyAccessedAPICategory{category}",
              "NSPrivacyAccessedAPITypeReasons": REQUIRED_REASONS[category]} for category in sorted(found)]
    return plistlib.dumps({"NSPrivacyAccessedAPITypes": types}, sort_keys=True)


def probe_link(lib, static, target, native, work, floor=None):
    """`static` linked as an app at `floor`, by default the Swift package's, links it: a dylib
    keeping only what the C ABI reaches, with its native libraries."""
    sdk, clang_target = APPLE_LINKS[target]
    names = work / "exported.txt"
    names.write_text(f"_{lib}_*\n", encoding="utf-8")
    probe = work / f"{target}.dylib"
    floor = floor or SWIFT_PLATFORMS[APPLE_TARGETS[target]]
    run("xcrun", "--sdk", sdk, "clang", "-target", clang_target.format(floor),
        "-dynamiclib", "-Wl,-dead_strip", f"-Wl,-exported_symbols_list,{names}", f"-Wl,-force_load,{static}",
        *shlex.split(native), "-o", probe)
    return probe


def check_symbolicated(lib, probe):
    """Fail unless a dSYM made from a link of the library, as an app's is, maps the C ABI to Rust source lines."""
    dsym = probe.with_suffix(".dSYM")
    run("dsymutil", probe, "-o", dsym)
    symbols = [line.split() for line in output("nm", "--no-llvm-bc", "-gU", probe).splitlines()]
    address, name = next((s[0], s[2]) for s in symbols if len(s) == 3 and s[2].startswith(f"_{lib}_"))
    where = output("atos", "-o", dsym / "Contents/Resources/DWARF" / probe.name, "-arch", "arm64", f"0x{address}")
    if not re.search(r"\.rs:[0-9]+\)", where):
        raise SystemExit(f"{probe.name}'s dSYM names no Rust source line for {name}: {where.strip()}")


def privacy_check(manifest, binary):
    """Fail unless `manifest` declares every required-reason category `binary` imports."""
    with open(manifest, "rb") as f:
        declared = {t["NSPrivacyAccessedAPIType"].removeprefix("NSPrivacyAccessedAPICategory")
                    for t in plistlib.load(f)["NSPrivacyAccessedAPITypes"]}
    missing = {c: sorted(apis) for c, apis in required_apis(binary).items() if c not in declared}
    if missing:
        raise SystemExit(f"{binary} imports required-reason APIs {manifest} leaves undeclared: {missing}")
    print(f"{binary} imports only what {manifest} declares: {sorted(declared)}")


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
    # What each slice calls once an app's link drops what the C ABI never reaches, as Apple's
    # upload scan sees it, and the line tables the app's dSYM carries for its crash reports.
    found = {}
    with tempfile.TemporaryDirectory() as work:
        for target, path in libs.items():
            slice_dir = Path(work) / target
            slice_dir.mkdir()
            probe = probe_link(lib, path, target, Path(natives[target]).read_text(encoding="utf-8").strip(), slice_dir)
            imports = required_apis(probe)
            print(f"{target} imports required-reason APIs: {({c: sorted(a) for c, a in imports.items()})}", file=sys.stderr)
            for category, apis in imports.items():
                found.setdefault(category, set()).update(apis)
            check_symbolicated(lib, probe)
    privacy = privacy_manifest(lib, found)
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
    print(f"{archive.name}: {archive.stat().st_size} bytes", file=sys.stderr)
    package = out / "package"
    sources = package / "Sources" / module
    sources.mkdir(parents=True)
    shutil.copy(export / "swift" / f"{lib}.swift", sources)
    # A static library holds no resources, so SwiftPM bundles the manifest into the app from a
    # target of its own, whose generated `Bundle.module` accessor no exported name can shadow.
    privacy_target = package / "Sources" / f"{module}Privacy"
    privacy_target.mkdir()
    (privacy_target / "PrivacyInfo.xcprivacy").write_bytes(privacy)
    (privacy_target / "Privacy.swift").write_text(f"// Bundles {module}'s privacy manifest into the app.\n", encoding="utf-8")
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


# The CPS revision CMake 4.4 writes, the oldest CMake that both reads and writes it.
CPS_VERSION = "0.14.1"


def compat_version(version):
    """The oldest version `version` serves a build-time request for, by Cargo's compatibility.

    Cargo calls 1.2.3 compatible with 1.x, 0.4.2 with 0.4.x, and 0.0.3 with itself; a
    prerelease promises nothing, so it serves only itself.
    """
    match = re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?", version)
    if match is None:
        raise SystemExit(f"version {version} is not semver")
    major, minor, patch, pre, _ = match.groups()
    if pre:
        return f"{major}.{minor}.{patch}{pre}"
    if major != "0":
        return f"{major}.0.0"
    if minor != "0":
        return f"0.{minor}.0"
    return f"0.0.{patch}"


def soname_version(version):
    """The version a shared library's name holds: all of it, as each release is its own ABI.

    Every binding checks at load that the library's contract, a hash of everything its header
    declares, is its header's, so even an addition makes two releases' libraries differ.
    """
    compat_version(version)
    return version.split("+")[0]


def c_files(lib, target, version):
    """The archive's library paths: static, shared, the shared library's link name, and its import library."""
    if "windows" in target:
        return {"static": f"lib/{lib}.lib", "shared": f"bin/{lib}.dll", "link": None, "implib": f"lib/{lib}.dll.lib"}
    if "apple" in target:
        return {"static": f"lib/lib{lib}.a", "shared": f"lib/lib{lib}.{soname_version(version)}.dylib", "link": f"lib/lib{lib}.dylib",
                "implib": None}
    return {"static": f"lib/lib{lib}.a", "shared": f"lib/lib{lib}.so.{soname_version(version)}", "link": f"lib/lib{lib}.so", "implib": None}


def native_libs(flags, target):
    """rustc's native-static-libs as (kind, name) pairs: a `lib`, an Apple `framework`, or an MSVC linker `flag`."""
    out = []
    args = iter(shlex.split(flags))
    for flag in args:
        if "windows" in target:
            if flag.endswith(".lib"):
                out.append(("lib", flag.removesuffix(".lib")))
            elif re.fullmatch(r"/defaultlib:[A-Za-z0-9_]+", flag):
                out.append(("flag", flag))
            else:
                raise SystemExit(f"unsupported native linker flag on {target}: {flag}")
        elif flag.startswith("-l") and len(flag) > 2:
            out.append(("lib", flag[2:]))
        elif flag == "-framework" and "apple" in target:
            out.append(("framework", next(args)))
        else:
            raise SystemExit(f"unsupported native linker flag on {target}: {flag}")
    return out


def pkg_config(info, natives):
    """`<lib>.pc`, relative to itself: the shared library by default, and what `--static` adds."""
    private = " ".join(f"-l{name}" if kind == "lib" else f"-framework {name}" for kind, name in natives)
    description = " ".join((info["description"] or f"The C and C++ binding of {info['name']}").split())
    lines = [
        "prefix=${pcfiledir}/../..",
        "includedir=${prefix}/include",
        "libdir=${prefix}/lib",
        "",
        f"Name: {info['name']}",
        f"Description: {description}",
        f"Version: {info['version']}",
    ]
    url = info["homepage"] or info["repository"]
    if url:
        lines.append(f"URL: {url}")
    lines += [f"Libs: -L${{libdir}} -l{info['lib']}", f"Libs.private: {private}", "Cflags: -I${includedir}"]
    return "\n".join(lines) + "\n"


def cmake_quote(value):
    """A CMake bracket argument, which takes any text but its own closing bracket."""
    if "]==]" in value:
        raise SystemExit(f"{value!r} has no CMake spelling here")
    return f"[==[{value}]==]"


def cmake_config(info, target, files, natives):
    """`<lib>-config.cmake`: `<lib>::<lib>` the static library, `<lib>::<lib>_shared` the shared one, each at C11 and C++17 or newer."""
    lib = info["lib"]
    libs = [name if kind == "lib" else f"-framework {name}" for kind, name in natives if kind != "flag"]
    options = [name for kind, name in natives if kind == "flag"]
    prefix = f"${{_{lib}_prefix}}"
    shared = [f'IMPORTED_LOCATION "{prefix}/{files["shared"]}"']
    if files["implib"]:
        shared.append(f'IMPORTED_IMPLIB "{prefix}/{files["implib"]}"')
    shared_props = "\n        ".join(shared)
    return f"""# find_package({lib}): {info['name']} {info['version']}'s C and C++ binding for {target}.
# {lib}::{lib} links the static library and what it needs of the system; {lib}::{lib}_shared links the
# shared library. Generated by rust2-ffi's release builder.

get_filename_component(_{lib}_prefix "${{CMAKE_CURRENT_LIST_DIR}}/../../.." ABSOLUTE)

if(NOT TARGET {lib}::{lib})
    add_library({lib}::{lib} STATIC IMPORTED)
    set_target_properties({lib}::{lib} PROPERTIES
        IMPORTED_LOCATION "{prefix}/{files['static']}"
        IMPORTED_LINK_INTERFACE_LANGUAGES C
        INTERFACE_INCLUDE_DIRECTORIES "{prefix}/include"
        INTERFACE_COMPILE_FEATURES "c_std_11;cxx_std_17"
        INTERFACE_LINK_LIBRARIES {cmake_quote(';'.join(libs))}
        INTERFACE_LINK_OPTIONS {cmake_quote(';'.join(options))}
    )
endif()

if(NOT TARGET {lib}::{lib}_shared)
    add_library({lib}::{lib}_shared SHARED IMPORTED)
    set_target_properties({lib}::{lib}_shared PROPERTIES
        {shared_props}
        INTERFACE_INCLUDE_DIRECTORIES "{prefix}/include"
        INTERFACE_COMPILE_FEATURES "c_std_11;cxx_std_17"
    )
endif()

unset(_{lib}_prefix)
"""


def cmake_version(info):
    """`<lib>-config-version.cmake`: Cargo's compatibility, a range's upper bound, and 64-bit consumers alone."""
    compat = compat_version(info["version"])
    if "-" in info["version"].split("+")[0]:
        # CMake's versions are numbers alone, so no request can name a prerelease, and none
        # may take one for the release it precedes.
        return f"""# The version check of find_package({info['lib']}): {info['version']} is a prerelease, which only a
# request naming no version finds. Generated by rust2-ffi's release builder.

set(PACKAGE_VERSION "{info['version']}")
set(PACKAGE_VERSION_COMPATIBLE FALSE)
"""
    return f"""# The version check of find_package({info['lib']}): a request from {compat} up to this version is met,
# as Cargo's compatibility has it, by a 64-bit consumer. Generated by rust2-ffi's release builder.

set(PACKAGE_VERSION "{info['version']}")

if(CMAKE_SIZEOF_VOID_P AND NOT CMAKE_SIZEOF_VOID_P STREQUAL "8")
    set(PACKAGE_VERSION "{info['version']} (64-bit)")
    set(PACKAGE_VERSION_UNSUITABLE TRUE)
    return()
endif()

set(PACKAGE_VERSION_COMPATIBLE FALSE)
if(NOT PACKAGE_FIND_VERSION VERSION_LESS "{compat}" AND NOT PACKAGE_FIND_VERSION VERSION_GREATER PACKAGE_VERSION)
    set(PACKAGE_VERSION_COMPATIBLE TRUE)
endif()
if(PACKAGE_VERSION_COMPATIBLE AND PACKAGE_FIND_VERSION_RANGE)
    if(PACKAGE_FIND_VERSION_RANGE_MAX STREQUAL "INCLUDE")
        if(PACKAGE_VERSION VERSION_GREATER PACKAGE_FIND_VERSION_MAX)
            set(PACKAGE_VERSION_COMPATIBLE FALSE)
        endif()
    elseif(PACKAGE_VERSION VERSION_GREATER_EQUAL PACKAGE_FIND_VERSION_MAX)
        set(PACKAGE_VERSION_COMPATIBLE FALSE)
    endif()
endif()
if(PACKAGE_FIND_VERSION VERSION_EQUAL PACKAGE_VERSION)
    set(PACKAGE_VERSION_EXACT TRUE)
endif()
"""


def cps(info, files, natives):
    """`<lib>.cps`, the Common Package Specification's description: the same two components."""
    lib = info["lib"]
    compat = compat_version(info["version"])
    static = {
        "type": "archive",
        "location": f"@prefix@/{files['static']}",
        "includes": ["@prefix@/include"],
        "compile_features": ["c11", "c++17"],
        "link_languages": ["c"],
    }
    libs = [name if kind == "lib" else f"-framework {name}" for kind, name in natives if kind != "flag"]
    flags = [name for kind, name in natives if kind == "flag"]
    if libs:
        static["link_libraries"] = libs
    if flags:
        static["link_flags"] = flags
    shared = {"type": "dylib", "location": f"@prefix@/{files['shared']}", "includes": ["@prefix@/include"],
              "compile_features": ["c11", "c++17"]}
    if files["implib"]:
        shared["link_location"] = f"@prefix@/{files['implib']}"
    description = {
        "cps_version": CPS_VERSION,
        "name": lib,
        "version": info["version"],
        "compat_version": compat,
        "cps_path": "@prefix@/lib/cps",
        # A simple version ignores a prerelease's suffix; a custom one matches only itself.
        "version_schema": "custom" if "-" in info["version"].split("+")[0] else "simple",
        # `<lib>_shared` is never `<lib>`, whatever the library is named.
        "components": {lib: static, f"{lib}_shared": shared},
        "default_components": [lib],
    }
    if info["description"]:
        description["description"] = " ".join(info["description"].split())
    if info["license"]:
        description["license"] = info["license"]
    if info["homepage"] or info["repository"]:
        description["website"] = info["homepage"] or info["repository"]
    return json.dumps(description, indent=2) + "\n"


def exported(shared, lib, target):
    """The C ABI's symbols, `<lib>_*`, that the shared library exports, as the object format spells them."""
    if "apple" in target:
        names = output("nm", "--no-llvm-bc", "-gUj", shared).split()
        want = f"_{lib}_"
    else:
        names = [line.split()[-1] for line in output("nm", "-D", "--defined-only", shared).splitlines() if line.strip()]
        want = f"{lib}_"
    found = sorted({name for name in names if name.startswith(want)})
    if not found:
        raise SystemExit(f"{shared} exports no {want}* symbol")
    return found


def undefined(path, kind):
    """The undefined symbols of an ELF object or archive that nm marks `kind`: `U` strong, `w` weak."""
    lines = output("nm", "--undefined-only", path).splitlines()
    return {line.split()[1] for line in lines if line.split()[:1] == [kind] and len(line.split()) == 2}


def commons_source(nm):
    """C defining each common symbol `nm -m` lists that no member defines, at its largest size and alignment."""
    commons, defined = {}, set()
    for line in nm.splitlines():
        common = re.fullmatch(r"([0-9a-f]+) \(common\) (?:\(alignment 2\^([0-9]+)\) )?external (\S+)", line.strip())
        if common:
            size, align = int(common.group(1), 16), int(common.group(2) or 0)
            old_size, old_align = commons.get(common.group(3), (0, 0))
            commons[common.group(3)] = (max(size, old_size), max(align, old_align))
        elif re.search(r"\) (?:weak )?(?:private )?external ", line) and "(undefined)" not in line:
            defined.add(line.split()[-1])
    return "".join(
        f'__attribute__((aligned({1 << align}))) char common{i}[{size}] __asm__("{name}");\n'
        for i, (name, (size, align)) in enumerate(sorted((n, c) for n, c in commons.items() if n not in defined))
    ) or "typedef int empty;\n"


def define_commons(static, arch, work):
    """An object defining each C tentative definition in a Mach-O archive that no member defines.

    Apple's clang emits `int x;` as a common symbol, such as aws-lc's `tty_in`, which a
    relocatable link leaves common, and global, past the exported-symbols list; Apple's
    current linker has no `-d` to define it. Defined here at its size and alignment, it is
    an ordinary symbol the list hides.
    """
    source = work / "commons.c"
    source.write_text(commons_source(output("nm", "--no-llvm-bc", "-m", static)), encoding="utf-8")
    obj = work / "commons.o"
    run("cc", "-c", "-fno-common", *arch, source, "-o", obj)
    return obj


def hide(static, shared, lib, target, work):
    """The static library as one prelinked object exporting only the C ABI, so its Rust runtime cannot clash.

    Another Rust static library in the same program brings its own std and compiler-builtins,
    whose unmangled symbols, such as `__addvdi3`, would otherwise collide. MSVC's linker has no
    relocatable link, so a Windows static library keeps every symbol.
    """
    keep = exported(shared, lib, target)
    names = work / "exported.txt"
    names.write_text("\n".join(keep) + "\n", encoding="utf-8")
    roots = [arg for name in keep for arg in ("-u", name)]
    merged, hidden, archive = work / "merged.o", work / "hidden.o", work / static.name
    if "apple" in target:
        arch = ["-arch", "arm64", f"-mmacosx-version-min={MACOS_WHEEL}"]
        run("cc", "-r", "-nostdlib", *arch, *(f"-Wl,{arg}" for arg in roots), f"-Wl,-exported_symbols_list,{names}",
            define_commons(static, arch, work), static, "-o", hidden)
        # Which stamps the member with the time unless told not to.
        run("libtool", "-static", "-o", archive, hidden, env={**os.environ, "ZERO_AR_DATE": "1"})
        # Rust's objects embed LLVM bitcode newer than Xcode's nm reads.
        found = output("nm", "--no-llvm-bc", "-gUj", hidden).split()
    else:
        # Only the members the C ABI reaches, without the sections it never references.
        run("ld", "-r", "--gc-sections", *roots, static, "-o", merged)
        # What std looks up only if the system has it, such as a newer glibc's
        # `posix_spawn_file_actions_addchdir`, stays weak: binutils 2.42's `ld -r
        # --gc-sections` makes it local, which objcopy makes strong, failing a link on an
        # older glibc.
        weak = undefined(static, "w") - undefined(static, "U")
        weaken = work / "weak.txt"
        weaken.write_text("".join(f"{name}\n" for name in sorted(weak)), encoding="utf-8")
        # A COMDAT group would still merge with another library's copy, such as std's
        # `DW.ref.rust_eh_personality`, whose symbol here is now local: as plain sections,
        # each library keeps its own.
        run("objcopy", "--remove-section=.group", f"--keep-global-symbols={names}", f"--weaken-symbols={weaken}",
            merged, hidden)
        if "There are no section groups" not in output("readelf", "-g", hidden):
            raise SystemExit(f"the prelinked {static.name} still holds section groups")
        strong = sorted(undefined(hidden, "U") & weak)
        if strong:
            raise SystemExit(f"the prelinked {static.name} needs what Rust's archive only looks up, such as {strong[:5]}")
        run("ar", "rcsD", archive, hidden)
        found = [line.split()[-1] for line in output("nm", "-g", "--defined-only", hidden).splitlines() if line.strip()]
    if sorted(set(found)) != keep:
        extra = sorted(set(found) - set(keep))[:10]
        lines = [line for line in output("nm", *(["--no-llvm-bc"] if "apple" in target else []), "-m" if "apple" in target else "-S",
                                         hidden).splitlines() if line.split() and line.split()[-1] in extra]
        sys.stderr.write("\n".join(lines) + "\n")
        raise SystemExit(f"the prelinked {static.name} exports other symbols than the C ABI, such as {extra}")
    return archive


def coexist(archive, include, lib, natives, manifest, target, work):
    """Link the prelinked static library with another Rust static library, in either order, and run it."""
    other = work / "other"
    (other / "src").mkdir(parents=True)
    (other / "Cargo.toml").write_text('[package]\nname = "other"\nversion = "0.0.0"\nedition = "2021"\n\n'
                                      '[lib]\ncrate-type = ["staticlib"]\n\n[workspace]\n', encoding="utf-8")
    (other / "src/lib.rs").write_text('#[no_mangle]\npub extern "C" fn rust2_other_sum(a: i64, b: i64) -> i64 {\n'
                                      '\t[a, b].to_vec().iter().sum::<i64>() + std::env::args().count() as i64\n}\n',
                                      encoding="utf-8")
    env = {**os.environ, "MACOSX_DEPLOYMENT_TARGET": MACOS_WHEEL} if "apple" in target else None
    run("cargo", "build", "--quiet", "--release", "--manifest-path", other / "Cargo.toml", "--target", target,
        "--target-dir", work / "target", cwd=Path(manifest).resolve().parent, env=env)
    theirs = work / "target" / target / "release" / "libother.a"
    main = work / "main.c"
    main.write_text(f'#include <stdio.h>\n#include "{lib}.h"\n\nint64_t rust2_other_sum(int64_t a, int64_t b);\n\n'
                    f"int main(void) {{\n\t{lib}_buf buf = {lib}_buf_copy(({lib}_buf){{.ptr = (const uint8_t *)\"hi\", .len = 2}});\n"
                    f'\tprintf("%zu %lld\\n", buf.len, (long long)rust2_other_sum(40, 1));\n\t{lib}_buf_free(buf);\n\treturn 0;\n}}\n',
                    encoding="utf-8")
    flags = [arg for kind, name in natives for arg in ((f"-l{name}",) if kind == "lib" else ("-framework", name))]
    for order in ((archive, theirs), (theirs, archive)):
        exe = work / "coexist"
        run("cc", "-std=c11", "-I", include, main, *order, *flags, "-o", exe)
        if output(exe) != "2 42\n":
            raise SystemExit(f"{archive.name} beside another Rust static library printed the wrong result")


def check_soname(shared, soname, target):
    """Fail unless the shared library names itself by its versioned name."""
    if "apple" in target:
        found = output("otool", "-D", shared).splitlines()[1:]
        want = f"@rpath/{soname}"
    else:
        found = re.findall(r"\(SONAME\)\s+Library soname: \[(.*)\]", output("readelf", "-d", shared))
        want = soname
    if found != [want]:
        raise SystemExit(f"{shared} names itself {found}, not {want}")


def c_archive(info, manifest, export_dir, target, out):
    """The C and C++ binding for `target`: headers, both libraries, and pkg-config, CMake, and CPS files."""
    lib, version = info["lib"], info["version"]
    files = c_files(lib, target, version)
    soname = Path(files["shared"]).name
    linux, apple = "linux" in target, "apple" in target
    link_args = ["-C", f"link-arg=-Wl,-soname,{soname}"] if linux else install_name(info, target, soname)
    env = {**reproducible(info, export_dir, target), **({"MACOSX_DEPLOYMENT_TARGET": MACOS_WHEEL} if apple else {})}
    # The C library alone, as the Swift package's: the glue's language features add symbols
    # a C program cannot resolve. Its link arguments, after `--`, reach only the generated
    # crate, so it shares the wheels' dependencies.
    # cargo-zigbuild's own `rustc`, as Cargo knows only its `zigbuild`, from the package's
    # directory for its toolchain.
    command = ["cargo-zigbuild", "rustc"] if linux else ["cargo", "rustc"]
    target_dir = export_dir / SHARED_DIR
    built = run(*command, "--color", "never", "--locked", "--release", "--manifest-path", export_dir / "Cargo.toml",
                "--target", f"{target}.{GLIBC}" if linux else target, "--target-dir", target_dir,
                "--lib", "--crate-type", "staticlib,cdylib", "--", "--print=native-static-libs", *link_args,
                cwd=Path(manifest).resolve().parent, env={**os.environ, **env}, capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    notes = re.findall(r"note: native-static-libs: (.*)", built.stderr)
    if len(notes) != 1:
        raise SystemExit(f"rustc reported no native libraries for {target}")
    natives = native_libs(notes[0].strip(), target)
    release = target_dir / target / "release"
    name = f"{info['name']}-{version}-{target}"
    with tempfile.TemporaryDirectory() as raw:
        raw = Path(raw)
        root = raw / name
        shutil.copytree(export_dir / "include", root / "include")
        for path in licenses(info):
            shutil.copy(path, root)
        for key in ("static", "shared", "implib"):
            if files[key]:
                (root / files[key]).parent.mkdir(parents=True, exist_ok=True)
        if "windows" in target:
            shutil.copy(release / f"{lib}.lib", root / files["static"])
            shutil.copy(release / f"{lib}.dll", root / files["shared"])
            shutil.copy(release / f"{lib}.dll.lib", root / files["implib"])
        else:
            shared = release / f"lib{lib}.{'dylib' if apple else 'so'}"
            check_soname(shared, soname, target)
            shutil.copy(shared, root / files["shared"])
            (root / files["link"]).symlink_to(soname)
            hidden = raw / "hide"
            hidden.mkdir()
            shutil.copy(hide(release / f"lib{lib}.a", shared, lib, target, hidden), root / files["static"])
            coexist(root / files["static"], root / "include", lib, natives, manifest, target, hidden)
            (root / "lib/pkgconfig").mkdir(parents=True)
            (root / f"lib/pkgconfig/{lib}.pc").write_text(pkg_config(info, natives), newline="\n", encoding="utf-8")
        cmake = root / "lib/cmake" / lib
        cmake.mkdir(parents=True)
        (cmake / f"{lib}-config.cmake").write_text(cmake_config(info, target, files, natives), newline="\n",
                                                   encoding="utf-8")
        (cmake / f"{lib}-config-version.cmake").write_text(cmake_version(info), newline="\n", encoding="utf-8")
        (root / "lib/cps").mkdir(parents=True)
        (root / f"lib/cps/{lib}.cps").write_text(cps(info, files, natives), newline="\n", encoding="utf-8")
        (out / "c").mkdir(parents=True, exist_ok=True)
        return pack(root, out / "c" / (name + (".zip" if "windows" in target else ".tar.gz")), info["epoch"])


def pack(root, path, stamp):
    """`root` as a ZIP or gzipped tarball under its own name, owned by no one, readable by all, stamped `stamp`, and the same bytes whoever packs it."""
    found = sorted(root.rglob("*"), key=lambda p: p.relative_to(root.parent).as_posix())
    if path.suffix == ".zip":
        write_zip(path, [(f"{root.name}/", b"", 0o755)] + [
            (p.relative_to(root.parent).as_posix() + ("/" if p.is_dir() else ""), b"" if p.is_dir() else p.read_bytes(),
             0o755 if p.is_dir() or p.stat().st_mode & 0o111 else 0o644) for p in found], stamp)
        return path

    def owned(member):
        member.uid = member.gid = 0
        member.uname = member.gname = ""
        member.mtime = stamp
        if not member.issym():
            member.mode = 0o755 if member.isdir() or member.mode & 0o100 else 0o644
        return member

    # No name or time in gzip's header, and the members in order rather than the file system's.
    with open(path, "wb") as raw, gzip.GzipFile("", "wb", 9, raw, mtime=0) as zipped, \
            tarfile.open(fileobj=zipped, mode="w", format=tarfile.PAX_FORMAT) as archive:
        archive.add(root, root.name, recursive=False, filter=owned)
        for member in found:
            archive.add(member, member.relative_to(root.parent).as_posix(), recursive=False, filter=owned)
    return path


def check_program_in(check, names):
    """The program of a check directory: the one file of `names` it holds, beside the files it reads."""
    found = [Path(check) / name for name in names if (Path(check) / name).is_file()]
    if len(found) != 1:
        raise SystemExit(f"{check} must hold exactly one of {', '.join(names)}, beside the files it reads")
    return found[0]


def c_check(lib, check, work):
    """The consumer's program: the caller's check, its directory copied whole so that its
    quoted includes resolve beside it, or else one that links a function of each header."""
    if check:
        shutil.copytree(check, work / "check")
        return check_program_in(work / "check", ("check.c", "check.cpp", "check.cc", "check.cxx"))
    source = work / "smoke.cpp"
    source.write_text(f'#include "{lib}.h"\n#include "{lib}.hpp"\n\n'
                      f"int main() {{\n\tvoid (*volatile fn)({lib}_buf) = {lib}_buf_free;\n"
                      f"\treturn fn == nullptr;\n}}\n", encoding="utf-8")
    return source


def run_check(exe, native, what, env=None):
    """Run a consumer from an empty directory, and compare its output with native Rust's."""
    with tempfile.TemporaryDirectory() as clean:
        result = subprocess.run([str(exe)], capture_output=True, cwd=clean, env={**os.environ, **(env or {})})
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{what} failed with status {result.returncode}")
    if native:
        compare(Path(native).read_bytes(), result.stdout, what)
    print(f"{what}: ok")


def check_linked(exe, soname, target):
    """Fail unless a shared consumer loads the library by its versioned name."""
    if "windows" in target:
        return
    if "apple" in target:
        found = f"@rpath/{soname}" in output("otool", "-L", exe)
    else:
        found = soname in re.findall(r"\(NEEDED\)\s+Shared library: \[(.*)\]", output("readelf", "-d", exe))
    if not found:
        raise SystemExit(f"{exe} does not load {soname}")


def consume_c(archive, check, native):
    """Build the check in clean consumers of the unpacked archive: CMake by its CPS file and by its config package, and pkg-config."""
    archive = Path(archive).resolve()
    with tempfile.TemporaryDirectory() as tmp:
        consume_c_in(Path(tmp), archive, check, native)


def consume_c_in(work, archive, check, native):
    """`consume_c` in the scratch directory `work`, which the caller deletes."""
    if archive.suffix == ".zip":
        shutil.unpack_archive(archive, work / "unpacked")
    else:
        shutil.unpack_archive(archive, work / "unpacked", filter="data")
    (prefix,) = (work / "unpacked").iterdir()
    (cps_file,) = (prefix / "lib/cps").glob("*.cps")
    description = json.loads(cps_file.read_text(encoding="utf-8"))
    lib, version = description["name"], description["version"]
    target = prefix.name.rsplit(f"-{version}-", 1)[1]
    # CMake spells a version in numbers alone, so a prerelease is found by none.
    request = "" if "-" in version.split("+")[0] else " " + re.match(r"[0-9]+\.[0-9]+\.[0-9]+", version).group()
    files = c_files(lib, target, version)
    soname = Path(files["shared"]).name
    source = c_check(lib, check, work)
    # Only the caller's check prints the transcript; the smoke program prints nothing.
    native = native if check else None
    windows = "windows" in target
    # Where the shared consumers find the library at run time; elsewhere the link records it.
    path_env = {"PATH": f"{prefix / 'bin'}{os.pathsep}{os.environ['PATH']}"} if windows else {}
    for mode in ("cps", "config"):
        project = work / f"cmake-{mode}"
        project.mkdir()
        configs = f" CONFIGS {lib}-config.cmake" if mode == "config" else ""
        found = r"\.cps$" if mode == "cps" else r"-config\.cmake$"
        (project / "CMakeLists.txt").write_text(f"""cmake_minimum_required(VERSION 4.3)
project(check LANGUAGES C CXX)
find_package({lib}{request} REQUIRED{configs})
if(NOT {lib}_CONFIG MATCHES "{found}")
    message(FATAL_ERROR "found ${{{lib}_CONFIG}}, not the {mode} file")
endif()
add_executable(check-static {cmake_quote(source.as_posix())})
target_link_libraries(check-static PRIVATE {lib}::{lib})
add_executable(check-shared {cmake_quote(source.as_posix())})
target_link_libraries(check-shared PRIVATE {lib}::{lib}_shared)
""", encoding="utf-8")
        build = project / "build"
        run("cmake", "-S", project, "-B", build, "-DCMAKE_BUILD_TYPE=Release", f"-DCMAKE_PREFIX_PATH={prefix}",
            "-DCMAKE_FIND_USE_PACKAGE_REGISTRY=OFF", "-DCMAKE_FIND_USE_SYSTEM_PACKAGE_REGISTRY=OFF")
        run("cmake", "--build", build, "--config", "Release")
        for linkage in ("static", "shared"):
            (exe,) = [p for p in build.rglob(f"check-{linkage}*") if p.is_file() and os.access(p, os.X_OK)
                      and p.suffix in ("", ".exe")]
            if linkage == "shared":
                check_linked(exe, soname, target)
            run_check(exe, native, f"CMake's {mode} consumer, {linkage}", path_env if linkage == "shared" else None)
    if windows:
        return
    compiler = ["c++", "-std=c++17"] if source.suffix in (".cpp", ".cc", ".cxx") else ["cc", "-std=c11"]
    # The shared library by default, and the static library where it is all a prefix holds.
    static_prefix = work / "static" / prefix.name
    shutil.copytree(prefix, static_prefix, symlinks=True)
    for path in (static_prefix / "lib").iterdir():
        if path.name.startswith(f"lib{lib}.so") or path.suffix == ".dylib":
            path.unlink()
    for linkage, root, flags in (("shared", prefix, []), ("static", static_prefix, ["--static"])):
        env = {**os.environ, "PKG_CONFIG_PATH": "", "PKG_CONFIG_LIBDIR": str(root / "lib/pkgconfig")}
        cflags = shlex.split(output("pkg-config", "--cflags", *flags, lib, env=env))
        libs = shlex.split(output("pkg-config", "--libs", *flags, lib, env=env))
        exe = work / f"pkg-config-{linkage}"
        rpath = [f"-Wl,-rpath,{root / 'lib'}"] if linkage == "shared" else []
        run(*compiler, *cflags, source, *libs, *rpath, "-o", exe)
        if linkage == "shared":
            check_linked(exe, soname, target)
        run_check(exe, native, f"pkg-config's consumer, {linkage}")


def compare(native, actual, what):
    """Fail with a diff unless a consumer printed exactly what native Rust printed."""
    if native == actual:
        return
    sys.stderr.buffer.writelines(difflib.diff_bytes(
        difflib.unified_diff, native.splitlines(keepends=True), actual.splitlines(keepends=True),
        fromfile=b"native Rust", tofile=what.encode()))
    raise SystemExit(f"{what} differs from native Rust")


def consume_python(check, native, module):
    """Run a check program, or else an import, on the installed wheel from an empty directory.
    The check's directory is copied apart, and it alone joins the path, where the check's own
    modules are found: `-E -s` is `-I` but for that."""
    with tempfile.TemporaryDirectory() as clean, tempfile.TemporaryDirectory() as scripts:
        if check:
            shutil.copytree(check, scripts, dirs_exist_ok=True)
            program = [str(check_program_in(scripts, ("check.py",)))]
        else:
            program = ["-I", "-c", f"import {module}"]
        result = subprocess.run([sys.executable, "-E", "-s", "-X", "dev", "-W", "error", *program],
                                capture_output=True, cwd=clean)
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{check} failed on the installed wheel with status {result.returncode}")
    if native:
        # Windows' CPython writes text-mode stdout with CRLF, where Rust's println! writes LF.
        actual = result.stdout.replace(b"\r\n", b"\n") if sys.platform == "win32" else result.stdout
        compare(Path(native).read_bytes(), actual, "installed wheel")


def exporter(manifest, features, *args):
    """Run rust2-ffi from this checkout on the package, an Android target's build scripts
    given the NDK's compilers."""
    manifest = Path(manifest).resolve()
    cargo = ["--", "--features", features] if features.strip() else []
    env = dict(os.environ)
    for target in (str(a) for a in args):
        if target in RN_ANDROID_TARGETS:
            env.update(android_toolchain(target, ANDROID_API))
    run("cargo", "run", "--locked", "--release", "--quiet", "--", "--manifest-path", manifest,
        *args, *cargo, cwd=ROOT / "ffi", env=env)


def version_key(version):
    """A Cargo version's precedence: a prerelease sorts before its release."""
    match = SEMVER.fullmatch(version)
    if match is None:
        raise SystemExit(f"version {version} is not X.Y.Z or X.Y.Z-(alpha|beta|rc).N")
    major, minor, patch, pre, number = match.groups()
    rank = {"alpha": 0, "beta": 1, "rc": 2, None: 3}[pre]
    return int(major), int(minor), int(patch), rank, int(number or 0)


def compatible(old, new):
    """Whether Cargo resolves a requirement on `old` to `new`, so `new` may not break it.

    The leftmost nonzero part is the breaking one; a prerelease promises nothing.
    """
    a, b = version_key(old), version_key(new)
    if a[3] != 3 or b[3] != 3:
        return False
    for k in range(3):
        if a[k] or b[k]:
            return a[k] == b[k]
    return True


def earlier_releases(name, version, tags):
    """The releases a version answers to, among `tags`, `<name>-v<version>`: the newest below it,
    which its changelog follows, and the newest below it Cargo treats as compatible, whose API
    it must not break, past any prerelease between them. Either may be None."""
    prefix = f"{name}-v"
    versions = [t[len(prefix):] for t in tags if t.startswith(prefix) and SEMVER.fullmatch(t[len(prefix):])]
    below = [v for v in versions if version_key(v) < version_key(version)]
    newest = max(below, key=version_key, default=None)
    baseline = max((v for v in below if compatible(v, version)), key=version_key, default=None)
    return newest, baseline


def previous(args):
    """The APIs of the releases this version follows and must not break, if any.

    Prints GitHub outputs: `previous`, the version the changelog follows, its api.json at
    <out>/previous/api.json, and `baseline`, the version whose API the export must not break,
    its api.json at <out>/baseline/api.json; each empty for none.
    """
    info = facts(args.manifest_path)
    releases = json.loads(output("gh", "api", "--paginate", "--slurp", f"repos/{args.repo}/releases"))
    tags = [r["tag_name"] for page in releases for r in page if not r["draft"]]
    found = dict(zip(("previous", "baseline"), earlier_releases(info["name"], info["version"], tags)))
    for role, version in found.items():
        if version is None:
            continue
        out = Path(args.out) / role
        out.mkdir(parents=True, exist_ok=True)
        tag = f"{info['name']}-v{version}"
        fetched = subprocess.run(["gh", "release", "download", tag, "--repo", args.repo, "--pattern", "api.json",
                                  "--dir", str(out), "--clobber"], capture_output=True, encoding="utf-8")
        if fetched.returncode != 0 or not (out / "api.json").exists():
            raise SystemExit(f"release {tag} has no api.json asset, which this release's API is checked against: "
                             f"attach the api.json an export of {tag} writes\n{fetched.stderr}")
    print("\n".join(f"{role}={version or ''}" for role, version in found.items()))


# Each kind of item an API lists, and how a changelog names it.
KINDS = {"records": "struct", "enums": "enum", "objects": "type", "callbacks": "trait", "functions": "fn",
         "constants": "const"}


def api_items(api):
    """Each item of an export's api.json by how a changelog names it, with what it declares.

    Types are named, not indexed, so an item added elsewhere changes no other; docs and
    source spans are no part of the API.
    """
    api = api["api"]
    targets = api["targets"]

    def name(kind, i):
        item = api[kind][i]
        return "::".join(item.get("shown") or item["path"])

    def resolve(value):
        if isinstance(value, dict):
            if isinstance(value.get("on"), int):
                value = {**value, "on": on(value["on"])}
            if len(value) == 1:
                (key, inner), = value.items()
                kind = {"Record": "records", "Enum": "enums", "Object": "objects"}.get(key)
                if kind and isinstance(inner, int):
                    return {key: name(kind, inner)}
                if key == "Callback" and isinstance(inner, list) and inner and isinstance(inner[0], int):
                    return {key: [name("callbacks", inner[0]), *resolve(inner[1:])]}
                if key == "Made" and isinstance(inner, dict) and isinstance(inner.get("object"), int):
                    return {key: {**resolve(inner), "object": name("objects", inner["object"])}}
            return {k: resolve(v) for k, v in value.items() if k not in ("docs", "span", "methods")}
        if isinstance(value, list):
            return [resolve(v) for v in value]
        return value

    def on(value):
        return [t for k, t in enumerate(targets) if value >> k & 1]

    items = {}
    for kind, label in KINDS.items():
        for i, item in enumerate(api[kind]):
            key = f"{accessor(item) if kind == 'functions' else label} `{name(kind, i)}`"
            items[key] = resolve(item)
            for method in item.get("methods", []) if kind != "functions" else []:
                path = "::".join(method.get("shown") or method["path"])
                items[f"{accessor(method)} `{path}`"] = resolve(method)
    return items


def accessor(function):
    """How a changelog names a method: a field's getter and setter share its path."""
    field = function.get("access")
    if isinstance(field, dict) and "Field" in field:
        return "set" if field["Field"]["set"] else "get"
    return "fn"


def changes(old, new, since):
    """A Markdown changelog of `new`'s API against `old`'s, both an export's api.json."""
    before, after = api_items(old), api_items(new)
    sections = [
        ("Removed", [k for k in before if k not in after]),
        ("Changed", [k for k in before if k in after and before[k] != after[k]]),
        ("Added", [k for k in after if k not in before]),
    ]
    lost = [t for t in old["api"]["targets"] if t not in new["api"]["targets"]]
    gained = [t for t in new["api"]["targets"] if t not in old["api"]["targets"]]
    text = f"## API changes since {since}\n"
    if lost or gained:
        text += "\n### Targets\n\n" + "".join(f"- Removed `{t}`\n" for t in lost) + \
            "".join(f"- Added `{t}`\n" for t in gained)
    for title, keys in sections:
        if keys:
            text += f"\n### {title}\n\n" + "".join(f"- {k}\n" for k in sorted(keys))
    if not any(keys for _, keys in sections) and not (lost or gained):
        text += "\nNone.\n"
    return text


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
    baseline = ["--baseline", Path(args.baseline).resolve()] if args.baseline else []
    exporter(args.manifest_path, args.features, *targets, *withs, *baseline, "--out", out)


def cargo_configs(directory):
    """Each config file cargo run from `directory` reads, parsed."""
    here = Path(directory).resolve()
    # Cargo joins a relative CARGO_HOME to where it runs.
    home = here / os.environ["CARGO_HOME"] if os.environ.get("CARGO_HOME") else Path.home() / ".cargo"
    for root in [home, *(r / ".cargo" for r in [here, *here.parents])]:
        for name in ("config.toml", "config"):
            found = root / name
            if found.is_file():
                yield found, tomllib.loads(found.read_text(encoding="utf-8"))


def rustflags_config(directory, target):
    """What sets rustflags for cargo where a RUSTFLAGS override would silently replace it."""
    for found, config in cargo_configs(directory):
        tables = {key: table for key, table in config.get("target", {}).items() if key == target or key.startswith("cfg(")}
        if "rustflags" in config.get("build", {}) or any("rustflags" in table for table in tables.values()):
            yield str(found)
    for key in os.environ:
        if key == "CARGO_BUILD_RUSTFLAGS" or re.fullmatch(r"CARGO_TARGET_.*_RUSTFLAGS", key):
            yield key


def reproducible(info, export_dir, target):
    """The environment that makes a build's bytes the same on any host, at any path.

    Release time is the commit's, and each directory the build reads from, which rustc and
    C compilers write into panic messages and debug info, is spelled the same, through `LINK`
    or not. An Apple dylib's name is `install_name`'s, which the generated crate takes alone.
    """
    named = {
        info["workspace"]: "/source",
        str(export_dir): "/export",
        os.environ.get("CARGO_HOME") or str(Path.home() / ".cargo"): "/cargo",
    }
    pairs = {path: new for old, new in named.items() for path in (old, os.path.realpath(old))}
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
    # The generated crate names an Apple dylib after `cargo rustc`'s `--`, which a caller's flags
    # would follow, and override. An `@file` flag's arguments are rustc's to read, relative to a
    # directory and with quoting this can't check, so it can't pass either.
    if "apple" in target:
        for flag in flags:
            if flag.startswith("@"):
                raise SystemExit(f"RUSTFLAGS reads arguments from {flag}, which could name an install name the packages set themselves: pass them directly")
            if APPLE_INSTALL_NAME.search(flag):
                raise SystemExit(f"RUSTFLAGS sets an install name in {flag}, which the packages set themselves: drop it")
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
        # Apple's ar and libtool stamp archive members with the time unless told not to.
        env["ZERO_AR_DATE"] = "1"
    env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(flags)
    return env


# ld64's install-name option, or an older spelling of it, as a whole argument.
APPLE_INSTALL_NAME = re.compile(r"(?:^|[\s,=])-(?:dylib_|dylinker_)?install_name(?:$|[\s,=])")


def install_name(info, target, name=None):
    """rustc's arguments naming an Apple dylib `name`, else its library's own name, where the
    linker would name it after the path it wrote it to, in the export's build directory. For
    the generated crate alone, after `cargo rustc`'s `--`: in the flags every crate builds
    with, a name would keep the libraries named apart from sharing their dependencies."""
    if "apple" not in target:
        return []
    return ["-C", f"link-arg=-Wl,-install_name,@rpath/{name or library_name(info['lib'], target)}"]


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


def sbom(info, crate, feature="python"):
    """A CycloneDX SBOM of the crates the binding is built from with `feature`, or the C ABI's
    alone with None, the same for every host and target."""
    features = ["--features", feature] if feature else []
    meta = json.loads(output("cargo", "metadata", "--locked", "--format-version", "1", *features,
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
        elif name.startswith("#1/") and name[3:].isdigit():
            # BSD's extended name, in the first bytes of the member's own data.
            length = int(name[3:])
            name = data[start:start + length].rstrip(b"\0").decode("utf-8", "replace")
            start += length
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


def tar_diff(one, two):
    """The members two gzipped tarballs hold differently, each with its content hash and attributes, and where a static library's own members differ."""
    def members(path):
        with tarfile.open(path) as archive:
            return {m.name: (archive.extractfile(m).read() if m.isfile() else m.linkname, m.mtime, m.mode, m.uid, m.gid, m.uname, m.gname)
                    for m in archive.getmembers()}
    left, right = members(one), members(two)
    lines = []
    for name in sorted(left.keys() | right.keys()):
        a, b = left.get(name), right.get(name)
        if a == b:
            continue
        digest = lambda m: (hashlib.sha256(m[0]).hexdigest(), *m[1:]) if m and isinstance(m[0], bytes) else m  # noqa: E731
        lines.append(f"  {name}: {digest(a)} against {digest(b)}")
        if name.endswith(".a") and a and b:
            with tempfile.TemporaryDirectory() as tmp:
                for index, content in enumerate((a[0], b[0])):
                    (Path(tmp) / f"{index}.a").write_bytes(content)
                lines += ["  " + line for line in ar_diff(Path(tmp) / "0.a", Path(tmp) / "1.a")]
    return lines


def same(first, second):
    """Fail unless two builds' wheels, sdist, static libraries, C archives, Dart libraries, Android and desktop JVM libraries, SBOMs, Maven repository, and npm tarballs, the Go module's included, are the same bytes, naming what differs."""
    found = {}
    patterns = ("wheels/*.whl", "sdist/*.tar.gz", "static/*/*.a", "react-native/*/*.a", "go/*.a", "c/*.tar.gz", "c/*.zip", "dart/*",
                "android/*/*.so", "jvm/*/*", "maven/**/*.*", "*.tgz", "*.cdx.json")
    for root in (Path(first), Path(second)):
        found[root] = {p.relative_to(root).as_posix(): p for pattern in patterns for p in root.glob(pattern) if p.is_file()}
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
        elif name.endswith((".tar.gz", ".tgz")):
            problems += tar_diff(one, two)
        elif name.endswith((".zip", ".aar", ".jar")):
            problems += zip_diff(one, two)
    if problems:
        raise SystemExit("a rebuild made other bytes:\n" + "\n".join(problems))
    print(f"{len(names)} files rebuilt byte for byte")


def cargo(manifest, *args, env=None):
    """Cargo with the package's own toolchain, as rustup finds it from the package's directory."""
    return run("cargo", *args, cwd=Path(manifest).resolve().parent, env={**os.environ, **(env or {})},
               capture_output=True, encoding="utf-8")


def local_packages(manifest, export, *features):
    """The local packages besides `export` that its crate builds with, and the workspace roots apart from them."""
    graph = json.loads(cargo(manifest, "metadata", "--format-version", "1", "--locked", *features,
                             "--manifest-path", export / "Cargo.toml").stdout)
    local = sorted({Path(p["manifest_path"]).parent for p in graph["packages"] if p["source"] is None} - {export})
    manifests = {d / "Cargo.toml" for d in local}
    roots = set()
    for directory in local:
        # From the build's directory, with its toolchain, not one a dependency's own file pins.
        root = Path(cargo(manifest, "locate-project", "--workspace", "--message-format", "plain",
                          "--manifest-path", directory / "Cargo.toml").stdout.strip())
        if not any(root.samefile(m) for m in manifests):
            roots.add(root)
    return local, sorted(roots)


@contextlib.contextmanager
def linked(manifest, export, *features):
    """`export` as cargo reaches it through `LINK`, which links to the deepest directory holding
    it and every local package and workspace root it builds with, so their layout below that
    directory alone names them. Cargo joins each relative path lexically from it, never
    resolving the link. Yields the spelling through `LINK` of a directory below it, or the
    directory itself where a manifest names a path absolutely, or a cargo config names a path
    package (a path `[patch]` or `paths`), which it joins from its own real directory: cargo
    reaches that package by that path, which no build at another path shares, and through the
    link by another, which it would read as two packages."""
    local, roots = local_packages(manifest, export, *features)
    for path in [export / "Cargo.toml", *(d / "Cargo.toml" for d in local), *roots]:
        if any(Path(p).is_absolute() for p in manifest_paths(tomllib.loads(path.read_text(encoding="utf-8")))):
            yield Path
            return
    # Cargo reads its config from where it runs, the manifest's real directory, as `cargo` runs it.
    if any(manifest_paths(config.get("patch", {})) or config.get("paths") for _, config in cargo_configs(Path(manifest).resolve().parent)):
        yield Path
        return
    try:
        base = Path(os.path.commonpath([p.resolve() for p in (export, *local, *(r.parent for r in roots))]))
    except ValueError:
        raise SystemExit(f"{export} and the packages it builds with lie on different drives, which no link spans") from None
    try:
        if sys.platform == "win32":
            # A junction, which unlike a symlink needs no privilege.
            import _winapi
            _winapi.CreateJunction(str(base), str(LINK))
        else:
            os.symlink(base, LINK, target_is_directory=True)
    except FileExistsError:
        raise SystemExit(f"{LINK} exists: another build is using it, or a killed one left it, to remove") from None
    try:
        yield lambda path: LINK / Path(path).resolve().relative_to(base)
    finally:
        # rmdir removes a junction itself, unlink a symlink; neither touches what it links to.
        if sys.platform == "win32":
            os.rmdir(LINK)
        else:
            os.unlink(LINK)


def staticlib(manifest, crate, lib, target, env=None, config=()):
    """The C library alone, apart from the Python extension, whose CPython symbols it would
    leave undefined, and the system libraries rustc says it needs beside it."""
    # An Apple slice's apart, at the Swift package's floor and with line tables, and Intel
    # macOS's at rustc's own floor, since cargo does not rebuild for a changed deployment target.
    # Windows GNU's builds nothing else to share, and dlltool names its import symbols after
    # the directory, so it keeps its own.
    target_dir = crate.parent / ("target-swift" if target in APPLE_TARGETS
                                 else "target-go" if "apple" in target or "windows" in target else SHARED_DIR)
    # cargo-zigbuild links Linux against the glibc floor's symbols, whichever glibc the host has.
    linux = target.endswith("linux-gnu")
    # Uncolored, so the note below reads as written whatever the caller's terminal.
    built = run(*(["cargo-zigbuild"] if linux else ["cargo"]), "rustc", "--color", "never", "--locked", "--release",
                "--manifest-path", crate, "--target", f"{target}.{GLIBC}" if linux else target,
                "--target-dir", target_dir, *config, "--lib", "--crate-type", "staticlib", "--", "--print=native-static-libs",
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
# dlltool names them after, as `<path>_out_rustc<6 random characters>_kernel32_dll_imports_lib`,
# and its members after the imported file, whatever its extension, such as ntoskrnl.exe's.
RUSTC_TEMP = re.compile(rb"(?<=_out_rustc)[0-9A-Za-z]{6}(?=_[0-9a-z_]+_imports_lib)")
IMPORT_MEMBER = re.compile(r".+\.[0-9A-Za-z]+(h|t|s[0-9]+)\.o")


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
    """This host's wheels, C archives, Apple and Go static libraries, Dart libraries, with
    `kotlin` the Kotlin binding's desktop and Android libraries, with `react_native` React
    Native's Android libraries, the export's sources, and native output."""
    info = facts(args.manifest_path)
    lib, export_dir, out = info["lib"], Path(args.export).resolve(), Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=True)
    info["sbom"] = sbom(info, export_dir / "Cargo.toml")
    (out / "sbom.cdx.json").write_bytes(info["sbom"])
    with linked(args.manifest_path, export_dir, "--all-features") as spell:
        # The facts with the workspace through the link, for `reproducible` to remap while it exists.
        through, inside = spell(export_dir), {**info, "workspace": str(spell(info["workspace"]))}
        crate = through / "Cargo.toml"
        for target in args.targets:
            if not any(target in t for t in (WHEEL_TARGETS, APPLE_TARGETS, GO_TARGETS, DART_TARGETS, ANDROID_TARGETS,
                                             RN_ANDROID_TARGETS)):
                raise SystemExit(f"no package ships {target}")
            run("rustup", "target", "add", target, cwd=Path(args.manifest_path).resolve().parent)
            if target in WHEEL_TARGETS:
                env = reproducible(inside, through, target)
                if "apple" in target:
                    env["MACOSX_DEPLOYMENT_TARGET"] = MACOS_WHEEL
                # pyo3 builds each stable ABI without asking a CPython, which the host may lack.
                env.pop("PYO3_PYTHON", None)
                env["PYO3_NO_PYTHON"] = "1"
                linux = "linux" in target
                for abi, feature in backend.FEATURES.items():
                    # cargo-zigbuild links Linux against the glibc floor's symbols, whichever glibc the host has.
                    built = run(*(["cargo-zigbuild"] if linux else ["cargo"]), "rustc", "--locked", "--release",
                                "--manifest-path", crate, "--target", f"{target}.{GLIBC}" if linux else target,
                                "--target-dir", through / SHARED_DIR, "--lib", "--features", feature, "--",
                                *install_name(inside, target), cwd=Path(args.manifest_path).resolve().parent,
                                env={**os.environ, **env}, capture_output=True, encoding="utf-8")
                    sys.stderr.write(built.stderr)
                    library = through / SHARED_DIR / target / "release" / library_name(lib, target)
                    with tempfile.TemporaryDirectory() as raw, tempfile.TemporaryDirectory() as fixed:
                        wheel = repair(write_wheel(inside, through, library, target, raw, abi), target, fixed,
                                       inside["epoch"])
                        if args.kotlin and target in JVM_TARGETS and (bundled := vendored(wheel)):
                            raise SystemExit(f"{lib} links {bundled} on {target}, which the wheel bundles but the desktop "
                                             "jar cannot: its loader loads one library")
                        (out / "wheels").mkdir(parents=True, exist_ok=True)
                        shutil.move(wheel, out / "wheels" / wheel.name)
                c_archive(inside, args.manifest_path, through, target, out)
            if target in APPLE_TARGETS:
                apple = APPLE_TARGETS[target]
                # Mac Catalyst's floor is an iOS version, which rustc and cc read as iOS's.
                name = "MACOSX_DEPLOYMENT_TARGET" if apple == "macos" else "IPHONEOS_DEPLOYMENT_TARGET"
                library, native = staticlib(args.manifest_path, crate, lib, target,
                                            env={**reproducible(inside, through, target), name: APPLE_BUILD[target]},
                                            config=line_tables(crate))
                static = out / "static" / target
                static.mkdir(parents=True)
                shutil.copy(library, static)
                (static / "native.txt").write_text(native + "\n", encoding="utf-8")
                # The Go module links the Swift package's macOS library, at its floor.
                if target in GO_TARGETS:
                    go_library(args, through, lib, target, library, native, out)
            elif target in GO_TARGETS or args.react_native and target in RN_ANDROID_TARGETS:
                env = reproducible(inside, through, target)
                # Android's at the AAR's API floor, React Native's too, through the NDK.
                if "android" in target:
                    env.update(android_toolchain(target, ANDROID_API))
                library, native = staticlib(args.manifest_path, crate, lib, target, env=env)
                if target in GO_TARGETS:
                    go_library(args, through, lib, target, library, native, out)
                if args.react_native and target in RN_ANDROID_TARGETS:
                    dest = out / "react-native" / RN_ANDROID_TARGETS[target]
                    dest.mkdir(parents=True)
                    shutil.copy(library, dest)
                    (dest / "native.txt").write_text(native + "\n", encoding="utf-8")
            if target in DART_TARGETS:
                dart_library(inside, args.manifest_path, through, target, out)
            if args.kotlin and target in JVM_TARGETS:
                jvm_library(inside, args.manifest_path, through, target, out)
            if args.kotlin and target in ANDROID_TARGETS:
                android_library(inside, args.manifest_path, through, target, out)
        # The crates the Kotlin binding's libraries link, which Maven serves beside them.
        if args.kotlin and any(t in JVM_TARGETS or t in ANDROID_TARGETS for t in args.targets):
            (out / "kotlin.cdx.json").write_bytes(sbom(info, export_dir / "Cargo.toml", "kotlin"))
        # And React Native's, the C ABI alone, which its npm tarball's attestation names.
        if args.react_native and any(t in RN_ANDROID_TARGETS for t in args.targets):
            (out / "react-native.cdx.json").write_bytes(sbom(info, export_dir / "Cargo.toml", None))
    # The generated sources, for the Swift package, the Go module, and the hosts' agreement.
    # Text this writes is LF on every host, as the jobs reading it compare bytes.
    for part in ("include", "swift", "python", "go", "kotlin", "dart", "csharp", "react-native"):
        shutil.copytree(export_dir / part, out / "export" / part, dirs_exist_ok=True)
    shutil.copy(export_dir / "Cargo.toml", out / "export")
    shutil.copy(export_dir / "api.json", out / "export")
    (out / "digest.txt").write_text(digest(export_dir, lib) + "\n", newline="\n", encoding="utf-8")
    (out / "lib.txt").write_text(lib + "\n", newline="\n", encoding="utf-8")
    # Native output for a host whose wheel or Go module a consumer runs; an iOS slice runs
    # nowhere here, and an Android one runs on the emulator, which runs the example's build.
    runs = [t for t in args.targets
            if t in WHEEL_TARGETS or t in GO_TARGETS and t not in APPLE_TARGETS and t not in ANDROID_TARGETS]
    if args.example and runs:
        if args.example not in info["examples"]:
            raise SystemExit(f"{info['name']} has no example {args.example}: it has {info['examples']}")
        features = ["--features", args.features] if args.features.strip() else []
        manifest = Path(args.manifest_path).resolve()
        # Bytes, as the consumers' output is compared, with no newline translation.
        native = run("cargo", "run", "--locked", "--release", "--quiet", "--manifest-path", manifest,
                     "--example", args.example, *features, cwd=manifest.parent, capture_output=True)
        (out / "native.txt").write_bytes(native.stdout)
    if args.example and ANDROID_EMULATOR in args.targets:
        android_example(info, args, out)
    if args.example and IOS_SIMULATOR in args.targets:
        ios_example(info, args, out)


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
    unpacked from its zip, in a new module of its own: no checkout, Rust, or CGO_LDFLAGS.
    The check's directory is the program's package, `check.go` beside the files it reads."""
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
        if check:
            check_program_in(check, ("check.go",))
            shutil.copytree(check, consumer, dirs_exist_ok=True)
        else:
            (consumer / "main.go").write_text(f'package main\n\nimport _ "{module}/rt"\n\nfunc main() {{}}\n', encoding="utf-8")
        # After the check's files, which hold no module of their own.
        (consumer / "go.mod").write_text(
            f"module consumer\n\ngo 1.24\n\nrequire {module} {version}\n\nreplace {module} => ../mod/{prefix}\n",
            encoding="utf-8")
        env = {k: v for k, v in os.environ.items() if not k.startswith("CGO_") or k == "CGO_ENABLED"}
        env.update(CGO_ENABLED="1", GOFLAGS="-mod=mod", GOPROXY="off", GOTOOLCHAIN="local", GOWORK="off")
        run("go", "vet", f"{module}/...", ".", cwd=consumer, env=env)
        exe = consumer / ("check.exe" if sys.platform == "win32" else "check")
        run("go", "build", "-o", exe, ".", cwd=consumer, env=env)
        result = subprocess.run([exe], capture_output=True, cwd=consumer)
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{check or 'the import'} failed on the Go module with status {result.returncode}")
    if native and check:
        compare(Path(native).read_bytes(), result.stdout, "Go module")


def dart_file(info, target):
    """The release asset of `target`'s Dart library."""
    return f"{info['name']}-{info['version']}-dart-{target}{Path(library_name(info['lib'], target)).suffix}"


def dart_library(info, manifest, export_dir, target, out):
    """`target`'s library for the Dart package, the C ABI with its Dart glue, built for the
    oldest system its hook lets an app target."""
    lib, floor = info["lib"], DART_TARGETS[target]
    name = library_name(lib, target)
    env = reproducible(info, export_dir, target)
    if target.endswith("apple-darwin"):
        env["MACOSX_DEPLOYMENT_TARGET"] = floor or MACOS_WHEEL
    elif "apple-ios" in target:
        env["IPHONEOS_DEPLOYMENT_TARGET"] = floor
    elif "android" in target:
        env.update(android_toolchain(target, floor))
    # Linux's against the glibc floor, as the wheel's.
    glibc = target.endswith("linux-gnu")
    command = ["cargo-zigbuild", "rustc"] if glibc else ["cargo", "rustc"]
    # Apart on Apple at a floor below the wheels', since cargo does not rebuild for a changed
    # deployment target; Android's floor is the AAR's, as Go's and React Native's libraries build.
    target_dir = export_dir / ("target-dart" if "apple" in target and floor is not None else SHARED_DIR)
    built = run(*command, "--color", "never", "--locked", "--release", "--manifest-path", export_dir / "Cargo.toml",
                "--target", f"{target}.{GLIBC}" if glibc else target, "--target-dir", target_dir,
                "--lib", "--crate-type", "cdylib", "--features", "dart", "--", *install_name(info, target),
                cwd=Path(manifest).resolve().parent, env={**os.environ, **env}, capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    dest = out / "dart" / dart_file(info, target)
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy(target_dir / target / "release" / name, dest)
    # The required-reason APIs the linked library imports, which `pub` declares for the app.
    if "apple" in target:
        privacy = out / "dart-privacy" / f"{target}.xcprivacy"
        privacy.parent.mkdir(parents=True, exist_ok=True)
        privacy.write_bytes(privacy_manifest(lib, required_apis(dest)))


def readme(info):
    """The crate's readme, as Cargo finds it, or else a line saying what the package is."""
    if info["readme"]:
        return (Path(info["dir"]) / info["readme"]).read_text(encoding="utf-8")
    said = f"{' '.join(info['description'].split())}\n\n" if info["description"] else ""
    return f"# {info['lib']}\n\n{said}The Dart binding of the Rust crate {info['name']}, generated by rust2-ffi.\n"


def pub_package(info, export, libs, url, changes, privacy, out):
    """The pub.dev package: the export's, its pubspec given Cargo's facts, with the license,
    readme, and changelog pub asks for, hook/release.json naming each target's library
    in `libs`, hosted at `url`, by size and SHA-256, and the privacy manifest of the Apple
    libraries' manifests in `privacy`."""
    out = Path(out)
    if out.exists():
        shutil.rmtree(out)
    shutil.copytree(Path(export) / "dart", out)
    pubspec = (out / "pubspec.yaml").read_text(encoding="utf-8")
    version = re.search(r"^version: (.*)$", pubspec, re.M)
    if version is None or version.group(1) != info["version"]:
        raise SystemExit(f"the export's pubspec.yaml is not the crate's version {info['version']}")
    # Cargo's facts beside what the export wrote, each a JSON string, which YAML reads alike.
    said = [f"{key}: {json.dumps(info[key])}" for key in ("repository", "homepage") if info[key]]
    if info["description"]:
        pubspec = re.sub(r"^description: .*$", lambda _: f"description: {json.dumps(' '.join(info['description'].split()))}",
                         pubspec, count=1, flags=re.M)
    pubspec = pubspec.replace(version.group(0), "\n".join([version.group(0), *said]), 1)
    (out / "pubspec.yaml").write_text(pubspec, newline="\n", encoding="utf-8")
    libraries = {}
    names = {dart_file(info, target): target for target in DART_TARGETS}
    found = {path.name for path in Path(libs).iterdir()}
    if found != set(names):
        raise SystemExit(f"{libs} lacks {sorted(set(names) - found)} and holds {sorted(found - set(names))} beside the Dart libraries")
    for name, target in names.items():
        data = (Path(libs) / name).read_bytes()
        libraries[target] = {"file": name, "size": len(data), "sha256": hashlib.sha256(data).hexdigest()}
        if DART_TARGETS[target]:
            libraries[target]["minimum"] = DART_TARGETS[target]
    if not url.endswith("/"):
        raise SystemExit(f"{url} is no directory: end it with /")
    release = {"url": url, "libraries": dict(sorted(libraries.items()))}
    (out / "hook" / "release.json").write_text(json.dumps(release, indent=2) + "\n", newline="\n", encoding="utf-8")
    # pub takes a license from a file named LICENSE alone, so one holds each of the crate's,
    # or else Cargo's SPDX expression.
    texts = [path.read_text(encoding="utf-8").strip() for path in licenses(info)]
    if not texts and info["license"]:
        texts = [f"SPDX-License-Identifier: {info['license']}"]
    if not texts:
        raise SystemExit("pub.dev needs a license: set Cargo's license, or put a license file beside the manifest")
    (out / "LICENSE").write_text("\n\n".join(texts) + "\n", newline="\n", encoding="utf-8")
    (out / "README.md").write_text(readme(info), newline="\n", encoding="utf-8")
    # Every Apple library's declarations, which an app on Apple's stores merges into its own
    # manifest: Flutter wraps a code asset in a framework it generates, which carries none.
    types = {}
    for target in (t for t in DART_TARGETS if "apple" in t):
        manifest = Path(privacy) / f"{target}.xcprivacy"
        if not manifest.is_file():
            raise SystemExit(f"{privacy} lacks {manifest.name}, the privacy manifest of {target}'s Dart library")
        with open(manifest, "rb") as f:
            for declared in plistlib.load(f)["NSPrivacyAccessedAPITypes"]:
                types[declared["NSPrivacyAccessedAPIType"]] = declared
    (out / "PrivacyInfo.xcprivacy").write_bytes(
        plistlib.dumps({"NSPrivacyAccessedAPITypes": [types[t] for t in sorted(types)]}, sort_keys=True))
    # This release's changes, under the version pub looks for.
    notes = Path(changes).read_text(encoding="utf-8")
    (out / "CHANGELOG.md").write_text(f"## {info['version']}\n\n" + re.sub(r"^(#+) ", r"#\1 ", notes, flags=re.M),
                                      newline="\n", encoding="utf-8")


def pub_name(package):
    """The name of the package in `package`, as its pubspec says."""
    found = re.search(r"^name: (\S+)$", (Path(package) / "pubspec.yaml").read_text(encoding="utf-8"), re.M)
    if found is None:
        raise SystemExit(f"{package}/pubspec.yaml names no package")
    return found.group(1)


class Served:
    """A directory served over HTTP on the loopback interface, as a release's host would."""

    def __init__(self, root):
        import functools
        import http.server
        import threading
        class Quiet(http.server.SimpleHTTPRequestHandler):
            def log_message(self, *_):
                pass

        handler = functools.partial(Quiet, directory=str(root))
        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
        self.url = f"http://127.0.0.1:{self.server.server_address[1]}/"
        threading.Thread(target=self.server.serve_forever, daemon=True).start()

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.server.shutdown()


def local_package(package, dest, url):
    """The package, its libraries named at `url` instead of the release's host."""
    shutil.copytree(package, dest)
    path = Path(dest) / "hook" / "release.json"
    release = json.loads(path.read_text(encoding="utf-8"))
    release["url"] = url
    path.write_text(json.dumps(release, indent=2) + "\n", encoding="utf-8")
    return Path(dest)


# The entry of a Dart check, beside it: it runs the check's `main`, which takes the arguments or
# none, as the Flutter app's does.
DART_MAIN = """import 'check.dart' as check;

Future<void> main(List<String> args) async {
  final Function run = check.main;
  final result = run is Function(List<String>) ? run(args) : run();
  if (result is Future) await result;
}
"""


def dart_check(lib, check, where):
    """Place the consumer's program in `where` as check.dart: the caller's check directory
    whole, its `check.dart` beside the files it imports, or else a program that calls the
    library's contract. It calls rather than takes the address: Dart 3.14's AOT resolves no
    `Native.addressOf` of a function naming its `assetId`, as this one must from outside the package."""
    where = Path(where)
    if check:
        check_program_in(check, ("check.dart",))
        if (Path(check) / "main.dart").exists():
            raise SystemExit(f"{check} holds a main.dart, which the consumer writes beside check.dart")
        shutil.copytree(check, where, dirs_exist_ok=True)
        return
    (where / "check.dart").write_text(
        f"import 'dart:ffi';\n\n"
        f"@Native<Uint64 Function()>(symbol: '{lib}_contract', assetId: 'package:{lib}/{lib}.dart', isLeaf: true)\n"
        f"external int _contract();\n\n"
        f"void main() {{\n  _contract();\n}}\n", encoding="utf-8")


def dart_entry(lib, check, app):
    """The app's `bin/` holding the check and the entry that runs it."""
    dart_check(lib, check, app / "bin")
    (app / "bin" / "main.dart").write_text(DART_MAIN, encoding="utf-8")


def dart_app(work, name, package, lib):
    """A new app depending on the package by path, as on a hosted one."""
    app = Path(work) / name
    (app / "bin").mkdir(parents=True)
    (app / "pubspec.yaml").write_text(
        f"name: {name}\npublish_to: none\n\nenvironment:\n  sdk: ^3.10.0\n\n"
        f"dependencies:\n  {lib}:\n    path: {json.dumps(Path(package).resolve().as_posix())}\n", encoding="utf-8")
    run("dart", "pub", "get", cwd=app, capture_output=True)
    return app


def consume_dart(package, libs, lib, check, native):
    """New Dart apps depending on the package, its libraries served from the loopback as the
    release hosts them: one served a library of other bytes, which its hook refuses, and one
    that runs the check by `dart run` and from the bundle `dart build cli` makes."""
    with tempfile.TemporaryDirectory() as raw:
        work = Path(raw)
        good, bad = work / "served", work / "tampered"
        shutil.copytree(libs, good)
        shutil.copytree(libs, bad)
        for path in bad.iterdir():
            data = bytearray(path.read_bytes())
            data[len(data) // 2] ^= 1
            path.write_bytes(bytes(data))
        with Served(bad) as served:
            app = dart_app(work, "tampered", local_package(package, work / "bad-package", served.url), lib)
            dart_entry(lib, None, app)
            result = subprocess.run(["dart", "run", "bin/main.dart"], cwd=app, capture_output=True, encoding="utf-8")
            if result.returncode == 0 or "SHA-256" not in result.stdout + result.stderr:
                sys.stderr.write(result.stdout + result.stderr)
                raise SystemExit("the hook took a library of other bytes than the release names")
        with Served(good) as served:
            app = dart_app(work, "consumer", local_package(package, work / "package", served.url), lib)
            dart_entry(lib, check, app)
            run("dart", "build", "cli", "--target", "bin/main.dart", "--output", work / "build", cwd=app,
                capture_output=True)
            exe = work / "build" / "bundle" / "bin" / ("main.exe" if sys.platform == "win32" else "main")
            run_dart(["dart", "run", "bin/main.dart"], app, check, native, "dart run")
            run_dart([exe], work, check, native, "dart build cli")


def run_dart(command, cwd, check, native, what):
    """Run a Dart consumer, and compare what it prints with native Rust."""
    result = subprocess.run([str(c) for c in command], cwd=cwd, capture_output=True)
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{what} failed on the Dart package with status {result.returncode}")
    # Dart 3.10's `dart run` says it runs the hooks on stdout, before the program does.
    stdout = re.sub(rb"^(Running build hooks\.\.\.)+", b"", result.stdout)
    # `print` writes through the C runtime's text mode, which ends each line in CRLF on Windows,
    # where Rust's `println!` ends it in LF; the transcript holds no CR of its own.
    if sys.platform == "win32":
        stdout = stdout.replace(b"\r\n", b"\n")
    if native and check:
        compare(Path(native).read_bytes(), stdout, f"Dart package, {what}")
    print(f"{what}: ok")


def consume_source(export, check, native):
    """A new Dart app depending on the export's own package, whose hook builds the exported
    crate with Cargo, as in a checkout."""
    package = Path(export).resolve() / "dart"
    lib = pub_name(package)
    with tempfile.TemporaryDirectory() as raw:
        app = dart_app(raw, "consumer", package, lib)
        dart_entry(lib, check, app)
        run_dart(["dart", "run", "bin/main.dart"], app, check, native, "dart run from source")


# The Flutter consumer's entry, which runs the check, collecting what it prints, then reports
# it base64-encoded in short lines, as Android's log splits long ones, and how it ended, as an
# app has no exit status.
FLUTTER_MAIN = """import 'dart:async';
import 'dart:convert';
import 'dart:io' as io;
import 'dart:math';

import 'check.dart' as check;

void main() {
  final out = StringBuffer();
  void report(String end) {
    final data = base64.encode(utf8.encode(out.toString()));
    Zone.root.print('rust2: start');
    for (var i = 0; i < data.length; i += 800) {
      Zone.root.print('rust2: ${data.substring(i, min(i + 800, data.length))}');
    }
    Zone.root.print('rust2: $end');
  }

  runZonedGuarded(
    () async {
      final Function run = check.main;
      final result = run is Function(List<String>) ? run(<String>[]) : run();
      if (result is Future) await result;
      report('exit ${io.exitCode}');
    },
    (error, stack) => report('threw $error $stack'),
    zoneSpecification: ZoneSpecification(print: (_, _, _, line) => out.writeln(line)),
  );
}
"""


# Silence longer than any quiet step a sound run has once the app is built, such as attach's
# first compile or the check itself, so a stalled run fails saying so rather than at the job's limit.
SILENT = 15 * 60

# The `--target-platform` Flutter builds for a device's ABI.
FLUTTER_TARGETS = {"arm64-v8a": "android-arm64", "armeabi-v7a": "android-arm", "x86_64": "android-x64"}


def reader(stream, handle):
    """A queue of the items `handle` makes of each of `stream`'s lines, from a thread reading
    them, ending in None once it closes."""
    import threading
    lines = queue.Queue()

    def read():
        try:
            for line in stream:
                for item in handle(line.rstrip("\r\n")):
                    lines.put(item)
        # A terminal's end, once what writes to it exits: Linux's ends reads in EIO.
        except OSError:
            pass
        lines.put(None)
    threading.Thread(target=read, daemon=True).start()
    return lines


def take(lines, what):
    """The next item from a `reader` queue, failing once `what` closes or falls silent."""
    try:
        item = lines.get(timeout=SILENT)
    except queue.Empty:
        raise SystemExit(f"{what} said nothing for {SILENT // 60} minutes") from None
    if item is None:
        raise SystemExit(f"{what} ended before the check did")
    return item


# The consumer app's package, and so its process's name.
FLUTTER_APP = "dev.rust2.consumer"


def check_output(said, what):
    """What one run of a consumer app's check printed, from the lines `said` returns that it
    marks `rust2: `: `start`, its output in base64 chunks, then `exit <code>`, which must be
    0, or `threw <error>`."""
    chunks = None
    while True:
        line = said()
        if not line.startswith("rust2: "):
            continue
        line = line[len("rust2: "):]
        if line == "start":
            chunks = []
        elif chunks is None:
            continue
        elif line.startswith("threw "):
            raise SystemExit(f"the check threw in the {what}: {line}")
        elif line.startswith("exit "):
            if line != "exit 0":
                raise SystemExit(f"the check failed in the {what}: {line}")
            return base64.b64decode("".join(chunks))
        else:
            chunks.append(line)


class Logcat:
    """What the device logs under the app's tag, Flutter's or React Native's, and any crash of
    the consumer app, from now on."""

    # The tag the app logs what it prints under, and the app it is.
    tag, what = "flutter", "Flutter app"

    def __init__(self, device, tag="flutter", what="Flutter app"):
        self.tag, self.what = tag, what
        # From the log's last line, as `flutter run` reads it: Android 7's emulator may refuse
        # to clear its log.
        last = adb("-s", device, "logcat", "-v", "time", "-t", "1", capture_output=True, encoding="utf-8").stdout
        if not (since := re.search(r"^\d\d-\d\d \d\d:\d\d:\d\d\.\d{3}", last, re.M)):
            raise SystemExit(f"the device's log holds no time to read it from: {last!r}")
        self.process = subprocess.Popen(
            [adb_path(), "-s", device, "logcat", "-T", since[0], "-v", "brief", "-s", f"{tag}:I", "libc:F", "DEBUG:F",
             "AndroidRuntime:E"], text=True, encoding="utf-8", errors="replace", stdout=subprocess.PIPE)
        self.lines = reader(self.process.stdout, lambda line: [line])
        # The app's process, known from its first line under Flutter's tag.
        self.pid = None
        # Where the app's Dart VM service listens, from the engine's announcement.
        self.vm_service = None

    def crashed(self, tag, pid, text):
        """Whether a crash tag's line is the consumer app's, rather than another process's: a
        fatal signal, such as a Rust panic's abort, in the app's process, debuggerd's dump
        naming it, or an uncaught Java exception in it."""
        return ((tag == "F/libc" and pid == self.pid) or (tag == "F/DEBUG" and f">>> {FLUTTER_APP} <<<" in text)
                or (tag == "E/AndroidRuntime" and text.startswith(f"Process: {FLUTTER_APP},")))

    def said(self):
        """The next line the app logs under its tag, failing on its crash."""
        while True:
            line = take(self.lines, "the device's log")
            print(line, file=sys.stderr)
            if not (entry := re.match(r"([A-Z]/[^(]*?)\s*\(\s*(\d+)\): (.*)", line)):
                continue
            tag, pid, said = entry[1], int(entry[2]), entry[3]
            if tag != f"I/{self.tag}":
                if self.crashed(tag, pid, said):
                    raise SystemExit(f"the {self.what} crashed: {line}")
                continue
            self.pid = self.pid or pid
            if listening := re.match(r"The Dart VM service is listening on (http://\S+)", said):
                self.vm_service = listening[1]
            return said

    def listening(self):
        """Where the app's Dart VM service listens, which a debug build may announce only
        after a quick check has run."""
        while not self.vm_service:
            self.said()
        return self.vm_service

    def output(self):
        """What one run of the check printed, failing unless it exits 0."""
        return check_output(self.said, self.what)


class Flutter:
    """`flutter attach --machine` to the running app: its events and its commands."""

    def __init__(self, app, device, vm_service):
        # Given the VM service, as attach otherwise looks for its announcement in the device's
        # whole log, which the check's own output may have rotated out by now.
        self.process = subprocess.Popen(["flutter", "attach", "--machine", "-d", device, "--debug-url", vm_service],
                                        cwd=app, text=True, encoding="utf-8", stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE)
        self.events = reader(self.process.stdout, self.event)
        self.app, self.started, self.ids = None, False, 0

    @staticmethod
    def event(line):
        try:
            events = json.loads(line) if line.startswith("[{") else None
        except json.JSONDecodeError:
            events = None
        if events is None:
            if line:
                print(line, file=sys.stderr)
            return []
        for event in events:
            # What flutter is doing, such as compiling, so a stall says where.
            if event.get("event") in ("app.progress", "daemon.logMessage") and event["params"].get("message"):
                print(f"flutter: {event['params']['message']}", file=sys.stderr, flush=True)
        return events

    def next(self):
        """The next event, failing once flutter exits or a command fails."""
        event = take(self.events, "flutter attach")
        if "error" in event:
            raise SystemExit(f"flutter attach refused a command: {event['error']}")
        params = event.get("params", {})
        if event.get("event") == "app.start":
            self.app = params["appId"]
        elif event.get("event") == "app.started":
            self.started = True
        return event

    def reply(self):
        """The reply to the last command, failing unless it succeeded."""
        while (event := self.next()).get("id") != self.ids:
            pass
        if (result := event.get("result")) and result.get("code"):
            raise SystemExit(f"flutter attach failed a command: {result}")
        return event

    def command(self, method, **params):
        """Send a command once flutter has attached, as it takes none before."""
        while not self.started:
            self.next()
        self.ids += 1
        self.process.stdin.write(json.dumps([{"id": self.ids, "method": method, "params": {"appId": self.app, **params}}]) + "\n")
        self.process.stdin.flush()


def consume_flutter(package, libs, lib, check, native, device):
    """A new Flutter app depending on the package, its libraries served from the loopback as
    the release hosts them, run in debug on the Android `device`, then again after a hot
    restart, which keeps the library and Rust's statics but not the isolate: each run prints
    the check's output."""
    abi = adb("-s", device, "shell", "getprop", "ro.product.cpu.abi", capture_output=True, encoding="utf-8").stdout.strip()
    if abi not in FLUTTER_TARGETS:
        raise SystemExit(f"Flutter builds for no Android ABI {abi}, only {sorted(FLUTTER_TARGETS)}")
    with tempfile.TemporaryDirectory() as raw, Served(libs) as served:
        work = Path(raw)
        app = work / "consumer"
        run("flutter", "create", "--platforms", "android", "--empty", "--project-name", "consumer", "--org", "dev.rust2",
            app, capture_output=True)
        run("flutter", "pub", "add", lib, "--path", local_package(package, work / "package", served.url), cwd=app,
            capture_output=True)
        dart_check(lib, check, app / "lib")
        (app / "lib" / "main.dart").write_text(FLUTTER_MAIN, encoding="utf-8")
        run("flutter", "build", "apk", "--debug", "--target-platform", FLUTTER_TARGETS[abi], cwd=app)
        # Installed here rather than by `flutter run`, whose streamed install, adb's default
        # from Android 7, hangs at random on Android 7's emulator; pushing it never has.
        adb("-s", device, "install", "-r", "--no-streaming", app / "build" / "app" / "outputs" / "flutter-apk" / "app-debug.apk",
            capture_output=True)
        logcat, flutter = Logcat(device), None
        try:
            # `am start` exits 0 whether or not it starts the app.
            started = adb("-s", device, "shell", "am", "start", "-n", f"{FLUTTER_APP}/.MainActivity",
                          capture_output=True, encoding="utf-8").stdout
            if re.search(r"Error|Exception", started):
                raise SystemExit(f"the Flutter app did not start: {started}")
            first = logcat.output()
            flutter = Flutter(app, device, logcat.listening())
            flutter.command("app.restart", fullRestart=True, pause=False)
            # A refused restart fails here, rather than as the log's silence.
            flutter.reply()
            second = logcat.output()
            flutter.command("app.stop")
            flutter.process.stdin.close()
            try:
                status = flutter.process.wait(timeout=SILENT)
            except subprocess.TimeoutExpired:
                raise SystemExit(f"flutter attach went on {SILENT // 60} minutes after the app stopped") from None
        except BaseException:
            if flutter:
                flutter.process.kill()
            raise
        finally:
            logcat.process.kill()
        if status:
            raise SystemExit(f"flutter attach failed with status {status}")
    for output, what in ((first, "Flutter app"), (second, "Flutter app after a hot restart")):
        if native and check:
            compare(Path(native).read_bytes(), output, what)
        print(f"{what}: ok")


def package_name(info, scope):
    """The web package's name on npm and JSR, `@<scope>/<crate>`, refusing what JSR cannot spell."""
    name = f"@{scope}/{info['name']}"
    if not re.fullmatch(r"@[a-z0-9][a-z0-9-]{1,19}/[a-z0-9][a-z0-9-]{1,57}", name):
        raise SystemExit(f"{name} is no JSR package name: a scope and crate name of lowercase letters, digits, and hyphens")
    return name


def web_package(info, web_dir, wasm, scope, out):
    """The npm package, the JavaScript and declarations, and the JSR package, the TypeScript,
    each with the module beside them where `load` finds it and the license files."""
    web_dir, out = Path(web_dir), Path(out)
    lib, name = info["lib"], package_name(info, scope)
    manifest = json.loads((web_dir / "package.json").read_text(encoding="utf-8"))
    if manifest["version"] != info["version"]:
        raise SystemExit(f"the export's package.json is {manifest['version']}, not the crate's {info['version']}")
    npm, jsr = out / "npm", out / "jsr"
    # A tarball of an earlier build, which `consume-web` would find beside this one.
    for tarball in out.glob("*.tgz") if out.exists() else []:
        tarball.unlink()
    files = licenses(info)
    # JSR takes a license from jsr.json's field, which Cargo's `license` fills, or else from
    # a file by one of its own names.
    jsr_names = {path: path.name for path in files}
    if not info["license"] and not any(path.name.lower() in JSR_LICENSES for path in files):
        if len(files) != 1:
            raise SystemExit(f"JSR needs Cargo's `license`, or one license file, not {[p.name for p in files]}")
        jsr_names = {files[0]: "LICENSE"}
    for part in (npm, jsr):
        if part.exists():
            shutil.rmtree(part)
        part.mkdir(parents=True)
        shutil.copy(wasm, part / f"{lib}.wasm")
        for path in files:
            shutil.copy(path, part / (jsr_names[path] if part == jsr else path.name))
    modules = []
    for path in sorted(web_dir.iterdir()):
        if path.name.endswith(".d.ts") or path.suffix == ".js":
            shutil.copy(path, npm)
        elif path.suffix == ".ts":
            shutil.copy(path, jsr)
            modules.append(path.stem)
    if "index" not in modules:
        raise SystemExit(f"{web_dir} holds no web binding")
    # What Cargo says of the package, beside what the export wrote.
    said = {"name": name, "version": info["version"]}
    if info["description"]:
        said["description"] = " ".join(info["description"].split())
    if info["license"]:
        said["license"] = info["license"]
    if info["repository"]:
        said["repository"] = {"type": "git", "url": f"git+{info['repository']}"}
    if info["homepage"]:
        said["homepage"] = info["homepage"]
    manifest = {**said, **{k: v for k, v in manifest.items() if k not in said}}
    # npm packs a license file by a name of its own only if `files` names it.
    manifest["files"] = manifest["files"] + [p.name for p in licenses(info) if p.name not in manifest["files"]]
    (npm / "package.json").write_text(json.dumps(manifest, indent="\t") + "\n", newline="\n", encoding="utf-8")
    # JSR takes no export pattern: each module by its name. The module ships unexported,
    # as Deno would link an exported one as a module of its own, whose imports none serves.
    exports = {".": "./index.ts", **{f"./{m}": f"./{m}.ts" for m in modules if m != "index"}}
    config = {"name": name, "version": info["version"]}
    if info["license"]:
        config["license"] = info["license"]
    config["exports"] = exports
    (jsr / "jsr.json").write_text(json.dumps(config, indent="\t") + "\n", newline="\n", encoding="utf-8")
    size = sum(p.stat().st_size for p in jsr.iterdir())
    if size > JSR_LIMIT:
        raise SystemExit(f"the JSR package is {size} bytes, past JSR's {JSR_LIMIT}")
    return npm, jsr


def ndk_prebuilt():
    """The NDK's toolchain for this host, which every Android build needs."""
    ndk = os.environ.get("ANDROID_NDK_HOME")
    if not ndk:
        raise SystemExit("building for Android needs the NDK: set ANDROID_NDK_HOME")
    host = {"linux": "linux-x86_64", "darwin": "darwin-x86_64", "win32": "windows-x86_64"}[sys.platform]
    return Path(ndk) / "toolchains" / "llvm" / "prebuilt" / host


def android_toolchain(target, api):
    """The NDK's clang for `target` at `api`, as Cargo links and cc builds C with it."""
    bin_dir = ndk_prebuilt() / "bin"
    # The NDK spells 32-bit Arm's compiler armv7a, where Rust's target is armv7.
    triple = "armv7a-linux-androideabi" if target.startswith("armv7") else target
    # Windows' NDK wraps each in a batch file.
    cmd = ".cmd" if sys.platform == "win32" else ""
    clang, clangxx = (bin_dir / f"{triple}{api}-{name}{cmd}" for name in ("clang", "clang++"))
    if not clang.is_file():
        raise SystemExit(f"the NDK has no {clang}")
    key = target.replace("-", "_")
    return {f"CARGO_TARGET_{key.upper()}_LINKER": str(clang), f"CC_{key}": str(clang), f"CXX_{key}": str(clangxx),
            f"AR_{key}": str(bin_dir / "llvm-ar")}


def android_sysroot(target):
    """The NDK's libraries for `target`: the C++ runtime, and in a directory per API level, the
    system's own, which every device of that level holds."""
    # The NDK's 32-bit Arm libraries lie under arm, where Rust's target is armv7.
    triple = "arm-linux-androideabi" if target.startswith("armv7") else target
    return ndk_prebuilt() / "sysroot" / "usr" / "lib" / triple


def android_runtime(binary, target):
    """The libraries `binary` needs beyond what a device at the AAR's floor holds, as names."""
    needed = json.loads(output(ndk_prebuilt() / "bin" / "llvm-readobj", "--elf-output-style=JSON", "--needed-libs",
                               binary))[0]["NeededLibraries"]
    system = {p.name for p in (android_sysroot(target) / str(ANDROID_API)).glob("*.so")}
    return [n for n in needed if n not in system]


def android_check(library, target):
    """Fail unless `library` aligns every segment to the 16 KB pages Google Play requires of a
    64-bit library from 2027-02-01, holds the line tables Play symbolicates a crash by, and
    needs no library a device at the AAR's floor lacks. The NDK's shared C++ runtime is one,
    which an AAR cannot ship: an app holding another AAR's copy packs only one of the two."""
    found = json.loads(output(ndk_prebuilt() / "bin" / "llvm-readobj", "--elf-output-style=JSON", "--program-headers",
                              "--section-headers", library))[0]
    segments = [h["ProgramHeader"] for h in found["ProgramHeaders"]]
    loads = [s["Alignment"] for s in segments if s["Type"]["Name"] == "PT_LOAD"]
    if not loads or min(loads) < 16384:
        raise SystemExit(f"{library} aligns its segments to {sorted(set(loads))}, not 16 KB pages")
    sections = {s["Section"]["Name"]["Name"] for s in found["Sections"]}
    if ".debug_line" not in sections:
        raise SystemExit(f"{library} holds no line tables, which a crash in it symbolicates by")
    if missing := android_runtime(library, target):
        raise SystemExit(f"{library} needs {missing}, which an Android {ANDROID_API} device lacks and an AAR cannot ship")


def android_library(info, manifest, export_dir, target, out):
    """`target`'s library for the AAR, the C ABI with its JNI glue, built for its floor. It keeps its line tables: an app's build strips them, and gives
    them to Play, which symbolicates a crash in Rust with them."""
    lib, crate = info["lib"], export_dir / "Cargo.toml"
    env = {**reproducible(info, export_dir, target), **android_toolchain(target, ANDROID_API)}
    built = run("cargo", "rustc", "--color", "never", "--locked", "--release", "--manifest-path", crate,
                "--target", target, "--target-dir", export_dir / "target-android", *line_tables(crate),
                "--lib", "--crate-type", "cdylib", "--features", "kotlin", cwd=Path(manifest).resolve().parent,
                env={**os.environ, **env}, capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    library = export_dir / "target-android" / target / "release" / f"lib{lib}.so"
    android_check(library, target)
    dest = out / "android" / ANDROID_TARGETS[target]
    dest.mkdir(parents=True)
    shutil.copy(library, dest)


def jvm_library(info, manifest, export_dir, target, out):
    """`target`'s library for the desktop jar, the C ABI with its JNI glue, built as the wheel's
    is: Linux's against the glibc floor, macOS's at the wheels' floor, in SHARED_DIR."""
    env = reproducible(info, export_dir, target)
    if "apple" in target:
        env["MACOSX_DEPLOYMENT_TARGET"] = MACOS_WHEEL
    linux = "linux" in target
    built = run(*(["cargo-zigbuild"] if linux else ["cargo"]), "rustc", "--locked", "--release",
                "--manifest-path", export_dir / "Cargo.toml", "--target", f"{target}.{GLIBC}" if linux else target,
                "--target-dir", export_dir / SHARED_DIR, "--lib", "--features", "kotlin", "--", *install_name(info, target),
                cwd=Path(manifest).resolve().parent, env={**os.environ, **env}, capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    dest = out / "jvm" / target
    dest.mkdir(parents=True)
    shutil.copy(export_dir / SHARED_DIR / target / "release" / library_name(info["lib"], target), dest)


def android_example(info, args, out):
    """The example built for the emulator, where its output is native Rust's on Android."""
    if args.example not in info["examples"]:
        raise SystemExit(f"{info['name']} has no example {args.example}: it has {info['examples']}")
    features = ["--features", args.features] if args.features.strip() else []
    manifest = Path(args.manifest_path).resolve()
    built = run("cargo", "build", "--locked", "--release", "--message-format", "json-render-diagnostics",
                "--manifest-path", manifest, "--target", ANDROID_EMULATOR, "--example", args.example, *features,
                cwd=manifest.parent, env={**os.environ, **android_toolchain(ANDROID_EMULATOR, ANDROID_API)},
                capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    found = [m["executable"] for m in map(json.loads, built.stdout.splitlines())
             if m.get("reason") == "compiler-artifact" and m.get("executable") and "example" in m["target"]["kind"]]
    if len(found) != 1:
        raise SystemExit(f"cargo built no single example {args.example}: {found}")
    # Named apart from the library, which the consumer runs it beside, with the NDK's C++
    # runtime should a dev-dependency need it, which no device holds.
    dest = out / "android-example"
    dest.mkdir(parents=True)
    shutil.copy(found[0], dest / "example")
    for needed in android_runtime(found[0], ANDROID_EMULATOR):
        if needed != "libc++_shared.so":
            raise SystemExit(f"example {args.example} needs {needed}, which neither an Android {ANDROID_API} device nor the NDK holds")
        shutil.copy(android_sysroot(ANDROID_EMULATOR) / needed, dest)


def ios_example(info, args, out):
    """The example built for the simulator, where its output is native Rust's on iOS."""
    if args.example not in info["examples"]:
        raise SystemExit(f"{info['name']} has no example {args.example}: it has {info['examples']}")
    features = ["--features", args.features] if args.features.strip() else []
    manifest = Path(args.manifest_path).resolve()
    built = run("cargo", "build", "--locked", "--release", "--message-format", "json-render-diagnostics",
                "--manifest-path", manifest, "--target", IOS_SIMULATOR, "--example", args.example, *features,
                cwd=manifest.parent, env={**os.environ, "IPHONEOS_DEPLOYMENT_TARGET": APPLE_BUILD[IOS_SIMULATOR]},
                capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    found = [m["executable"] for m in map(json.loads, built.stdout.splitlines())
             if m.get("reason") == "compiler-artifact" and m.get("executable") and "example" in m["target"]["kind"]]
    if len(found) != 1:
        raise SystemExit(f"cargo built no single example {args.example}: {found}")
    dest = out / "ios-example"
    dest.mkdir(parents=True)
    shutil.copy(found[0], dest / "example")


def java_package(name):
    """The dotted `name` as a Java package, which the Android Gradle Plugin requires an AAR's
    namespace to be, spelled as the Java Language Specification's 6.1 advises: a hyphen as an
    underscore, a reserved word followed by one, and a segment no identifier starts with
    preceded by one."""
    def segment(part):
        part = part.replace("-", "_")
        return f"{part}_" if part in JAVA_RESERVED else f"_{part}" if part[0].isdigit() else part
    return ".".join(segment(p) for p in name.split("."))


def maven_version(version):
    """Cargo's version as Maven's, which orders a prerelease before its release as Cargo does,
    refusing build metadata, which Maven would order as a later release."""
    if SEMVER.fullmatch(version) is None:
        raise SystemExit(f"version {version} has no Maven equivalent: use X.Y.Z or X.Y.Z-(alpha|beta|rc).N")
    return version


def maven_licenses(info):
    """The POM's licenses, any of which a user may choose, as Maven reads several: each of an
    SPDX expression's alternatives, or the expression whole when it is more than a choice."""
    expression = info["license"]
    if not expression:
        raise SystemExit("Maven Central requires a license: set Cargo's license")
    if "/" in expression:
        raise SystemExit(f"license {expression!r} is no SPDX expression: write it with OR")
    choices = expression.split(" OR ")
    if any(not re.fullmatch(r"[A-Za-z0-9.+-]+", c) for c in choices):
        return [(expression, None)]
    return [(c, f"https://spdx.org/licenses/{c.removesuffix('+')}.html") for c in choices]


def maven_developers(info):
    """The POM's developers: Cargo's authors, or else the owner of its repository, who publishes it."""
    developers = []
    for author in info["authors"]:
        name, email = re.fullmatch(r"\s*(.*?)\s*(?:<([^<>]*)>)?\s*", author).groups()
        developers.append({"name": name or email, **({"email": email} if email else {})})
    if not developers and info["repository"]:
        url = urllib.parse.urlsplit(info["repository"])
        owner = url.path.strip("/").split("/")[0]
        if owner:
            developers.append({"id": owner, "name": owner, "url": f"{url.scheme}://{url.netloc}/{owner}"})
    if not developers:
        raise SystemExit("Maven Central requires a developer: set Cargo's authors or repository")
    return developers


def xml(tag, children, indent=1):
    """An XML element of escaped text, or of nested `(tag, children)` pairs, indented by tabs."""
    pad = "\t" * indent
    if isinstance(children, str):
        return f"{pad}<{tag}>{xml_escape(children)}</{tag}>\n"
    return f"{pad}<{tag}>\n{''.join(xml(t, c, indent + 1) for t, c in children)}{pad}</{tag}>\n"


def xml_escape(text):
    """Text as an XML element holds it."""
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def maven_dependencies(coroutines):
    """The libraries the binding's classes need, as `(group, artifact, version)`."""
    return [("org.jetbrains.kotlin", "kotlin-stdlib", f"{KOTLIN_API}.0"),
            ("org.jetbrains.kotlinx", "kotlinx-coroutines-core", coroutines)]


def pom(info, group, coroutines):
    """The POM Maven Central requires: coordinates, name, description, URL, licenses,
    developers, and source control, each from Cargo, and the libraries the classes need. Its
    packaging is the jar, which Maven, reading no Gradle metadata, resolves on any JVM."""
    if not info["description"]:
        raise SystemExit("Maven Central requires a description: set Cargo's description")
    if not info["repository"]:
        raise SystemExit("Maven Central requires the source's repository: set Cargo's repository")
    repository = info["repository"].removesuffix("/")
    git = repository if repository.endswith(".git") else f"{repository}.git"
    fields = [
        ("modelVersion", "4.0.0"), ("groupId", group), ("artifactId", info["name"]),
        ("version", maven_version(info["version"])), ("packaging", "jar"), ("name", info["name"]),
        ("description", " ".join(info["description"].split())), ("url", info["homepage"] or repository),
        ("licenses", [("license", [("name", name), *([("url", url)] if url else [])]) for name, url in maven_licenses(info)]),
        ("developers", [("developer", list(d.items())) for d in maven_developers(info)]),
        ("scm", [("url", repository), ("connection", f"scm:git:{git}"), ("developerConnection", f"scm:git:{git}")]),
        # Compile scope: the binding's API names their types.
        ("dependencies", [("dependency", [("groupId", g), ("artifactId", a), ("version", v), ("scope", "compile")])
                          for g, a, v in maven_dependencies(coroutines)]),
    ]
    return ('<?xml version="1.0" encoding="UTF-8"?>\n'
            '<project xmlns="http://maven.apache.org/POM/4.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" '
            'xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 https://maven.apache.org/xsd/maven-4.0.0.xsd">\n'
            # What tells Gradle to read the .module beside it, which chooses the AAR on Android.
            "\t<!-- do_not_remove: published-with-gradle-metadata -->\n"
            + "".join(xml(tag, value) for tag, value in fields) + "</project>\n")


def android_manifest(namespace):
    """The AAR's manifest: its namespace, which no other library's R class shares, and its
    floor, which an app's manifest merge refuses to go below."""
    return ('<?xml version="1.0" encoding="utf-8"?>\n'
            f'<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="{namespace}">\n'
            f'\t<uses-sdk android:minSdkVersion="{ANDROID_API}" />\n</manifest>\n')


def kotlin_classes(lib, export, module):
    """The export's Kotlin compiled as an Android library's is, against android.jar and
    kotlinx-coroutines, as a jar's entries, and the coroutines' version, which the POM requires.
    A desktop JVM runs the same classes: they name no Android API, and `@FastNative`, which
    android.jar alone holds, is an annotation the JVM never loads."""
    sdk = os.environ.get("ANDROID_HOME")
    if not sdk:
        raise SystemExit("compiling the Kotlin binding's classes needs the Android SDK: set ANDROID_HOME")
    android_jar = Path(sdk) / "platforms" / f"android-{ANDROID_COMPILE_SDK}" / "android.jar"
    if not android_jar.is_file():
        raise SystemExit(f"{android_jar} is missing: install platforms;android-{ANDROID_COMPILE_SDK}")
    coroutines = os.environ.get("KOTLINX_COROUTINES")
    if not coroutines:
        raise SystemExit("compiling the Kotlin binding's classes needs kotlinx-coroutines-core-jvm's jar: set KOTLINX_COROUTINES")
    with zipfile.ZipFile(coroutines) as jar:
        version = jar.read("META-INF/kotlinx_coroutines_core.version").decode().strip()
    with tempfile.TemporaryDirectory() as raw:
        classes = Path(raw)
        # Android's library alone, no JDK's, as an Android build compiles; and moq's Kotlin
        # outgrows the 512 MiB heap kotlinc's launcher defaults to.
        run("kotlinc", "-Werror", "-no-jdk", "-no-reflect", "-language-version", KOTLIN_API, "-api-version", KOTLIN_API,
            "-jvm-target", JVM_TARGET, "-module-name", module, "-cp", os.pathsep.join([str(android_jar), coroutines]),
            Path(export) / "kotlin" / f"{lib}.kt", "-d", classes, env={**os.environ, "JAVA_OPTS": "-Xmx2g -Xms128M"})
        files = sorted(p for p in classes.rglob("*") if p.is_file())
        return [(p.relative_to(classes).as_posix(), p.read_bytes(), 0o644) for p in files], version


def gradle_module(group, artifact, version, dependencies, files):
    """The Gradle module metadata naming each variant's file: the AAR for Android and the jar
    for the JVM, by the attributes Kotlin Multiplatform publishes them with, which AGP's and
    Kotlin's builds ask for, and the sources and javadoc for either."""
    def file(name):
        data = files[name]
        return {"name": name, "url": name, "size": len(data),
                **{algorithm: hashlib.new(algorithm, data).hexdigest() for algorithm in ("sha512", "sha256", "sha1", "md5")}}
    requires = [{"group": g, "module": a, "version": {"requires": v}} for g, a, v in dependencies]
    name = f"{artifact}-{version}"
    variants = []
    for platform, environment, kotlin, elements, binary in (
            ("android", "android", "androidJvm", {}, f"{name}.aar"),
            ("jvm", "standard-jvm", "jvm", {"org.gradle.libraryelements": "jar"}, f"{name}.jar")):
        for usage in ("api", "runtime"):
            variants.append({"name": f"{platform}{usage.capitalize()}Elements", "attributes": {
                "org.gradle.category": "library", "org.gradle.jvm.environment": environment, **elements,
                "org.gradle.usage": f"java-{usage}", "org.jetbrains.kotlin.platform.type": kotlin,
            }, "dependencies": requires, "files": [file(binary)]})
    for docs in ("sources", "javadoc"):
        variants.append({"name": f"{docs}Elements", "attributes": {
            "org.gradle.category": "documentation", "org.gradle.dependency.bundling": "external",
            "org.gradle.docstype": docs, "org.gradle.usage": "java-runtime",
        }, "files": [file(f"{name}-{docs}.jar")]})
    module = {"formatVersion": "1.1",
              "component": {"group": group, "module": artifact, "version": version,
                            "attributes": {"org.gradle.status": "release"}},
              "variants": variants}
    return (json.dumps(module, indent=2) + "\n").encode()


def maven(args):
    """The jar and the AAR, and the files Maven Central requires beside them, laid out as a
    Maven repository: the POM, the Gradle metadata choosing between the two, sources and
    javadoc jars, the CycloneDX SBOM, and each file's MD5 and SHA-1."""
    info = facts(args.manifest_path)
    group, lib, stamp = args.group, info["lib"], info["epoch"]
    if not MAVEN_GROUP.fullmatch(group):
        raise SystemExit(f"the Maven group must be a dotted namespace, such as dev.example, not {group!r}")
    export, libs = Path(args.export).resolve(), Path(args.libs).resolve()
    artifact, version = info["name"], maven_version(info["version"])
    missing = sorted([f"android/{abi}" for abi in ANDROID_TARGETS.values() if not (libs / "android" / abi / f"lib{lib}.so").is_file()]
                     + [f"jvm/{t}" for t in JVM_TARGETS if not (libs / "jvm" / t / library_name(lib, t)).is_file()])
    if missing:
        raise SystemExit(f"{libs} holds no library for {missing}: the jar and the AAR ship every target")
    # The POM's facts first, as a missing one fails before the slow compile.
    pom(info, group, "")
    classes, coroutines = kotlin_classes(lib, export, f"{group}.{artifact}")
    natives = [n for n, _, _ in classes if n.endswith("/_Native.class")]
    if len(natives) != 1:
        raise SystemExit(f"the compiled Kotlin holds no single _Native: {natives}")
    package = natives[0].removesuffix("/_Native.class")
    namespace = java_package(f"{group}.{lib}")
    classes_jar = io.BytesIO()
    write_zip(classes_jar, classes, stamp)
    aar = [("AndroidManifest.xml", android_manifest(namespace).encode(), 0o644), ("R.txt", b"", 0o644),
           ("classes.jar", classes_jar.getvalue(), 0o644),
           *((f"jni/{abi}/{p.name}", p.read_bytes(), 0o755)
             for abi in sorted(ANDROID_TARGETS.values()) for p in sorted((libs / "android" / abi).iterdir())),
           # R8 reads an AAR's proguard.txt as the consumer rules of each app that shrinks it.
           ("proguard.txt", (export / "kotlin" / f"{lib}.pro").read_bytes(), 0o644)]
    # The index the binding's loader reads beside _Native, naming each target's library, and the libraries.
    bundled = [(t, library_name(lib, t)) for t in sorted(JVM_TARGETS)]
    jar = [*classes, (f"{package}/{lib}.natives", "".join(f"{t} {f}\n" for t, f in bundled).encode(), 0o644),
           *((f"{package}/native/{t}/{f}", (libs / "jvm" / t / f).read_bytes(), 0o755) for t, f in bundled)]
    kotlin = (export / "kotlin" / f"{lib}.kt").read_bytes()
    readme = (f"# {artifact} {version}\n\nThe API's documentation is the KDoc in {artifact}-{version}-sources.jar,\n"
              "which an IDE shows beside the classes.\n").encode()
    base = Path(args.out).resolve().joinpath(*group.split("."), artifact, version)
    if base.exists():
        shutil.rmtree(base)
    base.mkdir(parents=True)
    name = f"{artifact}-{version}"
    files = {f"{name}.pom": pom(info, group, coroutines).encode(),
             f"{name}-cyclonedx.json": Path(args.sbom).read_bytes()}
    for file, entries in ((f"{name}.jar", jar), (f"{name}.aar", aar),
                          (f"{name}-sources.jar", [(f"{lib}/{lib}.kt", kotlin, 0o644)]),
                          (f"{name}-javadoc.jar", [("README.md", readme, 0o644)])):
        sink = io.BytesIO()
        write_zip(sink, entries, stamp)
        files[file] = sink.getvalue()
    files[f"{name}.module"] = gradle_module(group, artifact, version, maven_dependencies(coroutines), files)
    for file, data in sorted(files.items()):
        (base / file).write_bytes(data)
        for algorithm in ("md5", "sha1"):
            (base / f"{file}.{algorithm}").write_text(hashlib.new(algorithm, data).hexdigest(), encoding="utf-8")
    return f"{group}:{artifact}:{version}"


# The consumer app's Android Gradle Plugin, which compiles its Kotlin itself, and the SDK it
# targets, the newest stable.
AGP = "9.5.2"
APP_SDK = 36
# The consumer's application ID, its instrumentation's name, and its Kotlin package.
APP = "rust2.check"

# The consumer's instrumentation, which runs the check, or loads the library, in the app's
# own process and class loader. A check may end its process with exitProcess, so its output
# goes to files, and a second run reports them with how the first ended: the status it
# exited with, from the system's record of the process (API 30), or the reason it died.
INSTRUMENTATION = """package rust2.check

import android.app.ActivityManager
import android.app.ApplicationExitInfo
import android.app.Instrumentation
import android.os.Build
import android.os.Bundle
import android.util.Base64
import java.io.File
import java.io.FileOutputStream
import java.io.PrintStream
import java.lang.reflect.InvocationTargetException

class Check : Instrumentation() {
	private var step: String? = null
	private var main: String? = null

	override fun onCreate(arguments: Bundle?) {
		super.onCreate(arguments)
		step = arguments?.getString("step")
		main = arguments?.getString("main")
		start()
	}

	override fun onStart() {
		val files = context.filesDir
		val report = Bundle()
		if (step == "run") {
			for (name in listOf("out", "err", "status")) File(files, name).delete()
			System.setOut(PrintStream(FileOutputStream(File(files, "out")), true, "UTF-8"))
			val err = PrintStream(FileOutputStream(File(files, "err")), true, "UTF-8")
			System.setErr(err)
			val status = try {
				@RUN@
				0
			} catch (thrown: Throwable) {
				((thrown as? InvocationTargetException)?.cause ?: thrown).printStackTrace(err)
				1
			}
			System.out.flush()
			File(files, "status").writeText("$status")
		} else {
			val status = File(files, "status")
			report.putString("status", if (status.exists()) status.readText() else ended())
			for (name in listOf("out", "err")) {
				val file = File(files, name)
				report.putString(name, Base64.encodeToString(if (file.exists()) file.readBytes() else ByteArray(0), Base64.NO_WRAP))
			}
		}
		finish(android.app.Activity.RESULT_OK, report)
	}

	private fun ended(): String {
		if (Build.VERSION.SDK_INT < 30) return "no record of how the process ended below API 30"
		val manager = context.getSystemService(ActivityManager::class.java)
		val exit = manager.getHistoricalProcessExitReasons(null, 0, 1).firstOrNull() ?: return "no record of the process"
		if (exit.reason == ApplicationExitInfo.REASON_EXIT_SELF) return "${exit.status}"
		return "the process ended: ${exit.description} (reason ${exit.reason}, status ${exit.status})"
	}
}
"""


def adb_path():
    """The SDK's adb."""
    sdk = os.environ.get("ANDROID_HOME")
    if not sdk:
        raise SystemExit("the Android consumer needs the SDK: set ANDROID_HOME")
    return Path(sdk) / "platform-tools" / "adb"


def adb(*args, **kwargs):
    """The SDK's adb, on the one device it reaches unless `-s` names one."""
    return run(adb_path(), *args, **kwargs)


def main_classes(listing, source):
    """The classes `javap` lists in `listing` as compiled from the file `source` with a
    `public static void main(String[])`, the entry point Kotlin compiles a top-level `main` into."""
    found, compiled, name = set(), None, None
    for line in listing.splitlines():
        if heading := re.fullmatch(r'Compiled from "(.*)"', line):
            compiled = heading[1]
        elif declared := re.match(r"\S.*?\b(?:class|interface|enum|record) ([\w.$]+)", line):
            name = declared[1]
        elif re.fullmatch(r"\s+public static (?:final )?void main\(java\.lang\.String(?:\[\]|\.\.\.)\);", line):
            if compiled == source:
                found.add(name)
        elif line == "}":
            compiled = name = None
    return sorted(found)


def jdk(tool):
    """The JDK's `tool`, from the JDK Gradle runs on."""
    home = os.environ.get("JAVA_HOME")
    exe = f"{tool}.exe" if sys.platform == "win32" else tool
    found = Path(home) / "bin" / exe if home else shutil.which(tool)
    if not found or not Path(found).is_file():
        raise SystemExit(f"the consumer needs the JDK's {tool}: set JAVA_HOME")
    return Path(found)


def instrument(step, *extra):
    """The consumer's instrumentation run with `step` and the `-e` arguments `extra`, as `am
    instrument` reports its results."""
    out = adb("shell", "am", "instrument", "-w", "-r", "-e", "step", step, *extra, f"{APP}/.Check",
              capture_output=True, encoding="utf-8").stdout
    return dict(re.findall(r"^INSTRUMENTATION_RESULT: (\w+)=(.*?)\r?$", out, re.M))


def kotlin_check(check, source):
    """The Kotlin check `check`, a file or a directory holding check.kt, as an app's sources in `source`."""
    check = Path(check)
    if check.is_dir():
        # Sources alone: the Android app has no file beside them to read at run time.
        if other := sorted(str(p.relative_to(check)) for p in check.rglob("*") if p.is_file() and p.suffix != ".kt"):
            raise SystemExit(f"{check} holds files other than Kotlin sources, which the app cannot read: {other}")
        shutil.copytree(check, source, dirs_exist_ok=True)
    else:
        shutil.copy(check, source / "check.kt")
    if not (source / "check.kt").is_file():
        raise SystemExit(f"{check} holds no check.kt")


def maven_coordinates(repo):
    """The group, artifact, and version of the one release in the Maven repository `repo`, and its directory."""
    poms = list(Path(repo).rglob("*.pom"))
    if len(poms) != 1:
        raise SystemExit(f"expected one POM in {repo}, found {poms}")
    version_dir = poms[0].parent
    group = ".".join(version_dir.parent.parent.relative_to(Path(repo)).parts)
    return group, version_dir.parent.name, version_dir.name, version_dir


def consume_android(repo, check, native):
    """A new Gradle app depending on the AAR from the Maven repository `repo`, as an app
    resolves it from Central, shrunk by R8 with its consumer rules and run on the device adb
    reaches: the check program, or else a load of the library, in the app's own process. With
    `native`, the example built for the device, the check prints what it prints there."""
    if native and not check:
        raise SystemExit("an example's output is compared with a check's: give a check with it")
    group, artifact, version, version_dir = maven_coordinates(repo)
    with zipfile.ZipFile(version_dir / f"{artifact}-{version}.aar") as aar:
        with zipfile.ZipFile(io.BytesIO(aar.read("classes.jar"))) as jar:
            natives = [n for n in jar.namelist() if n.endswith("/_Native.class")]
        libraries = [n for n in aar.namelist() if n.startswith("jni/")]
    abis = {n.split("/")[1] for n in libraries}
    if len(natives) != 1:
        raise SystemExit(f"the AAR's classes hold no single _Native: {natives}")
    package = natives[0].removesuffix("/_Native.class").replace("/", ".")
    abi = adb("shell", "getprop", "ro.product.cpu.abi", capture_output=True, encoding="utf-8").stdout.strip()
    if abi not in abis:
        raise SystemExit(f"the device's ABI {abi} is not among the AAR's {sorted(abis)}")
    with tempfile.TemporaryDirectory() as raw:
        app = Path(raw)
        source = app / "src" / "main" / "kotlin"
        (source / "rust2" / "check").mkdir(parents=True)
        if check:
            kotlin_check(check, source)
            # Its main, in whichever class the compiler names, which reflection reaches from
            # the instrumentation's package and which is kept, as an app keeps its entry points.
            body = 'Class.forName(main!!).getMethod("main", Array<String>::class.java).invoke(null, arrayOf<String>())'
            keep = "-keepclasseswithmembers class * { public static void main(java.lang.String[]); }\n"
        else:
            # Loading the library checks its contract against the Kotlin's.
            body = f'Class.forName("{package}._Native")'
            keep = ""
        (source / "rust2" / "check" / "Check.kt").write_text(INSTRUMENTATION.replace("@RUN@", body), encoding="utf-8")
        (app / "app.pro").write_text(keep, encoding="utf-8")
        (app / "src" / "main" / "AndroidManifest.xml").write_text(
            '<manifest xmlns:android="http://schemas.android.com/apk/res/android">\n'
            # A check may serve or dial on the loopback, which an app may not without it.
            '\t<uses-permission android:name="android.permission.INTERNET" />\n'
            f'\t<instrumentation android:name=".Check" android:targetPackage="{APP}" />\n</manifest>\n', encoding="utf-8")
        local = Path(repo).resolve().as_uri()
        # The NDK the libraries were built with: without one, AGP packs them unstripped and
        # hands Play no debug info.
        ndk = os.environ.get("ANDROID_NDK_HOME")
        if not ndk:
            raise SystemExit("stripping the app's libraries needs the Android NDK: set ANDROID_NDK_HOME")
        (app / "settings.gradle.kts").write_text(f"""pluginManagement {{
	repositories {{
		google()
		mavenCentral()
	}}
}}

dependencyResolutionManagement {{
	repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
	repositories {{
		// The release's repository serves its group alone, as Central would.
		exclusiveContent {{
			forRepository {{ maven("{local}") }}
			filter {{ includeGroup("{group}") }}
		}}
		google()
		mavenCentral()
	}}
}}

rootProject.name = "check"
""", encoding="utf-8")
        # Signing a bundle holds it in memory, and a library's debug info can make it larger
        # than Gradle's default 512 MB heap.
        (app / "gradle.properties").write_text("org.gradle.jvmargs=-Xmx4g -Dfile.encoding=UTF-8\n", encoding="utf-8")
        (app / "build.gradle.kts").write_text(f"""plugins {{
	id("com.android.application") version "{AGP}"
}}

android {{
	namespace = "{APP}"
	compileSdk = {APP_SDK}
	ndkPath = {json.dumps(str(Path(ndk).resolve()))}
	defaultConfig {{
		applicationId = "{APP}"
		minSdk = {ANDROID_API}
		targetSdk = {APP_SDK}
		versionCode = 1
		// The libraries' debug info, which the bundle gives Play to symbolicate a crash by.
		ndk {{ debugSymbolLevel = "FULL" }}
	}}
	buildTypes {{
		release {{
			isMinifyEnabled = true
			proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "app.pro")
			signingConfig = signingConfigs.getByName("debug")
		}}
	}}
}}

dependencies {{
	implementation("{group}:{artifact}:{version}")
}}
""", encoding="utf-8")
        run("gradle", "--no-daemon", "--console", "plain", "--stacktrace", "assembleRelease", "bundleRelease", cwd=app)
        bundles = list((app / "build" / "outputs" / "bundle" / "release").glob("*.aab"))
        if len(bundles) != 1:
            raise SystemExit(f"the consumer's build made no single bundle: {bundles}")
        # What the app's build gives Play from each library it stripped: its line tables.
        with zipfile.ZipFile(bundles[0]) as bundle:
            for name in libraries:
                symbols = f"BUNDLE-METADATA/com.android.tools.build.debugsymbols/{name.removeprefix('jni/')}.dbg"
                if symbols not in bundle.namelist() or b".debug_line" not in bundle.read(symbols):
                    raise SystemExit(f"the app's bundle holds no line tables of {name} for Play: {symbols}")
        apks = list((app / "build" / "outputs" / "apk" / "release").glob("*.apk"))
        if len(apks) != 1:
            raise SystemExit(f"the consumer's build made no single APK: {apks}")
        entry = []
        if check:
            # The class the compiler made check.kt's main a member of, whatever its package,
            # @file:JvmName, or layout, among the app's own classes the build compiled.
            classes = sorted(str(p) for p in (app / "build").rglob("*.class"))
            listing = output(jdk("javap"), *classes) if classes else ""
            mains = main_classes(listing, "check.kt")
            if len(mains) != 1:
                raise SystemExit(f"the consumer's build compiled no single main from check.kt: {mains}")
            entry = ["-e", "main", mains[0]]
        adb("install", "-r", apks[0])
    instrument("run", *entry)
    report = instrument("report")
    if "status" not in report:
        raise SystemExit(f"the consumer's instrumentation reported no status: {report}")
    out, err = (base64.b64decode(report.get(key, "")) for key in ("out", "err"))
    sys.stderr.buffer.write(err)
    if report["status"] != "0":
        raise SystemExit(f"{check or 'the load'} failed in the Android app: {report['status']}")
    if native:
        compare(android_native(native, version_dir / f"{artifact}-{version}.aar", abi), out, "Android app")


def android_native(example, aar, abi):
    """The output on the device of the example in the directory `example`, beside the libraries
    it needs, run beside the AAR's libraries for its ABI, if any."""
    device = "/data/local/tmp/rust2-example"
    with tempfile.TemporaryDirectory() as raw:
        staged = Path(raw) / "example"
        shutil.copytree(example, staged)
        with zipfile.ZipFile(aar) if aar else contextlib.nullcontext() as z:
            for name in z.namelist() if z else []:
                if name.startswith(f"jni/{abi}/"):
                    (staged / posixpath.basename(name)).write_bytes(z.read(name))
        adb("shell", "rm", "-rf", device)
        adb("push", staged, device, capture_output=True)
    # No terminal, which would end each line with a carriage return.
    result = adb("shell", "-T", f"cd {device} && chmod 755 example && LD_LIBRARY_PATH={device} ./example",
                 capture_output=True)
    return result.stdout


# The consumer's Kotlin Gradle plugin: the kotlinc the classes are compiled with.
KOTLIN_GRADLE = "2.4.21"


def consume_jvm(repo, check, native):
    """A new Gradle JVM app depending on the release from the Maven repository `repo`, as an
    app resolves it from Central, which must choose the jar, run with the classpath Gradle
    resolved: the check program, or else a load of this host's library. With `native`,
    native Rust's output here, the check prints exactly that."""
    if native and not check:
        raise SystemExit("an example's output is compared with a check's: give a check with it")
    group, artifact, version, version_dir = maven_coordinates(repo)
    with zipfile.ZipFile(version_dir / f"{artifact}-{version}.jar") as jar:
        natives = [n for n in jar.namelist() if n.endswith("/_Native.class")]
    if len(natives) != 1:
        raise SystemExit(f"the jar holds no single _Native: {natives}")
    package = natives[0].removesuffix("/_Native.class").replace("/", ".")
    gradle = shutil.which("gradle")
    if not gradle:
        raise SystemExit("the JVM consumer needs Gradle on PATH")
    with tempfile.TemporaryDirectory() as raw:
        app = Path(raw)
        source = app / "src" / "main" / "kotlin"
        source.mkdir(parents=True)
        if check:
            kotlin_check(check, source)
        else:
            # Loading the library checks its contract against the Kotlin's.
            (source / "check.kt").write_text(f'fun main() {{\n\tClass.forName("{package}._Native")\n}}\n', encoding="utf-8")
        (app / "settings.gradle.kts").write_text(f"""pluginManagement {{
	repositories {{
		mavenCentral()
	}}
}}

dependencyResolutionManagement {{
	repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
	repositories {{
		// The release's repository serves its group alone, as Central would.
		exclusiveContent {{
			forRepository {{ maven({json.dumps(Path(repo).resolve().as_uri())}) }}
			filter {{ includeGroup("{group}") }}
		}}
		mavenCentral()
	}}
}}

rootProject.name = "check"
""", encoding="utf-8")
        (app / "build.gradle.kts").write_text(f"""plugins {{
	kotlin("jvm") version "{KOTLIN_GRADLE}"
}}

dependencies {{
	implementation("{group}:{artifact}:{version}")
}}

// The classpath the app runs with, as `run` would pass it.
val runtime: FileCollection = configurations.runtimeClasspath.get()
val listing = layout.buildDirectory.file("classpath.txt")
tasks.register("classpath") {{
	inputs.files(runtime)
	outputs.file(listing)
	doLast {{ listing.get().asFile.writeText(runtime.joinToString("\\n")) }}
}}
""", encoding="utf-8")
        run(gradle, "--no-daemon", "--console", "plain", "--stacktrace", "classes", "classpath", cwd=app)
        classpath = (app / "build" / "classpath.txt").read_text(encoding="utf-8").splitlines()
        names = sorted(Path(p).name for p in classpath)
        if f"{artifact}-{version}.jar" not in names or any(n.endswith(".aar") for n in names):
            raise SystemExit(f"Gradle resolved {group}:{artifact} on the JVM to {names}, not its jar")
        compiled = app / "build" / "classes" / "kotlin" / "main"
        mains = main_classes(output(jdk("javap"), *sorted(str(p) for p in compiled.rglob("*.class"))), "check.kt")
        if len(mains) != 1:
            raise SystemExit(f"the consumer's build compiled no single main from check.kt: {mains}")
        result = subprocess.run([jdk("java"), "-cp", os.pathsep.join([str(compiled), *classpath]), mains[0]],
                                capture_output=True, cwd=app)
    sys.stderr.buffer.write(result.stderr)
    if result.returncode:
        raise SystemExit(f"{check or 'the load'} failed in the JVM app with status {result.returncode}")
    if native:
        # Kotlin's println ends a line with the platform's separator, Rust's with LF.
        actual = result.stdout.replace(b"\r\n", b"\n") if sys.platform == "win32" else result.stdout
        compare(Path(native).read_bytes(), actual, "JVM app")


def line_tables(manifest):
    """An app's native build with line tables at least, from which a crash in Rust symbolicates:
    the Apple slices', whose app's dSYM holds them, and Android's, whose app's build strips
    them for Play. As cargo's `--config` arguments, which take precedence over the crate's
    release profile, which the export carries, the environment, and any `.cargo/config.toml`,
    which this cannot read: the profile's level pinned, `line-tables-only` unless the profile or
    environment sets one holding them, which it keeps, and no strip, which a static library
    ignores and a shared one would lose them to. A package's override of either is overridden
    likewise where the profile sets one."""
    with open(manifest, "rb") as f:
        release = tomllib.load(f).get("profile", {}).get("release", {})
    root = {**release, **{key: os.environ[f"CARGO_PROFILE_RELEASE_{key.upper()}"] for key in ("debug", "strip")
                          if f"CARGO_PROFILE_RELEASE_{key.upper()}" in os.environ}}
    config = ["--config", f'profile.release.debug="{symbolicating(root.get("debug")) or "line-tables-only"}"',
              "--config", 'profile.release.strip="none"']
    for name, table in release.get("package", {}).items():
        key = f"profile.release.package.{json.dumps(name)}"
        if "debug" in table and not symbolicating(table["debug"]):
            config += ["--config", f'{key}.debug="line-tables-only"']
        if table.get("strip", "none") not in (False, "false", "none"):
            config += ["--config", f'{key}.strip="none"']
    return config


def symbolicating(level):
    """Cargo's name for a debug level, as a profile or the environment spells it, that holds
    line tables, or None for one that holds none."""
    if level is True or level in (2, "2", "true", "full"):
        return "full"
    if level in (1, "1", "limited"):
        return "limited"
    return level if level == "line-tables-only" else None


def web_profile(manifest):
    """The web module's build at `opt-level = "s"`, which a page downloads, unless the crate's
    release profile, which the export carries, chooses its own level."""
    with open(manifest, "rb") as f:
        release = tomllib.load(f).get("profile", {}).get("release", {})
    return {} if "opt-level" in release else {"CARGO_PROFILE_RELEASE_OPT_LEVEL": "s"}


def web(args):
    """The web binding's npm tarball and JSR package, and native Rust's output on its target."""
    info = facts(args.manifest_path)
    if args.version and info["version"] != args.version:
        raise SystemExit(f"{info['name']} is {info['version']}, not the release's {args.version}")
    out = Path(args.out).resolve()
    export_dir = out / "export"
    exporter(args.manifest_path, args.features, "--target", WEB_TARGET, "--out", export_dir)
    manifest = Path(args.manifest_path).resolve()
    run("rustup", "target", "add", WEB_TARGET, cwd=manifest.parent)
    with linked(args.manifest_path, export_dir, "--features", "web") as spell:
        through, inside = spell(export_dir), {**info, "workspace": str(spell(info["workspace"]))}
        built = cargo(args.manifest_path, "build", "--locked", "--release", "--manifest-path", through / "Cargo.toml",
                      "--target", WEB_TARGET, "--target-dir", through / "target", "--features", "web",
                      env={**reproducible(inside, through, WEB_TARGET), **web_profile(export_dir / "Cargo.toml")})
    sys.stderr.write(built.stderr)
    wasm = export_dir / "target" / WEB_TARGET / "release" / f"{info['lib']}.wasm"
    npm, _ = web_package(info, export_dir / "web", wasm, args.scope, out)
    # Its own, as the web crate's dependencies are not the native export's.
    (out / "sbom.cdx.json").write_bytes(sbom(info, export_dir / "Cargo.toml", "web"))
    packed = output("npm", "pack", "--silent", "--pack-destination", out, cwd=npm).strip()
    if not (out / packed).is_file():
        raise SystemExit(f"npm pack wrote no {packed}")
    shutil.rmtree(export_dir)
    if args.example:
        if args.example not in info["examples"]:
            raise SystemExit(f"{info['name']} has no example {args.example}: it has {info['examples']}")
        features = ["--features", args.features] if args.features.strip() else []
        target_dir = out / "native-target"
        cargo(args.manifest_path, "build", "--locked", "--release", "--manifest-path", manifest, "--example",
              args.example, "--target", WEB_TARGET, "--target-dir", target_dir, *features)
        program = target_dir / WEB_TARGET / "release" / "examples" / f"{args.example}.wasm"
        native = run("node", "--no-warnings", "--input-type=module", "-e", WASI_RUN, program, capture_output=True)
        (out / "native.txt").write_bytes(native.stdout)
        shutil.rmtree(target_dir)


def check_program(check, name, binding="web"):
    """A check's module written beside an export, importing `./<binding>/<module>.ts`, the
    web's or React Native's, as it imports the package."""
    text = Path(check).read_text(encoding="utf-8")
    # Whitespace and comments, which may separate the keyword from its specifier.
    gap = r"(?:\s|/\*[\s\S]*?\*/|//[^\n]*\n)*"
    # Only a specifier, after `from` or `import`, in either quote, kept as written: a string
    # the check prints stays as the transcript has it.
    text = re.sub(rf"""(\b(?:from|import){gap}(?:\({gap})?)(["'])\./{binding}/([A-Za-z0-9_$]+)\.ts\2""",
                  lambda m: m.group(1) + m.group(2) + (name if m.group(3) == "index" else f"{name}/{m.group(3)}") + m.group(2),
                  text)
    if re.search(rf"""\b(?:from|import){gap}(?:\({gap})?["']\./{binding}/""", text):
        raise SystemExit(f"{check} imports the binding by a path other than ./{binding}/<module>.ts")
    return text


def consume_web(dist, check, native):
    """Install the npm tarball into a new project, as an app would, and run the check, or else
    a load, on Node, Deno, and Bun, and bundled by Vite and by webpack; and check the JSR
    package as publishing it would."""
    dist = Path(dist).resolve()
    tarballs = list(dist.glob("*.tgz"))
    if len(tarballs) != 1:
        raise SystemExit(f"{dist} holds {len(tarballs)} npm tarballs")
    name = json.loads((dist / "npm" / "package.json").read_text(encoding="utf-8"))["name"]
    if check:
        check_program_in(check, ("check.ts",))
    expected = Path(native).read_bytes() if native and check else None
    with tempfile.TemporaryDirectory() as clean:
        clean = Path(clean)
        # The check's directory whole, each TypeScript module in it importing the package;
        # `check.ts` runs, reaching the rest by path. The project's own files then replace any
        # it holds of the same names.
        if check:
            shutil.copytree(check, clean, dirs_exist_ok=True)
            for module in clean.rglob("*.ts"):
                module.write_text(check_program(module, name), encoding="utf-8")
        else:
            (clean / "check.ts").write_text(
                f"import * as binding from {json.dumps(name)};\n\nawait binding.load();\n", encoding="utf-8")
        manifest = {"type": "module", "private": True, "dependencies": {name: f"file:{tarballs[0]}"},
                    "devDependencies": BUNDLERS}
        (clean / "package.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
        run("npm", "install", "--no-audit", "--no-fund", "--ignore-scripts", cwd=clean, capture_output=True)
        # The bundles target the browser, whose `document` Vite's preload polyfill reads;
        # each runs on Node from where the bundler wrote it, a relative base reaching its module.
        (clean / "index.html").write_text('<!doctype html><script type="module" src="./check.ts"></script>\n', encoding="utf-8")
        (clean / "vite.config.js").write_text(
            'export default { base: "./", build: { target: "es2024", modulePreload: { polyfill: false } } };\n', encoding="utf-8")
        # webpack reads the check's TypeScript through Node's own type stripping, as Node runs it.
        (clean / "strip-types.cjs").write_text(
            'module.exports = (source) => require("node:module").stripTypeScriptTypes(source);\n', encoding="utf-8")
        (clean / "webpack.config.js").write_text(
            'export default { mode: "production", entry: "./check.ts", target: ["web", "es2024"],'
            r' module: { rules: [{ test: /\.ts$/, exclude: /node_modules/, loader: new URL("./strip-types.cjs", import.meta.url).pathname }] },'
            ' experiments: { outputModule: true },'
            ' output: { module: true, path: new URL("./webpack", import.meta.url).pathname, filename: "check.js" } };\n',
            encoding="utf-8")
        run("npx", "--no-install", "vite", "build", "--logLevel", "warn", cwd=clean, capture_output=True)
        run("npx", "--no-install", "webpack", "--no-stats", cwd=clean, capture_output=True)
        bundled = list((clean / "dist" / "assets").glob("*.js"))
        if len(bundled) != 1:
            raise SystemExit(f"Vite wrote {len(bundled)} scripts")
        consumers = {
            "Node": ["node", "check.ts"],
            "Deno": ["deno", "run", "--allow-read", "--allow-env", "check.ts"],
            "Bun": ["bun", "check.ts"],
            "Vite": ["node", bundled[0]],
            "webpack": ["node", "webpack/check.js"],
        }
        # JSR's own checks of the TypeScript package, its types against Deno's included.
        run("deno", "publish", "--dry-run", "--allow-dirty", cwd=dist / "jsr", capture_output=True,
            env={**os.environ, "NO_COLOR": "1", "DENO_NO_UPDATE_CHECK": "1"})
        for consumer, command in consumers.items():
            result = subprocess.run([str(a) for a in command], capture_output=True, cwd=clean,
                                    env={**os.environ, "NO_COLOR": "1", "DENO_NO_UPDATE_CHECK": "1"})
            sys.stderr.buffer.write(result.stderr)
            if result.returncode:
                raise SystemExit(f"{consumer} failed on the installed package with status {result.returncode}")
            if expected is not None:
                compare(expected, result.stdout, f"{consumer}'s installed package")


def rn_package_name(info, scope):
    """React Native's package name on npm, `@<scope>/<crate>-rn`, beside the web's `@<scope>/<crate>`,
    refusing what npm cannot spell."""
    name = f"@{scope}/{info['name']}-rn"
    if not re.fullmatch(r"@[a-z0-9][a-z0-9._-]*/[a-z0-9][a-z0-9._-]*", name) or len(name) > 214:
        raise SystemExit(f"{name} is no npm package name: a scope and crate name of lowercase letters, digits, hyphens, dots, and underscores")
    return name


def native_flags(flags, what, frameworks=False):
    """The system libraries, and on Apple frameworks, rustc's `native-static-libs` names, in its
    order, once each, as (kind, name) pairs."""
    found = []
    args = iter(shlex.split(flags))
    for flag in args:
        if flag.startswith("-l") and len(flag) > 2:
            item = ("library", flag[2:])
        elif frameworks and flag == "-framework":
            item = ("framework", next(args))
        else:
            raise SystemExit(f"unsupported native linker flag on {what}: {flag}")
        if item not in found:
            found.append(item)
    return found


def agreed(natives, what):
    """The one set of native flags every slice of `what` links with, from each slice's native.txt."""
    found = {Path(path).read_text(encoding="utf-8").strip() for path in natives}
    if len(found) != 1:
        raise SystemExit(f"{what}'s slices link different native libraries: {sorted(found)}")
    return found.pop()


def rn_cmake(text, lib, flags):
    """android/CMakeLists.txt linking the module with the system libraries rustc links the Rust library with, beside its own."""
    anchor = f"/../libs/android/${{ANDROID_ABI}}/lib{lib}.a log dl m)"
    if text.count(anchor) != 1:
        raise SystemExit("android/CMakeLists.txt no longer links the Rust library with log, dl, and m: update the packager")
    names = ["log", "dl", "m"]
    names += [name for _, name in native_flags(flags, "Android") if name not in names]
    return text.replace(anchor, f"/../libs/android/${{ANDROID_ABI}}/lib{lib}.a {' '.join(names)})")


def rn_podspec(text, lib, flags, bundle):
    """The podspec linking the XCFramework's frameworks and system libraries, as rustc names them,
    and bundling the privacy manifest as the resource bundle `bundle`, since a static library holds none."""
    vendored = f's.vendored_frameworks = "libs/ios/{lib}.xcframework"\n'
    anchor = "  install_modules_dependencies(s)\n"
    if text.count(vendored) != 1 or text.count(anchor) != 1:
        raise SystemExit("the podspec no longer vendors the XCFramework before its dependencies: update the packager")
    found = native_flags(flags, "iOS", frameworks=True)
    lines = [f"  s.frameworks = {json.dumps([n for kind, n in found if kind == 'framework'])}\n",
             f"  s.libraries = {json.dumps([n for kind, n in found if kind == 'library'])}\n",
             f"  s.resource_bundles = {{ {json.dumps(bundle)} => [\"ios/PrivacyInfo.xcprivacy\"] }}\n"]
    return text.replace(anchor, "".join(lines) + anchor)


def tree_bytes(root):
    """Every file under `root`, by its relative path, with its bytes."""
    return {p.relative_to(root).as_posix(): p.read_bytes() for p in sorted(Path(root).rglob("*")) if p.is_file()}


def react_native_package(info, manifest_path, export, android, ios, natives, scope, sbom_path, out):
    """React Native's npm package: the export's `react-native/` with its libraries built for every
    ABI it serves, Android's static libraries and an XCFramework of iOS's, linked with what
    rustc names, the iOS libraries' privacy manifest, and Cargo's facts; and its tarball."""
    if sys.platform != "darwin":
        raise SystemExit("React Native's XCFramework needs Apple's xcodebuild, on macOS")
    if set(ios) != set(RN_IOS_TARGETS) or set(natives) != set(RN_IOS_TARGETS):
        raise SystemExit(f"React Native's package ships {sorted(RN_IOS_TARGETS)} on iOS, given {sorted(ios)} and {sorted(natives)}")
    lib, name = info["lib"], rn_package_name(info, scope)
    source, android, out = Path(export) / "react-native", Path(android), Path(out).resolve()
    manifest = json.loads((source / "package.json").read_text(encoding="utf-8"))
    if manifest["version"] != info["version"]:
        raise SystemExit(f"the export's react-native/package.json is {manifest['version']}, not the crate's {info['version']}")
    podspecs = list(source.glob("*.podspec"))
    if len(podspecs) != 1:
        raise SystemExit(f"{source} holds no single podspec: {podspecs}")
    if out.exists():
        shutil.rmtree(out)
    package = out / "npm"
    # The export's, without any libraries a local build put in it: the package's are these.
    shutil.copytree(source, package, ignore=lambda d, _: ["libs"] if Path(d) == source else [])
    for path in licenses(info):
        shutil.copy(path, package)
    # Each library without debug info or LTO bitcode, as the Go module's: an unlinked static
    # library's DWARF, with its relocations, is most of its bytes, and moq's tarball would be
    # twice the 200 MB npm takes.
    objcopy = llvm_tool(manifest_path, "llvm-objcopy")
    strip = lambda library, dest: run(objcopy, "--strip-debug", *(f"--remove-section={s}" for s in LTO_SECTIONS),  # noqa: E731
                                      library, dest)
    # Android's: one static library per ABI, linked with the system libraries rustc names.
    for target, abi in RN_ANDROID_TARGETS.items():
        found = android / abi / f"lib{lib}.a"
        if not found.is_file():
            raise SystemExit(f"React Native's package ships {abi}, but {android} holds no {found.name} for it")
        (package / "libs" / "android" / abi).mkdir(parents=True)
        strip(found, package / "libs" / "android" / abi / found.name)
    cmake = package / "android" / "CMakeLists.txt"
    flags = agreed([android / abi / "native.txt" for abi in RN_ANDROID_TARGETS.values()], "Android")
    cmake.write_text(rn_cmake(cmake.read_text(encoding="utf-8"), lib, flags), encoding="utf-8")
    # iOS's: each slice at React Native's floor, stripped, what an app's link of it calls of
    # Apple's required-reason APIs, and one XCFramework of both, made twice, which must be the
    # same bytes, as a rerun of the release must pack the same tarball.
    found, slices = {}, {}
    with tempfile.TemporaryDirectory() as raw:
        work = Path(raw)
        for target in RN_IOS_TARGETS:
            floor = APPLE_BUILD[target]
            if min_os(ios[target]) > tuple(int(x) for x in floor.split(".")):
                raise SystemExit(f"{ios[target]} needs a newer iOS than React Native's {floor}: build it with the deployment target set")
            slice_dir = work / target
            slice_dir.mkdir()
            slices[target] = slice_dir / f"lib{lib}.a"
            strip(ios[target], slices[target])
            probe = probe_link(lib, slices[target], target, Path(natives[target]).read_text(encoding="utf-8").strip(),
                               slice_dir, floor)
            for category, apis in required_apis(probe).items():
                found.setdefault(category, set()).update(apis)
        frameworks = []
        for tree in (package / "libs" / "ios" / f"{lib}.xcframework", work / "again" / f"{lib}.xcframework"):
            args = ["xcodebuild", "-create-xcframework"]
            for target in RN_IOS_TARGETS:
                args += ["-library", slices[target]]
            # libtool, which xcodebuild runs on each library, stamps the time unless told not to.
            run(*args, "-output", tree, env={**os.environ, "ZERO_AR_DATE": "1"}, capture_output=True)
            sort_plist(tree / "Info.plist")
            frameworks.append(tree_bytes(tree))
        if frameworks[0] != frameworks[1]:
            raise SystemExit("xcodebuild made different XCFrameworks from the same libraries: "
                             f"{sorted(k for k in frameworks[0].keys() | frameworks[1].keys() if frameworks[0].get(k) != frameworks[1].get(k))}")
    (package / "ios" / "PrivacyInfo.xcprivacy").write_bytes(privacy_manifest(lib, found))
    podspec = package / podspecs[0].name
    flags = agreed([natives[t] for t in RN_IOS_TARGETS], "iOS")
    podspec.write_text(rn_podspec(podspec.read_text(encoding="utf-8"), lib, flags, f"{podspec.stem}Privacy"),
                       encoding="utf-8")
    # What Cargo says of the package, beside what the export wrote, as the web's.
    said = {"name": name, "version": info["version"]}
    if info["description"]:
        said["description"] = " ".join(info["description"].split())
    if info["license"]:
        said["license"] = info["license"]
    if info["repository"]:
        said["repository"] = {"type": "git", "url": f"git+{info['repository']}"}
    if info["homepage"]:
        said["homepage"] = info["homepage"]
    manifest = {**said, **{k: v for k, v in manifest.items() if k not in said}}
    manifest["files"] = manifest["files"] + [p.name for p in licenses(info) if p.name not in manifest["files"]]
    (package / "package.json").write_text(json.dumps(manifest, indent="\t") + "\n", newline="\n", encoding="utf-8")
    shutil.copy(sbom_path, out / "sbom.cdx.json")
    packed = output("npm", "pack", "--silent", "--pack-destination", out, cwd=package).strip()
    if not (out / packed).is_file():
        raise SystemExit(f"npm pack wrote no {packed}")
    print(f"{packed}: {(out / packed).stat().st_size} bytes", file=sys.stderr)
    return out / packed


# The consumer app's entry: it runs the check once its component first renders, and logs what it
# printed as `rust2: start`, base64 chunks of it, then `rust2: exit <code>` or `rust2: threw
# <error>`, by hand, as Hermes may lack TextEncoder and btoa. React Native and the package come
# from Metro, through `check/imports.js`, which the bundler writes with the check's bundle.
RN_INDEX = r"""import { AppRegistry } from "react-native";

const log = console.log;
let out = "";
let ended = false;

function base64(text) {
	const bytes = [];
	for (const char of text) {
		let c = char.codePointAt(0);
		if (c < 0x80) bytes.push(c);
		else if (c < 0x800) bytes.push(0xc0 | (c >> 6), 0x80 | (c & 63));
		else if (c < 0x10000) bytes.push(0xe0 | (c >> 12), 0x80 | ((c >> 6) & 63), 0x80 | (c & 63));
		else bytes.push(0xf0 | (c >> 18), 0x80 | ((c >> 12) & 63), 0x80 | ((c >> 6) & 63), 0x80 | (c & 63));
	}
	const digits = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
	let encoded = "";
	for (let i = 0; i < bytes.length; i += 3) {
		const n = (bytes[i] << 16) | ((bytes[i + 1] ?? 0) << 8) | (bytes[i + 2] ?? 0);
		encoded += digits[n >> 18] + digits[(n >> 12) & 63];
		encoded += i + 1 < bytes.length ? digits[(n >> 6) & 63] : "=";
		encoded += i + 2 < bytes.length ? digits[n & 63] : "=";
	}
	return encoded;
}

function report(end) {
	if (ended) return;
	ended = true;
	const data = base64(out);
	log("rust2: start");
	for (let i = 0; i < data.length; i += 800) log(`rust2: ${data.slice(i, i + 800)}`);
	log(`rust2: ${end}`);
}

function Check() {
	if (globalThis.__rust2_import === undefined) {
		const imports = require("./check/imports.js");
		globalThis.__rust2_import = (path) => imports[path];
		console.log = (...args) => {
			out += `${args.join(" ")}\n`;
		};
		globalThis.process = Object.assign(globalThis.process ?? {}, {
			argv: ["react-native", "check"],
			exit(code) {
				report(`exit ${code ?? 0}`);
				throw new Error("__exited");
			},
			stderr: {
				write(text) {
					log(`rust2! ${text}`);
					return true;
				},
			},
		});
		globalThis.__rust2_end = (error) => {
			if (error === undefined) report("exit 0");
			else if (!(error instanceof Error && error.message === "__exited"))
				report(`threw ${error instanceof Error ? (error.stack ?? error.message) : String(error)}`.replace(/\n/g, " | "));
		};
		require("./check/bundle.js");
	}
	return null;
}

AppRegistry.registerComponent("Consumer", () => Check);
"""

# Bundles the check, `check/check.ts`, into `check/bundle.js`, its top-level await in an async
# function, as Metro takes no top-level await: each import of React Native or the package, named
# by the first argument, is left to Metro, through the map `check/imports.js` requires. The entry
# imports the check rather than being it, so nothing the check exports stays an `export`, which
# the function could not hold.
RN_BUNDLE = r"""import * as esbuild from "esbuild";
import { writeFileSync } from "node:fs";

const name = process.argv[2];
const imported = new Set(["react-native", name]);
const result = await esbuild.build({
	stdin: { contents: 'import "./check.ts";', resolveDir: "check", sourcefile: "entry.ts", loader: "ts" },
	bundle: true,
	write: false,
	format: "esm",
	target: "es2022",
	platform: "neutral",
	define: { "import.meta.url": "undefined" },
	logLevel: "error",
	plugins: [
		{
			name: "metro",
			setup(build) {
				build.onResolve({ filter: /.*/ }, (args) => {
					if (args.path !== "react-native" && args.path !== name && !args.path.startsWith(`${name}/`)) return undefined;
					imported.add(args.path);
					return { path: args.path, namespace: "metro" };
				});
				build.onLoad({ filter: /.*/, namespace: "metro" }, (args) => ({
					contents: `module.exports = globalThis.__rust2_import(${JSON.stringify(args.path)});`,
					loader: "js",
				}));
			},
		},
	],
});
const [code] = result.outputFiles;
writeFileSync("check/bundle.js", `(async () => {\n${code.text}\n})().then(() => globalThis.__rust2_end(), (e) => globalThis.__rust2_end(e ?? new Error(String(e))));\n`);
const entries = [...imported].sort().map((path) => `\t${JSON.stringify(path)}: require(${JSON.stringify(path)}),`);
writeFileSync("check/imports.js", `module.exports = {\n${entries.join("\n")}\n};\n`);
"""


def rn_app(work, tarball, name, check):
    """A new React Native app at the release the package is tested with, depending on the
    tarball, whose entry runs the check, or else a load, bundled for Metro."""
    run("npx", "--yes", f"@react-native-community/cli@{RN_CLI}", "init", "Consumer", "--version", REACT_NATIVE,
        "--skip-git-init", "--install-pods", "false", "--package-name", FLUTTER_APP, "--pm", "npm", cwd=work,
        capture_output=True)
    app = work / "Consumer"
    run("npm", "install", "--no-audit", "--no-fund", tarball, f"esbuild@{ESBUILD}", cwd=app, capture_output=True)
    checks = app / "check"
    if check:
        shutil.copytree(check, checks)
        for module in checks.rglob("*.ts"):
            module.write_text(check_program(module, name, "react-native"), encoding="utf-8")
    else:
        checks.mkdir()
        (checks / "check.ts").write_text(f"import * as binding from {json.dumps(name)};\n\nawait binding.load();\n",
                                         encoding="utf-8")
    (app / "bundle.mjs").write_text(RN_BUNDLE, encoding="utf-8")
    run("node", "bundle.mjs", name, cwd=app)
    (app / "index.js").write_text(RN_INDEX, encoding="utf-8")
    return app


def consume_react_native(dist, platform, check, native):
    """A new React Native app depending on the npm tarball, as an app installs it, built for
    release: on Android for each of React Native's ABIs and run on the device adb reaches, on
    iOS for a device and for the simulator, run on one booted for it. It runs the check, or
    else a load; with `native`, the example built for the device, whose output it must print."""
    dist = Path(dist).resolve()
    tarballs = list(dist.glob("*.tgz"))
    if len(tarballs) != 1:
        raise SystemExit(f"{dist} holds {len(tarballs)} npm tarballs")
    if native and not check:
        raise SystemExit("an example's output is compared with a check's: give a check with it")
    name = json.loads((dist / "npm" / "package.json").read_text(encoding="utf-8"))["name"]
    if check:
        check_program_in(check, ("check.ts",))
    with tempfile.TemporaryDirectory() as raw:
        app = rn_app(Path(raw), tarballs[0], name, check)
        if platform == "android":
            printed, expected = rn_android(app, native)
        else:
            printed, expected = rn_ios(app, native)
    if native:
        compare(expected, printed, f"React Native app on {platform}")
    print(f"React Native app on {platform}: ok")


def rn_android(app, native):
    """The app built for release for every ABI, its check's output on the device adb reaches,
    and the example's there."""
    run("./gradlew", "--console", "plain", "assembleRelease", cwd=app / "android")
    apk = app / "android" / "app" / "build" / "outputs" / "apk" / "release" / "app-release.apk"
    with zipfile.ZipFile(apk) as z:
        abis = {n.split("/")[1] for n in z.namelist() if n.startswith("lib/")}
    if abis != set(RN_ANDROID_TARGETS.values()):
        raise SystemExit(f"the app's APK holds {sorted(abis)}, not each of React Native's ABIs")
    device = adb("get-serialno", capture_output=True, encoding="utf-8").stdout.strip()
    abi = adb("-s", device, "shell", "getprop", "ro.product.cpu.abi", capture_output=True, encoding="utf-8").stdout.strip()
    adb("-s", device, "install", "-r", "--no-streaming", apk, capture_output=True)
    logcat = Logcat(device, "ReactNativeJS", "React Native app")
    try:
        # `am start` exits 0 whether or not it starts the app.
        started = adb("-s", device, "shell", "am", "start", "-n", f"{FLUTTER_APP}/.MainActivity",
                      capture_output=True, encoding="utf-8").stdout
        if re.search(r"Error|Exception", started):
            raise SystemExit(f"the React Native app did not start: {started}")
        printed = logcat.output()
    finally:
        logcat.process.kill()
    return printed, android_native(native, None, abi) if native else None


def rn_ios(app, native):
    """The app built for release for a device and for the simulator, its check's output on a
    simulator booted for it, and the example's there."""
    ios = app / "ios"
    run("pod", "install", cwd=ios, capture_output=True)
    built = app / "xcode"
    build = ["xcodebuild", "-workspace", "Consumer.xcworkspace", "-scheme", "Consumer", "-configuration", "Release",
             "-quiet", "-derivedDataPath", built]
    # The package's simulator slice is arm64's alone, as the Swift package's is.
    run(*build, "-destination", "generic/platform=iOS Simulator", "ARCHS=arm64", "build", cwd=ios)
    run(*build, "-destination", "generic/platform=iOS", "CODE_SIGNING_ALLOWED=NO", "build", cwd=ios)
    devices = json.loads(output("xcrun", "simctl", "list", "devices", "available", "--json"))["devices"]
    phones = [d for runtime, found in sorted(devices.items()) if "iOS" in runtime for d in found if "iPhone" in d["name"]]
    if not phones:
        raise SystemExit("no iPhone simulator is available")
    udid = phones[-1]["udid"]
    if phones[-1]["state"] != "Booted":
        run("xcrun", "simctl", "boot", udid)
    run("xcrun", "simctl", "bootstatus", udid, "-b", capture_output=True)
    run("xcrun", "simctl", "install", udid, built / "Build" / "Products" / "Release-iphonesimulator" / "Consumer.app")
    # `log stream` buffers what it writes to a pipe, so it writes to a terminal of this one's.
    import pty
    leader, follower = pty.openpty()
    stream = subprocess.Popen(["xcrun", "simctl", "spawn", udid, "log", "stream", "--level", "info", "--style",
                               "compact", "--predicate",
                               'subsystem == "com.facebook.react.log" AND category == "javascript"'],
                              stdin=subprocess.DEVNULL, stdout=follower, stderr=follower)
    os.close(follower)
    lines = reader(os.fdopen(leader, encoding="utf-8", errors="replace"), lambda line: [line])
    try:
        # The line it prints as it starts reading, before the app's first: a launch that beat
        # it would lose the app's lines, failing the check rather than passing it.
        while not take(lines, "the simulator's log").startswith("Filtering the log data"):
            pass
        run("xcrun", "simctl", "launch", "--terminate-running-process", udid, FLUTTER_APP, capture_output=True)

        def said():
            line = take(lines, "the simulator's log")
            print(line, file=sys.stderr)
            return line[line.find("rust2: "):] if "rust2: " in line else line
        printed = check_output(said, "React Native app")
    finally:
        stream.kill()
        stream.wait()
    expected = None
    if native:
        # An artifact keeps no file mode. No terminal, which would end each line with a carriage return.
        example = Path(native) / "example"
        example.chmod(0o755)
        expected = run("xcrun", "simctl", "spawn", udid, example, capture_output=True).stdout
    return printed, expected


def template(info, manifest, rust2, mirror, go_module, scope=None, group=None):
    """The caller workflow, with every identity Cargo cannot establish spelled out in it.
    With a scope, it ships the crate's own web package and its React Native package, and with
    a group, its Kotlin library; without, the lines that would are left commented out."""
    if not re.fullmatch(r"[0-9a-f]{40}", rust2):
        raise SystemExit("pin rust2 by a full commit SHA")
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", mirror):
        raise SystemExit(f"the Swift mirror must be OWNER/REPO, not {mirror!r}")
    # A proxy serves only a path whose first element is a host name.
    if not re.fullmatch(r"[a-z0-9-]+(\.[a-z0-9-]+)+(/[A-Za-z0-9._~-]+)*", go_module):
        raise SystemExit(f"the Go module must be a path a proxy serves, such as example.com/name, not {go_module!r}")
    check_major(go_module, go_version(info["version"]))
    indent = "      "
    if scope:
        name = package_name(info, scope)
        rn = rn_package_name(info, scope)
        web = (f"{indent}# The web package, {name} on npm and JSR, and the React Native package, {rn} on npm:\n"
               f"{indent}# delete web-manifest-path to ship no web package, and both lines to ship neither.\n"
               f"{indent}web-manifest-path: {manifest}\n{indent}scope: {scope}")
    else:
        web = (f"{indent}# The React Native package on npm, @<scope>/{info['name']}-rn, and with web-manifest-path, for a\n"
               f"{indent}# crate that builds for wasm32-wasip1, the web package on npm and JSR, @<scope>/{info['name']}:\n"
               f"{indent}# web-manifest-path: {manifest}\n{indent}# scope: <scope>")
    if group:
        if not MAVEN_GROUP.fullmatch(group):
            raise SystemExit(f"the Maven group must be a dotted namespace, such as dev.example, not {group!r}")
        maven = (f"{indent}# The Kotlin binding for desktop JVMs and Android, {group}:{info['name']} on Maven Central: delete this line to ship none.\n"
                 f"{indent}maven-group: {group}")
    else:
        maven = (f"{indent}# The Kotlin binding for desktop JVMs and Android on Maven Central, <group>:{info['name']}, under a namespace you hold there:\n"
                 f"{indent}# maven-group: <group>")
    values = {
        "GO_MODULE": go_module,
        "NAME": info["name"],
        "MANIFEST": manifest,
        "RUST2": rust2,
        "MODULE": pascal(info["lib"]),
        "MIRROR": mirror,
        "MIRROR_OWNER": mirror.split("/")[0],
        "MIRROR_REPO": mirror.split("/")[1],
        "WEB": web,
        "MAVEN": maven,
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
    text = template(info, relative, rust2, args.swift_mirror, args.go_module, args.scope, args.maven_group)
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
                           ("build", "build this host's wheels, C archives, and Apple, Go, Dart, and Kotlin libraries")):
        p = sub.add_parser(name, help=helptext)
        p.add_argument("--manifest-path", required=True)
        p.add_argument("--features", default="")
        p.add_argument("--out", required=True)
        p.add_argument("targets", nargs="+")
        if name == "export":
            p.add_argument("--api", required=True)
            p.add_argument("--baseline", help="the api.json of the release this one must not break")
            p.add_argument("--go-module", default="", help="the Go module path; the export's own, the prefix, by default")
        if name == "build":
            p.add_argument("--export", required=True)
            p.add_argument("--example", default="")
            p.add_argument("--kotlin", action="store_true", help="build the Kotlin binding's libraries, for the desktop jar and the AAR")
            p.add_argument("--react-native", action="store_true", help="build React Native's Android libraries")
    p = sub.add_parser("sdist", help="write the sdist of the export's crate and the local packages it builds with")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p.add_argument("--out", required=True)
    p = sub.add_parser("same", help="fail unless two builds made the same wheels and libraries")
    p.add_argument("first")
    p.add_argument("second")
    p = sub.add_parser("previous", help="fetch the API of the release this version follows")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--repo", required=True, metavar="OWNER/REPO", help="the repository whose releases to read")
    p.add_argument("--out", required=True)
    p = sub.add_parser("changes", help="print the API changelog of one api.json against another")
    p.add_argument("--old", help="the previous release's api.json; none for a first release")
    p.add_argument("--new", required=True)
    p.add_argument("--since", default="", help="the previous release's version")
    p = sub.add_parser("web", help="export, build, and pack the web binding for npm and JSR")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--features", default="")
    p.add_argument("--scope", required=True, help="the npm and JSR scope, without its @")
    p.add_argument("--version", default="", help="the release's version, which the crate's must equal")
    p.add_argument("--example", default="")
    p.add_argument("--out", required=True)
    p = sub.add_parser("consume-web", help="run a check program, or a load, on the installed npm package")
    p.add_argument("--dist", required=True, help="what `web` wrote")
    p.add_argument("--check")
    p.add_argument("--native")
    p = sub.add_parser("react-native", help="write React Native's npm package around its libraries, and pack it")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True, help="an export holding react-native/")
    p.add_argument("--android", required=True, help="a directory of each ABI's static library and native.txt, as build names them")
    p.add_argument("--lib", action="append", default=[], metavar="TARGET=PATH", help="each iOS slice's static library")
    p.add_argument("--native", action="append", default=[], metavar="TARGET=PATH", help="each iOS slice's native.txt")
    p.add_argument("--scope", required=True, help="the npm scope, without its @")
    p.add_argument("--sbom", required=True, help="the react-native.cdx.json build wrote")
    p.add_argument("--out", required=True)
    p = sub.add_parser("consume-react-native", help="run a check program, or a load, in a new React Native app on the npm package")
    p.add_argument("--dist", required=True, help="what `react-native` wrote")
    p.add_argument("--platform", required=True, choices=("android", "ios"),
                   help="android, run on the device adb reaches, or ios, on a simulator it boots")
    p.add_argument("--check", help="a directory holding check.ts beside the modules it imports")
    p.add_argument("--native", help="a directory holding the example built for the device, whose output the check must print")
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
    p = sub.add_parser("privacy-check", help="fail unless a manifest declares each required-reason API a linked binary imports")
    p.add_argument("--manifest", required=True, help="the PrivacyInfo.xcprivacy the app bundles")
    p.add_argument("binary")
    p = sub.add_parser("go", help="write the Go module as a GOPROXY serves it")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p.add_argument("--libs", required=True, help="a directory of every Go target's library, as build names them")
    p.add_argument("--out", required=True)
    p = sub.add_parser("consume-go", help="run a check program, or an import, on the unpacked Go module")
    p.add_argument("--proxy", required=True, help="the directory `go` wrote")
    p.add_argument("--check")
    p.add_argument("--native")
    p = sub.add_parser("maven", help="write the jar, the AAR, and the files Maven Central requires beside them as a Maven repository")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p.add_argument("--libs", required=True, help="a directory holding android/ and jvm/, each target's libraries as build names them")
    p.add_argument("--sbom", required=True, help="the kotlin.cdx.json build wrote")
    p.add_argument("--group", required=True, help="the groupId, a namespace the publisher holds on Central")
    p.add_argument("--out", required=True)
    p = sub.add_parser("consume-android", help="run a check program, or a load, in a Gradle app on the AAR, on the device adb reaches")
    p.add_argument("--repo", required=True, help="the Maven repository `maven` wrote")
    p.add_argument("--check", help="a Kotlin file, or a directory holding it as check.kt beside the files it reads")
    p.add_argument("--native", help="a directory holding the example built for the device, whose output the check must print, beside the libraries it needs")
    p = sub.add_parser("consume-jvm", help="run a check program, or a load, in a Gradle JVM app on the jar")
    p.add_argument("--repo", required=True, help="the Maven repository `maven` wrote")
    p.add_argument("--check", help="a Kotlin file, or a directory holding it as check.kt beside the files it reads")
    p.add_argument("--native", help="native Rust's output on this host, which the check must print")
    p = sub.add_parser("consume-python", help="run a check program, or an import, on the installed wheel")
    group = p.add_mutually_exclusive_group(required=True)
    group.add_argument("--check")
    group.add_argument("--import", dest="module")
    p.add_argument("--native")
    p = sub.add_parser("consume-c", help="build a check program, or a link of each header, on the C archive")
    p.add_argument("--archive", required=True)
    p.add_argument("--check")
    p.add_argument("--native")
    p = sub.add_parser("pub", help="write the pub.dev package, naming each target's Dart library by its hash")
    p.add_argument("--manifest-path", required=True)
    p.add_argument("--export", required=True)
    p.add_argument("--libs", required=True, help="a directory of every Dart target's library, as build names them")
    p.add_argument("--url", required=True, help="where the release hosts the libraries, ending in /")
    p.add_argument("--changes", required=True, help="the release's changes.md")
    p.add_argument("--privacy", required=True, help="a directory of each Apple Dart library's privacy manifest, as build names them")
    p.add_argument("--out", required=True)
    for name, helptext in (("consume-dart", "run a check program, or a load, in new Dart apps on the package"),
                           ("consume-flutter", "run a check program, or a load, in a new Flutter app, then after a hot restart")):
        p = sub.add_parser(name, help=helptext)
        p.add_argument("--package", required=True, help="the directory `pub` wrote")
        p.add_argument("--libs", required=True, help="the libraries, served as the release hosts them")
        p.add_argument("--check")
        p.add_argument("--native")
        if name == "consume-flutter":
            p.add_argument("--device", required=True, help="the Android device adb runs it on")
    p = sub.add_parser("consume-source", help="run a check program, or a load, in a new Dart app on the export's own package")
    p.add_argument("--export", required=True)
    p.add_argument("--check")
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
    p.add_argument("--scope", help="the npm scope of the crate's web and React Native packages, and the web's JSR one, without its @; without it, neither ships")
    p.add_argument("--maven-group", help="the Maven groupId of the crate's Kotlin binding, a namespace you hold on Central; without it, none ships")
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
    elif args.command == "sdist":
        print(sdist(facts(args.manifest_path), args.manifest_path, args.export, args.out))
    elif args.command == "same":
        same(args.first, args.second)
    elif args.command == "previous":
        previous(args)
    elif args.command == "changes":
        new = json.loads(Path(args.new).read_text(encoding="utf-8"))
        if args.old:
            print(changes(json.loads(Path(args.old).read_text(encoding="utf-8")), new, args.since), end="")
        else:
            print("## API\n\nThe first release.\n", end="")
    elif args.command == "web":
        web(args)
    elif args.command == "consume-web":
        consume_web(args.dist, args.check, args.native)
    elif args.command == "react-native":
        pairs = lambda items: dict(item.split("=", 1) for item in items)  # noqa: E731
        print(react_native_package(facts(args.manifest_path), args.manifest_path, args.export, args.android, pairs(args.lib),
                                   pairs(args.native), args.scope, args.sbom, args.out))
    elif args.command == "consume-react-native":
        consume_react_native(args.dist, args.platform, args.check, args.native)
    elif args.command == "digest":
        print(digest(args.export, facts(args.manifest_path)["lib"]))
    elif args.command == "swift":
        info = facts(args.manifest_path)
        pairs = lambda items: dict(item.split("=", 1) for item in items)  # noqa: E731
        swift(info, args.export, pairs(args.lib), pairs(args.native), args.out)
        release_manifest(args.out, args.url)
    elif args.command == "privacy-check":
        privacy_check(args.manifest, args.binary)
    elif args.command == "swift-local":
        local_manifest(args.state, args.package, args.framework)
    elif args.command == "go":
        print(go_module(facts(args.manifest_path), args.export, args.libs, args.out))
    elif args.command == "consume-go":
        consume_go(args.proxy, args.check, args.native)
    elif args.command == "maven":
        print(maven(args))
    elif args.command == "consume-android":
        consume_android(args.repo, args.check, args.native)
    elif args.command == "consume-jvm":
        consume_jvm(args.repo, args.check, args.native)
    elif args.command == "consume-python":
        consume_python(args.check, args.native, args.module)
    elif args.command == "consume-c":
        consume_c(args.archive, args.check, args.native)
    elif args.command == "pub":
        pub_package(facts(args.manifest_path), args.export, args.libs, args.url, args.changes, args.privacy, args.out)
    elif args.command == "consume-dart":
        consume_dart(args.package, args.libs, pub_name(args.package), args.check, args.native)
    elif args.command == "consume-source":
        consume_source(args.export, args.check, args.native)
    elif args.command == "consume-flutter":
        consume_flutter(args.package, args.libs, pub_name(args.package), args.check, args.native, args.device)
    elif args.command == "compare":
        compare(Path(args.native).read_bytes(), Path(args.actual).read_bytes(), "installed Swift package")
    elif args.command == "init":
        init(args)


if __name__ == "__main__":
    main()
