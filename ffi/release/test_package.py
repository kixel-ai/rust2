"""The builder's decisions that need no toolchain: names, versions, metadata, and manifests."""

import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock
import zipfile

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("package", HERE / "package.py")
package = importlib.util.module_from_spec(spec)
spec.loader.exec_module(package)


def info(root, **overrides):
    """Facts as `facts` reads them, for a package in `root`."""
    base = {
        "name": "moq-ffi", "version": "0.4.12", "description": "Media over QUIC,\n bindings",
        "license": "MIT OR Apache-2.0", "license_file": None, "repository": "https://github.com/moq-dev/moq",
        "homepage": None, "lib": "moq", "dir": str(root), "workspace": str(root), "examples": [],
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
    (root / "Cargo.toml").write_text(f'pyo3-ffi = {{ features = ["abi3-py{floor}", "extension-module"] }}\n')
    return root


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
            wheel = package.write_wheel(info(root), built, library, "x86_64-unknown-linux-gnu", out)
            self.assertEqual(wheel.name, "moq_ffi-0.4.12-cp311-abi3-linux_x86_64.whl")
            package.check_wheel(wheel)
            with zipfile.ZipFile(wheel) as archive:
                names = set(archive.namelist())
                meta = archive.read("moq_ffi-0.4.12.dist-info/METADATA").decode()
            self.assertIn("moq/__init__.py", names)
            self.assertIn("moq/libmoq.so", names)
            self.assertIn("moq_ffi-0.4.12.dist-info/licenses/LICENSE-MIT", names)
            self.assertIn("Requires-Python: >=3.11\n", meta)
            self.assertIn("Summary: Media over QUIC, bindings\n", meta)
            self.assertIn("License-Expression: MIT OR Apache-2.0\n", meta)
            self.assertIn("Project-URL: Repository, https://github.com/moq-dev/moq\n", meta)

    def test_refuses_a_legacy_license(self):
        with self.assertRaises(SystemExit):
            package.metadata(info("/src", license="MIT/Apache-2.0"), 11)

    def test_directory_entries_are_not_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            library = root / "libmoq.dylib"
            library.write_bytes(b"\xcf\xfa\xed\xfe")
            wheel = package.write_wheel(info(root), export(root / "export"), library, "aarch64-apple-darwin", root)
            repaired = root / "repaired.whl"
            # As delocate and auditwheel rewrite a wheel: each directory listed before its files.
            with zipfile.ZipFile(wheel) as src, zipfile.ZipFile(repaired, "w") as dst:
                dst.writestr("moq/", b"")
                dst.writestr("moq_ffi-0.4.12.dist-info/", b"")
                for name in src.namelist():
                    dst.writestr(name, src.read(name))
            package.check_wheel(repaired)

    def test_a_tampered_record_fails(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            library = root / "moq.dll"
            library.write_bytes(b"MZ")
            wheel = package.write_wheel(info(root), export(root / "export"), library, "x86_64-pc-windows-msvc", root)
            tampered = root / "tampered.whl"
            with zipfile.ZipFile(wheel) as src, zipfile.ZipFile(tampered, "w") as dst:
                for name in src.namelist():
                    data = src.read(name)
                    dst.writestr(name, b"MZ!" if name.endswith("moq.dll") else data)
            with self.assertRaises(SystemExit):
                package.check_wheel(tampered)

    def test_floor_comes_from_the_glue(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.assertEqual(package.python_floor(export(Path(tmp), "312")), 12)


class Reproducible(unittest.TestCase):
    def wheel(self, root, **touch):
        """A wheel built from inputs of the given mtimes, whose repair tool put its files in any order and time."""
        built = export(root / "export")
        library = root / "libmoq.so"
        library.write_bytes(b"\x7fELF")
        for path in (library, built / "python/moq.py"):
            os.utime(path, (touch.get("time", 0), touch.get("time", 0)))
        return package.write_wheel(info(root), built, library, "x86_64-unknown-linux-gnu", root)

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
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"RUSTFLAGS": "-C target-cpu=native", "CARGO_HOME": "/home/u/.cargo"}):
            os.environ.pop("CARGO_ENCODED_RUSTFLAGS", None)
            env = package.reproducible(info(tmp), Path(tmp) / "export", "x86_64-unknown-linux-gnu")
            flags = env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
            self.assertEqual(flags[:2], ["-C", "target-cpu=native"])
            self.assertIn(f"--remap-path-prefix={tmp}/export=/export", flags)
            # The deeper directory comes last, as rustc prefers the last match.
            self.assertLess(flags.index(f"--remap-path-prefix={tmp}=/source"), flags.index(f"--remap-path-prefix={tmp}/export=/export"))
            self.assertEqual(env["SOURCE_DATE_EPOCH"], "1700000000")
            self.assertIn(f"-ffile-prefix-map={tmp}=/source", env["CFLAGS"])
            windows = package.reproducible(info(tmp), Path(tmp) / "export", "x86_64-pc-windows-msvc")
            self.assertIn("link-arg=/Brepro", windows["CARGO_ENCODED_RUSTFLAGS"])
            self.assertNotIn("CFLAGS", windows)
            apple = package.reproducible(info(tmp), Path(tmp), "aarch64-apple-darwin")
            self.assertEqual(apple["ZERO_AR_DATE"], "1")
            self.assertIn("link-arg=-Wl,-install_name,@rpath/libmoq.dylib", apple["CARGO_ENCODED_RUSTFLAGS"])

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
            self.assertEqual(document["metadata"]["timestamp"], "2023-11-14T22:13:20Z")
            self.assertEqual([c["name"] for c in document["components"]], ["dep"])
            self.assertEqual(document["components"][0]["licenses"], [{"license": {"name": "MIT/Apache-2.0"}}])
            self.assertEqual(document["dependencies"], [
                {"ref": "pkg:cargo/dep@1.2.3", "dependsOn": []},
                {"ref": "pkg:cargo/moq-ffi@0.4.12", "dependsOn": ["pkg:cargo/dep@1.2.3"]},
            ])


class Digest(unittest.TestCase):
    def test_any_byte_splits_the_hosts(self):
        with tempfile.TemporaryDirectory() as a, tempfile.TemporaryDirectory() as b:
            first, second = export(Path(a)), export(Path(b))
            self.assertEqual(package.digest(first, "moq"), package.digest(second, "moq"))
            # A Windows checkout once turned the embedded runtime CRLF.
            (second / "python/moq.py").write_bytes(b"PENDING = 0\r\n")
            self.assertNotEqual(package.digest(first, "moq"), package.digest(second, "moq"))


class Swift(unittest.TestCase):
    def test_linker_settings_by_platform(self):
        settings = package.linker_settings({
            "macos": "-framework Security -framework CoreFoundation -liconv -lSystem",
            "ios": "-framework Security -liconv -lSystem",
        })
        self.assertIn('.linkedFramework("Security")', settings)
        self.assertIn('.linkedFramework("CoreFoundation", .when(platforms: [.macOS]))', settings)
        self.assertIn('.linkedLibrary("iconv")', settings)

    def test_unknown_linker_flags_fail(self):
        with self.assertRaises(SystemExit):
            package.linker_settings({"macos": "-Wl,-dead_strip"})

    def test_manifest(self):
        text = package.package_swift("Moq", "1.7.5", ['.linkedLibrary("iconv")'],
                                     '.binaryTarget(name: "MoqFFI", url: "https://x/MoqFFI.xcframework.zip", checksum: "00")')
        self.assertIn('platforms: [.macOS("13.0"), .iOS("16.0")]', text)
        self.assertIn('exact: "1.7.5"', text)
        self.assertIn('dependencies: ["MoqFFI", .product(name: "BasicContainers", package: "swift-collections")]', text)


class Template(unittest.TestCase):
    def test_fills_every_identity(self):
        text = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi")
        self.assertIn("uses: kixel-ai/rust2/.github/workflows/bindings.yml@" + "a" * 40, text)
        self.assertIn('- "moq-ffi-v*"', text)
        self.assertIn("url: https://pypi.org/p/moq-ffi", text)
        self.assertIn("owner: moq-dev\n", text)
        self.assertIn("repositories: moq-swift-ffi\n", text)
        self.assertNotRegex(text, r"@[A-Z_]+@")

    def test_refuses_a_movable_pin_or_bad_mirror(self):
        with self.assertRaises(SystemExit):
            package.template(info("/src"), "Cargo.toml", "main", "moq-dev/moq-swift-ffi")
        with self.assertRaises(SystemExit):
            package.template(info("/src"), "Cargo.toml", "a" * 40, "moq-swift-ffi")


if __name__ == "__main__":
    unittest.main()
