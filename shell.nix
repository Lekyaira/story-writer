let
  pkgs = import <nixpkgs> { };
  # Unstable Nix
  # To use this, just prepend your package name with `unstable.`
  unstable = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {};
  # Rust toolchain
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
  rust_toolchain = fenix.combine [
    fenix.stable.toolchain # Standard Rust
	 # fenix.complete.toolchain # Nightly
    # fenix.targets.wasm32-unknown-unknown.latest.rust-std # Web Assembly Target
    # fenix.targets.x86_64-apple-darwin.latest.rust-std # MacOS target
    # fenix.targets.x86_64-pc-windows-gnu.latest.rust-std # Windows target
  ];
  # Get project directory.
  pd = builtins.toString ./.;
in
pkgs.mkShell {
  # Binaries, tools, etc. go here.
  packages = [
  ];

  # C/C++ libraries go here.
  nativeBuildInputs = with pkgs; [
    gcc
    udev
    rust_toolchain
    pkg-config
  ];

  # Other dependencies go here.
  buildInputs = with pkgs; [
    openssl.dev
  ];

  # Libraries
  LD_LIBRARY_PATH = with pkgs; pkgs.lib.makeLibraryPath [ openssl ];
}
