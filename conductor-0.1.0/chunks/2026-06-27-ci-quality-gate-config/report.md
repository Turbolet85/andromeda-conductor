# Report — 2026-06-27-ci-quality-gate-config

**Chunk:** CI quality-gate config — line-coverage-threshold gate + zero-retry flakiness budget + nextest JUnit XML + cargo-llvm-cov/JUnit artifact upload layered onto the GitHub Actions build/test pipeline; CI-config only.
**Date:** 2026-06-27
**Commits:** (none yet — wrap authors the chunk commit)

## Changes (structured — detectors read this)
- **Files:** `.github/workflows/ci.yml` (modified — the sole code change). `.config/nextest.toml` was a planned *optional* target but needed **no edit** (already carries `[profile.ci.junit]` + `retries = 0`).
- **Symbols / APIs:** none — no Rust symbols, fns, IPC methods, endpoints, exports, ports, sockets, or env vars added/changed. New **GitHub Actions artifacts** only: `coverage-report` (`lcov.info` + `coverage.xml`) and `test-results` (`target/nextest/ci/junit.xml`).
- **Crates / modules:** none added / removed / changed (zero `crates/**` diff).
- **Dependencies:** none added / bumped — no `Cargo.toml` / `Cargo.lock` change. (`cargo-llvm-cov` was already installed by the CI `taiki-e/install-action` list from the base-ci chunk.)
- **Schema / config:** no Conductor config keys changed. CI workflow (`ci.yml` `rust` job) gained: a zero-retry flakiness-budget assertion step; the coverage step promoted from `--summary-only` to collect → LCOV+Cobertura reports → `--fail-under-lines 60` line gate (with `--ignore-filename-regex '[\\/]tests[\\/]'`); two `actions/upload-artifact@v4` steps (`if: always()`).
- **Coverage of new surfaces** (no new *application* surface — CI-config only; the only new outputs are CI artifacts):
  - `CI coverage artifact (lcov.info)` → validation n/a · instrumentation n/a · PII **absolute build paths present (raw✗ — see Deviations)** · tests n/a (this IS the test/coverage infra) · a11y n/a · tokens n/a
  - `CI coverage artifact (coverage.xml, Cobertura)` → PII redacted✓ (only the standard `<source>` root; per-file paths relative) · others n/a
  - `CI test-results artifact (junit.xml)` → PII redacted✓ (verified 0 drive-letter paths) · others n/a

## Deviations from intent
1. **`.config/nextest.toml` not modified** — the plan listed it as an *optional* target ("no functional change expected"); verified it already carries `[profile.ci.junit] path = "junit.xml"` + `retries = 0` (both profiles) from `2026-06-16-test-framework-fixtures-coverage-tooling`, so no edit was warranted.
2. **`lcov.info` carries absolute build paths** (58 `SF:` lines) — a cargo-llvm-cov 0.8.x default; partially misses the plan's security AC ("uploaded coverage artifacts → relative crate identifiers only"). **Justification / scope:** (a) in CI these are *ephemeral GitHub-runner* paths (`D:\a\…`), never a developer host path, so the security *intent* (no host-path leak in the shared artifact) holds; (b) `coverage.xml` (Cobertura `<source>`-root only) and `junit.xml` are clean (verified); (c) the lcov is produced by a **third-party tool (cargo-llvm-cov)** as an ephemeral CI artifact — categorically outside the security-plan §Error Handling / obs-plan §6 redaction boundary, which governs **Conductor's own** artifacts (run-report MD · runs.db · JSONL journal, sanitized at the anyhow edge + tracing field-allowlist). cargo-llvm-cov has no clean relativize flag and a Windows-`sed` strip is fragile + untestable locally. → candidate for a documented residual / AC carve-out, or a `RUSTFLAGS=--remap-path-prefix` follow-up.
3. **`--ignore-filename-regex '[\\/]tests[\\/]'` is currently a no-op** — cargo-llvm-cov reports only `src/`, so `tests/` files aren't in the denominator (the 89.89% already reflects product code). Kept per test-plan §10 exclusion intent + future-proofing.
4. **Full `agent-run run` not re-executed** — a ci.yml-only change has zero Rust delta; the nextest leg was verified green via the coverage collect (417 tests, exit 0), and doctest/clippy/the harness scripts are unmodified.

## Decisions & corrections
- **Reorder (user-directed):** "CI quality-gate config" pulled forward from Epoch 10 to lead, ahead of the display-gated "Desktop a11y verification" — Windows-only session; it's Windows-doable + scaffolds the later Obs/A11y CI gates. The display-gated + live-Pulse Epoch-10 chunks stay deferred.
- **P5 plan-review decisions (approved):** (#1) JUnit consumption = **artifact-only** — no `dorny/test-reporter`, no `permissions: checks: write` elevation (solo dev on a long-lived `build/` branch, no PR-review surface; consumed via the artifact API / `agent-run logs`); (#2) **`windows-latest` single-OS** — no ubuntu/macos matrix (out of the chunk's four deliverables; the matrix implies the display-gated webview leg); (#3) flakiness budget = a lightweight CI grep-assertion of `retries = 0`.
- **Verification learnings:** real workspace line coverage is **89.89%** (>> the 60% floor) — gating is safe; cargo-llvm-cov reports `src/` only (the `tests/` exclusion is a no-op); cargo-llvm-cov emits absolute paths in LCOV by default (Cobertura/JUnit stay clean).

## Outcome
- **Acceptance criteria:** 7 / 8 fully met; **AC6 (no absolute paths in coverage artifacts) partial** — met for `coverage.xml` + `junit.xml`, not `lcov.info` (Deviation #2).
- **Gates green (commands run locally on the Windows host):** coverage gate `cargo llvm-cov report --fail-under-lines 60 …` → exit 0 at 89.89%; `--fail-under-lines 95` → exit 1 (mechanism proven); LCOV+Cobertura emit; flakiness assertion passes on `retries=0` and catches `retries=2`/`retries={…}`; `cargo audit` exit 0 (18 allowed warns); `cargo deny check` exit 0 (advisories/bans/licenses/sources ok); `ci.yml` parses as valid YAML.
- **Smoke (boot-path touch):** `bash scripts/agent-run.sh status` → exit 0, valid run-record JSON. (Full `agent-run run` not re-run — Deviation #4.)
