{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "multi-rust-projects";

  buildInputs = with pkgs; [
    rustup           # Rust toolchain manager
    cargo            # Rust package manager
    rust-analyzer    # Language server for Rust
    clippy           # Rust linter
    cargo-watch      # Automatically rebuild/tests on file changes
    ripgrep          # For better search in Neovim
    direnv           # Optional: Auto-load Nix environment
  ];

  shellHook = ''
    echo "Entering Nix shell for multi-rust-projects development"
    rustup default stable
    rustup component add rustfmt clippy
  '';
}
