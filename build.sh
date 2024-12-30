set -euxo pipefail

# code format
cargo fmt --check

# linter
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings

# doc
cargo clippy --all -- -W missing_docs -W clippy::missing_docs_in_private_items -W clippy::missing_errors_doc -W clippy::missing_panics_doc -D warnings
cargo test --doc

# tests
cargo test

# build
cargo build --release
