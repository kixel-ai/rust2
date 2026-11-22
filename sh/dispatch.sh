#!/usr/bin/env bash
# Run `just check`, `just test`, or `just ci` over the modules the branch changed
# since BASE: the queue's explicit argument, the fixer's QUEST_LAND_BASE, or origin/main.
# --all runs every module. --query prints the selected modules instead.
# --query-ffi and --query-ts print the sub-scopes selected inside ffi and ts.
# --only MODULE runs only that module (when selected).
set -euo pipefail

usage="usage: sh/dispatch.sh check|test|ci [BASE|--all] [--query|--query-ffi|--query-ts|--only MODULE]"
action=${1:-}
if [[ ! "$action" =~ ^(check|test|ci)$ ]]; then
    echo "$usage" >&2
    exit 2
fi
shift

base=''
query=''
only=''
while [[ $# -gt 0 ]]; do
    case "$1" in
        --query | --query-ffi | --query-ts)
            if [[ -n "$query" || -n "$only" ]]; then
                echo "$usage" >&2
                exit 2
            fi
            query=$1
            shift
            ;;
        --only)
            if [[ -n "$query" || -n "$only" || $# -lt 2 ]]; then
                echo "$usage" >&2
                exit 2
            fi
            only=$2
            shift 2
            ;;
        --all)
            if [[ -n "$base" ]]; then
                echo "$usage" >&2
                exit 2
            fi
            base=--all
            shift
            ;;
        *)
            if [[ -n "$base" ]]; then
                echo "$usage" >&2
                exit 2
            fi
            base=$1
            shift
            ;;
    esac
done

cd "$(git rev-parse --show-toplevel)"

