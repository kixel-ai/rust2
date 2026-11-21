#!/usr/bin/env bash
# Writes macros.txt: every macro a C23, C++23, or POSIX header defines that C or C++ would
# expand in a header naming it, on glibc with libstdc++ (as g++'s default _GNU_SOURCE exposes
# them), musl, and macOS, those reserved for the implementation included, as libc exports
# glibc's `__WALL`. Run as `just ffi macros`, in the dev shell.
set -euo pipefail
cd "$(dirname "$0")"

c="assert.h complex.h ctype.h errno.h fenv.h float.h inttypes.h iso646.h limits.h locale.h
math.h setjmp.h signal.h stdalign.h stdarg.h stdatomic.h stdbit.h stdbool.h stdckdint.h stddef.h
stdint.h stdio.h stdlib.h stdnoreturn.h string.h tgmath.h threads.h time.h uchar.h wchar.h
wctype.h"
posix="aio.h arpa/inet.h cpio.h dirent.h dlfcn.h fcntl.h fmtmsg.h fnmatch.h ftw.h glob.h
grp.h iconv.h langinfo.h libgen.h libintl.h monetary.h mqueue.h ndbm.h net/if.h netdb.h
netinet/in.h netinet/tcp.h nl_types.h poll.h pthread.h pwd.h regex.h sched.h search.h
semaphore.h spawn.h strings.h sys/ipc.h sys/mman.h sys/msg.h sys/resource.h sys/select.h
sys/sem.h sys/shm.h sys/socket.h sys/stat.h sys/statvfs.h sys/time.h sys/times.h
sys/types.h sys/uio.h sys/un.h sys/utsname.h sys/wait.h syslog.h tar.h termios.h ulimit.h
unistd.h utime.h utmpx.h wordexp.h"
cxx="algorithm any array atomic barrier bit bitset charconv chrono codecvt compare complex
concepts condition_variable coroutine deque exception execution expected filesystem
format forward_list fstream functional future initializer_list iomanip ios iosfwd iostream
istream iterator latch limits list locale map memory memory_resource mutex new numbers
numeric optional ostream print queue random ranges ratio regex scoped_allocator semaphore
set shared_mutex source_location span spanstream sstream stack stacktrace stdexcept
stdfloat stop_token streambuf string string_view syncstream system_error thread tuple
type_traits typeindex typeinfo unordered_map unordered_set utility valarray variant vector
version cassert cctype cerrno cfenv cfloat cinttypes climits clocale cmath csetjmp csignal
cstdarg cstddef cstdint cstdio cstdlib cstring ctime cuchar cwchar cwctype"

# Includes each of `$@` the toolchain has.
includes() {
	for h in "$@"; do
		printf '#if __has_include(<%s>)\n#include <%s>\n#endif\n' "$h" "$h"
	done
}

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
musl=$(nix build --inputs-from .. --no-link --print-out-paths nixpkgs#musl.dev)
sdk=$(nix build --inputs-from .. --no-link --print-out-paths --system aarch64-darwin nixpkgs#apple-sdk_15)
sdk="$sdk/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk"
# Unwrapped, as the dev shell's clang adds glibc's headers to every target.
clang=$(nix build --inputs-from .. --no-link --print-out-paths nixpkgs#llvmPackages.clang-unwrapped)/bin/clang
resource=$("$clang" -print-resource-dir)
# libc++, which wraps the C headers in C++ as Apple's toolchain does, undefining the macros
# C++ declares as functions.
libcxx=$(nix build --inputs-from .. --no-link --print-out-paths nixpkgs#llvmPackages.libcxx.dev)/include/c++/v1

# C++ reads each macro, and a function-like one where a name is called, as a method is. C
# reads only an object-like one, since the C header calls no name but its own prefixed ones.
cxx_flags=(-x c++ -std=gnu++23 -dM -E -)
c_flags=(-x c -std=gnu23 -dM -E -)
musl_flags=(-target x86_64-linux-musl -nostdinc -isystem "$musl/include" -isystem "$resource/include")
mac_flags=(-target arm64-apple-macos14 -isysroot "$sdk")
libcxx_flags=(-nostdinc++ -isystem "$libcxx")
# shellcheck disable=SC2086
{
	includes $c $posix $cxx | g++ "${cxx_flags[@]}"
	# libc++ names no rune table of musl's, which no C++ header takes a macro from.
	includes $c $posix $cxx | "$clang" "${libcxx_flags[@]}" -D_LIBCPP_PROVIDES_DEFAULT_RUNE_TABLE \
		"${musl_flags[@]}" "${cxx_flags[@]}"
	includes $c $posix $cxx | "$clang" "${libcxx_flags[@]}" "${mac_flags[@]}" "${cxx_flags[@]}"
} | awk '$1 == "#define" { sub(/\(.*/, "", $2); print $2 }' >"$tmp/names"
# shellcheck disable=SC2086
{
	includes $c $posix | gcc "${c_flags[@]}"
	includes $c $posix | "$clang" "${musl_flags[@]}" "${c_flags[@]}"
	includes $c $posix | "$clang" "${mac_flags[@]}" "${c_flags[@]}"
} | awk '$1 == "#define" && $2 !~ /\(/ { print $2 }' >>"$tmp/names"

{
	echo "# Written by macros.sh: a macro on each line, sorted."
	LC_ALL=C sort -u "$tmp/names"
} >macros.txt
