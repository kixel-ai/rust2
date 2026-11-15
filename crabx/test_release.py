"""Exercise crabx's release planning, npm packages, launcher, installer, and publish job offline."""

import importlib.util
import io
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("release", HERE / "release.py")
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
LAUNCHER = HERE / "npm/crabx.js"
# Node and Bun exec the binary; "old node" lacks process.execve and runs it as a child.
RUNTIMES = ("node", "bun", "old node")
WORKFLOW = HERE.parent / ".github/workflows/release.yml"


def index(*entries):
    # The index's own compact form, which install.sh reads with sed.
    entries = ({"name": "crabx", "vers": v, "deps": [{"name": "dep", "req": "^1"}], "cksum": "0", "features": {}, "yanked": y} for v, y in entries)
    return "\n".join(json.dumps(e, separators=(",", ":")) for e in entries).encode()


def archive(path, members):
    with tarfile.open(path, "w:gz") as tar:
        for name, data in members.items():
            info = tarfile.TarInfo(name)
            info.size = len(data)
            info.mode = 0o755
            tar.addfile(info, io.BytesIO(data))


def archives(directory, version):
    directory.mkdir(parents=True, exist_ok=True)
    for target in r.NPM_TARGETS:
        filename = "crabx.exe" if "windows" in target else "crabx"
        archive(directory / f"crabx-{target}-v{version}.tgz", {f"crabx-{target}-v{version}/{filename}": target.encode()})


class Plan(unittest.TestCase):
    def test_newest_stable_skips_yanked_and_prereleases(self):
        versions = {"0.9.0": False, "0.10.0": False, "0.11.0": True, "1.0.0-rc.1": False, "0.10.1+build-5": False}
        self.assertEqual(r.newest_stable(versions), "0.10.1+build-5")

    def test_newest_stable_refuses_garbage_and_nothing(self):
        with self.assertRaises(ValueError):
            r.newest_stable({"1.0": False})
        with self.assertRaises(SystemExit):
            r.newest_stable({"1.0.0": True, "2.0.0-beta": False})

    def test_unpublished_crate_needs_a_manual_first_publish(self):
        with patch.object(r, "fetch", return_value=None), self.assertRaisesRegex(SystemExit, "by hand"):
            r.published()

    def run_crate(self, versions, bare=frozenset()):
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / "out"
            env = {"GITHUB_OUTPUT": str(out), "GITHUB_SHA": "c" * 40}
            with patch.dict(os.environ, env), patch.object(r, "fetch", return_value=index(*versions)), patch.object(r, "bare_tags", return_value=bare):
                r.crate()
            return dict(line.split("=", 1) for line in out.read_text().splitlines())

    def test_crate_skips_a_published_version_even_yanked(self):
        version = r.tomllib.loads((HERE / "Cargo.toml").read_text())["package"]["version"]
        self.assertEqual(self.run_crate([(version, True)]), {"publish": "false", "builds": '{"include":[]}'})

    def test_crate_builds_a_new_version_on_every_target_first(self):
        result = self.run_crate([("0.0.1", False)])
        self.assertEqual(result["publish"], "true")
        builds = json.loads(result["builds"])["include"]
        self.assertEqual({b["target"] for b in builds}, set(r.NPM_TARGETS))
        self.assertTrue(all(b["sha"] == "c" * 40 and b["path"] == "crabx" and b["os"] for b in builds))

    def test_crate_refuses_bare_tags_on_new_or_published_versions(self):
        # The pipeline would release under them, where npm and the installers never look.
        version = r.tomllib.loads((HERE / "Cargo.toml").read_text())["package"]["version"]
        self.assertEqual(self.run_crate([("0.0.1", False)], {"9.9.9"})["publish"], "true")
        for bare in ({version}, {"0.0.1"}):
            with self.assertRaisesRegex(SystemExit, "delete them"):
                self.run_crate([("0.0.1", False)], bare)

    def test_npm_plan_publishes_when_any_package_is_missing(self):
        def fetch(url):
            if url == r.INDEX:
                return index(("0.1.0", False), ("0.2.0", False))
            return None if url.endswith("/@crabx/win32-arm64/0.2.0") else b"{}"

        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / "out"
            with patch.dict(os.environ, {"GITHUB_OUTPUT": str(out)}), patch.object(r, "fetch", side_effect=fetch):
                r.npm_plan()
            self.assertEqual(out.read_text(), "version=0.2.0\npublish=true\n")

    def test_missing_looks_up_the_version_npm_stores(self):
        # npm drops build metadata on publish, so 0.2.0+build-5 is stored as 0.2.0.
        with patch.object(r, "fetch", side_effect=lambda url: b"{}" if url.endswith("/0.2.0") else None):
            self.assertEqual(r.missing("0.2.0+build-5"), [])


