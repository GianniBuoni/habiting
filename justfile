[arg("package", short="p")]
build package="": (test package)
  if [ "{{package}}" = "" ]; then \
    cargo build; \
  else \
    cargo build -p habiting-{{package}}; \
  fi

[arg("package", short="p")]
test package="":
  if [ "{{package}}" = "" ]; then \
    just _init_db; \
    cargo test; \
  else \
    cargo build -p habiting-{{package}}; \
  fi

[arg("package", short="p")]
lint package="":
  if [ "{{package}}" = "" ]; then \
    cargo fmt --check; \
    cargo clippy --all-targets -- -D warnings; \
  else \
    cargo fmt --check; \
    cargo clippy -p habiting-{{package}} -- -D warnings; \
  fi

run package="server" *ARGS:
  cargo run -p habiting-{{package}} -- {{ARGS}}

@_start_db:
  if [ "$(pg_ctl status | grep "no server running")" = "" ]; then \
    echo "PG server already running."; \
  else \
    pg_ctl start -l $PGDATA/logfile -o --unix_socket_directories=$PWD/$PGDATA; \
  fi;

@_init_db:
  if [ "$(pg_ctl status)" ]; then \
    just _start_db; \
  else \
    pg_ctl init; \
    just _start_db; \
    sqlx database create; \
    sqlx migrate run --source ./crates/habiting-server/migrations; \
  fi;

[working-directory: "crates/habiting-server"]
prepare:
  cargo sqlx prepare -- --all-targets
