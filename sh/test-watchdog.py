#!/usr/bin/env python3
"""Run a command that runs libtest binaries, such as `cargo test`, and kill it when one
test runs past its bound, naming that test.

    sh/test-watchdog.py [--bound 15m] [--bound PATTERN=2h ...] -- cargo test ...

Each test binary keeps running all of its tests in one process, so a target where a
process launch is dear (the iOS simulator, Intel SDE, an adb device) pays it once per
binary rather than once per test. libtest prints `test NAME has been running for over 60
seconds` for every test still running after 60 seconds of its own, and `test NAME ...
ok` when one ends, so the watchdog times each test from its own start without any flag
in the test binary. A binary run with one test thread prints no such line, but prints
`test NAME ... ` as the test starts, which starts the clock instead.

A `PATTERN=DURATION` bound applies to the tests whose names match the regular
expression; the first match wins, and the plain bound covers the rest. Silence while no
test is known to be running, such as a binary hung before its first test or after its
last, or a harness that prints no such lines, is bounded by the plain bound too, and
names the binary cargo last started.

On a bound, the command's process group gets SIGTERM, then SIGKILL after 10 seconds, and
the watchdog exits 124. The grace matters where the test runs outside the group: `simctl
spawn` passes a SIGTERM on to its child in the simulator, but dies to a SIGKILL and leaves
the child running.
"""

import os
import re
import select
import signal
import subprocess
import sys
import time

UNITS = {"s": 1, "m": 60, "h": 3600}
# libtest prints the warning once, after this long.
WARN_AFTER = 60
# Between SIGTERM and SIGKILL, as nextest's default.
GRACE = 10

RUNNING = re.compile(r"^test (\S+) has been running for over 60 seconds$")
STARTED = re.compile(r"^test (\S+) \.\.\. $")
ENDED = re.compile(r"^test (\S+) \.\.\. (ok|FAILED|ignored)")
BINARY = re.compile(r"^\s+(Running|Doc-tests) (.+)$")
RESULT = re.compile(r"^test result: ")
# Colour, such as cargo's under CARGO_TERM_COLOR=always, would hide a line from the
# patterns.
ESCAPE = re.compile(r"\x1b\[[0-9;]*m")


def duration(text):
    match = re.fullmatch(r"(\d+)([smh])", text)
    if not match:
        sys.exit(f"test-watchdog: a bound is a whole number of s, m, or h, not {text!r}")
    seconds = int(match.group(1)) * UNITS[match.group(2)]
    if seconds <= WARN_AFTER:
        sys.exit(f"test-watchdog: a bound must exceed libtest's {WARN_AFTER} s warning, not {text!r}")
    return seconds


def parse(argv):
    if "--" not in argv:
        sys.exit("usage: test-watchdog.py [--bound DURATION] [--bound PATTERN=DURATION] -- COMMAND...")
    split = argv.index("--")
    options, command = argv[:split], argv[split + 1 :]
    default, overrides = 15 * 60, []
    while options:
        if options[0] != "--bound" or len(options) < 2:
            sys.exit(f"test-watchdog: unknown option {options[0]!r}")
        value = options[1]
        options = options[2:]
        if "=" in value:
            pattern, _, text = value.rpartition("=")
            overrides.append((re.compile(pattern), duration(text)))
        else:
            default = duration(value)
    if not command:
        sys.exit("test-watchdog: no command after --")
    return default, overrides, command


def main():
    default, overrides, command = parse(sys.argv[1:])

    def bound(name):
        return next((s for p, s in overrides if p.search(name)), default)

    # stdout and stderr share one pipe, so cargo's `Running` lines and the binary's
    # libtest lines arrive in the order they were written.
    proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)
    # Forward a cancel, such as a CI step's or Ctrl-C, to the whole group.
    for sig in (signal.SIGINT, signal.SIGTERM):
        signal.signal(sig, lambda s, _: os.killpg(proc.pid, s))

    running = {}  # test name -> monotonic start
    binary = "(no binary yet)"
    last_output = time.monotonic()
    partial = b""
    fd = proc.stdout.fileno()
    out = sys.stdout.buffer

    def kill(reason):
        out.write(f"\ntest-watchdog: {reason}; killing the run\n".encode())
        out.flush()
        os.killpg(proc.pid, signal.SIGTERM)
        proc.wait()
        deadline = time.monotonic() + GRACE
        while time.monotonic() < deadline:
            try:
                os.killpg(proc.pid, 0)
            except ProcessLookupError:
                break
            time.sleep(0.1)
        else:
            os.killpg(proc.pid, signal.SIGKILL)
        sys.exit(124)

    def line(text, now):
        nonlocal binary
        if m := BINARY.match(text):
            binary = m.group(2)
            running.clear()
        elif m := RUNNING.match(text):
            running.setdefault(m.group(1), now - WARN_AFTER)
        elif m := ENDED.match(text):
            running.pop(m.group(1), None)
        elif RESULT.match(text):
            running.clear()

    while True:
        ready, _, _ = select.select([fd], [], [], 1.0)
        now = time.monotonic()
        if ready:
            chunk = os.read(fd, 65536)
            if not chunk:
                break
            out.write(chunk)
            out.flush()
            last_output = now
            lines = (partial + chunk).split(b"\n")
            partial = lines.pop()
            for raw in lines:
                line(ESCAPE.sub("", raw.decode(errors="replace")).rstrip("\r"), now)
            # A one-thread libtest prints the start of a test's line and ends it only
            # when the test does.
            if m := STARTED.match(ESCAPE.sub("", partial.decode(errors="replace"))):
                running.setdefault(m.group(1), now)
        for name, start in running.items():
            if now - start > bound(name):
                kill(f"test {name} in {binary} ran past its {bound(name)} s bound")
        if not running and now - last_output > default:
            kill(f"no output for {default} s and no test running, in {binary}")

    sys.exit(proc.wait())


if __name__ == "__main__":
    main()
