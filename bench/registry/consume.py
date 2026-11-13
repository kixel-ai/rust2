"""Run an installed wheel from an isolated consumer directory against native Rust."""
from pathlib import Path
import subprocess
import sys

artifacts = Path(sys.argv[1]).resolve()
python = Path(sys.executable)
for case in [None, "futures", "owned", "values", "calls"]:
    command = [str(python), "-I", "-X", "dev", "-W", "error", str(artifacts / "consumer.py")]
    if case:
        command.append(case)
    output = subprocess.run(command, check=True, capture_output=True)
    if case is None and output.stdout != (artifacts / "native.txt").read_bytes():
        raise SystemExit("installed wheel differs from native Rust")
print("installed wheel: transcript matches Rust; futures, ownership, values and calls pass")
