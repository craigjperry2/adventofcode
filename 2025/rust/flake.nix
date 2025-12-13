{
  description = "Rust Rover Dev Env";
  # I use this with a simple .envrc file containing only:
  #   use flake
  #

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-docs" "clippy" "rustfmt" ];
        };

        stdlibPath = "${rustToolchain}/lib/rustlib/src/rust/library";
        toolchainBin = "${rustToolchain}/bin";

        # Launcher script for Rust Rover
        rr = pkgs.writeShellScriptBin "rr" ''
          rustrover "$PWD" &
        '';

        # Script to configure .idea/workspace.xml with Nix toolchain paths
        configureIdeaWorkspace = pkgs.writeShellScript "configure-idea-workspace" ''
          IDEA_DIR="$PWD/.idea"
          WORKSPACE_FILE="$IDEA_DIR/workspace.xml"

          mkdir -p "$IDEA_DIR"

          if [ ! -f "$WORKSPACE_FILE" ]; then
            cat > "$WORKSPACE_FILE" << 'EOF'
          <?xml version="1.0" encoding="UTF-8"?>
          <project version="4">
            <component name="RustProjectSettings">
              <option name="explicitPathToStdlib" value="${stdlibPath}" />
              <option name="toolchainHomeDirectory" value="${toolchainBin}" />
            </component>
          </project>
          EOF
          elif ! grep -q "RustProjectSettings" "$WORKSPACE_FILE"; then
            ${pkgs.gnused}/bin/sed -i \
              's|</project>|  <component name="RustProjectSettings">\n    <option name="explicitPathToStdlib" value="${stdlibPath}" />\n    <option name="toolchainHomeDirectory" value="${toolchainBin}" />\n  </component>\n</project>|' \
              "$WORKSPACE_FILE"
          else
            ${pkgs.gnused}/bin/sed -i \
              -e 's|explicitPathToStdlib" value="[^"]*"|explicitPathToStdlib" value="${stdlibPath}"|' \
              -e 's|toolchainHomeDirectory" value="[^"]*"|toolchainHomeDirectory" value="${toolchainBin}"|' \
              "$WORKSPACE_FILE"
          fi
        '';
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ rustToolchain ];

          buildInputs = with pkgs; [
            openssl
            pkg-config
            jetbrains.rust-rover
            rr
          ];

          RUST_SRC_PATH = stdlibPath;

          shellHook = ''
            ${configureIdeaWorkspace}
          '';
        };
      }
    );
}

