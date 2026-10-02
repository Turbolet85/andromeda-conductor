# Fan-out results — 2026-10-02-p-075-assert-round-against-pulse

Seven Explore doc-agents in one parallel batch, each holding its master + the report + its scoped detectors (no
keyed-contract render: `registry.py contracts` read arch / test-plan / obs-plan / a11y-plan `NOT MIGRATED` and the
other three `n/a`, so `{contracts_line}` was dropped). Strip: each return was YAML plus `#` comment notes. Entity probe:
no `&lt;` / `&gt;` / `&amp;` in any return, so nothing needed decoding. No raw twin, because no `proposals: []`
return was changed by stripping (its comment notes are reasoning only) and none failed the parse.

## Verdicts
- architecture: 3 proposals (A1 primary · A2 / A3 `dependent-of: D-arch-resources`)
- security-plan: 1 proposal (S1)
- design-system: `proposals: []` (D-design-tokens: all coverage tokens n/a · D-design-derived-count: no moved count
  in the doc · D-platform-claim: no stating sentence)
- layout-templates: `proposals: []` (no new surface; moved counts absent; `tools 4/4` is the unchanged MCP tool count)
- test-plan: 2 proposals (T1 · T2)
- obs-plan: 1 proposal (O1)
- a11y-plan: `proposals: []` (no UI element; no schema change; `headless` mentions name a surface, no verdict)

## Parsed proposals + dispositions

### A1 — D-arch-resources · warning · §Occupied Resources → On-disk artifacts
change: register `runs/span-landing/span-{a,b}.jsonl`. It is harness-owned with no `CONDUCTOR_*` handle and moves
with `CONDUCTOR_RUNS_DIR`. The two named files are cleared by a non-recursive `rm -f` before drive A. Its reader is
`span_landing_live` via `capture_paths::runs_dir_from` (`span_dir()`), and `pair_is_current()` refuses a stale pair.
basis: report.md:31-37.
**Disposition: APPLY.** Check 1: playbook "Accurate this-chunk addition" (routine), because the artifact path moved
this chunk (report Changes → Symbols, `span_landing_live`). Check 3: it matches plan Expected amendments (arch
§Occupied Resources). Bytes are held to the plan's neutral-or-negative registry target, so it is a compact entry.

### A2 — D-arch-resources · warning · §Occupied Resources → `runs/live-suite/{leg}.jsonl` · dependent-of A1
change: drop the "Second, operator-local writer: the span-landing pass's `cp` … (move route-owned)" sentence.
**Disposition: APPLY** (atomic with A1). The report Changes → Harness side effect says the suite's `rm -f` no
longer reaches the pair. D-arch-collision: the move REMOVES a second writer from a registered path and adds no owner.

### A3 — D-arch-resources · warning · §Infrastructure Patterns → directory tree · dependent-of A1
change: trim the `live-suite/` tree comment's span-pair clause and add a sibling `span-landing/` line.
**Disposition: APPLY** (atomic with A1).

### S1 — D-security-input · escalate · §Input Validation → span-landing ingest row (:122)
change: name `runs/span-landing/span-{a,b}.jsonl` and add the `pair_is_current()` stale-pair refusal.
basis: `crates/conductor-run/tests/span_landing_live.rs:40-41, :46, :196`.
**Disposition: REJECTED, then RAISED as SR1.** Validate preamble: the `basis` cites source line numbers the report
does not carry, which is the re-derivation tell. The fact itself is in the report (Changes → Symbols,
`span_landing_live`) and is a plan Expected amendment, so check 5 raises it from the report alone as SR1. The
D-security-input escalate class does not fire: there is no new input surface. The reader, the handle and the guard
(`capture_paths::runs_dir_from` → `resolve_under`) are unchanged, and only the joined subdir constant moved. The
added refusal narrows what is graded. So the playbook rule "consuming shipped hardened infra" (routine) governs,
and no boundary is widened.

### T1 — D-tests-coverage · warning · §2 Test Strategy → Deterministic bullet (:124)
change: add the operator-gated `p075_round_live` leg to the real-wall-clock operator-local exception list, with its
grades held at the harvest tier.
**Disposition: APPLY.** Check 1: "Accurate this-chunk addition". The leg is NEW this chunk (report Symbols), runs on
real wall clock against a live Pulse, and §2 enumerates every such leg. The text is re-derived from the report, so the
change line's tool roster is kept only where the report states it.

### T2 — D-tests-derived-count · warning · §9 Live-Pulse scenarios (:467)
change: add `conductor-run/tests/p075_round_live.rs` (2026-10-02) to the `live-pulse` gated SET, naming the set with
no count literal.
**Disposition: APPLY.** Check 1: "Accurate this-chunk addition". The report Counts bullet reads the gated set 4 → 5.
This also discharges the plan expected "§9 `p075_round_live` joins the gated set with its clippy line".

### O1 — D-obs-instrumentation · warning · §4 Fingerprint-storm → `verify.readback_fingerprints` bullet (:315)
change: retire "no span attribute or shipped check computes it". A test-tier check now computes membership; there
is still no span attribute.
**Disposition: APPLY.** Check 1: "Accurate this-chunk addition" (the check is this chunk's,
`lifecycle_harvest::p075_round_assertion_1_*`). Check 5: plan expected (obs §4 Fingerprint-storm). Scope is held
to the measurement, Pulse S `03ec944` on one capture.

