# SLSA L1

Starting at **SLSA Level 1 (v1.0)** is the right move. Here’s exactly what to do for a Rust repo, how to prove it, and how to add a badge.

### What SLSA L1 means

* **Generate provenance** for your build artifacts (who/what/how produced them).
* **Make the provenance available** to consumers (attach to releases or keep it visible in the repo/registry).
  * [Producing artifacts](https://slsa.dev/spec/v1.1/requirements)  
  * [Security levels](https://slsa.dev/spec/v1.0/levels)  

> At L1 the bar is: your builds are scripted/automated and you produce & publish provenance. It doesn’t yet defend against strong tampering, but it’s the foundation for higher levels.  

### Rust + GitHub Actions: minimal, practical setup  

Add a workflow that builds your Rust binaries, packages them, and **generates a signed provenance attestation** using GitHub’s official action.

```yaml
# .github/workflows/release-provenance.yml
name: release (build + provenance)
on:
  push:
    tags: ["v*.*.*"]   # tag a release like v1.2.3
permissions:
  contents: read
  id-token: write
  attestations: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Set up Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Build release binaries
        run: |
          cargo build --locked --release
          mkdir -p dist
          cp target/release/* dist/ || true
          # (Optional) package per-target; add your exact binary names if you prefer
          tar -czf dist/artifacts.tar.gz -C dist .

      - name: Generate provenance attestation (binaries)
        uses: actions/attest-build-provenance@v3
        with:
          subject-path: dist/**

      # (Optional) Upload artifacts so they’re attached to the run / release
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: rust-build
          path: dist/**
```
* `actions/attest-build-provenance@v3` creates **in-toto/SLSA provenance** and signs it via Sigstore with your job’s OIDC identity. Attestations are visible in the repo’s **Actions → Attestations** and can be linked or downloaded.
  * [GitHub Action: Build Attestation](https://github.com/actions/attest-build-provenance)  
  * [Using artifact attestations to establish provenance for builds](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations)  

> You can also use GitHub’s artifact attestations in container workflows or pair with other builders later to reach L3.
> [Using artifact attestations and reusable workflows to achieve SLSA v1 Build Level 3](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/increase-security-rating)  

### How to prove SLSA L1

1. **Point to the attestation** produced by the workflow (from the Actions “Attestations” tab or attach the `.intoto.jsonl` file to your GitHub Release). L1 requires that provenance exists and is distributed.
  [Using artifact attestations to establish provenance for builds](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations)  

2. **Let others verify** with the GitHub CLI:
```bash
# Example: verify a release artifact's provenance
gh attestation verify \
  --repo <owner>/<repo> \
  --subject dist/artifacts.tar.gz
```
This checks the attestation signature and ties it to your repo identity. Include the command in your README’s “Verify” section.
[Using artifact attestations and reusable workflows to achieve SLSA v1 Build Level 3](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/increase-security-rating)  

### Add a badge to your README

There isn’t an official, auto-validated SLSA badge service yet, but the SLSA quickstart explicitly suggests adding a Level 1 badge once you meet the requirements. Until there’s a canonical badge, use a Shields.io static badge that links to your provenance/verification docs or the Actions Attestations page. [Reaching SLSA Level 1](https://devmoran.github.io/slsa/get-started)  

```md
[![SLSA Level 1](https://img.shields.io/badge/SLSA-Level%201-blue)](./docs/PROVENANCE.md)
```

Create `docs/PROVENANCE.md` with:  
* Where the attestation lives (Actions → Attestations or attached to Releases)  
* How to verify (the `gh attestation verify` command above)  
* A note that the repo targets SLSA v1.0 L1 today, with a roadmap to L3.  
  * [Using artifact attestations to establish provenance for builds](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations)  

### Optional: nicer “status” badges

You can also add:  
* A workflow status badge for the release/provenance workflow.  
  * [Adding a workflow status badge](https://docs.github.com/en/actions/how-tos/monitor-workflows/add-a-status-badge)  
* An **OpenSSF Scorecard** badge (separate from SLSA, but complementary).  
  * [OpenSSF Scorecard](https://github.com/ossf/scorecard)  

### Conventional commit (for adding L1)

If you’re adding the workflow + README badge + docs:

```bash
feat(ci): generate SLSA v1.0 Level 1 provenance and add README badge

- build Rust release artifacts via GitHub Actions
- generate signed SLSA provenance attestations (actions/attest-build-provenance)
- document verification steps and add Level 1 badge
```
---

### What’s next (toward L2/L3 for Rust)

When you’re ready: move build steps into a **reusable workflow** and use **artifact attestations** end-to-end (plus separation of duties) to reach **SLSA Level 3** on GitHub; or use the **slsa-github-generator** “trusted builders.” We can harden this around `cargo` (e.g., reproducible builds, SBOMs, release signing) as a follow-up.  
[Using artifact attestations and reusable workflows to achieve SLSA v1 Build Level 3](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/increase-security-rating)  

### Example docs/PROVENANCE.md (Rust + GitHub Actions + SLSA L1)  

```markdown
# Build Provenance

This project generates and publishes SLSA v1.0 Level 1 provenance for all release artifacts.  
Provenance describes **how**, **when**, and **by whom** the build was produced. It helps consumers verify that artifacts were built from the expected source and were not tampered with.

## 1. How Provenance Is Generated

All release artifacts are built using GitHub Actions in a fully scripted workflow:

- Rust toolchain pinned via `rust-toolchain.toml`
- Builds executed with `cargo build --locked --release`
- Build outputs placed in `dist/`
- Provenance attestation generated with:
```

uses: actions/attest-build-provenance@v3

```bash
This action produces an in-toto/SLSA provenance file tied to the repository’s OIDC identity.

## 2. Where Provenance Is Published

Consumers can access provenance in two places:

### 2.1 GitHub Actions Attestations
Navigate to:
```

GitHub → Actions → Attestations

```markdown
Each build shows:
- attestation statement
- signing identity
- subject hash
- build metadata

### 2.2 GitHub Releases
For tagged releases (`vX.Y.Z`) provenance files are attached alongside binaries:
```

attestation.intoto.jsonl

```bash
This provides an immutable, versioned record of build provenance.

## 3. How to Verify Provenance

Use GitHub CLI:

```bash
gh attestation verify \
  --repo <owner>/<repo> \
  --subject dist/artifacts.tar.gz
```

This command verifies:
* the attestation signature
* the GitHub OIDC identity
* that the attestation matches the specific artifact

If using Sigstore tooling, provenance can also be inspected using `cosign`:

```bash
cosign verify-attestation --key github dist/artifacts.tar.gz
```
***4. SLSA Compliance Level***  

This project is compliant with:

***SLSA v1.0 Level 1***
* Build is fully scripted
* Provenance is generated
* Provenance is published

This level ensures baseline supply-chain transparency and is the foundation for higher SLSA levels.

A roadmap for L2, L3, and L3+CI-hardening is maintained in `SUPPLYCHAIN.md`.

***5. Consumer Expectations***
Users integrating this project’s artifacts should:
* verify checksums
* verify signatures
* verify provenance matches the expected source revision
* retain provenance files for audit or compliance requirements

***6. Questions***
If you need help verifying provenance, contact the maintainers or open a GitHub Discussion.

