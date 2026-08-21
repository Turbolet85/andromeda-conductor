# Scope — 2026-08-21-delegated-timing-budgets-proven

**Working entry:** Delegated timing budgets proven — halo hue, constellation discovery, report render and
counter refresh at real values (P-025, P-027, P-037, P-045)

**Version:** conductor-0.2.0 · **Epoch:** 4 — Lifecycle & delegated timing
**Matrix cap in play:** `v2-20` (Four delegated timing budgets return real Pass/Fail) — `chunk:null`,
`status:planned`, `method: dynamic-external`

---

## What this chunk is

Pulse delegated four timing budgets to Conductor and has now built every measurement surface they need.
This chunk is the **assertion round** Pulse cannot perform for itself: drive a live Pulse, obtain each
capability's real measured value, and grade it against its own budget —

| P-ID | Capability | Budget | Conductor scenario |
|---|---|---|---|
| P-025 | severity→hue update | ≤2s | `scenarios/halo-hue-encoding.toml` |
| P-027 | constellation discovery | ≤5s | `scenarios/service-constellation-discovery.toml` |
| P-037 | report render | ≤2s | `scenarios/report-render-surface.toml` |
| P-045 | findings counter refresh | ≤1s | `scenarios/findings-counter-refresh.toml` |

The deliverable is the **graded outcome plus its recorded basis** — whichever grading tier the measurement
actually supports. It is NOT a Pulse-side change of any kind.

## Verified coordinates (annotation folded as hypothesis, then checked against the artifacts)

The working entry's `CONTEXT` names four metric targets. All four verified at Pulse HEAD `f0c38f5`
(chunk `andromeda-pulse-0.3.0/chunks/2026-08-21-delegated-timing-observables`, commit `624e26a`):

| Target | Fire site | Allowlist fields |
|---|---|---|
| `metric.constellation.hue_update_ms` | `pulse-app/ui/src/widget/ConstellationCanvas.tsx:152` (webview → TauRPC) | `duration_ms`, `severity_tier` |
| `metric.constellation.discovery_ms` | `pulse-app/ui/src/widget/ConstellationCanvas.tsx:106` (webview → TauRPC) | `duration_ms`, `discovered_count` |
| `metric.findings.counter_refresh_ms` | `pulse-app/ui/src/hooks/use-findings.ts:58` (webview → TauRPC) | `duration_ms` |
| `metric.report.render_ms` | `pulse-app/src/incidents_router.rs:560` (GUI backend router) | `value`, `section_count`, `degraded_mode` |

Two corrections to the dictated coordinates, both material:

- **P-037's field is `value`, not `duration_ms`** — the pre-existing leaf diverges in field name from the
  three new ones. Any extraction that assumes a uniform `duration_ms` reads three of four and silently
  misses the fourth.
- The `CONTEXT` line reads as if all four were shipped by that Pulse chunk; **`metric.report.render_ms`
  is pre-existing** and is the only one NOT routed through `crates/ui-bridge/src/telemetry.rs`. The
  annotation says so parenthetically; the distinction matters because its fire site is a different
  process path.

`halo-hue-encoding.toml:27` verified — it is exactly the `error-pressure` phase line grading the hue by
operator observation in halo wording, as the `PREMISE` annotation states.

## The structural finding that shapes this chunk [verified]

**None of the four observables reaches any MCP read-back surface.** All four are `tracing::info!` events on
Pulse's own self-observation stream, each behind an exact allowlist leaf. Pulse's MCP surface is the same
8 tools (`query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` + the 4 corpus tools), and
none of them reads Pulse's tracing output.

For P-037 specifically the unreachability is proven twice over: `dispatch_retrieve_report`
(`crates/mcp-server/src/tools.rs:360`) builds the report **inside the sidecar** from the corpus row and
never calls `incidents_router.rs`; `crates/mcp-server/Cargo.toml:32-33` records that importing pulse-app
would be a dependency cycle. So Conductor calling `retrieve_report` cannot make `render_ms` fire.

Three of the four additionally fire only on **webview activity** in pulse-app's GUI process — and
"no UI automation" is a stated Conductor non-goal (CLAUDE.md §Cross-cutting Patterns, Scope law).

## Premises — closed at P3 (research.md carries the evidence)

- **VERIFIED — the grading tier is the harvest tier, and option (b) is unnecessary.** Pulse writes its own
  tracing stream to a durable on-disk JSONL via `tracing_appender::rolling::daily(&logs_dir,
  "agent-latest.jsonl")` (`pulse-app/src/observability.rs:2462`), so all four `metric.*` lines land in an
  agent-parseable artifact — the same class Conductor's prior harvest legs already read (`buffer.tick`,
  `duckdb.append`, `storms_detected_total`). The claims are graded in a `conductor-run` harvest test over
  verbatim leg captures. **No new Conductor ingestion surface, no fourth interface, no trust-boundary
  amendment** — the option (b) fork is dropped rather than escalated.
