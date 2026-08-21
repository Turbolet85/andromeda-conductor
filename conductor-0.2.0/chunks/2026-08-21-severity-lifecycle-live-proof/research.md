# Codebase Research — 2026-08-21-severity-lifecycle-live-proof

## Scope
- **Depth:** deep (mature codebase + SUT-source verification) · **Reads:** 14 targeted file/section reads (7 Conductor · 7 Pulse) · **Globs/Greps:** ~18

## Files inspected
- `scenarios/severity-tier-{autonomous,suggested,curious}.toml` (full) — one emitting phase each (`error` kind: 8@100% / 12@40% / 10@10%), one `Contains` CalibrationRegion check each ("Autonomous"/"Suggested"/"Curious"); seeds 4317019-21; tiers `<5s`/`<20s`/`<90s`.
- `scenarios/incident-auto-resolution.toml` (full) — 5 phases (trigger 8×identical exceptions · sustain-10min 600000ms/60 · cease · wait-120s · retrigger 8×identical); 3 checks: `Contains "Resolved"` Hard · `CountAtLeast "2"` Hard · `Contains "resolution summary"` CalibrationRegion; tier `<90s`.
- `scenarios/ack-cooldown.toml` (full) — 5 phases with **NO `[phases.emission]` tables at all**; 1 check `CountAtLeast "2"` Hard; the TOML's own Q2 note records ack has no tool in the four-tool contract.
- `crates/conductor-run/src/lib.rs:330-470` — `execute_scenario` flow; `route_read_back` + `enum ReadBack{Graded,AutoResolved,Blocked}` at `:444-463`; the AutoResolved arm sets `Observation{degraded:true,..}` so the declare-only row rides the existing KnownResidual mapping; unit-pinned ×3, unexercised live.
- `crates/conductor-verify/src/extract.rs:1-140` — `observe()` is ONE-SHOT: `query_incident_list` (empty ⇒ `EmptyCorpus`) then per-ACTIVE-id `retrieve_report` + `retrieve_telemetry_slice`; `Observation.text` = list `status`/`severity`/`title` + report markdowns; `observed_for`: `CountAtLeast` ⇒ `evidence_count` (Σ `span_refs`), substring kinds ⇒ composed text.
- `crates/conductor-verify/src/client.rs:26,147-148` — `mark_incident_resolved` client method exists (raw `{resolved,incident_id}`), **no production call site** (tests only: `preflight.rs`, `common/mod.rs`).
- `crates/conductor-core/src/phase_spec.rs:34-53` — `PhaseSpec.emission` is `#[serde(default)]` **defaulting to one plain trace** (not silence); `gap_ms` = inter-phase gap before the boundary.
- `crates/conductor-core/src/coverage.rs:107-111,147-148` — ALL SEVEN family P-IDs are `CoverageMode::Auto` (P-022 = "Auto-Resolution and Lifecycle"); the TOMLs' "drive+observe" comments are stale prose.
- `crates/conductor-core/src/drift.rs:61-62` — `UNBACKED_AUTO` = P-031/033/034/039/041-044; none of the family. Check retirement keeps `p_ids` ⇒ backing intact ⇒ neither gate disturbed.
- `crates/conductor-core/src/redact.rs` — allowlist carries `phase_count`; NONE of obs-plan CP4's `auto_resolve_triggered`/`summary_received`/`severity_level`/`lifecycle_phase`/`severity_choice_calibrated`.
- `crates/conductor-run/tests/` — harvest precedents confirmed: `storm_harvest.rs` · `baseline_harvest.rs` · `restart_harvest.rs` · `pii_harvest.rs` · `connection_harvest.rs` (own-binary rule).
- `contracts/pulse-run-contract.toml:23-60` — `warmup_ms=45000` · `warmup_emissions=3` · `min_canary_poll_seconds=90`; terms l4-deterministic (shell-declaration) · shared-data-dir (declared-not-observable) · sidecar-built/incident-formation/load-envelope (asserted).
- `contracts/pulse-load-envelope.toml:25-32` — `max_sustained_rate_spans_per_s=10000` · `max_sustained_storm_ms=600000` (the shipped sustain phase sits AT the bound) · `max_scenario_duration_ms=600000` recorded-not-asserted.
- `contracts/pulse-capabilities.toml` — bare id list (no labels); P-019..023/059/060 all accepted.
- `scripts/agent-run.sh:81-107` — `SCENARIO=<name|P-ID> [SEED=<n>] run` → `conductor run <scenario> --agent-mode`; `--seed` only on explicit SEED; no wrapper timeout on `run` (boot owns the budget).
- **SUT (andromeda-pulse @ HEAD `efabe8e` — unmoved since the 2026-08-17 visit):**
  - `crates/mcp-server/src/tools.rs:336-467,487` — `query_incident_list` → `load_active_incidents` (**ACTIVE ONLY**; own test `query_incident_list_returns_active_incidents`, ":933 no longer in the active list"); items = `{incident_id,status,severity,title,opened_at_unix_nano}`; `severity_str` ∈ `info/warn/error/critical`; `retrieve_report` loads by id (any status), `degraded_mode = resolution_summary_text-does-not-parse-as-L4Output`; `mark_incident_resolved` sets corpus status=resolved directly (cross-process; pulse-app registry unaware).
  - `crates/triage/src/incident/{persistence,registry,state_machine}.rs` — `DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS=120` (boundary-inclusive); `record_reemission` resets the timer and ERRORS on Resolved; `attach_resolution_summary` requires status==Resolved; `evaluate_auto_resolution` filters non-Resolved.
  - `pulse-app/src/incident_observer.rs` — `AutoResolveObserver` 30s ticks; per transition: registry `mark_resolved` → persistence `update_incident_status` → lifecycle broadcast (to_state=Resolved); aggregate tick line `triage.incident.auto_resolve.tick` (evaluated_count/resolved_count/duration_ms).
  - `pulse-app/src/inference_runtime.rs:110-160,512-560,627-740` — L4 subscriber: `DigestKind::ResolutionSummary` OR `parsed.is_resolution_summary` ⇒ attach; else `create_incident_from_l4_output`. Incident identity = **(cue.kind, scope, scope_id) — per-service for storms**; dedup searches `list_active` then `observe_reemission` (+persist) — **a Resolved incident is not found ⇒ a NEW incident is created (new-not-reopen HOLDS)**; Reflection digests mint a workspace-global `ReflectionTrend` incident (dedup one-per-workspace, refreshed per digest); non-cue baseline digests SKIP; new incidents carry `fingerprint = cue_fingerprint` (the real L1 hash) and **`evidence_refs.span_ids = Vec::new()`**.
  - `crates/triage/src/cadence/coordinator.rs:142-230,325-440` — Tier-1 digest **only on `PriorityTier::Autonomous` cues**; Tier-2 acceleration **skipped on CpuPrimary** (`tier2_skipped_cpu_primary`); Tier-3/Reflection interval-driven (`baseline_seconds`/`reflection_seconds`, config-sourced).
  - `crates/triage/src/cue/evaluate.rs:32-68` + `baseline/mod.rs:447,513` — error-rate cue: `short_term_error_rate` (30s **per-sample EWMA** — frozen when samples stop) vs `max(long_term, base_error_rate)`; `is_finite` guard only.
  - `pulse-app/src/deterministic_inference.rs:30-80` — `CANNED_L4_OUTPUT_JSON`: `severity:"autonomous"` (→`Error`→list string `"error"`), `is_resolution_summary:false`, det-* evidence triple, `decision:"surface"`.
  - `crates/interpretation/src/markdown.rs:205-225,355` — `## Resolution Summary` section renders only when `report.resolution_summary` is Some; status labels render lowercase (`"resolved"`).
