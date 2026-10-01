# Fan-out results — 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix

Seven Explore doc-agents, one parallel batch, the amendment-flow prompt verbatim (no doc migrated under U35: the
contracts line dropped for all seven — `registry.py contracts` exit 3 `NOT MIGRATED` for arch/tests/obs/a11y, `n/a`
for security/design/layouts). Every return was YAML; stripping removed only `#` commentary lines (each doc's
per-detector no-hit reasoning, kept in substance below); no HTML entity appeared in any return (`entities=0`).

## Verdicts

- **a11y-plan** — `proposals: []` (D-a11y-surface: no interactive UI; D-a11y-obs-schema: neither schema changed;
  D-platform-claim: no retired platform verdict).
- **layout-templates** — `proposals: []` (no surface; the moved counts appear in no caption/sample — the `dismiss` hit
  at `:115` is Radix dialog dismissal; `--live real-model` documented unchanged at `:188`/`:191`).
- **design-system** — `proposals: []` (the one `2026-09-30` hit at `:265` is the footer strip's shipped date).
- **obs-plan** — `proposals: []` (no must-trace op, no dependency, no CI gate step; §4 Real-model posture bullet at
  `:305` still holds — envelopes `ManualCheck`/null, eleven keys).
- **test-plan** — 1 proposal (T1).
- **security-plan** — 3 proposals (S1 primary; S2, S3 `dependent-of: D-security-input`).
- **architecture** — 2 proposals (A1, A2); the agent ran `arch-registry-check.py measure`: within target, §Occupied
  Resources 38110 B against a 38115 B threshold (5 B headroom).

## Proposals and dispositions

### T1 — test-plan · D-tests-derived-count · warning
- section: §6 E2E Test Strategy → Scenario: Fingerprint-storm → Real-model interpretation leg (`test-plan.md:336`)
- change: add the 2026-10-01 series beside the dated list (Pulse `a2addb3`; d1 `Identified`, d2 not graded — its
  scenario spans refused `append_failed`, d3 `NotIdentified`; `v3-09` not met), the fix's live witness beside the
  verdict, and the `interpretation.incident.skipped` / `skip_reason` harvest arms; no total count — the dated list names
  the set.
- **Disposition: APPLY** — check 1 playbook `:308` (Accurate this-chunk addition: the series and the arms are in the
  report's Changes / Outcome); checks 2-6 clear; check 5 carries the plan's test-plan entry.

### A1 — architecture · D-arch-resources · warning
- section: §Occupied Resources → On-disk artifacts → `contracts/pulse-real-model-leg-posture.md` (`architecture.md:182`)
- change: the drive-series list reads (2026-09-29, 2026-09-30, 2026-10-01 — each an add-only section fixed before its
  first drive), byte-neutral or shrinking.
- **Disposition: APPLY** — playbook `:308`; check 5 carries the plan's arch entry; D-arch-registry-size re-measured
  after apply.

### A2 — architecture · D-arch-resources · warning
- section: same row
- change: the provenance clause names both re-pins (`fcc31b2` 2026-09-30, `a2addb3` 2026-10-01), shorter than today's
  parenthetical so it pays for A1.
- **Disposition: APPLY** — playbook `:308`; the report's Cross-project bullet carries `a2addb3`.

### S1 — security-plan · D-security-input · escalate (primary)
- section: §Input Validation → Real-model capture ingest row (`security-plan.md:121`)
- change: the capture also prints Pulse's `interpretation.incident.skipped` lines fields-only and a trailing
  `skip_reason` on every `canary:` line, through the unchanged chain; `workspace_key` coordinate `fcc31b2` → `a2addb3`
  (content unchanged); qualify "a fingerprint-shaped token prints as `<fingerprint>`" — `elide_fingerprints` keeps
  all-digit runs by design, so an all-digit `fingerprint_hex` prefix passes; the d3 residual named.
### S2 — security-plan · D-security-input · escalate · dependent-of D-security-input
- section: §Security Anti-Patterns → Data Protection, the corpus.db ban's capture exception (`security-plan.md:335`)
- change: qualify "every fingerprint-shaped token elided" for all-digit runs; name `rm-capture-d3.txt`'s all-digit
  prefix beside the frozen 2026-09-22 residual.
### S3 — security-plan · D-security-input · escalate · dependent-of D-security-input
- section: same bullet (`:335` @1068)
- change: the enumeration gains "the 2026-10-01 series' d1 and d3 captures each carry one report body …; d2 read none".
- **Disposition (the group, atomic): ESCALATE** — check 1: playbook `:124` (Boundary widening → escalate, "always a
  human's call"). The witness-line half, the coordinate re-pin and S3 are accurate this-chunk additions (`:308`) and
  would be routine alone; the residual half is not: both sites state an elision guarantee ("every fingerprint-shaped
  token elided" at `:335` @~750; "a fingerprint-shaped token prints as `<fingerprint>`" at `:121` @~1460) and name ONE
  residual whose basis is that frozen evidence is never edited (`:335` @1890). d3 is a GRADED copy committed after
  the fact with an un-elided (all-digit) fingerprint prefix — committed evidence admits a value the boundary says
  it removes. `:118` does not govern (its precondition — the operator dispositioned the defect as a ROUTE candidate
  — fails: the ruling was a stated residual, no route). Project rule `.claude/rules/security.md` 2026-09-29: "A boundary
  widening is ratified only by the founder's live word, never by a delegate". The overseer's option-(a) ruling is on
  record (report Deviations); whether it is a widening, and so whose word records it, is the operator's call.

- **Escalation RESOLVED (the overseer, 2026-10-01, AskUserQuestion):** "Residual, founder pending" — "a delegate must
  not decide whether this is a widening, so it is recorded as overseer-ruled residual (synthetic canary content, counted
  exactly by the harvest; elide_fingerprints keeps all-digit runs by its own definition), founder ratification PENDING.
  I am asking him now beside the v3-09 question; I will relay his word. Apply the routine halves unchanged." → S1, S2,
  S3 APPLIED in that form; no playbook rule minted (`:124` forbids a routine rule for this class).

## Validate — the other checks
- Check 2 cross-contradiction: none (A1/A2 one row, complementary; S1-S3 one group).
- Check 3 intent-consistency: the scope record's two lines are `widening` with the overseer's quoted word — justified.
  The report diverges from intent on no criterion (v3-09 not met is a measured outcome the plan pre-states).
- Check 4 absence-needs-evidence: the agents' no-hit claims cite their searches; security-plan lines read by offset
  (`splice.py summary`: 9 lines over 2 000 chars; `:121` 3571c, `:335` 2267c — both windows read whole).
- Check 5 expected amendments: arch → A1+A2 · test-plan → T1 · security-plan → S1(+S2, S3) · obs-plan → not carried,
  no proposal (report states the envelope shape is the one the bullet names). Floor met.
- Check 6 disproved claims: (1) `.claude/rules/verification-harness.md:67` "Pulse logs NO line when its model
  DISMISSES" → P3 curation (a rules file, not a master; test-plan/arch grep 0). (2) the drive-spacing presumption →
  routed to P5 (relay §2 entry 1, "Per-run span identity in the real-model harness"); no master states it sufficient.
