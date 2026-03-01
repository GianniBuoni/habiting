{
  flake.aspects.devshells.lint.commands = [
    {
      name = "enterTest";
      help = "Test to check lint shell has all necessary tooling";
      command = ''
        cargo -V
        cargo clippy -V
        just -V
        protoc --version
      '';
    }
  ];
}
