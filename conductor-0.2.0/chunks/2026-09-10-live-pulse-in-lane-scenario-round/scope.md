# Scope — Live-Pulse in-lane scenario round

**Marker:** `2026-09-10-live-pulse-in-lane-scenario-round`
**Working entry:** `conductor-0.2.0/working-route.md:133`
**Claims:** `verification-matrix.json#v2-04` — *In-lane SUT-capability scenarios*

## Surface statement (from the working entry, verbatim)

> Live-Pulse in-lane scenario round — P-067, P-072 and P-079 each yielding a non-blocked live
> verdict, with journal and runs.db evidence per scenario

## What this chunk does

Drive the three committed in-lane scenarios against a live, operator-launched Pulse and produce, for
each, a **non-blocked verdict** with the journal + `runs.db` evidence `v2-04`'s acceptance names. The
capability under test is Pulse's (P-067 live service truth · P-072 Investigate result · P-079
single-sourced workspace key); Conductor's deliverable is the driven proof, its captured evidence,
and whatever minimal wiring the legs need to be re-runnable by the wrap's light gate.

`v2-04` acceptance as it stands today (read from the ledger this pass, unmodified):

> Scenarios keyed to P-067, P-072 and P-079 each drive a live Pulse and yield a non-blocked verdict
> via MCP read-back, with journal and runs.db evidence per scenario.

## Boundaries

- **No Pulse UI automation** — the standing non-goal. P-067 and P-072 are `drive+observe`: Conductor
  drives the stimulus, the operator confirms Pulse's rendered surface.
- **Conductor never starts or stops `pulse-app`.** The SUT is the operator's to launch; its env
  posture is handed per command and read back from Pulse's own witness line, never inferred from the
  launch string.
- **No new verification capability.** This round asserts through the read-back path that already
  ships; a gap that would need new extraction machinery is a finding to record, not to build here.
- **No scenario re-authoring** to make a verdict greener. The three `.toml` files are committed and
  verified; a scenario that cannot yield its claimed verdict as written is a recorded premise
  correction, not an edit.
- `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is **out of scope** — boot-wide, and it matters only
  to an empty-active-set arm, which none of these three scenarios has (confirmed below).

## Folded freight

The working entry carries **one `CONTEXT` annotation** and no `PREREQ` / `CARRY` / `BLOCKED-ON`
(measured: `grep -oE` over line 133 returns `CONTEXT` ×1 and nothing else). No annotation migration
is owed, and no external-decay pin is standing — the advisory deferral was CLOSED at
`2026-09-05-audit-corrective`.

### Coordinates re-verified this pass (the fold's own re-verify — named artifacts only)

Every coordinate the `CONTEXT` and the phase directive name was checked against the artifact before
it shaped this scope. **All exact; nothing to correct:**

| Claim | Artifact | Verdict |
|---|---|---|
| Verb + non-priming `preconditions` short-circuit | `scripts/agent-run.sh:91-97` (the `if !` at `:95`, refusal at `:96`) | exact |
| `live_suite()` legs are H · B1/B2 · A + driven a11y arm | `scripts/agent-run.sh:120-141` | exact |
| `observe()` calls `query_incident_list(None)` — unfiltered — and grades every id returned | `crates/conductor-verify/src/extract.rs:76-130` | exact |
| Preflight canary is a unique fingerprint-storm, emitted by the composition root | `crates/conductor-run/src/canary.rs:37` · `conductor-verify/src/preflight.rs:6` | exact |
| `DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120` | **Pulse repo**, HEAD `83d4060`, `crates/triage/src/incident/persistence.rs:42` | exact |
| Boot triple (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED` + matching `ANDROMEDA_PULSE_DATA_DIR`) | `scripts/agent-run.sh:168-169` | exact |
| Canned-L4 formation ~2 s vs real-model ~110 s | `scripts/agent-run.sh:118-119` | exact |
| `p_ids` per scenario | `live-only-service-truth.toml:20` · `investigate-actions-functional.toml:21` · `constellation-severity-live-wiring.toml:22` | exact |

### Directive correction ACCEPTED, and re-measured

The entry's own `CONTEXT` says *"a live ROUND, not harness work — every piece is already committed"*.
The directive corrects this as **too strong**, and the correction holds: **none of the three in-lane
scenarios is wired into `live_suite()`** — its legs are `h halo-hue-encoding`, `b1`/`b2
degraded-mode-report`, `a auto-resolve-idle-window`, plus the driven a11y arm. What *is* committed
and verified is the verb, the per-leg capture/freeze machinery (`live_leg()` at `agent-run.sh:78` —
**scenario-agnostic**, taking `label · scenario · budget`), and the three scenarios themselves.

