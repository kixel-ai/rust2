"""Prints run.sh's rounds as one table: each case's median wall time per route, ns per call
(MB/s for the mbps rows), with the load's range."""

import collections
import statistics
import sys

cells = collections.defaultdict(list)
loads = []
for line in open(sys.argv[1]):
    route, _, case, wall, _cpu, load = line.rstrip("\n").split("\t")
    cells[case, route].append(float(wall))
    loads.append(float(load))
cases = list(dict.fromkeys(case for case, _ in cells))
routes = ["wasm", "nf", "napi"]
print(f"{len(cells[cases[0], routes[0]])} rounds, load {min(loads):.1f} to {max(loads):.1f}")
print("| case | " + " | ".join(routes) + " |")
print("|---" * (len(routes) + 1) + "|")
for case in cases:
    row = [f"{statistics.median(cells[case, r]):,.1f}" if cells[case, r] else "" for r in routes]
    print(f"| {case} | " + " | ".join(row) + " |")