- **VERIFIED — `budget_ms` is the WRONG instrument; binding it here would be a category error.**
  `evaluate_slo` computes `latency_ms = read_back_observed_at_ms − journal_emitted_at_ms`
  (`conductor-verify/src/slo.rs:39`), and the call site states outright that a scenario's checks "share one
  latency by construction — the corpus is observed once" (`conductor-run/src/lib.rs:414-416`). So
  `budget_ms` bounds **Conductor's MCP round-trip**, never a Pulse-internal duration. Grading P-025's ≤2s
  hue bound with it would measure Conductor's read-back speed and label it Pulse's hue latency — a hollow
  `verified`. The four budgets must be graded from Pulse's own measured `duration_ms`/`value`, not through
  the `budget_ms` path.
- **[premise-corrected: 3 of the 4 fire autonomously; only P-037 needs an operator]** The earlier premise
  that the webview-sourced observables need a human-driven Pulse window is FALSE for three of them.
  P-027 (`ConstellationCanvas.tsx:106`) and P-025 (`:152`) fire from a `useEffect` keyed on `[dots, items]`
  — new services appearing, and a severity **tier change** respectively — both driven by the telemetry
  Conductor emits. P-045 (`use-findings.ts:58`) fires inside `refetch` on a **1000 ms background re-poll**
  (`FINDINGS_REPOLL_MS`), continuously and unprompted. All three need only Pulse's window open, which is
  already a live-leg precondition. **P-037 is the exception**: `metric.report.render_ms` fires on
  `incidents.get_report`, called from a `useEffect` keyed on `incidentId` (`use-report.ts:51`) — it fires
  only when an operator opens a report for an incident, which is a genuine drive+observe action.

## Additional finding — the matrix cap's `observed_gap` is inaccurate

`v2-20`'s `observed_gap` states "All four scenarios currently carry empty expected blocks". Measured false:
`scenarios/findings-counter-refresh.toml:33` carries one `[[expected]]` (`kind = "CountAtLeast"`,
`class = "Hard"`, `expected = "3"`), grading the derived COUNT — its own header already records that the
≤1s refresh timing is a deferred measurement with "no content token for the latency". The other three are
declare-only. All four already declare `slo_tier = "<5s"`.

## Folded annotations

- **PREMISE (P-025, research not task).** The Halo State Pulse canvas is orphaned at Pulse HEAD; the
  severity→hue semantic lives on the **constellation dot** (`severityToHueFraction`,
  `pulse-app/ui/src/widget/constellation-types.ts`) and `hue_update_ms` measures that. `halo-hue-encoding.toml:27`
  still grades the hue in halo wording by operator observation. Expect a **premise refine** on the scenario's
  wording; **the capability semantic is UNCHANGED**. The canvas's own fate is Pulse's entry — never scoped here.
- **PREREQ.** Re-check `cargo audit` — the **37th consecutive** standing deferral. PROBE-AUTO-SATISFY
  signature: `cargo audit` true exit 1 whose first diagnostic line is `duplicate advisory ID: RUSTSEC-2026-0244`
  **and** `cargo deny check advisories bans licenses sources` true exit 0. Reproduce byte-identically → the pin
  is satisfied by the one-line record `probe unchanged, 37th consecutive`, no basis re-authoring. ANY deviation
  (changed diagnostic, moved exit code, overlap shift, a dependency delta that ADMITS a package) restores the
  FULL form. Remedy stays the bounded wait — no floor raise, no `deny.toml` ignore, no CI edit.
  Rationale: `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

## Boundaries

- **Not** a Pulse-side change. The observables exist; this chunk consumes them and never edits that repo.
- **Not** the Halo State Pulse canvas disposition — Pulse owns that entry.
- **Not** UI automation of Pulse, and **not** Pulse process management (both stated non-goals).
- **Not** `v2-30` (live per-P-ID verdict lamps) — a separate Epoch-5 entry despite the adjacency.
- Evidence rule: **cite** the Pulse chunk's report and its P-075 `notes`; never copy them into this repo.

## Surfaces / contracts touched (narrowed at P3 by measurement)

`scenarios/{halo-hue-encoding,service-constellation-discovery,report-render-surface,findings-counter-refresh}.toml`
(headers only) · a new harvest-tier test under `crates/conductor-run/tests/` ·
`conductor-0.2.0/verification-matrix.json#v2-20`.

**Removed from the original list, each for a measured reason** — the pre-closure draft named these
speculatively:
- the run-report envelope + per-check `CheckRecord`, and the `conductor-verify` read-back/verdict path —
  NOT touched, because `budget_ms`/`effective_deadline_ms` grade Conductor's read-back round-trip and are
  the wrong instrument for a Pulse-internal duration (premise 2 above).
- `coverage-matrix` classification + the `UNBACKED_AUTO` pin — NOT touched: P-045 is `Auto` and already
  backed; P-025/P-027/P-037 are `DriveObserve` and do not participate in `check_scenario_backing`. No mode
  change is required to grade the budgets, so neither coverage gate moves.