class Pack(unittest.TestCase):
    def test_packages_select_by_platform_and_carry_the_binary(self):
        with tempfile.TemporaryDirectory() as temp:
            temp = Path(temp)
            archives(temp / "archives", "1.2.3")
            r.pack("1.2.3", temp / "archives", temp / "packages")
            main = json.loads((temp / "packages/crabx/package.json").read_text())
            self.assertEqual(main["bin"], {"crabx": "crabx.js"})
            self.assertEqual(main["optionalDependencies"], {r.platform_name(t): "1.2.3" for t in r.NPM_TARGETS})
            self.assertEqual((temp / "packages/crabx/crabx.js").read_bytes(), LAUNCHER.read_bytes())
            linux = json.loads((temp / "packages/@crabx/linux-arm64-gnu/package.json").read_text())
            self.assertEqual((linux["os"], linux["cpu"], linux["libc"]), (["linux"], ["arm64"], ["glibc"]))
            windows = json.loads((temp / "packages/@crabx/win32-x64/package.json").read_text())
            self.assertNotIn("libc", windows)
            self.assertEqual((temp / "packages/@crabx/win32-x64/bin/crabx.exe").read_text(), "x86_64-pc-windows-msvc")
            # npm keeps the executable bit, so the launcher can run the binary as installed.
            packed = subprocess.run(
                ["npm", "pack", "--dry-run", "--json", str(temp / "packages/@crabx/darwin-arm64")],
                check=True, capture_output=True, text=True, cwd=temp,
            )
            files = {f["path"]: f["mode"] for f in json.loads(packed.stdout)[0]["files"]}
            self.assertEqual(files, {"bin/crabx": 0o755, "LICENSE": 0o644, "package.json": 0o644})

    def test_archive_with_another_layout_is_refused(self):
        with tempfile.TemporaryDirectory() as temp:
            temp = Path(temp)
            for members in (
                {"crabx-x86_64-apple-darwin-v1.0.0/crabx": b"", "crabx-x86_64-apple-darwin-v1.0.0/extra": b""},
                {"../crabx": b""},
                {"crabx-x86_64-apple-darwin-v1.0.1/crabx": b""},
            ):
                archive(temp / "a.tgz", members)
                with self.assertRaises(ValueError):
                    r.extract(temp / "a.tgz", "x86_64-apple-darwin", "1.0.0", temp / "out")

    def test_launcher_names_the_packages_release_publishes(self):
        script = f"const l = require({json.dumps(str(LAUNCHER))}); console.log(JSON.stringify(Object.fromEntries(Object.entries(l.TARGETS).map(([k, t]) => [t, l.packageName(...k.split(' '))]))))"
        names = json.loads(subprocess.run(["node", "-e", script], check=True, capture_output=True, text=True).stdout)
        self.assertEqual(names, {target: r.platform_name(target) for target in r.NPM_TARGETS})

    def test_attestation_must_name_this_crate_and_version(self):
        def attested(crate, version):
            parameters = {"crate": crate, "version": version}
            out = [{"verificationResult": {"statement": {"predicate": {"buildDefinition": {"externalParameters": parameters}}}}}]
            return subprocess.CompletedProcess([], 0, json.dumps(out), "")

        with patch.object(r.subprocess, "run", return_value=attested("crabx", "1.0.0")) as run:
            r.verify(Path("crabx-x86_64-apple-darwin-v1.0.0.tgz"), "1.0.0")
        argv = run.call_args.args[0]
        self.assertEqual(argv[argv.index("--signer-workflow") + 1], r.SIGNER)
        # The signer names only the pipeline's path, so a run from another branch passes it.
        self.assertEqual(argv[argv.index("--source-ref") + 1], "refs/heads/main")
        self.assertIn("--deny-self-hosted-runners", argv)
        for crate, version in (("crabx-pipeline-fixture", "1.0.0"), ("crabx", "1.0.1")):
            with patch.object(r.subprocess, "run", return_value=attested(crate, version)), self.assertRaises(ValueError):
                r.verify(Path("a.tgz"), "1.0.0")


