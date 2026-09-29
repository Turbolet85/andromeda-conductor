# Fan-out results — 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin wrap

This is a resume: P1 ran in an earlier window and its `report.md` is used as it stands. No `fanout-results.md`
existed, so P2 re-fanned the seven detectors in one batch. None of the seven docs carries keyed contracts (U35 has not
run, and there is no `.andromeda/registries/`), so the contracts line was dropped from every prompt.

## Verdicts
- **architecture** — 4 proposals. Stripped: trailing commentary on detectors with no hit. D-arch-collision: `:4317`
  is still Pulse's. D-platform-claim: no verdict retired. D-arch-decisions: no dependency. Pulse model env handles:
  no shipped artifact names them. D-arch-registry-size is left to the orchestrator.
- **security-plan** — 3 proposals, all `escalate`. Nothing stripped.
- **design-system** — `proposals: []`. Stripped: commentary per detector (no platform verdict retired; both new
  surfaces `tokens n/a`; no old count in the doc). Raw twin: `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: commentary. `:188` keeps the placeholder `(N unbacked)`, which
  is correct; no new surface; no platform verdict. Raw twin: `.raw-fanout-layout-templates.md`.
- **test-plan** — 5 proposals. Nothing stripped.
- **obs-plan** — 1 proposal. Stripped: the no-hit notes for D-platform-claim, D-obs-stack, D-obs-redaction and
  D-obs-ci-gates, which are folded into the proposal's rationale.
- **a11y-plan** — `proposals: []`. Stripped: commentary (no UI element, no schema change, no platform verdict). Raw
  twin: `.raw-fanout-a11y-plan.md`.

## Parsed proposals, with dispositions

### architecture
- **A1** · D-arch-decisions · warning · §Established Decisions → [Read-Back Dependency Posture] (`:70` @c4018).
  - Change: the diagnostic-quality four are no longer "in the `UNBACKED_AUTO` ledger". They are backed by
    `real-model-interpretation`'s `p_ids` and their own harvest arms. The pin now holds 4 ids (P-039/P-041/P-042/P-043).
    Backing is not verifying, so `v3-09` is still not met.
  - Basis: `drift.rs:61` (the report carries it); `architecture.md:70`.
  - **Disposition: APPLY.** Playbook `:308` (Accurate this-chunk addition): the ids and the count are in the report's
    Counts, Schema and Spec-claims-disproved bullets. It covers expected amendment 3: `interpretation-correctness` has
    0 hits in arch, and `:53` (@c2241, window read) describes the pin generically, with no membership. Check 6: this
    disposes disproved claim 2 (the arch half). The rules-file half, `verification-harness.md:47`, is a distillation
    and goes to the cascade.
- **A2** · D-arch-resources · warning · §Occupied Resources → `contracts/pulse-real-model-leg-posture.md` (`:182`).
  - Change: the contract now carries the pre-stated drive series (§The drive series), and its Pulse coordinates are
    re-pinned at `e98d838`.
  - **Disposition: APPLY.** Playbook `:308`; expected amendment 4. The headroom is 18 B
    (`arch-registry-check.py measure`: 38 097 of 38 115 B), so the entry's replaced history moves to the sidecar and
    the edit is kept within the threshold. The body text is re-derived from the report (re-pin `f15536b` → `e98d838`,
    cited files byte-identical), not pasted.
- **A3** · D-arch-resources · warning · dependent-of D-arch-resources · [Read-Back Dependency Posture] (`:70`).
  - Change: "driven ONCE (2026-09-23)" → the pre-stated series. The 2026-09-29 series ran 5 counted drives. The
    three-storm canary cleared the canary block. b2 read `NotIdentified` on `[redacted: credit_card]`. `v3-09` is not
    met.
  - **Disposition: APPLY** (the group's primary, A2, is applied). Playbook `:308`; disposes disproved claim 1 (arch
    restatement). The route state ("BLOCKED-ON Pulse's scrubber fix") is left out of the body; it belongs to P5.
- **A4** · D-arch-resources · warning · §Standard Contracts → Readiness gate (`:93`).
  - Change: under the real-model posture, the canary is `REAL_MODEL_CANARY_STORMS = 3` storms of `CANARY_STORM_COUNT`
    12, `REAL_MODEL_CANARY_STORM_GAP = 90 s` apart, with the stamp taken before the first. The deterministic canary is
    byte-identical.
  - **Disposition: APPLY.** Playbook `:308`; expected amendment 5 (step 9 landed). `:178` ("the shipped
    `CANARY_STORM_COUNT` of 12 clears") still holds per storm, so it needs no edit.

### security-plan
- **S1** · D-security-input · escalate · §Input Validation → Real-model capture ingest row (`:121`).
  - Change: sweep window, report body, new line classes, `elide_fingerprints` on every line, per-drive captures, and
    the 2026-09-22 residue.
  - **Disposition: REJECTED** under Validate's pre-check (the re-derivation tell). Its `basis` cites
    `real_model_common/mod.rs:149/:176/:187` and `real_model_live.rs:62`, source line coordinates the report does not
    carry. **Its fact is RAISED by the orchestrator** under check 5 (expected amendment 6, "security-plan §Input
    Validation real-model capture ingest row"), re-derived from the report alone: Files bullet (`real_model_live.rs`),
    Schema bullet (line grammar), Deviations (the sweep window), Cross-project bullet 3, Spec claims disproved bullet 3.
    See **OR-S1** below.
- **S2** · D-security-input · escalate · dependent-of D-security-input · §Security Anti-Patterns → Data Protection
  (`:335`).
  - Change: "envelope fingerprints elided" → `elide_fingerprints` on every fingerprint-shaped token; "The 2026-09-23
    capture carries none — no report was read" → dated, plus the 2026-09-29 series carries report bodies.
  - **Disposition: REJECTED with its primary** (check 6's atomic group rule). Its content re-enters as OR-S1's
    same-master duplicate in the cascade sweep.
- **S3** · D-security-input · escalate · §Security Anti-Patterns → Data Protection (the exception's scope, `:335`).
  - Change: the exception scopes corpus-rendered text to "a chunk's committed `evidence/` tree … anywhere else the
    ban binds unchanged". The six series capture blocks, including b2's Pulse-rendered report body, are committed
    verbatim in `crates/conductor-run/tests/real_model_series/mod.rs`, outside any `evidence/` tree. Ratify a widening,
    or record a breach.
  - Premise measured by the orchestrator: that module holds `SERIES: [Drive; 6]`, and `## Project Context` and
    `[redacted: credit_card]` each appear in it (grep counts 1 and 2), copied byte-equal from `rm-capture-b2.txt`.
  - **Disposition: ESCALATE.** Playbook `:124` (Boundary widening — always a human's call, never routine).

### Orchestrator raises (check 5 / check 6)
- **OR-S1** · raised · security-plan §Input Validation → Real-model capture ingest row (`:121`), plus its same-master
  duplicate at §Security Anti-Patterns → Data Protection (`:335`).
  - The row gains the report body printed verbatim from its first `## `, the corpus-rows line, the `canary:` lines,
    `scenario_storm=` on storm lines, `elide_fingerprints` after `redact_value` + `mask_host_paths` on every printed
    line, the anchored sweep window (highest id seen minus 63, floor 1, cap 64), and per-drive captures
    `evidence/rm-capture-{drive}.txt`.
  - `:335`: "envelope fingerprints elided" → every fingerprint-shaped token elided. "The 2026-09-23 capture carries
    none" is kept as dated history, and the 2026-09-29 b2 capture carries a report body.
  - **Disposition: routine APPLY** for everything the report substantiates. It rides S3's resolution for the scope
    clause, and E2 (below) for the workspace-basename clause.

### test-plan
- **T1** · D-tests-coverage · warning · §9 Live-Pulse scenarios (`:467` @c610). "fired once and never re-driven" →
  the posture contract's pre-stated series.
  - **Disposition: APPLY.** Playbook `:308`; expected amendment 1; disproved claim 1.
- **T2** · dependent-of T1 · §6 Real-model interpretation leg (`:336`). "operator-gated, fired once, never CI" → the
  pre-stated series; the 2026-09-29 series recorded beside the 2026-09-23 drive.
  - **Disposition: APPLY.**
- **T3** · dependent-of T1 · §2 Test Strategy (`:124` @c1362). "fired once and graded" → the series, each drive graded
  against the rule recorded before it.
  - **Disposition: APPLY.**
- **T4** · dependent-of T1 · §3 5-command implementation (`:155`). "so the one drive is never spent" → "so no counted
  drive of the series is spent".
  - **Disposition: APPLY.** The test §3 ↔ obs §3 bind holds: obs-plan §3 (`:181-270`) carries no real-model selector
    text (its only `real-model` hits are `:305` §4 and `:369`).
- **T5** · D-tests-coverage · warning · §6 Real-model interpretation leg (`:336`). The P-ID set is named
  (P-018/P-031/P-033/P-034/P-044); the harvest grades P-031 `structure`, P-034 `steps`, P-044 `retrieval` and
  `canary_attempts`, appended below the byte-exact 2026-09-23 rule; each series capture is pinned byte-equal.
  - **Disposition: APPLY.** Playbook `:308`; expected amendment 2. The set is named, not counted.

### obs-plan
- **O1** · D-obs-instrumentation · warning · §4 → Real-model posture (`:305`). "The one drive (2026-09-23) was blocked
  at the preflight, so this chain was not observed live under the real-model posture" → the 2026-09-29 series
  observed it: both Stage B preflights reached ready, and b2's envelope landed `ManualCheck` with `verdict` null and
  the eleven keys. The three-storm canary adds no span or field.
  - **Disposition: APPLY.** Playbook `:308`; report Outcome (obs), Gates (b2 status), Coverage of new surfaces.

## Checks
- **1 · Playbook:** 11 applied under `:308` (A1-A4, T1-T5, O1, OR-S1 minus its escalated clauses). S3 escalates
  under `:124`.
- **2 · Cross-contradiction:** none. A1 and A3 both edit `:70` in the same direction, and apply as one splice.
- **3 · Intent-consistency:**
  - The scope record's single line (in-intent `real_model_series/mod.rs`, which serves the "each new capture pinned
    byte-equal" acceptance) holds.
  - `v3-09` NOT MET is an outcome the plan's acceptance already provides for, not a divergence.
  - The deviations are justified in the report.
- **4 · Absence needs evidence:** every hit on a line over 2 000 chars was read by offset window (`splice.py summary`:
  15 such lines in arch). A1's "`:53` is generic" rests on the `:53` window at @c1900-2971. T-group sites were read
  whole (`:336`, `:124` window, `:467` @c0-2200).
- **5 · Expected amendments:**
  - 1 → T1. 2 → T2 + T5. 3 → A1. 4 → A2. 5 → A4. 6 → OR-S1.
  - "residuals.md pins the interpretation gap" → P5 (route-resolve owns it), recorded NOT discharged.
- **6 · Disproved claims:**
  - (1) `test-plan:467` → T1-T4.
  - (2) `architecture.md:70` → A1 + A3; `verification-harness.md:47` → the cascade (a distillation).
  - (3) capture fingerprints → OR-S1 (the code is fixed); the 2026-09-22 residue → a CARRY on the new series entry at
    P5 (relay §2.3).
  - (4) the posture contract's canary classification → corrected in the contract itself. It is not a master; the
    cascade sweep checks that no master restates the "between its cadence tick and the next" segment.

## Escalations (HALT)
- **E1** = S3: the pinned series copies sit in test source, outside the exception's `evidence/` tree.
- **E2** = raised by the orchestrator from S1's clause (4) and Cross-project bullet 3. The ingest row states "the
  workspace basename is never printed".
  - At b2 the verbatim report body carries Pulse's Project Context, which shows the data dir's workspace key however
    Pulse's scrubber renders it. Measured: `[redacted: credit_card]`, with 0 raw `rm-2026…` basenames across all six
    captures and the pin module.
  - Once Pulse's scrubber is fixed (the new series' BLOCKED-ON), the next series' capture would print the raw
    basename unless the capture masks it.

## Escalation resolutions (operator, via the overseer, this wrap)
- **E1 → breach + CARRY.** Overseer: "ratifying would WIDEN the corpus exception, and a boundary widening halts for
  the founder live word (his ruling 2026-09-27); he is away." The body records the series pins in
  `crates/conductor-run/tests/real_model_series/mod.rs` as a BREACH of the exception's `evidence/`-only scope. The
  remedy is a digest pin (sha256 per evidence file, no corpus text in test source), carried by the new v3-09 series
  entry, which lands before the next series runs. It is pinned at P5 as a CARRY. No playbook rule is proposed:
  `:124` is never-routine by its own note.
- **E2 → CARRY.** Overseer: "keeps the never-printed guarantee true without widening." The row states the measured
  fact: the fields-only reads never print the basename, and b2's verbatim report body shows Pulse's rendering of the
  workspace key, measured `[redacted: credit_card]`. A CARRY on the new series entry makes the capture mask the
  data-dir workspace key in the report body before the next series. It is pinned at P5.
- OR-S1 is now fully routine (its two held clauses are resolved above) → APPLY.
