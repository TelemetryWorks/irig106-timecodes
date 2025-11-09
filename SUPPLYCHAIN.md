# Supply Chain

This project aims for a clean, auditable, and reproducible supply chain.  
Below are the key practices we follow (and expect contributors to respect).

## Tooling and Dependencies
- Pin toolchain versions to avoid drift across environments.
- Pin all direct dependencies; regularly review transitive ones.
- Track and update dependency advisories (automated audit recommended).

## Build Integrity
- Generate SBOMs for each release build.
- Produce build provenance metadata (SLSA-style where possible).
- Ensure builds are reproducible or verifiable across environments.

## Artifact Security
- Sign all release artifacts with a trusted key or Sigstore.
- Publish checksums and signatures alongside each release.
- Verify signatures in CI before consumption.

## Repository Hygiene
- Protect main branch with required checks (linting, testing, audit).
- Require reviewed, signed commits for contributors (optional but recommended).
- Use dependency update automation with review (Dependabot, Renovate, etc.).

## CI and Release Pipeline
- Use minimal-permission tokens for workflows.
- Keep CI environment definitions pinned and reproducible.
- Gate releases on successful audit, tests, and signature steps.
