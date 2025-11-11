.PHONY: all fmt lint test bench deny audit sbom dist

all: fmt lint test

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -D warnings

test:
	cargo test --all-features

bench:
	cargo bench

deny:
	cargo deny check --all

audit:
	cargo audit --db tools/advisory-db || true

sbom:
	cargo install cargo-cyclonedx --locked || true
	cargo cyclonedx -o target/sbom.cdx.json

dist:
	./scripts/make_release.sh
