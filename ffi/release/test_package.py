"""The builder's decisions that need no toolchain: names, versions, metadata, and manifests."""

import importlib.util
import json
import os
import plistlib
import queue
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import tomllib
import unittest
import xml.etree.ElementTree
from unittest import mock
import zipfile

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("package", HERE / "package.py")
package = importlib.util.module_from_spec(spec)
spec.loader.exec_module(package)
backend = package.backend


def info(root, **overrides):
    """Facts as `facts` reads them, for a package in `root`."""
    base = {
        "name": "moq-ffi", "version": "0.4.12", "description": "Media over QUIC,\n bindings",
        "license": "MIT OR Apache-2.0", "license_file": None, "repository": "https://github.com/moq-dev/moq",
        "homepage": None, "readme": None, "authors": [], "lib": "moq", "dir": str(root), "workspace": str(root),
        "examples": [],
        "epoch": 1_700_000_000, "sbom": b"{}\n",
    }
    return {**base, **overrides}


def export(root, floor="311"):
    """The parts of an export a wheel and a digest read."""
    (root / "python").mkdir(parents=True)
    (root / "swift").mkdir()
    (root / "include").mkdir()
    (root / "python/moq.py").write_text("PENDING = 0\n")
    (root / "include/moq.h").write_text("int moq_contract(void);\n")
    (root / "swift/moq.swift").write_text("import MoqFFI\n")
    (root / "Cargo.toml").write_text(f'[features]\npython = ["pyo3-ffi/abi3-py{floor}"]\n'
                                     'python-abi3t = ["pyo3-ffi/abi3t-py315"]\n')
    (root / "go/rt").mkdir(parents=True)
    (root / "go/go.mod").write_text("module moq.dev/moq-ffi\n\ngo 1.24\n")
    lines = "".join(f"#cgo {p.replace('_', ',')} LDFLAGS: ${{SRCDIR}}/libmoq_{p}.a -lgcc_s -lc\n"
                    for p in package.GO_TARGETS.values())
    (root / "go/rt/rt.go").write_text(f"package rt\n\n/*\n{lines}*/\nimport \"C\"\n")
    (root / "kotlin").mkdir()
    (root / "kotlin/moq.kt").write_text("package moq\n")
    (root / "kotlin/moq.pro").write_text("-keep class moq._Native { *; }\n")
    return root


def go_libs(root):
    """A directory of every Go target's library, as `build` names them."""
    (root / "libs").mkdir()
    for platform in package.GO_TARGETS.values():
        (root / "libs" / f"libmoq_{platform}.a").write_bytes(b"!<arch>\n")
    return root / "libs"


class Versions(unittest.TestCase):
    def test_release_and_prereleases(self):
        self.assertEqual(package.pep440("0.4.12"), "0.4.12")
        self.assertEqual(package.pep440("1.0.0-alpha.1"), "1.0.0a1")
        self.assertEqual(package.pep440("1.0.0-beta.0"), "1.0.0b0")
        self.assertEqual(package.pep440("2.1.0-rc.3"), "2.1.0rc3")

    def test_refuses_what_pypi_cannot_spell(self):
        for version in ("1.0.0-pre", "1.0.0+build.1", "1.0.0-alpha", "01.0.0", "1.0"):
            with self.assertRaises(SystemExit, msg=version):
                package.pep440(version)


class Baseline(unittest.TestCase):
    def test_breaking_is_the_leftmost_nonzero_part(self):
        for old, new, ok in [
            ("1.2.3", "1.9.0", True), ("1.2.3", "2.0.0", False),
            ("0.4.12", "0.4.13", True), ("0.4.12", "0.5.0", False),
            ("0.0.3", "0.0.4", False),
            ("1.0.0-rc.1", "1.0.0", False), ("1.0.0", "1.0.1-alpha.1", False),
        ]:
            self.assertEqual(package.compatible(old, new), ok, (old, new))

    def test_releases_a_version_answers_to(self):
        tags = ["moq-ffi-v0.4.10", "moq-ffi-v0.4.12", "moq-ffi-v0.5.0-rc.1", "other-v0.4.11", "moq-ffi-v0.4.11",
                "moq-ffi-v1.2.3", "moq-ffi-v1.3.0-rc.1"]
        self.assertEqual(package.earlier_releases("moq-ffi", "0.4.12", tags), ("0.4.11", "0.4.11"))
        self.assertEqual(package.earlier_releases("moq-ffi", "0.5.0", tags), ("0.5.0-rc.1", None))
        self.assertEqual(package.earlier_releases("moq-ffi", "0.4.10", tags), (None, None))
        # A prerelease between them leaves the stable release the baseline.
        self.assertEqual(package.earlier_releases("moq-ffi", "1.3.0", tags), ("1.3.0-rc.1", "1.2.3"))


def api(records, functions):
    """An api.json of these records and functions, each a dict of its own fields."""
    return {"api": {"records": records, "enums": [], "objects": [], "callbacks": [], "functions": functions,
                    "constants": [], "targets": ["x86_64-unknown-linux-gnu"]}}


def record(name, docs=""):
    return {"path": ["moq", name], "shown": ["moq", name], "on": 1, "docs": docs, "span": {"lo": len(docs)},
            "fields": [], "methods": []}


def function(name, ret):
    return {"path": ["moq", name], "shown": ["moq", name], "on": 1, "docs": "", "params": [], "ok": ret}


class Changes(unittest.TestCase):
    def test_types_are_named_not_indexed(self):
        # A record added before `Track` moves its index, which changes nothing that names it.
        old = api([record("Track")], [function("open", {"Record": 0}), function("close", None)])
        new = api([record("Group", docs="new docs"), record("Track", docs="moved")],
                  [function("open", {"Record": 1}), function("seek", {"Record": 0})])
        text = package.changes(old, new, "0.4.11")
        self.assertEqual(text, "## API changes since 0.4.11\n\n### Removed\n\n- fn `moq::close`\n"
                               "\n### Added\n\n- fn `moq::seek`\n- struct `moq::Group`\n")
        # An object added before the one a function's task makes moves its index too.
        def obj(name):
            return {**record(name), "fields": None}
        def poll(i):
            return {**function("poll", None), "task": {"at": 0, "kind": {"Made": {"object": i, "make": "new"}}}}
        old = api([], [poll(0)])
        old["api"]["objects"] = [obj("Waiter")]
        new = api([], [poll(1)])
        new["api"]["objects"] = [obj("Clock"), obj("Waiter")]
        self.assertEqual(package.changes(old, new, "0.4.11"), "## API changes since 0.4.11\n\n### Added\n\n- type `moq::Clock`\n")

    def test_accessors_and_nested_targets(self):
        def method(name, access):
            return {**function(name, None), "access": access}
        old = api([{**record("Track"), "methods": [method("seq", {"Field": {"shared": False, "set": False}}),
                                                     method("seq", {"Field": {"shared": False, "set": True}})],
                    "variants": [{"name": "A", "on": 1}]}], [])
        # A target sorting first shifts every mask, whose targets stay the same.
        new = api([{**record("Track"), "on": 2,
                    "methods": [{**method("seq", {"Field": {"shared": False, "set": False}}), "on": 2}],
                    "variants": [{"name": "A", "on": 2}]}], [])
        new["api"]["targets"] = ["aarch64-apple-darwin", "x86_64-unknown-linux-gnu"]
        text = package.changes(old, new, "0.4.11")
        self.assertIn("### Removed\n\n- set `moq::seq`\n", text)
        self.assertNotIn("struct `moq::Track`", text)

    def test_a_signature_change(self):
        old = api([record("Track"), record("Group")], [function("open", {"Record": 0})])
        new = api([record("Track"), record("Group")], [function("open", {"Record": 1})])
        self.assertIn("### Changed\n\n- fn `moq::open`\n", package.changes(old, new, "0.4.11"))


class Names(unittest.TestCase):
    def test_swift_module_matches_the_exporter(self):
        self.assertEqual(package.pascal("moq"), "Moq")
        self.assertEqual(package.pascal("moq_ffi"), "MoqFfi")
        self.assertEqual(package.pascal("api"), "Api")

    def test_library_of_any_crate_type(self):
        for kinds in (["lib"], ["rlib"], ["dylib"], ["cdylib", "rlib"], ["staticlib"]):
            targets = [{"kind": kinds, "name": "dep-fixture"}, {"kind": ["example"], "name": "transcript"}]
            self.assertEqual(package.library({"name": "dep-fixture", "targets": targets}), "dep_fixture", kinds)
        for targets in ([{"kind": ["proc-macro"], "name": "derive"}], [{"kind": ["bin"], "name": "tool"}]):
            with self.assertRaises(SystemExit):
                package.library({"name": "tool", "targets": targets})

    def test_wheel_platforms(self):
        self.assertEqual(package.wheel_platform("aarch64-apple-darwin"), "macosx_11_0_arm64")
        self.assertEqual(package.wheel_platform("x86_64-pc-windows-msvc"), "win_amd64")
        with self.assertRaises(SystemExit):
            package.wheel_platform("armv7-unknown-linux-gnueabihf")
        self.assertEqual(package.library_name("moq", "x86_64-pc-windows-msvc"), "moq.dll")
        self.assertEqual(package.library_name("moq", "aarch64-apple-darwin"), "libmoq.dylib")
        self.assertEqual(package.library_name("moq", "aarch64-unknown-linux-gnu"), "libmoq.so")


class Wheel(unittest.TestCase):
    def test_metadata_and_record(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "LICENSE-MIT").write_text("MIT\n")
            (root / "LICENSE-APACHE").write_text("Apache\n")
            built = export(root / "export")
            library = root / "libmoq.so"
            library.write_bytes(b"\x7fELF")
            out = root / "out"
            out.mkdir()
            wheel = package.write_wheel(info(root), built, library, "x86_64-unknown-linux-gnu", out, "abi3")
            self.assertEqual(wheel.name, "moq_ffi-0.4.12-cp311-abi3-linux_x86_64.whl")
            package.check_wheel(wheel)
            with zipfile.ZipFile(wheel) as archive:
                names = set(archive.namelist())
                meta = archive.read("moq_ffi-0.4.12.dist-info/METADATA").decode()
            self.assertIn("moq/__init__.py", names)
            self.assertIn("moq/py.typed", names)
            self.assertIn("moq/libmoq.so", names)
            self.assertIn("moq_ffi-0.4.12.dist-info/licenses/LICENSE-MIT", names)
            self.assertIn("Requires-Python: >=3.11\n", meta)
            self.assertIn("Summary: Media over QUIC, bindings\n", meta)
            self.assertIn("License-Expression: MIT OR Apache-2.0\n", meta)
            self.assertIn("Project-URL: Repository, https://github.com/moq-dev/moq\n", meta)

    def test_a_free_threaded_wheel_has_the_same_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            library = root / "libmoq.so"
            library.write_bytes(b"\x7fELF")
            wheel = package.write_wheel(info(root), export(root / "export"), library, "x86_64-unknown-linux-gnu",
                                        root, "abi3t")
            self.assertEqual(wheel.name, "moq_ffi-0.4.12-cp315-abi3t-linux_x86_64.whl")
            with zipfile.ZipFile(wheel) as archive:
                self.assertIn("Requires-Python: >=3.11\n", archive.read("moq_ffi-0.4.12.dist-info/METADATA").decode())
                self.assertIn("Tag: cp315-abi3t-linux_x86_64\n",
                              archive.read("moq_ffi-0.4.12.dist-info/WHEEL").decode())

    def test_refuses_a_legacy_license(self):
        with self.assertRaises(SystemExit):
            package.metadata(info("/src", license="MIT/Apache-2.0"), 11)

    def test_directory_entries_are_not_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            library = root / "libmoq.dylib"
            library.write_bytes(b"\xcf\xfa\xed\xfe")
            wheel = package.write_wheel(info(root), export(root / "export"), library, "aarch64-apple-darwin", root, "abi3")
            repaired = root / "repaired.whl"
            # As delocate and auditwheel rewrite a wheel: each directory listed before its files.
            with zipfile.ZipFile(wheel) as src, zipfile.ZipFile(repaired, "w") as dst:
                dst.writestr("moq/", b"")
                dst.writestr("moq_ffi-0.4.12.dist-info/", b"")
                for name in src.namelist():
                    dst.writestr(name, src.read(name))
            package.check_wheel(repaired)

    def test_vendored_libraries_are_each_tools(self):
        with tempfile.TemporaryDirectory() as tmp:
            wheel = Path(tmp) / "repaired.whl"
            # Where auditwheel and delvewheel, then delocate, put the libraries they copy in.
            with zipfile.ZipFile(wheel, "w") as archive:
                for name in ("moq/libmoq.so", "moq_ffi.libs/", "moq_ffi.libs/libssl-1a2b.so.3",
                             "moq/.dylibs/libz.1.dylib", "moq_ffi-0.4.12.dist-info/RECORD"):
                    archive.writestr(name, b"")
            self.assertEqual(package.vendored(wheel), ["libssl-1a2b.so.3", "libz.1.dylib"])

    def test_a_tampered_record_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            library = root / "moq.dll"
            library.write_bytes(b"MZ")
            wheel = package.write_wheel(info(root), export(root / "export"), library, "x86_64-pc-windows-msvc", root, "abi3")
            tampered = root / "tampered.whl"
            with zipfile.ZipFile(wheel) as src, zipfile.ZipFile(tampered, "w") as dst:
                for name in src.namelist():
                    data = src.read(name)
                    dst.writestr(name, b"MZ!" if name.endswith("moq.dll") else data)
            with self.assertRaises(SystemExit):
                package.check_wheel(tampered)

    def test_floor_comes_from_the_glue(self):
        with tempfile.TemporaryDirectory() as tmp:
            built = export(Path(tmp), "312")
            self.assertEqual(package.python_floor(built), 12)
            self.assertEqual(package.python_floor(built, "abi3t"), 15)


