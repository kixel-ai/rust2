#!/usr/bin/env bash
# Measures Rust build time on the pinned corpus and variants in corpus.toml.
# Usage and output are described in README.md.
set -euo pipefail

bench=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(cd "$bench/../.." && pwd)

# The build-bench shell carries hyperfine, the alternative linkers, and the pinned nightly.
if [[ -z ${RUST2_BENCH_NIGHTLY:-} ]]; then
    exec nix develop "$root#build-bench" --command "$bench/run.sh" "$@"
fi

die() {
    echo "bench: $*" >&2
    exit 1
}

usage() {
    cat <<'EOF'
Usage: just bench [OPTIONS]

Options:
  --corpus NAME    Measure only this corpus (repeatable; default: all)
  --variant NAME   Measure only this variant (repeatable; default: all)
  --runs N         Timed runs per scenario (default: 5 for the edit-* scenarios, else 3)
  --no-attribute   Skip the nightly build that attributes where the time goes
  --out DIR        Write results to this new or empty directory
                   (default: target/bench/results/<UTC time>)
EOF
}

# The incremental scenarios rewrite one marker, so every run is a real edit that
# reaches codegen: a function appended to a crate root or, given the line that
# opens a function's body, a statement at the top of that body. A rewrite moves
# no other line.
if [[ ${1:-} == edit ]]; then
    file=$2
    line=${3:-}
    [[ -f $file ]] || die "edit target $file does not exist"
    sed -i '/__rust2_bench_edit/d' "$file"
    if [[ -z $line ]]; then
        printf '#[doc = "rust2 bench edit"] pub fn __rust2_bench_edit() -> u64 { %s }\n' "$(date +%s%N)" >>"$file"
        exit 0
    fi
    # A statement, not a binding, so an async body holds nothing new across an await.
    awk -v line="$line" -v stmt="std::hint::black_box($(date +%s%N)_u64); // __rust2_bench_edit" '
        { print; text = $0; sub(/^[ \t]+/, "", text) }
        text == line { found++; print stmt }
        END { exit found != 1 }' "$file" >"$file.rust2-bench" ||
        { rm -f "$file.rust2-bench" && die "$file has no single line \`$line\`"; }
    mv "$file.rust2-bench" "$file"
    exit 0
fi

# Renders a results directory's report.md.
report() {
    local dir=$1
    local -a summaries=()
    [[ ! -d $dir/attribution ]] || mapfile -t summaries < <(find "$dir/attribution" -name summary.json | sort)
    # A file, not an argument: a clean build's per-unit summary outgrows the argument list.
    jq -n -r -f "$bench/report.jq" --slurpfile host "$dir/host.json" --slurpfile results "$dir/results.jsonl" \
        --slurpfile attribution <(((${#summaries[@]} == 0)) || cat "${summaries[@]}") >"$dir/report.md"
}

if [[ ${1:-} == report ]]; then
    report "${2:?report needs a results directory}"
    exit 0
fi

corpora=()
variants=()
runs=
attribute=1
out=
while (($#)); do
    case $1 in
        --corpus) corpora+=("${2:?--corpus needs a name}") && shift 2 ;;
        --variant) variants+=("${2:?--variant needs a name}") && shift 2 ;;
        --runs) runs=${2:?--runs needs a count} && shift 2 ;;
        --no-attribute) attribute=0 && shift ;;
        --out) out=${2:?--out needs a directory} && shift 2 ;;
        -h | --help) usage && exit 0 ;;
        *) usage >&2 && die "unknown argument: $1" ;;
    esac
done
[[ -z $runs || $runs =~ ^[1-9][0-9]*$ ]] || die "--runs must be a positive integer, got $runs"

