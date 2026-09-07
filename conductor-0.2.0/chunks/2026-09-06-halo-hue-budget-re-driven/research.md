# Codebase Research — 2026-09-06-halo-hue-budget-re-driven

## Scope
- **Depth:** deep (mature codebase; the chunk turns on an external fire site's arithmetic) · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read as a STRUCTURAL EXTRACTION (a 60-line file that renders 56.2 KB; `## Session Additions` indexed by `grep -n` on the `- 2026-` introducers, then offset-bounded reads of entries `:50` `:52` `:55` `:56` `:60`, the live-leg-relevant span). `.claude/rules/testing.md` frontmatter globs confirmed to cover `scenarios/**` and `crates/**/tests/**`; `security.md` + `host-win32.md` load unconditionally and were already in context.

## THE HEADLINE FINDING — the fire site's arithmetic decides the design, and two of its three terms were unrecorded

The CONTEXT's mechanism is **verified at HEAD** and is **sharper than any Conductor artifact states**. Fire site:
`andromeda-pulse pulse-app/ui/src/widget/ConstellationCanvas.tsx:117-155` (HEAD `83d4060`), a `useEffect` keyed `[dots, items]`:

```
const elapsedMs = nowMs - item.last_seen_unix_nano / 1_000_000;   // :141  — STALENESS, as recorded
if (elapsedMs > slowestMs) { slowestMs = elapsedMs; slowestTier = tier ?? "none"; }   // :142-145
...
void recordConstellationHueLatency({ duration_ms: slowestMs, severity_tier: slowestTier });   // :152
```

Three terms, only the first of which was known:

1. **Staleness, confirmed** — `now − last_seen`, exactly as the CONTEXT and the scenario header state. ✓
2. **SLOWEST-WINS, previously unrecorded.** The effect loops EVERY dot whose tier changed in that render pass and emits the **maximum** staleness, with `severity_tier` naming *that* dot's tier — not the scenario's. So if the scenario's dot and the canary's dot change tier in the SAME pass, the emitted sample is the canary's, and its `severity_tier` is reported. A `changed === 0` pass emits nothing.
3. **A 60-SECOND CEILING, previously unrecorded.** `visibleDots` filters on `isServiceLive` — `now − last_seen <= LIVE_RECENCY_WINDOW_NANOS`, and `LIVE_RECENCY_WINDOW_NANOS = 60 * 1_000_000_000` (`constellation-types.ts:27`, mirroring the Rust FSM's `ACTIVE_TO_QUIET_THRESHOLD_SECONDS`). A service quiet longer than 60 s is not a dot at all, so **no `hue_update_ms` sample can exceed ~60 000 ms.** The measured 35 581 / 36 705 ms sit inside that ceiling, which corroborates the staleness reading rather than a slow render.

**What this makes the design target, quantified:** the budget is met iff, at the instant the scenario's dot changes tier, that service's `last_seen` is under 2 000 ms old — i.e. **the scenario's dispatch interval must be well under 2 s and emission must still be running when the tier flips.**

## Graph impact (code-graph, rust plane — `rows: 72`, `db_state: fresh`, no probe warnings)

Query: `SELECT callee_name, caller_name, file, line FROM calls WHERE callee_name IN (...) ORDER BY ...`
(trace: `.andromeda/runs/2026-09-06T19-16-25-phase/tree-query-2026-09-06-halo-hue-budget-re-driven.json`)

- **`phase_rate_exceeds`** — 1 caller, `phase_breach` (`load_envelope.rs:209`). **`max_spans_per_dispatch`** — 1 production caller, the same `phase_breach` (`:211`); the other 9 rows are `phase_spec.rs`'s own arm-by-arm test. **`phase_breach`** — exactly 2 callers, `classify` (`:267`) and `check_load_envelope` (`:370`). The shared-basis property arch asserts holds **by construction at HEAD**, measured, not inherited.
- **`check_checklist`** — 1 caller, `from_toml_str` (`scenario.rs:160`); **`check_budgets`** — `from_toml_str` (`:159`). Both load-path rules are genuinely invoked; the checklist/`expected` conflict is a real `CoreError::Config` at load, not a documented intention.
- **`route_read_back`** — 1 production caller, `execute` (`execute.rs:88`); 6 test rows. **`state_for`** — `execute` (`:177`) + `manual_record` (`:260`).
- `classify` collides across crates (31 rows: `conductor-core::load_envelope`, `conductor-verify::slo`/`verdict`). Disambiguated by `callee_file`; the load-envelope one is the relevant symbol.

## Files inspected
- `crates/conductor-core/src/phase_spec.rs` (`:100-200`, `:700-770`) — `EmissionSpec { signal, occurrences, shape }`; **`occurrences` are "paced evenly across its gap"** (doc `:100-102`), which is the sustained-window mechanism *already in the model*. `max_spans_per_dispatch` (`:156-190`) maps all 9 `EmissionShape` arms; `Plain`/`Exception`/`Severity` → 1, `Error{depth}` → `depth+1`, `Latency` → `samples`, `Pii` → per-signal, `Ramp`/`Breathing` → `curve_bound` at a transcribed 115 % jitter ceiling, `Topology` → per service.
- `crates/conductor-core/src/load_envelope.rs` (`:205-215`, `:267`, `:370`) — the single `phase_rate_exceeds` → `phase_breach` → {`classify`, `check_load_envelope`} chain.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (`:1-45`, `:314-347`) — module doc states the capture discipline (grade over a live-leg capture of `{data_dir}/logs/agent-latest.jsonl.<date>`; per-target field pinned in `bounds()` because the field name is NOT uniform; `budget_ms` deliberately unused; **absence is never a pass**). `grade()` folds observations with `f64::max` and fails if the **worst** exceeds the budget.
- `crates/conductor-run/src/execute.rs` — `route_read_back` / `state_for` call sites (graph-confirmed above).
- `scenarios/halo-hue-encoding.toml` (full) · `scenarios/auto-resolve-idle-window.toml` (`:25-40`) · `contracts/pulse-load-envelope.toml` (`:21-47`).
- `scripts/agent-run.sh` (`:62-136`, `:200-210`) — the `--live` suite: `live_suite()` refuses at exit 1 on an unmet precondition with no leg fired; legs are **b1 + b2 `degraded-mode-report` → 150 s quiet window → a `auto-resolve-idle-window` → the driven a11y arm**. Each leg freezes `logs/agent-latest.jsonl` into `$RUNS_DIR/live-suite/{label}.jsonl`. **`halo-hue-encoding` is NOT a leg today.**
- SUT (`andromeda-pulse`, HEAD `83d4060`, read-only): `pulse-app/ui/src/widget/ConstellationCanvas.tsx:117-155` · `widget/constellation-types.ts:23-31,160-188` · `crates/ui-bridge/src/telemetry.rs:272-285` · `pulse-app/src/observability.rs:980-995` · `crates/triage/src/cue/thresholds.rs:8-52,228-238` · `crates/triage/src/baseline/activity_floor.rs:26-40` · `crates/triage/src/cue/emitter.rs:180-192` · `pulse-app/src/main.rs:472`.

## Patterns detected
- **Attribution rides a field only the scenario could produce** (`delegated_timing_harvest.rs:1-38`): P-027 was attributed by `discovered_count: 3` — the scenario's own topology — not by service name. That is the shape available here, and the field set is the constraint (see Open question 1).
- **Harvest grades the WORST observation** (`delegated_timing_harvest.rs:114-130`): correct for pinning a disproof, and exactly wrong for a re-drive on a leg that also contains the canary's sample.
- **Load-path `check_*()` for sibling-spanning rules** (`scenario.rs:159-160`) — the shipped precedent; the checklist/`expected` conflict is enforced there.
- **Per-leg journal freeze** (`agent-run.sh:78-88`) — the durable read for a multi-leg proof, since the sink truncates per invocation.

## Conventions to follow
- **Occurrences pace across the gap** (`phase_spec.rs:100-102`) — a sustained window is `gap_ms` + a matching `occurrences`, needing **no scenario-model change**.
- **Grade the delegated bound at the harvest tier over Pulse's own leaf**, never through `budget_ms` (`delegated_timing_harvest.rs:16-20`).
- **Absence is never a pass** (`:22`) — a re-drive that produces no scenario-attributable sample must grade `Err`, not a satisfied budget.

## New files to create
- (none anticipated) — `conductor-0.2.0/chunks/2026-09-06-halo-hue-budget-re-driven/evidence/` for the leg captures.

## Files to modify
- `scenarios/halo-hue-encoding.toml` — add `[phases.emission]` to the error-pressure phase (sustained, sub-2s dispatch interval), re-tier `slo_tier`, resolve `[[checklist]]` vs any `[[expected]]`.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — re-aim the P-025 assertion; its `hue_lines()` fixture, `bounds()[0]` and the worst-wins `grade()` selection are all in the boundary (a scenario-attributable selection cannot use `grade()` as written).
- `scenarios/auto-resolve-idle-window.toml` — CARRY 3.3: the `:28-32` header margin model.
- `scripts/agent-run.{sh,ps1}` — only if the re-driven scenario becomes a `--live` leg (it is not one today); both shells move together.
- **Crate-local companions** (not caller-threading — no signature changes): `crates/conductor-core/src/load_envelope.rs`'s catalog gate runs over the committed scenario set, so a re-shaped phase is already gated by `check_load_envelope`'s existing test surface; `crates/conductor-run/tests/dispatch_wire.rs` + `crates/conductor-timeline` stream goldens **pin dispatcher output per scenario** — whether `halo-hue-encoding` appears in a committed golden is Open question 3.

## Open questions
1. **Which on-line field attributes a hue sample to the scenario rather than the canary?** The leaf carries ONLY `duration_ms` + `severity_tier` — the SUT's allowlist comment reads "Aggregate-only: no service identifier" (`observability.rs:986-990`) — so the `conductor` / `conductor-canary` split cannot discriminate. Candidates: the `severity_tier` VALUE (the canary storms 12 ≥ the Autonomous threshold 10 and reports `autonomous`; a scenario tuned to the Suggested band would report `suggested`), and sample ORDER/timing within the frozen leg capture. → blocks: **plan-decision** (it decides both the scenario's target band and the harvest's selection rule).
2. **Does the plan lean on `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`?** Measured: the knob is live (`main.rs:472`) and reaches the silence gate. It is **irrelevant to the hue leg** (the storm/error-rate path consults no baseline) and **decisive for the CARRY 3.3 AutoResolved leg** (a stretched window keeps `service_went_silent` from firing during the silent phase). → blocks: **plan-decision** — the directive requires the plan to state which posture, and any value rides the firing form.
3. **Does adding `[phases.emission]` to this scenario disturb a committed stream golden?** `dispatch_wire__*` / `pacing__*` / `replay__*` goldens are per test-file family × seed. → blocks: **implementation-scope**.