class Reproducible(unittest.TestCase):
    def wheel(self, root, **touch):
        """A wheel built from inputs of the given mtimes, whose repair tool put its files in any order and time."""
        built = export(root / "export")
        library = root / "libmoq.so"
        library.write_bytes(b"\x7fELF")
        for path in (library, built / "python/moq.py"):
            os.utime(path, (touch.get("time", 0), touch.get("time", 0)))
        return package.write_wheel(info(root), built, library, "x86_64-unknown-linux-gnu", root, "abi3")

    def test_wheel_bytes_ignore_when_and_where(self):
        with tempfile.TemporaryDirectory() as a, tempfile.TemporaryDirectory() as b:
            first = self.wheel(Path(a), time=1_000_000_000)
            second = self.wheel(Path(b), time=1_600_000_000)
            self.assertEqual(first.read_bytes(), second.read_bytes())
            with zipfile.ZipFile(first) as archive:
                names = archive.namelist()
                self.assertTrue(all(entry.date_time == (2023, 11, 14, 22, 13, 20) for entry in archive.infolist()))
            self.assertTrue(names[-1].endswith(".dist-info/RECORD"))
            self.assertEqual(names[:2], ["moq/__init__.py", "moq/libmoq.so"])

    def test_a_repair_tools_wheel_is_rewritten_as_one_of_ours(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            wheel = self.wheel(root)
            repaired = root / "repaired.whl"
            with zipfile.ZipFile(wheel) as src, zipfile.ZipFile(repaired, "w") as dst:
                # As a repair tool leaves it: directories, reversed order, its own times, and a RECORD of its own.
                dst.writestr("moq/", b"")
                for name in reversed(src.namelist()):
                    entry = zipfile.ZipInfo(name, (2031, 5, 6, 7, 8, 10))
                    entry.external_attr = src.getinfo(name).external_attr
                    dst.writestr(entry, b"other\n" if name.endswith("/RECORD") else src.read(name))
            package.normalize_wheel(repaired, 1_700_000_000)
            package.check_wheel(repaired)
            self.assertEqual(repaired.read_bytes(), wheel.read_bytes())

    def test_the_sbom_rides_in_the_wheel(self):
        with tempfile.TemporaryDirectory() as tmp:
            with zipfile.ZipFile(self.wheel(Path(tmp))) as archive:
                self.assertEqual(archive.read("moq_ffi-0.4.12.dist-info/sboms/moq-ffi.cdx.json"), b"{}\n")

    def test_xcframework_zip_ignores_mtimes_and_umask(self):
        with tempfile.TemporaryDirectory() as tmp:
            zips = []
            for index, (mode, time) in enumerate(((0o644, 1), (0o664, 2_000_000_000))):
                tree = Path(tmp) / str(index) / "MoqFFI.xcframework"
                (tree / "macos").mkdir(parents=True)
                (tree / "Info.plist").write_text("plist\n")
                (tree / "macos/libmoq.a").write_bytes(b"!<arch>\n")
                for path in (tree, tree / "macos", tree / "Info.plist", tree / "macos/libmoq.a"):
                    os.utime(path, (time, time))
                (tree / "Info.plist").chmod(mode)
                package.zip_tree(tree, Path(tmp) / f"{index}.zip", 1_700_000_000)
                zips.append((Path(tmp) / f"{index}.zip").read_bytes())
            self.assertEqual(zips[0], zips[1])
            with zipfile.ZipFile(Path(tmp) / "0.zip") as archive:
                self.assertEqual(archive.namelist(), [
                    "MoqFFI.xcframework/", "MoqFFI.xcframework/Info.plist", "MoqFFI.xcframework/macos/",
                    "MoqFFI.xcframework/macos/libmoq.a"])

    def test_xcframework_plist_lists_libraries_in_name_order(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "Info.plist"
            libraries = [{"LibraryIdentifier": name} for name in ("macos-arm64", "ios-arm64-simulator", "ios-arm64")]
            for order in (libraries, libraries[::-1]):
                path.write_bytes(package.plistlib.dumps({"AvailableLibraries": order, "CFBundlePackageType": "XFWK"}))
                package.sort_plist(path)
                first = path.read_bytes() if order is libraries else first
            self.assertEqual(path.read_bytes(), first)
            self.assertEqual([x["LibraryIdentifier"] for x in package.plistlib.loads(first)["AvailableLibraries"]],
                             ["ios-arm64", "ios-arm64-simulator", "macos-arm64"])

    def test_a_symlink_in_the_xcframework_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            tree = Path(tmp) / "MoqFFI.xcframework"
            tree.mkdir()
            (tree / "link").symlink_to("target")
            with self.assertRaises(SystemExit):
                package.zip_tree(tree, Path(tmp) / "out.zip", 1_700_000_000)

    def test_epoch_is_the_environment_or_the_commit(self):
        with mock.patch.dict(os.environ, {"SOURCE_DATE_EPOCH": "1700000000"}):
            self.assertEqual(package.epoch("."), 1_700_000_000)
        with mock.patch.dict(os.environ, {"SOURCE_DATE_EPOCH": "86400"}), self.assertRaises(SystemExit):
            package.epoch(".")
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ):
            os.environ.pop("SOURCE_DATE_EPOCH", None)
            git = ["git", "-C", tmp, "-c", "user.name=t", "-c", "user.email=t@t"]
            subprocess.run([*git, "init", "-q"], check=True)
            subprocess.run([*git, "commit", "-q", "--allow-empty", "-m", "x", "--date", "2023-11-14T22:13:20Z"],
                           check=True, env={**os.environ, "GIT_COMMITTER_DATE": "2023-11-14T22:13:20Z"})
            self.assertEqual(package.epoch(tmp), 1_700_000_000)

    def test_paths_are_remapped_and_rustflags_kept(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"RUSTFLAGS": "-C target-cpu=native -Lnative=C:\\deps", "CARGO_HOME": "/home/u/.cargo"}):
            os.environ.pop("CARGO_ENCODED_RUSTFLAGS", None)
            env = package.reproducible(info(tmp), Path(tmp) / "export", "x86_64-unknown-linux-gnu")
            flags = env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
            # Cargo splits on whitespace alone, so a Windows path keeps its backslash.
            self.assertEqual(flags[:3], ["-C", "target-cpu=native", "-Lnative=C:\\deps"])
            self.assertIn(f"--remap-path-prefix={tmp}/export=/export", flags)
            # The deeper directory comes last, as rustc prefers the last match.
            self.assertLess(flags.index(f"--remap-path-prefix={tmp}=/source"), flags.index(f"--remap-path-prefix={tmp}/export=/export"))
            self.assertEqual(env["SOURCE_DATE_EPOCH"], "1700000000")
            self.assertIn(f"-ffile-prefix-map={tmp}=/source", env["CFLAGS"])
            self.assertIn(f"-ffile-prefix-map={tmp}=/source", env["CXXFLAGS"])
            windows = package.reproducible(info(tmp), Path(tmp) / "export", "x86_64-pc-windows-msvc")
            self.assertIn("link-arg=/Brepro", windows["CARGO_ENCODED_RUSTFLAGS"])
            self.assertNotIn("CFLAGS", windows)
            self.assertNotIn("CXXFLAGS", windows)
            apple = package.reproducible(info(tmp), Path(tmp), "aarch64-apple-darwin")
            self.assertEqual(apple["ZERO_AR_DATE"], "1")
            self.assertIn("link-arg=-Wl,-install_name,@rpath/libmoq.dylib", apple["CARGO_ENCODED_RUSTFLAGS"])
            named = package.reproducible(info(tmp), Path(tmp), "aarch64-apple-darwin", "libmoq.0.4.12.dylib")
            self.assertIn("link-arg=-Wl,-install_name,@rpath/libmoq.0.4.12.dylib", named["CARGO_ENCODED_RUSTFLAGS"])
            self.assertNotIn("@rpath/libmoq.dylib", named["CARGO_ENCODED_RUSTFLAGS"])

    def test_an_empty_encoded_rustflags_is_no_flags(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_ENCODED_RUSTFLAGS": "", "RUSTFLAGS": "-C x", "CARGO_HOME": tmp + "/home"}):
            env = package.reproducible(info(tmp), Path(tmp) / "export", "x86_64-unknown-linux-gnu")
            flags = env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
            self.assertTrue(flags[0].startswith("--remap-path-prefix="), flags)
            self.assertNotIn("-C", flags)

    def test_both_spellings_of_a_linked_directory_are_remapped(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_HOME": tmp + "/home"}):
            os.environ.pop("CARGO_ENCODED_RUSTFLAGS", None)
            real = Path(tmp).resolve() / "real"
            (real / "export").mkdir(parents=True)
            (Path(tmp) / "link").symlink_to(real, target_is_directory=True)
            link = Path(tmp) / "link"
            flags = package.reproducible(info(link), link / "export", "x86_64-unknown-linux-gnu")["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
            for spelled in (link, real):
                self.assertIn(f"--remap-path-prefix={spelled}=/source", flags)
                self.assertIn(f"--remap-path-prefix={spelled / 'export'}=/export", flags)

    def test_a_build_reaches_its_packages_through_the_link(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_HOME": tmp + "/home"}):
            root = Path(tmp).resolve()
            crate, export = root / "checkout/source/crate", root / "checkout/export"
            crate.mkdir(parents=True)
            export.mkdir()
            (export / "Cargo.toml").write_text('[dependencies]\ncrate = { path = "../source/crate" }\n')
            (crate / "Cargo.toml").write_text('[package]\nname = "crate"\n')
            (root / "checkout/source/Cargo.toml").write_text('[workspace]\nmembers = ["crate"]\n')
            link = root / "link"
            graph = ([crate], [root / "checkout/source/Cargo.toml"])
            # Where cargo runs, and so where it reads its config from.
            manifest = export / "Cargo.toml"
            with mock.patch.object(package, "LINK", link), mock.patch.object(package, "local_packages", return_value=graph):
                with package.linked(manifest, export) as spell:
                    # The deepest directory holding the export, the crate, and its workspace root.
                    self.assertEqual(link.resolve(), root / "checkout")
                    self.assertEqual(spell(export), link / "export")
                    self.assertEqual(spell(crate), link / "source/crate")
                    # A second build while one holds the link would build the other's sources.
                    with self.assertRaises(SystemExit), package.linked(manifest, export):
                        pass
                self.assertFalse(os.path.lexists(link))
                self.assertTrue(export.is_dir())
                # A package named by its absolute path would be two packages, by it and through the link.
                (crate / "Cargo.toml").write_text(f'[package]\nname = "crate"\n\n[dependencies]\nother = {{ path = {json.dumps(str(root))} }}\n')
                with package.linked(manifest, export) as spell:
                    self.assertFalse(os.path.lexists(link))
                    self.assertEqual(spell(export), export)
                # So would one a cargo config names, which cargo joins from the config's real directory.
                (crate / "Cargo.toml").write_text('[package]\nname = "crate"\n')
                (root / "checkout/.cargo").mkdir()
                for config in ('[patch.crates-io]\nother = { path = "source/other" }\n', 'paths = ["source/other"]\n'):
                    (root / "checkout/.cargo/config.toml").write_text(config)
                    with package.linked(manifest, export) as spell:
                        self.assertFalse(os.path.lexists(link))
                        self.assertEqual(spell(export), export)
                (root / "checkout/.cargo/config.toml").write_text('[build]\nincremental = false\n')
                with package.linked(manifest, export) as spell:
                    self.assertEqual(spell(export), link / "export")

    def test_a_relative_cargo_home_is_where_cargo_runs(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_HOME": "home"}):
            crate = Path(tmp).resolve() / "crate"
            (crate / "home").mkdir(parents=True)
            (crate / "home/config.toml").write_text('paths = ["other"]\n')
            self.assertIn(crate / "home/config.toml", [found for found, _ in package.cargo_configs(crate)])

    def test_a_path_cc_would_split_fails(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_HOME": tmp + "/home"}):
            os.environ.pop("CARGO_ENCODED_RUSTFLAGS", None)
            spaced = Path(tmp) / "a b"
            with self.assertRaises(SystemExit):
                package.reproducible(info(tmp), spaced, "x86_64-unknown-linux-gnu")
            # MSVC's C flags are not CFLAGS, so a space there is harmless.
            package.reproducible(info(tmp), spaced, "x86_64-pc-windows-msvc")

    def test_a_cargo_config_that_RUSTFLAGS_would_replace_fails(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_HOME": tmp + "/home"}):
            crate = Path(tmp) / "crate"
            (crate / ".cargo").mkdir(parents=True)
            (crate / ".cargo/config.toml").write_text('[target.x86_64-pc-windows-msvc]\nrustflags = ["-C", "x"]\n')
            self.assertEqual(list(package.rustflags_config(crate, "x86_64-unknown-linux-gnu")), [])
            self.assertEqual(len(list(package.rustflags_config(crate, "x86_64-pc-windows-msvc"))), 1)
            (crate / ".cargo/config.toml").write_text('[build]\nrustflags = ["-C", "x"]\n')
            with self.assertRaises(SystemExit):
                package.reproducible(info(crate), crate / "export", "x86_64-unknown-linux-gnu")

    def test_same_names_what_differs(self):
        with tempfile.TemporaryDirectory() as tmp:
            roots = [Path(tmp) / "a", Path(tmp) / "b"]
            for root in roots:
                (root / "static/aarch64-apple-ios").mkdir(parents=True)
                (root / "static/aarch64-apple-ios/libmoq.a").write_bytes(b"same")
                (root / "wheels").mkdir()
            first = self.wheel(roots[0])
            second = self.wheel(roots[1])
            for root, wheel in zip(roots, (first, second)):
                wheel.replace(root / "wheels" / wheel.name)
            package.same(roots[0], roots[1])
            (roots[1] / "static/aarch64-apple-ios/libmoq.a").write_bytes(b"other")
            with self.assertRaisesRegex(SystemExit, "static/aarch64-apple-ios/libmoq.a"):
                package.same(roots[0], roots[1])


class Sbom(unittest.TestCase):
    def crate(self, root):
        """A generated crate whose normal, proc-macro, and build dependencies are each a path crate."""
        for name, extra in (("dep", ""), ("builder", ""), ("derive", "[lib]\nproc-macro = true\n")):
            (root / name / "src").mkdir(parents=True)
            (root / name / "src/lib.rs").write_text("")
            (root / name / "Cargo.toml").write_text(f'[package]\nname = "{name}"\nversion = "1.2.3"\nedition = "2021"\nlicense = "MIT/Apache-2.0"\n{extra}')
        (root / "glue/src").mkdir(parents=True)
        (root / "glue/src/lib.rs").write_text("")
        (root / "glue/Cargo.toml").write_text(
            '[package]\nname = "glue"\nversion = "0.0.0"\nedition = "2021"\n[features]\npython = []\n'
            '[dependencies]\ndep = { path = "../dep" }\nderive = { path = "../derive" }\n'
            '[build-dependencies]\nbuilder = { path = "../builder" }\n[workspace]\n')
        subprocess.run(["cargo", "generate-lockfile", "--offline", "--manifest-path", root / "glue/Cargo.toml"], check=True,
                       capture_output=True)
        return root / "glue/Cargo.toml"

    def test_what_ships_is_what_the_library_links(self):
        with tempfile.TemporaryDirectory() as tmp:
            crate = self.crate(Path(tmp))
            first = package.sbom(info(tmp, name="moq-ffi", version="0.4.12"), crate)
            self.assertEqual(first, package.sbom(info(tmp, name="moq-ffi", version="0.4.12"), crate))
            document = json.loads(first)
            self.assertEqual(document["bomFormat"], "CycloneDX")
            # actions/attest rejects a CycloneDX file without one.
            self.assertRegex(document["serialNumber"], r"urn:uuid:[0-9a-f-]{36}")
            self.assertEqual(document["metadata"]["timestamp"], "2023-11-14T22:13:20Z")
            self.assertEqual([c["name"] for c in document["components"]], ["dep"])
            self.assertEqual(document["components"][0]["licenses"], [{"license": {"name": "MIT/Apache-2.0"}}])
            self.assertEqual(document["dependencies"], [
                {"ref": "pkg:cargo/dep@1.2.3", "dependsOn": []},
                {"ref": "pkg:cargo/moq-ffi@0.4.12", "dependsOn": ["pkg:cargo/dep@1.2.3"]},
            ])


    def test_one_version_from_two_sources_is_two_components(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = self.crate(root)
            # The same name and version again, from a git repository.
            fork = root / "fork"
            shutil.copytree(root / "dep", fork)
            git = ["git", "-C", str(fork), "-c", "user.name=t", "-c", "user.email=t@t"]
            subprocess.run([*git, "init", "-q"], check=True)
            subprocess.run([*git, "add", "."], check=True)
            subprocess.run([*git, "commit", "-q", "-m", "x"], check=True)
            manifest = crate.read_text().replace("[build-dependencies]", f'forked = {{ package = "dep", git = "file://{fork}" }}\n[build-dependencies]')
            crate.write_text(manifest)
            # A local repository, which cargo fetches without a network.
            subprocess.run(["cargo", "generate-lockfile", "--manifest-path", crate], check=True, capture_output=True)
            document = json.loads(package.sbom(info(tmp, name="moq-ffi", version="0.4.12"), crate))
            refs = [c["bom-ref"] for c in document["components"]]
            self.assertEqual(len(refs), 2)
            self.assertIn("pkg:cargo/dep@1.2.3", refs)
            self.assertTrue(any(ref.startswith("pkg:cargo/dep@1.2.3?vcs_url=git%2Bfile") for ref in refs), refs)


class Archive(unittest.TestCase):
    def test_members_that_differ_are_named(self):
        def ar(*members):
            out = b"!<arch>\n"
            for name, date, body in members:
                out += f"{name + '/':<16}{date:<12}{0:<6}{0:<6}{100644:<8}{len(body):<10}`\n".encode() + body + b"\n" * (len(body) % 2)
            return out
        with tempfile.TemporaryDirectory() as tmp:
            one, two = Path(tmp) / "one.a", Path(tmp) / "two.a"
            one.write_bytes(ar(("a.o", 0, b"same"), ("b.o", 0, b"before")))
            two.write_bytes(ar(("a.o", 0, b"same"), ("b.o", 5, b"before")))
            lines = package.ar_diff(one, two)
            self.assertEqual(len(lines), 1)
            self.assertIn("b.o", lines[0])


class ImportNames(unittest.TestCase):
    @staticmethod
    def archive(*members):
        out = b"!<arch>\n"
        for name, body in members:
            out += f"{name + '/':<16}{0:<12}{0:<6}{0:<6}{100644:<8}{len(body):<10}`\n".encode() + body + b"\n" * (len(body) % 2)
        return out

    def test_only_the_import_library_loses_its_random_directory(self):
        imported = b"_head_D__a_x_out_rustc4TX99l_kernel32_dll_imports_lib\x00"
        data = b"_out_rustcAbCdEf_kernel32_dll_imports_lib is a string the crate returns"
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "lib.a"
            path.write_bytes(self.archive(("", imported), ("kernel32.dllh.o", imported), ("api.o", data)))
            package.fix_import_names(path)
            members = package.ar_members(path)
            self.assertEqual(members["kernel32.dllh.o"][0], imported.replace(b"4TX99l", b"000000"))
            self.assertEqual(members["api.o"][0], data)


class BsdArchive(unittest.TestCase):
    def test_extended_names_are_read_from_the_data(self):
        body = b"a_long_object_name.o\0\0\0\0" + b"CONTENT"
        header = f"{'#1/24':<16}{0:<12}{0:<6}{0:<6}{100644:<8}{len(body):<10}`\n".encode()
        data = b"!<arch>\n" + header + body + b"\n" * (len(body) % 2)
        entries = package.ar_entries(data)
        self.assertEqual([(name, data[start:end]) for name, _, start, end in entries], [("a_long_object_name.o", b"CONTENT")])


class Purl(unittest.TestCase):
    def test_each_registry_is_its_own_package(self):
        def purl(source):
            return package.purl({"name": "dep", "version": "1.2.3", "source": source})
        crates_io = "registry+https://github.com/rust-lang/crates.io-index"
        self.assertEqual(purl(crates_io), "pkg:cargo/dep@1.2.3")
        self.assertEqual(purl(None), "pkg:cargo/dep@1.2.3")
        git = purl("registry+https://example.com/index")
        sparse = purl("sparse+https://example.com/index/")
        self.assertIn("?repository_url=https%3A%2F%2Fexample.com%2Findex", git)
        self.assertIn("?repository_url=https%3A%2F%2Fexample.com%2Findex%2F", sparse)
        self.assertEqual(len({purl(crates_io), git, sparse}), 3)


class Digest(unittest.TestCase):
    def test_any_byte_splits_the_hosts(self):
        with tempfile.TemporaryDirectory() as a, tempfile.TemporaryDirectory() as b:
            first, second = export(Path(a)), export(Path(b))
            self.assertEqual(package.digest(first, "moq"), package.digest(second, "moq"))
            # A Windows checkout once turned the embedded runtime CRLF.
            (second / "python/moq.py").write_bytes(b"PENDING = 0\r\n")
            self.assertNotEqual(package.digest(first, "moq"), package.digest(second, "moq"))
            (second / "python/moq.py").write_bytes(b"PENDING = 0\n")
            (second / "go/rt/rt.go").write_text("package rt\n")
            self.assertNotEqual(package.digest(first, "moq"), package.digest(second, "moq"))
            (second / "go/rt/rt.go").write_text((first / "go/rt/rt.go").read_text())
            (second / "kotlin/moq.pro").write_text("")
            self.assertNotEqual(package.digest(first, "moq"), package.digest(second, "moq"))


class Go(unittest.TestCase):
    def test_versions_and_major_suffixes(self):
        self.assertEqual(package.go_version("1.0.0-rc.1"), "v1.0.0-rc.1")
        with self.assertRaises(SystemExit):
            package.go_version("1.0.0+build")
        package.check_major("moq.dev/moq-ffi", "v1.2.3")
        package.check_major("moq.dev/moq-ffi/v2", "v2.0.0")
        for module, version in [("moq.dev/moq-ffi", "v2.0.0"), ("moq.dev/moq-ffi/v3", "v2.0.0"), ("moq.dev/x/v2", "v1.0.0")]:
            with self.assertRaises(SystemExit):
                package.check_major(module, version)

    def test_zip_root_of_a_nested_module(self):
        self.assertEqual(package.zip_root("example.com/api/v2@v2.0.0-rc.1/rt/rt.go"), ("example.com/api/v2", "v2.0.0-rc.1"))
        with self.assertRaises(SystemExit):
            package.zip_root("rt/rt.go")

    def test_module_path_as_go_reads_it(self):
        with tempfile.TemporaryDirectory() as root:
            go = Path(root) / "go"
            go.mkdir()
            for text, want in [("module example.com/own/v2 // a release\n", "example.com/own/v2"),
                               ('module "example.com/q"\n', "example.com/q"),
                               ("// module wrong\nmodule example.com/s\n\ngo 1.24\n", "example.com/s"),
                               ('module ( // the app\n\t"example.com/app"\n)\n', "example.com/app")]:
                (go / "go.mod").write_text(text)
                self.assertEqual(package.go_module_path(root), want)

    def test_escapes_capitals(self):
        self.assertEqual(package.escape("github.com/Azure/x"), "github.com/!azure/x")

    def test_links_from_the_cgo_line(self):
        with tempfile.TemporaryDirectory() as root:
            root = export(Path(root))
            self.assertEqual(package.go_links(root, "moq", "x86_64-unknown-linux-gnu"), "-lgcc_s -lc")
            (root / "go/rt/rt.go").write_text("package rt\n")
            with self.assertRaises(SystemExit):
                package.go_links(root, "moq", "x86_64-unknown-linux-gnu")

    def test_module_as_a_proxy_serves_it(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            built = export(root / "export")
            (root / "LICENSE-MIT").write_text("MIT\n")
            summary = package.go_module(info(root), built, go_libs(root), root / "proxy")
            self.assertIn("`rt/libmoq_windows_amd64.a`", summary)
            versions = root / "proxy/moq.dev/moq-ffi/@v"
            self.assertEqual((versions / "v0.4.12.mod").read_text(), "module moq.dev/moq-ffi\n\ngo 1.24\n")
            self.assertEqual(json.loads((versions / "v0.4.12.info").read_text()), {"Version": "v0.4.12"})
            self.assertEqual((versions / "list").read_text(), "v0.4.12\n")
            with zipfile.ZipFile(versions / "v0.4.12.zip") as z:
                self.assertEqual(sorted(z.namelist()), [f"moq.dev/moq-ffi@v0.4.12/{n}" for n in [
                    "LICENSE-MIT", "go.mod", "rt/libmoq_android_amd64.a", "rt/libmoq_android_arm64.a",
                    "rt/libmoq_darwin_amd64.a", "rt/libmoq_darwin_arm64.a", "rt/libmoq_ios_arm64.a",
                    "rt/libmoq_linux_amd64.a", "rt/libmoq_linux_arm64.a", "rt/libmoq_windows_amd64.a", "rt/rt.go"]])
            # The module ships exactly the libraries the runtime links.
            (root / "libs/libmoq_linux_arm64.a").unlink()
            with self.assertRaises(SystemExit):
                package.go_module(info(root), built, root / "libs", root / "other")
            (root / "libs/libmoq_linux_arm64.a").write_bytes(b"!<arch>\n")
            rt = built / "go/rt/rt.go"
            rt.write_text(rt.read_text().replace("#cgo ios,arm64", "// none"))
            with self.assertRaises(SystemExit):
                package.go_module(info(root), built, root / "libs", root / "third")

    def test_refuses_names_a_case_folds_together(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            built = export(root / "export")
            (built / "go/RT.go").write_text("package rt\n")
            (built / "go/rt.go").write_text("package rt\n")
            if len(list((built / "go").glob("[Rr][Tt].go"))) != 2:
                self.skipTest("a case-insensitive file system holds one of the two names")
            with self.assertRaises(SystemExit):
                package.go_files(info(root), built, go_libs(root))


class Swift(unittest.TestCase):
    def test_linker_settings_by_platform(self):
        settings = package.linker_settings({
            "macos": "-framework Security -framework CoreFoundation -liconv -lSystem",
            "ios": "-framework Security -liconv -lSystem",
        })
        self.assertIn('.linkedFramework("Security")', settings)
        self.assertIn('.linkedFramework("CoreFoundation", .when(platforms: [.macOS]))', settings)
        self.assertIn('.linkedLibrary("iconv")', settings)

    def test_linker_settings_catalyst(self):
        settings = package.linker_settings({
            "macos": "-framework AppKit -lSystem",
            "ios": "-framework UIKit -lSystem",
            "maccatalyst": "-framework UIKit -lSystem",
        })
        self.assertIn('.linkedFramework("UIKit", .when(platforms: [.iOS, .macCatalyst]))', settings)
        self.assertIn('.linkedLibrary("System")', settings)

    def test_api_names(self):
        self.assertEqual(package.api_name("_stat"), "stat")
        self.assertEqual(package.api_name("_fstatat$INODE64"), "fstatat")
        self.assertEqual(package.api_name("_OBJC_CLASS_$_NSUserDefaults"), "OBJC_CLASS_$_NSUserDefaults")

    def test_required_apis(self):
        imports = "_fstat$INODE64\n_NSFileSystemFreeSize\n_NSURLCreationDateKey\n_memcpy\n"
        with mock.patch.object(package, "output", return_value=imports):
            self.assertEqual(package.required_apis("probe.dylib"), {
                "FileTimestamp": {"fstat", "NSURLCreationDateKey"},
                "DiskSpace": {"NSFileSystemFreeSize"},
            })
        with self.assertRaises(SystemExit) as raised:
            package.privacy_manifest("moq", {"DiskSpace": {"NSFileSystemFreeSize"}})
        self.assertIn("NSFileSystemFreeSize", str(raised.exception))

    def test_privacy_manifest_declares_reasons(self):
        found = {"FileTimestamp": {"fstat", "stat"}, "SystemBootTime": {"mach_absolute_time"}}
        plist = plistlib.loads(package.privacy_manifest("moq", found))
        self.assertEqual(plist, {"NSPrivacyAccessedAPITypes": [
            {"NSPrivacyAccessedAPIType": "NSPrivacyAccessedAPICategoryFileTimestamp",
             "NSPrivacyAccessedAPITypeReasons": ["C617.1", "3B52.1"]},
            {"NSPrivacyAccessedAPIType": "NSPrivacyAccessedAPICategorySystemBootTime",
             "NSPrivacyAccessedAPITypeReasons": ["35F9.1"]},
        ]})
        self.assertEqual(plistlib.loads(package.privacy_manifest("moq", {})), {"NSPrivacyAccessedAPITypes": []})

    def test_privacy_manifest_refuses_unknown_reasons(self):
        for category, api in (("DiskSpace", "statfs"), ("FileTimestamp or DiskSpace", "getattrlist"),
                              ("UserDefaults", "OBJC_CLASS_$_NSUserDefaults")):
            with self.assertRaises(SystemExit, msg=category) as raised:
                package.privacy_manifest("moq", {category: {api}, "FileTimestamp": {"stat"}})
            self.assertIn(api, str(raised.exception))

    def test_line_tables(self):
        def root(level):
            return ["--config", f'profile.release.debug="{level}"', "--config", 'profile.release.strip="none"']
        with tempfile.TemporaryDirectory() as raw, mock.patch.dict(os.environ):
            os.environ.pop("CARGO_PROFILE_RELEASE_DEBUG", None)
            os.environ.pop("CARGO_PROFILE_RELEASE_STRIP", None)
            manifest = Path(raw) / "Cargo.toml"
            manifest.write_text('[package]\nname = "x"\n')
            # Pinned even then, over a level a .cargo/config.toml may set.
            self.assertEqual(package.line_tables(manifest), root("line-tables-only"))
            for level in ("false", "0", '"none"', '"line-directives-only"'):
                manifest.write_text(f'[package]\nname = "x"\n\n[profile.release]\ndebug = {level}\n')
                self.assertEqual(package.line_tables(manifest), root("line-tables-only"), level)
            # A level holding them is kept, as cargo names it.
            for level, name in (("true", "full"), ("2", "full"), ('"full"', "full"), ("1", "limited"),
                                ('"limited"', "limited"), ('"line-tables-only"', "line-tables-only")):
                manifest.write_text(f'[package]\nname = "x"\n\n[profile.release]\ndebug = {level}\n')
                self.assertEqual(package.line_tables(manifest), root(name), level)
            for strip in ("true", '"symbols"', '"debuginfo"', "false", '"none"'):
                manifest.write_text(f'[package]\nname = "x"\n\n[profile.release]\ndebug = 1\nstrip = {strip}\n')
                self.assertEqual(package.line_tables(manifest), root("limited"), strip)
            # A package's strip would drop them from a shared library too.
            manifest.write_text('[package]\nname = "x"\n\n[profile.release]\ndebug = 1\n\n'
                                '[profile.release.package.x-ffi]\nstrip = "debuginfo"\ndebug = 0\n\n'
                                '[profile.release.package."*"]\nopt-level = 3\ndebug = 2\n')
            self.assertEqual(package.line_tables(manifest), [
                *root("limited"),
                "--config", 'profile.release.package."x-ffi".debug="line-tables-only"',
                "--config", 'profile.release.package."x-ffi".strip="none"'])
            # The environment overrides the profile, as it does for cargo.
            for level in ("0", "false", "none", "line-directives-only"):
                os.environ["CARGO_PROFILE_RELEASE_DEBUG"] = level
                self.assertEqual(package.line_tables(manifest)[:4], root("line-tables-only"), level)
            for level, name in (("1", "limited"), ("2", "full"), ("true", "full"), ("limited", "limited"),
                                ("full", "full"), ("line-tables-only", "line-tables-only")):
                os.environ["CARGO_PROFILE_RELEASE_DEBUG"] = level
                self.assertEqual(package.line_tables(manifest)[:4], root(name), level)

    def test_java_package(self):
        self.assertEqual(package.java_package("dev.moq.moq"), "dev.moq.moq")
        self.assertEqual(package.java_package("io.7mind.my-lib"), "io._7mind.my_lib")
        self.assertEqual(package.java_package("com.int.new"), "com.int_.new_")

    def test_main_classes(self):
        # javap's listing of kotlinc's classes: a suspend main's bridge, a companion's main that
        # is no entry point, a renamed file's vararg main, and another file's main.
        listing = """Compiled from "check.kt"
public final class x.Foo$Companion {
  public final void main();
}
Compiled from "check.kt"
public final class x.CheckKt {
  public static final java.lang.Object main(kotlin.coroutines.Continuation<? super kotlin.Unit>);
  public static void main(java.lang.String[]);
}
Compiled from "check.kt"
public final class y.z.Named {
  public static final void main(java.lang.String...);
}
Compiled from "other.kt"
public final class OtherKt {
  public static final void main(java.lang.String[]);
}
public final class rust2.check.Check extends android.app.Instrumentation {
  public static void main(java.lang.String[]);
}
"""
        self.assertEqual(package.main_classes(listing, "check.kt"), ["x.CheckKt", "y.z.Named"])
        self.assertEqual(package.main_classes(listing, "other.kt"), ["OtherKt"])
        self.assertEqual(package.main_classes(listing, "none.kt"), [])

    def test_unknown_linker_flags_fail(self):
        with self.assertRaises(SystemExit):
            package.linker_settings({"macos": "-Wl,-dead_strip"})

    def test_manifest(self):
        text = package.package_swift("Moq", "1.7.5", ['.linkedLibrary("iconv")'],
                                     '.binaryTarget(name: "MoqFFI", url: "https://x/MoqFFI.xcframework.zip", checksum: "00")')
        self.assertIn('platforms: [.macOS("13.0"), .iOS("16.0"), .macCatalyst("16.0")]', text)
        self.assertIn('.target(name: "MoqPrivacy", resources: [.copy("PrivacyInfo.xcprivacy")])', text)
        self.assertIn('exact: "1.7.5"', text)
        self.assertIn('dependencies: ["MoqFFI", "MoqPrivacy", .product(name: "BasicContainers", package: "swift-collections")]', text)


class CArchive(unittest.TestCase):
    def test_versions_compat_and_soname(self):
        self.assertEqual(package.compat_version("1.4.2"), "1.0.0")
        self.assertEqual(package.compat_version("0.4.12"), "0.4.0")
        self.assertEqual(package.compat_version("0.0.3"), "0.0.3")
        self.assertEqual(package.compat_version("2.0.0-rc.1"), "2.0.0-rc.1")
        self.assertEqual(package.soname_version("1.2.3+build.5"), "1.2.3")
        self.assertEqual(package.soname_version("2.0.0-rc.1"), "2.0.0-rc.1")
        for version in ("1.0", "01.2.3", "v1.2.3"):
            with self.assertRaises(SystemExit, msg=version):
                package.soname_version(version)

    def test_files_per_platform(self):
        self.assertEqual(package.c_files("moq", "x86_64-unknown-linux-gnu", "0.4.12")["shared"], "lib/libmoq.so.0.4.12")
        self.assertEqual(package.c_files("moq", "aarch64-apple-darwin", "1.0.0")["shared"], "lib/libmoq.1.0.0.dylib")
        windows = package.c_files("moq", "x86_64-pc-windows-msvc", "1.0.0")
        self.assertEqual((windows["static"], windows["shared"], windows["implib"]),
                         ("lib/moq.lib", "bin/moq.dll", "lib/moq.dll.lib"))

    def test_native_libraries_by_platform(self):
        self.assertEqual(package.native_libs("-lgcc_s -lutil -lc", "x86_64-unknown-linux-gnu"),
                         [("lib", "gcc_s"), ("lib", "util"), ("lib", "c")])
        self.assertEqual(package.native_libs("-framework Security -liconv", "aarch64-apple-darwin"),
                         [("framework", "Security"), ("lib", "iconv")])
        self.assertEqual(package.native_libs("kernel32.lib ntdll.lib /defaultlib:msvcrt", "x86_64-pc-windows-msvc"),
                         [("lib", "kernel32"), ("lib", "ntdll"), ("flag", "/defaultlib:msvcrt")])
        for flags, target in (("-Wl,-dead_strip", "aarch64-apple-darwin"), ("-framework Security", "x86_64-unknown-linux-gnu"),
                              ("-lkernel32", "x86_64-pc-windows-msvc")):
            with self.assertRaises(SystemExit, msg=flags):
                package.native_libs(flags, target)

    def test_descriptions(self):
        natives = [("framework", "Security"), ("lib", "iconv")]
        files = package.c_files("moq", "aarch64-apple-darwin", "0.4.12")
        pc = package.pkg_config(info("/src"), natives)
        self.assertIn("Libs: -L${libdir} -lmoq\nLibs.private: -framework Security -liconv\n", pc)
        self.assertIn("Description: Media over QUIC, bindings\n", pc)
        config = package.cmake_config(info("/src"), "aarch64-apple-darwin", files, natives)
        self.assertIn("add_library(moq::moq STATIC IMPORTED)", config)
        self.assertIn("add_library(moq::moq_shared SHARED IMPORTED)", config)
        self.assertIn('IMPORTED_LOCATION "${_moq_prefix}/lib/libmoq.0.4.12.dylib"', config)
        self.assertIn("INTERFACE_LINK_LIBRARIES [==[-framework Security;iconv]==]", config)
        self.assertEqual(config.count('INTERFACE_COMPILE_FEATURES "c_std_11;cxx_std_17"'), 2)
        self.assertIn('VERSION_LESS "0.4.0"', package.cmake_version(info("/src")))
        cps = json.loads(package.cps(info("/src"), files, natives))
        self.assertEqual((cps["name"], cps["version"], cps["compat_version"]), ("moq", "0.4.12", "0.4.0"))
        self.assertEqual(cps["default_components"], ["moq"])
        self.assertEqual(cps["components"]["moq"]["link_libraries"], ["-framework Security", "iconv"])
        self.assertEqual(cps["components"]["moq_shared"]["location"], "@prefix@/lib/libmoq.0.4.12.dylib")
        self.assertEqual(cps["website"], "https://github.com/moq-dev/moq")
        self.assertEqual(cps["version_schema"], "simple")
        prerelease = info("/src", version="2.0.0-rc.1")
        self.assertEqual(json.loads(package.cps(prerelease, files, natives))["version_schema"], "custom")
        self.assertIn("set(PACKAGE_VERSION_COMPATIBLE FALSE)\n", package.cmake_version(prerelease))
        self.assertNotIn("VERSION_LESS", package.cmake_version(prerelease))
        windows = package.cps(info("/src"), package.c_files("moq", "x86_64-pc-windows-msvc", "0.4.12"),
                              [("lib", "ntdll"), ("flag", "/defaultlib:msvcrt")])
        windows = json.loads(windows)["components"]
        self.assertEqual(windows["moq_shared"]["link_location"], "@prefix@/lib/moq.dll.lib")
        self.assertEqual(windows["moq"]["link_flags"], ["/defaultlib:msvcrt"])

    def test_the_older_toolchain_is_a_stable_release_whatever_the_channel(self):
        self.assertEqual(package.older_toolchain("rustc 1.98.1\nhost: x86_64-unknown-linux-gnu\nrelease: 1.98.1\n"), "1.96.0")
        # A nightly's release is two ahead of stable, its beta one.
        self.assertEqual(package.older_toolchain("release: 1.102.0-nightly\n"), "1.100.0")
        with self.assertRaises(SystemExit):
            package.older_toolchain("release: 2.0.0\n")

    def test_archive_is_owned_by_no_one(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "moq-ffi-0.4.12-x86_64-unknown-linux-gnu"
            (root / "lib").mkdir(parents=True)
            (root / "lib/libmoq.so.0.4.12").write_bytes(b"\x7fELF")
            (root / "lib/libmoq.so.0.4.12").chmod(0o700)
            (root / "lib/libmoq.so").symlink_to("libmoq.so.0.4.12")
            (root / "lib/libmoq.a").write_bytes(b"!<arch>\n")
            (root / "lib/libmoq.a").chmod(0o600)
            path = package.pack(root, Path(tmp) / "out.tar.gz", 1_700_000_000)
            with tarfile.open(path) as archive:
                members = {m.name.split("/", 1)[-1]: m for m in archive.getmembers()}
            self.assertEqual(members["lib/libmoq.so.0.4.12"].mode, 0o755)
            self.assertEqual(members["lib/libmoq.a"].mode, 0o644)
            self.assertEqual(members["lib/libmoq.so"].linkname, "libmoq.so.0.4.12")
            self.assertEqual({(m.uid, m.uname) for m in members.values()}, {(0, "")})
            self.assertEqual({m.mtime for m in members.values()}, {1_700_000_000})

    def test_archives_are_the_same_bytes_whenever_and_wherever_packed(self):
        with tempfile.TemporaryDirectory() as tmp:
            made = []
            for n, (touch, suffix) in enumerate(((1, ".tar.gz"), (2_000_000_000, ".tar.gz"), (1, ".zip"), (2_000_000_000, ".zip"))):
                base = Path(tmp) / f"base{n}"
                root = base / "moq-ffi-0.4.12-x"
                (root / "lib").mkdir(parents=True)
                for name in ("lib/b.txt", "lib/a.txt", "z.txt"):
                    (root / name).write_text(name)
                    os.utime(root / name, (touch, touch))
                (root / "lib/a.txt").chmod(0o640 if n % 2 else 0o600)
                made.append(package.pack(root, Path(tmp) / f"out{n}{suffix}", 1_700_000_000).read_bytes())
            self.assertEqual(made[0], made[1])
            self.assertEqual(made[2], made[3])


def dart_export(root, version="0.4.12"):
    """The Dart package an export writes, as `dart::pubspec` lays out its pubspec."""
    dart = root / "dart"
    (dart / "lib").mkdir(parents=True)
    (dart / "hook").mkdir()
    (dart / "lib/moq.dart").write_text("library;\n")
    (dart / "hook/build.dart").write_text("void main() {}\n")
    (dart / "pubspec.yaml").write_text(
        "# Generated by rust2-ffi.\nname: moq\ndescription: The pub API of moq-ffi.\n"
        f"version: {version}\n\nenvironment:\n  sdk: ^3.10.0\n")
    return root


def dart_libs(root, facts, skip=()):
    """A library of each Dart target, as `build` names it, its bytes its target."""
    root.mkdir()
    for target in package.DART_TARGETS:
        if target not in skip:
            (root / package.dart_file(facts, target)).write_text(target)
    return root


def dart_privacy(root, found):
    """Each Apple Dart library's privacy manifest, as `build` writes them, from `found`, its
    required-reason APIs by target."""
    root.mkdir()
    for target in (t for t in package.DART_TARGETS if "apple" in t):
        (root / f"{target}.xcprivacy").write_bytes(package.privacy_manifest("moq", found.get(target, {})))
    return root


class PubPackage(unittest.TestCase):
    URL = "https://github.com/moq-dev/moq/releases/download/moq-ffi-v0.4.12/"

    def make(self, facts, libs=None, changes="## API\n\nAdded `moq::Track`.\n", url=URL, privacy=None):
        tmp = Path(self.enterContext(tempfile.TemporaryDirectory()))
        export = dart_export(tmp / "export")
        (tmp / "changes.md").write_text(changes)
        out = tmp / "pub"
        package.pub_package(facts, export, libs or dart_libs(tmp / "libs", facts), url, tmp / "changes.md",
                            privacy or dart_privacy(tmp / "privacy", {}), out)
        return out

    def test_declares_every_apple_librarys_apis(self):
        tmp = Path(self.enterContext(tempfile.TemporaryDirectory()))
        privacy = dart_privacy(tmp / "privacy", {"aarch64-apple-ios": {"FileTimestamp": {"fstat"}},
                                                 "x86_64-apple-darwin": {"SystemBootTime": {"mach_absolute_time"},
                                                                         "FileTimestamp": {"stat"}}})
        with open(self.make(info(tmp), privacy=privacy) / "PrivacyInfo.xcprivacy", "rb") as f:
            declared = package.plistlib.load(f)["NSPrivacyAccessedAPITypes"]
        self.assertEqual(declared, [
            {"NSPrivacyAccessedAPIType": "NSPrivacyAccessedAPICategoryFileTimestamp",
             "NSPrivacyAccessedAPITypeReasons": ["C617.1", "3B52.1"]},
            {"NSPrivacyAccessedAPIType": "NSPrivacyAccessedAPICategorySystemBootTime",
             "NSPrivacyAccessedAPITypeReasons": ["35F9.1"]},
        ])
        # An Apple library's manifest missing fails the package.
        (privacy / "aarch64-apple-ios-sim.xcprivacy").unlink()
        with self.assertRaises(SystemExit):
            self.make(info(tmp), privacy=privacy)

    def test_names_each_library_by_size_hash_and_floor(self):
        tmp = Path(self.enterContext(tempfile.TemporaryDirectory()))
        out = self.make(info(tmp))
        release = json.loads((out / "hook/release.json").read_text())
        self.assertEqual(release["url"], self.URL)
        self.assertEqual(set(release["libraries"]), set(package.DART_TARGETS))
        android = release["libraries"]["aarch64-linux-android"]
        self.assertEqual(android, {"file": "moq-ffi-0.4.12-dart-aarch64-linux-android.so", "size": 21,
                                   "sha256": package.hashlib.sha256(b"aarch64-linux-android").hexdigest(),
                                   "minimum": "24"})
        self.assertEqual(release["libraries"]["aarch64-apple-ios-sim"]["file"],
                         "moq-ffi-0.4.12-dart-aarch64-apple-ios-sim.dylib")
        self.assertEqual(release["libraries"]["x86_64-pc-windows-msvc"]["file"], "moq-ffi-0.4.12-dart-x86_64-pc-windows-msvc.dll")
        # Every arm64 Mac runs macOS 11, so no app is too old for its library.
        self.assertNotIn("minimum", release["libraries"]["aarch64-apple-darwin"])
        self.assertEqual(release["libraries"]["x86_64-apple-darwin"]["minimum"], "10.15")
        self.assertTrue((out / "lib/moq.dart").is_file() and (out / "hook/build.dart").is_file())

    def test_states_cargos_facts(self):
        tmp = Path(self.enterContext(tempfile.TemporaryDirectory()))
        pubspec = (self.make(info(tmp, homepage="https://moq.dev")) / "pubspec.yaml").read_text()
        self.assertIn('\ndescription: "Media over QUIC, bindings"\nversion: 0.4.12\n'
                      'repository: "https://github.com/moq-dev/moq"\nhomepage: "https://moq.dev"\n\nenvironment:', pubspec)
        # Without Cargo's, the export's own description stays.
        pubspec = (self.make(info(tmp, description=None, repository=None)) / "pubspec.yaml").read_text()
        self.assertIn("\ndescription: The pub API of moq-ffi.\nversion: 0.4.12\n\n", pubspec)

    def test_license_readme_and_changelog(self):
        tmp = Path(self.enterContext(tempfile.TemporaryDirectory()))
        (tmp / "LICENSE-MIT").write_text("MIT text\n")
        (tmp / "LICENSE-APACHE").write_text("Apache text\n")
        (tmp / "README.md").write_text("# moq\n")
        out = self.make(info(tmp, readme="README.md"))
        self.assertEqual((out / "LICENSE").read_text(), "Apache text\n\nMIT text\n")
        self.assertEqual((out / "README.md").read_text(), "# moq\n")
        self.assertEqual((out / "CHANGELOG.md").read_text(), "## 0.4.12\n\n### API\n\nAdded `moq::Track`.\n")
        bare = Path(self.enterContext(tempfile.TemporaryDirectory()))
        out = self.make(info(bare))
        self.assertEqual((out / "LICENSE").read_text(), "SPDX-License-Identifier: MIT OR Apache-2.0\n")
        self.assertEqual((out / "README.md").read_text(),
                         "# moq\n\nMedia over QUIC, bindings\n\nThe Dart binding of the Rust crate moq-ffi, generated by rust2-ffi.\n")
        with self.assertRaises(SystemExit):
            self.make(info(bare, license=None))

    def test_refuses_a_missing_or_stray_library_and_a_bad_url(self):
        tmp = Path(self.enterContext(tempfile.TemporaryDirectory()))
        facts = info(tmp)
        with self.assertRaises(SystemExit):
            self.make(facts, libs=dart_libs(tmp / "some", facts, skip={"x86_64-linux-android"}))
        stray = dart_libs(tmp / "stray", facts)
        (stray / "moq-ffi-0.4.11-dart-x86_64-linux-android.so").write_text("old")
        with self.assertRaises(SystemExit):
            self.make(facts, libs=stray)
        with self.assertRaises(SystemExit):
            self.make(facts, url=self.URL.rstrip("/"))
        with self.assertRaises(SystemExit):
            self.make(info(tmp, version="0.4.13"))


class FlutterLog(unittest.TestCase):
    """What the Flutter consumer reads from the device's log, in logcat's brief format."""

    def log(self, *lines):
        log = object.__new__(package.Logcat)
        log.lines, log.pid, log.vm_service = queue.Queue(), None, None
        for line in lines:
            log.lines.put(line)
        log.lines.put(None)
        return log

    def output(self, *lines):
        with mock.patch("sys.stderr"):
            return self.log(*lines).output()

    START = ("--------- beginning of main", "I/flutter ( 4321): engine up", "I/flutter ( 4321): rust2: start")
    LISTENING = "I/flutter ( 4321): The Dart VM service is listening on http://127.0.0.1:37635/Vfh1w_Ak2ZQ=/"

    def test_decodes_the_check_output(self):
        self.assertEqual(self.output(*self.START, "I/flutter ( 4321): rust2: aGk=", "I/flutter ( 4321): rust2: exit 0"), b"hi")

    def test_finds_the_vm_service_before_or_after_the_check(self):
        url = "http://127.0.0.1:37635/Vfh1w_Ak2ZQ=/"
        for lines in ((self.LISTENING, *self.START, "I/flutter ( 4321): rust2: exit 0"),
                      (*self.START, "I/flutter ( 4321): rust2: exit 0", self.LISTENING)):
            with self.subTest(lines), mock.patch("sys.stderr"):
                log = self.log(*lines)
                self.assertEqual(log.output(), b"")
                self.assertEqual(log.listening(), url)

    def test_fails_on_a_crash_before_the_vm_service(self):
        with mock.patch("sys.stderr"), self.assertRaisesRegex(SystemExit, "crashed"):
            log = self.log(*self.START, "I/flutter ( 4321): rust2: exit 0",
                           "F/libc    ( 4321): Fatal signal 6 (SIGABRT), code -6 in tid 4340 (1.ui)")
            log.output()
            log.listening()

    def test_fails_on_the_apps_crash(self):
        for line in ("F/libc    ( 4321): Fatal signal 6 (SIGABRT), code -6 in tid 4340 (1.ui)",
                     "F/DEBUG   (  999): pid: 4321, tid: 4340, name: 1.ui  >>> dev.rust2.consumer <<<",
                     "E/AndroidRuntime( 4321): Process: dev.rust2.consumer, PID: 4321"):
            with self.subTest(line), self.assertRaisesRegex(SystemExit, "crashed"):
                self.output(*self.START, line)

    def test_ignores_another_process_crash(self):
        self.assertEqual(self.output(
            *self.START, "F/libc    (  777): Fatal signal 11 (SIGSEGV)",
            "F/DEBUG   (  999): pid: 777, tid: 777, name: foo  >>> com.android.phone <<<",
            "E/AndroidRuntime(  777): Process: com.android.phone, PID: 777",
            "I/flutter ( 4321): rust2: exit 0"), b"")

    def test_fails_on_a_nonzero_exit(self):
        with self.assertRaisesRegex(SystemExit, "exit 1"):
            self.output(*self.START, "I/flutter ( 4321): rust2: exit 1")


class Sdist(unittest.TestCase):
    def test_moves_each_local_path(self):
        manifest = ('[dependencies]\napi_export = { package = "api", path = "/src/api", features = ["x"] }\n'
                    'serde = "1"\n[target."cfg(unix)".dependencies]\ntls = { path = "/src/tls" }\n'
                    '[patch.crates-io.url]\npath = "/vendor/url"\n')
        moves = {"/src/api": "../sources/src/api", "/src/tls": "../sources/src/tls", "/vendor/url": "../sources/vendor/url"}
        moved = tomllib.loads(package.relocate(manifest, moves, strict=True))
        self.assertEqual(moved["dependencies"]["api_export"], {"package": "api", "path": "../sources/src/api", "features": ["x"]})
        self.assertEqual(moved["target"]["cfg(unix)"]["dependencies"]["tls"]["path"], "../sources/src/tls")
        self.assertEqual(moved["patch"]["crates-io"]["url"]["path"], "../sources/vendor/url")

    def test_moves_each_path_once(self):
        # One package's new path is another's old one, as relative keys can make: each path
        # moves by its original value alone.
        manifest = '[dependencies]\na = { path = "../a" }\nb = { path = "../sources/a" }\n'
        moves = {"../a": "../sources/a", "../sources/a": "../sources/sources/a"}
        moved = tomllib.loads(package.relocate(manifest, moves, strict=True))
        self.assertEqual(moved["dependencies"], {"a": {"path": "../sources/a"}, "b": {"path": "../sources/sources/a"}})

    def test_refuses_a_path_no_package_lies_at(self):
        manifest = '[dependencies]\na = { path = "/src/a" }\nb = { path = "/elsewhere/b" }\n'
        with self.assertRaises(SystemExit):
            package.relocate(manifest, {"/src/a": "../sources/a"}, strict=True)
        # A package's own manifest may name what its build never loads, such as a dev-dependency.
        moved = tomllib.loads(package.relocate(manifest, {"/src/a": "../a"}))
        self.assertEqual(moved["dependencies"], {"a": {"path": "../a"}, "b": {"path": "/elsewhere/b"}})

    def test_a_dependency_named_path_is_searched(self):
        manifest = ('[dependencies]\npath = { package = "helper", path = "/src/helper" }\n'
                    '[target.x.dependencies]\nb = { path = "../b" }\n')
        self.assertEqual(package.manifest_paths(tomllib.loads(manifest)), ["/src/helper", "../b"])
        moved = tomllib.loads(package.relocate(manifest, {"/src/helper": "../helper", "../b": "../b"}, strict=True))
        self.assertEqual(moved["dependencies"]["path"], {"package": "helper", "path": "../helper"})

    def test_refuses_a_path_spelled_otherwise(self):
        with self.assertRaises(SystemExit):
            package.relocate("[dependencies]\na = { path = '/src/a' }\n", {"/src/a": "../a"})

    def test_a_root_package_keeps_its_workspace_alone(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "Cargo.toml"
            root.write_text('cargo-features = ["edition2027"]\n[package]\nname = "root"\n[dependencies]\nx = "1"\n'
                            '[workspace]\nmembers = ["fixtures/*"]\n[workspace.package]\nversion = "0.4.12"\n'
                            '[workspace.dependencies]\nkio = { path = "rs/kio", features = ["runtime"] }\n'
                            '[workspace.lints.clippy]\nall = { level = "warn", priority = -1 }\n'
                            'quoted = "a \\"b\\" \\u00e9"\n[profile.release]\nlto = true\n')
            reduced = tomllib.loads(package.workspace_manifest(root))
        self.assertEqual(set(reduced), {"cargo-features", "workspace"})
        self.assertEqual(reduced["workspace"]["dependencies"]["kio"], {"path": "rs/kio", "features": ["runtime"]})
        self.assertEqual(reduced["workspace"]["lints"]["clippy"]["all"], {"level": "warn", "priority": -1})
        self.assertEqual(reduced["workspace"]["lints"]["clippy"]["quoted"], 'a "b" \u00e9')

    def test_moves_a_reduced_roots_absolute_paths(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "Cargo.toml"
            root.write_text('[workspace]\nmembers = ["rs/*"]\n[workspace.dependencies]\n'
                            'kio = { path = "/src/kio", "odd key" = 1 }\n')
            moved = package.relocate(package.workspace_manifest(root), {"/src/kio": "kio"})
        self.assertEqual(tomllib.loads(moved)["workspace"]["dependencies"]["kio"], {"path": "kio", "odd key": 1})

    def test_pins_the_channel_alone(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            crate = root / "rs" / "moq-ffi"
            crate.mkdir(parents=True)
            self.assertIsNone(package.toolchain(crate))
            (root / "rust-toolchain.toml").write_text('[toolchain]\nchannel = "1.98.1"\ncomponents = ["clippy"]\n')
            self.assertEqual(tomllib.loads(package.toolchain(crate)),
                             {"toolchain": {"channel": "1.98.1", "profile": "minimal"}})
            # The legacy file, nearer, wins, as rustup reads it.
            (crate / "rust-toolchain").write_text("nightly-2026-11-12\n")
            self.assertEqual(tomllib.loads(package.toolchain(crate))["toolchain"]["channel"], "nightly-2026-11-12")
            (crate / "rust-toolchain").write_text('[toolchain]\npath = "/opt/rust"\n')
            with self.assertRaises(SystemExit):
                package.toolchain(crate)

    def test_builds_on_every_target_it_stages(self):
        targets = tomllib.loads(package.pyproject(["x86_64-unknown-linux-musl"]))
        self.assertEqual(targets["build-system"], {"requires": [], "build-backend": "backend", "backend-path": ["rust2"]})
        self.assertEqual(targets["tool"]["rust2-ffi"]["targets"], ["x86_64-unknown-linux-musl"])
        # The builder stages each sdist target's API on some host.
        workflow = (package.ROOT / ".github" / "workflows" / "bindings.yml").read_text(encoding="utf-8")
        for target in package.SDIST_TARGETS:
            self.assertRegex(workflow, rf"\n            targets: .*\b{target}\b")

    def test_archive_and_its_rebuild(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            files = {"PKG-INFO": (b"Metadata-Version: 2.4\nName: moq-ffi\nVersion: 0.4.12rc1\n\n", False),
                     "sources/kio/configure": (b"#!/bin/sh\n", True), "binding/Cargo.toml": (b"[package]\n", False)}
            path = backend.write_sdist(root, "moq-ffi", "0.4.12rc1", files, 1700000000)
            self.assertEqual(path.name, "moq_ffi-0.4.12rc1.tar.gz")
            with tarfile.open(path) as tar:
                members = tar.getmembers()
                self.assertEqual([m.name for m in members], [f"moq_ffi-0.4.12rc1/{n}" for n in sorted(files)])
                self.assertEqual({m.name.split("/", 1)[1]: (m.mode, m.mtime) for m in members},
                                 {"PKG-INFO": (0o644, 1700000000), "binding/Cargo.toml": (0o644, 1700000000),
                                  "sources/kio/configure": (0o755, 1700000000)})
                tar.extractall(root / "tree", filter="data")
            tree = root / "tree" / "moq_ffi-0.4.12rc1"
            # What a build wrote is no source.
            (tree / "binding" / "target" / "release").mkdir(parents=True)
            (tree / "binding" / "target" / "release" / "libmoq.so").write_bytes(b"\x7fELF")
            (tree / "rust2" / "__pycache__").mkdir(parents=True)
            (tree / "rust2" / "__pycache__" / "backend.pyc").write_bytes(b"")
            (tree / "dist").mkdir()
            (tree / "dist" / "moq_ffi-0.4.12rc1.tar.gz").write_bytes(path.read_bytes())
            with mock.patch.object(Path, "cwd", return_value=tree), mock.patch.dict(os.environ, clear=True):
                (tree / "dist" / backend.build_sdist(tree / "dist")).replace(root / "again.tar.gz")
            # The same bytes, stamped with the time the sdist's files carry.
            self.assertEqual((root / "again.tar.gz").read_bytes(), path.read_bytes())


class Backend(unittest.TestCase):
    def test_macos_tags_as_pip_reads_them(self):
        with mock.patch.object(backend.sys, "platform", "darwin"):
            for target, asked, built, tag in [("aarch64-apple-darwin", None, "11.0", "macosx_11_0_arm64"),
                                              ("x86_64-apple-darwin", None, "10.12", "macosx_10_12_x86_64"),
                                              ("x86_64-apple-darwin", "10.15", "10.15", "macosx_10_15_x86_64"),
                                              ("aarch64-apple-darwin", "14.2", "14.2", "macosx_14_0_arm64"),
                                              # Older than the target's oldest, which rustc builds for instead.
                                              ("x86_64-apple-darwin", "10.9", "10.12", "macosx_10_12_x86_64"),
                                              ("aarch64-apple-darwin", "10.15", "11.0", "macosx_11_0_arm64")]:
                env = {} if asked is None else {"MACOSX_DEPLOYMENT_TARGET": asked}
                self.assertEqual(backend.platform_tag(target, env), tag)
                # cargo builds for the floor the tag claims.
                self.assertEqual(env["MACOSX_DEPLOYMENT_TARGET"], built)
            with self.assertRaises(SystemExit):
                backend.platform_tag("aarch64-apple-darwin", {"MACOSX_DEPLOYMENT_TARGET": "sonoma"})

    def test_musl_links_its_c_runtime_dynamically(self):
        flag = "-Ctarget-feature=-crt-static"
        self.assertEqual(backend.dynamic_crt("x86_64-unknown-linux-gnu", {}), [])
        self.assertEqual(backend.dynamic_crt("x86_64-unknown-linux-musl", {}),
                         ["--config", f'target.x86_64-unknown-linux-musl.rustflags = ["{flag}"]'])
        # Either variable, even empty, is all cargo reads, so the flag joins it.
        for name, value, want in [("RUSTFLAGS", "", flag), ("RUSTFLAGS", "-Copt-level=2", f"-Copt-level=2 {flag}"),
                                  ("CARGO_ENCODED_RUSTFLAGS", "-Copt-level=2", f"-Copt-level=2\x1f{flag}")]:
            env = {name: value}
            self.assertEqual(backend.dynamic_crt("aarch64-unknown-linux-musl", env), [])
            self.assertEqual(env[name], want)

    def test_refuses_an_emulated_python(self):
        with mock.patch.object(backend.platform, "machine", return_value="x86_64"), \
                mock.patch.object(backend.os, "name", "posix"):
            backend.check_arch("x86_64-apple-darwin")
            with self.assertRaises(SystemExit):
                backend.check_arch("aarch64-apple-darwin")
            # An architecture apart from these two is no emulation to catch.
            backend.check_arch("riscv64gc-unknown-linux-gnu")


class Template(unittest.TestCase):
    def test_fills_every_identity(self):
        text = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        self.assertIn("uses: kixel-ai/rust2/.github/workflows/bindings.yml@" + "a" * 40, text)
        self.assertIn('- "moq-ffi-v*"', text)
        self.assertIn("url: https://pypi.org/p/moq-ffi", text)
        self.assertIn("owner: moq-dev\n", text)
        self.assertIn("repositories: moq-swift-ffi\n", text)
        self.assertIn("go-module: moq.dev/moq-ffi\n", text)
        self.assertIn("name: moq-ffi-pub\n", text)
        self.assertIn('"$XCFRAMEWORK" release/api.json dist/c/* dist/dart/*', text)
        self.assertNotRegex(text, r"@[A-Z_]+@")

    def test_refuses_a_movable_pin_or_bad_mirror(self):
        with self.assertRaises(SystemExit):
            package.template(info("/src"), "Cargo.toml", "main", "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        with self.assertRaises(SystemExit):
            package.template(info("/src"), "Cargo.toml", "a" * 40, "moq-swift-ffi", "moq.dev/moq-ffi")
        for module in ["moq", "moq.dev/moq-ffi/", "moq.dev/moq-ffi/v2"]:
            with self.assertRaises(SystemExit):
                package.template(info("/src"), "Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", module)

    def test_web_lines_follow_the_scope(self):
        shipped = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi", "moq")
        self.assertIn("\n      web-manifest-path: rs/moq-ffi/Cargo.toml\n      scope: moq\n", shipped)
        self.assertIn("@moq/moq-ffi on npm and JSR, and the React Native package, @moq/moq-ffi-rn on npm", shipped)
        none = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        self.assertIn("      # web-manifest-path: rs/moq-ffi/Cargo.toml\n", none)
        self.assertNotIn("\n      web-manifest-path:", none)
        self.assertNotRegex(none, r"@[A-Z_]+@")

    def test_maven_lines_follow_the_group(self):
        args = (info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        shipped = package.template(*args, group="dev.moq")
        self.assertIn("\n      maven-group: dev.moq\n", shipped)
        self.assertIn("dev.moq:moq-ffi on Maven Central", shipped)
        self.assertIn("name: moq-ffi-maven", shipped)
        none = package.template(*args)
        self.assertIn("      # maven-group: <group>\n", none)
        self.assertNotIn("\n      maven-group:", none)
        self.assertNotRegex(none, r"@[A-Z_]+@")
        for group in ["moq", "dev.moq/", "dev..moq"]:
            with self.assertRaises(SystemExit):
                package.template(*args, group=group)


class Maven(unittest.TestCase):
    def fields(self, **overrides):
        text = package.pom(info("/src", **overrides), "dev.moq", "1.10.2")
        root = xml.etree.ElementTree.fromstring(text)
        ns = {"m": "http://maven.apache.org/POM/4.0.0"}
        return root, ns

    def test_pom_states_what_central_requires(self):
        root, ns = self.fields(authors=["Luke Curley <kixelated@gmail.com>", "Someone"])
        get = lambda path: [e.text for e in root.findall(path, ns)]  # noqa: E731
        self.assertEqual(get("m:groupId") + get("m:artifactId") + get("m:version") + get("m:packaging"),
                         ["dev.moq", "moq-ffi", "0.4.12", "jar"])
        # Gradle reads the .module, which gives Android the AAR, only when the POM says it has one.
        self.assertIn("<!-- do_not_remove: published-with-gradle-metadata -->", package.pom(info("/src"), "dev.moq", "1.10.2"))
        self.assertEqual(get("m:description"), ["Media over QUIC, bindings"])
        # No homepage: the repository is the project's URL.
        self.assertEqual(get("m:url"), ["https://github.com/moq-dev/moq"])
        # Each alternative of an OR, any of which a user may choose, as Maven reads several.
        self.assertEqual(get("m:licenses/m:license/m:name"), ["MIT", "Apache-2.0"])
        self.assertEqual(get("m:licenses/m:license/m:url"),
                         ["https://spdx.org/licenses/MIT.html", "https://spdx.org/licenses/Apache-2.0.html"])
        self.assertEqual(get("m:developers/m:developer/m:name"), ["Luke Curley", "Someone"])
        self.assertEqual(get("m:developers/m:developer/m:email"), ["kixelated@gmail.com"])
        self.assertEqual(get("m:scm/m:connection"), ["scm:git:https://github.com/moq-dev/moq.git"])
        self.assertEqual(get("m:dependencies/m:dependency/m:artifactId"), ["kotlin-stdlib", "kotlinx-coroutines-core"])
        self.assertEqual(get("m:dependencies/m:dependency/m:version"), [f"{package.KOTLIN_API}.0", "1.10.2"])

    def test_pom_derives_or_refuses_the_rest(self):
        # No authors: the repository's owner, who publishes it.
        root, ns = self.fields()
        self.assertEqual([e.text for e in root.findall("m:developers/m:developer/m:url", ns)], ["https://github.com/moq-dev"])
        # An expression more than a choice is one license, with no single URL.
        root, ns = self.fields(license="MIT AND (Apache-2.0 OR BSD-3-Clause)")
        self.assertEqual([e.text for e in root.findall("m:licenses/m:license/m:name", ns)], ["MIT AND (Apache-2.0 OR BSD-3-Clause)"])
        self.assertEqual(root.findall("m:licenses/m:license/m:url", ns), [])
        # Text is escaped.
        root, ns = self.fields(description="a <b> & c")
        self.assertEqual(root.find("m:description", ns).text, "a <b> & c")
        for missing in ({"description": None}, {"repository": None}, {"license": None}, {"license": "MIT/Apache-2.0"},
                        {"version": "1.0.0+build"}):
            with self.assertRaises(SystemExit, msg=missing):
                package.pom(info("/src", **missing), "dev.moq", "1.10.2")

    def test_module_gives_android_the_aar_and_the_jvm_the_jar(self):
        files = {f"moq-ffi-0.4.12{suffix}": suffix.encode() for suffix in (".jar", ".aar", "-sources.jar", "-javadoc.jar")}
        module = json.loads(package.gradle_module("dev.moq", "moq-ffi", "0.4.12", package.maven_dependencies("1.10.2"), files))
        self.assertEqual(module["component"], {"group": "dev.moq", "module": "moq-ffi", "version": "0.4.12",
                                               "attributes": {"org.gradle.status": "release"}})
        attribute = lambda v, *names: next(v["attributes"][n] for n in names if n in v["attributes"])  # noqa: E731
        chosen = {(attribute(v, "org.gradle.jvm.environment", "org.gradle.docstype"), v["attributes"]["org.gradle.usage"]):
                  v["files"][0]["name"] for v in module["variants"]}
        self.assertEqual(chosen, {("android", "java-api"): "moq-ffi-0.4.12.aar", ("android", "java-runtime"): "moq-ffi-0.4.12.aar",
                                  ("standard-jvm", "java-api"): "moq-ffi-0.4.12.jar", ("standard-jvm", "java-runtime"): "moq-ffi-0.4.12.jar",
                                  ("sources", "java-runtime"): "moq-ffi-0.4.12-sources.jar",
                                  ("javadoc", "java-runtime"): "moq-ffi-0.4.12-javadoc.jar"})
        jar = next(v for v in module["variants"] if v["name"] == "jvmRuntimeElements")
        self.assertEqual(jar["files"][0]["size"], 4)
        self.assertEqual(jar["files"][0]["sha256"], package.hashlib.sha256(b".jar").hexdigest())
        self.assertEqual([d["module"] for d in jar["dependencies"]], ["kotlin-stdlib", "kotlinx-coroutines-core"])
        self.assertEqual(jar["attributes"]["org.jetbrains.kotlin.platform.type"], "jvm")

    def test_prerelease_orders_before_its_release(self):
        self.assertEqual(package.maven_version("1.0.0-rc.1"), "1.0.0-rc.1")
        with self.assertRaises(SystemExit):
            package.maven_version("1.0.0-dev")


def web_export(root, version="0.4.12"):
    """The parts of an export's web/ directory a web package reads."""
    web = root / "web"
    web.mkdir(parents=True)
    for stem in ("index", "rt", "glue", "Session"):
        (web / f"{stem}.ts").write_text(f"// {stem}\n")
        (web / f"{stem}.js").write_text(f"// {stem}\n")
        (web / f"{stem}.d.ts").write_text(f"// {stem}\n")
    manifest = {"name": "moq-ffi", "version": version, "description": "generated", "type": "module",
                "sideEffects": False, "exports": {".": {"types": "./index.d.ts", "default": "./index.js"}},
                "files": ["*.js", "*.d.ts", "moq.wasm"]}
    (web / "package.json").write_text(package.json.dumps(manifest))
    (root / "moq.wasm").write_bytes(b"\0asm\1\0\0\0")
    return web


class Web(unittest.TestCase):
    def test_npm_and_jsr_layouts(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "LICENSE-MIT").write_text("MIT\n")
            web = web_export(root / "export")
            npm, jsr = package.web_package(info(root), web, root / "export/moq.wasm", "moq", root / "out")
            self.assertEqual(sorted(p.name for p in npm.iterdir()), [
                "LICENSE-MIT", "Session.d.ts", "Session.js", "glue.d.ts", "glue.js", "index.d.ts", "index.js",
                "moq.wasm", "package.json", "rt.d.ts", "rt.js"])
            self.assertEqual(sorted(p.name for p in jsr.iterdir()), [
                "LICENSE-MIT", "Session.ts", "glue.ts", "index.ts", "jsr.json", "moq.wasm", "rt.ts"])
            manifest = package.json.loads((npm / "package.json").read_text())
            self.assertEqual(list(manifest)[:6], ["name", "version", "description", "license", "repository", "type"])
            self.assertEqual(manifest["name"], "@moq/moq-ffi")
            self.assertEqual(manifest["description"], "Media over QUIC, bindings")
            self.assertEqual(manifest["repository"], {"type": "git", "url": "git+https://github.com/moq-dev/moq"})
            self.assertEqual(manifest["exports"]["."]["types"], "./index.d.ts")
            self.assertIn("LICENSE-MIT", manifest["files"])
            config = package.json.loads((jsr / "jsr.json").read_text())
            self.assertEqual(config["name"], "@moq/moq-ffi")
            self.assertEqual(config["exports"], {".": "./index.ts", "./Session": "./Session.ts", "./glue": "./glue.ts", "./rt": "./rt.ts"})

    def test_names_a_lone_license_file_for_jsr(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "terms.txt").write_text("Terms\n")
            web = web_export(root / "export")
            unlicensed = info(root, license=None, license_file="terms.txt")
            npm, jsr = package.web_package(unlicensed, web, root / "export/moq.wasm", "moq", root / "out")
            self.assertTrue((npm / "terms.txt").is_file())
            self.assertEqual((jsr / "LICENSE").read_text(), "Terms\n")
            self.assertFalse((jsr / "terms.txt").exists())
            (root / "terms.txt").unlink()
            with self.assertRaises(SystemExit):
                package.web_package(info(root, license=None), web, root / "export/moq.wasm", "moq", root / "out")

    def test_size_unless_the_crate_chooses(self):
        with tempfile.TemporaryDirectory() as tmp:
            manifest = Path(tmp) / "Cargo.toml"
            manifest.write_text('[profile.release]\nlto = "fat"\n')
            self.assertEqual(package.web_profile(manifest), {"CARGO_PROFILE_RELEASE_OPT_LEVEL": "s"})
            manifest.write_text('[profile.release]\nopt-level = 3\n')
            self.assertEqual(package.web_profile(manifest), {})

    def test_drops_an_earlier_tarball(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            web = web_export(root / "export")
            (root / "out").mkdir()
            (root / "out/moq-moq-ffi-0.4.11.tgz").write_bytes(b"old")
            package.web_package(info(root), web, root / "export/moq.wasm", "moq", root / "out")
            self.assertEqual(list((root / "out").glob("*.tgz")), [])

    def test_refuses_another_version(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            web = web_export(root / "export", version="0.0.1")
            with self.assertRaises(SystemExit):
                package.web_package(info(root), web, root / "export/moq.wasm", "moq", root / "out")

    def test_names_jsr_can_spell(self):
        self.assertEqual(package.package_name(info("/src"), "moq"), "@moq/moq-ffi")
        for scope in ("Moq", "-moq", "moq_dev", "m"):
            with self.assertRaises(SystemExit):
                package.package_name(info("/src"), scope)
        with self.assertRaises(SystemExit):
            package.package_name(info("/src", name="moq_ffi"), "moq")

    def test_check_imports_the_package(self):
        with tempfile.TemporaryDirectory() as tmp:
            check = Path(tmp) / "check.ts"
            check.write_text('import * as api from "./web/index.ts";\nimport type * as rt from "./web/rt.ts";\n'
                             "import * as Error_ from './web/Error_.ts';\n")
            self.assertEqual(package.check_program(check, "@moq/moq-ffi"),
                             'import * as api from "@moq/moq-ffi";\nimport type * as rt from "@moq/moq-ffi/rt";\n'
                             "import * as Error_ from '@moq/moq-ffi/Error_';\n")
            check.write_text('import * as api from "./web/index.ts";\nconsole.log("./web/index.ts", await import("./web/rt.ts"));\n')
            self.assertEqual(package.check_program(check, "@moq/moq-ffi"),
                             'import * as api from "@moq/moq-ffi";\nconsole.log("./web/index.ts", await import("@moq/moq-ffi/rt"));\n')
            check.write_text("import * as api from /* binding */ './web/index.ts';\nimport(\n  // rt\n  './web/rt.ts');\n")
            self.assertEqual(package.check_program(check, "@moq/moq-ffi"),
                             "import * as api from /* binding */ '@moq/moq-ffi';\nimport(\n  // rt\n  '@moq/moq-ffi/rt');\n")
            check.write_text('import * as api from "./web/nested/index.ts";\n')
            with self.assertRaises(SystemExit):
                package.check_program(check, "@moq/moq-ffi")

    def test_check_directory_names_its_program(self):
        with tempfile.TemporaryDirectory() as tmp:
            (Path(tmp) / "transcript.ts").write_text("")
            with self.assertRaises(SystemExit):
                package.check_program_in(tmp, ("check.ts",))
            (Path(tmp) / "check.ts").write_text("")
            self.assertEqual(package.check_program_in(tmp, ("check.ts",)), Path(tmp) / "check.ts")
            (Path(tmp) / "check.c").write_text("")
            (Path(tmp) / "check.cpp").write_text("")
            with self.assertRaises(SystemExit):
                package.check_program_in(tmp, ("check.c", "check.cpp"))

    def test_dart_check_lands_whole_beside_its_entry(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            check, bin_ = tmp / "check", tmp / "bin"
            check.mkdir()
            bin_.mkdir()
            (check / "transcript.dart").write_text("")
            with self.assertRaises(SystemExit):
                package.dart_check("moq", check, bin_)
            (check / "check.dart").write_text("void main() {}\n")
            package.dart_check("moq", check, bin_)
            self.assertEqual(sorted(p.name for p in bin_.iterdir()), ["check.dart", "transcript.dart"])
            # The entry is the consumer's own.
            (check / "main.dart").write_text("")
            with self.assertRaises(SystemExit):
                package.dart_check("moq", check, tmp / "other")

    def test_dart_without_a_check_calls_the_contract(self):
        with tempfile.TemporaryDirectory() as tmp:
            package.dart_check("moq", None, Path(tmp))
            program = (Path(tmp) / "check.dart").read_text()
            self.assertIn("symbol: 'moq_contract'", program)
            self.assertIn("package:moq/moq.dart", program)

    def test_python_check_imports_its_modules_alone(self):
        # The check's directory is on the path, and only it: not the working directory, and
        # not PYTHONPATH. The generated script reaches the check's functions as `__main__`.
        with tempfile.TemporaryDirectory() as tmp:
            check, elsewhere = Path(tmp) / "check", Path(tmp) / "elsewhere"
            check.mkdir()
            elsewhere.mkdir()
            (elsewhere / "stray.py").write_text("")
            (check / "check.py").write_text(
                "import importlib.util\n"
                "def hook():\n    return 'hooked'\n"
                "assert importlib.util.find_spec('stray') is None\n"
                "import transcript\ntranscript.script()\n")
            (check / "transcript.py").write_text("import __main__ as local\ndef script():\n    print(local.hook())\n")
            (Path(tmp) / "native.txt").write_bytes(b"hooked\n")
            with mock.patch.dict(os.environ, {"PYTHONPATH": str(elsewhere)}):
                package.consume_python(check, Path(tmp) / "native.txt", None)
            (Path(tmp) / "native.txt").write_bytes(b"other\n")
            with self.assertRaises(SystemExit):
                package.consume_python(check, Path(tmp) / "native.txt", None)



class ReactNative(unittest.TestCase):
    CMAKE = ("target_link_libraries(rust2_moq jsi reactnative react_codegen_Rust2MoqSpec\n"
             "\t${CMAKE_CURRENT_SOURCE_DIR}/../libs/android/${ANDROID_ABI}/libmoq.a log dl m)\n")
    PODSPEC = ('  s.source_files = "cpp/*.{h,cpp}", "ios/*.mm", "include/*.h"\n'
               '  s.vendored_frameworks = "libs/ios/moq.xcframework"\n'
               "  install_modules_dependencies(s)\nend\n")

    def test_names_beside_the_web_package(self):
        self.assertEqual(package.rn_package_name(info("/src"), "moq"), "@moq/moq-ffi-rn")
        for scope in ("Moq", "-moq", ""):
            with self.assertRaises(SystemExit):
                package.rn_package_name(info("/src"), scope)
        with self.assertRaises(SystemExit):
            package.rn_package_name(info("/src", name="Moq"), "moq")

    def test_cmake_links_what_rustc_names_beside_its_own(self):
        linked = package.rn_cmake(self.CMAKE, "moq", "-lunwind -ldl -llog -lunwind -ldl -lm -lc")
        self.assertIn("/libmoq.a log dl m unwind c)\n", linked)
        self.assertEqual(package.rn_cmake(self.CMAKE, "moq", "-ldl -llog"), self.CMAKE)
        with self.assertRaisesRegex(SystemExit, "update the packager"):
            package.rn_cmake(self.CMAKE.replace(" log dl m", " log"), "moq", "-ldl")
        with self.assertRaisesRegex(SystemExit, "unsupported"):
            package.rn_cmake(self.CMAKE, "moq", "-ldl -Wl,--as-needed")

    def test_podspec_links_frameworks_and_bundles_the_manifest(self):
        spec = package.rn_podspec(self.PODSPEC, "moq", "-framework Security -framework CoreFoundation -liconv -lSystem -lc -lm "
                                  "-framework Security", "Rust2MoqPrivacy")
        self.assertIn('  s.frameworks = ["Security", "CoreFoundation"]\n  s.libraries = ["iconv", "System", "c", "m"]\n'
                      '  s.resource_bundles = { "Rust2MoqPrivacy" => ["ios/PrivacyInfo.xcprivacy"] }\n'
                      "  install_modules_dependencies(s)\nend\n", spec)
        with self.assertRaisesRegex(SystemExit, "update the packager"):
            package.rn_podspec(self.PODSPEC.replace("moq.xcframework", "other.xcframework"), "moq", "-lc", "Rust2MoqPrivacy")
        with self.assertRaisesRegex(SystemExit, "unsupported"):
            package.rn_podspec(self.PODSPEC, "moq", "-lc -weak_framework Foo", "Rust2MoqPrivacy")

    def test_slices_agree_on_their_libraries(self):
        with tempfile.TemporaryDirectory() as tmp:
            one, two = Path(tmp) / "one.txt", Path(tmp) / "two.txt"
            one.write_text("-ldl -llog\n")
            two.write_text("-ldl -llog\n")
            self.assertEqual(package.agreed([one, two], "Android"), "-ldl -llog")
            two.write_text("-ldl -llog -lz\n")
            with self.assertRaisesRegex(SystemExit, "different native libraries"):
                package.agreed([one, two], "Android")

    def test_check_imports_the_package(self):
        with tempfile.TemporaryDirectory() as tmp:
            check = Path(tmp) / "check.ts"
            check.write_text('import * as moq from "./react-native/index.ts";\nimport * as Client from "./react-native/Client.ts";\n')
            self.assertEqual(package.check_program(check, "@moq/moq-ffi-rn", "react-native"),
                             'import * as moq from "@moq/moq-ffi-rn";\nimport * as Client from "@moq/moq-ffi-rn/Client";\n')
            check.write_text('import * as moq from "./react-native/nested/index.ts";\n')
            with self.assertRaises(SystemExit):
                package.check_program(check, "@moq/moq-ffi-rn", "react-native")

    def test_reads_the_app_log(self):
        log = object.__new__(package.Logcat)
        log.tag, log.what = "ReactNativeJS", "React Native app"
        log.lines, log.pid, log.vm_service = queue.Queue(), None, None
        for line in ("I/ReactNativeJS( 4321): Running \"Consumer\"", "I/ReactNativeJS( 4321): rust2: start",
                     "I/ReactNativeJS( 4321): rust2: aGk=", "I/flutter ( 4321): rust2: Ym9vbQ==",
                     "I/ReactNativeJS( 4321): rust2: exit 0", None):
            log.lines.put(line)
        with mock.patch("sys.stderr"):
            self.assertEqual(log.output(), b"hi")


if __name__ == "__main__":
    unittest.main()
