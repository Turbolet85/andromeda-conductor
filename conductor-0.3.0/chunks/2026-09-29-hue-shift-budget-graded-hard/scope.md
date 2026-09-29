# Scope — 2026-09-29-hue-shift-budget-graded-hard

**Working entry (`working-route.md:54`):** Hue-shift budget graded hard — `halo-hue-encoding` re-driven once Pulse
emits the contracted observable, restoring the fourth delegated budget.

**Matrix target:** `v3-08` (*P-025 graded hard at its real value*, `dynamic-external`, unclaimed at take-up). Its
acceptance: "The hue-shift scenario is driven live against a Pulse emitting the contracted observable, and its budget
is graded hard at a real measured value rather than routed to a calibration region, restoring the fourth delegated
budget to the set that grades hard."

_P3 premise closure (2026-09-29): every `[inferred]` bullet below was re-read against research.md and closed —
VERIFIED bullets lost the tag, falsified ones carry `[premise-corrected: …]`. Pulse coordinates are read at Pulse's
committed HEAD `226554a` (research.md §Pulse coordinate re-verification)._

## What this chunk builds
- **A live re-drive of `halo-hue-encoding`** against a Pulse build carrying `e98d838` (P-025 shipped). Pulse's local
  HEAD at take-up is `226554a` on `chore/migrate-pulse-to-v3`, equal to `origin/chore/migrate-pulse-to-v3`, and
  `e98d838` is its ancestor (`git merge-base --is-ancestor`).
  - The leg needs the operator's slot at implement (overseer directive, below). It is an operator-gated live leg and
    never a CI gate (contract §The hard grade; `rules/testing.md` §Quality gates).
  - The leg needs the COMPACT-WIDGET window open and mounted BEFORE the tier flips. Verified at `226554a`:
    `CompactWidget.tsx:90` is the canvas's only mount, and `hueShiftSamples` skips any change whose effective
    instant precedes the canvas mount (`constellation-types.ts:267`). The window must also stay VISIBLE, not
    minimized: WebView2 background-timer throttling on the 1 s poll is not determinable from source, and a throttled
    poll inflates `duration_ms`.
  - The `pulse-app` binary the leg drives is PROVEN to carry the fix by content: the field name
    `tier_effective_at_unix_nano` appears in `pulse-app.exe` (Pulse's own leg's method, `evidence/green-leg.md`). The
    Pulse checkout is `226554a`, or any HEAD with `e98d838` as an ancestor. Pulse's worktree is dirty with another
    session's uncommitted `ci.yml` / `xtask/src/*` / `quality_gate_workflow.rs` edits; none of them is in the
    `pulse-app` or `mcp-server` binaries' source.
