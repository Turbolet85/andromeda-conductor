# Report — 2026-08-18-error-baseline-spike-live-proof

**Chunk:** error-baseline-spike live proof — baseline convergence, ramp and candidate persistence over the sample floors against a live deterministic-L4 Pulse, per-surface token gradeability settled first (conductor-run, scenarios, P-009..P-012, v2-12)
**Date:** 2026-08-18
**Commits:** none since last_wrap (this wrap's commit is the chunk commit)

## Changes (structured — detectors read this)
- **Files:** `scenarios/error-baseline-spike.toml` · `scenarios/latency-regression.toml` ·
  `crates/conductor-run/src/lib.rs` · `crates/conductor-run/tests/dispatch_wire.rs` ·
  `crates/conductor-run/tests/baseline_harvest.rs` (NEW) ·
  `crates/conductor-run/tests/snapshots/dispatch_wire__error_baseline_stream_seed_424242.snap` (NEW) ·
  `crates/conductor-core/src/scenario.rs` (loader tests only) · `Cargo.lock` ·
  `conductor-0.2.0/verification-matrix.json` (v2-12 claim at phase P5) · chunk folder
  (scope/research/plan/evidence incl. `evidence/leg-verdict.md`) · master/working route (promotion) ·
  `.andromeda/runs/2026-08-18T17-47-56-phase/` (extracts + graph trace).
- **Symbols / APIs:** NO new or changed public symbols, IPC methods, endpoints, ports, or env vars. One new
  self-obs EVENT line in `conductor_run::emit_canary` (message-borne: `canary fingerprint computed {32-hex}`
  — the allowlisted `message` channel, the key-set-witness precedent). Test-only fns in
  `baseline_harvest.rs` (parser + predicates + the prefix-8 equality comparator, pinned to verbatim leg-A
  captures).
- **Crates / modules:** none added / removed / re-shaped.
- **Dependencies:** `h2` 0.4.15 → 0.4.16 (LOCK-ONLY bump, transitive via tonic/hyper; remediation of the
  NEW external advisory RUSTSEC-2026-0258, low severity, patched upstream; zero new `[[package]]` entries).
  No Cargo.toml change anywhere.
- **Schema / config:** both family scenario TOMLs re-shaped under the family acceptance's re-calibration
  clause: `[[expected]]` blocks RETIRED to declare-only (empty expected → the ManualCheck/KnownResidual
  path), `slo_tier` re-declared, headers re-aimed at the harvest tier. gap_ms / seed / jitter_ms /
  `[phases.emission]` untouched (the seed-named replay golden `fixture_seed_424242` stayed byte-identical
  and was confirmed EXECUTED by name). No envelope / runs.db / violation-schema change.
- **Spec-master edits:** none by the chunk (this wrap's P2 applies the expected amendments below).
- **Counts / qualifiers moved:**
  - `slo_tier` error-baseline-spike `<5s` → `<90s`; latency-regression `<20s` → `<90s` (docs stating the
    old values: test-plan §6 scenario signal; design-system §Surface: cli Component Patterns #4 carries a
    `P-009 … <5s` SAMPLE line).
  - The family's `[[expected]]` check count 2 → 0 per TOML (docs stating the old shape: test-plan §6
    error-baseline-spike scenario + its §1 restatement — "all-Hard floor+candidate" signal).
  - The envelope `fingerprints` field under deterministic L4: `[]` → the 3 `det-*` refs (doc stating the
    old value: obs-plan §4 Fingerprint-storm required-fields row + §1; measured live on leg A).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:**
  - obs-plan §4 (+§1): "`fingerprints` present but pinned `[]` under deterministic L4" — leg A's envelope
    carried `["det-span-…","det-template-0007","det-fingerprint-…"]`; the SUT's deterministic fixture now
    populates `evidence_refs` (SUT HEAD `efabe8e`). Evidence: `runs/2026-08-18T18-47-32-786.jsonl` +
    `evidence/leg-verdict.md`. [expected amendment]
  - test-plan §6 (Scenario: error-baseline-spike) + §1 restatement: the verification signal "exit 0 with
    `[PASS]`; runs.db `verdict="Pass"` / `state="Pass"`" is unattainable under the settled surfaces — the
    declare-only path yields `verdict null` / `state KnownResidual` and the live assertions grade at the
    harvest tier (`triage.cue.emit`, `baseline_harvest.rs`). Evidence: leg A row + leg-verdict. [expected
    amendment, TWO sites per the two-site rule]
  - The v2-12 CONCRETIZED acceptance's latency half (matrix ledger, not a spec master): two structural
    obstacles measured — the canary incident auto-resolves across a ≥180s benign window (30s resolver
    ticks; read-back finds an empty corpus → honest `Blocked`), and the latency cue cannot fire under the
    shipped 90/90 shape (the long-window t-digest p99 absorbs the ramp: threshold ≈6000 vs short ≈2400;
    zero `latency_regression` lines all leg). Disposition: the coverage-gate escalation this wrap
    (operator directive: refine-with-evidence + forward-route). Evidence: `evidence/leg-verdict.md` §Leg B.
- **Coverage of new surfaces:** no new external surface, hot-path op, or UI element. The one new self-obs
  line → validation n/a · instrumentation ✓ (allowlisted `message`, `run_id` carried) · PII redacted ✓
  (32-hex value only) · tests unit ✓ (`baseline_harvest.rs` witness parser + captured-pair test) · a11y n/a
  · tokens n/a.

## Deviations from intent
- `crates/conductor-core/src/scenario.rs` edited beyond "verification-only": the planned check-at-implement
  found two loader tests pinning the retired check shapes; re-shaped to the declare-only guard + a pinned
  `Tier90s` assertion (listed file; the direct consequence of the planned TOML change).
- `Cargo.lock` moved (not in the modify list): NEW external advisory RUSTSEC-2026-0258 landed mid-chunk and
  turned `cargo deny` advisories red; remediated with the sanctioned lock-only bump (security-plan
  §Dependency Security remediation class). Consequence: the standing cargo-audit deferral's basis wording
  changes — "audit surface unchanged (zero new packages) + deny verified green over the NEW lock".
- Matrix flip withheld at implement (the premise-disproof rule): v2-12's latency half measured unprovable as
  concretized → surfaced; resolved at THIS wrap's coverage-gate escalation per the operator directive.
- The latency harvest predicates ship with source-derived fixtures only — no live `latency_regression` line
  exists to pin (part of the surfaced gap).
- Leg B ran under `SEED=424242` — `scripts/agent-run.sh` forces that default onto scenario legs, overriding
  the TOML's declared 4317011. No bearing on either obstacle (seed drives identity/jitter, not cue math);
  recorded as a leg-fidelity trap needing an owner (route-resolve).

## Decisions & corrections
- **The live equality is PROVEN**: Conductor's computed canary fingerprint prefix-8-matches Pulse's own
  storm-line value (`bf2c0bf8`) on a live leg — the 2026-08-17 transcription confirmed against the SUT, the
  TIME-axis chain's first live green; pinned as a verbatim-pair test.
- **A windowed PERCENTILE is not an EWMA** (research-model correction): a long-window t-digest p99 absorbs
  a sustained ramp that contributes half the window's samples, so baseline-relative regression detection
  needs the regression SHORT relative to accumulated history — the shipped 90/90 recipe structurally
  saturates it.
- **A benign emission window outlives the canary incident**: Pulse auto-resolves idle incidents (30s
  ticks); only a scenario whose own traffic re-raises the incident identity keeps read-back non-empty
  across long windows (leg A's error traffic did; leg B's latency traffic could not).
- **Harness seed-forcing**: `agent-run.sh` SEED default overrides TOML-declared seeds on scenario legs.
- **Audit deferral basis updated** (28th pin): h2 bump = a real lock delta; basis is now "audit surface
  unchanged (zero new packages) + deny verified green over the NEW lock" — re-verified this wrap, never
  echoed.
- Operator P4 leans confirmed at P5 review: both TOMLs to `<90s`; the Error-shape stream golden taken.
- The second-incident observation (18:49:56, invisible to the data-dir workspace key) → next Pulse visit's
  intake (the dedupe-semantics precedent's sibling); no cross-repo action.

## Outcome
- **Deterministic gates green** (named): `cargo nextest run -p conductor-run` 63/63 · `cargo nextest run
  --workspace --profile ci` **628/628, zero retries** (611 at chunk start) · `cargo test --workspace
  --doc` ok · `cargo clippy --workspace --all-targets -- -D warnings` clean (over the new lock) ·
  `cargo deny check advisories bans licenses sources` TRUE exit 0 (over the new lock) · `cargo audit` true
  exit 1 — byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` DB fault (28th consecutive; deferral
  re-pinned, basis updated).
- **Live legs** (operator-gated, `SCENARIO=… bash scripts/agent-run.sh run`, fresh dir per leg): leg A
  error-baseline-spike **PROVEN** (suggested cues at magnitude 3.09–3.19 / confidence 1.0 /
  `suppression_bypassed:true`; row `KnownResidual` `<90s`; key sets zero-divergence; equality MATCH);
  leg B latency-regression honest `Blocked` on the two measured obstacles above.
- **Smoke:** ran as P2 gates (two live agent-harness legs + MINT-THEN-READ `status`).
- Acceptance criteria: all deterministic criteria met; the live criteria met for the error half; the
  latency-half criterion disproved-by-measurement and surfaced (this wrap's escalation).