@unittest.skipIf(sys.platform == "win32", "the fake binary is a shell script")
class Launcher(unittest.TestCase):
    """Install the launcher beside a platform package whose binary is a shell script."""

    def setUp(self):
        self.temp = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, self.temp)
        self.root = self.temp / "node_modules"
        (self.root / "crabx").mkdir(parents=True)
        shutil.copy(LAUNCHER, self.root / "crabx/crabx.js")
        arch = {"x86_64": "x64", "aarch64": "arm64", "arm64": "arm64"}[os.uname().machine]
        platform = {"linux": "linux", "darwin": "darwin"}[sys.platform]
        self.package = self.root / r.platform_name(next(t for t, p in r.NPM_TARGETS.items() if p == (platform, arch)))
        (self.package / "bin").mkdir(parents=True)
        (self.package / "package.json").write_text("{}")
        # A Node without process.execve, to reach the child-process fallback.
        self.old_node = self.temp / "old-node.js"
        self.old_node.write_text("process.execve = undefined;\n")

    def binary(self, body):
        path = self.package / "bin/crabx"
        path.write_text("#!/bin/sh\n" + body)
        path.chmod(0o755)

    def command(self, runtime):
        """The launcher under one of the runtimes RUNTIMES names."""
        launcher = str(self.root / "crabx/crabx.js")
        return ["node", "--require", str(self.old_node), launcher] if runtime == "old node" else [runtime, launcher]

    def launch(self, runtime, *args):
        return subprocess.run([*self.command(runtime), *args], capture_output=True, text=True, timeout=30)

    def test_arguments_streams_and_exit_code_pass_through(self):
        self.binary('printf "%s|" "$@"; echo err >&2; exit 7\n')
        for runtime in RUNTIMES:
            result = self.launch(runtime, "moq-cli", "--flag", "two words", "")
            self.assertEqual((result.returncode, result.stdout, result.stderr), (7, "moq-cli|--flag|two words||", "err\n"), runtime)

    def test_death_by_signal_passes_through(self):
        for runtime in RUNTIMES:
            self.binary("kill -TERM $$\n")
            self.assertEqual(self.launch(runtime).returncode, -signal.SIGTERM, runtime)
            # exec resets the runtime's ignored SIGPIPE; a child dies of it, but Node survives it,
            # so the shell's code for it stands in.
            self.binary("kill -PIPE $$\n")
            expected = 128 + signal.SIGPIPE if runtime == "old node" else -signal.SIGPIPE
            self.assertEqual(self.launch(runtime).returncode, expected, runtime)

    def test_exec_replaces_the_launcher(self):
        self.binary("echo $$\n")
        for runtime in ("node", "bun"):
            child = subprocess.Popen(self.command(runtime), stdout=subprocess.PIPE, text=True)
            self.assertEqual(child.communicate(timeout=30)[0], f"{child.pid}\n", runtime)

    def test_signals_reach_the_binary(self):
        # `wait` returns as soon as the trapped signal arrives, unlike a foreground sleep.
        self.binary("trap 'kill $!; echo got INT; exit 2' INT\ntrap 'kill $!; echo got TERM; exit 3' TERM\necho ready\nsleep 60 >/dev/null &\nwait\n")
        # The fallback ignores SIGINT, which a terminal already sends the binary through the group.
        cases = [(r, s) for r in ("node", "bun") for s in (signal.SIGINT, signal.SIGTERM)] + [("old node", signal.SIGTERM)]
        for runtime, sig in cases:
            child = subprocess.Popen(self.command(runtime), stdout=subprocess.PIPE, text=True)
            self.assertEqual(child.stdout.readline(), "ready\n")
            child.send_signal(sig)
            name = "INT" if sig == signal.SIGINT else "TERM"
            self.assertEqual(child.communicate(timeout=30), (f"got {name}\n", None), (runtime, name))
            self.assertEqual(child.returncode, 2 if sig == signal.SIGINT else 3, (runtime, name))

    def test_missing_platform_package_fails_loudly(self):
        shutil.rmtree(self.package)
        result = self.launch("node")
        self.assertEqual(result.returncode, 1)
        self.assertIn("is not installed; reinstall crabx without omitting optional dependencies", result.stderr)


