#!/usr/bin/env bash
# Run `just check`, `just test`, or `just ci` over the modules the branch changed
# since BASE: the queue's explicit argument, the fixer's QUEST_LAND_BASE, or origin/main.
# --all runs every module, and --only=MODULE,... the named ones, as a CI job given its
# share of a selection does. --query prints the selected modules instead.
set -euo pipefail

usage="usage: sh/dispatch.sh check|test|ci [BASE|--all|--only=MODULE,...] [--query]"
action=${1:-}
base=${2:-}
query=${3:-}
# BASE is optional before --query.
if [[ "$base" == --query && $# -eq 2 ]]; then
    base='' query=--query
fi
if [[ $# -gt 3 || ! "$action" =~ ^(check|test|ci)$ || (-n "$query" && "$query" != --query) ]]; then
    echo "$usage" >&2
    exit 2
fi

cd "$(git rev-parse --show-toplevel)"

# The impact map: a module runs when a changed path matches its pattern. An
# empty pattern runs on every diff.
declare -A scope=(
    [lint]=''
    [quest]='^quest/'
    [sh]='^sh/dispatch\.test\.py$'
    # The driver and ffi link ts's toolchain, and both of the others build on
    # the driver's front end.
    [driver]='^(driver/|ts/rust-toolchain\.toml$)'
    [ffi]='^(ffi/|driver/|ts/rust-toolchain\.toml$)'
    # The parked go translator runs on its own diff only; its shared inputs
    # (the driver, ts's toolchain and codec fixture) reach it in the nightly's
    # `just gate`, whose failure issue reports a break.
    [go]='^go/'
    [ts]='^(ts|driver)/'
)
# Cheapest first, so within one run a lint fails before the builds start.
modules=(lint quest sh driver ffi go ts)

# Paths that only the repository-wide lints read.
lints='^(bench/|\.agents/|\.claude/|\.codex/|[^/]+\.md$|LICENSE-|\.(editorconfig|gitattributes|gitignore|taplo\.toml)$)'

selected=()
if [[ "$base" == --all ]]; then
    selected=("${modules[@]}")
elif [[ "$base" == --only=* ]]; then
    only=${base#--only=}
    if [[ ! "$only" =~ ^[a-z]+(,[a-z]+)*$ ]]; then
        echo "error: --only takes a comma-separated list of: ${modules[*]}" >&2
        exit 2
    fi
    for module in ${only//,/ }; do
        if [[ -z "${scope[$module]+set}" ]]; then
            echo "error: no module named '$module'; the modules are ${modules[*]}" >&2
            exit 2
        fi
    done
    # The map's order, so a lint in the list still runs before its builds.
    for module in "${modules[@]}"; do
        [[ ",$only," != *",$module,"* ]] || selected+=("$module")
    done
else
    base=${base:-${QUEST_LAND_BASE:-origin/main}}
    merge_base=$(git merge-base "$base" HEAD) || {
        echo "error: cannot resolve a merge base with $base; pass BASE, or run \`just gate\`" >&2
        exit 1
    }
    echo "$action: changes since $base" >&2
    # Untracked files count, so a new file selects its module, and a rename
    # lists both sides, since a reader of the old path is affected too.
    changed=$({
        git diff --name-only --no-renames "$merge_base"
        git ls-files --others --exclude-standard
    } | sort -u)

    # A path no module claims is a shared input, such as the flake, a
    # toolchain, the root justfile, .github, or this map, so it selects
    # every module but the parked go, which only its own diff selects. A new
    # project selects everything until it is mapped.
    claimed=$lints
    for module in "${modules[@]}"; do
        [[ -z "${scope[$module]}" ]] || claimed+="|${scope[$module]}"
    done
    shared=$(grep -vE "$claimed" <<<"$changed" | grep . || true)
    if [[ -n "$shared" ]]; then
        echo "$action: running everything but the parked go for $(paste -sd ' ' <<<"$shared")" >&2
        # Everything but the parked go, which only its own diff selects.
        for module in "${modules[@]}"; do
            if [[ "$module" != go ]] || grep -qE "${scope[go]}" <<<"$changed"; then
                selected+=("$module")
            fi
        done
    else
        for module in "${modules[@]}"; do
            pattern=${scope[$module]}
            if [[ -z "$pattern" ]] || grep -qE "$pattern" <<<"$changed"; then
                selected+=("$module")
            fi
        done
    fi
fi

if [[ -n "$query" ]]; then
    printf '%s\n' "${selected[@]}"
    exit
fi

scripts=(bench/*/*.sh ffi/release/*.sh sh/*.sh)

# One function per module and phase; a module skips a phase it has none for.
check_lint() {
    actionlint
    shellcheck "${scripts[@]}"
    shfmt -d "${scripts[@]}"
    taplo format --check
    nixfmt --check flake.nix
}
check_quest() { quest check; }
check_driver() { just driver check; }
check_ffi() { just ffi check; }
check_go() { just go check; }
check_ts() { just ts check; }
test_sh() { python3 -I sh/dispatch.test.py; }
test_driver() { just driver test; }
# moq's export at the pin, cut down to C, and every binding's rendered API, which the
# surface files lock on x86_64 Linux, the queue's runners, before the slower test build.
test_ffi() {
    python3 -I ffi/release/test_package.py
    python3 -I ffi/surface/test_surface.py
    if [[ "$(uname -sm)" == "Linux x86_64" ]]; then
        # One invocation, so the export both read runs once: moq-c's checkout comes first.
        just ffi::moq-c '' ffi::surface --check
    else
        echo "test: the surface files are x86_64 Linux's exports, which the queue checks" >&2
        just ffi moq-c
    fi
    just ffi test
}
test_go() { just go test; }
# moq-net's translation at the pin, its first consumer, before the slower test build.
test_ts() {
    just ts moq
    just ts test
}

# Every check runs before any test, so within one run a lint fails before the test builds.
case "$action" in
    check | test) phases=("$action") ;;
    ci) phases=(check test) ;;
esac
for phase in "${phases[@]}"; do
    for module in "${selected[@]}"; do
        declare -F "${phase}_$module" >/dev/null || continue
        echo "==> $phase $module" >&2
        "${phase}_$module"
    done
done
