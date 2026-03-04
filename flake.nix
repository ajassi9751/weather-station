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
      runtimeInputs = with pkgs; [ cowsay ];
      text = ''
        ${pkgs.cowsay}/bin/cowsay "Hi from nix run"
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
 
    packages.x86_64-linux.default = pkgs.writeShellScriptBin "default" ''
      ${pkgs.cowsay}/bin/cowsay "Hello from a Nix build!"
    '';

    apps.x86_64-linux.default = {
      program = "${run64}/bin/run64";
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
