{
  description = "Rust Rover Dev Env";
  # I use this with a simple .envrc file containing only:
  #   use flake
  #   PATH_add bin
  #

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable."1.92.0".default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" ];
        };
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchain
          ];

          buildInputs = with pkgs; [
            openssl
            pkg-config

            jetbrains.rust-rover
          ];

          shellHook = ''
            # Generate a bin/rr launcher script which i add to the $PATH via .envrc
            mkdir -p bin
            cat > bin/rr << 'SCRIPT'
            #!/bin/sh
            rustrover "$PWD" &
            SCRIPT
            chmod +x bin/rr

            # Update .idea/workspace.xml with correct Rust toolchain paths
            IDEA_DIR="$PWD/.idea"
            WORKSPACE_FILE="$IDEA_DIR/workspace.xml"
            STDLIB_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
            TOOLCHAIN_BIN="${rustToolchain}/bin"

            mkdir -p "$IDEA_DIR"

            if [ -f "$WORKSPACE_FILE" ]; then
              # File exists - update or add RustProjectSettings
              if grep -q "RustProjectSettings" "$WORKSPACE_FILE"; then
                # Update existing settings
                ${pkgs.gnused}/bin/sed -i \
                  -e "s|explicitPathToStdlib\" value=\"[^\"]*\"|explicitPathToStdlib\" value=\"$STDLIB_PATH\"|" \
                  -e "s|toolchainHomeDirectory\" value=\"[^\"]*\"|toolchainHomeDirectory\" value=\"$TOOLCHAIN_BIN\"|" \
                  "$WORKSPACE_FILE"
              else
                # Add RustProjectSettings before closing </project> tag
                ${pkgs.gnused}/bin/sed -i \
                  "s|</project>|  <component name=\"RustProjectSettings\">\n    <option name=\"explicitPathToStdlib\" value=\"$STDLIB_PATH\" />\n    <option name=\"toolchainHomeDirectory\" value=\"$TOOLCHAIN_BIN\" />\n  </component>\n</project>|" \
                  "$WORKSPACE_FILE"
              fi
            else
              # Create new workspace.xml with RustProjectSettings
              cat > "$WORKSPACE_FILE" << EOF
            <?xml version="1.0" encoding="UTF-8"?>
            <project version="4">
              <component name="RustProjectSettings">
                <option name="explicitPathToStdlib" value="$STDLIB_PATH" />
                <option name="toolchainHomeDirectory" value="$TOOLCHAIN_BIN" />
              </component>
            </project>
            EOF
            fi
            
            export RUST_SRC_PATH=$STDLIB_PATH
          '';
        };
      }
    );
}

