set shell := ["bash", "-cu"]
set windows-shell := ["pwsh", "-Command"]

# Default action
_:
    just lint
    just fmt
    just test

# Lint code
lint:
    ls-lint -config ./.ls-lint.yaml
    typos
    cargo check
    cargo clippy
    cargo test -p jder_axum -- --nocapture

# Format code
fmt:
    cargo fmt

# Run test
test:
    cargo test -p test -- --nocapture

# Run test server
server:
    cargo run -p test

# Publish package as dry-run
publish-try:
    cd ./crate && cargo publish --dry-run

# Publish package
publish:
    cd ./crate && cargo publish

# Clean
clean:
    cargo clean
