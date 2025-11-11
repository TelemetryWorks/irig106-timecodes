# ROADMAP

## Licensing / Compliance
- [ ] Place SPDX header at the top of all source files  
- [ ] Add GitHub Action to enforce SPDX headers  
- [ ] Add a `SECURITY.md` with coordinated disclosure policy.  
- [ ] Add supply-chain compliance justification section
  (NIST SSDF, DoD SBOM requirements, internal governance)
- [ ] Add policy check for allowed/untrusted registries  
  (e.g., `allowed-registries` in `.cargo/config.toml`)

## Governance & Repository Integrity
- [ ] Add `CODEOWNERS` file.  
- [ ] Enable required reviews and branch protection rules.  
- [ ] Enforce signed commits and signed tags.  
- [ ] Add license scanning & auto-audit.  

## Reproducibility + Build Integrity
- [ ] Enable hermetic builds (no network access during build)  
- [ ] Configure `cargo vendor` and lock vendored crates in repo or artifact storage
- [ ] Update .cargo/config.toml and Cargo.toml to support:  
  **Reproducible Build Profiles**
  - [ ] Create a strict reproducible-builds profile bundle  
  **Security-Hardened Build Profiles**
  - [ ] Add hardened compilation settings  
        (PIE, RELRO, CET, shadow-call-stack, stack-protector, FORTIFY, LTO, etc.)
  **CI Integration**
  - [ ] Implement GitHub Actions matrix that exercises all build profiles  
  **Compliance Documentation**
  - [ ] Document rationale for hardened build flags  
        (NIST, DoD, internal security baselines)  
- [ ] `cargo fmt --all && cargo clippy -D warnings`  
- [ ] `cargo test --all-features`  

## Dependency & Supply-Chain Scanning
- [ ] Integrate dependency scanning in CI:
  - [ ] `cargo audit`  
  - [ ] `cargo deny`  
- [ ] Automate SBOM generation (`make sbom`)  
- [ ] Automate SBOM attestation upload to GitHub Releases  
- [ ] Add build-time provenance generation  
      (e.g., `slsa-framework/slsa-github-generator`)  

## CI / CD Hardening
- [ ] Harden GitHub Actions with:  
  - [ ] OIDC-based signing instead of long-lived secrets.  
  - [ ] Least-privilege workflow permissions.  
  - [ ] Dependency pinning for all third-party actions.  
- [ ] Add CI gate for formatting, linting, tests, and `-D warnings`  
- [ ] Prevent untrusted code from executing in PR builds.  
- [ ] Add job-level isolation and artifact integrity checks.  

## Documentation & Developer Experience
- [ ] Add `CONTRIBUTING.md` with workflow standards.  
- [ ] Add architecture diagram or ADRs for major decisions.  
- [ ] Add Makefile targets for onboarding (bootstrap scripts, lint, test, release).  
- [ ] Publish internal developer documentation (mdBook or GitHub Pages).  
- [ ] Add local development environment bootstrap script (optional suggestion).  

## Release Engineering & Artifact Security
- [ ] `make sbom && ./scripts/make_release.sh`  
- [ ] Optional: sign tarball with `cosign sign-blob`  
- [ ] Push tag `v0.1.0` and create first GitHub Release  
- [ ] Adopt semantic versioning policy.  
- [ ] Add CHANGELOG generation automation (`git-cliff`, `release-please`, etc.)  
- [ ] Automate published crate releases signed with cargo-crev or sigstore  
- [ ] Add release artifact integrity verification in CI  

## SLSA Progression
- [ ] Achieve **SLSA Level 1**  
- [ ] Achieve **SLSA Level 2**  
- [ ] Achieve **SLSA Level 3**  
- [ ] Achieve **SLSA Level 3 + CI Hardening**  
- [ ] (Optional) Plan for SLSA Level 4 requirements  

## Additional Recommended Improvements (Optional, High Value)
- [ ] Require MFA for all repo collaborators  
- [ ] Set up automated stale dependency alerts  
- [ ] Add periodic security review cadence (quarterly)  
- [ ] Add threat model document (STRIDE or similar)   
- [ ] Add automated license compliance checks  
- [ ] Add provenance verification step before consuming build artifacts  

