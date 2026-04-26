.PHONY: all fmt clippy build test examples ci

all: fmt clippy build test examples

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

build:
	cargo build --all --examples --verbose

test:
	cargo test --all --verbose

examples:
	@for ex in examples/*.rs; do \
		name=$$(basename $$ex .rs); \
		echo "Running example: $$name"; \
		cargo run --example $$name || true; \
	done

ci: fmt clippy build test examples
