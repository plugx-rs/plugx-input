build:
	@set -e; \
	echo ">>> build: no features"; \
	cargo build --no-default-features; \
	echo ">>> build: serde"; \
	cargo build --no-default-features --features=serde; \
	echo ">>> build: rkyv"; \
	cargo build --no-default-features --features=rkyv; \
	echo ">>> build: serde,rkyv"; \
	cargo build --no-default-features --features=serde,rkyv

check-style:
	cargo fmt --check --verbose

clippy:
	cargo clippy --all-features --no-deps

test:
	@set -e; export RUST_BACKTRACE=1; \
	echo ">>> test: serde,rkyv"; \
	cargo test --no-default-features --features=serde,rkyv; \
	echo ">>> test: all-features"; \
	cargo test --all-features

doc:
	cargo doc --all-features
