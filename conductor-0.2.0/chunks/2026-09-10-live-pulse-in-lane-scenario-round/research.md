# Codebase Research — 2026-09-10-live-pulse-in-lane-scenario-round

## Scope
- **Depth:** deep · **Reads:** 18 (11 Conductor · 4 Pulse · 1 matrix · 2 prior plans) · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full as a **structural
  extraction** (63 lines, 64 849 bytes, longest line 13 378 chars: `awk` index of every entry by
  line + length, then offset-bounded reads of entries 18–22, 27–33, 44–45, **47–58, 60**) · 1 live-leg
  recipe item newly applied, 6 re-confirmed · `.claude/rules/observability.md` — read in full (40 lines;
  the second rule whose `paths:` covers `scripts/agent-run.*`). `testing.md` scopes to `scenarios/**` +
  `crates/**/src/**/*.rs`, **not** to the harness scripts, so it is not in the required harness set.

## Graph impact
Plane `rust`, `db_state` warm (not cold-start). Trace: `.andromeda/runs/2026-09-10T10-39-57-phase/tree-query-2026-09-10-live-pulse-in-lane-scenario-round.json`.

- **`observe`** — `rows: 0` **AND `probe_hits` 0**, so this is a **query-pattern/index miss, NOT a leaf**;
  the script warned exactly that. `conductor-verify` has **323 indexed symbols** and two siblings
  *from the same file* are indexed (`extract/impl#[Observation]observed_for()` @ `extract.rs:53`,
  `extract/log_observed_keys()` @ `extract.rs:229`), so the gap is symbol-level, not plane-level.
  Resolved by grep per the cookbook: **exactly 2 call sites**, one production —
  `crates/conductor-run/src/execute.rs:98` — and one test (`conductor-verify/tests/readback.rs:22`).
- **`CountAtLeast`** — indexed, `type`, `conductor-core` @ `crates/conductor-core/src/expected.rs:44`
  (graph `def_line` 43 is 0-indexed).

## Files inspected
- `crates/conductor-run/src/execute.rs` (70–160, 208–240) — the read-back call, the declare-only branch, `route_read_back`, `manual_record`, `state_for`.
- `crates/conductor-verify/src/extract.rs` (33–130, 160–265) — `observe`, `Observation`, `evidence_count`, `observed_for`.
- `crates/conductor-verify/src/slo.rs` (48–110) — `compare`, `evaluate_check`, the class override.
- `crates/conductor-verify/src/preflight.rs` (1–12, 40–50) — the freshness assertion; `fingerprint_refs` recorded as "the former carrier".
- `crates/conductor-run/src/canary.rs` (30–45) — the per-run preflight canary.
- `scripts/agent-run.sh` (78–86 `live_leg`, 88–100, 107–145 `live_suite`, 165–232, 255–280) — stage dispatch, the `SCENARIO=` leg, the `--live` composition.
- `scenarios/{live-only-service-truth,investigate-actions-functional,constellation-severity-live-wiring}.toml` (full).
- **Pulse** `crates/mcp-server/src/tools.rs` (405–445) — `dispatch_retrieve_telemetry_slice`.
- **Pulse** `pulse-app/src/inference_runtime.rs` (838–885) — `create_incident_from_l4_output`.
- **Pulse** `pulse-app/src/deterministic_inference.rs` (20–78) — the canned L4 fixture.
- **Pulse** `crates/triage/src/incident/persistence.rs` (35–50) — the 120 s auto-resolve window.

## The load-bearing equality — MEASURED, and it decides P-079

The design needs: *does P-079's `CountAtLeast ≥ 1` yield a met floor for a live Pulse read-back?*
**No — it is structurally unsatisfiable, and every link is measured:**

| # | Site | Fact |
|---|---|---|
| 1 | Pulse `inference_runtime.rs:871` | `evidence_refs: EvidenceRefs { … span_ids: Vec::new() … }` — **hard-empty at the only production construction site** |
| 2 | Pulse, whole repo | the only other `EvidenceRefs.span_ids` writes are `markdown.rs:507` (`vec![]`) and `tools.rs:868` (`vec![[1u8;8]]`, a unit-test fixture). **Nothing mutates it after creation** |
| 3 | Pulse `tools.rs:430–439` | `span_refs` maps 1:1 from `incident.evidence_refs.span_ids` ⇒ always `[]` live |
| 4 | Conductor `extract.rs:116` | `observation.evidence_count += string_array(&slice, "span_refs").len()` ⇒ **0**, summed over every open incident |
| 5 | Conductor `extract.rs:55` | `CountAtLeast` observes `evidence_count.to_string()` ⇒ `"0"` |
| 6 | Conductor `slo.rs:53–62` | `compare` parses both sides: `0 >= 1` ⇒ **false** |
| 7 | Conductor `slo.rs:96–100` | unmet `CountAtLeast` ⇒ `ClaimClass::CalibrationRegion` **regardless of the declared class** — "sample floors never hard-fail" |

