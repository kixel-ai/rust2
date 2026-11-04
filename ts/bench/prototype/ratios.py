#!/usr/bin/env python3
"""Per-run median ratios to `translated`, summarized over runs: median (min-max).
Prototype only, for moq-dev/rust2#555. Each run's ratio is the median over its rounds
of the route's time over `translated`'s in the same round.

usage: ratios.py <route>:<report.json>,... ...
"""
import json, math, statistics, sys

KEY = ["push/stack push", "push/vec push", "push/deque push", "order/binary-heap order", "order/sort order"]
ENG = ["Node", "Bun", "Chromium", "Mozilla Firefox"]
for spec in sys.argv[1:]:
    route, files = spec.split(":")
    per = {}  # (engine, name) -> [run median]
    aa = {}
    for f in files.split(","):
        for e in json.load(open(f))["engines"]:
            eng = e["engine"].split(" ")[0] if not e["engine"].startswith("Mozilla") else "Mozilla Firefox"
            by = {}
            for t in e["timings"]:
                by.setdefault(f"{t['workload']} {t['op']}", {})[t["route"]] = t["ns"]
            fams = {}
            for name, rs in by.items():
                if route not in rs:
                    continue
                m = statistics.median(x / b for x, b in zip(rs[route], rs["translated"]))
                per.setdefault((eng, name), []).append(m)
                if "A/A" in rs:
                    aa.setdefault((eng, name), []).append(statistics.median(x / b for x, b in zip(rs["A/A"], rs["translated"])))
                if not name.startswith("push/") and not name.startswith("depth"):
                    fams.setdefault(name.split("/")[0], []).append(m)
            # every workload but push/: geomean of the run's medians
            allm = [x for xs in fams.values() for x in xs]
            per.setdefault((eng, "rest geomean"), []).append(math.exp(sum(map(math.log, allm)) / len(allm)))
            per.setdefault((eng, "rest max family"), []).append(max(math.exp(sum(map(math.log, xs)) / len(xs)) for xs in fams.values()))
    print(f"### {route} ({len(files.split(','))} runs)\n\n| workload | " + " | ".join(e.split()[-1] for e in ENG) + " |\n|---|" + "---|" * len(ENG))
    for name in KEY + ["rest geomean", "rest max family"]:
        cells = []
        for e in ENG:
            xs = per.get((e, name), [])
            cells.append(f"{statistics.median(xs):.2f} ({min(xs):.2f}-{max(xs):.2f})" if xs else "-")
        print(f"| {name} | " + " | ".join(cells) + " |")
    cells = []
    for e in ENG:
        xs = aa.get((e, "push/stack push"), [])
        cells.append(f"{statistics.median(xs):.2f} ({min(xs):.2f}-{max(xs):.2f})")
    print("| A/A push/stack | " + " | ".join(cells) + " |\n")
