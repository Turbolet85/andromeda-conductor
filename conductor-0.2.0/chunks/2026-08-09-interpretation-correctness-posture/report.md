# Report — 2026-08-09-interpretation-correctness-posture

**Chunk:** Interpretation-correctness posture — the recorded answer to intent F3: either a real-model leg or a deferral naming its owner and stating that "Conductor green" does not mean interpretation is trustworthy; closes the 0.1.0-era open posture decision (v2-05)
**Date:** 2026-08-09
**Commits:** none yet — this wrap authors the chunk commit (prior HEAD `fde458f` is the previous chunk)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-core/src/error.rs` — new `CoreError::UnbackedCoverage(String)` variant
  - `crates/conductor-core/src/drift.rs` — new `UNBACKED_AUTO` pin + `check_scenario_backing` + `backing_message`; module doc extended to describe two integrity gates on different axes; 7 new tests
  - `crates/conductor-core/src/lib.rs` — re-export widened to `drift::{KNOWN_UNCLASSIFIED, UNBACKED_AUTO, check_scenario_backing, check_sut_drift}`
  - `crates/conductor-report/src/coverage.rs` — `summary_line` gained an `unbacked: usize` parameter; roll-up qualifies the auto term; 1 new test
  - `crates/conductor-cli/src/render.rs` — `coverage_summary_styled` mirrors the qualifier
  - `crates/conductor-tauri/src/commands.rs` — new read-only `unbacked_auto` command + registration in the mock-runtime handler + 1 new IPC test
  - `crates/conductor-tauri/src/main.rs` — `unbacked_auto` registered in `generate_handler!`
  - `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — `tally()` takes an `unbacked` count; new optional `unbacked` prop
  - `crates/conductor-tauri/ui/src/App.tsx` — sources the count via `invoke<string[]>('unbacked_auto')`, degrades to 0
- **Symbols / APIs:**
  - NEW public: `conductor_core::UNBACKED_AUTO` (`&[&str]`, 11 ids) · `conductor_core::check_scenario_backing(&[CapabilityRow], &[&str], &[&str]) -> Result<()>` · `CoreError::UnbackedCoverage`
  - NEW Tauri IPC command: `unbacked_auto` → `Result<Vec<String>, String>` (read-only, nullary; app command — no capability-ACL entry needed)
  - CHANGED private: `conductor_report::coverage::summary_line(rows)` → `(rows, unbacked)`
  - CHANGED webview prop: `CoverageMatrix({rows, lamps})` → `({rows, lamps, unbacked?})`
  - No new ports, no new sockets, no new env vars.
- **Crates / modules:** none added or removed; changes confined to `conductor-core`, `conductor-report`, `conductor-cli`, `conductor-tauri` (+ its webview).
- **Dependencies:** **NONE added, none bumped.** `Cargo.lock` and `package-lock.json` both un-drifted (verified via `git status`). Host-tool change only: `cargo-audit` 0.22.1 → 0.22.2 (a dev CLI, not a repo artifact).
- **Schema / config:** no migrations, no config keys, no violation-schema change. `runs.db` untouched. The run-report envelope is **unchanged** — the posture rides a `const` + a gate + a roll-up qualifier, not a new envelope field, so no `conductor-core::redact` allowlist entry was needed.
- **Coverage of new surfaces:**
  - `conductor_core::check_scenario_backing` (internal gate, no external input) → validation n/a (pure set comparison over committed artifacts) · instrumentation n/a (test-time gate, no production call site — mirrors `check_sut_drift`) · PII n/a · tests unit ✓ (7: committed-artifact green + 3 drift conditions + non-Auto exemption + empty-ledger pass + identity-only message) · a11y n/a · tokens n/a
  - `unbacked_auto` Tauri IPC command → validation n/a (nullary, returns a `const`) · instrumentation ✓ (manual `tauri.command.unbacked_auto` span + `count`/`latency_ms`, the shipped pattern) · PII n/a (capability ids only) · tests integ ✓ (mock-runtime IPC dispatch asserts the payload equals the core ledger) · a11y n/a (no DOM) · tokens n/a
  - coverage roll-up qualifier (Markdown · CLI · webview header strip) → validation n/a · instrumentation n/a · PII n/a (counts + capability ids) · tests unit ✓ (exact-format golden with and without the qualifier; a not-a-fifth-summand test) · a11y **not asserted — display-gated harness unrunnable on the Windows host, recorded skip** · tokens ✓ (zero new tokens/ANSI/hex; the qualifier is plain text inside the existing summary line)

## Deviations from intent

