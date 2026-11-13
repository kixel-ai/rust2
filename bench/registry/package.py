"""Package the api fixture's export without changing the Rust or generated API."""

import base64
import csv
import hashlib
import io
import json
import re
import shlex
import platform as host_platform
from pathlib import Path
import shutil
import subprocess
import sys
import sysconfig
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[2]
EXPORT = ROOT / "ffi/target/registry-export"
OUT = ROOT / "ffi/target/registry"
COLLECTIONS = re.search(r'swift_collections_rev := "([a-f0-9]{40})"', (ROOT / "ffi/justfile").read_text()).group(1)


def run(*args, **kwargs):
    """Fail the package build when any tool fails."""
    return subprocess.run(args, check=True, **kwargs)


def wheel():
    """Write a native wheel with hashes for every installed file."""
    manifest = tomllib.loads((ROOT / "ffi/fixtures/api/Cargo.toml").read_text())
    version = manifest["package"]["version"]
    # Native tags deliberately make no manylinux or older macOS compatibility claim.
    platform = sysconfig.get_platform().replace("-", "_").replace(".", "_")
    if sys.platform == "darwin":
        major, *_ = host_platform.mac_ver()[0].split(".")
        platform = f"macosx_{major}_0_{host_platform.machine()}"
    tag = f"cp311-abi3-{platform}"
    info = f"api_fixture-{version}.dist-info"
    library = {"linux": "libapi.so", "darwin": "libapi.dylib"}[sys.platform]
    binary = EXPORT / "target/release" / library
    if sys.platform == "linux":
        binary_copy = OUT / library
        shutil.copyfile(binary, binary_copy)
        run("patchelf", "--remove-rpath", str(binary_copy))
        dynamic = run("readelf", "-d", str(binary_copy), capture_output=True, text=True).stdout
        needed = set(re.findall(r"Shared library: \[(.*?)\]", dynamic))
        allowed = {"libgcc_s.so.1", "libc.so.6", "ld-linux-x86-64.so.2", "ld-linux-aarch64.so.1"}
        if needed - allowed or "RPATH" in dynamic or "RUNPATH" in dynamic:
            raise SystemExit(f"unpackaged native dependencies: {dynamic}")
        versions = run("readelf", "--version-info", str(binary_copy), capture_output=True, text=True).stdout
        glibc = [tuple(map(int, v.split("."))) for v in re.findall(r"GLIBC_([0-9.]+)", versions)]
        if not glibc or max(glibc) > (2, 39):
            raise SystemExit("wheel requires glibc newer than the Ubuntu 24.04 consumer")
        binary = binary_copy
    files = {
        "api/__init__.py": (EXPORT / "python/api.py").read_bytes(),
        f"api/{library}": binary.read_bytes(),
        f"{info}/METADATA": (
            f"Metadata-Version: 2.1\nName: api-fixture\nVersion: {version}\n"
            f"Summary: {manifest['package']['description']}\nRequires-Python: >=3.11\n\n"
        ).encode(),
        f"{info}/WHEEL": (
            f"Wheel-Version: 1.0\nGenerator: rust2-registry-spike\n"
            f"Root-Is-Purelib: false\nTag: {tag}\n"
        ).encode(),
    }
    record = io.StringIO(newline="")
    writer = csv.writer(record)
    for name, data in files.items():
        digest = base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b"=").decode()
        writer.writerow([name, f"sha256={digest}", len(data)])
    writer.writerow([f"{info}/RECORD", "", ""])
    files[f"{info}/RECORD"] = record.getvalue().encode()
    with zipfile.ZipFile(OUT / f"api_fixture-{version}-{tag}.whl", "w", zipfile.ZIP_DEFLATED) as archive:
        for name, data in files.items():
            archive.writestr(name, data)


def swift():
    """Wrap the host's C static library in an XCFramework and Swift source package."""
    if sys.platform != "darwin":
        raise SystemExit("XCFramework packaging requires Apple's xcodebuild on macOS")
    package = OUT / "Api"
    if package.exists():
        shutil.rmtree(package)
    package.mkdir()
    headers = OUT / "headers"
    if headers.exists():
        shutil.rmtree(headers)
    headers.mkdir()
    shutil.copy(EXPORT / "include/api.h", headers)
    shutil.copy(EXPORT / "swift/api_atomics.h", headers)
    modulemap = (EXPORT / "swift/module.modulemap").read_text().replace("../include/api.h", "api.h")
    (headers / "module.modulemap").write_text(modulemap)
    run("xcodebuild", "-create-xcframework", "-library", str(EXPORT / "target/release/libapi.a"),
        "-headers", str(headers), "-output", str(package / "ApiFFI.xcframework"))
    sources = package / "Sources/Api"
    sources.mkdir(parents=True)
    shutil.copy(EXPORT / "swift/api.swift", sources)
    built = run("cargo", "rustc", "--locked", "--release", "--manifest-path", str(EXPORT / "Cargo.toml"),
                "--lib", "--", "--print=native-static-libs", capture_output=True, text=True)
    notes = re.findall(r"note: native-static-libs: (.*)", built.stderr)
    if len(notes) != 1:
        raise SystemExit(f"rustc did not report native link libraries: {built.stderr}")
    flags = iter(shlex.split(notes[0]))
    settings = []
    for flag in flags:
        if flag.startswith("-l"):
            settings.append(f".linkedLibrary({json.dumps(flag[2:])})")
        elif flag == "-framework":
            settings.append(f".linkedFramework({json.dumps(next(flags))})")
        else:
            raise SystemExit(f"unsupported native linker flag: {flag}")
    (package / "collections-revision.txt").write_text(COLLECTIONS + "\n")
    (package / "Package.swift").write_text('''// swift-tools-version: 6.2
import PackageDescription
let package = Package(
    name: "Api",
    platforms: [.macOS(.v15)],
    products: [.library(name: "Api", targets: ["Api"])],
    dependencies: [.package(url: "https://github.com/apple/swift-collections", revision: "''' + COLLECTIONS + '''")],
    targets: [
        .binaryTarget(name: "ApiFFI", path: "ApiFFI.xcframework"),
        .target(name: "Api", dependencies: ["ApiFFI", .product(name: "BasicContainers", package: "swift-collections")], linkerSettings: [''' + ", ".join(settings) + '''])
    ]
)
''')
    # Consumers receive only the package, script and native reference transcript.
    shutil.make_archive(str(OUT / "Api"), "zip", OUT, "Api")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    {"wheel": wheel, "swift": swift}[sys.argv[1]]()
