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

# pre-commit.ci sets PRE_COMMIT_HOME and has no network, so it runs only the offline checks
ci_opt := if env("PRE_COMMIT_HOME", "") != "" { "-ci" } else { "" }

# Lints run before each commit
precommit:
    {{just_executable()}} pc{{ci_opt}}

pc: pc-ci clippy

pc-ci:
    cargo fmt --all --check

# Checks run before each push
prepush: precommit test

# Install the `exegenius` binary from this checkout
install:
    cargo install --path exegenius-cli --locked

docs:
    cargo doc --workspace --no-deps --document-private-items
