"""The builder's decisions that need no toolchain: names, versions, metadata, and manifests."""

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
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
    (root / "go/rt").mkdir(parents=True)
    (root / "go/go.mod").write_text("module moq.dev/moq-ffi\n\ngo 1.24\n")
    (root / "go/rt/rt.go").write_text(
        "package rt\n\n/*\n#cgo linux,!android,amd64 LDFLAGS: ${SRCDIR}/libmoq_linux_amd64.a -lgcc_s -lc\n*/\nimport \"C\"\n")
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

    def test_escapes_capitals(self):
        self.assertEqual(package.escape("github.com/Azure/x"), "github.com/!azure/x")

    def test_links_from_the_cgo_line(self):
        with tempfile.TemporaryDirectory() as root:
            root = export(Path(root))
            self.assertEqual(package.go_links(root, "moq", "x86_64-unknown-linux-gnu"), "-lgcc_s -lc")
            with self.assertRaises(SystemExit):
                package.go_links(root, "moq", "aarch64-unknown-linux-gnu")

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
            with zipfile.ZipFile(versions / "v0.4.12.zip") as z:
                self.assertEqual(sorted(z.namelist()), [f"moq.dev/moq-ffi@v0.4.12/{n}" for n in [
                    "LICENSE-MIT", "go.mod", "rt/libmoq_darwin_arm64.a", "rt/libmoq_linux_amd64.a",
                    "rt/libmoq_linux_arm64.a", "rt/libmoq_windows_amd64.a", "rt/rt.go"]])
            # The module ships every Go target's library, and nothing else.
            (root / "libs/libmoq_linux_arm64.a").unlink()
            with self.assertRaises(SystemExit):
                package.go_module(info(root), built, root / "libs", root / "other")

    def test_refuses_names_a_case_folds_together(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            built = export(root / "export")
            (built / "go/RT.go").write_text("package rt\n")
            (built / "go/rt.go").write_text("package rt\n")
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
        text = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        self.assertIn("uses: kixel-ai/rust2/.github/workflows/bindings.yml@" + "a" * 40, text)
        self.assertIn('- "moq-ffi-v*"', text)
        self.assertIn("url: https://pypi.org/p/moq-ffi", text)
        self.assertIn("owner: moq-dev\n", text)
        self.assertIn("repositories: moq-swift-ffi\n", text)
        self.assertIn("go-module: moq.dev/moq-ffi\n", text)
        self.assertNotRegex(text, r"@[A-Z_]+@")

    def test_refuses_a_movable_pin_or_bad_mirror(self):
        with self.assertRaises(SystemExit):
            package.template(info("/src"), "Cargo.toml", "main", "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        with self.assertRaises(SystemExit):
            package.template(info("/src"), "Cargo.toml", "a" * 40, "moq-swift-ffi", "moq.dev/moq-ffi")
        for module in ["moq", "moq.dev/moq-ffi/", "moq.dev/moq-ffi/v2"]:
            with self.assertRaises(SystemExit):
                package.template(info("/src"), "Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", module)


if __name__ == "__main__":
    unittest.main()
