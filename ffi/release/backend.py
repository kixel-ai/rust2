"""Build a rust2-ffi binding's wheel from its sdist with cargo, needing only the standard library.

An sdist names this module as its PEP 517 backend, beside PKG-INFO, the binding's generated
crate in binding/, and the local crates that crate builds with in sources/. package.py writes
both distributions through the functions here, so a wheel built from the sdist holds what a
prebuilt one does.
"""

import base64
import csv
import email.parser
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import re
import struct
import subprocess
import sys
import sysconfig
import tarfile
import time
import tomllib
import zipfile

# The oldest macOS each architecture's Rust target supports, which a wheel built here claims
# unless MACOSX_DEPLOYMENT_TARGET asks for a newer one.
MACOS_FLOORS = {"aarch64": "11.0", "x86_64": "10.12"}


def dist_name(name):
    """The project's name as a distribution's file names spell it."""
    return re.sub(r"[-_.]+", "_", name).lower()


def library_name(lib, target):
    """The file the generated module loads from beside itself."""
    if "windows" in target:
        return f"{lib}.dll"
    return f"lib{lib}.dylib" if "apple" in target else f"lib{lib}.so"


def python_floor(manifest):
    """The oldest CPython the generated crate's stable-ABI extension loads in, from its pyo3 feature."""
    found = set(re.findall(r'"abi3-py3([0-9]+)"', manifest))
    if len(found) != 1:
        raise SystemExit(f"the generated Cargo.toml names no single abi3 floor: {sorted(found)}")
    return int(found.pop())


def record(files):
    """RECORD: every file's hash and size, and RECORD's own row unhashed."""
    out = io.StringIO(newline="")
    writer = csv.writer(out, lineterminator="\n")
    for name, data in files.items():
        digest = base64.urlsafe_b64encode(hashlib.sha256(data).digest()).rstrip(b"=").decode()
        writer.writerow([name, f"sha256={digest}", len(data)])
    return out


def write_wheel(out, metadata, tag, lib, module, library, target, licenses):
    """A wheel holding the generated module as the typed package `lib` beside its library."""
    meta = email.parser.HeaderParser().parsestr(metadata)
    info = f"{dist_name(meta['Name'])}-{meta['Version']}.dist-info"
    files = {
        f"{lib}/__init__.py": module,
        # PEP 561's marker, without which mypy and pyright skip an installed package.
        f"{lib}/py.typed": b"",
        f"{lib}/{library_name(lib, target)}": library,
    }
    for name, data in licenses.items():
        files[f"{info}/licenses/{name}"] = data
    files[f"{info}/METADATA"] = metadata.encode()
    files[f"{info}/WHEEL"] = (
        f"Wheel-Version: 1.0\nGenerator: rust2-ffi\nRoot-Is-Purelib: false\nTag: {tag}\n"
    ).encode()
    rows = record(files)
    csv.writer(rows, lineterminator="\n").writerow([f"{info}/RECORD", "", ""])
    files[f"{info}/RECORD"] = rows.getvalue().encode()
    path = Path(out) / f"{dist_name(meta['Name'])}-{meta['Version']}-{tag}.whl"
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, data in files.items():
            archive.writestr(name, data)
    return path


def write_sdist(out, name, version, files):
    """An sdist of `files`, each path's bytes and whether it is executable, in sorted order."""
    base = f"{dist_name(name)}-{version}"
    mtime = int(os.environ.get("SOURCE_DATE_EPOCH", time.time()))
    path = Path(out) / f"{base}.tar.gz"
    with open(path, "wb") as raw, gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=mtime) as zipped, \
            tarfile.open(fileobj=zipped, mode="w", format=tarfile.PAX_FORMAT) as tar:
        for name in sorted(files):
            data, executable = files[name]
            entry = tarfile.TarInfo(f"{base}/{name}")
            entry.size, entry.mtime, entry.mode = len(data), mtime, 0o755 if executable else 0o644
            tar.addfile(entry, io.BytesIO(data))
    return path


def host(crate):
    """The target rustc builds for here, as rustup resolves the toolchain from the crate's directory."""
    try:
        version = subprocess.run(["rustc", "-vV"], cwd=crate, stdout=subprocess.PIPE, text=True)
    except FileNotFoundError:
        raise SystemExit("building this package needs Rust's cargo and rustc on PATH: see https://rustup.rs") from None
    if version.returncode:
        raise SystemExit(f"rustc -vV failed with status {version.returncode}")
    return re.search(r"^host: (\S+)$", version.stdout, re.M).group(1)


def check_arch(target):
    """Refuse a target of another architecture than this 64-bit Python's, such as an x86_64
    Python emulated on an arm64 Mac or Windows, whose Rust builds for arm64."""
    if struct.calcsize("P") != 8:
        return
    here = (sysconfig.get_platform() if os.name == "nt" else platform.machine()).lower()
    here = {"amd64": "x86_64", "win-amd64": "x86_64", "arm64": "aarch64", "win-arm64": "aarch64"}.get(here, here)
    arch = target.split("-")[0]
    if {here, arch} <= {"x86_64", "aarch64"} and here != arch:
        raise SystemExit(f"this Python runs {here} code, but Rust builds for {target}: "
                         f"set CARGO_BUILD_TARGET to the {here} target")


