{moduleWithSystem, ...}: {
  flake.aspects.devshells.dev = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [
      commitizen
      just
      rust-analyzer
    ];

    git.hooks = {
      enable = true;
      commit-msg.text = ''
        cz check $1
      '';
    };
  });
}
