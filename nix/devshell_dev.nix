{moduleWithSystem, ...}: {
  flake.aspects.devshells.dev = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [
      commitizen
      rust-analyzer
    ];

    git.hooks = {
      enable = true;
      commit-msg.text = "cz check --commit-msg-file $1";
      pre-commit.text = "just lint && just";
    };
  });
}
