{
  inputs,
  config,
  ...
}: {
  flake-file.inputs.devshell.url = "github:numtide/devshell";
  imports = [inputs.devshell.flakeModule];

  perSystem = {self', ...}: {
    devShells.default = self'.devShells.dev;
    devshells = {
      # development shell with full tooling
      dev = {extraModulesPath, ...}: {
        imports = [
          "${extraModulesPath}/language/rust.nix"
          "${extraModulesPath}/language/c.nix"
          "${extraModulesPath}/git/hooks.nix"
          config.flake.aspects.devshells.dev
        ];
      };
      # ci shell with only the tooling necessary for linting
      lint = {extraModulesPath, ...}: {
        imports = [
          "${extraModulesPath}/language/rust.nix"
          "${extraModulesPath}/language/c.nix"
        ];
      };
      # ci shell with only the tooling necessary for testing and building
      ci = {extraModulesPath, ...}: {
        imports = [
          "${extraModulesPath}/language/rust.nix"
          "${extraModulesPath}/language/c.nix"
        ];
      };
    };
  };
}
