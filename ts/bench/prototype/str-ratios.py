#!/usr/bin/env python3
"""Per-run median ratios to `translated`, summarized over runs: median (min-max). Prototype
only, for moq-dev/rust2#574. Each run's ratio is the median over its rounds of the route's
time over `translated`'s in the same round.

usage: str-ratios.py <route>:<report.json>,... ...
"""
import json, math, statistics, sys

ENG = ["Node", "Bun", "Chromium", "Firefox"]


def engine(name):
    return "Firefox" if "Firefox" in name else "Chromium" if "Chrome" in name else name.split(" ")[0]


def geomean(xs):
    return math.exp(sum(map(math.log, xs)) / len(xs))


for spec in sys.argv[1:]:
    route, files = spec.split(":")
    per = {}
    names = []
    for f in files.split(","):
        for e in json.load(open(f))["engines"]:
            eng = engine(e["engine"])
            by = {}
            for t in e["timings"]:
                by.setdefault(f"{t['workload']} {t['op']}", {})[t["route"]] = t["ns"]
            rest, aa = [], []
            for name, rs in by.items():
                if route not in rs:
                    continue
                m = statistics.median(x / b for x, b in zip(rs[route], rs["translated"]))
                a = statistics.median(x / b for x, b in zip(rs["A/A"], rs["translated"]))
                aa.append(a)
                if name.startswith(("str/", "fmt/")):
                    if name not in names:
                        names.append(name)
                    per.setdefault((eng, name), []).append(m)
                    per.setdefault((eng, f"A/A {name}"), []).append(a)
                else:
                    rest.append(m)
            per.setdefault((eng, "every other workload, geomean"), []).append(geomean(rest))
            per.setdefault((eng, "A/A, geomean of all"), []).append(geomean(aa))
    print(f"### {route} ÷ translated ({len(files.split(','))} runs)\n\n| workload | " + " | ".join(ENG) + " |\n|---|" + "---|" * len(ENG))
    for name in names + ["every other workload, geomean", "A/A, geomean of all"]:
        cells = []
        for e in ENG:
            xs = per.get((e, name), [])
            cells.append(f"{statistics.median(xs):.2f} ({min(xs):.2f}-{max(xs):.2f})" if xs else "-")
        print(f"| {name} | " + " | ".join(cells) + " |")
    print()
