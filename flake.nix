{
  description = "Flake for Flurr, a widget system";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      allSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all;
      toSystems = passPkgs: allSystems (system: passPkgs (import nixpkgs { inherit system; }));
    in
    {
      packages = toSystems (pkgs: import ./nix/pkgs { inherit pkgs; });

      devShells = toSystems (pkgs: {
        default = pkgs.mkShell {
          name = "Flurr";

          nativeBuildInputs = [ pkgs.pkg-config ];
          packages = with pkgs; [
            glib
            gobject-introspection
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

        rust = pkgs.mkShell {
          name = "Flurr-rust";
          buildInputs = [ pkgs.dbus ];
          packages = with pkgs; [
            rustc
            cargo

            rust-analyzer
            rustfmt
            clippy
          ];
        };
      });
    };
}
