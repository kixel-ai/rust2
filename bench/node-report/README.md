# The Node report's harness

What [quest/m3/node.md](../../quest/m3/node.md)'s report ran: bench/ffi's cases on moq's
stand-in from Node 26.13.1 three ways. `wasm` and `wasm-tf` are ffi/moq/bench/export.ts
over the web target, the second under `--no-liftoff`. `nf` is node.ts over node:ffi and
nf.c's pointer-only twins. `napi` is node.ts over addon.c, a Node-API addon.

```bash
cd ffi && just moq-export && just moq-web && just ts-program && cd ..
NODE=$(nix build --no-link --print-out-paths --inputs-from . nixpkgs#nodejs_26)/bin/node \
  quest gate bench/node-report/run.sh 5 .tmp/node-report
python3 bench/node-report/summary.py .tmp/node-report/rounds.tsv
```

`rounds.tsv` holds the five rounds the report quotes, and `rounds-bun.tsv` and
`rounds-deno.tsv` one round of `napi` on Bun 1.4.4 and Deno 2.9.8 (`bun node.ts napi <dir>`,
`deno run -A node.ts napi <dir>`). `probe/` holds the two threading probes: a callback
from another thread through node:ffi (`thread.c`), which aborts, and through Node-API,
served on the waiting caller's thread (`serve.c`).
