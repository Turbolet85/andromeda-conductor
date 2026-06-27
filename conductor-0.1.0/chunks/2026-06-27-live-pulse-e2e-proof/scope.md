# Scope — Live-Pulse E2E proof

**Marker:** `2026-06-27-live-pulse-e2e-proof`
**Version:** conductor-0.1.0 · **Epoch:** 10 (Polish & ship), pulled up to be the next chunk
**Working entry:** _Live-Pulse E2E proof — error-baseline-spike/fingerprint-storm/restart-suppression/pii-scrub/connection-lifecycle MCP-verified_

## Intent
The first **true end-to-end proof** that Conductor's loop — seeded timeline → OTLP emission to a **live Pulse** on `127.0.0.1:4317` → MCP read-back → typed verdict → run report — works against a **real Pulse instance**, not test stubs. Until now every seam was unit/golden/stub-tested; this chunk closes the loop against the live SUT and produces the agent-parseable evidence (JSONL journal + `runs.db` rows + Markdown report) that the harness actually verifies Pulse's claimed reactions within their SLOs.

This is **headless** work (CLI `conductor preflight/run/suite` + MCP read-back) and runs on the **Windows dev host** — it is NOT Linux/xvfb-gated. It needs only a running Pulse, which is proven to build (`cargo build --features mcp-server` → exit 0) and run here.

## The blocking gap = this chunk's core work
Live runs today come back **`[BLOCKED]`** on the **preflight canary round-trip**. The readiness gate already pins protocol `2024-11-05` and asserts the 4 required tools are present, but the **canary bridge is not wired**: the path _canary-emission → Pulse corpus → `query_incident_list` read-back → content-fidelity assertion_ does not yet complete a round trip, so every dependent scenario degrades to `Blocked` and nothing is actually MCP-verified.

**Core deliverable:** wire that canary bridge so the preflight readiness gate reaches `ready: true` against a live Pulse — emit one known/identifiable canary incident, let Pulse ingest it into the shared `{data_dir}/corpus/corpus.db`, retrieve it via `query_incident_list`, and assert content fidelity (the read-back incident is unmistakably the one Conductor emitted, proving the data-dir/workspace wiring end-to-end — tool presence alone never proved corpus visibility).

## What this chunk builds / proves
1. **Canary bridge (preflight)** — the emit→ingest→read-back→content-fidelity round trip that flips preflight from `Blocked` to `ready`, including a bounded wait/poll for Pulse ingest and the typed outcome on each failure mode (version mismatch / missing tool / empty or non-matching canary / keychain read-while-write) → distinct `Blocked` with named precondition, never a false pass.
2. **Five-family live MCP-verified E2E pass** — drive the existing catalog scenarios against the live Pulse and verify each reaction via MCP read-back, classified verdict-first into the canonical report states:
   - `error-baseline-spike` (statistical anomaly, P-009..P-012 region)
   - `fingerprint-storm` (P-017/P-018 — identity + storm-cue thresholds)
   - `restart-suppression` (P-015/P-016/P-057 — gap→restart, surgical suppression/bypass)
   - `pii-scrub` (P-035/P-047/P-048 — corpus scrub)
   - `connection-lifecycle` (P-001..P-004 — Listening/Receiving/Idle/Stalled + port-occupier)
3. **Evidence** — each live run persists its JSONL journal, `runs.db` row, and Markdown report with journal-relative SLO latency (`read_back_observed_at − journal_emitted_at`), tier-scaled tolerance, and the verdict-first lamp.

## Surfaces & contracts touched (read/extend, not redesign)
- `conductor-verify` — preflight readiness gate + the canary round-trip + the MCP read-back client (rmcp over `TokioChildProcess` stdio, negotiate-down to `2024-11-05`); the verdict/SLO classify→Assessment model.
- `conductor-run` — composition root: `preflight` + `execute_scenario` + `persist` + `drive_run`; resolve `ANDROMEDA_PULSE_DATA_DIR` and propagate it to the spawned `andromeda-pulse-mcp` sidecar via `.env(...)` (hardened, no argv/shell injection).
- `conductor-cli` — the `run`/`suite`/`preflight` verbs + `--agent-mode`; the source-of-truth release gate.
- `scenarios/*.toml` — the 5 named families already exist; this chunk may refine their `expected`/SLO blocks to match observed live Pulse behavior, but adds no new scenario without a P-ID.
- `conductor-report` — JSONL journal + `runs.db` + Markdown report envelope (Blocked-row NULL rule).

## Proven run environment (Windows)
`pulse-app` on `127.0.0.1:4317` · `andromeda-pulse-mcp` on PATH · `ANDROMEDA_PULSE_MCP_ENABLED=true` · `ANDROMEDA_PULSE_DATA_DIR=%APPDATA%\andromeda-pulse` · sidecar protocol `2024-11-05`, all 4 required tools present.

## Boundaries / non-goals
- **Scope law:** no scenario without a P-ID; Conductor opens no inbound listener of its own except the deliberate `:4317` port-occupier inside connection-lifecycle (released on cleanup).
- **Verdict/error wall:** live transport/MCP/canary problems become typed `Blocked`/`Fail` values, never panics; `Result::Err` stays harness-faults-only.
- **Probabilistic-assertion policy:** deterministic claims (baseline math, fingerprint identity, suppression logic, lifecycle timing) are hard Pass/Fail; model-interpretive claims (severity choice, hypothesis quality) route to CalibrationRegion → ManualCheck — never hard-failed on exact values against a live model-backed Pulse.
- **Determinism:** same scenario+seed ⇒ same emission-stream shape; the live Pulse is a real-time black box, so SLOs use journal-relative tier-scaled tolerance + sample-count calibration, not wall-clock-from-start.
- **NOT in scope:** the GUI a11y sweep (Desktop a11y verification, A11y CI gate) — those stay Linux+xvfb-gated polish, left in their working-route positions. No GUI work here. No Pulse process management (operator/local gate). Not a load test.

## Acceptance shape (refined in plan.md)
- `conductor preflight` against live Pulse → `ready: true` with a passing canary round-trip (was `Blocked`).
- Each of the 5 families runs against live Pulse and lands a non-`Blocked` verdict-first report state (Pass / Fail / ManualCheck / KnownResidual as the claim class dictates), with journal-relative latency recorded and within (or calibration-routed for) its SLO tier.
- Evidence artifacts (JSONL + `runs.db` + `.md`) written per run; no host-path / struct-name leakage in any artifact.
- Zero unlogged panics; determinism and verdict/error-wall invariants intact.
