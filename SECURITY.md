# Security Policy

This project takes security and supply-chain integrity seriously.  
If you believe you’ve found a vulnerability, please follow the guidelines below.

## Reporting a Vulnerability

If you discover a security issue:

1. **Do not open a public issue.**
2. Email the maintainers at `<add-your-contact-here>` with:
   - A description of the issue
   - Steps to reproduce (if applicable)
   - Any relevant proof-of-concept material

We will acknowledge your report within **7 days** and provide updates as we investigate.

If you cannot find a dedicated security contact, you may also use GitHub’s private security advisories feature.

## Supported Versions

Security fixes are provided for:
- The latest release
- The development branch

Older releases may receive fixes on a best-effort basis.

## Project Security Features

See `CODE_ANALYSIS.md` and `SUPPLYCHAIN.md` for full technical details.  
Key highlights of our security posture:

### Code Safety
- `#![forbid(unsafe_code)]` enforced throughout the codebase.
- CI gates:
  - `rustfmt`
  - `clippy` with `-D warnings`
  - test suite
  - `cargo-deny`
  - `cargo-audit`
  - Semgrep
  - CodeQL analysis

### Supply Chain & Build Integrity
- SBOM generation (CycloneDX)
- Release provenance metadata
- Deterministic or reproducible builds where possible
- Signed release artifacts

## Responsible Disclosure

We request a **reasonable amount of time** to investigate and remediate before any public disclosure.

Thank you for contributing to a safer ecosystem.
