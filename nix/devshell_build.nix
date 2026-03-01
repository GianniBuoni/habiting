{moduleWithSystem, ...}: {
  flake.aspects.devshells.build = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [sqlx-cli];

    commands = [
      {
        name = "enterTest";
        help = "Test to check if build shell has all necessary tooling";
        command = ''
          cargo -V
          cargo clippy -V
          just -V
          protoc --version
          sqlx -V
        '';
      }
    ];
  });
}
