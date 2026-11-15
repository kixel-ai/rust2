#!/bin/sh
# Install crabx's release binary:
#   curl -fsSL https://crabx.kixel.ai/install.sh | sh
# CRABX_VERSION picks a version instead of the newest stable one on crates.io, and
# CRABX_INSTALL_DIR the directory instead of ~/.local/bin.
set -eu

fail() {
    echo "crabx install: $*" >&2
    exit 1
}

target() {
    arch=$(uname -m)
    case $(uname -s) in
        Linux)
            # The release binaries need glibc 2.28 or later; getconf ships with glibc.
            glibc=$(getconf GNU_LIBC_VERSION 2>/dev/null) ||
                fail "crabx's Linux builds need glibc, and this system has none; install it with \`cargo install crabx\`"
            echo "${glibc#glibc }" | awk -F . '{ exit !($1 > 2 || ($1 == 2 && $2 >= 28)) }' ||
                fail "crabx's Linux builds need glibc 2.28 or later, not $glibc; install it with \`cargo install crabx\`"
            os=unknown-linux-gnu
            ;;
        Darwin)
            os=apple-darwin
            # A shell under Rosetta reports x86_64 on Apple silicon.
            if [ "$(sysctl -n hw.optional.arm64 2>/dev/null)" = 1 ]; then
                arch=arm64
            fi
            ;;
        MINGW* | MSYS* | CYGWIN*) fail "on Windows, run install.ps1 from PowerShell" ;;
        *) fail "no prebuilt crabx for $(uname -s); install it with \`cargo install crabx\`" ;;
    esac
    case $arch in
        x86_64 | amd64) echo "x86_64-$os" ;;
        aarch64 | arm64) echo "aarch64-$os" ;;
        *) fail "no prebuilt crabx for $arch; install it with \`cargo install crabx\`" ;;
    esac
}

# The highest version on crates.io that is neither yanked nor a prerelease.
newest() {
    index=$(curl -fsSL https://index.crates.io/cr/ab/crabx) || fail "cannot read crabx's versions from crates.io"
    echo "$index" |
        sed -n 's/.*"vers":"\([^"]*\)".*"yanked":false.*/\1/p' |
        awk -F '[.+]' '
			{ core = $0; sub(/\+.*/, "", core) }
			core ~ /-/ { next }
			!found || $1 > a || ($1 == a && ($2 > b || ($2 == b && $3 > c))) {
				found = 1; a = $1 + 0; b = $2 + 0; c = $3 + 0; best = $0
			}
			END { if (found) print best }'
}

target=$(target)
version=${CRABX_VERSION:-$(newest)}
[ -n "$version" ] || fail "crates.io has no stable crabx release"
case $version in
    *[!0-9A-Za-z.+-]*) fail "invalid version $version" ;;
esac
dir=${CRABX_INSTALL_DIR:-$HOME/.local/bin}
name=crabx-$target-v$version
url=https://github.com/kixel-ai/rust2/releases/download/crabx-v$version/$name.tgz

temp=$(mktemp -d)
trap 'rm -rf "$temp"' EXIT
echo "crabx install: downloading crabx $version for $target" >&2
curl -fsSL "$url" -o "$temp/crabx.tgz" || fail "cannot download $url; crabx $version's binaries may not be released yet"
tar -xzf "$temp/crabx.tgz" -C "$temp" "$name/crabx"
mkdir -p "$dir"
# Replace by rename, so a running crabx keeps its old file.
mv "$temp/$name/crabx" "$dir/.crabx.$$"
chmod 755 "$dir/.crabx.$$"
mv "$dir/.crabx.$$" "$dir/crabx"
echo "crabx install: installed $dir/crabx" >&2
case ":$PATH:" in
    *":$dir:"*) ;;
    *) echo "crabx install: add $dir to PATH to run crabx" >&2 ;;
esac
