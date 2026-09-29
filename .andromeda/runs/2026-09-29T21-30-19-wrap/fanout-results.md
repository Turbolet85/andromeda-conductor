# Fan-out results — 2026-09-29-hue-shift-budget-graded-hard

Seven Explore doc-agents, one parallel batch, prompt from `amendment-flow.md` §Fan-out sent verbatim (contracts line
dropped for all seven: `registry.py contracts` → `NOT MIGRATED` for arch / test / obs / a11y, `n/a` for the rest).
Report: `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/report.md`. Entity probe: the returns carried
no `&lt;` / `&gt;` / `&amp;` (entities=0; the one `<` in the obs return is a literal `≤` sign, untouched).

## Verdicts

- **architecture** — 1 proposal (D-arch-resources). Stripping removed trailing commentary: no hit for new resources,
  decisions, collision, platform; registry-size deferred to the orchestrator's tooling run.
- **security-plan** — `proposals: []`. Stripped commentary: no new input surface, spawn untouched (the sidecar rebuild
  and the agent-launched `pulse-app` are SUT-side), zero dependency delta, no platform verdict retired. Raw twin kept.
- **design-system** — `proposals: []`. Stripped: no new UI; no old count in a palette/ANSI/token row; no platform
  verdict. Raw twin kept.
- **layout-templates** — `proposals: []`. Stripped: no new surface; `:64` wireframe row carries no timing claim; no
  old count in a caption; no platform verdict. Raw twin kept.
- **test-plan** — `proposals: []`. Stripped: 6 new tests at the harvest tier, both runners on spec, harness §3
  unchanged, no old count baked (`:616`/`:621` "27 -> 17" are mutation-survivor history); no platform verdict. Raw
  twin kept.
- **obs-plan** — 2 proposals (D-obs-instrumentation ×2, same line). Stripping removed trailing commentary: stack,
  redaction, CI gates, platform no hit.
- **a11y-plan** — `proposals: []`. Stripped: no interactive element, no schema move, no platform verdict. Raw twin
  kept.

## Proposals and dispositions

### A1 — architecture · D-arch-resources · warning
- section: §Occupied Resources → On-disk artifacts → `contracts/pulse-p025-measurement-contract.md` (`:181`)
- change: rewrite the row so it no longer reads as an unmet ask; it records the observable Pulse emits since
  `e98d838` (paint − `tier_effective_at`), the leg window and §The grading rule stated before the drive (sha256
  recorded pre-leg), the 2026-09-29 PASS worst 684.98 ms; the retired `83d4060` instrument kept as history under the
  six verbatim headings; provenance MIXED per clause at `226554a`; the no-Rust-reader sentence unchanged.
- basis: `architecture.md:181`; report Spec claims disproved 3, Counts moved.
- **Disposition: APPLY — routine.** Playbook `:149` (the row's own "what Pulse would have to emit … to become
  measurable at all … expires when that moves" is retired by exactly the measurement it names, carried in the report
  with its evidence; the replacement states the new status) and `:308` (reconciles to what the chunk shipped).
  Check 3 intent: matches the plan's expected amendment 2. Check 4: the agent's sweep (`unmeasurable|quantiz|…|become
  measurable|83d4060`) dispositions :70/:85/:135/:182/:242 and the other `83d4060` hits as unrelated — consistent with
  the report's own sweep. Applied text re-derived from the report, not pasted.

### O1 — obs-plan · D-obs-instrumentation · warning
- section: §4 → Known-residual classification path → Delegated-timing family (`:350`)
- change: P-025 from "UNMEASURABLE … MECHANISM PIN with no pass arm" to graded hard (PASS, worst 684.98 ms, the live
  pin); the tick-quantization reading and its test kept as the retired `83d4060` record; "lifts only if Pulse emits a
  DIFFERENT QUANTITY … Still Pulse intake" retired; the closing coordinate clause re-dated per clause.
- basis: `obs-plan.md:350`; report Spec claims disproved 2, Counts moved (3 → 4).
- **Disposition: APPLY — routine.** Playbook `:149`: the passage names its own precondition (Pulse emitting the
  contracted quantity) and its own expiry ("expires when that moves"); the leg is that precondition, measured, with
  evidence. Check 5: plan expected amendment 1.

### O2 — obs-plan · D-obs-instrumentation · warning
- section: same paragraph — the start-instant clause.
- change: fall source `resolved_at_unix_nano` (not `transitioned_at_unix_nano`); acknowledgement inert.
- basis: `obs-plan.md:350`; report Spec claims disproved 1.
- **Disposition: APPLY — routine**, merged with O1 into one edit of the P-025 passage (check 2: same sentence group,
  same direction — no contradiction). Playbook `:308` (reconciles a stated mechanism to the measured one). Check 5:
  plan expected amendment 1's SCOPE-clause half.

## Validate summary
- Re-derivation tell: none (every rationale/basis cites the report or the doc).
- Check 1 playbook: 3 routine (`:149`, `:308`); 0 escalations.
- Check 2 cross-contradiction: O1/O2 same site, same direction → one merged apply.
- Check 3 intent: all three match the plan's `Expected amendments (wrap)`; scope record empty (`gate.py scope` clean).
- Check 4 absence: the no-other-site claims rest on the per-line seven-master sweep in the report and each agent's own
  sweep; obs-plan `:350` is 4 572 chars — read by offset (P-025 passage from char 1 932 to end, read whole).
- Check 5 expected amendments: obs §4 → O1+O2 · arch :181 → A1 · `v3-08` ledger note → not a wrap item (phase P5).
- Check 6 disproved claims: (1) contract in-chunk + O2 · (2) O1 · (3) A1 · (4) contract dated correction at this wrap
  (operator directive; scenario comment at implement) · (5) the contract follows the harvest file; no master states
  the pairing — `grep -c 'f0c38f5'` over each of the seven masters returns 0 for all seven, so no master carries the
  2026-08-21 HEAD at all — disposed: no amendment.