So this chunk carries a **small wiring decision**, deferred to P3/P4 research as the directive
directs — three new legs inside `live_suite()`, or three `leg = 'live'` gate entries driven
directly. Both honest; the second keeps the wrap's light gate able to re-run them literally.

### Directive mechanism claims — folded as `[inferred]`, marker text kept verbatim

Per `promotion.md`, a causal-mechanism claim in the folded freight enters scope `[inferred]`-tagged
with its marker text verbatim, measured-marked or not — statedness is not measurement at HEAD. P3's
scope premise closure re-verifies these.

- **VERIFIED (P3)** — *"READ-BACK MECHANIC, measured at HEAD"* — because `observe()` is unfiltered and
  every `conductor run` fires its own preflight canary, and Pulse auto-resolves only after a 120 s idle
  window plus a resolver tick the harness comment puts at 30 s (*"tick period not re-derived by me —
  treat as `hypothesis:`"*), **each read-back sees its own canary still open**. Confirmed by summing
  each TOML's `gap_ms`: **P-067 18 s · P-072 35 s · P-079 30 s**, all far inside the 120 s window. The
  resolver tick stays `hypothesis:` — `TARGET_INCIDENT_AUTO_RESOLVE_TICK` exists
  (`pulse-app/src/incident_observer.rs:29`) but the period constant is not in Pulse source; 30 s is
  corroborated in-repo by `verification-harness.md:50` only.
- `[premise-corrected: a declare-only scenario is NEVER SLO-graded — `ScenarioOutcome::without_checks`
  writes zero `CheckRecord`s and `deadline_ms` exists only on a `CheckRecord`, so P-072's `<90s` tier is
  recorded on the envelope and never asserted. The handle IS load-bearing, for ALL THREE legs and by a
  different mechanism: without deterministic L4 an incident forms only if the 3B model surfaces it
  (`verification-harness.md:47`), so the PREFLIGHT CANARY may form no incident and every leg blocks.]*
  The directive's conclusion — set `ANDROMEDA_PULSE_L4_DETERMINISTIC` — stands; its stated reason
  (P-072's tier) does not bind, and the true reason is broader.

### Directive mechanism claim PARTIALLY FALSIFIED at P1 — the correction that changes the work

The directive states the canary consequence as uniform across all three:

> With these scenarios' tiers … the canary CANNOT have resolved by read-back, so each read-back sees
> its own canary still open and the honest outcome is a graded MANUAL row. That is `ManualCheck`,
> which is NOT `Blocked` … Plan for it explicitly per scenario.

Reading the three `.toml` files in full falsifies the universal, and on a different mechanism than
the one claimed. **The scenarios do not share a grading shape:**

- **P-067 `live-only-service-truth`** — carries **no `[[expected]]` block**. Its own header states
  the path: *"empty `expected` → verdict None → Lamp::Manual, the ManualCheck / operator-checklist
  path"*. Coverage mode `drive+observe`. → **ManualCheck**, as the directive says.
- **P-072 `investigate-actions-functional`** — carries **no `[[expected]]` block**, same declared
  path, coverage mode `drive+observe`. → **ManualCheck**, as the directive says.
- **P-079 `constellation-severity-live-wiring`** — carries a **`[[expected]]` block**:
  `kind = "CountAtLeast"`, `class = "Hard"`, `expected = "1"`. Coverage mode **`auto`** — *"Conductor
  drives the storm AND asserts the reaction through MCP read-back."* → **NOT ManualCheck.** It grades
  programmatically and can Pass or Fail.

Two consequences, both scope-shaping:

1. **The manual-ness of P-067/P-072 is structural, not canary-caused.** It follows from an empty
   `expected` — a property of the committed scenarios, stated in their own headers — and would hold
   with no canary in the picture at all. The directive reaches the right verdict for these two
   through a mechanism that is not the operative one; the distinction matters because it means the
   canary's open state is *not* what makes them non-blocked.
2. **P-079 needs a genuinely different plan.**
   `[premise-corrected: the risk is not that the CANARY satisfies the floor — NOTHING can. Measured at
   Pulse HEAD `83d4060`: `Incident.evidence_refs.span_ids` is `Vec::new()` at the only production
   construction site (`pulse-app/src/inference_runtime.rs:871`) and nothing mutates it after, so
   `retrieve_telemetry_slice.span_refs` is `[]` for every live incident, `evidence_count` is always 0,
   and `0 >= 1` is false. P-079's `CountAtLeast >= 1` is STRUCTURALLY UNSATISFIABLE against a live
   Pulse — a vacuous check, not a contaminable one.]* The consequence for grading is benign and
   measured: an unmet `CountAtLeast` routes to `ClaimClass::CalibrationRegion` **regardless of the
   declared `Hard`** (`slo.rs:96-100` — "sample floors never hard-fail"), so P-079 cannot `Fail` on
   this floor and yields a non-blocked `CalibrationRegion`/`ManualCheck` row. It satisfies `v2-04`'s
   acceptance while asserting nothing about P-079's capability.

   **Disposition RESOLVED at P4 (val-1: intent-incomplete — planning found the finding was already
   ruled).** It is not a P-079 defect but an **unswept member of a ratified class**:
   `architecture-amendments.md:403` states the mechanism verbatim — "`CountAtLeast` grades `span_refs`
   the incident producer writes empty" (2026-08-21, the ground on which five severity-lifecycle
   scenarios were retired to declare-only) — and **`architecture.md:69`** (offset ~5712 in a ~10 KB line)
   records the grading outcome as measured and ACCEPTED for a still-shipping sibling
   (`findings-counter-refresh`, unmet floor → `CalibrationRegion`, "a sample floor never hard-fails"); its
   clause is about the degraded-read-back route, so it is acceptance-in-practice rather than an explicit
   ruling. (`architecture.md:63` is the freshness-carrier paragraph and carries neither claim — an earlier
   draft mis-cited it.) Exactly three committed scenarios carry a LIVE `kind = "CountAtLeast"` key —
   `constellation-severity-live-wiring.toml:53` · `findings-counter-refresh.toml:49` ·
   `pulse-run-contract.toml:58`; a bare token grep returns seven files and is the wrong basis. So
   retiring only P-079 would contradict an accepted sibling and retiring all three is beyond this chunk.
   The round therefore **drives, records and claims**, and routes the class as a finding with a named
   owner. No scenario edit, no master amendment.

