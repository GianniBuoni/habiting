{moduleWithSystem, ...}: let
  mkEnv = name: value: {inherit name value;};
in {
  flake.aspects.devshells.build = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [sqlx-cli postgresql];

    env = [
      (mkEnv "PGDATA" ".postgres")
      (mkEnv "DATABASE_URL" "postgres://[::1]:5432/habiting")
    ];

    commands = [
      {
        name = "enterTest";
        help = "Test to check if build shell has all necessary tooling";
        command = ''
          cargo -V
          cargo clippy -V
          just -V
          pg_ctl -V
          protoc --version
          sqlx -V
        '';
      }
    ];
  });
}
