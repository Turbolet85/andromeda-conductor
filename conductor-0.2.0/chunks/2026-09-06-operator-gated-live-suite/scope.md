# Scope — 2026-09-06-operator-gated-live-suite

**Working entry (title, verbatim):** Operator-gated live suite — re-runnable live-Pulse proof invocation carrying its
evidence, never a CI gate
**Epoch:** 6b — Polish & ship (its second chunk; head was `2026-09-05-audit-corrective`).
**Kind:** harness + live-proof chunk. Needs a LIVE PULSE for its proof legs (measured absent at promotion time:
`:4317` had no listener and no `pulse-app` process at 2026-09-06T06:34Z). Source-touching, so the workspace gates run
in full.
**Basis artifacts:** the two CARRY annotations folded off the working entry (below), `.andromeda/test-plan.md` §6
(the three suite families + their firing forms) / §9 (the live-leg CI ban) / §12 (the accepted-deliberate roster),
and `conductor-0.2.0/chunks/2026-09-03-conductor-run-composition-root-survivors-dispositioned/evidence/disposition-ledger.md`
(classes B and C).
**Scope premise closure:** PERFORMED at P3 (2026-09-06) — every `[inferred]` bullet below is now VERIFIED (tag
dropped) or `[premise-corrected: …]`; two facts research ADDED are marked `[added at P3]`. `research.md` carries
the basis.

## Folded annotations (re-verified at promotion, per promotion.md)

The working entry carries TWO `CARRY` annotations. Both fold here. Their named coordinates were re-verified against
the artifacts themselves before shaping scope; **the second CARRY's three coordinates are all STALE** because
`conductor-run/src/lib.rs` was split 1944 → 36 lines across seven siblings on 2026-09-05 (`2026-09-05-audit-corrective`),
after the CARRY was written. Re-aimed by function + column, cross-confirmed against test-plan §12's own re-pointed roster:

| CARRY-2 coordinate as written | Re-verified site at HEAD `76decf7` | mutant coordinate (test-plan §12) |
|---|---|---|
| `conductor-run/src/lib.rs:518` — `Observation.degraded` on the `ReadBack::AutoResolved` arm | `crates/conductor-run/src/execute.rs:91-96` (arm opens `:91`; `Observation { degraded: true, .. }` at `:95`) | `execute.rs:95:27` |
| `conductor-run/src/lib.rs:559` — the manual-path `latency_ms` subtraction | `crates/conductor-run/src/execute.rs:136` (`observed_ms - emitted_ms`, passed to `manual_record`) | `execute.rs:136:25` ×2 |
| `conductor-run/src/lib.rs:66` — the `preflight blocked` self-obs line | `crates/conductor-run/src/canary.rs:58-59` (the `!state.ready` guard at `:58`, the line at `:59`) | `canary.rs:58:8` |

Verified-as-written (no correction needed): test-plan **§6** does carry the driven arm's full firing form including the
quiet window; test-plan **§9** does carry the live-leg CI ban; the disposition ledger does carry **classes B and C**; and
the SUT dedupe coordinate holds byte-exact — `andromeda-pulse pulse-app/src/inference_runtime.rs:811` at HEAD `83d4060`
is `.find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)`, re-read this session.

### CARRY 1 (from `2026-09-01-desktop-a11y-sweep`, operator wrap directive)
- **Mechanism, marker text kept verbatim — VERIFIED at P3:** the driven a11y arm's live precondition is
  SELF-COLLIDING — `conductor preflight` fires its own canary storm, and Pulse dedupes a new incident against any
  OPEN one on the same `(kind, scope, scope_id)` tuple (*as measured at
  `andromeda-pulse pulse-app/src/inference_runtime.rs:811`, HEAD `83d4060`*), so a second canary inside the ~120s
  idle + 30s resolver window forms no FRESH incident and preflight blocks. The `measured at` marker set the depth:
  the evidence pointer was spot-checked still-true at HEAD (the line reads
  `.find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)`).