# The impact map: a module runs when a changed path matches its pattern. An
# empty pattern runs on every diff.
declare -A scope=(
    [lint]=''
    [quest]='^quest/'
    [sh]='^sh/dispatch\.test\.py$'
    [crabx]='^crabx/'
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
# Cheapest first, so a lint fails before the builds start.
modules=(lint quest sh crabx driver ffi go ts)

# Paths that only the repository-wide lints (and .github/crabx's pipeline test) read.
lints='^(bench/|\.agents/|\.claude/|\.codex/|\.github/|[^/]+\.md$|LICENSE-|\.(editorconfig|gitattributes|gitignore|taplo\.toml)$)'

changed=''
shared=''
selected=()
if [[ "$base" == --all ]]; then
    selected=("${modules[@]}")
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
    # toolchain, the root justfile, or this map, so it selects every module
    # but the parked go, which only its own diff selects. A new project
    # selects everything until it is mapped.
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

has_module() {
    local want=$1 m
    for m in "${selected[@]}"; do
        [[ "$m" == "$want" ]] && return 0
    done
    return 1
}

add_unique() {
    local -n arr=$1
    local val=$2 item
    for item in "${arr[@]}"; do
        [[ "$item" == "$val" ]] && return 0
    done
    arr+=("$val")
}

# Narrow `ffi` tests to the affected language(s), loom binary, or tool when no
# shared/core ffi path changed.
ffi_scopes=()
if has_module ffi; then
    if [[ "$base" == --all || -n "$shared" ]] || grep -qE '^(driver/|ts/rust-toolchain\.toml$)' <<<"$changed"; then
        ffi_scopes=(all)
    else
        while IFS= read -r path; do
            [[ "$path" == ffi/* ]] || continue
            case "$path" in
                ffi/release/*) add_unique ffi_scopes release ;;
                ffi/surface/surface.py | ffi/surface/test_surface.py) add_unique ffi_scopes surface-tool ;;
                ffi/tests/borrow.rs) add_unique ffi_scopes loom-borrow ;;
                ffi/tests/future.rs) add_unique ffi_scopes loom-future ;;
                ffi/tests/hold.rs) add_unique ffi_scopes loom-hold ;;
                ffi/tests/thread.rs) add_unique ffi_scopes loom-thread ;;
                ffi/tests/loom/*)
                    add_unique ffi_scopes loom-future
                    add_unique ffi_scopes loom-thread
                    ;;
                ffi/tests/api.c | ffi/tests/protocol.c | ffi/moq/moq.c | ffi/surface/*/c.h)
                    add_unique ffi_scopes lang-c
                    ;;
                ffi/src/cpp.rs | ffi/runtime/rt.hpp | ffi/tests/api.cpp | ffi/tests/api_strict.cpp | ffi/tests/protocol.cpp | ffi/moq/moq.cpp | ffi/surface/*/cpp.hpp)
                    add_unique ffi_scopes lang-cpp
                    ;;
                ffi/src/kotlin.rs | ffi/runtime/rt.kt | ffi/runtime/types.kt | ffi/runtime/FastNative.kt | ffi/runtime/jni.rs | ffi/tests/api.kt | ffi/tests/protocol.kt | ffi/tests/Caller.java | ffi/moq/moq.kt | ffi/surface/*/kotlin.kt)
                    add_unique ffi_scopes lang-kotlin
                    ;;
                ffi/src/python.rs | ffi/runtime/rt.py | ffi/runtime/native.py | ffi/runtime/python.rs | ffi/tests/api.py | ffi/tests/api_types.py | ffi/tests/bare_builtins.py | ffi/tests/protocol.py | ffi/tests/pyright.json | ffi/moq/moq.py | ffi/surface/*/python.pyi)
                    add_unique ffi_scopes lang-python
                    ;;
                ffi/src/swift.rs | ffi/runtime/rt.swift | ffi/tests/api.swift | ffi/tests/protocol.swift | ffi/moq/moq.swift | ffi/surface/*/swift.swift)
                    add_unique ffi_scopes lang-swift
                    ;;
                ffi/src/go.rs | ffi/runtime/rt.go | ffi/runtime/ffi_cgo.go | ffi/tests/api.go | ffi/tests/protocol.go | ffi/moq/moq.go | ffi/surface/*/go.txt)
                    add_unique ffi_scopes lang-go
                    ;;
                ffi/src/dart.rs | ffi/runtime/rt.dart | ffi/runtime/hook.dart | ffi/runtime/dart.rs | ffi/tests/api.dart | ffi/tests/protocol.dart | ffi/tests/dart_lints.yaml | ffi/moq/moq.dart | ffi/surface/*/dart.dart)
                    add_unique ffi_scopes lang-dart
                    ;;
                ffi/src/web.rs | ffi/src/js.rs | ffi/src/wasm.rs | ffi/runtime/rt.ts | ffi/runtime/web.rs | ffi/tests/api.ts | ffi/tests/dispose.ts | ffi/tests/browser.ts | ffi/tests/protocol.ts | ffi/surface/*/web.d.ts)
                    add_unique ffi_scopes lang-web
                    ;;
                ffi/README.md | ffi/FACADE.md | ffi/moq/bench/* | ffi/corpus/*)
                    add_unique ffi_scopes docs
                    ;;
                *)
                    ffi_scopes=(all)
                    break
                    ;;
            esac
        done <<<"$changed"
        [[ "${#ffi_scopes[@]}" -gt 0 ]] || ffi_scopes=(all)
    fi
fi

# Narrow `ts` tests to specific `lang/*` fixtures when only those fixtures changed.
ts_scopes=()
if has_module ts; then
    if [[ "$base" == --all || -n "$shared" ]] || grep -qE '^(driver/|ts/rust-toolchain\.toml$)' <<<"$changed"; then
        ts_scopes=(all)
    else
        while IFS= read -r path; do
            [[ "$path" == ts/* ]] || continue
            case "$path" in
                ts/fixtures/lang/*/*)
                    rest=${path#ts/fixtures/lang/}
                    fixture=${rest%%/*}
                    add_unique ts_scopes "lang-$fixture"
                    ;;
                ts/README.md | ts/bench/*)
                    add_unique ts_scopes docs
                    ;;
                *)
                    ts_scopes=(all)
                    break
                    ;;
            esac
        done <<<"$changed"
        [[ "${#ts_scopes[@]}" -gt 0 ]] || ts_scopes=(all)
    fi
fi

case "$query" in
    --query)
        printf '%s\n' "${selected[@]}"
        exit
        ;;
    --query-ffi)
        [[ "${#ffi_scopes[@]}" -eq 0 ]] || printf '%s\n' "${ffi_scopes[@]}"
        exit
        ;;
    --query-ts)
        [[ "${#ts_scopes[@]}" -eq 0 ]] || printf '%s\n' "${ts_scopes[@]}"
        exit
        ;;
esac

if [[ -n "$only" ]]; then
    if has_module "$only"; then
        selected=("$only")
    else
        selected=()
    fi
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
check_crabx() { just crabx check; }
check_driver() { just driver check; }
check_ffi() { just ffi check; }
check_go() { just go check; }
check_ts() { just ts check; }
test_lint() { python3 -I .github/crabx/test_pipeline.py; }
test_sh() { python3 -I sh/dispatch.test.py; }
test_crabx() { just crabx test; }
test_driver() { just driver test; }
# moq's export at the pin, cut down to C, and every binding's rendered API, which the
# surface files lock on x86_64 Linux, before the slower test build; narrowed to the
# affected languages or tools when only those changed.
test_ffi() {
    if [[ "${ffi_scopes[0]:-all}" == all ]]; then
        python3 -I ffi/release/test_package.py
        python3 -I ffi/surface/test_surface.py
        if [[ "$(uname -sm)" == "Linux x86_64" ]]; then
            # One invocation, so the export both read runs once: moq-c's checkout comes first.
            just ffi::moq-c '' ffi::surface --check
        else
            echo "test: the surface files are x86_64 Linux's exports, which CI checks" >&2
            just ffi moq-c
        fi
        just ffi test
        return
    fi

    echo "test ffi: scoped to ${ffi_scopes[*]}" >&2
    local need_moq_c=false
    local ran_full_surface=false
    local surface_files=()
    local lib_filters=()
    local fixture_filters=()
    local loom_targets=()
    local s
    for s in "${ffi_scopes[@]}"; do
        case "$s" in
            release)
                python3 -I ffi/release/test_package.py
                ;;
            surface-tool)
                python3 -I ffi/surface/test_surface.py
                if [[ "$(uname -sm)" == "Linux x86_64" ]]; then
                    just ffi surface --check
                    ran_full_surface=true
                fi
                ;;
            loom-borrow) loom_targets+=(--test borrow) ;;
            loom-future) loom_targets+=(--test future) ;;
            loom-hold) loom_targets+=(--test hold) ;;
            loom-thread) loom_targets+=(--test thread) ;;
            lang-c)
                need_moq_c=true
                surface_files+=(c.h)
                fixture_filters+=(api_c api_protocol shapes::gate::c)
                ;;
            lang-cpp)
                surface_files+=(cpp.hpp)
                lib_filters+=(cpp)
                fixture_filters+=(cpp)
                ;;
            lang-kotlin)
                surface_files+=(kotlin.kt)
                lib_filters+=(kotlin)
                fixture_filters+=(kotlin)
                ;;
            lang-python)
                surface_files+=(python.pyi)
                lib_filters+=(python)
                fixture_filters+=(python api_py_types api_free_threaded)
                ;;
            lang-swift)
                surface_files+=(swift.swift)
                lib_filters+=(swift)
                fixture_filters+=(swift)
                ;;
            lang-go)
                surface_files+=(go.txt)
                lib_filters+=(go)
                fixture_filters+=(go)
                ;;
            lang-dart)
                surface_files+=(dart.dart)
                lib_filters+=(dart)
                fixture_filters+=(dart)
                ;;
            lang-web)
                surface_files+=(web.d.ts)
                lib_filters+=(web js wasm)
                fixture_filters+=(web)
                ;;
            docs) ;;
        esac
    done

    if [[ "$need_moq_c" == true ]]; then
        just ffi moq-c
    fi
    if [[ "${#surface_files[@]}" -gt 0 && "$ran_full_surface" == false ]]; then
        if [[ "$(uname -sm)" == "Linux x86_64" ]]; then
            just ffi surface-check "${surface_files[@]}"
        else
            echo "test: the surface files are x86_64 Linux's exports, which CI checks" >&2
        fi
    fi
    if [[ "${#lib_filters[@]}" -gt 0 ]]; then
        just ffi test --bins -- "${lib_filters[@]}"
    fi
    if [[ "${#fixture_filters[@]}" -gt 0 ]]; then
        just ffi test --test fixtures -- "${fixture_filters[@]}"
    fi
    if [[ "${#loom_targets[@]}" -gt 0 ]]; then
        just ffi test "${loom_targets[@]}"
    fi
}
test_go() { just go test; }
# moq-net's translation at the pin, its first consumer, before the slower test build;
# narrowed to changed `lang/*` fixtures when only those changed.
test_ts() {
    if [[ "${ts_scopes[0]:-all}" == all ]]; then
        just ts moq
        just ts test
        return
    fi

    echo "test ts: scoped to ${ts_scopes[*]}" >&2
    local fixture_filters=()
    local s
    for s in "${ts_scopes[@]}"; do
        case "$s" in
            lang-*) fixture_filters+=("lang::${s#lang-}") ;;
            docs) ;;
        esac
    done
    if [[ "${#fixture_filters[@]}" -gt 0 ]]; then
        just ts test --test fixtures -- "${fixture_filters[@]}"
    fi
}

# Every check runs before any test, so a lint fails before the test builds.
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
