default: check

# Fast compile check of the workspace
check:
    cargo check --workspace

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

test *args:
    cargo test --workspace {{args}}

# Lints run before each commit
precommit:
    cargo fmt --all --check
    just clippy

# Checks run before each push
prepush: precommit test

# Install the `exegenius` binary from this checkout
install:
    cargo install --path exegenius-cli --locked

docs:
    cargo doc --workspace --no-deps --document-private-items
