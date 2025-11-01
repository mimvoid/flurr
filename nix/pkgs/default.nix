{ pkgs }:

rec {
  flurrdbus = pkgs.callPackage ./flurrdbus.nix { };
  flurr = pkgs.callPackage ./flurr.nix { inherit flurrdbus; };
  flurrctl = pkgs.callPackage ./flurrctl.nix { };
}
