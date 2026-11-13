"""Exercise release discovery, tag validation, and recovery without write credentials."""

import importlib.util
import io
import json
import os
from pathlib import Path
import tarfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("pipeline", Path(__file__).with_name("pipeline.py"))
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)
ITEM = {"name": "demo", "version": "1.2.3", "sha": "a" * 40, "path": "cli", "tag": "demo-v1.2.3"}


class Pipeline(unittest.TestCase):
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

    def test_registry_metadata(self):
        for dirty, path in ((False, "cli"), (True, "cli"), (False, "../escape"), (False, "/escape")):
            archive = io.BytesIO()
            with tarfile.open(fileobj=archive, mode="w:gz") as tar:
                for name, value in {
                    "Cargo.toml": b'[package]\nname="demo"\nversion="1.2.3"\n[[bin]]\nname="demo"\n',
                    ".cargo_vcs_info.json": json.dumps({"git": {"sha1": ITEM["sha"], "dirty": dirty}, "path_in_vcs": path}).encode(),
                }.items():
                    info = tarfile.TarInfo("demo-1.2.3/" + name)
                    info.size = len(value)
                    tar.addfile(info, io.BytesIO(value))
            archive.seek(0)
            with patch.object(p.urllib.request, "urlopen", return_value=archive):
                if dirty or path != "cli":
                    with self.assertRaises(ValueError):
                        p.published("demo", "1.2.3")
                else:
                    manifest, item = p.published("demo", "1.2.3")
                    self.assertEqual(item["sha"], ITEM["sha"])
                    self.assertEqual(manifest["package"]["version"], ITEM["version"])

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