- **A hard grade at the harvest tier**: `duration_ms ≤ 2000` over Pulse's own emitted `metric.constellation.hue_update_ms`
  lines from the leg's capture. A breach is a hard `Fail`. It is never an `[[expected]]` check and never `budget_ms`
  (entry freight; overseer directive; contract §The window's excluded-subject clause).
  - The grade lives in `crates/conductor-run/tests/delegated_timing_harvest.rs` over lines pinned verbatim in test
    source, the file's own convention (`:415-699`; `live_suite_harvest.rs:5-7`). The file's P-025 tests today pin
    the RETIRED instrument: the 2026-08-21 disproof `:473-521` and the 2026-09-07 tick-quantization mechanism pin
    `:632-699`. Those readings are TRUE of Pulse at `83d4060` and are cited by the contract (`:135-137`) and by
    obs-plan §4. So they stay, re-documented as the retired instrument's record, and the new grade is added beside
    them.
  - Attribution stays TEMPORAL and still holds against the new per-service emission. Verified: the sample carries no
    service id (`HueShiftSample` = `{duration_ms, severity_tier}`; the line's `service.name` is Pulse's own). The
    canary's rise precedes `scenario.run`. A hidden dot (quiet over 60 s) is never sampled, so the canary's fall
    emits nothing in-window. A stale remembered-tier sample (clamped to 60 000) can only fire when a service
    reappears (the canary at warm-up, `conductor` at phase-1 start), which is outside a window opening at phase-2
    start. The window stays phase-2 start → the `scenario.run` close.
    - [premise-corrected: "whether the leg captures a RISE only or also a FALL" — the scenario has NO healthy tail,
      and its incident auto-resolves ~120-150 s after its last deduped re-emission (`persistence.rs:42`,
      `incident_observer.rs:24`), by which time its dot is hidden. So the leg as shaped witnesses the RISE, plus any
      mid-storm resolve / re-open pair, and no end-of-storm fall. Whether to add a tail is a P4 fork (research.md
      §Open questions).]
- **The contract document brought to the measured state.** `contracts/pulse-p025-measurement-contract.md` is pinned
  at `83d4060`. At `226554a` six of its claims no longer hold as written:
  1. the fall source (the broadcast's `transitioned_at` → `resolved_at_unix_nano`);
  2. acknowledgement lowering the tier (it does not);
  3. `ServiceListItem` carrying no tier timestamp (it now carries `tier_effective_at_unix_nano`);
  4. "that site holds both timestamps" (true only since `list_for_workspace`);
  5. the "eight writes" count (11 non-test writes of the field, 5 on the incident chain; the immutability substance
     holds);
  6. the redaction-precedent sentence (both leaves are full at `226554a` and were already at `83d4060`).

  The resolution and fire-site sections describe the RETIRED instrument. The document is corrected in place, with
  `pinned_at` / `provenance` moved to `226554a`, and it states the grading rule this chunk applies BEFORE the leg
  fires, so the grade cannot be fitted to the reading (the posture-contract precedent, arch §Occupied Resources).
- **The scenario's header comment corrected.** Verified stale: header terms (1) SLOWEST-WINS and (3) QUANTIZATION are
  GONE from the hue path at `226554a`; term (2), the 60 s recency window, HOLDS. The `:80-86` checklist comment
  ("unmeasurable through this leaf") is stale the same way. Phase data and the `[[checklist]]` item are unchanged
  unless P4 takes the tail fork. The companion sweep (`grep -rln "halo-hue-encoding" crates/ scripts/ contracts/
  .github/`, 11 files, every hit read) is in research.md.
- **The two harness comments corrected** — `scripts/agent-run.sh:109-114` and `scripts/agent-run.ps1:175-179` say the
  hue observable "fires against a tick-refreshed last_seen", which is stale at `226554a`. (Added at P3: the same claim,
  found by the sweep.)
- **`v3-08` claimed**, if P5 concretizes an acceptance this chunk's own lifecycle can reach.

## Boundaries (out of scope)
- No edit to Pulse's repository. Pulse is read at its committed HEAD. A Pulse-side defect found here is relayed to
  Pulse's route, never patched.
- No `[[expected]]` check on `halo-hue-encoding`. `Scenario::check_checklist` rejects a checklist beside expected
  checks at load, and the scenario declares one `[[checklist]]` item (entry freight).
- The Conductor-side `latency_ms` / `budget_ms` / `effective_deadline_ms` are never bound to the hue duration (contract
  §The window).
- Neither previously-named SUT fix counts as satisfying the bound (entry freight; contract §Why the two recorded fix
  candidates are insufficient; obs-plan §4).
- No `v3-09` work. Its series stays BLOCKED-ON Pulse's scrubber fix (`working-route.md:56`).
- No P-027 `discovery_ms` grading. Pulse routed the 9 986 ms first-sighting rise to its own P-027 entry (Pulse
  report §Decisions & corrections).
- No new env-reading test and no new `CONDUCTOR_*` / Pulse handle. No P-025 coverage-classification change (it stays
  `DriveObserve`, `coverage.rs:238-243`): the hue RENDER stays the operator checklist, and only the timing budget
  grades hard.
- The seven spec masters stay read-only here. obs-plan §4's delegated-timing passage and architecture §Occupied
  Resources' P-025 contract row are wrap amendments.

## Folded freight (`working-route.md:54`, three blocks, all per `route.py pins`)
- **CONTEXT (from 2026-09-13-p-025-measurement-contract-for-pulse)**, 648 chars, folded whole:
  - "The contracted observable" is a committed artifact, `contracts/pulse-p025-measurement-contract.md`. It states the
    leaf and its literal field names, the required millisecond resolution, the window, and `duration_ms <= 2000`
    graded hard as a `Fail`.
  - The ask is to EXPOSE a start instant Pulse already holds where it computes the tier, not to mint one:
    `opened_at_unix_nano` when the service's max tier RISES, `transitioned_at_unix_nano` when it FALLS.
  - Causal claim, marker kept verbatim: "the two cases exhaustive because an incident's `priority_tier` is immutable
    after opening (measured at Pulse HEAD `83d4060`)". VERIFIED at `226554a`: the only field assignment is the
    DERIVED item at `services_router.rs:101`, the dedupe path only calls `observe_reemission`, and no corpus UPDATE
    touches the tier.
    - [premise-corrected: its fall case names `transitioned_at_unix_nano` — the shipped fall is
      `resolved_at_unix_nano` (`tier_effective.rs:30-35`), per the CARRY below.]
- **CONTEXT (2026-09-29 wrap)**, 699 chars, folded whole:
  - Its BLOCKED-ON (one Pulse release emitting that observable) has cleared: Pulse's P-025 shipped at `e98d838`,
    measured on `origin/chore/migrate-pulse-to-v3` (relay `conductor-wrap-50-2026-09-29` §2.2).
  - Re-verified at take-up: `e98d838` is an ancestor of Pulse's local HEAD `226554a`, which equals the local
    `origin/chore/migrate-pulse-to-v3` ref. No fetch was run, so that ref is as of Pulse's last fetch.
  - The re-drive must NOT grade through an `[[expected]]` check (the reason is in Boundaries above).
  - The re-drive must NOT treat either previously-named SUT fix as sufficient: both address the staleness term alone
    and would make the budget pass over the wrong quantity (obs-plan §4 records this).
- **CARRY**, 412 chars, folded whole. Pulse's P-025 phase measured two premises of the contract wrong at Pulse HEAD.
  Both are re-verified at `226554a` (marker at fold: "measured by Pulse's P-025 phase and relayed 2026-09-29; not
  re-measured here"):
  - "the reconciler resolves without the broadcast event the contract names (the fall is read from `resolved_at`)" —
    VERIFIED. `persistence.rs:250-283` calls `mark_resolved` with no broadcast. The broadcast has no production
    subscriber. The fall is `resolved_at_unix_nano` (`tier_effective.rs:30-35`).
  - "an acknowledgement does not lower the tier" — VERIFIED. Acknowledged stays in the active set
    (`registry.rs:219-227`, `services_router.rs:104`); test `tier_effective.rs:167-171`
    `acknowledgement_keeps_the_tier`.

## Operator / overseer directives (take-up, 2026-09-29)
- Pulse P-025 shipped at `e98d838`. **Re-verify EVERY coordinate** of `contracts/pulse-p025-measurement-contract.md`
  at Pulse's current HEAD, not only the CARRY's two premises. Done at P3 for the contract and for the scenario header
  (research.md, 28 rows).
- Grade through the harvest tier, never an `[[expected]]` check.
- **Any diff-shaped gate probe names the chunk base explicitly** (`git diff --numstat <base> -- f`), because the
  operator pre-CI commit moves HEAD before the wrap (W182). The chunk base is `cdb7082`
  (`cdb708266d59c3600876363b4720e85f33332feb`), HEAD at take-up.
- **The live re-drive needs the operator's slot at implement.** Implement pauses for it rather than launching the leg
  unannounced.
- **P5 approval (overseer, founder-delegated, 2026-09-29), verbatim:** "At implement step 5, stop and ask me for the live
  slot: the Pulse builder is chasing a Linux-only app death and may start pulse-app itself, so I will free 4317 for you
  first." ("Step 5" is the review card's numbering of the live leg, which is plan.md step 6.) Implement asks for the
  slot and waits for the overseer to free `:4317` before launching or probing anything.

## Take-up readings (closed at P3 against Pulse source at `226554a`)
- The start instant ships as `ServiceListItem.tier_effective_at_unix_nano: Option<i64>`
  (`lifecycle/registry.rs:54-69`), derived by the pure replay `triage::contract::tier_effective_at(&[Incident],
  service)` (`tier_effective.rs:23-63`):
  - rise: the max-raising incident's `opened_at_unix_nano`;
  - fall: the last max-holder's `resolved_at_unix_nano`;
  - acknowledgement: inert.

  VERIFIED.
- The leaf keeps its NAME and FIELDS, `metric.constellation.hue_update_ms {duration_ms, severity_tier}`
  (`telemetry.rs:278-282`; allowlist `observability.rs:989-992`). Only its value's meaning changed: paint instant −
  `tier_effective_at`, one sample per changed service, witnessed only, the NEW tier as `severity_tier` (`none` on a
  fall to no incident). No field was added, so the contract's allowlist-admission requirement is satisfied without
  one. VERIFIED.
- Pulse's own leg (`smoke:hue-shift`) read rise `duration_ms=9986` and fall `duration_ms=510` on a fresh boot, and
  later 9841 / 578, with anchor errors 44 / 25 ms and 36 / 28 ms.
  - VERIFIED: the rise was a FIRST SIGHTING. A service is listed only after its first 15 s lifecycle tick
    (`lifecycle/registry.rs:320-337`, first tick skipped `lifecycle/mod.rs:95-96`).
  - `halo-hue-encoding`'s 30 s `healthy-baseline` at 2/s precedes its error phase, so its dot is listed before the
    rise, putting the rise in the poll-bounded case (1 s `items` poll).
  - That is the premise the 2 000 ms grade rests on. Only the live leg measures it, and this argument does not settle
    it.
- Pulse's working tree is DIRTY at take-up. Every Pulse read in this chunk is `git show 226554a:<path>`, never the
  worktree. VERIFIED (research.md).

## CI (Setup 5a)
- `cdb7082` (the 2026-09-29 dual-license wrap commit): **`verdict: green` · checks 3/3 · wall 581 s**,
  CI#36621215714 push completed/success. Nothing to fold.
