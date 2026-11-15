"""Publish crabx to crates.io and npm from release.yml, and pack its npm packages."""

import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib
import urllib.error
import urllib.request

REPO = "kixel-ai/rust2"
# The reusable pipeline that builds and attests every release archive.
SIGNER = f"{REPO}/.github/workflows/crabx.yml"
INDEX = "https://index.crates.io/cr/ab/crabx"
NPM = "https://registry.npmjs.org"
HERE = Path(__file__).resolve().parent
# npm's names for each release target, as the launcher maps them.
NPM_TARGETS = {
    "x86_64-unknown-linux-gnu": ("linux", "x64"),
    "aarch64-unknown-linux-gnu": ("linux", "arm64"),
    "x86_64-apple-darwin": ("darwin", "x64"),
    "aarch64-apple-darwin": ("darwin", "arm64"),
    "x86_64-pc-windows-msvc": ("win32", "x64"),
    "aarch64-pc-windows-msvc": ("win32", "arm64"),
}
VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?")


def fetch(url):
    """The response body, or None when the server has no such resource."""
    request = urllib.request.Request(url, headers={"User-Agent": f"crabx-release (github.com/{REPO})"})
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return response.read()
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise


def published():
    """Every crabx version on crates.io, yanked included, mapped to whether it is yanked."""
    body = fetch(INDEX)
    if body is None:
        raise SystemExit("crabx is not on crates.io yet: its first publish is by hand, as quest/m2/crabx/first-publish.md says")
    versions = {}
    for line in body.decode().splitlines():
        entry = json.loads(line)
        versions[entry["vers"]] = entry["yanked"]
    return versions


def newest_stable(versions):
    """The highest version that is neither yanked nor a prerelease."""
    stable = []
    for version, yanked in versions.items():
        match = VERSION.fullmatch(version)
        if not match:
            raise ValueError(f"unparsable crates.io version {version!r}")
        if not yanked and not match[4]:
            stable.append((tuple(int(part) for part in match.group(1, 2, 3)), version))
    if not stable:
        raise SystemExit("crabx has no stable, unyanked version on crates.io")
    return max(stable)[1]


def output(**values):
    with open(os.environ["GITHUB_OUTPUT"], "a") as file:
        for key, value in values.items():
            print(f"{key}={value if isinstance(value, str) else json.dumps(value, separators=(',', ':'))}", file=file)


def targets():
    """The pipeline's release targets and the GitHub runner that builds each."""
    spec = importlib.util.spec_from_file_location("pipeline", HERE.parent / ".github/crabx/pipeline.py")
    pipeline = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(pipeline)
    if {target for target, _ in pipeline.TARGETS} != set(NPM_TARGETS):
        raise ValueError("the pipeline's targets differ from crabx's npm packages")
    return pipeline.TARGETS


def tagged(tag):
    """The commits a tag in the repository names, directly or peeled."""
    ref = f"refs/tags/{tag}"
    out = subprocess.run(["git", "ls-remote", f"https://github.com/{REPO}", ref, ref + "^{}"], capture_output=True, text=True, check=True).stdout
    return {line.split()[0] for line in out.splitlines()}


def crate():
    """Decide whether this commit's crabx version is new to crates.io, and what to build first."""
    version = tomllib.loads((HERE / "Cargo.toml").read_text())["package"]["version"]
    if version in published():
        output(publish="false", builds={"include": []})
        return
    sha = os.environ["GITHUB_SHA"]
    # The pipeline releases under a bare vX.Y.Z tag naming this commit instead of
    # crabx-vX.Y.Z, where npm and the installers would never find it.
    if sha in tagged(f"v{version}"):
        raise SystemExit(f"v{version} tags this commit, so the pipeline would release crabx under it; delete that tag")
    item = {"name": "crabx", "version": version, "sha": sha, "path": "crabx"}
    output(publish="true", builds={"include": [{**item, "target": target, "os": runner} for target, runner in targets()]})


def package_names():
    """Each npm package crabx publishes, in publish order: the launcher's dependencies first."""
    return [platform_name(target) for target in NPM_TARGETS] + ["crabx"]


def platform_name(target):
    platform, arch = NPM_TARGETS[target]
    return f"@crabx/{platform}-{arch}{'-gnu' if platform == 'linux' else ''}"


def missing(version):
    """The packages npm lacks at this version."""
    return [name for name in package_names() if fetch(f"{NPM}/{name}/{version}") is None]


