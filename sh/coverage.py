"""Check sh/dispatch.sh's map against what each module reads, as the nightly does.

usage: coverage.py MODULE...

Runs each module's checks and tests, `sh/dispatch.sh ci --only=MODULE`, under strace, and
records every path of the checkout that the module or anything it starts opens, stats, lists,
or executes. A diff of any one of them must select the module: dispatch is asked what that
diff alone selects, and the check fails on each path whose diff leaves the module out. A
directory stands for a file added to it, which changes what a listing of it reads.

Ignored paths, such as target directories, are no source a diff touches. The parked go, which
only its own diff selects by design, has nothing to check here. Its builds must run cold,
outside a compilation cache, since a cached build reads no source it would compile.
"""

import os
import re
import shutil
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

# The syscalls that name a path, and those that make a process, whose working directory a
# relative path in it resolves against.
SYSCALLS = "%file,fchdir,clone,clone3,fork,vfork"

# A directory file descriptor as `strace -y` decodes it, then the path relative to it.
AT = re.compile(r'(?:AT_FDCWD|-?\d+)<([^>]*)>, "((?:[^"\\]|\\.)*)"')
STRING = re.compile(r'"((?:[^"\\]|\\.)*)"')
CALL = re.compile(r"^(\w+)\((.*)\)\s+= (.*)$")
# A descriptor a call returned, decoded to the path it opened.
OPENED = re.compile(r"^\d+<([^>]*)>")
CLONED = re.compile(r"^(\d+)")

# The name a probe of a directory adds to it.
PROBE = "coverage-probe"


def unescape(text):
    """A path as strace quoted it: C escapes, and hex or octal bytes."""
    out = bytearray()
    i = 0
    raw = text.encode("utf-8", "surrogateescape")
    while i < len(raw):
        c = raw[i]
        if c != ord("\\"):
            out.append(c)
            i += 1
            continue
        nxt = chr(raw[i + 1])
        if nxt == "x":
            out.append(int(raw[i + 2:i + 4], 16))
            i += 4
        elif nxt.isdigit():
            j = i + 1
            while j < len(raw) and j < i + 4 and chr(raw[j]).isdigit():
                j += 1
            out.append(int(raw[i + 1:j], 8))
            i = j
        else:
            out.append(ord({"n": "\n", "t": "\t", "r": "\r", "v": "\v", "f": "\f"}.get(nxt, nxt)))
            i += 2
    return os.fsdecode(bytes(out))


def events(lines):
    """Each path-naming event of one process's trace, in order.

    ("cwd", path) is a working directory strace decoded, ("chdir", path) one the process
    changed to, ("clone", pid) a process it made, and ("path", path, absolute, opened) a path it
    named: `absolute` is whether it resolves without the working directory, and `opened` whether
    the call opened it, which is how a directory is listed rather than only looked at.
    """
    out = []
    for line in lines:
        call = CALL.match(line.rstrip("\n"))
        if not call:
            continue
        name, args, result = call.groups()
        ok = not result.startswith("-1")
        if name in ("clone", "clone3", "fork", "vfork"):
            child = CLONED.match(result)
            if ok and child:
                out.append(("clone", int(child.group(1))))
            continue
        if name == "fchdir":
            directory = re.match(r"\d+<([^>]*)>", args)
            if ok and directory:
                out.append(("cwd", directory.group(1)))
            continue
        at = AT.match(args)
        if at:
            base, path = at.group(1), unescape(at.group(2))
            if args.startswith("AT_FDCWD"):
                out.append(("cwd", base))
            out.append(("path", os.path.join(base, path), True, name.startswith("open")))
        else:
            string = STRING.search(args)
            if not string:
                continue
            path = unescape(string.group(1))
            if name == "chdir":
                if ok:
                    out.append(("chdir", path))
                continue
            out.append(("path", path, os.path.isabs(path), name in ("open", "creat")))
        opened = OPENED.match(result)
        if opened:
            out.append(("path", opened.group(1), True, True))
    return out


def resolve(traces, root):
    """Every absolute path the processes of `traces`, each pid's events, named, and whether
    any opened it.

    A process starts in its parent's working directory as it was when the parent made it;
    the first process, which no traced one made, starts in `root`. A process that named nothing
    and made none, such as a thread whose making strace saw only as `???`, has nothing to resolve.
    """
    traces = {pid: evs for pid, evs in traces.items() if evs}
    children = {child for evs in traces.values() for ev in evs if ev[0] == "clone" for child in [ev[1]]}
    first = [pid for pid in traces if pid not in children]
    if len(first) != 1:
        raise SystemExit(f"expected one first process in the trace, found {sorted(first)}")
    paths = {}
    start = {first[0]: str(root)}
    pending = [first[0]]
    while pending:
        pid = pending.pop()
        cwd = start[pid]
        for ev in traces.get(pid, []):
            if ev[0] == "cwd":
                cwd = ev[1]
            elif ev[0] == "chdir":
                cwd = os.path.normpath(os.path.join(cwd, ev[1]))
            elif ev[0] == "clone":
                start[ev[1]] = cwd
                pending.append(ev[1])
            else:
                path = os.path.normpath(ev[1] if ev[2] else os.path.join(cwd, ev[1]))
                paths[path] = paths.get(path, False) or ev[3]
    unreached = set(traces) - set(start)
    if unreached:
        raise SystemExit(f"no traced process made {sorted(unreached)}")
    return paths


