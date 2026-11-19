"""Build and check the Python wheels, Swift package, and C archives of a crate's rust2-ffi export.

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
    paths = [export / "python" / f"{lib}.py", *headers, *sorted((export / "swift").iterdir())]
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


def abi_line(version):
    """The Cargo-compatible line of `version`: the SONAME's suffix, and the oldest version it serves.

    Cargo calls 1.2.3 compatible with 1.x, 0.4.2 with 0.4.x, and 0.0.3 with itself; a
    prerelease promises nothing, so it serves only itself. The API lock refuses a break,
    an opaque value's size included, within a line.
    """
    match = re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?", version)
    if match is None:
        raise SystemExit(f"version {version} is not semver")
    major, minor, patch, pre, _ = match.groups()
    if pre:
        exact = f"{major}.{minor}.{patch}{pre}"
        return exact, exact
    if major != "0":
        return major, f"{major}.0.0"
    if minor != "0":
        return f"0.{minor}", f"0.{minor}.0"
    return f"0.0.{patch}", f"0.0.{patch}"


def c_files(lib, target, abi):
    """The archive's library paths: static, shared, the shared library's link name, and its import library."""
    if "windows" in target:
        return {"static": f"lib/{lib}.lib", "shared": f"bin/{lib}.dll", "link": None, "implib": f"lib/{lib}.dll.lib"}
    if "apple" in target:
        return {"static": f"lib/lib{lib}.a", "shared": f"lib/lib{lib}.{abi}.dylib", "link": f"lib/lib{lib}.dylib",
                "implib": None}
    return {"static": f"lib/lib{lib}.a", "shared": f"lib/lib{lib}.so.{abi}", "link": f"lib/lib{lib}.so", "implib": None}


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
    """`<lib>-config.cmake`: `<lib>::<lib>` the static library, `<lib>::shared` the shared one."""
    lib = info["lib"]
    libs = [name if kind == "lib" else f"-framework {name}" for kind, name in natives if kind != "flag"]
    options = [name for kind, name in natives if kind == "flag"]
    prefix = f"${{_{lib}_prefix}}"
    shared = [f'IMPORTED_LOCATION "{prefix}/{files["shared"]}"']
    if files["implib"]:
        shared.append(f'IMPORTED_IMPLIB "{prefix}/{files["implib"]}"')
    shared_props = "\n        ".join(shared)
    return f"""# find_package({lib}): {info['name']} {info['version']}'s C and C++ binding for {target}.
# {lib}::{lib} links the static library and what it needs of the system; {lib}::shared links the
# shared library. Generated by rust2-ffi's release builder.

get_filename_component(_{lib}_prefix "${{CMAKE_CURRENT_LIST_DIR}}/../../.." ABSOLUTE)

if(NOT TARGET {lib}::{lib})
    add_library({lib}::{lib} STATIC IMPORTED)
    set_target_properties({lib}::{lib} PROPERTIES
        IMPORTED_LOCATION "{prefix}/{files['static']}"
        IMPORTED_LINK_INTERFACE_LANGUAGES C
        INTERFACE_INCLUDE_DIRECTORIES "{prefix}/include"
        INTERFACE_LINK_LIBRARIES {cmake_quote(';'.join(libs))}
        INTERFACE_LINK_OPTIONS {cmake_quote(';'.join(options))}
    )
endif()

if(NOT TARGET {lib}::shared)
    add_library({lib}::shared SHARED IMPORTED)
    set_target_properties({lib}::shared PROPERTIES
        {shared_props}
        INTERFACE_INCLUDE_DIRECTORIES "{prefix}/include"
    )
endif()

