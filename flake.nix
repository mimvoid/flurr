{
  description = "Flake for Flurr, a widget system";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      allSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all;
      toSystems = passPkgs: allSystems (system: passPkgs (import nixpkgs { inherit system; }));
    in
    {
      devShells = toSystems (pkgs: {
        default = pkgs.mkShell {
          name = "Flurr";

          packages = with pkgs; [
            pkg-config
            rustc
            cargo

            rust-analyzer
            rustfmt
            clippy

            glib
            gtk4
            gtk4-layer-shell
          ];

          shellHook = ''
            export GTK_THEME=Adwaita
          '';
        };

        core = pkgs.mkShell {
          name = "Flurr-core";

          packages = with pkgs; [
            meson
            ninja
            vala
            gcc
            vala-language-server
          ];
        };
      });
    };
}