**Consequences.** P-079's declared `class = "Hard"` is overridden, so it **cannot `Fail`** on this floor;
it grades `CalibrationRegion` → `ManualCheck` — **non-blocked**, satisfying `v2-04`. But the check
**asserts nothing about P-079's capability**: it is met by no live corpus state whatsoever.

This is *stronger* than the false-green the scope anticipated. The scope feared the still-open canary
could satisfy the floor; in fact **nothing** can. It is the same shape as the five live-proof re-bases
the tests extract names (test-plan §6 Fingerprint-storm, where `Absent` passed **vacuously**), whose
prescribed remedy is: retire the ungradeable check to declare-only and move the live claim to the
**harvest tier**, applied at **both** its §6 and §1 sites.

**Correction to a harness-rule premise:** `verification-harness.md:47` states the deterministic-L4
fixture pins `evidence_refs` to `[]` (`deterministic_inference.rs:35`, measured 2026-08-16). At Pulse
HEAD `83d4060` that is **stale** — the fixture now POPULATES them (`det-span-…` / `det-template-0007` /
`det-fingerprint-…`) and its own doc comment says the population is load-bearing precisely to stop
vacuous passes. It changes nothing here: only `fingerprint_hashes` is joined from L4 output
(`grounded_fingerprint_hashes`, `inference_runtime.rs:845`), and `span_ids` is hard-empty independently
of L4 mode.

## Open questions from scope — all four closed

1. **Does an empty-`expected` scenario reach MCP read-back? → YES.** `execute.rs:98` calls
   `observe(client).await` **unconditionally**; `scenario.expected.is_empty()` is merely the second
   argument to `route_read_back`, which routes the *interpretation*, not whether read-back happens.
   `v2-04`'s "via MCP read-back" mechanism clause is satisfiable for all three scenarios.
2. **Is `span_refs` populated? → NO, never, live.** See the table above.
3. **Where do ManualCheck rows come from? → `execute.rs:120–147`.** The empty-`expected` branch builds a
   `HoldPoint { step: "operator-checklist", checklist: scenario.checklist.clone() }` — **empty for all
   three** — resolves it through `resolve_hold` (headless resolver, never blocking), logs the resolution
   at `info`, and returns `manual_record(...)`: `verdict: None`, `state: state_for(observation, ManualCheck)`.
   **`state_for` (`execute.rs`) returns `KnownResidual` when `observation.degraded`, else the measured
   state.** So P-067/P-072 land `ManualCheck` on a non-empty corpus and **`KnownResidual`** on an empty
   one (the `AutoResolved` arm sets `degraded: true`) or under a degraded read-back — **both non-blocked**.
4. **Wiring fork → three `leg = 'live'` gate entries driven directly** (a decisive material lean, see
   Conventions below). Not a P4 question.
5. **Resolver tick** — `TARGET_INCIDENT_AUTO_RESOLVE_TICK = "triage.incident.auto_resolve.tick"`
   (`pulse-app/src/incident_observer.rs:29`) exists, but the **period constant was not located in Pulse
   source**. `verification-harness.md:50` states it fires every 30 s. Recorded as
   **corroborated-in-repo, not source-derived** — the directive's `hypothesis:` marker stands.

## The sequencing constraint — the real threat to `v2-04`

`verification-harness.md:50` (measured, `fingerprint-storm-live-proof`): **Pulse dedupes a new incident
against any OPEN incident, and NOT by fingerprint** — the canary's `exception_type` is unique per run and
its incident still returned `created:false, deduped:true`, producing
`Blocked: no incident opened after the canary storm was emitted`. **Every `conductor run` fires its own
preflight canary** (`canary.rs`, `preflight.rs:6`), so this blocks **any back-to-back leg**.

Three legs run back-to-back therefore yield **two `Blocked` rows** — exactly what `v2-04` forbids. The
cure on one shared data dir (the only option here: `ANDROMEDA_PULSE_DATA_DIR` must EQUAL the live
`pulse-app`'s dir, so a per-leg dir of Conductor's own empties every read-back) is the **quiet window:
≥150 s = Pulse's 120 s idle + up to a full 30 s resolver tick** after the preceding canary. The shipped
`live_suite()` does exactly this (`agent-run.sh:127`).

Scenario emission windows, summed from each TOML's `gap_ms` — all far inside the 120 s window, so each
leg's own canary is **still open at its read-back** (the directive's arithmetic, confirmed):
**P-067 18 s · P-072 35 s · P-079 30 s.**

Counter-fact that bounds it (`verification-harness.md:50`, extended 2026-08-18): a scenario emitting only
benign traffic for **≥~2–3 min** outlives its canary and read-back finds an EMPTY corpus. None of these
three comes close, so that arm does not fire.

