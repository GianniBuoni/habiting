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

[arg("package", short="p")]
run package="server" *ARGS:
  cargo run -p habiting-{{package}} -- {{ARGS}}

@start_db:
  if [ "$(pg_ctl status | grep "is running")" ]; then \
    echo "PG server already running."; \
  else \
    pg_ctl start -l $PGDATA/logfile -o --unix_socket_directories=$PWD/$PGDATA; \
  fi;

@init_db:
  if [ "$(pg_ctl status | grep "is running")" ]; then \
    echo "PG server already initialized."; \
  else \
    mkdir -p $PGDATA; \
    pg_ctl init; \
    just start_db; \
    sqlx database create; \
    sqlx migrate run --source ./crates/habiting-server/migrations; \
  fi;

[working-directory: "crates/mathing-server"]
prepare:
  cargo sqlx prepare -- --all-targets
