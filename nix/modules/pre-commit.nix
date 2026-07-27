{ inputs, ... }:
{
  imports = [
    (inputs.git-hooks + /flake-module.nix)
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    pre-commit.settings = {
      hooks = {
        cargo-readme = {
          enable = true;
          name = "cargo readme";
          entry = "${lib.getExe pkgs.cargo-readme} -o README.md";
          files = "^(src/lib\\.rs|README\\.md)$";
          pass_filenames = false;
        };

        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
      };
    };
  };
}
