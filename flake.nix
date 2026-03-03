{
  description = "Code for the GHSweather station for the geo/yci club";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux; # Adjust system if needed
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustc
        cargo
        rustfmt
      ];
    };
  };
}