def trace(module, root, scratch):
    """The absolute paths `module`'s checks and tests name, which must pass."""
    prefix = scratch / "trace"
    result = subprocess.run(
        ["strace", "-f", "-ff", "-qq", "-y", "--seccomp-bpf", "-e", f"trace={SYSCALLS}", "-o", str(prefix),
         "sh/dispatch.sh", "ci", f"--only={module}"],
        cwd=root)
    if result.returncode != 0:
        raise SystemExit(f"{module} failed under strace, so what it reads is unknown")
    traces = {}
    for file in scratch.glob("trace.*"):
        with open(file, encoding="utf-8", errors="surrogateescape") as lines:
            traces[int(file.suffix[1:])] = events(lines)
        file.unlink()
    return resolve(traces, root)


def sources(paths, root):
    """The paths of `paths`, each with whether it was opened, within the checkout, relative
    to it: but the ignored, git's own, and a directory only looked at, whose entries were
    not read.
    """
    inside = {}
    real = os.path.realpath(root)

    def add(path, opened):
        inside[path] = inside.get(path, False) or opened

    for path, opened in paths.items():
        for base in {str(root), real}:
            if path == base:
                add(".", opened)
            elif path.startswith(base + os.sep):
                add(path[len(base) + 1:], opened)
    # A path through a link in the checkout is the link's and its target's, which git names
    # apart: the link itself is what a diff changes.
    for path, opened in list(inside.items()):
        parts = path.split("/")
        for n in range(1, len(parts)):
            link = "/".join(parts[:n])
            if (root / link).is_symlink():
                del inside[path]
                add(link, False)
                target = os.path.relpath(os.path.realpath(root / path), real)
                if not target.startswith(".."):
                    add(target, opened)
                break
    kept = {p for p, opened in inside.items()
            if p != ".git" and not p.startswith(".git/") and (opened or not (root / p).is_dir())}
    ignored = subprocess.run(["git", "check-ignore", "--stdin"], cwd=root, input="\n".join(sorted(kept)),
                             capture_output=True, text=True)
    if ignored.returncode not in (0, 1):
        raise SystemExit(f"git check-ignore: {ignored.stderr}")
    return kept - set(ignored.stdout.splitlines())


def probe(path, root):
    """The changed path a diff touching `path` lists: a file added to it, for a directory."""
    if (root / path).is_dir():
        return PROBE if path == "." else f"{path}/{PROBE}"
    return path


def selects(change, root):
    """The modules dispatch selects for a diff of `change` alone."""
    result = subprocess.run(["sh/dispatch.sh", "ci", "--paths", "--query"], cwd=root, input=change + "\n",
                            capture_output=True, text=True)
    if result.returncode != 0:
        raise SystemExit(f"dispatch failed for {change}: {result.stderr}")
    return set(result.stdout.split())


def holes(module, paths, root, select):
    """Each of `paths` whose diff does not select `module`, with the change it was asked for."""
    changes = {path: probe(path, root) for path in paths}
    with ThreadPoolExecutor(max_workers=os.cpu_count() or 1) as pool:
        selected = dict(zip(changes, pool.map(lambda change: select(change, root), changes.values())))
    return sorted((path, changes[path]) for path in paths if module not in selected[path])


def main():
    modules = sys.argv[1:]
    if not modules or any(m.startswith("-") for m in modules):
        raise SystemExit(__doc__.split("\n\n")[1])
    if "go" in modules:
        raise SystemExit("go is parked: only its own diff selects it, by design")
    if shutil.which("strace") is None:
        raise SystemExit("strace is not on PATH")
    root = Path(subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True,
                               check=True).stdout.strip())
    failed = False
    for module in modules:
        with tempfile.TemporaryDirectory() as scratch:
            paths = sources(trace(module, root, Path(scratch)), root)
        found = holes(module, paths, root, selects)
        print(f"{module}: reads {len(paths)} paths of the checkout, {len(found)} of whose diffs skip it")
        for path, change in found:
            failed = True
            print(f"  {path}" + ("" if path == change else f" (a diff adding {change})"))
    if failed:
        raise SystemExit("a diff can skip a module that reads it: widen that module's pattern in sh/dispatch.sh")


if __name__ == "__main__":
    main()