- The RELIABLE shape is the quiet window, already recorded as part of the leg's firing form (test-plan §6).
- **An undecided design fork this entry MAY weigh — `not decided` in the entry's own words:** a PER-RUN CANARY
  IDENTITY, salting the canary's `scope_id` so the preflight tuple and the driven-run tuple differ by construction,
  removing the collision instead of waiting it out. Weighable here because this entry's stated job is a re-runnable
  live-Pulse proof invocation. **Not resolvable at P1** — it goes to P4 as a fork, not a lean.
- Either way the suite stays **never a CI gate**, and test-plan §9's live-leg ban is unchanged.

### CARRY 2 (from `2026-09-03-conductor-run-composition-root-survivors-dispositioned`, operator WRAP directive item 2)
Three `conductor-run` mutation survivors are accepted-deliberate at the TIER because no hermetic test can reach their
sites, but their OBSERVABLES are live-path facts a driven run can assert **from persisted records** — so the coverage
is owned here rather than parked inside an accepted class. Assert, on a live-Pulse driven run:
1. `Observation.degraded == true` on a declare-only scenario whose read-back finds an empty active set (the
   `ReadBack::AutoResolved` arm — re-aimed to `execute.rs:91-96`).
2. the persisted `latency_ms` equals `read_back_observed_at − journal_emitted_at` on the manual path (re-aimed to
   `execute.rs:136`) — **a subtraction, not a sum or quotient**.
3. the `preflight blocked: readiness gate not satisfied` self-obs line appears on a NOT-ready gate and is absent on a
   ready one (re-aimed to `canary.rs:58-59`), which needs a connected sidecar and therefore a live leg.
- **Mechanism, marker text kept verbatim — VERIFIED at P3:** each is unreachable from a unit test *for a measured
  reason* — a fixed `http://127.0.0.1:4317` endpoint const contended under `--jobs 2`; a crate-private `Preflight`;
  a spawned `andromeda-pulse-mcp` absent on a hermetic host. All three re-derived at HEAD:
  `DEFAULT_OTLP_ENDPOINT = "http://127.0.0.1:4317"` (`conductor-emit/src/client.rs:23`); `Preflight`'s `client` and
  `ready` are `pub(crate)` (`canary.rs:32-33`) under a doc comment stating they are never `pub`; the sidecar is a
  fixed program NAME resolved through the inherited `PATH`.
- `[added at P3]` **The persisted proxy for observable 1 exists, but not as a `degraded` field.** `RunRecord`'s
  eleven fields carry no `degraded` key and `degraded_mode_response` appears NOWHERE in `crates/` (0 hits) — so
  obs-plan's "envelope extras like `degraded_mode_response`" has no implementation. What DOES discriminate is the
  declare-only state mapping: `manual_record` sets `state: state_for(observation, ReportState::ManualCheck)` and
  `state_for` returns `KnownResidual` iff `observation.degraded`, so on a declare-only scenario
  `state == KnownResidual` ⟺ `degraded == true`. Which ROUTE produced it is separated by the self-obs line
  `declare-only read-back empty: no active incident outlived the emission window` (`execute.rs:92-94`), which
  fires only on the `AutoResolved` arm.
- `[added at P3]` **Observable 2 cannot be asserted as an exact equality from persisted records.**
  `now_rfc3339` (`conductor-core/src/obs.rs:212-219`) truncates to `as_secs()`, so both envelope instants are
  whole-second while `latency_ms` is millisecond-grained from `now_ms()`. The assertion is a BOUNDED one
  (|`latency_ms` − 1000·Δsecs| < 1000), and it still kills `execute.rs:136:25`'s `-` → `+` / `/` mutants, whose
  outputs miss by far more than a second.
- **Landing these does NOT retire the tier acceptances:** the mutants stay accepted-deliberate (a live leg is never a
  CI gate, test-plan §9), and this CARRY buys the behavioural coverage the accepted class cannot. Test-plan §12's
  roster is unchanged by this chunk.

