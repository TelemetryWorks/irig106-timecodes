# Code Analysis & Hardening

This document defines the *required* static checks, tests, and release-time integrity steps for this repository. It is Rust-specific and aligns with our SLSA Level 1 baseline.

## Goals
- Catch correctness & security issues before merge.
- Keep dependencies healthy and policy-compliant.
- Produce traceable releases (SBOM + provenance) and signed artifacts.

## 1. Per-PR Required Checks

Run locally and in CI; PRs must pass all of these.

### 1.1 Formatting
```bash
cargo fmt --all -- --check
```

### 1.2 Lints (deny warnings)
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
***Policy***: Prefer `clippy::pedantic` where practical; allow per-line `#[allow]` with justification.  

### 1.3 Tests + minimal coverage
```bash
cargo test --all --all-features
```
If using coverage (recommended):
```bash
# Linux example with llvm-cov
cargo llvm-cov --workspace --all-features --lcov --output-path target/lcov.info
# Enforce a floor (tune threshold):
cargo llvm-cov --fail-under-lines 70
```

### 1.4 Dependency Policy: `cargo-deny`

```bash
cargo deny check licenses bans advisories sources
```
* licenses: must be OSI-approved or whitelisted.
* bans: forbid duplicated critical crates or disallowed crates.
* advisories: no unresolved vulnerability advisories.
* sources: only allowed registries (e.g., crates.io) and pinned git sources.

### 1.5 Vulnerability Scan: `cargo-audit`

```bash
cargo audit
```
Fail on known vulnerabilities unless an ***expiring*** ignore is documented.

### 1.6 Semgrep (optional but recommended)

```bash
semgrep scan --config p/r2c-security-audit --error
```

Pin rule packs in CI for determinism; document local overrides.  

### 1.7 CodeQL (recommended)  

Run GitHub Advanced Security CodeQL on PRs and weekly; treat new alerts as blockers.  

### 1.8 Unsafe Code Policy 
* Crates set `#![forbid(unsafe_code)]` by default.
* If unsafe is necessary, isolate in a small module with:
  * Extensive documentation of invariants
  * Unit tests and `cargo miri test` where applicable.

---

## 2. Hardening (Continuous)

### 2.1 Miri (UB checks)

```bash
cargo +nightly miri setup
cargo +nightly miri test
```

### 2.2 Sanitizers (where supported)

```bash
RUSTFLAGS="-Z sanitizer=address" \
RUSTDOCFLAGS="-Z sanitizer=address" \
cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

### 2.3 Fuzzing (critical parsers/logic)

```bash
cargo fuzz run <target> -- -runs=0
```

---

## 3. Release-Time Integrity (SLSA L1)

On tagged releases:  

### 3.1 SBOM (CycloneDX)

```bash
cargo install cyclonedx-bom --locked
cyclonedx-bom -o target/sbom.json
```

### 3.2 Provenance (documented, repeatable build)
* Build with a pinned toolchain (`rust-toolchain.toml`) and reproducible flags where possible:

```bash
export SOURCE_DATE_EPOCH="$(git log -1 --pretty=%ct)"
RUSTFLAGS="-C link-arg=-Wl,--build-id=none -C debuginfo=0"
cargo build --release --locked
```

* Capture build metadata (builder, commit, inputs, outputs) into `docs/PROVENANCE.md` and attach to the release.  

### 3.3 Signatures & Checksums

```bash
shasum -a 256 target/release/<bin> > target/release/<bin>.sha256
gpg --detach-sign --armor target/release/<bin>
```
(or Sigstore/cosign if you prefer keyless signing).

---

## 4. Local Developer Shortcuts

Common `make` targets (optional but encouraged):

```Makefile
fmt:        ; cargo fmt --all
lint:       ; cargo clippy --all-targets --all-features -- -D warnings
test:       ; cargo test --all --all-features
audit:      ; cargo deny check licenses bans advisories sources && cargo audit
miri:       ; cargo +nightly miri test
coverage:   ; cargo llvm-cov --workspace --all-features --fail-under-lines 70
sbom:       ; cyclonedx-bom -o target/sbom.json
release-prep: fmt lint test audit sbom
```
For offline checks:  
```bash
make fmt lint test
cargo deny check --offline
cargo audit --db ~/.cargo/advisory-db # if you maintain a local mirror
```

---

## 5. Exceptions & Waivers
* Allowed only with:
  * Link to issue explaining risk and timeline
  * Scoped `#[allow(...)]` or tool config ignore with **expiry date**
* Security exceptions require maintainer approval.

## 6. Example CI Outline (GitHub Actions)
(YAML abbreviated; adapt to your repo)
```yaml
name: CI
on:
  pull_request:
  push:
    branches: [main]

jobs:
  checks:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      security-events: write   # for CodeQL (if enabled)
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: rustfmt, clippy }
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all --all-features --locked
      - uses: EmbarkStudios/cargo-deny-action@v2
      - uses: actions-rs/audit-check@v1
        with: { token: ${{ secrets.GITHUB_TOKEN }} }
      # Optional:
      # - uses: github/codeql-action/init@v3
      # - uses: github/codeql-action/analyze@v3
      # - run: semgrep scan --config p/r2c-security-audit --error
```

## 7. Artifacts to Keep

* `target/lcov.info` (coverage)
* `target/sbom.json` (SBOM)
* `docs/PROVENANCE.md` capturing build inputs/outputs for each release
* Release checksums and signatures

