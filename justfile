set shell := ["bash", "-cu"]
set windows-shell := ["pwsh", "-Command"]

crate := "jder_axum"

tst := "test"

# Default action
_:
    just --list -u

# Format code
fmt:
    cargo fmt

# Lint code with ls-lint
ls-lint:
    ls-lint -config ./.ls-lint.yaml

# Lint code with ls-lint
lslint: ls-lint

# Lint code with typos
typos:
    typos

# Lint code
lint:
    cargo clippy

# Run test for doc
test-doc:
    cargo test -p {{crate}} -- --nocapture

# Run test
test:
    cargo test -p {{tst}} -- --nocapture

# Run test server
server:
    cargo run -p {{tst}}

# Check code
check: fmt ls-lint typos lint test-doc test

# Publish package as dry-run
publish-try:
    cargo publish -p {{crate}} --dry-run

# Publish package
publish:
    cargo publish -p {{crate}}

# Clean
clean:
    cargo clean
