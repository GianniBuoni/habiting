{moduleWithSystem, ...}: let
  mkEnv = name: value: {inherit name value;};
in {
  flake.aspects.devshells.dev = moduleWithSystem ({pkgs, ...}: {
    packages = with pkgs; [
      commitizen
      rust-analyzer
    ];

    env = [
      (mkEnv "LOG_LEVEL" "info")
      (mkEnv "HABITING_URI" "[::1]:50051")
    ];

    git.hooks = {
      enable = true;
      commit-msg.text = "cz check --commit-msg-file $1";
      pre-commit.text = "just lint && just";
      pre-push.text = ''
        if [ "$(git rebase origin/main | grep "up to date")" = "" ]; then
          exit 1;
        else
          exit 0;
        fi
      '';
    };
  });
}
