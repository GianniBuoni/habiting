{moduleWithSystem, ...}: {
  flake.aspects.devshells.base = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [grpc-tools just];
  });
}
