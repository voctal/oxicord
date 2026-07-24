# Runs the CI workflow in local
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --workspace --all-features