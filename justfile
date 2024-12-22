# Default recipe, when nothing's selected
[private]
default:
  just --list --list-submodules

# Build kdl-fmt with the tracing feature enabled.
build:
  cargo build

# Run kdl-fmt with log level set to debug and some default arguments
run args="":
  RUST_LOG=debug cargo run -p kdl-fmt -- ./tests/config.kdl {{args}}

# Watch and run tests with nextest.
test:
  RUST_LOG=trace cargo watch -x "nextest run --lib"

# Lint with clippy and cargo audit.
lint:
  cargo clippy --all-features --lib
  cargo audit

# Create and push a new release version.
release:
  #!/usr/bin/env bash
  export VERSION="$( git cliff --bumped-version )"
  cargo set-version "${VERSION:1}"
  direnv exec . cargo build --release
  git commit -am "chore: bump version to $VERSION"
  git tag -m "$VERSION" "$VERSION"
  git push origin main
  git push origin "$VERSION"
