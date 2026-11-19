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

    def test_web_lines_follow_the_scope(self):
        shipped = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi", "moq")
        self.assertIn("\n      web-manifest-path: rs/moq-ffi/Cargo.toml\n      scope: moq\n", shipped)
        self.assertIn("@moq/moq-ffi on npm and JSR", shipped)
        none = package.template(info("/src"), "rs/moq-ffi/Cargo.toml", "a" * 40, "moq-dev/moq-swift-ffi", "moq.dev/moq-ffi")
        self.assertIn("      # web-manifest-path: rs/moq-ffi/Cargo.toml\n", none)
        self.assertNotIn("\n      web-manifest-path:", none)
        self.assertNotRegex(none, r"@[A-Z_]+@")


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
            check.write_text('import * as api from "./web/nested/index.ts";\n')
            with self.assertRaises(SystemExit):
                package.check_program(check, "@moq/moq-ffi")


if __name__ == "__main__":
    unittest.main()
