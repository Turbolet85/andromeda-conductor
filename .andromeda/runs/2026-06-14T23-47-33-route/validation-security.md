# Security validation — route draft

## Insert
- Between `Base CI + agent-run harness skeleton` and `Seeded phase scheduler`: **"Error-boundary sanitization — thiserror typed enums, anyhow collapse at CLI/Tauri edges, path/struct-name scrubbing"** (epoch: `Foundation`)
  Reason: Per security-plan §Error Handling + Bootstrap phases (error-sanitization-wire), boundary sanitization must be wired before any chunk producing external output.
- Between `Dependency-audit gate` and `Structured logging stack`: **"Dependency version pins — tauri ≥2.10.3, libsqlite3-sys current, toolchain ≥1.94.1"** (epoch: `Foundation`)
  Reason: Per security-plan §Dependency Security + Decisions Log, CVE-2026-42184 (tauri) and CVE-2026-33056 (tar-rs) are required bumps to clear supply-chain residual risk.
- Between `Dependency-audit gate` and `Structured logging stack`: **"Secret-scanning CI gate — defense-in-depth no-secrets invariant enforcement"** (epoch: `Foundation`)
  Reason: Per security-plan §Secret Management + Bootstrap phases (secret-scanning-ci-gate), optional CI scanning prevents accidental credential introduction despite zero secrets today.

## Reorder
- Move `Artifact redaction layer` before `Run-report envelope serializer`
  Reason: Per security-plan §Error Handling sequencing, redaction must precede any emitted output so sanitization applies to all reportable artifacts.
