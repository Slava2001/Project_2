@REM code format
cargo fmt --check || goto :error

@REM linter
cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings || goto :error

@REM doc
cargo clippy --all -- -W missing_docs -W clippy::missing_docs_in_private_items -W clippy::missing_errors_doc -W clippy::missing_panics_doc -D warnings || goto :error
cargo test --doc || goto :error

@REM tests
cargo test || goto :error

@REM build
cargo build --release || goto :error

goto :EOF

:error
echo Failed to build
exit /b %errorlevel%
