# Report — 2026-08-21-delegated-timing-budgets-proven

**Chunk:** Delegated timing budgets proven — the four Pulse-delegated timing bounds (P-025/P-027/P-037/P-045) graded at their real measured values against a live Pulse
**Date:** 2026-08-21
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-run/tests/delegated_timing_harvest.rs` (NEW) — harvest-tier extraction + grading of the four delegated bounds, with the live-leg captures pinned verbatim.
  - `scenarios/halo-hue-encoding.toml` · `scenarios/service-constellation-discovery.toml` · `scenarios/report-render-surface.toml` · `scenarios/findings-counter-refresh.toml` — header comments only.
  - `conductor-0.2.0/verification-matrix.json` — `v2-20` claimed, acceptance refined, `ref` + `status:implemented`, two PREMISE-CORRECTION narratives in `notes`.
- **Symbols / APIs:** none public. The new file is an integration test whose items are all test-local (`DelegatedBound`, `bounds()`, `harvest_since`, `is_target`, `num_field`, `observations`, `grade`) — no crate export, no new `pub` item, no IPC method, no endpoint, no socket, no port, no env var. No existing signature changed, so no caller set moved.
- **Crates / modules:** none added, none removed, none changed. Workspace stays 9 members.
- **Dependencies:** **none added, none bumped.** `Cargo.toml`, every `crates/*/Cargo.toml` and `Cargo.lock` are byte-untouched (verified via `git status --porcelain`). The test uses only `std` plus the crate's existing dev-dependencies.
- **Schema / config:** no config KEY added, changed or removed. The four scenario TOMLs gained header COMMENTS only; all four re-parse and their `[[expected]]` counts are unchanged (0 / 0 / 0 / 1) and their `slo_tier` values unchanged (`<5s` each). No migration, no violation-schema change, no run-report envelope or `CheckRecord` field change.
- **Spec-master edits:** none. `/implement` authored no amendment; the seven `.andromeda/` masters are untouched by this chunk's code phase.
- **Counts / qualifiers moved:** **one.** `architecture.md:130` states "**SEVEN** families now ship declare-only … with the live claims graded at the harvest tier" and enumerates them. This chunk live-proves the delegated-timing family (`halo-hue-encoding` · `service-constellation-discovery` · `report-render-surface`, each zero `[[expected]]`, each landing `KnownResidual` at the harvest tier), which is an **eighth** such family by that sentence's own definition. Docs stating the value: `architecture.md:130`. (`findings-counter-refresh` is NOT part of that count — it carries one `[[expected]]`.)
- **Dev-tool versions:** none — no external CLI installed or upgraded.
- **Reverted / negative API facts:** none. One deliberate non-write worth recording: the harvest test's leg-fixture functions were left ABSENT rather than stubbed during the first implement pass, because inventing capture lines would fabricate measurement evidence; they were filled only from real legs.
- **Spec claims disproved by measurement:** **four.**
  1. **P-025's ≤2 s hue budget** — stated at `.andromeda/input.md:110` ("hue shift ≤2s pipeline latency (delegated)") and in `scenarios/halo-hue-encoding.toml`'s header. Measured **35581 ms** (leg A) and **36705 ms** (leg B) — 18× over on two independent legs. Structural, not a slow run: the scenario declares zero `[phases.emission]` so drives no tier change of its own (both samples are the preflight canary's, `severity_tier: autonomous`, within a second of its incident), and the observable computes `now − item.last_seen_unix_nano` — staleness at tier change, not update latency — which for a quiesced service reports Pulse's own L2→L3(20-60 s)→L4 formation path. Evidence: `delegated_timing_harvest.rs::p025_hue_update_is_measured_over_budget_by_a_staleness_mechanism`. **Disposition: already resolved** — operator-ratified refine-with-evidence, P-025 removed from `v2-20`'s acceptance and recorded as PREMISE-CORRECTION 2 in its `notes`; `input.md` is deliberately NOT amended (immutable input record — the ledger is the sanctioned channel).
  2. **`metric.report.render_ms` requires an operator to open a Report** — stated in this chunk's own `research.md` and `plan.md`, and in the first draft of `report-render-surface.toml`'s header. Measured false: the leaf fires on an `incidentId` TRANSITION, which Pulse's UI performs by auto-selecting a newly formed incident; all four samples were auto-fired, and an operator opening the already-selected incident produced no further line. **Disposition: corrected in the scenario header this chunk.**
  3. **`findings-counter-refresh` will land `Blocked` because it carries an `[[expected]]` check** — stated in this chunk's `plan.md` Implementation notes. Measured false: it landed `verdict: CalibrationRegion` / `state: KnownResidual`. The degraded-mode residual route (not gated on declare-only) governs, and an unmet `CountAtLeast` floor routes to CalibrationRegion rather than failing. **Disposition: corrected in the scenario header this chunk.**
  4. **`v2-20`'s `observed_gap`: "All four scenarios currently carry empty expected blocks"** — measured false at phase (`findings-counter-refresh.toml:33` carries one). **Disposition: already recorded** as PREMISE-CORRECTION 1 in `v2-20` `notes`.
- **Coverage of new surfaces:** no new external surface, hot-path operation, or UI element was added — the chunk's only new artifact is a test.
  - `crates/conductor-run/tests/delegated_timing_harvest.rs` (test-only, no runtime reach) → validation n/a (reads no external input at runtime; the harvest reads a foreign log only inside the test) · instrumentation n/a (emits no span/metric/log) · PII redacted✓ (pinned fixtures carry no absolute host path and no internal struct name — Pulse's own field set only: `duration_ms`/`value`/`severity_tier`/`discovered_count`/`section_count`/`degraded_mode`/`service.name`) · tests unit✓ (12 tests, `cargo nextest run -p conductor-run`) · a11y n/a (no UI) · tokens n/a (no UI)

## Deviations from intent

1. **Plan steps 1–3 could not run on the first implement pass** (environmental: neither binary on PATH, no `pulse-app` running, GUI + operator action required). Resolved in this session: the operator brought Pulse up and all four legs ran.
2. **P-037's "operator step" turned out unnecessary** for the measurement (see disproved claim 2). The operator still supplied the drive+observe visual confirmation, which is what P-037's classification actually wants; the timing sample was auto-fired.
3. **Leg C was run twice.** The first run's incident auto-resolved (created 18:45:46, active set empty by 18:49:42, 9 auto-resolve ticks) before the operator could open its Report, so nothing was open to click. Re-run on the same data dir was safe and deliberate: the active set was empty so dedupe could not fire, and the 60 s storm-retention window had long passed. Justification: the auto-resolve lifecycle is Pulse's, not a leg defect.
4. **`v2-20`'s acceptance was narrowed from four budgets to three** — a weakening of the stated outcome, so it was put to the operator explicitly and ratified (the fourth such ratified refinement after v2-12 / v2-13 / v2-14). P-025 returns to the pool; it is NOT un-verified, it was never verified.
5. **The code-graph refresh was fired at P1 rather than at Setup.** Setup's placement assumes "wrap touches no `src/`, so the indexed code is fixed at entry"; that assumption was false while the chunk still needed its live legs and fixture writes, so the refresh was deferred until source was final. Both planes built clean afterwards.

## Decisions & corrections

- **Ratified (operator, 2026-08-21):** include P-037 in the chunk via an operator step rather than leaving it unclaimed — chosen at phase P4 because P-037 is already `DriveObserve`, so an operator action is native to it, and because `v2-20`'s acceptance named all four budgets.
- **Ratified (operator, 2026-08-21):** refine `v2-20` to the three measured budgets and route P-025 forward, rather than un-claiming the cap entirely or redesigning the scenario in-chunk.
- **Correction (mine, caught pre-scope):** an "no production callers" claim about the three Pulse frontend procedures came from a grep whose include-set missed the camelCase TS wrapper layer; a broader search found real call sites. Recorded as `contract.narrow-basis-claim`.
- **Correction (mine, caught pre-record):** the `cargo audit` PREREQ probe was first read through a pipe, so `$?` reported `head`'s exit (0) rather than cargo's (1) — which reads as a signature DEVIATION and would have needlessly restored the full re-check form. The pinned signature turns on the exit code, so it must be captured before any pipe.
- **Standing:** the four delegated budgets are graded from Pulse's own observable and never through `budget_ms` / `effective_deadline_ms`. Confirmed live this chunk, not merely by reading: leg D's envelope recorded `latency_ms: 6139` while `metric.findings.counter_refresh_ms` in the same leg measured 1.9 ms median — one run, two quantities three orders apart.
- **Method note:** three of five `[inferred]` scope premises were verified and one premise-corrected at P3, before P4 consumed scope; val-1 then caught a stale derived section (the surfaces list) the closure had left behind.

## Outcome

**Acceptance criteria: met, as refined.** `v2-20` claims P-027 / P-037 / P-045, each graded hard at a real measured value:

| Cap | Measured | Budget | Verdict |
|---|---|---|---|
| P-027 constellation discovery | 702.4326171875 ms @ `discovered_count: 3` (leg B) | ≤5000 ms | PASS |
| P-037 report render | 1 ms / 0 ms (leg C, both `degraded_mode`) | ≤2000 ms | PASS |
| P-045 findings counter refresh | worst 7.0 ms of 269 in-window samples (leg D); ~1000 more across legs A–C, none over | ≤1000 ms | PASS |
| P-025 hue update | 35581 ms, 36705 ms | ≤2000 ms | **FAIL — premise disproved, routed forward** |

**Gates green** (all re-run after the final edits):
- `cargo nextest run -p conductor-run` → 124/124
- `cargo nextest run --workspace --profile ci` → **731/731**
- `cargo test --workspace --doc` → 0 failed
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- PREREQ probe → **auto-satisfied, 37th consecutive**: `cargo audit` true exit 1 with first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`, `cargo deny check advisories bans licenses sources` true exit 0, zero dependency delta.

**Smoke / live legs:** four legs against a live Pulse at HEAD `f0c38f5`, each on a fresh data dir with `pulse-app` restarted and its window open. All four exited 0 and landed a reported (non-`Fail`) state: `[RESIDUAL]` × 4 — runs `2026-08-21T18-38-48-967` (A), `18-42-31-734` (B), `18-45-01-029` + `18-50-22-415` (C), `18-53-35-135` (D). Deterministic L4 independently confirmed by the operator's Report screenshots carrying the constant `det-*` evidence triple.

No gate deferral was taken or is outstanding: the chunk's delta includes `.rs`, so every changed-surface and workspace gate ran.
