{
  description = "Nix flake for the GHSweather station for the geo/yci club";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    # Old version of nixpgks for neofetch
    nixpkgs-old.url = "github:nixos/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs, nixpkgs-old }: let
    # Nixpkgs stuff
    pkgs-old64 = import nixpkgs-old { system = "x86_64-linux"; };
    pkgs-oldArm = import nixpkgs-old { system = "aarch64-linux"; };
    pkgs64 = import nixpkgs { system = "x86_64-linux"; };
    pkgsArm = import nixpkgs { system = "aarch64-linux"; };
    # File source stuff
    fs64 = pkgs64.lib.fileset;
    fsArm = pkgsArm.lib.fileset;
    # Include files c and rust files for the derivation
    sourceFiles64 = fs64.unions [ ./c ./src ./Cargo.toml ./Cargo.lock ];
    sourceFilesArm = fsArm.unions [ ./c ./src ./Cargo.toml ./Cargo.lock ];
  in {
    packages.x86_64-linux.default = pkgs64.rustPlatform.buildRustPackage rec {
      pname = "weather-station";
      version = "0.1.0";

      # src = ./.;

      src = fs64.toSource {
        root = ./.;
        fileset = sourceFiles64;
      };

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
        gh
        pkgs-old64.neofetch
      ];
    };
 
    # Use for raspberry pi
    packages.aarch64-linux.default = pkgsArm.rustPlatform.buildRustPackage rec {
      pname = "weather-station";
      version = "0.1.0";

      # src = ./.;

      src = fsArm.toSource {
        root = ./.;
        fileset = sourceFilesArm;
      };     

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
        gh
	      tmux
        pkgs-oldArm.neofetch
	      # yazi
	      # btop
      ];
    };
  };
}
