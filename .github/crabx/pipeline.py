"""Build crates.io releases and publish binstall-compatible archives."""

import datetime
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
import tarfile
import tomllib
import urllib.error
import urllib.parse
import urllib.request

TARGETS = [
    ("x86_64-unknown-linux-gnu", "ubuntu-24.04"),
    ("aarch64-unknown-linux-gnu", "ubuntu-24.04-arm"),
    ("x86_64-apple-darwin", "macos-15"),
    ("aarch64-apple-darwin", "macos-15"),
    ("x86_64-pc-windows-msvc", "windows-latest"),
    ("aarch64-pc-windows-msvc", "windows-11-arm"),
]


def run(*args, cwd=None):
    return subprocess.check_output(args, cwd=cwd, text=True).strip()


def request(url, data=None, missing=False):
    headers = {"User-Agent": "crabx-release-pipeline (github.com/kixel-ai/rust2)"}
    if url.startswith("https://api.github.com/"):
        headers["Authorization"] = "Bearer " + os.environ["GH_TOKEN"]
        headers["Accept"] = "application/vnd.github+json"
    body = None if data is None else json.dumps(data).encode()
    if body is not None:
        headers["Content-Type"] = "application/json"
    try:
        with urllib.request.urlopen(urllib.request.Request(url, body, headers)) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        if missing and error.code == 404:
            return None
        raise


def api(path, **kwargs):
    return request("https://api.github.com/repos/" + os.environ["GITHUB_REPOSITORY"] + "/" + path, **kwargs)


def safe(value):
    if not re.fullmatch(r"[A-Za-z0-9_.+-]+", value) or value in (".", ".."):
        raise ValueError(f"unsafe name: {value!r}")
    return value


def asset(item, target):
    return f"{safe(item['name'])}-{target}-v{safe(item['version'])}.tgz"


def tag(item):
    found = []
    for name in (f"{item['name']}-v{item['version']}", f"v{item['version']}"):
        ref = api("git/ref/tags/" + urllib.parse.quote(name, safe=""), missing=True)
        if ref is None:
            continue
        obj = ref["object"]
        seen = set()
        while obj["type"] == "tag":
            if obj["sha"] in seen:
                raise ValueError("cyclic annotated tag")
            seen.add(obj["sha"])
            obj = api("git/tags/" + obj["sha"])["object"]
        if obj["type"] != "commit" or obj["sha"] != item["sha"]:
            raise ValueError(f"{name} does not point to published commit {item['sha']}")
        found.append(name)
    return found[0] if found else f"{item['name']}-v{item['version']}"


def published(name, version):
    # Stream the archive, never extract paths supplied by the registry.
    url = f"https://static.crates.io/crates/{name}/{name}-{version}.crate"
    req = urllib.request.Request(url, headers={"User-Agent": "crabx-release-pipeline"})
    wanted = {f"{name}-{version}/Cargo.toml", f"{name}-{version}/.cargo_vcs_info.json"}
    files = {}
    with urllib.request.urlopen(req) as response, tarfile.open(fileobj=response, mode="r|gz") as archive:
        for member in archive:
            if member.name in wanted:
                if not member.isfile() or member.name in files:
                    raise ValueError("invalid or duplicate crate metadata")
                files[member.name] = archive.extractfile(member).read()
    manifest = tomllib.loads(files[f"{name}-{version}/Cargo.toml"].decode())
    if not manifest.get("bin"):
        return manifest, None
    vcs = json.loads(files[f"{name}-{version}/.cargo_vcs_info.json"])
    if vcs["git"].get("dirty", False) or not re.fullmatch(r"[0-9a-f]{40}", vcs["git"]["sha1"]):
        raise ValueError(f"{name}@{version}: dirty or invalid published commit")
    path = PurePosixPath(vcs.get("path_in_vcs", ""))
    if path.is_absolute() or ".." in path.parts or "\\" in str(path):
        raise ValueError("invalid published crate path")
    if manifest["package"]["name"] != name or manifest["package"]["version"] != version:
        raise ValueError("registry archive identity mismatch")
    return manifest, {"name": safe(name), "version": safe(version), "sha": vcs["git"]["sha1"], "path": str(path)}


def discover():
    workflow = os.environ["CALLER_WORKFLOW"].split("/.github/", 1)[1].split("@", 1)[0]
    added = run("git", "log", "--follow", "--diff-filter=A", "--format=%cI", "--", ".github/" + workflow).splitlines()
    if not added:
        raise ValueError("cannot find when caller workflow was added")
    since = datetime.datetime.fromisoformat(added[-1])
    names = set()
    for path in run("git", "ls-files", "**/Cargo.toml", "Cargo.toml").splitlines():
        package = tomllib.loads(Path(path).read_text()).get("package", {})
        if package and package.get("publish") != False:
            names.add(safe(package["name"]))
    items = []
    repo = "https://github.com/" + os.environ["GITHUB_REPOSITORY"]
    for name in sorted(names):
        info = request(f"https://crates.io/api/v1/crates/{name}", missing=True)
        if info is None or info["crate"].get("repository", "").removesuffix("/").removesuffix(".git").lower() != repo.lower():
            continue
        for version in info["versions"]:
            if version["yanked"] or datetime.datetime.fromisoformat(version["created_at"].replace("Z", "+00:00")) < since:
                continue
            manifest, item = published(name, safe(version["num"]))
            if manifest["package"].get("repository", "").removesuffix("/").removesuffix(".git").lower() != repo.lower():
                continue
            if not manifest.get("bin"):
                continue
            item["tag"] = tag(item)
            release = api("releases/tags/" + urllib.parse.quote(item["tag"], safe=""), missing=True)
            item["targets"] = [target for target, _ in TARGETS]
            if release is not None:
                assets = run("gh", "api", "--paginate", f"repos/{os.environ['GITHUB_REPOSITORY']}/releases/{release['id']}/assets", "--jq", ".[].name").splitlines()
                item["targets"] = [target for target, _ in TARGETS if asset(item, target) not in assets]
                if not item["targets"]:
                    continue
            items.append(item)
    return items


