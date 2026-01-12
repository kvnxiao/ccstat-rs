install:
    cargo install --path .

build:
    cargo build --release

# Use nightly for linting and formatting
lint:
    cargo +nightly clippy --workspace --all-targets --all-features
    cargo +nightly fmt --check

fix:
    cargo +nightly clippy --workspace --all-targets --all-features --fix --allow-dirty
    cargo +nightly fmt --all