1. **Two files edited outside the plan's `Files to modify` list** — `crates/conductor-tauri/src/main.rs` (the `generate_handler!` registration) and `crates/conductor-tauri/ui/src/App.tsx` (sourcing the new prop). *Justification:* both are the inseparable second half of changes the plan did mandate — an unregistered command is dead code that fails `clippy -D warnings`, and an unsourced prop renders nothing. Judged in-scope under the code-writing-discipline gray-area rule rather than soft-exited.
2. **`CoverageMatrix.css` listed but not edited** — the plan carried a nit to sweep a stale "all 60 capabilities" comment. *Justification:* grep proved the comment was already corrected by the prior chunk (`CoverageMatrix.css:1` now reads "the SUT capability manifest's accepted set"). Nothing to sweep.
3. **`summary_line` gained a parameter rather than reading the const directly** — the plan said the roll-ups read `UNBACKED_AUTO.len()`. *Justification:* reading the const *inside* `summary_line` would make the synthetic-set format golden render `2 auto (11 unbacked)`, coupling a pure function to global state and defeating the very test that exists to lock the shape without baking live counts. `render()` reads the const and passes it in; the CLI and webview read it at their own call sites as planned.
4. **`cargo audit` PREREQ could not be closed by the prescribed remedy** — see *Decisions & corrections* and *Outcome*. The remedy was attempted empirically, not skipped.

## Decisions & corrections

- **Operator decision (P4 dialogue, 3 questions, all recommendations accepted):** take `v2-05`'s **Branch B** (recorded deferral) rather than a real-model leg; make it load-bearing via a **live exact-set gate** over the unbacked-`Auto` set; name **a conductor-0.3.0 entry** as the deferral's owner.
- **Branch A was infeasible in this chunk's boundaries** — `execute_scenario` grades every `ExpectedCheck` against the literal placeholder `"incidents-listed"` (`conductor-run/src/lib.rs:216`); real per-check extraction is `v2-09` in Epoch 2. A P-033 hypothesis assertion today would assert against that string.
- **The gap is 11, not 1** — measured at P3: `P-031 P-033 P-034 P-039 P-041 P-042 P-043 P-044 P-073 P-074 P-079` are classified `Auto` with no scenario naming them. `check_sut_drift` compares classification↔manifest and can never see this. `scope.md` was amended at P5 validation-1 (**intent-incomplete**, justified by the measurement) to record the generalization from P-033 to the whole set.
- **`v2-26` examined and deliberately NOT claimed** — its acceptance gates capabilities left *unclassified* (the `check_sut_drift` axis); this chunk gates a different axis (`Auto` rows with no scenario). Recorded in the plan's provenance as adjacent-not-advanced; it stays owned by the Epoch-6 entry.
- **`v2-05` method refined `manual` → `unit`** at P5 — the deferral branch is enforced by a nextest gate over committed artifacts, not by operator attestation.
- **CORRECTION — the `cargo audit` remedy is unexecutable as written (empirically proven).** `.claude/rules/security.md` (2026-08-09) and `security-plan.md` §Dependency Security prescribe *"raise the cargo-audit FLOOR to the fixed release"* when the advisory-DB parse failure persists. That presumes a **tool** fault. Evidence gathered this chunk: 0.22.1 → exit 1 `duplicate advisory ID: RUSTSEC-2026-0244`; installed **0.22.2 (latest published)** → **byte-identical failure**. The duplicate ID is committed **data** in RustSec's advisory-db, so no released version can parse it — there is no fixed release to raise a floor to, and doing so would look like compliance while changing nothing. An advisory-**database** fault is a distinct case from a tool fault; its remedy is the bounded wait with the audit↔deny overlap verified green, re-checked each chunk.
- **PREREQ disposition:** re-checked, still red, **deferral extended** with the data-fault fact (not closed, not suppressed). Zero dependency delta and both lockfiles un-drifted, so the red gate is not this chunk's; `cargo deny check` green across all four classes is the verified overlapping signal.

## Outcome

**All 11 acceptance criteria met** except the a11y one, which is met in its recorded-skip form.

Gates, all first run — **0 fix iterations**:
- `cargo nextest run -p conductor-core` → **199/199** (+7)
- `cargo nextest run -p conductor-report -p conductor-cli` → **65/65** (+1)
- `cargo nextest run --workspace --profile ci` → **456/456** (+9), zero retries
- `cargo test --workspace --doc` → ok
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `npm run build` (incl. `tsc --noEmit`) → ok
- `cargo deny check advisories bans licenses sources` → **ok (all four)**
- `cargo audit` → **exit 1, external advisory-DB data fault** (see above)
- `cargo run -p conductor-cli -- coverage` and `-- coverage --write` → both render the qualifier; the Markdown artifact was inspected then deleted (never committed — the standing Epoch-6 CARRY owns that question)

**Smoke:** `bash scripts/agent-run.sh run` → **exit 0** (bounded at 900s; no timeout, no SIGKILL, no leftover processes).

**Not asserted:** the webview leg has no headful proof — the axe/contrast/keyboard harness is display-gated to Linux+xvfb and unrunnable on this Windows host. Its evidence is the `tsc` typecheck plus the Rust-side IPC test. Recorded, never a silent pass.

All three coverage surfaces render the identical roll-up:
`82 capabilities · 66 in scope (43 auto (11 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`
