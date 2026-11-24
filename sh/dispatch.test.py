#!/usr/bin/env python3
"""Check sh/dispatch.sh's selection against real diffs in a scratch repository."""

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

DISPATCH = Path(__file__).with_name("dispatch.sh").resolve()
EVERYTHING = ["lint", "quest", "sh", "release", "driver", "ffi", "go", "ts"]
# A shared input runs everything but the parked go, which only `just gate` and
# its own diff select.
SHARED = [m for m in EVERYTHING if m != "go"]


class SelectTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.repo = Path(self.temp.name)
        (self.repo / "sh").mkdir()
        shutil.copy(DISPATCH, self.repo / "sh" / "dispatch.sh")
        self.env = {
            "PATH": os.environ["PATH"],
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_CONFIG_GLOBAL": os.devnull,
        }
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.name", "test")
        self.git("config", "user.email", "test@example.com")
        self.git("add", ".")
        self.git("commit", "-qm", "base")
        self.git("update-ref", "refs/remotes/origin/main", "HEAD")

    def git(self, *args):
        return subprocess.run(
            ["git", *args], cwd=self.repo, env=self.env, text=True, capture_output=True, check=True
        ).stdout.strip()

    def change(self, *paths):
        for path in paths:
            file = self.repo / path
            file.parent.mkdir(parents=True, exist_ok=True)
            file.write_text(file.read_text() + "\n" if file.exists() else "changed\n")

    def commit(self, *paths):
        self.change(*paths)
        self.git("add", ".")
        self.git("commit", "-qm", "change")
        return self.git("rev-parse", "HEAD")

    def query(self, *base):
        return subprocess.run(
            ["bash", "sh/dispatch.sh", "ci", *base, "--query"],
            cwd=self.repo,
            env=self.env,
            text=True,
            capture_output=True,
        )

    def select(self, *base):
        result = self.query(*base)
        self.assertEqual(result.returncode, 0, result.stderr)
        return result.stdout.split()

    def test_projects_select_themselves_and_the_lints(self):
        for paths, expected in (
            (["ts/src/lib.rs"], ["lint", "quest", "ts"]),
            (["ffi/src/lib.rs"], ["lint", "quest", "ffi"]),
            (["go/src/lib.rs"], ["lint", "quest", "go"]),
            (["ts/fixtures/codec/src/lib.rs"], ["lint", "quest", "ts"]),
            (["quest/m0/example.md"], ["lint", "quest"]),
            (["sh/dispatch.test.py"], ["lint", "quest", "sh"]),
            (["sh/coverage.test.py"], ["lint", "quest", "sh"]),
            # The release builder's scripts are their own module, apart from the exporter's.
            (["ffi/release/package.py"], ["lint", "quest", "release"]),
            (["ffi/release/package.py", "ffi/src/lib.rs"], ["lint", "quest", "release", "ffi"]),
            (["ts/src/lib.rs", "ffi/src/lib.rs"], ["lint", "quest", "ffi", "ts"]),
        ):
            with self.subTest(paths=paths):
                self.change(*paths)
                self.assertEqual(self.select(), expected)
                self.git("clean", "-fdq")

    def test_the_driver_selects_its_dependents(self):
        self.change("driver/src/lib.rs")
        self.assertEqual(self.select(), ["lint", "quest", "driver", "ffi", "ts"])

    def test_the_shared_nightly_selects_each_project_on_it(self):
        self.change("ts/rust-toolchain.toml")
        self.assertEqual(self.select(), ["lint", "quest", "driver", "ffi", "ts"])

    def test_a_go_diff_with_a_shared_input_still_selects_go(self):
        self.change("go/src/lib.rs", "flake.nix")
        self.assertEqual(self.select(), EVERYTHING)

    def test_lint_only_paths_select_only_the_lints(self):
        for path in (
            "bench/ffi/run.sh",
            "README.md",
            ".claude/settings.json",
            # A project's prose, which no build or test reads.
            "ffi/FACADE.md",
            "ffi/release/README.md",
        ):
            with self.subTest(path=path):
                self.change(path)
                self.assertEqual(self.select(), ["lint", "quest"])
                self.git("clean", "-fdq")

    def test_other_prose_in_a_project_still_selects_it(self):
        for path, expected in (
            # A fixture's expected output is Markdown its test compares with.
            ("ts/fixtures/census/census.md", ["lint", "quest", "ts"]),
            # Cargo looks for a crate's README.
            ("ffi/README.md", ["lint", "quest", "ffi"]),
            ("driver/pub/README.md", ["lint", "quest", "driver", "ffi", "ts"]),
        ):
            with self.subTest(path=path):
                self.change(path)
                self.assertEqual(self.select(), expected)
                self.git("clean", "-fdq")

    def test_what_loading_the_justfiles_reads_selects_each_module_running_just(self):
        for path in ("ffi/justfile", "ts/justfile", "driver/justfile", "ts/fixtures/model/Cargo.toml"):
            with self.subTest(path=path):
                self.change(path)
                self.assertEqual(self.select(), ["lint", "quest", "driver", "ffi", "ts"])
                self.git("clean", "-fdq")
        # The parked go's own still selects it.
        self.change("go/justfile")
        self.assertEqual(self.select(), ["lint", "quest", "driver", "ffi", "go", "ts"])

    def test_paths_on_stdin_select_without_git(self):
        for paths, expected in (
            ("ffi/release/package.py\n", ["lint", "quest", "release"]),
            ("ffi/FACADE.md\n", ["lint", "quest"]),
            ("flake.nix\nffi/FACADE.md\n", SHARED),
            ("", ["lint", "quest"]),
        ):
            with self.subTest(paths=paths):
                result = subprocess.run(
                    ["bash", "sh/dispatch.sh", "ci", "--paths", "--query"],
                    cwd=self.repo, env=self.env, input=paths, text=True, capture_output=True,
                )
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(result.stdout.split(), expected)

    def test_empty_diff_keeps_the_lints(self):
        self.assertEqual(self.select(), ["lint", "quest"])
        self.assertEqual(self.select(""), ["lint", "quest"])

    def test_shared_inputs_select_everything(self):
        for path in (
            "flake.nix",
            "flake.lock",
            "rust-toolchain.toml",
            "justfile",
            ".github/workflows/nightly.yml",
            "sh/dispatch.sh",
            ".gitignore",
            ".gitattributes",
            # An unmapped project runs everything until the map claims it.
            "next/src/lib.rs",
        ):
            with self.subTest(path=path):
                self.change(path)
                self.assertEqual(self.select(), SHARED)
                self.git("checkout", "-q", ".")
                self.git("clean", "-fdq")

    def test_all_selects_everything(self):
        self.assertEqual(self.select("--all"), EVERYTHING)

    def test_only_selects_the_named_modules_in_the_map_order(self):
        # The diff is ignored: a CI job runs exactly its share of a selection.
        self.change("flake.nix")
        self.assertEqual(self.select("--only=ts,lint,ts"), ["lint", "ts"])
        self.assertEqual(self.select("--only=go"), ["go"])

    def test_only_refuses_an_unknown_or_empty_module(self):
        for arg in ("--only=", "--only=lint,", "--only=lint,docs", "--only=lint ts"):
            with self.subTest(arg=arg):
                result = self.query(arg)
                self.assertEqual(result.returncode, 2, result.stderr)

    def test_deletions_count(self):
        self.commit("ts/src/lib.rs")
        self.git("update-ref", "refs/remotes/origin/main", "HEAD")
        (self.repo / "ts/src/lib.rs").unlink()
        self.assertEqual(self.select(), ["lint", "quest", "ts"])

    def test_base_changes_since_the_branch_do_not_count(self):
        branch = self.git("rev-parse", "HEAD")
        self.commit("go/src/lib.rs")
        self.git("update-ref", "refs/remotes/origin/main", "HEAD")
        self.git("checkout", "-q", "-b", "work", branch)
        self.commit("ts/src/lib.rs")
        self.assertEqual(self.select(), ["lint", "quest", "ts"])

    def test_land_gate_base_comes_before_origin_main(self):
        base = self.git("rev-parse", "HEAD")
        self.commit("ts/src/lib.rs")
        # The gate's clone has no remote; origin/main standing at HEAD selects nothing.
        self.git("update-ref", "refs/remotes/origin/main", "HEAD")
        self.assertEqual(self.select(), ["lint", "quest"])
        self.env["QUEST_LAND_BASE"] = base
        self.assertEqual(self.select(), ["lint", "quest", "ts"])
        # An explicit base still wins.
        self.assertEqual(self.select("HEAD"), ["lint", "quest"])

    def test_an_unknown_base_fails(self):
        result = self.query("no-such-ref")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no-such-ref", result.stderr)

    def test_usage_errors(self):
        for args in (["deploy"], ["ci", "origin/main", "--all"], ["ci", "a", "--query", "extra"]):
            with self.subTest(args=args):
                result = subprocess.run(
                    ["bash", "sh/dispatch.sh", *args], cwd=self.repo, env=self.env, text=True, capture_output=True
                )
                self.assertEqual(result.returncode, 2, result.stderr)


if __name__ == "__main__":
    unittest.main()
