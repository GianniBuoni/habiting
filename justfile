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
