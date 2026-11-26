#!/usr/bin/env python3
"""Check sh/coverage.py's reading of strace's output and its holes against a stand-in map."""

import importlib.util
import subprocess
import tempfile
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location("coverage", Path(__file__).with_name("coverage.py"))
coverage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(coverage)


def traced(text):
    return coverage.events(text.strip().splitlines())


class ReadTests(unittest.TestCase):
    def test_paths_resolve_against_each_process_working_directory(self):
        traces = {
            # The first process names a path relative to the directory strace decoded, then
            # makes a child before and after changing to `ffi`.
            10: traced(r'''
openat(AT_FDCWD</repo>, "flake.nix", O_RDONLY) = 3</repo/flake.nix>
clone(child_stack=NULL, flags=CLONE_CHILD_CLEARTID|SIGCHLD) = 11
chdir("ffi") = 0
vfork() = 12
access("missing", F_OK) = -1 ENOENT (No such file or directory)
'''),
            # A child names a relative path with no directory decoded: its parent's at its making.
            11: traced(r'''
stat("sh/dispatch.sh", {st_mode=S_IFREG|0755, ...}) = 0
'''),
            12: traced(r'''
execve("release/build.sh", ["release/build.sh"], 0x7ffd /* 3 vars */) = 0
newfstatat(3</repo/ffi/src>, "lib.rs", {st_mode=S_IFREG|0644, ...}, 0) = 0
'''),
        }
        self.assertEqual(coverage.resolve(traces, Path("/repo")), {
            "/repo/flake.nix": True,
            "/repo/sh/dispatch.sh": False,
            "/repo/ffi/missing": False,
            "/repo/ffi/release/build.sh": False,
            "/repo/ffi/src/lib.rs": False,
        })

    def test_a_returned_descriptor_names_what_a_link_opened(self):
        evs = traced(r'''
openat(AT_FDCWD</repo/ffi>, "rust-toolchain.toml", O_RDONLY) = 3</repo/ts/rust-toolchain.toml>
''')
        self.assertEqual(coverage.resolve({1: evs}, Path("/repo")),
                         {"/repo/ffi/rust-toolchain.toml": True, "/repo/ts/rust-toolchain.toml": True})

    def test_escaped_paths(self):
        self.assertEqual(coverage.unescape(r'a\"b\\c\x41\101\n'), 'a"b\\cAA\n')

    def test_a_process_no_trace_made_fails(self):
        with self.assertRaises(SystemExit):
            coverage.resolve({1: traced('stat("a", 0) = 0'), 2: traced('stat("b", 0) = 0')}, Path("/repo"))

    def test_a_process_that_named_nothing_is_skipped(self):
        evs = traced('stat("a", 0) = 0')
        self.assertEqual(coverage.resolve({1: evs, 2: traced("???()  = ?"), 3: []}, Path("/repo")),
                         {"/repo/a": False})


class SourceTests(unittest.TestCase):
    def test_a_directory_counts_only_when_listed(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            subprocess.run(["git", "init", "-q"], cwd=root, check=True)
            (root / "ffi" / "release").mkdir(parents=True)
            (root / "ffi" / "target").mkdir()
            (root / ".gitignore").write_text("target\n")
            paths = {
                f"{root}/ffi": False,
                f"{root}/ffi/release": True,
                f"{root}/ffi/target/x": True,
                f"{root}/ffi/missing": False,
                f"{root}/.git/HEAD": True,
                "/elsewhere": True,
            }
            self.assertEqual(coverage.sources(paths, root), {"ffi/release", "ffi/missing"})


class HoleTests(unittest.TestCase):
    def test_a_path_whose_diff_skips_the_module_is_a_hole(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "ffi" / "release").mkdir(parents=True)
            (root / "ffi" / "src.rs").write_text("")
            # A stand-in map: ffi/release/ selects only `release`, the rest of ffi/ `ffi`.
            def select(change, _root):
                return {"release"} if change.startswith("ffi/release/") else {"ffi"}
            found = coverage.holes("ffi", {"ffi/src.rs", "ffi/release", "ffi/release/package.py"}, root, select)
            self.assertEqual(found, [
                ("ffi/release", "ffi/release/coverage-probe"),
                ("ffi/release/package.py", "ffi/release/package.py"),
            ])


if __name__ == "__main__":
    unittest.main()