@unittest.skipIf(sys.platform == "win32", "install.sh is for Unix")
class Install(unittest.TestCase):
    """Run install.sh against fake curl, uname, and getconf commands."""

    def setUp(self):
        self.temp = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, self.temp)
        self.bin = self.temp / "fakes"
        self.bin.mkdir()
        self.served = self.temp / "served"
        self.served.mkdir()
        (self.served / "index").write_bytes(index(("0.9.0", False), ("0.10.0", False), ("0.11.0", True), ("1.0.0-rc.1", False)))
        for version in ("0.10.0", "0.9.0"):
            name = f"crabx-x86_64-unknown-linux-gnu-v{version}"
            archive(self.served / f"{name}.tgz", {f"{name}/crabx": f"#!/bin/sh\necho crabx {version}\n".encode()})
        self.fake("uname", 'case $1 in -s) echo Linux ;; -m) echo x86_64 ;; esac')
        self.fake("getconf", "echo glibc 2.28")
        self.fake("curl", f"""
for arg; do
	case $arg in https://*) url=$arg ;; esac
	[ "$prev" = -o ] && out=$arg
	prev=$arg
done
case $url in
https://index.crates.io/cr/ab/crabx) cat {self.served}/index ;;
https://github.com/kixel-ai/rust2/releases/download/crabx-v*/*) cp "{self.served}/${{url##*/}}" "$out" 2>/dev/null || exit 22 ;;
*) exit 22 ;;
esac""")

    def fake(self, name, body):
        (self.bin / name).write_text("#!/bin/sh\n" + body + "\n")
        (self.bin / name).chmod(0o755)

    def install(self, **env):
        env = {"PATH": f"{self.bin}:{os.environ['PATH']}", "HOME": str(self.temp / "home"), **env}
        return subprocess.run(["sh", str(HERE / "install.sh")], capture_output=True, text=True, env=env, timeout=30)

    def test_installs_the_newest_stable_release(self):
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        installed = self.temp / "home/.local/bin/crabx"
        self.assertEqual(subprocess.run([installed], capture_output=True, text=True).stdout, "crabx 0.10.0\n")
        self.assertIn("add", result.stderr)

    def test_a_hyphen_in_build_metadata_is_stable(self):
        (self.served / "index").write_bytes(index(("0.10.0", False), ("0.10.1+build-5", False), ("0.11.0-rc.1", False)))
        name = "crabx-x86_64-unknown-linux-gnu-v0.10.1+build-5"
        archive(self.served / f"{name}.tgz", {f"{name}/crabx": b"#!/bin/sh\necho crabx 0.10.1+build-5\n"})
        result = self.install()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(subprocess.run([self.temp / "home/.local/bin/crabx"], capture_output=True, text=True).stdout, "crabx 0.10.1+build-5\n")

    def test_a_pinned_version_and_directory(self):
        result = self.install(CRABX_VERSION="0.9.0", CRABX_INSTALL_DIR=str(self.temp / "dir"))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(subprocess.run([self.temp / "dir/crabx"], capture_output=True, text=True).stdout, "crabx 0.9.0\n")

    def test_unreleased_binaries_and_bad_versions_fail(self):
        result = self.install(CRABX_VERSION="0.11.0")
        self.assertEqual(result.returncode, 1)
        self.assertIn("may not be released yet", result.stderr)
        self.assertIn("invalid version", self.install(CRABX_VERSION="1.0.0/../x").stderr)

    def test_musl_and_old_glibc_fail(self):
        self.fake("getconf", "exit 1")
        self.assertIn("has none", self.install().stderr)
        self.fake("getconf", "echo glibc 2.17")
        self.assertIn("not glibc 2.17", self.install().stderr)
        self.assertFalse((self.temp / "home/.local/bin/crabx").exists())

    def test_targets(self):
        self.fake("getconf", "echo glibc 2.39")
        for system, machine, target in (("Linux", "aarch64", "aarch64-unknown-linux-gnu"), ("Darwin", "x86_64", "x86_64-apple-darwin"), ("Darwin", "arm64", "aarch64-apple-darwin")):
            self.fake("uname", f'case $1 in -s) echo {system} ;; -m) echo {machine} ;; esac')
            self.fake("sysctl", "echo 0")
            self.assertIn(f"for {target}", self.install(CRABX_VERSION="0.10.0").stderr)
        # A shell under Rosetta still installs the native build.
        self.fake("sysctl", "echo 1")
        self.assertIn("for aarch64-apple-darwin", self.install(CRABX_VERSION="0.10.0").stderr)
        self.fake("uname", "case $1 in -s) echo MINGW64_NT-10.0 ;; -m) echo x86_64 ;; esac")
        self.assertIn("install.ps1", self.install().stderr)


