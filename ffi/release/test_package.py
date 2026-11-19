"""The builder's decisions that need no toolchain: names, versions, metadata, and manifests."""

import importlib.util
import json
import os
from pathlib import Path
import shutil
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
    (root / "go/rt").mkdir(parents=True)
    (root / "go/go.mod").write_text("module moq.dev/moq-ffi\n\ngo 1.24\n")
    lines = "".join(f"#cgo {p.replace('_', ',')} LDFLAGS: ${{SRCDIR}}/libmoq_{p}.a -lgcc_s -lc\n"
                    for p in package.GO_TARGETS.values())
    (root / "go/rt/rt.go").write_text(f"package rt\n\n/*\n{lines}*/\nimport \"C\"\n")
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

    def test_an_empty_encoded_rustflags_is_no_flags(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.dict(os.environ, {"CARGO_ENCODED_RUSTFLAGS": "", "RUSTFLAGS": "-C x", "CARGO_HOME": tmp + "/home"}):
            env = package.reproducible(info(tmp), Path(tmp) / "export", "x86_64-unknown-linux-gnu")
            flags = env["CARGO_ENCODED_RUSTFLAGS"].split("\x1f")
            self.assertTrue(flags[0].startswith("--remap-path-prefix="), flags)
            self.assertNotIn("-C", flags)

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
    def test_the_random_temporary_directory_is_fixed(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "lib.a"
            path.write_bytes(b"_head_D__a_x_out_rustc4TX99l_kernel32_dll_imports_lib\x00_rustcAbCdEf_other")
            package.fix_import_names(path)
            self.assertEqual(path.read_bytes(), b"_head_D__a_x_out_rustc000000_kernel32_dll_imports_lib\x00_rustcAbCdEf_other")


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
                    "LICENSE-MIT", "go.mod", "rt/libmoq_darwin_arm64.a", "rt/libmoq_ios_arm64.a",
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
