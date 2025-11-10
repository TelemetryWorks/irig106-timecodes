# Security Policy

This document defines the security reporting and response process for this project.  
It outlines contact methods, disclosure expectations, support commitments, and the security controls implemented across the codebase and build pipeline.

## 1. Reporting a Vulnerability

We strongly prefer coordinated, private disclosure.

### 1.1 Contact Method
Report suspected vulnerabilities using one of the following:
- **Primary**: <security-contact@example.com>  
- **Fallback**: GitHub private security advisory (Security → Advisories → Report a vulnerability)

Do not file public GitHub issues for security matters.

### 1.2 Required Information
Include, when possible:
- Description of the vulnerability
- Affected components or versions
- Steps to reproduce
- Potential impact
- Proof of concept or exploit scenario (if applicable)

### 1.3 Disclosure Expectations
We request that researchers:
- Allow maintainers **up to 7 days** to acknowledge the report
- Provide a **30-day minimum** remediation window before public disclosure
- Coordinate on publication details once a fix is available

We do not pursue legal action against good-faith security research.

## 2. Supported Versions

Security updates are provided for:
- The most recent stable release
- The default development branch

Older releases are addressed on a best-effort basis depending on severity and feasibility.

## 3. Security Posture and Controls

This project maintains multiple layers of security controls designed to limit attack surface, reduce supply-chain risk, and maintain integrity of released artifacts.

### 3.1 Code Safety Controls
- Strict enforcement of `#![forbid(unsafe_code)]`
- Mandatory CI gates:
  - `rustfmt`
  - `clippy` with `-D warnings`
  - Full test suite with coverage checks
  - `cargo-deny` policy enforcement
  - `cargo-audit` scans for vulnerable dependencies
  - Semgrep static analysis
  - GitHub CodeQL advanced security analysis

### 3.2 Supply Chain Integrity
- CycloneDX SBOM generation for each release
- Automated, scripted builds executed in GitHub Actions (SLSA Level 1)
- Provenance metadata generated and published for each release
- Reproducible or deterministic builds where supported
- Pinned and reviewed toolchain versions (`rust-toolchain.toml`)
- Pinned direct dependencies with regular audit of transitive ones
- Mandatory review on dependency changes (automated PRs allowed, not auto-merged)

### 3.3 Release Security
- Cryptographically signed release artifacts using trusted keys or Sigstore
- Published checksums for all artifacts
- Provenance attestation made available per release
- Verification steps in CI prior to publishing

### 3.4 CI and Infrastructure Controls
- Least-privilege tokens and short-lived credentials for all CI workflows
- Immutable or pinned CI build environments
- Branch protection rules enforced on main branches
- Signed commits required for maintainers (encouraged for contributors)

## 4. Incident Response Process

If a security incident is confirmed:
1. Triage severity and scope
2. Develop and test a patch or mitigation
3. Notify the reporter of progress
4. Release fixed versions and advisory notes
5. Update SBOM and provenance metadata
6. Publish a coordinated security advisory with CVE assignment if appropriate

## 5. Commitment to Researchers

We value responsible security research and commit to:
- Respectful, timely communication
- Proper attribution where desired
- No legal retaliation for good-faith reports
