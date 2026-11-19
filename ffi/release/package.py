"""Build and check the Python wheels, Swift package, C archives, and Go module of a crate's rust2-ffi export.

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
import re
import shlex
import shutil
import subprocess
import sys
import tarfile
import tempfile
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
# Every target a wheel and a C archive are built for, each of 8-byte pointers, as the CMake version file checks.
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
COLLECTIONS = "https://github.com/apple/swift-collections"
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


def record(files):
    """RECORD: every file's hash and size, and RECORD's own row unhashed."""
    out = io.StringIO(newline="")
    writer = csv.writer(out, lineterminator="\n")
    for name, data in files.items():
        digest = base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b"=").decode()
        writer.writerow([name, f"sha256={digest}", len(data)])
    return out


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
        f"{lib}/__init__.py": (Path(export) / "python" / f"{lib}.py").read_bytes(),
        f"{lib}/{library_name(lib, target)}": Path(library).read_bytes(),
    }
    for path in licenses(info):
        files[f"{meta}/licenses/{path.name}"] = path.read_bytes()
    files[f"{meta}/METADATA"] = metadata(info, floor).encode()
    files[f"{meta}/WHEEL"] = (
        f"Wheel-Version: 1.0\nGenerator: rust2-ffi\nRoot-Is-Purelib: false\nTag: {tag}\n"
    ).encode()
    rows = record(files)
    csv.writer(rows, lineterminator="\n").writerow([f"{meta}/RECORD", "", ""])
    files[f"{meta}/RECORD"] = rows.getvalue().encode()
    path = Path(out) / f"{dist}-{version}-{tag}.whl"
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, data in files.items():
            archive.writestr(name, data)
    return path


def repair(wheel, target, out):
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
    headers = sorted(p for p in (export / "include").rglob("*") if p.is_file())
    paths = [export / "python" / f"{lib}.py", *headers, *sorted((export / "swift").iterdir()),
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
    args = ["xcodebuild", "-create-xcframework"]
    for target in APPLE_TARGETS:
        args += ["-library", libs[target], "-headers", headers]
    run(*args, "-output", framework)
    shutil.rmtree(headers)
    archive = out / f"{module}FFI.xcframework.zip"
    run("/usr/bin/ditto", "-c", "-k", "--sequesterRsrc", "--keepParent", framework, archive)
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
        run("libtool", "-static", "-o", archive, hidden)
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
    link_args = []
    if linux:
        link_args = ["-C", f"link-arg=-Wl,-soname,{soname}"]
    elif apple:
        # The last -install_name rustc's linker sees wins over its own unversioned one.
        link_args = ["-C", f"link-arg=-Wl,-install_name,@rpath/{soname}"]
    env = {"MACOSX_DEPLOYMENT_TARGET": MACOS_WHEEL} if apple else {}
    # The C library alone, as the Swift package's: the glue's language features add symbols
    # a C program cannot resolve. Its own target directory, since its link arguments differ.
    # cargo-zigbuild's own `rustc`, as Cargo knows only its `zigbuild`, from the package's
    # directory for its toolchain.
    command = ["cargo-zigbuild", "rustc"] if linux else ["cargo", "rustc"]
    built = run(*command, "--color", "never", "--locked", "--release", "--manifest-path", export_dir / "Cargo.toml",
                "--target", f"{target}.{GLIBC}" if linux else target, "--target-dir", export_dir / "target-c",
                "--lib", "--crate-type", "staticlib,cdylib", "--", "--print=native-static-libs", *link_args,
                cwd=Path(manifest).resolve().parent, env={**os.environ, **env}, capture_output=True, encoding="utf-8")
    sys.stderr.write(built.stderr)
    notes = re.findall(r"note: native-static-libs: (.*)", built.stderr)
    if len(notes) != 1:
        raise SystemExit(f"rustc reported no native libraries for {target}")
    natives = native_libs(notes[0].strip(), target)
    release = export_dir / "target-c" / target / "release"
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
        return pack(root, out / "c" / (name + (".zip" if "windows" in target else ".tar.gz")))


def pack(root, path):
    """`root` as a ZIP or gzipped tarball under its own name, owned by no one, readable by all."""
    if path.suffix == ".zip":
        with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as archive:
            for file in sorted(p for p in root.rglob("*") if p.is_file()):
                archive.write(file, file.relative_to(root.parent).as_posix())
        return path

    def owned(member):
        member.uid = member.gid = 0
        member.uname = member.gname = ""
        if not member.issym():
            member.mode = 0o755 if member.isdir() or member.mode & 0o100 else 0o644
        return member

    with tarfile.open(path, "w:gz") as archive:
        archive.add(root, root.name, filter=owned)
    return path


def c_check(lib, check, work):
    """The consumer's program: the caller's check, or else one that links a function of each header."""
    if check:
        source = work / Path(check).name
        shutil.copy(check, source)
        return source
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


def build(args):
    """This host's wheels, C archives, Apple and Go static libraries, the export's sources, and native output."""
    info = facts(args.manifest_path)
    lib, export_dir, out = info["lib"], Path(args.export).resolve(), Path(args.out).resolve()
    crate = export_dir / "Cargo.toml"
    out.mkdir(parents=True, exist_ok=True)
    for target in args.targets:
        if target not in WHEEL_TARGETS and target not in APPLE_TARGETS and target not in GO_TARGETS:
            raise SystemExit(f"no package ships {target}")
        run("rustup", "target", "add", target, cwd=Path(args.manifest_path).resolve().parent)
        if target in WHEEL_TARGETS:
            env = {"MACOSX_DEPLOYMENT_TARGET": MACOS_WHEEL} if "apple" in target else {}
            linux = "linux" in target
            # cargo-zigbuild links Linux against the glibc floor's symbols, whichever glibc the host has.
            built = cargo(args.manifest_path, *(["zigbuild"] if linux else ["build"]), "--locked", "--release",
                          "--manifest-path", crate, "--target", f"{target}.{GLIBC}" if linux else target,
                          "--target-dir", export_dir / "target-python", "--features", "python", env=env)
            sys.stderr.write(built.stderr)
            library = export_dir / "target-python" / target / "release" / library_name(lib, target)
            with tempfile.TemporaryDirectory() as raw:
                repair(write_wheel(info, export_dir, library, target, raw), target, out / "wheels")
            c_archive(info, args.manifest_path, export_dir, target, out)
        if target in APPLE_TARGETS:
            apple = APPLE_TARGETS[target]
            name = "MACOSX_DEPLOYMENT_TARGET" if apple == "macos" else "IPHONEOS_DEPLOYMENT_TARGET"
            library, native = staticlib(args.manifest_path, crate, lib, target, env={name: SWIFT_PLATFORMS[apple]})
            static = out / "static" / target
            static.mkdir(parents=True)
            shutil.copy(library, static)
            (static / "native.txt").write_text(native + "\n", encoding="utf-8")
            # The Go module links the Swift package's macOS library, at its floor.
            if target in GO_TARGETS:
                go_library(args, export_dir, lib, target, library, native, out)
        elif target in GO_TARGETS:
            library, native = staticlib(args.manifest_path, crate, lib, target)
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
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as z:
        for name, data in files.items():
            # A fixed time, so the zip is a function of its files.
            entry = zipfile.ZipInfo(f"{module}@{version}/{name}", date_time=(1980, 1, 1, 0, 0, 0))
            entry.compress_type = zipfile.ZIP_DEFLATED
            entry.external_attr = 0o644 << 16
            z.writestr(entry, data)
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
    p = sub.add_parser("consume-c", help="build a check program, or a link of each header, on the C archive")
    p.add_argument("--archive", required=True)
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
    elif args.command == "consume-c":
        consume_c(args.archive, args.check, args.native)
    elif args.command == "compare":
        compare(Path(args.native).read_bytes(), Path(args.actual).read_bytes(), "installed Swift package")
    elif args.command == "init":
        init(args)


if __name__ == "__main__":
    main()
