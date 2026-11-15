"""Run a command and print its wall seconds, CPU seconds, and peak RSS in bytes, tab-separated:
CPU over every process it waited for, and the peak of the largest one, as getrusage reports
them for children. The command's own output goes to stderr, so stdout holds only the line.

Usage: measure.py <command> [args...]
"""

import resource
import subprocess
import sys
import time

if len(sys.argv) < 2:
	sys.exit(__doc__)
start = time.monotonic()
code = subprocess.run(sys.argv[1:], stdout=sys.stderr).returncode
wall = time.monotonic() - start
if code != 0:
	sys.exit(f"measure: {sys.argv[1:]} exited {code}")
usage = resource.getrusage(resource.RUSAGE_CHILDREN)
# Linux reports ru_maxrss in KiB, macOS in bytes.
rss = usage.ru_maxrss if sys.platform == "darwin" else usage.ru_maxrss * 1024
print(f"{wall:.3f}\t{usage.ru_utime + usage.ru_stime:.3f}\t{rss}")
