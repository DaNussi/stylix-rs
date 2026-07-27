{ inputs, ... }:
{
  imports = [
    (inputs.git-hooks + /flake-module.nix)
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    pre-commit.settings = {
      hooks = {
        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
        cargo-readme = {
          enable = true;
          name = "cargo-readme";
          description = "Generate the readme from src files.";
          files = "src/*";
          entry = "${pkgs.cargo-readme} > README.md";
        };

      };
    };
  };
}
