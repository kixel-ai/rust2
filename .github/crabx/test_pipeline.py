"""Exercise release discovery, tag validation, and recovery without write credentials."""

import importlib.util
import io
import json
import os
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("pipeline", Path(__file__).with_name("pipeline.py"))
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)
ITEM = {"name": "demo", "version": "1.2.3", "sha": "a" * 40, "path": "cli", "tag": "demo-v1.2.3"}
REPO = "https://github.com/example/demo"


def crate(files):
    archive = io.BytesIO()
    with tarfile.open(fileobj=archive, mode="w:gz") as tar:
        for name, value in files.items():
            info = tarfile.TarInfo("demo-1.2.3/" + name)
            info.size = len(value)
            tar.addfile(info, io.BytesIO(value))
    archive.seek(0)
    return archive


class Pipeline(unittest.TestCase):
    PROVENANCE_ENV = {
        "GITHUB_REPOSITORY": "example/demo",
        "GITHUB_WORKFLOW_REF": "example/demo/.github/workflows/binaries.yml@refs/heads/main",
        "GITHUB_EVENT_NAME": "push",
        "GITHUB_REPOSITORY_ID": "1",
        "GITHUB_REPOSITORY_OWNER_ID": "2",
        "RUNNER_ENVIRONMENT": "github-hosted",
        "GITHUB_RUN_ID": "3",
        "GITHUB_RUN_ATTEMPT": "1",
        "PIPELINE_WORKFLOW": "kixel-ai/rust2/.github/workflows/crabx.yml@refs/tags/pipeline-v1.0.0",
        "PIPELINE_SHA": "b" * 40,
        "GITHUB_SHA": "c" * 40,
    }

    def test_provenance_uses_githubs_workflow_build_type(self):
        with patch.dict(os.environ, self.PROVENANCE_ENV):
            definition = p.provenance(ITEM)["buildDefinition"]
        self.assertEqual(definition["buildType"], "https://actions.github.io/buildtypes/workflow/v1")
        self.assertEqual(definition["externalParameters"], {
            "workflow": {"ref": "refs/heads/main", "repository": REPO, "path": ".github/workflows/binaries.yml"},
            "crate": "demo", "version": "1.2.3", "path": "cli",
        })
        self.assertEqual(definition["internalParameters"]["github"]["runner_environment"], "github-hosted")
        self.assertEqual(definition["resolvedDependencies"], [
            {"uri": "git+https://github.com/example/demo@refs/heads/main", "digest": {"gitCommit": "c" * 40}},
            {"uri": "git+https://github.com/example/demo", "digest": {"gitCommit": ITEM["sha"]}},
            {"uri": "https://github.com/kixel-ai/rust2/.github/workflows/crabx.yml", "digest": {"gitCommit": "b" * 40}},
        ])

    def test_provenance_refuses_a_workflow_from_another_repository(self):
        env = {**self.PROVENANCE_ENV, "GITHUB_WORKFLOW_REF": "other/demo/.github/workflows/binaries.yml@refs/heads/main"}
        with patch.dict(os.environ, env), self.assertRaisesRegex(ValueError, "not in example/demo"):
            p.provenance(ITEM)

    def test_tag_identity(self):
        with patch.object(p, "api", return_value=None):
            self.assertEqual(p.tag(ITEM), "demo-v1.2.3")
        for kind in ("commit", "tag"):
            def api(path, **kwargs):
                if path == "git/ref/tags/demo-v1.2.3":
                    return {"object": {"type": kind, "sha": ITEM["sha"]}}
                if path.startswith("git/tags/"):
                    return {"object": {"type": "commit", "sha": ITEM["sha"]}}
            with patch.object(p, "api", side_effect=api):
                self.assertEqual(p.tag(ITEM), ITEM["tag"])
        with patch.object(p, "api", return_value={"object": {"type": "commit", "sha": "b" * 40}}):
            with self.assertRaisesRegex(ValueError, "does not point"):
                p.tag(ITEM)
        # A bare version tag on another commit is another crate's, never this one's.
        for sha, expected in (("b" * 40, "demo-v1.2.3"), (ITEM["sha"], "v1.2.3")):
            def api(path, **kwargs):
                if path == "git/ref/tags/v1.2.3":
                    return {"object": {"type": "commit", "sha": sha}}
            with patch.object(p, "api", side_effect=api):
                self.assertEqual(p.tag(ITEM), expected)
        # A crate named like a version tag still needs its own tag to match.
        vtool = {**ITEM, "name": "vtool"}
        with patch.object(p, "api", side_effect=lambda path, **kwargs: {"object": {"type": "commit", "sha": "b" * 40}} if path == "git/ref/tags/vtool-v1.2.3" else None):
            with self.assertRaisesRegex(ValueError, "does not point"):
                p.tag(vtool)

    def test_registry_metadata(self):
        manifest = f'[package]\nname="demo"\nversion="1.2.3"\nrepository="{REPO}"\n[[bin]]\nname="demo"\n'.encode()
        for dirty, path in ((False, "cli"), (True, "cli"), (False, "../escape"), (False, "/escape")):
            archive = crate({"Cargo.toml": manifest, ".cargo_vcs_info.json": json.dumps({"git": {"sha1": ITEM["sha"], "dirty": dirty}, "path_in_vcs": path}).encode()})
            with patch.object(p.urllib.request, "urlopen", return_value=archive):
                if dirty or path != "cli":
                    with self.assertRaises(ValueError):
                        p.published("demo", "1.2.3", REPO)
                else:
                    manifest_, item = p.published("demo", "1.2.3", REPO)
                    self.assertEqual(item["sha"], ITEM["sha"])
                    self.assertEqual(manifest_["package"]["version"], ITEM["version"])
        with patch.object(p.urllib.request, "urlopen", return_value=crate({"Cargo.toml": manifest})):
            with self.assertRaisesRegex(ValueError, "without its git commit"):
                p.published("demo", "1.2.3", REPO)
        # Another repository's release is skipped before its source is checked.
        dirty = json.dumps({"git": {"sha1": ITEM["sha"], "dirty": True}}).encode()
        with patch.object(p.urllib.request, "urlopen", return_value=crate({"Cargo.toml": manifest, ".cargo_vcs_info.json": dirty})):
            self.assertIsNone(p.published("demo", "1.2.3", "https://github.com/example/other")[1])

    def test_null_repository(self):
        with patch.dict(os.environ, {"GITHUB_REPOSITORY": "example/demo", "CALLER_WORKFLOW": "example/demo/.github/workflows/binaries.yml@main"}), patch.object(p, "run", side_effect=["2026-01-01T00:00:00+00:00", "Cargo.toml"]), patch.object(Path, "read_text", return_value='[package]\nname="demo"'), patch.object(p, "request", return_value={"crate": {"repository": None}}):
            self.assertEqual(p.discover(), [])

    def test_implicit_binary(self):
        for source, autobins, expected in (("src/main.rs", True, True), ("src/bin/tool.rs", True, True), ("src/bin/tool/main.rs", True, True), ("src/lib.rs", True, False), ("src/main.rs", False, False)):
            archive = crate({
                "Cargo.toml": f'[package]\nname="demo"\nversion="1.2.3"\nrepository="{REPO}"\nautobins={str(autobins).lower()}\n'.encode(),
                ".cargo_vcs_info.json": json.dumps({"git": {"sha1": ITEM["sha"]}}).encode(),
                source: b"fn main() {}",
            })
            with patch.object(p.urllib.request, "urlopen", return_value=archive):
                self.assertEqual(p.published("demo", "1.2.3", REPO)[1] is not None, expected)

    def test_template_name(self):
        with patch.dict(os.environ, {"GITHUB_REPOSITORY": "example/demo", "CALLER_WORKFLOW": "example/demo/.github/workflows/binaries.yml@main"}), patch.object(p, "run", side_effect=["2026-01-01T00:00:00+00:00", "Cargo.toml"]), patch.object(Path, "read_text", return_value='[package]\nname="{{project-name}}"'), patch.object(p, "request") as request:
            self.assertEqual(p.discover(), [])
        request.assert_not_called()

    def test_archive_layout(self):
        metadata = {"target_directory": "target", "packages": [{"name": "demo", "version": "1.2.3", "targets": [{"name": "demo", "kind": ["bin"]}, {"name": "tool", "kind": ["bin"]}, {"name": "demo", "kind": ["lib"]}]}]}
        cwd = os.getcwd()
        with tempfile.TemporaryDirectory() as tmp:
            os.chdir(tmp)
            try:
                for target in ("x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"):
                    exe = ".exe" if "windows" in target else ""
                    release = Path("target", target, "release")
                    release.mkdir(parents=True, exist_ok=True)
                    for binary in ("demo", "tool"):
                        (release / (binary + exe)).write_text(binary)
                    with patch.object(p, "run", side_effect=[json.dumps(metadata), ""]) as run:
                        p.build(ITEM, target)
                    self.assertEqual(run.call_args_list[1].args[-1], target + (".2.28" if "linux" in target else ""))
                    args = run.call_args_list[1].args
                    self.assertEqual(args[args.index("--package") + 1], "demo@1.2.3")
                    root = f"demo-{target}-v1.2.3"
                    with tarfile.open(Path("archives", root + ".tgz")) as archive:
                        self.assertEqual(sorted(archive.getnames()), [f"{root}/demo{exe}", f"{root}/tool{exe}"])
                        self.assertEqual(archive.extractfile(f"{root}/tool{exe}").read(), b"tool")
                # A crate missing any archive is not attested.
                p.built({**ITEM, "targets": ["x86_64-unknown-linux-gnu"]})
                with self.assertRaisesRegex(ValueError, "missing archive"):
                    p.built(ITEM)
            finally:
                os.chdir(cwd)

    def test_release_kind(self):
        for version, prerelease in (("1.0.0+build-5", False), ("1.0.0-rc.1+build-5", True)):
            item = {**ITEM, "version": version}
            with patch.dict(os.environ, {"GITHUB_REPOSITORY": "example/demo"}), patch.object(p, "tag", return_value=ITEM["tag"]), patch.object(p, "api", side_effect=[None, {"id": 7}]) as api, patch.object(p, "run", return_value="[[]]"), patch.object(Path, "is_file", return_value=True):
                p.publish(item)
            self.assertEqual(api.call_args.kwargs["data"]["prerelease"], prerelease)

    def test_partial_release_recovery(self):
        first = p.asset(ITEM, p.TARGETS[0][0])
        existing = [[{"name": first}, {"name": "unrelated.txt"}]]
        with patch.dict(os.environ, {"GITHUB_REPOSITORY": "example/demo"}), patch.object(p, "tag", return_value=ITEM["tag"]), patch.object(p, "api", return_value={"id": 7}), patch.object(p, "run", return_value=json.dumps(existing)) as run, patch.object(Path, "is_file", return_value=True):
            p.publish(ITEM)
        uploads = [call.args for call in run.call_args_list if call.args[:3] == ("gh", "release", "upload")]
        self.assertEqual(len(uploads), 5)
        self.assertNotIn("archives/" + first, [args[4] for args in uploads])

    def test_discovery_missing_targets(self):
        repo = "https://github.com/example/demo"
        def run(*args):
            if args[:2] == ("git", "log"):
                return "2026-01-01T00:00:00+00:00"
            if args[:2] == ("git", "ls-files"):
                return "Cargo.toml"
            return p.asset(ITEM, p.TARGETS[0][0])
        with patch.dict(os.environ, {"GITHUB_REPOSITORY": "example/demo", "CALLER_WORKFLOW": "example/demo/.github/workflows/binaries.yml@main"}), patch.object(p, "run", side_effect=run), patch.object(Path, "read_text", return_value='[package]\nname="demo"'), patch.object(p, "request", return_value={"crate": {"repository": repo}, "versions": [{"num": "1.2.3", "created_at": "2026-02-01T00:00:00Z", "yanked": False}, {"num": "0.1.0", "created_at": "2025-01-01T00:00:00Z", "yanked": False}]}), patch.object(p, "published", return_value=({"package": {"repository": repo}, "bin": [{"name": "demo"}]}, dict(ITEM))) as published, patch.object(p, "tag", return_value=ITEM["tag"]), patch.object(p, "api", return_value={"id": 7}):
            items = p.discover()
        self.assertEqual(published.call_count, 1)
        self.assertEqual(items[0]["targets"], [t for t, _ in p.TARGETS[1:]])


if __name__ == "__main__":
    unittest.main()
