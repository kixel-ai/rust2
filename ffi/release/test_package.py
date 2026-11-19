"""The builder's decisions that need no toolchain: names, versions, metadata, and manifests."""

import importlib.util
from pathlib import Path
import json
import tarfile
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
        self.assertIn('IMPORTED_LOCATION "${_moq_prefix}/lib/libmoq.0.4.12.dylib"', config)
        self.assertIn("INTERFACE_LINK_LIBRARIES [==[-framework Security;iconv]==]", config)
        self.assertEqual(config.count('INTERFACE_COMPILE_FEATURES "c_std_11;cxx_std_17"'), 2)
        self.assertIn('VERSION_LESS "0.4.0"', package.cmake_version(info("/src")))
        cps = json.loads(package.cps(info("/src"), files, natives))
        self.assertEqual((cps["name"], cps["version"], cps["compat_version"]), ("moq", "0.4.12", "0.4.0"))
        self.assertEqual(cps["default_components"], ["moq"])
        self.assertEqual(cps["components"]["moq"]["link_libraries"], ["-framework Security", "iconv"])
        self.assertEqual(cps["components"]["shared"]["location"], "@prefix@/lib/libmoq.0.4.12.dylib")
        self.assertEqual(cps["website"], "https://github.com/moq-dev/moq")
        self.assertEqual(cps["version_schema"], "simple")
        prerelease = info("/src", version="2.0.0-rc.1")
        self.assertEqual(json.loads(package.cps(prerelease, files, natives))["version_schema"], "custom")
        self.assertIn("set(PACKAGE_VERSION_COMPATIBLE FALSE)\n", package.cmake_version(prerelease))
        self.assertNotIn("VERSION_LESS", package.cmake_version(prerelease))
        windows = package.cps(info("/src"), package.c_files("moq", "x86_64-pc-windows-msvc", "0.4.12"),
                              [("lib", "ntdll"), ("flag", "/defaultlib:msvcrt")])
        windows = json.loads(windows)["components"]
        self.assertEqual(windows["shared"]["link_location"], "@prefix@/lib/moq.dll.lib")
        self.assertEqual(windows["moq"]["link_flags"], ["/defaultlib:msvcrt"])

    def test_commons_are_defined_unless_a_member_defines_them(self):
        nm = """
libmoq.a(ui.o):
0000000000000008 (common) (alignment 2^3) external _tty_in
0000000000000004 (common) (alignment 2^2) external _is_a_tty
0000000000000010 (common) (alignment 2^4) external _shared
                 (undefined) external _tty_out

libmoq.a(other.o):
0000000000000008 (common) (alignment 2^3) external _tty_out
0000000000000020 (common) external _shared
0000000000000000 (__DATA,__bss) external _is_a_tty
0000000000000008 (common) (alignment 2^3) external _state
0000000000000010 (__DATA,__data) private external _state
"""
        self.assertEqual(package.commons_source(nm),
                         '__attribute__((aligned(16))) char common0[32] __asm__("_shared");\n'
                         '__attribute__((aligned(8))) char common1[8] __asm__("_tty_in");\n'
                         '__attribute__((aligned(8))) char common2[8] __asm__("_tty_out");\n')
        self.assertEqual(package.commons_source("0000 (__TEXT,__text) external _moq_x\n"), "typedef int empty;\n")

    def test_archive_is_owned_by_no_one(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp) / "moq-ffi-0.4.12-x86_64-unknown-linux-gnu"
            (root / "lib").mkdir(parents=True)
            (root / "lib/libmoq.so.0.4.12").write_bytes(b"\x7fELF")
            (root / "lib/libmoq.so.0.4.12").chmod(0o700)
            (root / "lib/libmoq.so").symlink_to("libmoq.so.0.4.12")
            (root / "lib/libmoq.a").write_bytes(b"!<arch>\n")
            (root / "lib/libmoq.a").chmod(0o600)
            path = package.pack(root, Path(tmp) / "out.tar.gz")
            with tarfile.open(path) as archive:
                members = {m.name.split("/", 1)[-1]: m for m in archive.getmembers()}
            self.assertEqual(members["lib/libmoq.so.0.4.12"].mode, 0o755)
            self.assertEqual(members["lib/libmoq.a"].mode, 0o644)
            self.assertEqual(members["lib/libmoq.so"].linkname, "libmoq.so.0.4.12")
            self.assertEqual({(m.uid, m.uname) for m in members.values()}, {(0, "")})


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
