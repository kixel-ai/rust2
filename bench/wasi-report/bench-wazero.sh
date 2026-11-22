#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for i in 1 2 3 4 5; do
	echo "## round $i load $(cut -d' ' -f1 /proc/loadavg)"
	./wazero/wb-wazero target-core/wasm32-wasip1/release/wb.wasm v1.12.0 2>/dev/null
done