def npm_plan():
    """Pick the newest stable crates.io version, and whether npm lacks any of its packages."""
    version = newest_stable(published())
    output(version=version, publish="true" if missing(version) else "false")


def verify(archive, version):
    """Check the archive's attestation names the pipeline, rust2, and this crabx version."""
    result = subprocess.run(
        ["gh", "attestation", "verify", str(archive), "--repo", REPO, "--signer-workflow", SIGNER, "--format", "json"],
        check=True,
        capture_output=True,
        text=True,
    )
    for attestation in json.loads(result.stdout):
        parameters = attestation["verificationResult"]["statement"]["predicate"]["buildDefinition"]["externalParameters"]
        if parameters.get("crate") == "crabx" and parameters.get("version") == version:
            return
    raise ValueError(f"{archive.name}'s attestations name another crate or version")


def extract(archive, target, version, destination):
    """Copy the archive's crabx binary to destination, refusing any other layout."""
    filename = "crabx.exe" if "windows" in target else "crabx"
    wanted = f"crabx-{target}-v{version}/{filename}"
    with tarfile.open(archive, "r:gz") as tar:
        members = [member for member in tar.getmembers() if member.isfile()]
        if [member.name for member in members] != [wanted]:
            raise ValueError(f"{archive.name} holds {[m.name for m in members]}, not {wanted}")
        destination.mkdir(parents=True)
        with tar.extractfile(members[0]) as source, open(destination / filename, "wb") as binary:
            shutil.copyfileobj(source, binary)
    (destination / filename).chmod(0o755)


def manifest(name, version, description, **fields):
    return {
        "name": name,
        "version": version,
        "description": description,
        "repository": {"type": "git", "url": f"git+https://github.com/{REPO}.git", "directory": "crabx"},
        "license": "GPL-3.0-only",
        "publishConfig": {"access": "public"},
        **fields,
    }


def write(directory, data, files=("LICENSE",)):
    """Write the package's manifest and copy its files, readable by everyone wherever it installs."""
    (directory / "package.json").write_text(json.dumps(data, indent=2) + "\n")
    (directory / "package.json").chmod(0o644)
    for file in files:
        (directory / Path(file).name).write_bytes((HERE / file).read_bytes())
        (directory / Path(file).name).chmod(0o644)


def pack(version, archives, out):
    """Write each npm package for version into out, from the release archives in archives."""
    if not VERSION.fullmatch(version):
        raise ValueError(f"invalid version {version!r}")
    for target, (platform, arch) in NPM_TARGETS.items():
        name = platform_name(target)
        directory = out / name
        extract(archives / f"crabx-{target}-v{version}.tgz", target, version, directory / "bin")
        fields = {"os": [platform], "cpu": [arch]}
        if platform == "linux":
            fields["libc"] = ["glibc"]
        write(directory, manifest(name, version, f"The crabx binary for {target}", **fields, preferUnplugged=True))
    directory = out / "crabx"
    directory.mkdir()
    write(
        directory,
        manifest(
            "crabx",
            version,
            "Run a Rust CLI from its author's release, without compiling",
            bin={"crabx": "crabx.js"},
            optionalDependencies={platform_name(target): version for target in NPM_TARGETS},
            engines={"node": ">=18"},
        ),
        ("LICENSE", "README.md", "npm/crabx.js"),
    )


def download(version, directory):
    """Download and verify the release archive of crabx at version for every target."""
    subprocess.run(
        ["gh", "release", "download", f"crabx-v{version}", "--repo", REPO, "--dir", str(directory), "--pattern", f"crabx-*-v{version}.tgz"],
        check=True,
    )
    for target in NPM_TARGETS:
        archive = directory / f"crabx-{target}-v{version}.tgz"
        if not archive.is_file():
            raise ValueError(f"crabx-v{version} has no {archive.name}")
        verify(archive, version)


def publish(version):
    """Publish each package npm lacks at version, the launcher last."""
    with tempfile.TemporaryDirectory() as temp:
        temp = Path(temp)
        download(version, temp / "archives")
        pack(version, temp / "archives", temp / "packages")
        for name in missing(version):
            # An absolute path, since npm reads a relative `a/b` as a GitHub repository.
            subprocess.run(["npm", "publish", str(temp.resolve() / "packages" / name)], check=True)


def main():
    match sys.argv[1:]:
        case ["crate"]:
            crate()
        case ["npm-plan"]:
            npm_plan()
        case ["publish", version]:
            publish(version)
        case _:
            raise SystemExit("usage: release.py crate | npm-plan | publish VERSION")


if __name__ == "__main__":
    main()
