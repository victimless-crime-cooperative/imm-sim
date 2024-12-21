{
  description = "Bevy DevShell";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-utils, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        libs = with pkgs; [
          alsa-lib
          libxkbcommon
          udev
          vulkan-loader
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            # Rust build inputs
            openssl
            pkg-config

            # Rust and LSP
            rust-analyzer
            rust-bin.beta.latest.default

            # Bevy dependencies
            alsa-lib
            libxkbcommon
            udev
            vulkan-loader
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

          shellHook = ''
            LD_LIBRARY_PATH=${lib.makeLibraryPath libs}
          '';
        };
      }
    );
}
