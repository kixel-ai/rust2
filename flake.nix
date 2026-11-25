{
  description = "rust2: Rust to other languages and faster builds";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The quest CLI, which also serves the quest guide and skills the stubs in
    # .claude/skills call. Bump the rev to upgrade them.
    quest = {
      url = "github:kixel-ai/quest";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    # The daemon the benchmark's ra-jit variant measures. Bumping the rev is a
    # benchmark corpus change: numbers before and after it do not compare.
    ra-jit = {
      url = "github:indexable-inc/ra-jit/d4f6027e82d312bf805ebfe6252369f6048a1e18";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
    };
    # CodSpeed's instrument-hooks, which the binding benchmark's programs call around each
    # case rust2-bench counts.
    instrument-hooks = {
      url = "github:CodSpeedHQ/instrument-hooks/8fd5b878499f819561b33219cdafaa6f8c28b4e4";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      quest,
      ra-jit,
      instrument-hooks,
      ...
    }:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ]
      (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          # Playwright's WPE build of WebKit, headless over Mesa's software EGL, so a host
          # with no GPU or display runs it: `webkit-headless <url>`. Its tip-of-tree engine
          # stands in for Safari, which has no Linux build.
          webkit-headless =
            let
              browsers = pkgs.playwright-driver.browsers;
              webkit = pkgs.lib.findFirst (
                name: pkgs.lib.hasPrefix "webkit-" name
              ) (throw "playwright-driver ships no WebKit") (builtins.attrNames browsers.passthru.entries);
            in
            pkgs.writeShellScriptBin "webkit-headless" ''
              export LIBGL_ALWAYS_SOFTWARE=1
              export __EGL_VENDOR_LIBRARY_DIRS=${pkgs.mesa}/share/glvnd/egl_vendor.d
              exec ${browsers.passthru.entries.${webkit}}/minibrowser-wpe/MiniBrowser --headless "$@"
            '';
          # Hermes at the tag React Native 0.88.1 pins, built for the host, which nixpkgs
          # doesn't package: the engine the React Native binding runs on off a device, in its
          # test, moq run, and benchmark column. Laid out as a checkout built into its
          # `build`, as RUST2_HERMES names one.
          hermes = pkgs.stdenv.mkDerivation {
            pname = "hermes";
            version = "260318099.0.5";
            src = pkgs.fetchFromGitHub {
              owner = "facebook";
              repo = "hermes";
              tag = "hermes-v260318099.0.5";
              hash = "sha256-OJb3UDHxvoN9euDgpsWb+OO8VSLzxq6CLZw4Vr942hk=";
            };
            nativeBuildInputs = with pkgs; [
              cmake
              ninja
              python3
            ];
            buildInputs = [ pkgs.icu ];
            cmakeFlags = [
              "-DCMAKE_BUILD_TYPE=Release"
              "-DHERMES_ENABLE_TEST_SUITE=OFF"
            ];
            ninjaFlags = [
              "hermesvm"
              "jsi"
            ];
            installPhase = ''
              mkdir -p $out/build/lib $out/build/jsi
              cp -r ../API ../public ../include $out/
              cp -r lib/config $out/build/lib/
              cp lib/libhermesvm${pkgs.stdenv.hostPlatform.extensions.sharedLibrary} $out/build/lib/
              cp jsi/libjsi.a $out/build/jsi/
            '';
          };
          # The benchmark's nightly levers (--jobs-frontend, cranelift, self-profile).
          # Bumping the date is a benchmark corpus change: numbers before and
          # after it do not compare.
          nightly = pkgs.rust-bin.nightly."2026-10-22".minimal.override {
            extensions = [ "rustc-codegen-cranelift-preview" ];
          };
          # ra-jit works with one rustc only. This is the toolchain its flake builds
          # with, so the daemon and the variant's plain cargo share it.
          raJitToolchain = pkgs.rust-bin.stable."1.99.0".default.override {
            extensions = [ "rust-src" ];
          };
          # ra-jit's dist build finds libzstd through pkg-config, which its flake never declares,
          # so it built only where nixpkgs put pkg-config on the build's PATH. Declare both here.
          # nixpkgs also points tikv-jemalloc-sys at a jemalloc path that does not exist, which
          # the build refuses. Unset it, so jemalloc builds from source as it did before.
          # Then wrap the binaries as ra-jit's `daemon` does, over the toolchain above.
          raJitDist = ra-jit.packages.${system}.dist.overrideAttrs (old: {
            nativeBuildInputs = old.nativeBuildInputs ++ [ pkgs.pkg-config ];
            buildInputs = (old.buildInputs or [ ]) ++ [ pkgs.zstd ];
            preBuild = ''
              unset JEMALLOC_OVERRIDE
            ''
            + (old.preBuild or "");
          });
          raJitDaemon =
            pkgs.runCommand "ra-jit-${raJitDist.version}"
              {
                nativeBuildInputs = [ pkgs.makeWrapper ];
              }
              ''
                mkdir -p $out/bin
                for bin in ${raJitDist}/bin/*; do
                  makeWrapper $bin $out/bin/$(basename $bin) --prefix PATH : ${raJitToolchain}/bin
                done
              '';
          # The TypeScript driver's own nightly, kept off PATH so the other projects keep
          # the repo toolchain. ts/justfile puts it first for its recipes.
          tsToolchain = pkgs.rust-bin.fromRustupToolchainFile ./ts/rust-toolchain.toml;
          # What a driver links beside rustc's own driver, which takes `-lz`, a library
          # nixpkgs' macOS SDK omits.
          driverLibs = pkgs.lib.optional pkgs.stdenv.hostPlatform.isDarwin pkgs.zlib;
          quest-cli = quest.packages.${system}.default;
          # std checked once per toolchain for the TypeScript driver's target, with the
          # profile translation reads its MIR at, as the sysroot rust2-ts takes instead of
          # checking std with -Zbuild-std in every target directory, and for the web
          # binding's, whose MIR rust2-ffi walks for what the web lacks. Offline, from the
          # crates rust-src vendors.
          rust2-ts-std = pkgs.stdenv.mkDerivation {
            name = "rust2-ts-std";
            dontUnpack = true;
            nativeBuildInputs = [ tsToolchain ];
            buildPhase = ''
              export CARGO_HOME=$TMPDIR/cargo CARGO_UNSTABLE_PROFILE_RUSTFLAGS=true
              mkdir -p sysroot/src && cd sysroot
              printf '[package]\nname = "sysroot"\nversion = "0.0.0"\nedition = "2024"\n' > Cargo.toml
              touch src/lib.rs
              # The crates ts/src/cli.rs's STD names.
              cargo check --offline --target wasm32-unknown-unknown --target wasm32-wasip1 -Zbuild-std=core,alloc,std,panic_abort \
                --config "source.crates-io.replace-with = 'vendored'" \
                --config "source.vendored.directory = '${tsToolchain}/lib/rustlib/src/rust/library/vendor'" \
                --config ${./ts/src/profile.toml}
            '';
            installPhase = ''
              for target in wasm32-unknown-unknown wasm32-wasip1; do
                lib=$out/lib/rustlib/$target/lib
                mkdir -p $lib
                find target/$target -name 'lib*.rmeta' ! -name 'libsysroot-*' -exec cp -t $lib {} +
              done
            '';
          };
          # The driver links its nightly's rustc libraries, and runs cargo from it too.
          rust2-ts =
            (pkgs.makeRustPlatform {
              cargo = tsToolchain;
              rustc = tsToolchain;
            }).buildRustPackage
              {
                pname = "rust2-ts";
                version = "0.0.0";
                # With the driver front end it shares, which it depends on by path.
                src = pkgs.lib.fileset.toSource {
                  root = ./.;
                  fileset = pkgs.lib.fileset.unions [
                    ./ts
                    ./driver
                  ];
                };
                buildAndTestSubdir = "ts";
                cargoRoot = "ts";
                cargoLock.lockFile = ./ts/Cargo.lock;
                cargoBuildFlags = [
                  "--package"
                  "rust2-ts"
                ];
                RUSTFLAGS = "-C link-args=-Wl,-rpath,${tsToolchain}/lib";
                nativeBuildInputs = [ pkgs.makeWrapper ];
                buildInputs = driverLibs;
                # tsc checks bindings, from the caller's PATH first.
                postInstall = ''
                  wrapProgram $out/bin/rust2-ts --set-default CARGO ${tsToolchain}/bin/cargo \
                    --set-default RUST2_TS_SYSROOT ${rust2-ts-std} \
                    --suffix PATH : ${pkgs.typescript}/bin
                '';
                # The tests need node, tsc, and wasm builds: `just ts test` runs them.
                doCheck = false;
              };
          # The public-visibility audit, on the same nightly, run by cargo as rustc.
          rust2-pub =
            (pkgs.makeRustPlatform {
              cargo = tsToolchain;
              rustc = tsToolchain;
            }).buildRustPackage
              {
                pname = "rust2-pub";
                version = "0.0.0";
                src = pkgs.lib.fileset.toSource {
                  root = ./.;
                  fileset = ./driver;
                };
                buildAndTestSubdir = "driver/pub";
                cargoRoot = "driver";
                cargoLock.lockFile = ./driver/Cargo.lock;
                RUSTFLAGS = "-C link-args=-Wl,-rpath,${tsToolchain}/lib";
                nativeBuildInputs = [ pkgs.makeWrapper ];
                buildInputs = driverLibs;
                # It checks with its own nightly's cargo and rustc, which build scripts run
                # as `$RUSTC`, unless the caller names others.
                postInstall = ''
                  wrapProgram $out/bin/rust2-pub \
                    --set-default CARGO ${tsToolchain}/bin/cargo \
                    --set-default RUSTC ${tsToolchain}/bin/rustc
                '';
                # The tests run cargo on a fixture workspace: `just driver test` runs them.
                doCheck = false;
              };
          # The exporter, on the drivers' nightly, run by cargo as rustc under the caller's
          # cargo, whose rustc the glue builds with.
          rust2-ffi =
            (pkgs.makeRustPlatform {
              cargo = tsToolchain;
              rustc = tsToolchain;
            }).buildRustPackage
              {
                pname = "rust2-ffi";
                version = "0.0.0";
                # With its fixtures, its workspace's members, and the front end it shares.
                src = pkgs.lib.fileset.toSource {
                  root = ./.;
                  fileset = pkgs.lib.fileset.unions [
                    ./ffi
                    ./driver
                  ];
                };
                buildAndTestSubdir = "ffi";
                cargoRoot = "ffi";
                cargoLock.lockFile = ./ffi/Cargo.lock;
                cargoBuildFlags = [
                  "--package"
                  "rust2-ffi"
                ];
                RUSTFLAGS = "-C link-args=-Wl,-rpath,${tsToolchain}/lib";
                nativeBuildInputs = [ pkgs.makeWrapper ];
                buildInputs = driverLibs;
                # A wasm32-wasip1 export checks std's MIR from the sysroot, which a stable
                # cargo could not build with -Zbuild-std.
                postInstall = ''
                  wrapProgram $out/bin/rust2-ffi --set-default RUST2_TS_SYSROOT ${rust2-ts-std}
                '';
                # The tests build every language's binding: `just ffi test` runs them.
                doCheck = false;
              };
          # The Valgrind rust2-bench counts under. Another can count differently, so a nixpkgs
          # bump that moves it fails here, and moves only by editing the version, a visible
          # change; its version also starts new series through each count's fingerprint.
          valgrind =
            pkgs.lib.throwIfNot (pkgs.valgrind.version == "3.27.1")
              "nixpkgs moved Valgrind to ${pkgs.valgrind.version}: bump the version flake.nix pins for rust2-bench"
              pkgs.valgrind;
          # The benchmark runner, on the repo's stable toolchain, with its Valgrind.
          rust2-bench =
            (pkgs.makeRustPlatform {
              cargo = toolchain;
              rustc = toolchain;
            }).buildRustPackage
              {
                pname = "rust2-bench";
                version = "0.0.0";
                src = pkgs.lib.fileset.toSource {
                  root = ./bench/runner;
                  fileset = pkgs.lib.fileset.unions [
                    ./bench/runner/Cargo.toml
                    ./bench/runner/Cargo.lock
                    ./bench/runner/src
                    # A workspace member, which Cargo reads though the runner builds without it.
                    ./bench/runner/harness
                  ];
                };
                cargoLock.lockFile = ./bench/runner/Cargo.lock;
                nativeBuildInputs = [ pkgs.makeWrapper ];
                postInstall = ''
                  wrapProgram $out/bin/rust2-bench --prefix PATH : ${valgrind}/bin
                '';
                # The tests run Valgrind: `just runner test` runs them.
                doCheck = false;
              };
          # rust2-ffi's generated async Kotlin needs coroutines on the
          # classpath; nixpkgs has no jar.
          kotlinx-coroutines = pkgs.fetchurl {
            url = "https://repo1.maven.org/maven2/org/jetbrains/kotlinx/kotlinx-coroutines-core-jvm/1.10.2/kotlinx-coroutines-core-jvm-1.10.2.jar";
            hash = "sha256-XKF1s43zMf1kFVs1zYyuElH6nuNpcJs21C4KKIzM4/0=";
          };
          # .NET Standard 2.1's reference assemblies, the API Unity's scripts compile
          # against, which rust2-ffi's generated C# also builds for; the SDK has none.
          netstandard-ref = pkgs.fetchzip {
            url = "https://www.nuget.org/api/v2/package/NETStandard.Library.Ref/2.1.0";
            extension = "zip";
            stripRoot = false;
            hash = "sha256-YD1IlzeVfK3aHwO9IyiCGnlh/yNIgfoEeezPBLOWV6M=";
          };
        in
        {
          packages = {
            quest = quest-cli;
            ts = rust2-ts;
            pub = rust2-pub;
            ffi = rust2-ffi;
            # Built only when a recipe running on it asks: it compiles from source.
            inherit hermes;
          }
          # Valgrind's counts are Linux's, and the fingerprint reads x86_64's CPUID.
          // pkgs.lib.optionalAttrs (pkgs.stdenv.hostPlatform.isLinux && pkgs.stdenv.hostPlatform.isx86_64) {
            bench = rust2-bench;
          }
          # ra-jit and its toolchain for the benchmark's ra-jit variants, which build it
          # only when they run: it compiles from source. ra-jit skips Intel macOS.
          // pkgs.lib.optionalAttrs (ra-jit.packages ? ${system}) {
            bench-ra-jit = pkgs.symlinkJoin {
              name = "bench-ra-jit";
              paths = [
                raJitDaemon
                raJitToolchain
              ];
            };
          };
          # `just bench` enters this shell; the nightly toolchain stays off PATH
          # so stable remains the default. It holds only what the benchmark runs:
          # a PR runs bench.yml only when this shell, or another output it lists
          # (bench-ra-jit), changes derivation.
          devShells.build-bench = pkgs.mkShell {
            packages =
              with pkgs;
              [
                toolchain
                just
                jq
                git
                taplo
                hyperfine
                cargo-nextest
                measureme
              ]
              # The alternative linkers the stable variants measure are ELF-only.
              ++ lib.optionals stdenv.hostPlatform.isLinux [
                mold
                wild
              ];
            RUST2_BENCH_NIGHTLY = "${nightly}/bin";
          };
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              toolchain
              quest-cli
              just
              jq
              git
              gh
              cargo-nextest
              cargo-deny
              cargo-shear
              cargo-sort
              actionlint
              shellcheck
              shfmt
              taplo
              nixfmt
              # The TypeScript driver's tests run and type-check its output, and bundle a
              # caller with Bun and esbuild to check that unused output shakes out.
              nodejs
              typescript
              bun
              esbuild
              # rust2-ffi's web binding runs its test on Deno as well as Node.
              deno
              # Its wasm-bindgen fixture's native side runs through the CLI's JS glue, whose
              # version must match the fixture's locked wasm-bindgen exactly.
              wasm-bindgen-cli_0_2_129
              # rust2-ffi's tests compile its generated Kotlin and run it on the JVM, run and
              # type-check its generated Python, run its Dart, and build its Go through cgo.
              kotlin
              python3
              mypy
              pyright
              # rust2-go's tests build and run its output against native Rust.
              go
              dart
              # ... and compile its C# with Roslyn and run it on .NET, NativeAOT included.
              dotnet-sdk_10
            ];
            # ... and its generated Swift, unless nixpkgs marks the compiler broken on this system.
            # The `swift` wrapper doesn't copy its compiler's `broken`, so ask the compiler.
            nativeBuildInputs =
              pkgs.lib.optional (!pkgs.swift.swiftc.meta.broken) pkgs.swift
              # rust2-bench's tests count under its Valgrind.
              ++ pkgs.lib.optional pkgs.stdenv.hostPlatform.isLinux valgrind
              ++ driverLibs;
            RUST2_TS_TOOLCHAIN = tsToolchain;
            RUST2_TS_SYSROOT = rust2-ts-std;
            # rust2-ffi's strict C++ build needs Clang 23's `-Wlifetime-safety`, which no
            # compiler on PATH has.
            RUST2_CLANG = "${pkgs.llvmPackages_23.clang}/bin/clang++";
            KOTLINX_COROUTINES = "${kotlinx-coroutines}";
            NETSTANDARD_REF = "${netstandard-ref}/ref/netstandard2.1";
            DOTNET_CLI_TELEMETRY_OPTOUT = "1";
            DOTNET_NOLOGO = "1";
            # The CPythons the generated Python loads in or refuses beside the default 3.14: the
            # free-threaded 3.14, which no stable ABI covers, and 3.15 with and without its GIL,
            # which load the abi3t extension. By path, since each python3 would shadow the
            # default's on PATH.
            PYTHON_314T = "${pkgs.python314FreeThreading}/bin/python3.14t";
            PYTHON_315T = "${pkgs.python315FreeThreading}/bin/python3.15t";
            PYTHON_315 = "${pkgs.python315}/bin/python3.15";
            # The pinned toolchain goes on PATH ahead of everything the host had, which
            # shadows the Cargo shim `mbx setup` installs. Put it back in front, as moq's dev
            # shell does, so cargo builds through mbx's cache wherever the host set one up,
            # the land gate included. The nightly projects' justfiles keep it ahead of their
            # toolchain through RUST2_CARGO_SHIM. `setup --status` names the shim on its first
            # line even when it is missing, and its exit code also fails over unrelated setup,
            # so only the shim existing decides.
            shellHook = ''
              # An outer shell's shim must not outlive a host without one: the justfiles
              # would still put its directory ahead of the toolchain.
              unset RUST2_CARGO_SHIM
              shim=$(mbx setup --status 2>/dev/null | sed -n '1s/.*: //p' || true)
              if [ -x "$shim" ]; then
                export RUST2_CARGO_SHIM=''${shim%/*}
                export PATH="$RUST2_CARGO_SHIM:$PATH"
              fi
              unset shim
            '';
          };
          # `just ts bench` adds Chrome and Firefox as engines, and `just ffi browsers` runs the
          # web binding's test in them, and in WebKit on Linux, which stands in for Safari. Apart for
          # their size. Binaryen's wasm2js lowers the bench's wasm32 build to JavaScript for its
          # wasm2js route.
          devShells.bench = self.devShells.${system}.default.overrideAttrs (old: {
            nativeBuildInputs =
              old.nativeBuildInputs
              ++ [
                pkgs.chromium
                pkgs.firefox
                pkgs.binaryen
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [ webkit-headless ]
              # `just ffi count` counts the programs' instructions with it, on x86_64 Linux only.
              ++ pkgs.lib.optionals (pkgs.stdenv.hostPlatform.isLinux && pkgs.stdenv.hostPlatform.isx86_64) [
                rust2-bench
              ];
            # `just ffi bench` builds moq's rs/libmoq, whose VAAPI bindings run bindgen.
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            # ... and its programs over CodSpeed's instrument-hooks, which rust2-bench reads.
            CODSPEED_INSTRUMENT_HOOKS = "${instrument-hooks}";
          });
          # Clang beside the default shell's GCC, which the nightly builds the exporter's C++ with.
          devShells.clang = self.devShells.${system}.default.overrideAttrs (old: {
            nativeBuildInputs = old.nativeBuildInputs ++ [ pkgs.clang ];
          });
          # `just ffi sanitize` builds the glue with the nightly's sanitizers, linked by clang,
          # runs it under the nightly's Miri and Valgrind, and runs the Python on a debug build of
          # the newest CPython and on a free-threaded one under ThreadSanitizer, which nothing
          # caches. llvm-symbolizer names a report's frames.
          devShells.sanitize = self.devShells.${system}.clang.overrideAttrs (old: {
            nativeBuildInputs = old.nativeBuildInputs ++ [
              pkgs.llvm
              pkgs.valgrind
            ];
            RUST2_PYTHON_DEBUG = "${pkgs.python315.override { enableDebug = true; }}/bin/python3";
            # Built by clang, whose ThreadSanitizer runtime the interpreter links and exports for
            # the extension it loads; its own reports during the build fail nothing.
            RUST2_PYTHON_TSAN = "${
              (pkgs.python315FreeThreading.override { stdenv = pkgs.clangStdenv; }).overrideAttrs (old: {
                configureFlags = old.configureFlags ++ [ "--with-thread-sanitizer" ];
                TSAN_OPTIONS = "exitcode=0:report_bugs=0";
              })
            }/bin/python3.15t";
            RUST2_TS_TOOLCHAIN = tsToolchain.override {
              extensions =
                (builtins.fromTOML (builtins.readFile ./ts/rust-toolchain.toml)).toolchain.components
                ++ [
                  "miri"
                ];
            };
          });
          # `just ffi android` builds the Kotlin's library for an Android device, with the NDK
          # cargo-ndk finds through ANDROID_NDK_HOME, and `just ffi moq-api` stages moq's API on
          # each Android ABI moq ships, and the x86 emulator runs the 32-bit test: the SDK and
          # NDK are the caller's, as moq's Android build takes the GitHub runner image's.
          devShells.android = self.devShells.${system}.default.overrideAttrs (old: {
            nativeBuildInputs = old.nativeBuildInputs ++ [ pkgs.cargo-ndk ];
            RUST2_TS_TOOLCHAIN = tsToolchain.override {
              targets = (builtins.fromTOML (builtins.readFile ./ts/rust-toolchain.toml)).toolchain.targets ++ [
                "aarch64-linux-android"
                "armv7-linux-androideabi"
                "i686-linux-android"
                "x86_64-linux-android"
              ];
            };
          });
          formatter = pkgs.nixfmt;
          checks.quest-tree = pkgs.runCommand "quest-tree" { nativeBuildInputs = [ quest-cli ]; } ''
            quest --root ${self} check
            touch "$out"
          '';
        }
      );
}
