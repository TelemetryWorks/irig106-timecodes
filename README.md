# irig106-timecodes
Spec-aligned IRIG-106 timecode parsing/formatting engine with precise encode/decode logic and full conformance traceability.  
Pure Rust, `#![forbid(unsafe_code)]`, with strong supply-chain and code analysis.  


| Component            | Path                    | What It’s For                                   |
|----------------------|-------------------------|-------------------------------------------------|
| Conformance suite    | `conformance_ch10/`     | Spec compliance tests                           |
| Conformance suite    | `conformance_ch11/`     | Spec compliance tests                           |
| Criterion benches    | `benches/parse_bench.rs`| Performance benchmarks for the parser           |
| Fuzz harness         | `fuzz/`                 | Fuzzing setup for robustness and crash detection|


See **docs/** (mdBook) for the secure development & assurance report.

## Quick start

```bash
cargo build
cargo test
cargo bench
```

## Security Audit

This project uses a local advisory database for dependency/security auditing.

### Configuration
The audit config lives in `audit.toml`:

```toml
[database]
path = "tools/advisory-db"
```
* `path` points to a local checkout/clone of the advisory database.

### Getting the advisory database
```bash
git clone <ADVISORY_DB_REPO_URL> tools/advisory-db
```
> Replace <ADVISORY_DB_REPO_URL> with the repository that contains the advisories for your ecosystem.

### Running the audit

Run your audit command (examples; adjust to your toolchain):
```bash
# Example placeholder — replace with your actual audit command
make audit
# or
./scripts/audit.sh
# or
your-audit-tool --config audit.toml
```

### Keeping the database up to date

Update the advisory DB periodically:

```bash
(cd tools/advisory-db && git fetch --all && git pull)
```

### CI notes
* Ensure the advisory DB exists at the configured path before running the audit step.  
* If you cache the DB in CI, persist `tools/advisory-db/` (or `.advisory-db/`) between runs.