config=$(taplo get -f "$bench/corpus.toml" -o json)
# Every scenario in the order it runs, and those a variant without a `scenarios` list runs.
scenarios='["check", "build", "edit-leaf", "edit-hub", "edit-sync", "edit-async", "test-build", "test-run", "test-edit"]'
build_scenarios='["check", "build", "edit-leaf", "edit-hub"]'
mapfile -t all_corpora < <(jq -r '.corpus[].name' <<<"$config")
mapfile -t all_variants < <(jq -r '.variant[].name' <<<"$config")
((${#corpora[@]})) || corpora=("${all_corpora[@]}")
((${#variants[@]})) || variants=("${all_variants[@]}")
for name in "${corpora[@]}"; do
    jq -e --arg n "$name" 'any(.corpus[]; .name == $n)' <<<"$config" >/dev/null || die "unknown corpus: $name"
done
for name in "${variants[@]}"; do
    base=$(jq -r --arg n "$name" '.variant[] | select(.name == $n) | .base // .name' <<<"$config")
    [[ -n $base ]] || die "unknown variant: $name"
    [[ " ${variants[*]} " == *" $base "* ]] || die "variant $name compares against $base; select it too"
    for key in corpora scenarios; do
        jq -e --arg n "$name" --arg b "$base" --arg k "$key" --argjson all "$scenarios" --argjson default "$build_scenarios" '
            {corpora: [[.corpus[].name], [.corpus[].name]], scenarios: [$all, $default]}[$k] as [$all, $default]
            | def runs($v): (.variant[] | select(.name == $v) | .[$k]) // $default;
            runs($n) - runs($b) == [] and runs($n) - $all == []' <<<"$config" >/dev/null ||
            die "variant $name lists $key that are unknown or that its base $base skips"
    done
done
for name in "${corpora[@]}"; do
    jq -e --arg c "$name" --args 'any(.variant[] | select(.name | IN($ARGS.positional[])); .corpora // [$c] | index($c))' \
        "${variants[@]}" <<<"$config" >/dev/null || die "no selected variant runs on corpus $name"
done

# Cargo reads every .cargo/config.toml above its working directory, so a user's
# ~/.cargo/config.toml (say, `build.incremental = false`) would reach checkouts
# under $HOME. Build outside it, with a private CARGO_HOME and none of the
# caller's cargo or rustc variables. Newer cargo also turns incremental off when
# CI or TF_BUILD is set, and the edit scenarios measure a developer's rebuild.
work=/var/tmp/rust2-bench-$(id -un)
results=$root/target/bench/results
out=${out:-$results/$(date -u +%Y%m%dT%H%M%SZ)}
# Runs append their samples, so an earlier run's would mix into this one's numbers.
[[ -z $(find "$out" -mindepth 1 -print -quit 2>/dev/null) ]] || die "$out is not empty; pass a new --out"
mkdir -p "$work" "$out"
out=$(cd "$out" && pwd)
dir=$work
while :; do
    for f in "$dir/.cargo/config" "$dir/.cargo/config.toml"; do
        [[ ! -e $f ]] || die "$f would configure the corpus builds; move it"
    done
    [[ $dir != / ]] || break
    dir=$(dirname "$dir")
done
caller_vars=("${!CARGO@}" "${!RUSTC@}" "${!RUSTDOC@}" "${!RUSTUP@}" "${!RA_JIT@}")
for name in RUSTFLAGS CI TF_BUILD; do
    [[ -z ${!name+set} ]] || caller_vars+=("$name")
done
if ((${#caller_vars[@]})); then
    echo "bench: ignoring ${caller_vars[*]}"
    unset "${caller_vars[@]}"
fi
export CARGO_HOME=$work/cargo-home
stable_path=$PATH
nightly_path=$RUST2_BENCH_NIGHTLY:$PATH

# ra-jit compiles from source, so only a run that measures it builds it.
rajit_path=
if jq -e --argjson corpora "$(jq -n -c '$ARGS.positional' --args "${corpora[@]}")" --args '
    any(.variant[] | select(.name | IN($ARGS.positional[])) | select(.toolchain == "ra-jit");
        any((.corpora // $corpora)[]; IN($corpora[])))' "${variants[@]}" <<<"$config" >/dev/null; then
    echo "bench: building ra-jit"
    rajit_path=$(nix build --no-link --print-out-paths "$root#bench-ra-jit")/bin:$PATH
fi

# ra-jit's daemon outlives the build that starts it, so it gets a home and a store
# of its own, and stops however the run ends.
export RA_JIT_HOME=$work/ra-jit/home RA_JIT_CACHE=$work/ra-jit/store
rajit_stop() {
    [[ ! -S $RA_JIT_HOME/daemon.sock ]] || PATH=$rajit_path cargo ra-jit stop >/dev/null
}
trap rajit_stop EXIT

field() { # corpus|variant, name, jq path
    jq -r --arg n "$2" ".$1[] | select(.name == \$n) | $3 // empty" <<<"$config"
}

# Whether a variant runs on a corpus or scenario: those its list under the key names,
# or else every corpus and the build scenarios.
runs_on() { # variant corpora|scenarios name
    jq -e --arg n "$1" --arg k "$2" --arg v "$3" --argjson default "$build_scenarios" '
        .variant[] | select(.name == $n) | .[$k] // (if $k == "scenarios" then $default else [$v] end) | index($v) != null' \
        <<<"$config" >/dev/null
}

record_host() {
    local cpu
    cpu=$(sed -n 's/^model name[[:space:]]*: //p' /proc/cpuinfo 2>/dev/null | head -1)
    [[ -n $cpu ]] || cpu=$(sysctl -n machdep.cpu.brand_string 2>/dev/null || echo unknown)
    jq -n \
        --arg date "$(date -u +%FT%TZ)" \
        --arg os "$(uname -srm)" \
        --arg cpu "$cpu" \
        --arg cpus "$(getconf _NPROCESSORS_ONLN)" \
        --arg memory "$(awk '/MemTotal/ { printf "%.0f GiB", $2 / 1048576 }' /proc/meminfo 2>/dev/null)" \
        --arg runner "${RUNNER_NAME:-local}" \
        --arg rust2 "$(git -C "$root" rev-parse HEAD)" \
        --arg stable "$(PATH=$stable_path rustc -V)" \
        --arg nightly "$(PATH=$nightly_path rustc -V)" \
        --arg ra_jit "$([[ -z $rajit_path ]] && echo "ra-jit not measured" ||
            echo "$(PATH=$rajit_path ra-jit --version) on $(PATH=$rajit_path rustc -V)")" \
        --arg hyperfine "$(hyperfine --version)" \
        --arg nextest "$(cargo-nextest nextest --version | head -1)" \
        --arg mold "$(mold --version 2>/dev/null || echo missing)" \
        --arg wild "$(wild --version 2>/dev/null || echo missing)" \
        '$ARGS.named' >"$out/host.json"
}

# Clones the corpus at its pinned rev, undoing edits an interrupted run left behind.
checkout() {
    local name=$1 dir=$2 rev
    rev=$(field corpus "$name" .rev)
    if [[ ! -d $dir/.git ]]; then
        git init -q "$dir"
        git -C "$dir" remote add origin "$(field corpus "$name" .git)"
    fi
    if [[ $(git -C "$dir" rev-parse -q --verify HEAD || true) != "$rev" ]]; then
        git -C "$dir" fetch -q --depth 1 origin "$rev"
        git -C "$dir" checkout -q --detach FETCH_HEAD
    fi
    git -C "$dir" reset -q --hard "$rev"
    git -C "$dir" clean -qfd
    (cd "$dir" && PATH=$stable_path cargo fetch --locked -q)
}

# Fails if a crate in the build has a `links` key or compiles C with cc or cmake.
check_no_c() { # corpus cargo-args...
    local name=$1 tree found
    shift
    tree=$(cargo tree --locked --offline "$@" -e normal,build --prefix none -f '{p}' |
        awk '{ sub(/^v/, "", $2); print $1 " " $2 }' | sort -u | jq -R . | jq -s .)
    found=$(cargo metadata --locked --offline --format-version 1 | jq -r --argjson tree "$tree" '
		.packages[]
		| select(("\(.name) \(.version)") as $k | $tree | index($k))
		| select(.links != null or .name == "cc" or .name == "cmake")
		| "\(.name) \(.version)"')
    [[ -z $found ]] || die "corpus $name is marked no-c but builds: $(tr '\n' ' ' <<<"$found")"
}

# Enters a variant's toolchain, flags, and target directory for the cargo calls that
# follow, sets `sub` to what goes between cargo and its subcommand, and `cfg` to
# what follows the command. Cargo does not forward a `--config` before an external
# subcommand such as nextest, so the variant's config goes last.
use_variant() { # corpus variant
    case $(field variant "$2" .toolchain) in
        stable) PATH=$stable_path ;;
        nightly) PATH=$nightly_path ;;
        ra-jit) PATH=$rajit_path ;;
        *) die "variant $2 has no valid toolchain" ;;
    esac
    case $(field variant "$2" .subcommand) in
        '') sub=() ;;
        ra-jit) sub=(ra-jit) ;;
        *) die "variant $2 has no valid subcommand" ;;
    esac
    mapfile -t cfg < <(field variant "$2" '(.config // [])[] | "--config=" + .')
    RUSTFLAGS=$(field variant "$2" .rustflags)
    CARGO_TARGET_DIR=$work/target/$1/$2
    export PATH RUSTFLAGS CARGO_TARGET_DIR
}

# The ra-jit daemon keeps working after a build returns: it loads the workspace again
# after cargo builds, and compiles ahead once nothing has asked it for a second.
# Waits until it has been idle for three seconds, so none of that overlaps another
# run, and prints its pid, the CPU seconds its processes and their children have
# used, and the memory they hold.
rajit_settle() { # dir
    local dir=$1 status pid cpu rss last="" quiet=0 i
    for ((i = 0; i < 1800; i++)); do
        status=$(cargo ra-jit status)
        pid=$(sed -n 's/^daemon: pid \([0-9]*\),.*/\1/p' <<<"$status")
        [[ -n $pid ]] || die "the ra-jit daemon is not running"
        # The daemon, its workers, and what they run; a child they have reaped counts in its parent.
        read -r cpu rss < <(ps -e -o pid=,ppid= | awk -v root="$pid" '
            { parent[$1] = $2 }
            END { for (p in parent) { q = p; while (q != root && q in parent) q = parent[q]; if (q == root) print p } }' |
            while read -r p; do cat "/proc/$p/stat" 2>/dev/null; done |
            awk -v hz="$(getconf CLK_TCK)" -v page="$(getconf PAGESIZE)" '
                { sub(/^.*\) /, ""); cpu += $12 + $13 + $14 + $15; rss += $22 }
                END { printf "%.2f %d\n", cpu / hz, rss * page }')
        if [[ $status != *"    $dir: loading"* && -n $last ]] && awk -v a="$cpu" -v b="$last" 'BEGIN { exit !(a - b < 0.05) }'; then
            ((++quiet < 3)) || {
                echo "$pid $cpu $rss"
                return
            }
        else
            quiet=0
        fi
        last=$cpu
        sleep 1
    done
    die "the ra-jit daemon was still busy after 30 minutes"
}

# How many builds of the checkout the daemon served, and how many it left to cargo
# by reason, summed over every build configuration it keeps.
rajit_counts() { # dir
    find "$RA_JIT_HOME/states" -regex '.*/[0-9a-f]+\.json' -print0 2>/dev/null | xargs -0 -r cat |
        jq -s -c --arg root "$1" '
            map(select(.root == $root) | .counts)
            | {served: (map(.served) | add // 0), cargo: (map(.cargo | to_entries[]) | group_by(.key) | map({key: .[0].key, value: (map(.value) | add)}) | from_entries)}'
}

# Times one run with hyperfine, appending its measurement to the scenario's raw log.
# Its output goes to a log, which a failure shows.
# Earlier runs leave dirty pages behind (over a GiB on moq), and writing them
# back stalls whichever run it overlaps, so each run starts after a sync.
# A ra-jit run also records whether the daemon or cargo built it, and the CPU time
# and memory of the daemon, which hyperfine does not see. A test run records how
# many tests failed, from nextest's summary; it runs them all and fails only
# when nextest cannot run them.
time_once() { # corpus dir scenario variant prepare cargo-args...
    local corpus=$1 dir=$2 scenario=$3 variant=$4 prepare=$5 json log
    local -a failure=()
    shift 5
    json=$out/raw/$corpus.$scenario.$variant.json
    log=$out/logs/$corpus.$scenario.$variant.log
    # nextest exits with 100 when tests fail.
    [[ $scenario != test-run ]] || failure=(--ignore-failure=100)
    (
        use_variant "$corpus" "$variant"
        cd "$dir"
        local before after counts measurement summary
        if ((${#sub[@]})); then
            before=$(rajit_settle "$dir")
            counts=$(rajit_counts "$dir")
        fi
        # hyperfine sends only stdout to the log; nextest and cargo report on stderr.
        hyperfine --style none --runs 1 --prepare "$prepare && sync" --export-json "$json" --output "$log" "${failure[@]}" \
            --shell bash "$(printf '%q ' cargo "${sub[@]}" "$@" "${cfg[@]}") 2>&1" ||
            { tail -40 "$log" >&2 && die "$corpus $scenario failed with $variant"; }
        measurement=$(jq -c '.results[0].measurements[0]' "$json")
        if ((${#failure[@]})); then
            # "Summary [  34.7s] 5418 tests run: 5376 passed, 42 failed, 33 skipped"
            summary=$(sed -n 's/^ *Summary \[.*\] \([0-9]*\) tests run: \([0-9]*\) passed.*/\1 \2/p' "$log")
            [[ -n $summary ]] || { tail -40 "$log" >&2 && die "$corpus $scenario with $variant printed no nextest summary"; }
            measurement=$(jq -c --arg summary "$summary" '($summary | split(" ") | map(tonumber)) as [$run, $passed]
                | . + {tests: {run: $run, failed: ($run - $passed)}}' <<<"$measurement")
        fi
        if ((${#sub[@]})); then
            after=$(rajit_settle "$dir")
            measurement=$(jq -c --argjson before "$counts" --argjson after "$(rajit_counts "$dir")" \
                --arg was "$before" --arg now "$after" '
                ($was | split(" ")) as [$pid0, $cpu0] | ($now | split(" ")) as [$pid1, $cpu1, $rss]
                | . + {ra_jit: {
                    served: ($after.served > $before.served),
                    cargo: [$after.cargo | to_entries[] | select(.value > ($before.cargo[.key] // 0)) | .key][0],
                    cpu: (($cpu1 | tonumber) - (if $pid0 == $pid1 then $cpu0 | tonumber else 0 end)),
                    rss: ($rss | tonumber)
                }}' <<<"$measurement")
        fi
        echo "$measurement" >>"$out/raw/$corpus.$scenario.$variant.jsonl"
        jq -r --arg label "$corpus $scenario $variant" '"bench: \($label) \(.time_wall_clock.value * 100 | round / 100) s"
            + if .ra_jit == null then "" elif .ra_jit.served then " by ra-jit" else " by cargo" end
            + if .tests == null then "" else ", \(.tests.failed) of \(.tests.run) tests failed" end' <<<"$measurement"
    )
}

# Folds a scenario's runs of one variant into a result record. Load on the host
# adds an occasional slow run, so the compared numbers are trimmed means: the
# mean without the fastest and slowest run. A ra-jit run's CPU time includes its
# daemon's; its instructions, which nothing counts in the daemon, are left out.
summarize_runs() { # corpus scenario variant
    jq -s -c --arg corpus "$1" --arg scenario "$2" --arg variant "$3" \
        --arg base "$(field variant "$3" '.base // .name')" '
        def mean: add / length;
        def trimmed: sort | if length > 2 then .[1:-1] else . end | mean;
        [.[].time_wall_clock.value] as $t
        | ($t | mean) as $mean
        | any(.[]; .ra_jit != null) as $daemon
        | {
            $corpus, $scenario, $variant, $base, $mean,
            stddev: (if ($t | length) > 1 then $t | map(. - $mean | . * .) | add / (length - 1) | sqrt else null end),
            trimmed: ($t | trimmed),
            cpu: (map(.time_cpu.value + (.ra_jit.cpu // 0)) | trimmed),
            instructions: (if all(.[]; .instructions.value != null) and ($daemon | not) then map(.instructions.value) | trimmed else null end),
            rss: (map(.memory_peak_resident.value) | max),
            times: $t
        }
        + if $daemon then {ra_jit: {
            served: map(select(.ra_jit.served)) | length,
            cargo: (map(select(.ra_jit.served | not) | .ra_jit.cargo // "no reason given") | group_by(.) | map({key: .[0], value: length}) | from_entries),
            rss: (map(.ra_jit.rss) | max)
        }} else {} end
        + if any(.[]; .tests != null) then {tests: {run: (map(.tests.run) | max), failed: (map(.tests.failed) | max)}} else {} end' \
        "$out/raw/$1.$2.$3.jsonl" >>"$out/results.jsonl"
}

# Times every scenario on one corpus. Runs interleave the variants, rotating
# which goes first each round, so drift over a long job spreads across all of
# them instead of favoring whichever ran last.
bench_corpus() { # corpus dir
    local corpus=$1 dir=$2 scenario prepare n i j variant
    local -a args test present active scenario_names cmd edit
    mapfile -t args < <(jq -r --arg n "$corpus" '.corpus[] | select(.name == $n) | .args[]' <<<"$config")
    mapfile -t test < <(jq -r --arg n "$corpus" '.corpus[] | select(.name == $n) | .test // [] | .[]' <<<"$config")
    mapfile -t scenario_names < <(jq -r '.[]' <<<"$scenarios")

    present=()
    for variant in "${variants[@]}"; do
        if runs_on "$variant" corpora "$corpus"; then
            present+=("$variant")
        fi
    done

    # Each corpus starts ra-jit from nothing: no daemon, and an empty store.
    rajit_stop
    rm -rf "$work/ra-jit"
    mkdir -p "$work/ra-jit" && mkdir -m 700 "$RA_JIT_HOME"

    # An untimed clean build per variant warms the disk cache and shows any failure in full.
    for variant in "${present[@]}"; do
        echo "bench: $corpus warm-up $variant"
        (
            use_variant "$corpus" "$variant"
            rm -rf "$CARGO_TARGET_DIR"
            cd "$dir"
            # The `||` below turns errexit off in here, so a failed build has to exit itself.
            cargo "${sub[@]}" build --locked --offline "${args[@]}" "${cfg[@]}" >"$out/logs/$corpus.$variant.log" 2>&1 || exit
            ((${#sub[@]} == 0)) || rajit_settle "$dir" >/dev/null
        ) || { tail -40 "$out/logs/$corpus.$variant.log" >&2 && die "$corpus failed to build with $variant"; }
    done

    for scenario in "${scenario_names[@]}"; do
        active=()
        for variant in "${present[@]}"; do
            if runs_on "$variant" scenarios "$scenario"; then
                active+=("$variant")
            fi
        done
        ((${#active[@]})) || continue
        # CI builds without incremental compilation, so the test scenarios do too.
        unset CARGO_INCREMENTAL
        case $scenario in
            check | build)
                cmd=("$scenario" --locked --offline -q "${args[@]}")
                prepare="cargo clean -q"
                n=${runs:-3}
                ;;
            edit-*)
                cmd=(build --locked --offline -q "${args[@]}")
                n=${runs:-5}
                ;;
            test-*)
                ((${#test[@]})) || continue
                export CARGO_INCREMENTAL=0
                cmd=(nextest run --tests --locked --offline --cargo-quiet "${test[@]}")
                [[ $scenario == test-run ]] && cmd+=(--no-fail-fast) || cmd+=(--no-run)
                prepare="cargo clean -q"
                [[ $scenario != test-run ]] || prepare=true
                n=${runs:-3}
                ;;
        esac
        if [[ $scenario == edit-* || $scenario == test-edit ]]; then
            # The file the scenario edits and, for a body edit, the line that opens the body.
            mapfile -t edit < <(jq -r --arg n "$corpus" --arg s "$scenario" '.corpus[] | select(.name == $n)
                | {"edit-leaf": .leaf, "edit-hub": .hub, "edit-sync": .sync, "edit-async": .async, "test-edit": .hub}[$s]
                | strings, arrays[]' <<<"$config")
            ((${#edit[@]})) || continue
            prepare=$(printf '%q ' "$bench/run.sh" edit "${edit[@]}")
            # Add the marker and build it, so each timed run is an edit.
            "$bench/run.sh" edit "$dir/${edit[0]}" "${edit[@]:1}"
            for variant in "${active[@]}"; do
                (
                    use_variant "$corpus" "$variant"
                    cd "$dir"
                    cargo "${sub[@]}" "${cmd[@]}" "${cfg[@]}"
                    ((${#sub[@]} == 0)) || rajit_settle "$dir" >/dev/null
                )
            done
        fi
        for ((i = 0; i < n; i++)); do
            for ((j = 0; j < ${#active[@]}; j++)); do
                variant=${active[(i + j) % ${#active[@]}]}
                time_once "$corpus" "$dir" "$scenario" "$variant" "$prepare" "${cmd[@]}"
            done
        done
        for variant in "${active[@]}"; do
            summarize_runs "$corpus" "$scenario" "$variant"
        done
    done
    unset CARGO_INCREMENTAL

    rajit_stop
    git -C "$dir" checkout -q -- .
    rm -rf "$work/target/$corpus" "$work/ra-jit"
}

# One nightly clean build and each edit, with cargo's build analysis log and
# rustc's self-profile, summarized into where the time goes.
attribute() { # corpus dir
    local corpus=$1 dir=$2 leaf hub prof stamp step log dest proc_macros
    local -a args logs steps=(build)
    mapfile -t args < <(jq -r --arg n "$corpus" '.corpus[] | select(.name == $n) | .args[]' <<<"$config")
    leaf=$(field corpus "$corpus" .leaf)
    hub=$(field corpus "$corpus" .hub)
    steps+=(edit-leaf)
    [[ -z $hub ]] || steps+=(edit-hub)
    echo "bench: $corpus attribution"
    # Build analysis logs proc macros as plain libs.
    proc_macros=$(cd "$dir" && cargo metadata --locked --offline --format-version 1 |
        jq -c '[.packages[] | select(any(.targets[]; .kind | index("proc-macro"))) | .id]')

    PATH=$nightly_path
    export CARGO_TARGET_DIR=$work/target/$corpus/attribution
    rm -rf "$CARGO_TARGET_DIR"
    prof=$CARGO_TARGET_DIR/self-profile
    stamp=$CARGO_TARGET_DIR/started
    mkdir -p "$prof"
    # One profile directory for every step: a changed RUSTFLAGS would rebuild everything.
    export RUSTFLAGS="-Z self-profile=$prof"
    # Add the markers up front so each edit step is a body edit, as in the timed runs.
    "$bench/run.sh" edit "$dir/$leaf"
    [[ -z $hub ]] || "$bench/run.sh" edit "$dir/$hub"
    for step in "${steps[@]}"; do
        case $step in
            edit-leaf) "$bench/run.sh" edit "$dir/$leaf" ;;
            edit-hub) "$bench/run.sh" edit "$dir/$hub" ;;
        esac
        touch "$stamp"
        (cd "$dir" && cargo -Z build-analysis -Z section-timings build --config build.analysis.enabled=true \
            --locked --offline "${args[@]}" >"$out/logs/$corpus.attribution.$step.log" 2>&1) ||
            { tail -40 "$out/logs/$corpus.attribution.$step.log" >&2 && die "$corpus failed the $step attribution build"; }
        # Cargo names the log by session, so find this build's among any written since.
        mapfile -t logs < <(find "$CARGO_HOME/log" -name '*.jsonl' -newer "$stamp")
        log=
        for candidate in "${logs[@]}"; do
            [[ $(head -1 "$candidate" | jq -r .target_dir) != "$CARGO_TARGET_DIR" ]] || log=$candidate
        done
        [[ -n $log ]] || die "found no cargo build log for $CARGO_TARGET_DIR"
        dest=$out/attribution/$corpus.$step
        mkdir -p "$dest"
        mv "$log" "$dest/cargo.jsonl"
        find "$prof" -name '*.mm_profdata' -print0 |
            xargs -0 -r -n 1 -P "$(getconf _NPROCESSORS_ONLN)" summarize summarize --json >/dev/null
        jq -n -f "$bench/self-profile.jq" "$prof"/*.json >"$dest/self-profile.json"
        rm -f "$prof"/*
        jq -s -f "$bench/attribute.jq" --arg corpus "$corpus" --arg step "$step" --argjson proc_macros "$proc_macros" \
            --slurpfile profile "$dest/self-profile.json" "$dest/cargo.jsonl" >"$dest/summary.json"
    done
    unset RUSTFLAGS

    git -C "$dir" checkout -q -- .
    rm -rf "$CARGO_TARGET_DIR"
}

mkdir -p "$out/raw" "$out/logs"
record_host
for corpus in "${corpora[@]}"; do
    dir=$work/src/$corpus
    checkout "$corpus" "$dir"
    mapfile -t args < <(jq -r --arg n "$corpus" '.corpus[] | select(.name == $n) | .args[]' <<<"$config")
    if [[ $(field corpus "$corpus" '."no-c"') == true ]]; then
        (cd "$dir" && PATH=$stable_path check_no_c "$corpus" "${args[@]}")
    fi
    bench_corpus "$corpus" "$dir"
    # A subshell keeps the nightly toolchain and flags from leaking into the next corpus.
    ((attribute == 0)) || (attribute "$corpus" "$dir")
done

report "$out"
mkdir -p "$results"
ln -sfn "$out" "$results/latest"
echo "bench: report at $out/report.md"
