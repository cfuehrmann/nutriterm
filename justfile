# ==============================================================================
# Development Workflow:
#   Iterating on code:  just test     (fast feedback on logic)
#   Before committing:  just check    (all quality gates)
#   Fix formatting:     just fix
# ==============================================================================

# Run all quality gates: fmt -> clippy -> spell -> test (for pre-commit)
check: fmt clippy spell test

# Run tests
test:
    cargo test

# Check spelling
spell:
    typos

# Run clippy lints
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Check code formatting
fmt:
    cargo fmt --all -- --check

# Auto-fix formatting issues
fix:
    cargo fmt --all

# Build optimized release binary (runs checks first)
release: check
    cargo build --release

# Build release binary only (no checks, for CI)
release-build:
    cargo build --release

# Remove build artifacts
clean:
    cargo clean
