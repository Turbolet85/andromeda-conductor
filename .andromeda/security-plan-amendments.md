# Security Plan — Amendments

_Append-only changelog of amendments to `security-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Input Validation (scenario-config boundary row) · §Bootstrap phases (input-validation-library-install)
**Change:** garde version 0.23.0 → 0.22.1 in both spots. The validation contract is otherwise unchanged (garde `#[derive(Validate)]` + `#[garde(custom)]` at load; the `CONDUCTOR_*` path handles canonicalize-and-bounds-check at the edge, OUTSIDE garde).
**Why:** mirrors the arch §Stack downgrade — `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable; user authorized 0.22.1 this session. §Dependency Security carried no garde version pin, so it was untouched.

## 2026-06-15-dependency-audit-gate — audit-tool versions are minimum floors; toolchain bump confirmed done
**Section:** §Dependency Security (Audit tool + Pinning) · §Bootstrap phases (dep-audit-tooling-install)
**Change:** the cargo-audit/cargo-deny version pins (0.22.2 / 0.19.8) reframed as minimum **floors** (installed 0.22.1 / 0.19.4), noting they are external CLI tools (not `Cargo.lock`-pinnable) and the RustSec advisory DB is fetched fresh each run — so any tool ≥ floor running green satisfies the gate. The toolchain "currently MSRV 1.88.0 — bump required" updated to "**done** (channel 1.95.0, `rust-version = 1.94.1`)".
**Why:** the audit gate landed **green** with cargo-audit 0.22.1 / cargo-deny 0.19.4 — a hair behind the researched 0.22.2 / 0.19.8. Dev CLI tools can't be locked in-repo and the advisory DB is runtime-fetched, so the body now states floors + records actuals rather than over-precise pins (user-approved at the wrap escalation; a `playbook.md` rule was added so this no longer escalates). Toolchain floor confirmed satisfied by this chunk's green audit. NOTE: `tauri` ≥2.10.3 is left as a forward "required bump" in §Update policy / §Anti-Patterns — tauri is declared in `[workspace.dependencies]` but not yet referenced/locked (dormant until the Tauri GUI, Epoch 9), so its audit remit + any further body reconciliation belong to that chunk.

## 2026-06-15-structured-logging-stack — obs identity env-handles noted as non-path (no validation)
**Section:** §Input Validation
**Change:** added a note that `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs-plan §3) are non-path string labels stamped into self-obs JSON log *values* (JSON-escaped; no path/SQL/argv exposure) requiring **no** validation — distinct from the `CONDUCTOR_*` *path* handles that canonicalize + bounds-check.
**Why:** D-security-input (escalate) fired on the two new env-var reads; resolved WITH the user (2026-06-15 wrap) — they are benign labels, not validation boundaries, so a clarifying note (not table rows) closes the gap. D-security-subprocess + D-security-deps cleared (sidecar untouched; `tracing`/`tracing-subscriber` audit + deny green).

## 2026-06-15-design-token-typography-bundle — npm (frontend) supply-chain gate added
**Section:** §Dependency Security (new Frontend (npm) supply chain paragraph)
**Change:** added the npm/frontend supply-chain control — `npm audit` clean (0 vulns) + committed `package-lock.json` + vendored fonts (no runtime CDN) for the `crates/conductor-tauri/ui/` tree, which cargo-audit/cargo-deny do not cover; npm advisories drive the same floor discipline as cargo (no rigid dependency allowlist at Minimal tier).
**Why:** D-security-deps (escalate) fired on the chunk's new npm dependency ecosystem (React 19 / Vite 8 / Tailwind 4.1 / Fontsource / TS); §Dependency Security was cargo-only. Resolved WITH the user (2026-06-15 wrap, all-4 recordings): the npm-audit-clean gate + committed lockfile is the control. Cascaded to `.claude/rules/security.md` + `.claude/docs/security-summary.md`.

## 2026-06-21-runs-db-index — bundled SQLite version corrected to 3.50.4
**Section:** §Infrastructure (Database)
**Change:** `bundled` SQLite `3.51.1 → 3.50.4` (noting `via libsqlite3-sys 0.36.0`); rusqlite 0.38.0 unchanged.
**Why:** mirrors the arch §Stack correction — the runs-db-index chunk's first real `rusqlite 0.38.0 bundled` compile resolves `libsqlite3-sys 0.36.0` → SQLite 3.50.4 (verified from the bundled `sqlite3.h` + `Cargo.lock`); the stated 3.51.1 was assumed. D-security-deps (escalate) cleared: the dep is arch-locked (§Established Decisions [Database]) and audit-green (cargo-audit exit 0 / cargo-deny advisories+bans ok); Cargo.lock committed this chunk. Cascade no-op for the distillations (security-summary.md / rules/security.md carry no bundled-SQLite version — grep-confirmed).
