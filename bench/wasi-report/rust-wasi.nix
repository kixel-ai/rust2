let
  f = builtins.getFlake (toString ../..);
  pkgs = import f.inputs.nixpkgs { system = "x86_64-linux"; overlays = [ f.inputs.rust-overlay.overlays.default ]; };
in pkgs.rust-bin.stable."1.100.0".default.override { targets = [ "wasm32-wasip1" "wasm32-wasip2" "wasm32-wasip3" ]; }
