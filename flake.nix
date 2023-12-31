{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    ...
  }: let
    inherit (nixpkgs) lib;
    genSystems = lib.genAttrs [
      "aarch64-linux"
      "x86_64-linux"
    ];
    pkgsFor = nixpkgs.legacyPackages;
  in {
    devShells = genSystems (system: let
      pkgs = pkgsFor.${system};
    in {
      default = pkgs.mkShell rec {
        name = "prisma-timer";

        nativeBuildInputs = with pkgs; [
          cargo
          clippy
          rustc
          rustfmt
          rust-analyzer

          wrapGAppsHook4
          meson
          ninja
          pkg-config
        ];

        buildInputs = with pkgs; [
          cmake
          fontconfig
          gtk4
          glib
          libxml2
          libadwaita
          gdk-pixbuf
          gsettings-desktop-schemas
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        RUST_BACKTRACE = 1;
        RUST_LOG = "debug";

        shellHook = ''
          export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH:$XDG_DATA_DIRS"
          export NIX_SHELL_ACTIVE=${name}
        '';
      };
    });
  };
}
