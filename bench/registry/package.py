"""Package the api fixture's export without changing the Rust or generated API."""

import base64
import csv
import hashlib
import io
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
COLLECTIONS = "f96b909e1a9e469620bc5037c1e24e00153f376f"


def run(*args, **kwargs):
    """Fail the package build when any tool fails."""
    return subprocess.run(args, check=True, **kwargs)


def wheel():
    """Write a native wheel with hashes for every installed file."""
    manifest = tomllib.loads((ROOT / "ffi/fixtures/api/Cargo.toml").read_text())
    version = manifest["package"]["version"]
    # Native tags deliberately make no manylinux or older macOS compatibility claim.
    platform = sysconfig.get_platform().replace("-", "_").replace(".", "_")
    tag = f"cp311-abi3-{platform}"
    info = f"api_fixture-{version}.dist-info"
    library = {"linux": "libapi.so", "darwin": "libapi.dylib"}[sys.platform]
    files = {
        "api/__init__.py": (EXPORT / "python/api.py").read_bytes(),
        f"api/{library}": (EXPORT / "target/release" / library).read_bytes(),
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
    package.mkdir()
    headers = OUT / "headers"
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
    (package / "Package.swift").write_text('''// swift-tools-version: 6.2
import PackageDescription
let package = Package(
    name: "Api",
    platforms: [.macOS(.v15)],
    products: [.library(name: "Api", targets: ["Api"])],
    dependencies: [.package(url: "https://github.com/apple/swift-collections", revision: "''' + COLLECTIONS + '''")],
    targets: [
        .binaryTarget(name: "ApiFFI", path: "ApiFFI.xcframework"),
        .target(name: "Api", dependencies: ["ApiFFI", .product(name: "BasicContainers", package: "swift-collections")])
    ]
)
''')
    # Consumers receive only the package, script and native reference transcript.
    shutil.make_archive(str(OUT / "Api"), "zip", OUT, "Api")


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    {"wheel": wheel, "swift": swift}[sys.argv[1]]()
