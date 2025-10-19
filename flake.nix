{
  description = "Flake configuration for my systems";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      utils,
      rust-overlay,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          propagatedBuildInputs = [
            pkgs.probe-rs-tools
          ];

          buildInputs = [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "clippy"
                "rust-analyzer"
              ];
              targets = [
                "thumbv8m.main-none-eabihf"
              ];
            })
          ];

          shellHook = ''
            # Check if user is in dialout group and probe-rs is available
            if ! groups | grep -q dialout; then
              echo "⚠️  Warning: You are not in the 'dialout' group."
              echo "   This may cause permission issues with USB debuggers (probe-rs)."
              echo ""
            fi

            if ! command -v probe-rs >/dev/null 2>&1; then
              echo "⚠️  Warning: probe-rs not found in system PATH."
              echo ""
              echo "   To fix this, add to your /etc/nixos/configuration.nix:"
              echo "   environment.systemPackages = [ pkgs.probe-rs ];"
              echo "   users.users.$(whoami).extraGroups = [ \"dialout\" ];"
              echo ""
              echo "   Then run: sudo nixos-rebuild switch"
              echo "   And log out and back in for group changes to take effect."
              echo ""
            fi
          '';
        };
      }
    );
}