This is the W73 shape the directive warns about, arriving from the opposite side: not a scenario
planned green that grades manual, but a scenario the directive planned manual that in fact grades
Hard — and whose Pass may not mean what it appears to.

## Open questions for P3 — CLOSED (see `research.md`)

1. **Does an empty-`expected` scenario reach MCP read-back? → YES.** `observe(client).await` is called
   **unconditionally** at `execute.rs:98`; `expected.is_empty()` is only the routing flag passed to
   `route_read_back`. `v2-04`'s "via MCP read-back" mechanism clause is satisfiable for all three.
2. **Is `span_refs` populated? → NO, never, live.** See the premise correction above.
3. **Where do ManualCheck rows come from? → `execute.rs:120-147`.** The empty-`expected` branch builds a
   `HoldPoint { step: "operator-checklist", checklist: … }` — empty for all three — resolves it headlessly
   and returns `manual_record`: `verdict: None`, `state: state_for(observation, ManualCheck)`. **`state_for`
   returns `KnownResidual` when `observation.degraded`**, so P-067/P-072 land `ManualCheck` on a non-empty
   corpus and `KnownResidual` on an empty one — **both non-blocked**.
4. **Wiring fork → three `leg = 'live'` gate entries** driven as `conductor run <scenario> --agent-mode`
   (decisive material lean; basis in `research.md` §Conventions). No harness source delta.
5. **Resolver tick** — see the VERIFIED bullet above; stays `hypothesis:`, and no leg depends on it.

## Constraint discovered at P3 that the entry, the directive and this scope all missed

**Three legs cannot run back-to-back.** `verification-harness.md:50` (measured): Pulse dedupes a new
incident against **any OPEN incident, and NOT by fingerprint** — the canary's unique `exception_type`
still returned `created:false, deduped:true`, producing `Blocked: no incident opened after the canary
storm was emitted`. Every `conductor run` fires its own preflight canary, so legs 2 and 3 would each
land the exact `Blocked` row `v2-04` forbids. The cure on one shared data dir — the only option, since
`ANDROMEDA_PULSE_DATA_DIR` must EQUAL the live `pulse-app`'s dir — is a **quiet window of ≥150 s
(120 s idle + a full 30 s resolver tick) between legs**, the same interval `live_suite()` already uses
at `agent-run.sh:127`. This is a sequencing requirement on the round, not harness work.

## Evidence this chunk must produce

Per scenario: the run journal (`runs/<run_id>.jsonl`), the `runs.db` row, the Markdown run report,
and the frozen self-obs capture — with the verdict, its class (`ManualCheck` vs graded), and the
SUT's boot posture read from **Pulse's own witness line** rather than the launch command.

## Ledger

- **Claim `v2-04` here** — this entry's whole reason.
- **`v2-21`** (`by-construction`) is claimable at either remaining chunk; leave it to *Release build
  and bundle*, where the matrix is final, unless P5 finds a reason to take it now.
