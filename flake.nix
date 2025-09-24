{
  description = "Flake for Rimice, a widget system";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      allSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all;
      toSystems = passPkgs: allSystems (system: passPkgs (import nixpkgs { inherit system; }));
    in
    {
      devShells = toSystems (pkgs: {
        default = pkgs.mkShell {
          name = "Rimice";

          nativeBuildInputs = with pkgs.buildPackages; [
            pkg-config
            rustc
            cargo

            rust-analyzer
            rustfmt
            clippy
          ];

          buildInputs = with pkgs.buildPackages; [
            glib
            gtk4
          ];

          shellHook = ''
            export GTK_THEME=Adwaita
          '';
        };

        rimicore = pkgs.mkShell {
          name = "Rimicore";

          nativeBuildInputs = with pkgs.buildPackages; [
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