def build(item, target):
    safe(item["name"])
    safe(item["version"])
    manifest = str((Path("source") / item["path"] / "Cargo.toml").resolve())
    metadata = json.loads(run("cargo", "metadata", "--locked", "--no-deps", "--format-version=1", "--manifest-path", manifest, cwd=str(Path(manifest).parent)))
    packages = [p for p in metadata["packages"] if p["name"] == item["name"] and p["version"] == item["version"]]
    if len(packages) != 1:
        raise ValueError("published crate does not match checked-out commit")
    bins = [safe(t["name"]) for t in packages[0]["targets"] if "bin" in t["kind"]]
    if not bins:
        raise ValueError("crate has no binaries")
    command = ["cargo-zigbuild", "zigbuild"] if "linux" in target else ["cargo", "build"]
    build_target = target + ".2.28" if "linux" in target else target
    run(*command, "--locked", "--release", "--bins", "--all-features", "--manifest-path", manifest, "--target", build_target, cwd=str(Path(manifest).parent))
    out = Path("archives")
    out.mkdir(exist_ok=True)
    root = asset(item, target).removesuffix(".tgz")
    with tarfile.open(out / asset(item, target), "w:gz") as archive:
        for binary in bins:
            filename = binary + (".exe" if "windows" in target else "")
            archive.add(Path(metadata["target_directory"]) / target / "release" / filename, arcname=root + "/" + filename)


def publish(item):
    # This job executes only pipeline code, never the crate's code or Cargo.
    for target in item.get("targets", [t for t, _ in TARGETS]):
        if not (Path("archives") / asset(item, target)).is_file():
            raise ValueError(f"missing archive for {target}")
    name = tag(item)
    if name != item["tag"]:
        raise ValueError("release tag changed since discovery")
    release = api("releases/tags/" + urllib.parse.quote(name, safe=""), missing=True)
    if release is None:
        release = api("releases", data={"tag_name": name, "target_commitish": item["sha"], "name": name, "prerelease": "-" in item["version"], "make_latest": "false"})
    existing = json.loads(run("gh", "api", "--paginate", "--slurp", f"repos/{os.environ['GITHUB_REPOSITORY']}/releases/{release['id']}/assets"))
    existing = {a["name"]: a for page in existing for a in page}
    for target, _ in TARGETS:
        path = Path("archives") / asset(item, target)
        if path.name not in existing:
            if not path.is_file():
                raise ValueError(f"missing asset: {path.name}")
            run("gh", "release", "upload", name, str(path), "--repo", os.environ["GITHUB_REPOSITORY"])



def provenance(item):
    repo = os.environ["GITHUB_REPOSITORY"]
    workflow = os.environ["PIPELINE_WORKFLOW"]
    return {
        "buildDefinition": {
            "buildType": "https://github.com/kixel-ai/rust2/tree/main/.github/crabx",
            "externalParameters": {"crate": item["name"], "version": item["version"], "path": item["path"]},
            "internalParameters": {},
            "resolvedDependencies": [
                {"uri": f"git+https://github.com/{repo}", "digest": {"gitCommit": item["sha"]}},
                {"uri": "https://github.com/" + workflow.split("@", 1)[0], "digest": {"gitCommit": os.environ["PIPELINE_SHA"]}},
            ],
        },
        "runDetails": {
            "builder": {"id": "https://github.com/" + workflow},
            "metadata": {"invocationId": f"https://github.com/{repo}/actions/runs/{os.environ['GITHUB_RUN_ID']}/attempts/{os.environ['GITHUB_RUN_ATTEMPT']}"},
        },
    }


def main():
    mode = sys.argv[1]
    if mode in ("discover", "fixture"):
        items = discover() if mode == "discover" else [{"name": "crabx-pipeline-fixture", "version": "0.0.0", "sha": os.environ["GITHUB_SHA"], "path": ".github/crabx/fixture"}]
        matrix = [{**item, "target": target, "os": runner} for item in items for target, runner in TARGETS if target in item.get("targets", [t for t, _ in TARGETS])]
        # GitHub's matrix has a hard limit. Do not silently omit old releases.
        if len(matrix) > 256:
            raise ValueError("backfill exceeds GitHub's 256-job matrix; publish older versions in batches")
        with open(os.environ["GITHUB_OUTPUT"], "a") as output:
            for key, value in {"builds": {"include": matrix}, "releases": {"include": items}, "any": bool(items)}.items():
                print(f"{key}={json.dumps(value, separators=(',', ':'))}", file=output)
    elif mode == "build":
        build(json.loads(os.environ["ITEM"]), os.environ["TARGET"])
    elif mode == "provenance":
        Path("predicate.json").write_text(json.dumps(provenance(json.loads(os.environ["ITEM"]))))
    elif mode == "publish":
        publish(json.loads(os.environ["ITEM"]))
    else:
        raise ValueError(f"unknown mode: {mode}")


if __name__ == "__main__":
    main()
