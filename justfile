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
    cargo clippy --all-targets -- -D warnings; \
  else \
    cargo clippy -p habiting-{{package}} -- -D warnings; \
  fi