## Goal

Give Conductor ONE named, re-runnable, operator-gated invocation that fires the live-Pulse proof legs in a single
correct shape and lands a durable evidence record — so a live proof stops being a hand-assembled paste of env blocks
whose partial forms fail silently, and so the three CARRY-2 observables get asserted from persisted records for the
first time. It is never a CI gate and never becomes one.

## Work items (WHAT, with boundaries)

### W1 — the single invocation
- **[premise-corrected: SIX families / eight invocations, not "at least four" — measured at P3]** The live-proof
  surface today is scattered across SIX firing-form families with different env blocks, orderings and evidence
  destinations: (1) `agent-run boot`; (2) `agent-run run` plus its CONDITIONAL `SCENARIO=<name|P-ID> [SEED=<n>]`
  scenario leg — the only leg that rewrites `logs/agent-latest.jsonl`; (3) `agent-run run --e2e` → `npm run a11y`;
  (4) `cargo test -p conductor-run --features live-pulse --test lifecycle_live` (the one feature-gated live driver,
  whose own header already documents a full one-paste env block and the two pacing facts); (5) `npm run a11y:driven`;
  (6) the three `sr*` suites `npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error`. Both harness shells ship each
  verb at identical semantics.
- Outcome: one entry point that composes the legs in the correct order with the quiet window honoured, refuses to run
  a partial env rather than degrading to a silent `[BLOCKED]`, and is invocable repeatedly without hand-editing.
- **Boundary — the invocation's SHAPE is not fixed by the entry** (a P4 decision, **narrowed at P3**): test-plan
  §9's sanctioned live-leg SET has exactly four members — `workflow_dispatch`, `scripts/agent-run.sh`, a
  cargo-feature-gated test file invoked directly, and an npm-script wdio suite. A NEW `conductor-cli` verb is not a
  member as written, and the harness's 5-command discipline forbids a 6th `agent-run` command without a test-plan
  amendment — so the live-set-compatible options are a STAGE FLAG on an existing `agent-run` verb, a
  `live-pulse`-gated test target, or both. Whichever is chosen must not put a live leg on any default
  `nextest` / `clippy` / release path.

### W2 — the evidence it carries
- **Precedent to follow — VERIFIED at P3:** the harvest tier (`crates/conductor-run/tests/*_harvest.rs`, **nine
  targets** confirmed) already grades live claims against COMMITTED frozen leg lines sliced from
  `{data_dir}/logs/agent-latest.jsonl.<date>` by a pre-leg line count, and `lifecycle_live.rs:30` states the split
  in its own words — it "PRINTS its observation rather than asserting a pinned expectation: the capture is the
  deliverable, and the grading lives in `lifecycle_harvest.rs` over the frozen lines." The capture/grade split is
  the existing shape; this chunk reuses it rather than minting a second evidence convention.
- Outcome: a re-run produces an evidence artifact under this chunk's `evidence/` that records WHAT was measured, on
  WHICH arm, and against WHICH SUT HEAD — host-path-free per the artifact-hygiene invariant.

### W3 — the three CARRY-2 observables asserted from persisted records
- Each of the three (above) asserted on a live-Pulse driven run, from persisted records rather than from process
  internals. The tier acceptances are untouched.

### W4 — the CARRY-1 fork, weighed and recorded
- The per-run canary identity option is either adopted (with its `scope_id` salting shipped) or explicitly declined
  in favour of the recorded quiet-window shape — **decided at P4 with the operator, never silently**. Whichever way,
  the decision and its basis are recorded.
- **RESOLVED at P4 (operator-decided): DECLINED.** The quiet window is kept as the recorded reliable shape, and
  shipped canary behaviour is unchanged. Basis: test-plan §6 already records the quiet window as the firing form,
  and research added the fact that the dedupe collision is the cheapest lever for producing observable 3's
  negative arm — a CONNECTED client with an UNSATISFIED gate, the only state that reaches `canary.rs:59`.

