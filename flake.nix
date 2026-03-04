{
  description = "Code for the GHSweather station for the geo/yci club";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
    armpkgs = nixpkgs.legacyPackages.arm-linux;
    run64 = pkgs.writeShellApplication {
      name = "run64";
      runtimeInputs = with pkgs; [ cargo rustc ];
      text = ''
        ${pkgs.cargo}/bin/cargo run --features "no_pi"
      '';
    };
    runArm = pkgs.writeShellApplication {
      name = "runArm";
      runtimeInputs = with pkgs; [ cargo rustc ];
      text = ''
        ${pkgs.cargo}/bin/cargo run
      '';
    };
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      # Starts zsh as the default shell
      shellHook = ''
        exec zsh
      '';
      buildInputs = with pkgs; [
        rustc
        cargo
        rustfmt
        rust-analyzer
        zsh
      ];
    };
 
    # This definitely doesnt work becuase it should've failed when compiling for arm but it didn't
    packages.x86_64-linux.default = pkgs.writeShellScriptBin "default" ''
      ${pkgs.cargo}/bin/cargo build --features "no_pi"
    '';

    apps.x86_64-linux.default = {
      program = "${run64}/bin/run64";
      type = "app";
    };

    apps.arm-linux.default = {
      program = "${runArm}/bin/runArm";
      type = "app";
    };

    # Use for raspberry pi
    devShells.arm-linux.default = pkgs.mkShell {
      # Starts zsh as the default shell
      shellHook = ''
        exec zsh
      '';
      buildInputs = with armpkgs; [
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
