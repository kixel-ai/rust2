#!/usr/bin/env python3
"""Check .githooks/pre-push through real pushes from a scratch repository."""

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

HOOK = Path(__file__).resolve().parent.parent / ".githooks" / "pre-push"


class PushTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.repo = root / "repo"
        self.remote = root / "remote.git"
        # A `just` that records its arguments, standing in for the suite.
        bin = root / "bin"
        bin.mkdir()
        self.ran = root / "ran"
        just = bin / "just"
        just.write_text(f'#!/bin/sh\necho "$@${{GIT_DIR:+ GIT_DIR=$GIT_DIR}}" >> {self.ran}\nexit "${{JUST_EXIT:-0}}"\n')
        just.chmod(0o755)
        # Only the tools the hook needs, so the host's own `quest` is not on PATH.
        for tool in ("git", "bash", "env"):
            (bin / tool).symlink_to(shutil.which(tool))
        self.bin = bin
        self.env = {
            "PATH": str(bin),
            "GIT_CONFIG_NOSYSTEM": "1",
            "GIT_CONFIG_GLOBAL": os.devnull,
            # The dev shell, so the hook runs `just` itself instead of entering Nix.
            "RUST2_DEV_SHELL": "1",
        }
        subprocess.run(["git", "init", "-q", "--bare", self.remote], env=self.env, check=True)
        self.repo.mkdir()
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.name", "test")
        self.git("config", "user.email", "test@example.com")
        hooks = self.repo / ".githooks"
        hooks.mkdir()
        (hooks / "pre-push").write_bytes(HOOK.read_bytes())
        (hooks / "pre-push").chmod(0o755)
        self.git("config", "core.hooksPath", ".githooks")
        self.git("remote", "add", "origin", str(self.remote))
        self.commit("base")

    def git(self, *args, check=True):
        return subprocess.run(
            ["git", *args], cwd=self.repo, env=self.env, text=True, capture_output=True, check=check
        )

    def commit(self, name):
        (self.repo / name).write_text(name)
        self.git("add", ".")
        self.git("commit", "-qm", name)

    def calls(self):
        return self.ran.read_text().splitlines() if self.ran.exists() else []

    def test_a_clean_push_runs_the_diffs_ci_directly_without_quest(self):
        self.git("push", "origin", "main")
        self.assertEqual(self.calls(), ["ci"])

    def test_ci_runs_through_quest_gate_when_quest_is_installed(self):
        # A `quest` that records its arguments, then runs the gated command.
        quest = self.bin / "quest"
        quest.write_text(f'#!/bin/sh\necho "quest $@" >> {self.ran}\nshift\nexec "$@"\n')
        quest.chmod(0o755)
        self.git("push", "origin", "main")
        self.assertEqual(self.calls(), ["quest gate just ci", "ci"])

    def test_a_shell_without_the_dev_shell_marker_enters_the_dev_shell(self):
        # `nix develop --command <hook>`, standing in with the marker the real shell exports.
        nix = self.bin / "nix"
        nix.write_text(f'#!/bin/sh\necho "nix $1" >> {self.ran}\nshift 2\nRUST2_DEV_SHELL=1 exec "$@"\n')
        nix.chmod(0o755)
        # Another shell, such as build-bench, sets IN_NIX_SHELL but lacks the hook's tools.
        self.env.update({"IN_NIX_SHELL": "impure", "RUST2_DEV_SHELL": ""})
        self.git("push", "origin", "main")
        self.assertEqual(self.calls(), ["nix develop", "ci"])

    def test_a_failing_gated_ci_stops_the_push(self):
        quest = self.bin / "quest"
        quest.write_text('#!/bin/sh\nshift\nexec "$@"\n')
        quest.chmod(0o755)
        self.env["JUST_EXIT"] = "1"
        result = self.git("push", "origin", "main", check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("main", self.git("ls-remote", "origin").stdout)

    def test_a_linked_worktree_leaves_git_dir_unset_for_the_recipes(self):
        worktree = Path(self.temp.name) / "worktree"
        self.git("worktree", "add", "-q", "-b", "feature", str(worktree))
        subprocess.run(["git", "push", "origin", "feature"], cwd=worktree, env=self.env, check=True)
        self.assertEqual(self.calls(), ["ci"])

    def test_a_failing_ci_stops_the_push(self):
        self.env["JUST_EXIT"] = "1"
        result = self.git("push", "origin", "main", check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("main", self.git("ls-remote", "origin").stdout)

    def test_deleting_a_branch_or_pushing_a_tag_runs_nothing(self):
        self.git("push", "--no-verify", "origin", "main")
        self.git("tag", "v1")
        self.git("push", "origin", "v1")
        self.git("push", "origin", "--delete", "main")
        self.assertEqual(self.calls(), [])

    def test_uncommitted_changes_refuse_the_push(self):
        for dirty in ("tracked", "untracked"):
            with self.subTest(dirty=dirty):
                (self.repo / "base" if dirty == "tracked" else self.repo / "new").write_text("more")
                result = self.git("push", "origin", "main", check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("uncommitted", result.stderr)
                self.git("checkout", "-q", ".")
                self.git("clean", "-fdq")
        self.assertEqual(self.calls(), [])

    def test_a_branch_that_is_not_checked_out_refuses_the_push(self):
        self.git("branch", "other")
        self.commit("next")
        result = self.git("push", "origin", "other", check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("not the checked-out commit", result.stderr)
        self.assertEqual(self.calls(), [])


if __name__ == "__main__":
    unittest.main()