- `conductor-0.2.0/chunks/2026-08-20-latency-regression-re-proof/{report,evidence/leg-verdict}.md` — the 375-cue refresh measurement + the arm's honest limit + cue-band table (suggested ≥3x, autonomous ≥5x + conf ≥0.9, conf = samples/100).

## Graph impact
- **`route_read_back`** — callers: `execute_scenario` (`conductor-run/src/lib.rs:372`) + its own 4 unit tests (`:778-818`); crate-private, zero cross-crate callers. This chunk changes NO production signature — trace: `tree-query-2026-08-21-severity-lifecycle-live-proof.json` (rows: 7, all in-crate).
- **`mark_incident_resolved`** — client method + const, referenced from `conductor-verify` tests only; no run-path caller. A live exercise (if taken) is a LEG step through the existing client, not new plumbing.

## Patterns detected
- **Harvest-tier grading** (`crates/conductor-run/tests/*_harvest.rs`, five precedents): test-only harvest of Pulse's verbatim tracing lines pinned at the `conductor-run` unit tier; own test binary per file (test-plan §11 singleton ban).
- **Declare-only retirement with measurement header** (`scenarios/fingerprint-storm.toml` etc. + arch §Standard Contracts SIX families): checks removed, measurement recorded in the TOML header; row lands `verdict:null`/`KnownResidual`.
- **AutoResolved residual routing** (`conductor-run/src/lib.rs:444-463`): declare-only + `EmptyCorpus` ⇒ KnownResidual via `degraded:true`; checks-bearing + empty ⇒ `Blocked`; `CallFailed` ⇒ `Blocked` always.
- **Fresh-dir leg discipline** (prior five live proofs): per-leg `%TEMP%/pulse-legs/<n>` data dir, pulse-app + preflight per leg, `SCENARIO=<name>` without `SEED`, evidence under `chunks/{marker}/evidence/`.
- **Split emission identities** (`conductor` vs `conductor-canary`, arch §Occupied Resources): the canary's incident is isolated from scenario-service dilution — WHY its frozen EWMA holds its incident active indefinitely at current HEAD.

