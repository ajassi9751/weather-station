{
  description = "Nix flake for the GHSweather station for the geo/yci club";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs64 = import nixpkgs { system = "x86_64-linux"; };
    pkgsArm = import nixpkgs { system = "aarch64-linux"; };
  in {
    packages.x86_64-linux.default = pkgs64.rustPlatform.buildRustPackage rec {
      pname = "weather-station";
      version = "0.1.0";

      src = pkgs64.lib.cleanSource ./.;

      cargoLock.lockFile = ./Cargo.lock;

      buildFeatures = ["no_pi"];

      # defaultFeatures = false;
    };

    devShells.x86_64-linux.default = pkgs64.mkShell {
      # Starts zsh as the default shell
      shellHook = ''
        exec zsh
      '';
      buildInputs = with pkgs64; [
        rustc
        cargo
        rustfmt
        rust-analyzer
        zsh
      ];
    };
 
    # Use for raspberry pi
    packages.aarch64-linux.default = pkgsArm.rustPlatform.buildRustPackage rec {
      pname = "weather-station";
      version = "0.1.0";

      src = ./.;

      cargoLock.lockFile = ./Cargo.lock;

      buildInputs = with pkgsArm; [ wiringpi ];
    };
    
    devShells.aarch64-linux.default = pkgsArm.mkShell {
      # Starts zsh as the default shell
      shellHook = ''
        exec zsh
      '';
      buildInputs = with pkgsArm; [
        rustc
        cargo
        rustfmt
        rust-analyzer
        zsh
        wiringpi
      ];
    };
  };
}