def jobs():
    """Each job's lines in release.yml, by name."""
    text = WORKFLOW.read_text()
    body = text.split("\njobs:\n", 1)[1]
    return {match[1]: match[0] for match in re.finditer(r"(?ms)^  ([a-z-]+):\n.*?(?=^  [a-z-]+:\n|\Z)", body)}


class Workflow(unittest.TestCase):
    def test_npm_job_is_pinned(self):
        # Changing the one job that can publish to npm means changing this test too.
        self.assertEqual(jobs()["npm"], """  npm:
    needs: npm-plan
    if: ${{ !cancelled() && needs.npm-plan.result == 'success' && needs.npm-plan.outputs.publish == 'true' }}
    runs-on: ubuntu-24.04
    environment: npm
    permissions:
      contents: read
      attestations: read
      id-token: write
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      # Trusted publishing needs npm 11.5.1 or later.
      - uses: actions/setup-node@949feb2413d6458794dcd2491c4babbbce0c15c1 # v7.1.0
        with:
          node-version: '24'
      - name: Publish
        env:
          GH_TOKEN: ${{ github.token }}
          VERSION: ${{ needs.npm-plan.outputs.version }}
        run: python3 crabx/release.py publish "$VERSION"
""")

    def test_npm_jobs_run_after_a_skipped_crate(self):
        # A retry skips crate once crates.io has the version; npm must still catch up.
        found = jobs()
        for name, needs in (("npm-plan", "binaries"), ("npm", "npm-plan")):
            self.assertIn(f"if: ${{{{ !cancelled() && needs.{needs}.result == 'success'", found[name], name)

    def test_publishing_rights_stay_in_their_jobs(self):
        found = jobs()
        self.assertEqual({name for name, job in found.items() if "id-token: write" in job}, {"crate", "binaries", "npm"})
        self.assertEqual({name for name, job in found.items() if "environment:" in job}, {"crate", "npm"})
        self.assertIn("environment: crates-io\n", found["crate"])
        # Build scripts run only where no OIDC credentials can be requested.
        self.assertIn("cargo package --locked", found["package"])
        self.assertNotIn("cargo package", found["crate"])
        self.assertIn("cargo publish --locked --no-verify", found["crate"])
        head = WORKFLOW.read_text().split("\njobs:\n", 1)[0]
        self.assertNotIn("pull_request", head)
        self.assertIn("permissions:\n  contents: read\n", head)

    def test_installer_urls_are_deployed(self):
        # Every crabx.kixel.ai URL users are told to fetch names a file the Pages job copies.
        site = (HERE.parent / ".github/workflows/installers.yml").read_text()
        texts = [(HERE / f).read_text() for f in ("README.md", "install.sh", "install.ps1")]
        urls = {u for t in texts for u in re.findall(r"https://crabx\.kixel\.ai/(\S*?)(?=[\s\"|])", t)}
        self.assertEqual(urls, {"install.sh", "install.ps1"})
        for name in urls:
            self.assertIn(f"crabx/{name}", site)
        self.assertEqual("".join(texts).count("raw.githubusercontent"), 0)


if __name__ == "__main__":
    unittest.main()
