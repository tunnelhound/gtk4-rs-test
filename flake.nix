{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.; # Todo add protoc
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc cargo rust-analyzer protobuf3_20 rustfmt pkg-config
          ];

          buildInputs = with pkgs; [
            openssl.dev util-linux.dev pcre.dev
            gtk4.dev libadwaita.dev atk.dev  glib.dev pcre2.dev zlib.dev libffi.dev libselinux.dev
            xorg.libX11.dev freetype.dev fribidi libepoxy.dev
            harfbuzz.dev
            xorg.libXinerama.dev xorg.libXi.dev xorg.libXrandr.dev
            pixman libxkbcommon.dev
          ];
        };
      }
    );
}