## Conventions to follow
- **Verdict/error wall** — every leg outcome a typed value; read-back faults → `VerifyError` → `Blocked` (`conductor-verify/src/extract.rs:63-74`).
- **Journal-relative wall-clock stamps** (`conductor-run/src/lib.rs:349-351,436-437` `now_ms`/`now_rfc3339` from `std::time`).
- **Two-runner + zero-retry gate** (test-plan §4/§10): nextest ci profile AND `cargo test -p conductor-run`; new harvest binary must pass both.
- **Redaction-clean evidence** (obs §11): no host paths / struct names in TOML headers, harvest pins, leg-verdict.

## New files to create
- `crates/conductor-run/tests/severity_harvest.rs` — the family's harvest-tier pins (own binary): auto-resolve tick + lifecycle-Resolved lines, cue-tier ladder lines, two-incident new-not-reopen ids, resolution-summary ABSENCE witness.
- `conductor-0.2.0/chunks/2026-08-21-severity-lifecycle-live-proof/evidence/leg-verdict.md` (+ captured leg logs) — the live evidence trail.

## Files to modify
- `scenarios/incident-auto-resolution.toml` — re-shape for the live path (autonomous-band trigger ≥12; OK-span dilution tail before cease so the short error EWMA ends below threshold; ≥150s resolve wait ≥ 120s window + 30s tick + margin; retrigger ≥12) + retire the 3 checks declare-only with the measurement header.
- `scenarios/severity-tier-autonomous.toml` / `-suggested.toml` / `-curious.toml` — retire the fixture-pinned `Contains` checks declare-only with measurements; re-shape stimuli onto the proven converged-baseline+spike recipe targeting the three cue bands (magnitude/conf derived from SUT constants).
- `scenarios/ack-cooldown.toml` — retire the `CountAtLeast` check declare-only with the measurement (no emission declared ⇒ 5 default plain spans, forms nothing; ack undrivable via the four-tool contract; `CountAtLeast` grades span_refs which are always 0).
- `conductor-0.2.0/verification-matrix.json` — v2-16 claim link + concretized acceptance (P5).
<!-- Caller threading: none — zero production-source changes planned; `execute_scenario`'s callers are
unaffected (graph: route_read_back has no cross-crate callers). The load-bearing mechanism equalities
verified above: active-list-only observation (tools.rs:336-357), evidence_count == Σ span_refs == 0
under the incident producer (extract.rs:53-60 × inference_runtime.rs:727-731), list_active-scoped dedup
⇒ new-not-reopen (inference_runtime.rs:686-708), attach reachable only via never-constructed digest
kind or pinned-false flag (inference_runtime.rs:147-149 + contract.rs:275 sole enum ref). Seam facts:
no new deps, no re-exports, no manifest edits — the harvest test rides existing conductor-run dev-deps. -->

## Open questions
- Do the re-shaped tier stimuli actually land in their predicted cue bands (suggested/curious), and does a Tier-3 digest carry a suggested cue into an incident on this host? → blocks: implementation-scope (leg measures; TOML shapes provisional until the leg).
- Does the persist loop resurrect a sidecar-`mark_incident_resolved`-resolved canary row (cross-process fight), i.e. can the AutoResolved arm fire live at all? → blocks: implementation-scope (a leg step measures it; the plan does NOT depend on the arm firing).
- Does a `ReflectionTrend` incident appear within a ~13-minute leg (`reflection_seconds` config-sourced, not read statically)? → blocks: implementation-scope (leg design places read-back accordingly; measured on the leg).
