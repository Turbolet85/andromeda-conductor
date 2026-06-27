# Scope — Live-Pulse E2E proof

**Marker:** `2026-06-27-live-pulse-e2e-proof`
**Version:** conductor-0.1.0 · **Epoch:** 10 (Polish & ship), pulled up to be the next chunk
**Working entry:** _Live-Pulse E2E proof — error-baseline-spike/fingerprint-storm/restart-suppression/pii-scrub/connection-lifecycle MCP-verified_

> **Re-promote note (2026-06-27).** This chunk was promoted once before; its planning was rolled back when the
> work revealed that the MCP read-back client itself was broken against the live sidecar. That blocker is now
> closed by **`2026-06-27-mcp-read-back-result-shape-adapter`** (rmcp removed → hand-rolled line-delimited
> JSON-RPC; `tools/call` parses live). The prior scope/research/plan are regenerated here because two
> assumptions are superseded: (1) the read-back transport (rmcp → hand-rolled JSON-RPC), and (2) the
> **canary-fidelity mechanism** (Pulse incident titles are scrubbed/generated, so a `query_incident_list`
> title-echo can't prove fidelity — see "Canary design" below).

## Intent
The first **true end-to-end proof** that Conductor's loop — seeded timeline → OTLP emission to a **live Pulse**
on `127.0.0.1:4317` → MCP read-back → typed verdict → run report — works against a **real Pulse instance**, not
test stubs. Every seam is unit/golden/stub-tested today; this chunk closes the loop against the live SUT and
produces the agent-parseable evidence (JSONL journal + `runs.db` rows + Markdown report) that the harness
actually verifies Pulse's claimed reactions within their SLOs.

This is **headless** work (CLI `conductor preflight/run/suite` + MCP read-back) and runs on the **Windows dev
host** — it is NOT Linux/xvfb-gated. It needs only a running Pulse, proven to build and run here.

## What is already proven (post-adapter)
- **Read-back works live.** The hand-rolled JSON-RPC client (`conductor-verify/src/jsonrpc.rs` + `client.rs`)
  `initialize`s (protocol `2024-11-05`), lists all 4 required tools, and a real `query_incident_list` now
  **parses** against the live sidecar (the `UnexpectedResponse` failure is gone). Pulse returns the **raw**
  tool payload as the JSON-RPC `result` (no MCP `{content:[…]}` envelope) — e.g. `query_incident_list` →
  `{items, total, next_cursor}`.
- **Live env is up + Windows-runnable.** `pulse-app` on `:4317` · `andromeda-pulse-mcp.exe` at
  `andromeda-pulse/target/debug` · `ANDROMEDA_PULSE_MCP_ENABLED=true` · `ANDROMEDA_PULSE_DATA_DIR=%APPDATA%\andromeda-pulse`.

## The blocking gap = this chunk's core work
Live `conductor preflight` still returns **`[BLOCKED]`**, but now for ONE reason only: **nothing emits the
canary**. The readiness gate pins protocol `2024-11-05`, asserts the 4 required tools present, and calls the
canary read-back — which comes back with the precondition **"incident not found in corpus"** (a real,
parsed-empty result, no longer a call error). The path _canary-emission → Pulse corpus → read-back →
content-fidelity assertion_ is not wired, so every dependent scenario degrades to `Blocked` and nothing is
actually MCP-verified.

**Core deliverable:** wire the canary bridge so the preflight readiness gate reaches `ready: true` against a
live Pulse — emit a known/identifiable canary, let Pulse ingest it into the shared `{data_dir}/corpus/corpus.db`,
read it back, and assert **content fidelity** (the read-back is unmistakably what Conductor emitted, proving the
data-dir/workspace wiring end to end — tool presence + a parsing call never proved corpus *visibility*).

## Canary design (updated from live findings)
Pulse incident **titles are scrubbed/generated**, not a verbatim echo of an emitted marker — so fidelity
**cannot** be a substring/title match in `query_incident_list`. Instead:
- Trigger a **fingerprint-storm** — ≥5 identical-fingerprint exceptions within ~30s with no baseline — using
  `conductor-emit`'s **Pulse-identical `fingerprint`** primitive, so Pulse raises an incident deterministically.
- Prove fidelity via **`retrieve_telemetry_slice` `fingerprint_refs`** (and/or the incident's fingerprint
  field): the read-back carries the **exact fingerprint** Conductor emitted. The fingerprint — computed by
  conductor-emit to match Pulse's algorithm — is the stable, non-scrubbed identity that survives Pulse's
  title generation, so it is the fidelity carrier.
- Keep the marker **unique per preflight** (run/seed/timestamp-derived) so a persistent live corpus can't pass
  the gate on a prior run's canary (stale-corpus false positive).
- Exact carrier field + the storm threshold are confirmed against the live instance at implement time.

## What this chunk builds / proves
1. **Canary bridge (preflight)** — emit→ingest→read-back→fingerprint-fidelity round trip that flips preflight
   from `Blocked` to `ready`, with a bounded ingest poll and a typed outcome on each failure mode (version
   mismatch / missing tool / empty or non-matching canary / keychain read-while-write) → distinct `Blocked`
   with named precondition, never a false pass, never a panic.
2. **Faithful emit/extract framework + two-family live MCP-verified E2E pass** (scope decided at P4 —
   *Bridge + 2 families*). Replace the coarse spine (one signal/phase + the literal `"incidents-listed"`
   read-back) with a scenario-faithful emission dispatcher + real observed-token extraction feeding the
   existing `evaluate_check`→`classify` machinery, and prove it live on two representative families:
   - `fingerprint-storm` (P-017/P-018 — identity + storm-cue thresholds; the canary IS a storm, so its
     faithful path falls out of the bridge almost for free)
   - `error-baseline-spike` (statistical anomaly, P-009..P-012 — a second, distinct deterministic family)
   The remaining three families — `restart-suppression` (P-015/P-016/P-057), `pii-scrub`
   (P-035/P-047/P-048), `connection-lifecycle` (P-001..P-004 + port-occupier) — **defer to a fast-follow
   sibling chunk** (a new markerless working-route entry), reusing the framework this chunk lands.
3. **Evidence** — each live run persists JSONL journal + `runs.db` row + Markdown report with journal-relative
   SLO latency (`read_back_observed_at − journal_emitted_at`), tier-scaled tolerance, verdict-first lamp.

## Surfaces & contracts touched (read/extend, not redesign)
- `conductor-verify` — preflight readiness gate + the canary round-trip + the **hand-rolled JSON-RPC**
  read-back client over the sidecar's stdio (negotiate-down read from `initialize`, pinned `2024-11-05`); the
  verdict/SLO classify→Assessment model. (NOT rmcp — removed by the adapter chunk.)
- `conductor-run` — composition root: `preflight` + `readiness` + `execute_scenario` + `persist` + `drive_run`;
  resolves `ANDROMEDA_PULSE_DATA_DIR` and propagates it to the spawned `andromeda-pulse-mcp` sidecar via
  `.env(...)` (hardened, no argv/shell injection). Composes BOTH `conductor-emit` + `conductor-verify` — the
  canary bridge adds **no new crate edge**.
- `conductor-emit` — the `fingerprint` primitive + per-family emission helpers (exception/error/latency/rate/
  severity/pii/topology) the faithful emit dispatches to; the canary uses the fingerprint-storm path.
- `conductor-cli` — the `run`/`suite`/`preflight` verbs + `--agent-mode`; the source-of-truth release gate.
- `scenarios/*.toml` — the 5 families already exist; may refine `expected`/SLO blocks to live-observed values,
  but adds no scenario without a P-ID.
- `conductor-report` — JSONL journal + `runs.db` + Markdown report envelope (Blocked-row NULL rule).

## Boundaries / non-goals
- **Scope law:** no scenario without a P-ID; Conductor opens no inbound listener of its own except the
  deliberate `:4317` port-occupier inside connection-lifecycle (released on cleanup).
- **Verdict/error wall:** live transport/MCP/canary problems become typed `Blocked`/`Fail` values, never panics;
  `Result::Err` stays harness-faults-only.
- **Probabilistic-assertion policy:** deterministic claims (baseline math, fingerprint identity, suppression
  logic, lifecycle timing) are hard Pass/Fail; model-interpretive claims (severity choice, hypothesis quality)
  route to CalibrationRegion → ManualCheck — never hard-failed on exact values against a live model-backed Pulse.
- **Determinism:** same scenario+seed ⇒ same emission-stream shape; the live Pulse is a real-time black box, so
  SLOs use journal-relative tier-scaled tolerance + sample-count calibration, not wall-clock-from-start.
- **NOT in scope (deferred to the fast-follow sibling chunk):** live faithful verification of the three
  remaining families — `restart-suppression`, `pii-scrub`, `connection-lifecycle`. The framework lands here;
  those families' live calibration + the port-occupier fault wiring ride the sibling.
- **NOT in scope:** the GUI a11y sweep (Desktop a11y verification, A11y CI gate) — Linux+xvfb-gated polish, left
  in their working-route positions. No GUI work here. No Pulse process management (operator/local gate). Not a
  load test. The live family leg is an operator/local gate, never a CI gate (CI keeps the stub + Blocked spine).

## Acceptance shape (refined in plan.md)
- `conductor preflight` against live Pulse → `ready: true` with a passing canary round-trip (was `Blocked`) —
  the Blocked-extinction proof.
- Each of the 2 in-scope families (`fingerprint-storm`, `error-baseline-spike`) runs against live Pulse and
  lands a non-`Blocked` verdict-first report state (Pass / Fail / ManualCheck / KnownResidual per claim class),
  with journal-relative latency recorded and within (or calibration-routed for) its SLO tier.
- Evidence artifacts (JSONL + `runs.db` + `.md`) written per run; no host-path / struct-name leakage.
- Zero unlogged panics; determinism and verdict/error-wall invariants intact.
