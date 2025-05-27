.PHONY: test test-policy test-doc lint fmt check all

# Default target: run full test + lint + fmt
all: check

# Run default tests
test:
	cargo test --all

# Run tests with policy feature enabled
test-policy:
	cargo test --all --features policy

# Run doctests (docs + #[doc] code blocks)
test-doc:
	cargo test --doc --features policy

# Run clippy with all targets/features (fail on warnings)
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Check rustfmt formatting
fmt:
	cargo fmt --all -- --check

# One command to run full CI logic locally
check: lint fmt test test-policy test-doc