## Amendments from validation-1 (intent-incomplete — P5, 2026-09-06)

Two justified divergences the plan surfaced; the intent was incomplete, so scope is amended to the better
understanding rather than the plan re-worked.
- **The `sr*` screen-reader suites are NOT composed by `--live`.** W1's "composes the legs" did not enumerate
  which. They are excluded because they are a screen-reader proof rather than a live-Pulse proof (only the `sr`
  subject uses live Pulse at all) and they need `CONDUCTOR_NVDA` plus a fifth `CONDUCTOR_SCENARIOS_DIR` handle.
  They keep their own npm invocations. The driven a11y arm IS composed, because CARRY 1 is specifically about
  its live precondition.
- **The suite fires THREE scenario legs, not one.** B1 (short declare-only — observable 2's subject, and
  observable 3's absent arm), B2 (fired deliberately INSIDE the quiet window so its canary dedupes and the gate
  goes not-ready-but-connected — observable 3's present arm), and A (the silent scenario — observable 1's
  subject). The scope named the observables but not the leg count they imply.
- **A purpose-built scenario is a DELIVERABLE, not just a subject.** `scenarios/auto-resolve-idle-window.toml`
  (one silent phase, `gap_ms = 165000`, `occurrences = 0`, `p_ids = ["P-022"]`, plus the mandatory
  `LOAD ENVELOPE` header comment) is authored for observable 1's subject. The mechanism: `observe`
  (`crates/conductor-verify/src/extract.rs:76`) calls `query_incident_list(None)` with no filter and returns
  `EmptyCorpus` only when the active set is empty, while the run's own preflight canary incident stays open for
  120s idle + a 30s tick.
  **[premise-corrected at P5 round 2: reachability is PER-SCENARIO, not a property of the family.** An earlier
  draft of this bullet said the arm is "unreachable with every committed declare-only scenario" — false.
  `ack-cooldown` (370s of phases, no emission tables) DOES reach it, proven live at leg E on 2026-08-21 and
  pinned at `crates/conductor-run/tests/severity_harvest.rs:505-513`. Only the SHORT declare-only scenarios
  (2–75s) are unreachable. The new scenario is therefore chosen on wall clock (165s roughly halves leg A) and
  declared intent, never on reachability.]**
- **Each leg's self-obs must be FROZEN before the next leg runs.** `logs/agent-latest.jsonl` is opened
  `.truncate(true)` (`conductor-core/src/obs.rs:108-111`), so a single end-of-suite read would see only the last
  leg; and a per-leg `CONDUCTOR_RUNS_DIR` does not separate them, because `agent_log_path` resolves the logs dir
  as `runs_dir.parent()/logs` (`conductor-cli/src/paths.rs:87-88`).
- **The suite's wall-clock budget is ~12–15 minutes**, not the ~150s per leg the scope's quiet-window framing
  implied: each preflight pays a 45s warm-up and a not-ready gate pays the 90s poll floor
  (`contracts/pulse-run-contract.toml:26-27,30`).

## Boundaries (what this chunk does NOT do)

- **Never a CI gate.** test-plan §9's live-leg ban and arch's CI/CD note stand; no CI workflow gains a live stage.
- **Does not retire the accepted-deliberate mutation classes B and C** (CARRY 2 says so in its own words); test-plan
  §12's roster is not edited by this chunk.
- **No PULSE UI automation** (scope law). Driving Conductor's own webview stays in scope.
- **No new inbound listener.** The `:4317` port-occupier and the dev-only `4444`/`4445` driver ports remain the whole
  deliberate-bind set.
- **Does not manage the Pulse process.** Launching `pulse-app` stays the operator's act; the suite may REFUSE to run
  when preconditions are unmet, but it never starts or stops the SUT.
- Claims no capability by itself; whether any pooled matrix entry becomes fully verifiable here is decided at P5.
