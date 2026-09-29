# Codebase Research — 2026-09-29-hue-shift-budget-graded-hard

## Scope
- **Depth:** moderate on the Conductor side (one test binary, one scenario header, one contract document, two
  harness comments) and deep on the Pulse side (every contract coordinate re-read at Pulse's committed HEAD).
- **Reads:** 19 (Conductor 13, Pulse 6 by `git show 226554a:<path>`) · **Globs/Greps:** 14, plus one delegated
  read-only Pulse re-verification whose load-bearing claims were spot-checked here (below).
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read in full (28 Session Additions). The
  firing-form items applied:
  - `:54` / `:61`: the PATH prefix in POSIX form;
  - `:47`: the paired `RUST_LOG`;
  - the fresh data dir per leg under `%TEMP%/pulse-legs/<ts>`;
  - `:56` (a): never `boot` before `conductor run`;
  - `:58`: a census before and after, and the stop form beside the firing form;
  - `:63`: a pre-leg line count on a fresh dir (~2.2k boot lines);
  - `:72`: the self-obs sink truncates per invocation, so the leg's stream is frozen before anything else runs;
  - 2026-09-10: `expect` atoms are read from a recorded print site.

  Also `.claude/rules/host-win32.md` (auto-loaded) and `.claude/rules/testing.md` (auto-loaded; the 2026-06-22
  companion-sweep rule as extended 2026-09-10 / 2026-09-15, and the 2026-09-07 nextest selector rule).
- **Platform issues consulted:** none. There is no runner-only bullet: Setup 5a read `cdb7082` green
  (CI#36621215714), and the chunk's leg is operator-local.

## Files inspected
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (full, 700 lines) — the P-025 harvest home.
  - `bounds()` (`:59-86`) pins P-025 as `metric.constellation.hue_update_ms` / `duration_ms` / 2 000 ms.
  - `grade()` (`:119-138`) is worst-observation, inclusive boundary, and absence is `Err` "UNGRADED".
  - `hue_samples_in_window()` (`:207-223`) is the TEMPORAL attribution.
  - `timestamp_ms()` (`:154-178`) and `days_from_civil()` (`:182-190`) are the line-stamp parse.
  - `tick_times_ms()` / `tick_offset_ms()` (`:226-245`) and `TICK_OFFSET_TOLERANCE_MS` (`:149`) are the RETIRED
    instrument's mechanism helpers.
  - The P-025 tests are the 2026-08-21 disproof (`:473-521`, verbatim lines 35 581.44 / 36 704.98), the synthetic
    selection and mechanism tests (`:528-630`), and the 2026-09-07 re-driven pin (`:632-699`), which pins the leg H
    line and its tick and the window `(1_788_767_041_678, 1_788_767_195_445)`.
  - The module doc (`:1-44`) states P-025 is "disproved and recorded", never a pass.
  - The contract cites this file for the raw 36 704.983642578125 reading (contract `:135-137`).
- `scenarios/halo-hue-encoding.toml` (full). Header `:1-58` is the stale mechanism prose, and `:80-86` is the
  checklist comment ("unmeasurable through this leaf").
  - Phase data `:59-78`: seed `4317025`, `slo_tier = "<90s"`, jitter 50, `healthy-baseline` 60 plain @ 30 s,
    `error-pressure` 300 identical exceptions @ 150 s.
  - One `[[checklist]]` item `:87-89`.
- `contracts/pulse-p025-measurement-contract.md` (full). `pinned_at = "Pulse HEAD 83d4060"`, with every coordinate
  re-verified below.
- `scripts/agent-run.sh` `:63-155` — `live_leg()` (`:79-90`): `timeout "$budget" "$CARGO" run -q -p conductor-cli
  --bin conductor -- run "$scenario" --agent-mode`, then `cp "$LIVE_LOG" "$LIVE_CAPTURE_DIR/${label}.jsonl"` with the
  print sites `[live] leg {label}: {scenario}` (`:81`) and `[live] leg {label}: froze N self-obs lines` (`:86`).
  - The leg H comment `:109-114` says "the hue observable fires against a tick-refreshed last_seen", which is stale.
  - The banner `:146-151` ("inert for leg H — the storm path consults no baseline") still holds.
  - `live_leg_budget_sec` is at `:74`.
- `scripts/agent-run.ps1` `:175-191` — the same leg H comment at `:175-179`, stale in the same words (`grep -n
  'tick-refreshed' scripts/agent-run.{sh,ps1}` → `.sh:111`, `.ps1:177`).
- `crates/conductor-run/tests/live_suite.rs` (full) — the `live-pulse`-gated capture reader. It reads `b1`/`b2`/`a`
  only, never `h`, and this chunk leaves it alone.
- `crates/conductor-run/tests/capture_paths/mod.rs` (full) — `runs_dir_from` / `pulse_logs_dir_from` guards. This
  chunk adds NO env-reading test, so no new reader of either handle.
- `crates/conductor-run/tests/real_model_harvest.rs` `:1453-1510` — the committed-evidence loader precedent
  (`committed_capture()` from `CARGO_MANIFEST_DIR` plus a hard-coded path, beside a literal `PINNED_CAPTURE`).
- `crates/conductor-run/tests/live_suite_harvest.rs` `:1-20` — the literal-pin convention: "no harvest reads a path
  at grading time … `evidence/` keeps the full per-leg copies as the human-readable record".
- `crates/conductor-core/src/coverage.rs` `:238-243` — P-025 is `CoverageMode::DriveObserve`, and nothing here
  moves it.
- `conductor-0.2.0/chunks/2026-09-06-halo-hue-budget-re-driven/evidence/hue-verdict.md` (full) and that plan's
  steps 5-10. These give the prior leg's capture method: a Pulse-log pre-leg line count taken after readiness and
  before dispatch; the window taken from the FROZEN Conductor self-obs (`evidence/h.jsonl`), running from phase-2
  start to the `scenario.run` close; and Pulse lines pinned verbatim in test source.
- Pulse at committed `226554a` (read-only, never the dirty worktree):
  - `crates/triage/src/incident/tier_effective.rs:20-63`;
  - `pulse-app/ui/src/widget/constellation-types.ts:228-276`;
  - `pulse-app/src/services_router.rs:95-112`;
  - `pulse-app/src/observability.rs:984-1003, 1373-1386, 2213-2218, 2398-2407`;
  - Pulse's own chunk report and `evidence/green-leg.md` (at
    `andromeda-pulse-0.3.0/chunks/2026-09-29-p-025-hue-shift-observable-made-gradable/`).

## Graph impact
- **`hue_samples_in_window`** — 4 callers, all in-file tests of `delegated_timing_harvest.rs`:
  - `the_window_selects_the_scenarios_own_samples_and_excludes_the_canarys @ :574`;
  - `a_hue_sample_reports_its_offset_to_the_preceding_lifecycle_tick @ :600`;
  - `an_empty_window_yields_no_samples_rather_than_a_satisfied_budget @ :623`;
  - `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency @ :671`.

  Basis: `calls WHERE callee_name = 'hue_samples_in_window' AND callee_kind = 'fn'`, rust plane, rows 4, db fresh
  (trail `tree-query-2026-09-29-hue-shift-budget-graded-hard.json`); the lines are the graph's 0-indexed + 1.
- The change is test-local: the helpers are private to one integration-test binary, with no cross-file caller and
  no production symbol touched.

## Pulse coordinate re-verification (the overseer's "every coordinate", at Pulse `226554a`)
Pulse `226554a` is the local `chore/migrate-pulse-to-v3` HEAD, equal to `origin/chore/migrate-pulse-to-v3`, with
`e98d838` an ancestor. The worktree is dirty with another session's uncommitted `ci.yml` / `xtask/src/*` /
`quality_gate_workflow.rs` edits, so every read below is `git show 226554a:<path>`.

Spot-checked by me beyond the delegated read:
- `tier_effective.rs:23-63`: the replay pushes `(opened_at, rank, +1)` per incident and `(resolved_at, rank, −1)`
  only for `Resolved` rows.
- `constellation-types.ts:245-276`: one sample per changed dot, witnessed only (`effectiveMs < mountedAtMs` →
  skip), `severity_tier: tier ?? "none"`, `duration_ms: Math.max(0, paintNowMs − effectiveMs)`.
- `services_router.rs:95-111`: one `list_for_workspace` snapshot feeds both `priority_tier` (`status != Resolved`)
  and `tier_effective_at_unix_nano`.
- `observability.rs:989-992`: the hue leaf is admitted as `["duration_ms", "severity_tier"]`.
- `observability.rs:1373-1386` / `:2398-2404`: both redaction-precedent leaves are FULL.
- `observability.rs:2214`: `interpretation.incident.created` admits `created` / `deduped` / `severity` /
  `priority_tier`, with no id and no fingerprint.

| contract / header coordinate | verdict at `226554a` | reading |
|---|---|---|
| leaf `metric.constellation.hue_update_ms` {`duration_ms`,`severity_tier`} | HOLDS | emit `crates/ui-bridge/src/telemetry.rs:278-282`; allowlist `observability.rs:989-992` |
| `discovery_ms` / `counter_refresh_ms` carry `duration_ms`; `render_ms` carries `value` | HOLDS | `telemetry.rs:295-298`, `:311-313`; `incidents_router.rs:563-565`; allowlist `observability.rs:993-1003`, `:2405-2407` |
| allowlist test `unit_observability_allowlist_delegated_timing.rs` | HOLDS | `:7-8`, `:27`; unchanged since `83d4060` |
| `triage.cue.tick` / `auto_resolve.tick` counters render `<redacted>` | CHANGED (history) | both leaves are full at `observability.rs:1373-1385` / `:2398-2404`, as they already were at `83d4060`; the measurements behind the sentence (2026-08-18 / -21) came from an older HEAD |
| `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL` 15 s, no env override | HOLDS | `crates/triage/src/lifecycle/mod.rs:52`; wired `pulse-app/src/main.rs:1489`; first tick skipped `:95-96` |
| `last_seen_unix_nano` has no ingest writer | HOLDS, but it is OFF the hue path | tick `lifecycle/registry.rs:334,340`; override / restart / restore `:215,:249,:261,:292` |
| tier computed from `list_active()` + `.max_by_key(tier_rank)` | CHANGED | `list_for_workspace` (Resolved included) then filtered `status != Resolved` (`services_router.rs:97-109`); same set |
| "that site holds both timestamps" | CHANGED (now true) | true only for the rise at `83d4060`; `list_for_workspace` (`incident/registry.rs:76-80,229-235`) made it true |
| `ServiceListItem` carries no tier timestamp | CHANGED | new `tier_effective_at_unix_nano: Option<i64>` (`lifecycle/registry.rs:54-69`; TS binding `ui/src/bindings/index.ts`) |
| rise = `opened_at_unix_nano` (`crates/triage/src/contract.rs`) | HOLDS | `contract.rs:418`; replay `tier_effective.rs:36` |
| fall = `transitioned_at_unix_nano` (`incident/broadcast.rs`) | CHANGED — **CARRY premise 1 VERIFIED** | the field exists (`broadcast.rs:41`) but the fall is `resolved_at_unix_nano` (`tier_effective.rs:30-35`) |
| fall "by resolution **or acknowledgement**" | CHANGED — **CARRY premise 2 VERIFIED** | Acknowledged stays active (`registry.rs:219-227`, `services_router.rs:104`); test `tier_effective.rs:167-171` `acknowledgement_keeps_the_tier` |
| `priority_tier` immutable after opening | HOLDS | the only field assignment is `services_router.rs:101` (the DERIVED item); the production `Incident` construction is `inference_runtime.rs:847/868`; the dedupe path only calls `observe_reemission` |
| "eight writes in total" | CHANGED (count not reproducible) | 11 non-test writes of a `priority_tier` field, the same set at `83d4060` (AttentionCue ×4, DigestCueRef ×2, Incident ×1, DTO ×2, ServiceListItem ×1, the assignment ×1); 5 on the incident chain alone. The substance (no in-place tier change) holds |
| corpus incident UPDATEs set only status/updated/resolved/payload/read | HOLDS | `crates/corpus/src/contract.rs:652`, `:682`; no `priority_tier` column (`corpus/src/schema.rs:58-67`) |
| fire site `ConstellationCanvas.tsx` staleness + slowest-wins | CHANGED | effect `:119-136`, `hueShiftSamples(...)` + one IPC call per sample (`:133-135`); no `last_seen` read |
| `last_observed_unix_nanos` exists and is stamped per observation | HOLDS | `baseline/activity_floor.rs:84`, `:137` |
| quiet-duration truncation | HOLDS (off the hue path) | `activity_floor.rs:142-148`; gate `lifecycle/registry.rs:339-341` |
| resolution = the 15 s tick | CHANGED | backend ns ÷ 1e6 against the webview's `Date.now()` ms, the same host wall clock (`constellation-types.ts:266,271`) |
| header `{data_dir}/logs/agent-latest.jsonl.<date>` | HOLDS | `observability.rs:2612-2614`, a daily roll; the sidecar logs to the same family (`mcp-server/src/tracing_setup.rs:249-256`) |
| header "no MCP tool reads that stream" | HOLDS | 8 tools `mcp-server/src/tools.rs:44-51` |
| header `ConstellationCanvas.tsx:117-155` / `:152` | MOVED + CHANGED | effect `:119-136`, call `:134` |
| header term (1) SLOWEST-WINS | GONE | one sample per changed service (`constellation-types.ts:253-273`) |
| header term (2) 60 s `LIVE_RECENCY_WINDOW_NANOS` at `constellation-types.ts:27` | HOLDS | the ceiling is now also a clamp: `canvas/frame-metrics.ts:28-40` sets >60 000 to 60 000, and `telemetry.rs:75,84` rejects >60 000 |
| header term (3) QUANTIZATION (`registry.rs:336`, `lifecycle/mod.rs:52`, `main.rs:1489`) | GONE from `hue_update_ms` | refresh line MOVED to `lifecycle/registry.rs:340`; no tick on the hue path |
| header "Aggregate-only", no service id | HOLDS | `observability.rs:984-992`; the ban test is in the allowlist test file |
| header: incidents form only from Autonomous cues | HOLDS | `cadence/coordinator.rs:395`; the incident tier comes from L4 severity (`inference_runtime.rs:686-691`) |
| header: Halo canvas has no production mount; the hue lives on the constellation dot | HOLDS | no non-test `<HaloCanvas`; `severityToHueFraction` is used at `constellation-types.ts:13,182` and defined at `halo/severity-to-halo.ts:41` |
| header: `CompactWidget.tsx` is the only mount | HOLDS | `widget/CompactWidget.tsx:90`; the dashboard `traces/ConstellationCanvas.tsx` emits no hue metric |

## Patterns detected
- **The contracted window is what Pulse now emits** (`constellation-types.ts:245-276`, `tier_effective.rs:23-63`).
  - `duration_ms = paint(Date.now()) − tier_effective_at`.
  - rise: the max-raising incident's `opened_at`; fall: the last max-holder's `resolved_at`; acknowledgement is inert.
  - One sample per changed service, only when witnessed (`effective ≥ mount`), with the NEW tier (`none` on a fall
    to no incident).
- **The equality the grade rests on**, stated at the criterion's grain. For an in-window sample of the driven
  service, `timestamp − duration_ms` equals that service's incident-open instant, to within the paint→log IPC and
  the L4 generation time (≈0 under deterministic L4).
  - The frame that produces the quantity is `hueShiftSamples` (`:271`, the paint instant minus
    `tier_effective_at_unix_nano / 1e6`). The backend witness is `interpretation.incident.created`
    (`created=true`), whose allowlist carries no id or fingerprint (`observability.rs:2214`).
  - Measured by Pulse's own leg: anchor error 44 ms / 36 ms against that record (`evidence/green-leg.md`), with the
    rise's `duration_ms` 9 986 on a FIRST SIGHTING (the service listed only after its first 15 s tick).
- **A service is listed only after its first lifecycle tick** (`lifecycle/registry.rs:320-337` `or_insert`; tick
  15 s, first skipped at `lifecycle/mod.rs:95-96`).
  - `halo-hue-encoding`'s 30 s `healthy-baseline` runs before its storm, so the `conductor` dot exists before the
    rise. That puts the rise in the poll-bounded case: `items` refresh every 1 s (`use-service-constellation.ts:22`).
    Pulse's own falls measured 510 / 578 ms in that case.
- **Temporal attribution still holds, re-checked against per-service emission.** The leaf carries no service id
  (`HueShiftSample` is `{duration_ms, severity_tier}`; the line's `service.name` is Pulse's own
  `com.andromeda.pulse`).
  - The canary's rise precedes `scenario.run` (preflight readiness requires its incident).
  - Its dot leaves `visibleDots` 60 s after its storm, and a hidden dot is never sampled, so its fall (~150 s
    later, possibly inside phase 2) emits NOTHING in-window.
  - A hidden service's REMEMBERED tier fires a stale sample, clamped to exactly 60 000, when it reappears. That can
    only land when a service becomes live again: the canary at its warm-up, before `scenario.run`; the `conductor`
    service at phase-1 start. Both are outside a window that opens at phase-2 start, and a fresh `pulse-app` launch
    removes the case entirely.
- **What the window measures, precisely** (a property of the contract, not a defect): incident open → repaint. Cue
  detection, digest cadence and L3 are outside it by the contract's own §The window. Under deterministic L4 the
  generation time between `opened_at` and the registry insert is ≈0.
- **Harvest convention: literal pins in test source** (`live_suite_harvest.rs:5-7`, `delegated_timing_harvest.rs:415-699`).
  The `evidence/` tree keeps the full copies as the human-readable record. The 2026-09-29 BREACH (security-plan
  §Data Protection) is corpus-rendered text in test source. Hue lines carry `duration_ms` / `severity_tier` plus
  Pulse's service identity only, and `interpretation.incident.created` carries four closed fields, so no corpus
  text is involved.

## Conventions to follow
- **Absence is never a pass** (`delegated_timing_harvest.rs:21-22`, `grade()` `:125-130`): an empty window is
  `Err` "UNGRADED".
- **Worst observation grades the budget, with an inclusive boundary** (`:114-138`; tests `:343-373`).
- **Verbatim live lines carry their run id and pre-leg count in the doc comment** (`:420`, `:632-651`).
- **The window literal is pinned beside the lines it selects** (`:669`).
- **A test binary's cwd is its package root**: any committed-file read uses `CARGO_MANIFEST_DIR` + `../../`
  (`real_model_harvest.rs:1494`), never a `CONDUCTOR_*` handle.
- **Live-leg firing form** (`live_suite.rs:13-24`, `agent-run.sh:79-90`):
  - the whole env block in one paste: PATH prefix to Pulse's `target/release` (POSIX form),
    `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_MCP_ENABLED=true`, `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`;
  - `conductor run <scenario> --agent-mode` under a `timeout`;
  - freeze `logs/agent-latest.jsonl` before anything else writes it.
- **Pulse binary provenance by content**: Pulse proved its own leg's subject by the field name appearing in the
  binary (`evidence/green-leg.md`: `tier_effective_at_unix_nano` 2× in `pulse-app.exe`).

## New files to create
- `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/` — the leg's frozen Conductor self-obs
  (`h.jsonl`), the in-window Pulse hue and incident-created lines, the verdict note with the SUT HEAD, the posture,
  the pre-leg count, the window, the samples and the anchor errors, and the censuses

## Files to modify
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — the P-025 hard grade over the new leg's pinned lines
- `contracts/pulse-p025-measurement-contract.md` — re-pinned to `226554a`; premise corrections; the grading rule stated before the drive
- `scenarios/halo-hue-encoding.toml` — header comments only; phase data and checklist byte-unchanged
- `scripts/agent-run.sh` — the leg H comment only
- `scripts/agent-run.ps1` — the leg H comment only

## Companion sweep (`testing.md` 2026-06-22 as extended; `grep -rln "halo-hue-encoding" crates/ scripts/ contracts/ .github/` → 11 files)
Every hit was read. The pattern `halo-hue-encoding` has 11 files · 4 changed · 7 no-change (dispositions below):
- `crates/conductor-core/src/scenario.rs` (`:1139`, `:1160`, `:1206`) — pins `p_ids = ["P-025"]` and empty
  `expected`; no change (neither moves).
- `crates/conductor-run/tests/operator_pause_harvest.rs` (`:70`, `:111`, `:141`) — pins the `[[checklist]]` text;
  no change (the checklist is byte-unchanged).
- `crates/conductor-tauri/ui/test/a11y/{operator-hold.e2e.ts, screen-reader.e2e.ts}`,
  `…/screen-reader/{rows.ts, nvda-pass-spec.md}` — catalog-name references for the hold / SR rows; no change (name
  and checklist unchanged).
- `contracts/scenario-audit-ledger.toml:66` — the `[[over_tier]]` row; no change (`slo_tier` unchanged).
- `crates/conductor-run/tests/delegated_timing_harvest.rs`, `contracts/pulse-p025-measurement-contract.md`,
  `scripts/agent-run.{sh,ps1}` — changed (listed above). The scenario file itself is outside the swept directories
  and is listed above as changed (comments only).
- Seed `4317025`: `grep -rln 4317025 crates/ scenarios/ contracts/` → `scenarios/halo-hue-encoding.toml` only, so
  no seed-named replay golden exists for it. There is no phase-data change either way, so no load-envelope input
  moves.

## Open questions
- If the one graded drive measures an in-window sample over 2 000 ms (a hard `Fail`), does `v3-08` stand ("graded
  hard at a real measured value"), or does the claim require a Pass? The rule must be fixed before the drive. →
  blocks: plan-decision (P4 fork).
- Should the leg also witness a FALL in-window? The scenario has no healthy tail. Its incident auto-resolves ~120-150
  s after its last deduped re-emission, by which time the dot is hidden (quiet over 60 s), so today the leg grades
  RISE samples (plus any mid-storm resolve / re-open pair). Adding a healthy tail phase moves phase data (audit
  ledger, load envelope, golden, and the read-back route: an auto-resolved incident flips the envelope to
  `KnownResidual`). → blocks: plan-decision (P4 fork).