## Orchestrator raises (check 5 — the plan's Expected amendments floor; check 6 — disproved claims)
- **AR1 — arch §Established Decisions [Read-Back Dependency Posture] (the `Freshness is the canary's carrier`
  passage, :63) · routine.** Re-measured at Pulse S `03ec944`: `fingerprint_refs` is the grounded union, written
  only at creation (a dedupe never updates it). Conductor's emitted fingerprint is a member (4 refs / 3 `det-*`),
  and `retrieve_report` read `degraded_mode: false`. Cited as measured at the chunk's `evidence/round-ledger.md`.
  Basis: report Changes → Cross-project + Outcome assertion 1. Arch :70 already states "`degraded` is a
  PER-READ-BACK property, never a mode-wide guarantee", so arch holds no contrary claim and :70 is unchanged.
- **SR1 — security-plan §Input Validation span-landing ingest row (:122) · routine** (see S1).
- **SR2 — security-plan reader roster (:114 / :123) for `p075_round_live` · NO CHANGE.** The plan entry is
  conditional ("any reader-list row naming test-binary readers of `ANDROMEDA_PULSE_DATA_DIR` gains it"). Both rows
  enumerate test-binary VALUE readers that join the handle with `logs/`. `p075_round_live` joins no path; it passes
  the value only to the shipped `ReadbackClient::connect` (report Symbols), like `lifecycle_live.rs`, which neither
  row lists. No qualifying row exists. The :115 / :222 / :326 `CONDUCTOR_RUNS_DIR` reader count is unchanged at
  four (report Counts).
- **TR1 — test-plan §2 (:124) span-landing input path · routine.** Applied in the same edit as T1: the witness reads
  `runs/span-landing/span-{a,b}.jsonl`, clears the two named files before drive A and refuses a stale pair.
  Search: `span-landing|span_landing|span-a|span-b|live-suite|same-seed drives` over test-plan returns :124 ×2,
  :155 ×2, :336 ×2, :467 and :548. The windows were read: :155 is the `--live` suite's `{leg}.jsonl` and the
  real-model `rm*` files (true, not span-landing); :336 is the dispatch_wire + witness proof (unchanged); :467 is
  the gated set (T2); :548 is the 180 s firing-form wait (unchanged). No §9 / §11 site states the input path, so
  none changes.
- **TR2 — test-plan §5 `mark_incident_resolved` (:284) · routine.** Re-graded at S: assertion 2 is
  `ProvenByLiveness` (idle 12 ms) through the unchanged `probe_resolve_lifecycle` + `attribute_by_liveness`, held
  by `lifecycle_harvest::p075_round_assertion_2_runtime_state_fidelity`. Basis: report Outcome.
- **TR3 — test-plan §6 Fingerprint-storm verification signal (:335) · routine — DISPROVED CLAIM 1.** Retire "because
  `retrieve_report` is permanently `degraded_mode` in this mode". MEASURED `degraded_mode=false` at S
  (`evidence/p075-leg.txt`, `evidence/pulse-r.jsonl`). The token checks stay declare-only on the surviving reason
  (every L4-authored field is a fixture constant under deterministic L4). Assertion 1's membership check is
  added. Playbook rule: "a spec master's OWN … claim retired by the measurement" / "Accurate this-chunk addition".
  Sweep for the same claim across all seven masters (`degraded|always-degraded|is_none()`), every hit read:
  - test-plan :33 / :54 / :79 / :245 / :283 / :355 / :357: the `degraded_mode ⇒ KnownResidual` mapping, which is
    true per read-back. No change.
  - test-plan :323 / :324: the `error-baseline-spike` CI path against the stub MCP (§6 Steps 1). That is the stub's
    degraded report, not a mode-wide live claim. No change.
  - test-plan :616 / :621: the mutation class B `degraded` field. Unrelated.
  - arch :62 / :153: the tool roster; :70 / :135 already per-read-back. No change.
  - obs :132 / :134 / :305 / :326 / :341 / :345 / :350: "under the degraded read-back", a per-read-back route; :345
    states `degraded_mode = parsed_l4.is_none()`, which is true.
  - obs :225 / :348 / :356 / :440: the retired `degraded_mode_response` extra; :465 is the log-level table. No
    change.
  - security / a11y / design / layouts: 0 hits.
  The `permanently` hit at test-plan :284 is the DECLINED-arm stub, unrelated (report Spec claims disproved 1).
- **OR1 — obs-plan §4 Delegated-timing family (:350) · routine.** Re-graded at S: P-025 worst 478.56 ms in-window
  (rise anchored 38.24 ms); P-027 worst 605.26 ms; P-037 0 ms; P-045 worst 5.0 ms of 163. Each grader is
  `delegated_timing_harvest::tests::p075_round_assertion_{3..6}_*`. The P-025 coordinate notes moved in the
  measurement contract (report Schema / config).

## Other validate checks
- Check 2 (cross-contradiction): none. A1–A3 move one claim, and T1 + TR1 share :124 in one edit.
- Check 3 (intent-consistency): every apply is on the plan's Expected list or follows from the report. The 5
  deviations each carry their justification. The scope record has one line, `in-intent`, `evidence_pin/mod.rs`,
  which serves step 3 — it holds.
- Check 4 (absence needs evidence): the TR1 / TR3 / SR2 searches are named above, with every hit dispositioned.
- Check 6: claim 1 → TR3. Claim 2 (the chunk's own P-037 premise) is not a spec-master claim; it was already
  corrected in scope / research / plan on the overseer's ratification, so it is routed and DISPOSED.
- Escalations: 0.