unset(_{lib}_prefix)
"""


def cmake_version(info):
    """`<lib>-config-version.cmake`: Cargo's compatibility, a range's upper bound, and 64-bit consumers alone."""
    _, compat = abi_line(info["version"])
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
    _, compat = abi_line(info["version"])
    static = {
        "type": "archive",
        "location": f"@prefix@/{files['static']}",
        "includes": ["@prefix@/include"],
        "link_languages": ["c"],
    }
    libs = [name if kind == "lib" else f"-framework {name}" for kind, name in natives if kind != "flag"]
    flags = [name for kind, name in natives if kind == "flag"]
    if libs:
        static["link_libraries"] = libs
    if flags:
        static["link_flags"] = flags
    shared = {"type": "dylib", "location": f"@prefix@/{files['shared']}", "includes": ["@prefix@/include"]}
    if files["implib"]:
        shared["link_location"] = f"@prefix@/{files['implib']}"
    description = {
        "cps_version": CPS_VERSION,
        "name": lib,
        "version": info["version"],
        "compat_version": compat,
        "cps_path": "@prefix@/lib/cps",
        "components": {lib: static, "shared": shared},
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
        run("cc", "-r", "-nostdlib", "-arch", "arm64", f"-mmacosx-version-min={MACOS_WHEEL}",
            *(f"-Wl,{arg}" for arg in roots), f"-Wl,-exported_symbols_list,{names}", static, "-o", hidden)
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
    abi, _ = abi_line(version)
    files = c_files(lib, target, abi)
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
    work = Path(tempfile.mkdtemp())
    if archive.suffix == ".zip":
        shutil.unpack_archive(archive, work / "unpacked")
    else:
        shutil.unpack_archive(archive, work / "unpacked", filter="data")
    (prefix,) = (work / "unpacked").iterdir()
    (cps_file,) = (prefix / "lib/cps").glob("*.cps")
    description = json.loads(cps_file.read_text(encoding="utf-8"))
    lib, version = description["name"], description["version"]
    target = prefix.name.rsplit(f"-{version}-", 1)[1]
    # CMake spells a version in numbers alone.
    numeric = re.match(r"[0-9]+\.[0-9]+\.[0-9]+", version).group()
    files = c_files(lib, target, abi_line(version)[0])
    soname = Path(files["shared"]).name
    source = c_check(lib, check, work)
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
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
find_package({lib} {numeric} REQUIRED{configs})
if(NOT {lib}_CONFIG MATCHES "{found}")
    message(FATAL_ERROR "found ${{{lib}_CONFIG}}, not the {mode} file")
endif()
add_executable(check-static {source.as_posix()})
target_link_libraries(check-static PRIVATE {lib}::{lib})
add_executable(check-shared {source.as_posix()})
target_link_libraries(check-shared PRIVATE {lib}::shared)
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
    exporter(args.manifest_path, args.features, *targets, *withs, "--out", Path(args.out).resolve())


def cargo(manifest, *args, env=None):
    """Cargo with the package's own toolchain, as rustup finds it from the package's directory."""
    return run("cargo", *args, cwd=Path(manifest).resolve().parent, env={**os.environ, **(env or {})},
               capture_output=True, encoding="utf-8")


def build(args):
    """This host's wheels, C archives, and Apple static libraries, the export's sources, and native output."""
    info = facts(args.manifest_path)
    lib, export_dir, out = info["lib"], Path(args.export).resolve(), Path(args.out).resolve()
    crate = export_dir / "Cargo.toml"
    out.mkdir(parents=True, exist_ok=True)
    for target in args.targets:
        if target not in WHEEL_TARGETS and target not in APPLE_TARGETS:
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
            # The C library alone, apart from the Python extension, whose CPython symbols it
            # would leave undefined; its own target directory, since cargo does not rebuild
            # for a changed deployment target.
            # Uncolored, so the note below reads as written whatever the caller's terminal.
            built = cargo(args.manifest_path, "rustc", "--color", "never", "--locked", "--release", "--manifest-path", crate,
                          "--target", target, "--target-dir", export_dir / "target-swift", "--lib",
                          "--crate-type", "staticlib", "--", "--print=native-static-libs",
                          env={name: SWIFT_PLATFORMS[apple]})
            sys.stderr.write(built.stderr)
            notes = re.findall(r"note: native-static-libs: (.*)", built.stderr)
            if len(notes) != 1:
                raise SystemExit(f"rustc reported no native libraries for {target}")
            static = out / "static" / target
            static.mkdir(parents=True)
            shutil.copy(export_dir / "target-swift" / target / "release" / f"lib{lib}.a", static)
            (static / "native.txt").write_text(notes[0].strip() + "\n", encoding="utf-8")
    # The generated sources, for the Swift package and the hosts' agreement.
    # Text this writes is LF on every host, as the jobs reading it compare bytes.
    for part in ("include", "swift", "python"):
        shutil.copytree(export_dir / part, out / "export" / part, dirs_exist_ok=True)
    shutil.copy(crate, out / "export")
    (out / "digest.txt").write_text(digest(export_dir, lib) + "\n", newline="\n", encoding="utf-8")
    (out / "lib.txt").write_text(lib + "\n", newline="\n", encoding="utf-8")
    # Native output for a host whose wheel a consumer runs; an iOS slice runs nowhere here.
    if args.example and any(target in WHEEL_TARGETS for target in args.targets):
        if args.example not in info["examples"]:
            raise SystemExit(f"{info['name']} has no example {args.example}: it has {info['examples']}")
        features = ["--features", args.features] if args.features.strip() else []
        manifest = Path(args.manifest_path).resolve()
        # Bytes, as the consumers' output is compared, with no newline translation.
        native = run("cargo", "run", "--locked", "--release", "--quiet", "--manifest-path", manifest,
                     "--example", args.example, *features, cwd=manifest.parent, capture_output=True)
        (out / "native.txt").write_bytes(native.stdout)


def template(info, manifest, rust2, mirror):
    """The caller workflow, with every identity Cargo cannot establish spelled out in it."""
    if not re.fullmatch(r"[0-9a-f]{40}", rust2):
        raise SystemExit("pin rust2 by a full commit SHA")
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", mirror):
        raise SystemExit(f"the Swift mirror must be OWNER/REPO, not {mirror!r}")
    values = {
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
    text = template(info, relative, rust2, args.swift_mirror)
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
    p.add_argument("--rust2", help="the rust2 commit to pin; this checkout's by default")
    args = parser.parse_args()

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