def platform_tag(target, env):
    """The platform tag pip installs this build's wheel under. On macOS, the deployment target
    it claims, set in `env` for cargo, so the library's minimum OS is the tag's: the one asked
    for, or the target's oldest, which rustc raises an older request to."""
    if sys.platform == "darwin":
        arch = target.split("-")[0]
        if arch not in MACOS_FLOORS:
            raise SystemExit(f"no wheel tag spells {target}")
        floor = [int(part) for part in MACOS_FLOORS[arch].split(".")]
        asked = env.get("MACOSX_DEPLOYMENT_TARGET") or MACOS_FLOORS[arch]
        try:
            major, minor = max([int(part) for part in (asked.split(".") + ["0"])[:2]], floor)
        except ValueError:
            raise SystemExit(f"MACOSX_DEPLOYMENT_TARGET {asked!r} is no macOS version") from None
        env["MACOSX_DEPLOYMENT_TARGET"] = f"{major}.{minor}"
        # From macOS 11, pip's tags name each major version's .0 alone.
        return f"macosx_{major}_{minor if major < 11 else 0}_{'arm64' if arch == 'aarch64' else arch}"
    tag = sysconfig.get_platform()
    # As pip's tags name a 32-bit Python on a 64-bit Linux kernel.
    if struct.calcsize("P") == 4:
        tag = {"linux-x86_64": "linux-i686", "linux-aarch64": "linux-armv8l"}.get(tag, tag)
    return re.sub(r"[-. ]", "_", tag)


def dynamic_crt(target, env):
    """On musl, whose targets link the C runtime statically and so build no cdylib, the flag
    that links it dynamically, as CPython does: in the rustflags cargo reads first, since it
    checks a crate type against them before any `cargo rustc -- <flag>` applies. With neither
    variable set, cargo reads the target's rustflags in place of a config's `build.rustflags`."""
    if "musl" not in target:
        return []
    flag = "-Ctarget-feature=-crt-static"
    # Set at all, even empty, either variable is what cargo reads.
    if "CARGO_ENCODED_RUSTFLAGS" in env:
        env["CARGO_ENCODED_RUSTFLAGS"] = "\x1f".join(filter(None, [env["CARGO_ENCODED_RUSTFLAGS"], flag]))
    elif "RUSTFLAGS" in env:
        env["RUSTFLAGS"] = f"{env['RUSTFLAGS']} {flag}".strip()
    else:
        return ["--config", f'target.{target}.rustflags = ["{flag}"]']
    return []


def build_wheel(wheel_directory, config_settings=None, metadata_directory=None):
    """PEP 517: build this platform's wheel with cargo, into `wheel_directory`."""
    root = Path.cwd()
    metadata = (root / "PKG-INFO").read_text(encoding="utf-8")
    crate = root / "binding"
    manifest = (crate / "Cargo.toml").read_text(encoding="utf-8")
    lib = tomllib.loads(manifest)["lib"]["name"]
    exported = tomllib.loads((root / "pyproject.toml").read_text(encoding="utf-8"))["tool"]["rust2-ffi"]["targets"]
    env = {**os.environ, "PYO3_PYTHON": sys.executable}
    target = env.get("CARGO_BUILD_TARGET") or host(crate)
    if target not in exported:
        raise SystemExit(f"this package builds for the targets it was exported for, {', '.join(exported)}, "
                         f"not {target}: set CARGO_BUILD_TARGET to one that runs here")
    check_arch(target)
    tag = f"cp3{python_floor(manifest)}-abi3-{platform_tag(target, env)}"
    built = subprocess.run(["cargo", "rustc", "--locked", "--release", "--lib", "--crate-type", "cdylib",
                            "--features", "python", "--message-format", "json-render-diagnostics",
                            # The target checked above, whatever a cargo config's build.target says.
                            "--target", target, *dynamic_crt(target, env)],
                           cwd=crate, env=env, stdout=subprocess.PIPE, text=True)
    if built.returncode:
        raise SystemExit(f"cargo failed with status {built.returncode}")
    want = library_name(lib, target)
    found = [file for line in built.stdout.splitlines() if (message := json.loads(line))["reason"] == "compiler-artifact"
             and message["target"]["name"] == lib and "cdylib" in message["target"]["crate_types"]
             for file in message["filenames"] if Path(file).name == want]
    if len(found) != 1:
        raise SystemExit(f"cargo built {len(found)} files named {want}")
    meta = email.parser.HeaderParser().parsestr(metadata)
    licenses = {name: (root / name).read_bytes() for name in meta.get_all("License-File", [])}
    module = (crate / "python" / f"{lib}.py").read_bytes()
    path = write_wheel(wheel_directory, metadata, tag, lib, module, Path(found[0]).read_bytes(), target, licenses)
    return path.name


def build_sdist(sdist_directory, config_settings=None):
    """PEP 517: this sdist again, of the files an sdist holds and nothing a build wrote beside them."""
    root = Path.cwd()
    meta = email.parser.HeaderParser().parsestr((root / "PKG-INFO").read_text(encoding="utf-8"))
    names = ["PKG-INFO", "pyproject.toml", "rust-toolchain.toml", *meta.get_all("License-File", [])]
    paths = [root / name for name in names if (root / name).is_file()]
    paths += [p for top in ("rust2", "binding", "sources") for p in sorted((root / top).rglob("*")) if p.is_file()]
    files = {}
    for path in paths:
        rel = path.relative_to(root)
        if "__pycache__" not in rel.parts and rel.parts[:2] != ("binding", "target"):
            files[rel.as_posix()] = (path.read_bytes(), os.access(path, os.X_OK))
    return write_sdist(sdist_directory, meta["Name"], meta["Version"], files).name
