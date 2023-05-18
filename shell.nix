{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs { overlays = [ (import ./nix/rust-overlay.nix) ]; }
}:

with pkgs;
let
  inherit (lib) optional optionals;

  rust = pkgs.rust-bin.stable."1.67.0".default.override {
    # for rust-analyzer
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  };

  basePackages = [
    (import ./nix/default.nix { inherit pkgs; })
    gcc12
    clang_13
    cmake
    pkgs.niv
    fswatch
    rust
    rust-analyzer
    nodejs-16_x
    (yarn.override { nodejs = nodejs-16_x; })
    openssl
    ffmpeg
    wasm-pack
    binaryen
    wasm-bindgen-cli
    bacon
    concurrently
    cargo-watch
    pkg-config
    nodePackages.prettier
    clippy
  ];

  inputs = basePackages ++
    lib.optionals stdenv.isLinux [ inotify-tools fontconfig ] ++
    lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [ CoreServices CoreVideo AppKit ]);

  hooks = ''
    export CARGO_INSTALL_ROOT=$PWD/.nix-cargo
    export CARGO_HOME=$PWD/.nix-cargo
    mkdir -p $CARGO_HOME
    export PATH=$CARGO_HOME/bin:bin:$PATH

    export LIBCLANG_PATH="${llvmPackages_13.libclang.lib}/lib"
    export UNWRAPPED_CC_BIN="${pkgs.llvmPackages_13.clang-unwrapped}/bin/clang++"
    export LLVM_AR_BIN="${pkgs.llvm_13}/bin/llvm-ar"
  '';
in

mkShell {
  buildInputs = inputs;
  shellHook = hooks;

  LOCALE_ARCHIVE = if pkgs.stdenv.isLinux then "${pkgs.glibcLocales}/lib/locale/locale-archive" else "";
}