## Patterns detected
- **`live_leg` is scenario-agnostic** (`agent-run.sh:78`): `live_leg <label> <scenario> <budget>` → `timeout … conductor run "$scenario" --agent-mode`, then freezes `logs/agent-latest.jsonl` to `runs/live-suite/<label>.jsonl`. Either wiring fork reuses this shape; option B reuses the invocation without the shell.
- **Per-run artifacts survive truncation** (`verification-harness.md:60(a)`): `logs/agent-latest.jsonl` is opened `.truncate(true)` so every `--agent-mode` run wipes the prior leg's self-obs — but `runs/<run_id>.jsonl` and the `runs.db` row are **per-run and do survive**. `v2-04`'s required evidence is exactly those two, so it is intact under any wiring.
- **Zero `CheckRecord`s for declare-only** (`execute.rs:208–213`, `ScenarioOutcome::without_checks` → `checks: Vec::new()`): P-067/P-072 write no `run_check` rows and get **no SLO grading** — `deadline_ms` lives only on `CheckRecord`. Their `latency_ms`/`slo_tier` are recorded on the envelope but never asserted.
- **`[BLOCKED]` in ~0 s = sidecar resolution, not the SUT** (`verification-harness.md:54`): elapsed time is the discriminator; verify `which andromeda-pulse-mcp` before the leg.

## Conventions to follow
- **Firing form** (`verification-harness.md:48/49/53/54`, re-confirmed): `PATH` in **POSIX form**
  (`/d/dev/projects/andromeda-pulse/target/release:$PATH` — bash splits a Windows-form path on the
  drive-letter colon) · `ANDROMEDA_PULSE_DATA_DIR` = **the live `pulse-app`'s dir** · `ANDROMEDA_PULSE_MCP_ENABLED=true`
  in **Conductor's own** environment · `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`. The sidecar must be
  **built** in the Pulse repo (`cargo build -p mcp-server --bin andromeda-pulse-mcp --features mcp-server`);
  unbuilt, the canary never emits and a missing witness is indistinguishable from a leg that never ran.
- **NEVER `boot` before `conductor run` on the same data dir** (`verification-harness.md:53(a)`) — `run`
  fires its own preflight canary and boot's canary is exactly what it would dedupe against. The leg
  invocation is `conductor run <scenario> --agent-mode` **alone**.
- **`L4_DETERMINISTIC` is load-bearing for ALL THREE legs**, and not for the reason the directive gave.
  The directive ties it to P-072's `<90s` tier — but a declare-only scenario is **never SLO-graded**
  (finding above), so that mechanism does not bind. The real one: without deterministic L4 an incident
  forms only if the 3B model surfaces it (`verification-harness.md:47`), so the **preflight canary** may
  form no incident and every leg blocks. Conclusion unchanged, mechanism corrected and widened.
- **Read the SUT's posture from Pulse's own log**, never the launch string: grep the TARGET
  `triage.baseline.bootstrap_window.override`, never the emitting fn name (`verification-harness.md:60(e)`).
- **Gate-entry format** — the current `## Test Commands` shape is a ```toml fence of `[[gate]]` tables
  (`run`/`role`/`new`/`baseline`/`expect`/`note`), per `2026-09-09-port-occupier-test-hygiene/plan.md`.
  No chunk has yet shipped a `leg = 'live'` entry (both current mentions are prose declining one), so
  this plan authors the first.
- **Wiring fork — decisive material lean: option B**, three `leg = 'live'` entries driven as
  `conductor run <scenario> --agent-mode`. Basis: it adds **no harness source delta** (so no `.sh`/`.ps1`
  parity obligation and no change to a shipped suite's leg order), the wrap light gate re-runs the
  entries **literally**, and `v2-04`'s required evidence is per-run and survives regardless. Option A
  would append ~3 legs × (scenario + 150 s quiet) to an operator session already carrying `live_suite()`'s
  H/B1/B2/quiet/A plus the a11y arm under a 15-minute mocha ceiling (a11y-plan §10). Rejected:
  `SCENARIO=<name> bash scripts/agent-run.sh run` — it is the documented artifact-producing firing form
  but also runs nextest + doctest + clippy on **every** leg (3× the full workspace suite).

## New files to create
- (none) — no source delta is required by the round itself. The plan's deliverable is driven evidence
  plus the recorded disposition of the P-079 finding.

## Files to modify
- (none required for the round.) The P-079 disposition may touch `scenarios/constellation-severity-live-wiring.toml`
  **only if** the operator rules for the test-plan re-base remedy at P5 — scope currently bans scenario
  re-authoring, and the tests extract's remedy would additionally require its §6 **and** §1 sites, which
  are spec masters (an Expected amendment at wrap, never a phase touchpoint). **Left to the P5 review.**

## Open questions
- **P-079's disposition** → blocks: **plan-decision**. The check is measurably ungradeable. Three honest
  options: (i) drive the leg, record the `CalibrationRegion`/`ManualCheck` verdict, claim `v2-04` on the
  non-blocked outcome, and route the re-base as a finding with a named owner; (ii) additionally re-base the
  check to declare-only in this chunk (scenario edit + two spec-master amendments); (iii) hold `v2-04`.
  P4 resolves it; the operator sees it at the P5 review. **Recommendation: (i)** — `v2-04`'s acceptance is
  met as written and the scenario ban stays intact.
