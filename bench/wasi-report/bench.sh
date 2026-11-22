#!/usr/bin/env bash
# Five rounds of the host benchmark, each with the 1-minute load average before it.
set -euo pipefail
cd "$(dirname "$0")"
for i in 1 2 3 4 5; do
	echo "## round $i load $(cut -d' ' -f1 /proc/loadavg)"
	./target-host/release/wb-host target-core/wasm32-wasip1/release/wb.wasm target-p2/wasm32-wasip2/release/wb_guest.wasm target-p3/wasm32-wasip3/release/wb_guest.wasm 2>/dev/null
done
