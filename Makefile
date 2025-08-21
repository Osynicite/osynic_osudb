# Development workflow commands

.PHONY: quick check fmt fmt-check clippy test test-fast all pre-commit clean update release docs audit help

# Quick checks (fast feedback loop)
quick:
	@echo "🚀 Running quick checks..."
	@echo "1/3 Checking compilation..."
	@cargo check --all-features
	@echo "2/3 Checking code formatting..."
	@cargo fmt --all -- --check
	@echo "3/3 Running clippy..."
	@cargo clippy --all-features -- -D warnings
	@echo "✅ Quick checks passed!"

# Pre-commit checks (recommended before committing)
pre-commit: quick
	@echo "🧪 Running tests..."
	@cargo test --all-features
	@echo "✅ Pre-commit checks passed!"

# Run cargo check
check:
	cargo check --all-features

# Format the code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-features -- -D warnings

# Run tests
test:
	cargo test --all-features

# Run fast tests (without features)
test-fast:
	cargo test

# Run all checks (equivalent to CI)
all: check fmt-check clippy test
	@echo "✅ All checks passed!"

# Clean build artifacts
clean:
	cargo clean

# Update dependencies
update:
	cargo update

# Build for release
release:
	cargo build --release --all-features

# Run benchmarks if available
bench:
	cargo bench

# Generate documentation
docs:
	cargo doc --no-deps --open

# Security audit
audit:
	@command -v cargo-audit >/dev/null 2>&1 || { echo "Installing cargo-audit..."; cargo install cargo-audit; }
	cargo audit

# Show help
help:
	@echo "Available commands:"
	@echo "  quick      - Run quick checks (check + fmt-check + clippy)"
	@echo "  pre-commit - Run pre-commit checks (quick + test)"
	@echo "  check      - Run cargo check"
	@echo "  fmt        - Format the code"
	@echo "  fmt-check  - Check formatting"
	@echo "  clippy     - Run clippy"
	@echo "  test       - Run tests"
	@echo "  test-fast  - Run tests without features"
	@echo "  all        - Run all checks (equivalent to CI)"
	@echo "  clean      - Clean build artifacts"
	@echo "  update     - Update dependencies"
	@echo "  release    - Build for release"
	@echo "  docs       - Generate documentation"
	@echo "  audit      - Run security audit"
