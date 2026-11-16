"""Run an installed wheel from an isolated consumer directory against native Rust."""
from pathlib import Path
import difflib
import os
import subprocess
import sys

artifacts = Path(sys.argv[1]).resolve()
python = Path(os.path.abspath(sys.executable))
for case in [None, "futures", "owned", "values", "calls"]:
    command = [str(python), "-I", "-X", "dev", "-W", "error", str(artifacts / "consumer.py")]
    if case:
        command.append(case)
    output = subprocess.run(command, capture_output=True)
    if output.returncode:
        sys.stderr.buffer.write(output.stderr)
        raise SystemExit(output.returncode)
    if case is None:
        native = (artifacts / "native.txt").read_bytes()
        if output.stdout != native:
            sys.stderr.buffer.writelines(difflib.diff_bytes(
                difflib.unified_diff, native.splitlines(keepends=True),
                output.stdout.splitlines(keepends=True), fromfile=b"native Rust", tofile=b"installed wheel"))
            raise SystemExit("installed wheel differs from native Rust")
print("installed wheel: transcript matches Rust; futures, ownership, values and calls pass")
