# P2 fan-out results — 2026-09-22-interpretation-proven-live

One line per doc as each doc-agent returned. A doc that returned proposals has a raw twin
`.raw-fanout-{doc}.md` beside this file.

- **a11y-plan** — `proposals: []` (clean). D-a11y-surface: no interactive UI element added. D-a11y-obs-schema: no
  envelope/verdict/state/span change. D-platform-claim: no report entry retires a platform, runner or driver
  verdict; a11y-plan carries 0 hits for `--live` / 110 / formation / real-model / posture / preconditions.
- **design-system** — `proposals: []` (clean). D-design-tokens: all five Coverage rows `tokens n/a`, `[PRECONDITION]`
  reused, no new label/ANSI/token. D-design-derived-count: none of the moved counts' OLD values appears (numeric
  hits measure other things: hex tokens, `P-037` sample, "11 of 34" CSS tokens :201, "6 columns" :320).
  D-platform-claim: no hit. Out-of-scope note (not proposed): `:320`/`:354` say a Blocked row carries "the named
  precondition string" — the posture-mismatch Blocked's reason is a scenario-level line, loosely still fitting.
- **layout-templates** — 4 proposals (raw twin `.raw-fanout-layout-templates.md`): D-layout-surface ×2 primaries —
  the `agent-run` bullet (:190) gains the `--live [real-model]` selector; the `preconditions` bullet (:187) gains
  `--for <SCENARIO>` — plus 2 `dependent-of` sites in the same bullet (the handle clause and the per-KIND grading
  sentence become posture-dependent). All four match plan Expected amendment "layout-templates §cli Primary
  screens". Caveat for validate: each `basis` cites a source line the report does not carry — re-derive the applied
  text from the report's facts. D-layout-derived-count / D-platform-claim: no hit.
- **obs-plan** — 1 proposal (raw twin `.raw-fanout-obs-plan.md`): D-obs-instrumentation — §4 CP1 ("Headless
  deterministic scenario run…") gains a posture note: the real-model scenario rides CP1's chain with no new
  span/attribute, non-degraded → `ManualCheck`, graded at the harvest tier; plus `execute_scenario`'s posture-mismatch
  Blocked arm and its message-borne line. Matches plan Expected amendment "obs-plan §4". Out-of-detector notes for the
  orchestrator: obs-plan :131 / :314 / :315 (fingerprint-storm) still state Pulse exposes no read-back surface carrying
  its computed fingerprint — the report's @83d4060 read (grounded union in `fingerprint_refs`) contradicts it (a
  cross-project fact, check-6/cascade candidate); §4 CP7 parity (:368) names no posture; §5/§10 pickup: nothing to
  record (no figure). D-obs-stack / D-obs-redaction / D-platform-claim: no hit.
- **security-plan** — 11 proposals, all D-security-input at detector severity ESCALATE (raw twin
  `.raw-fanout-security-plan.md`, condensed transcription — context ceiling): S1 READ SET row posture-aware (+S2
  dependent, the probe short-circuit bullet :378) · S3 committed-manifests row (shell-absence + posture tag) · S4
  scenario-config row (`l4_posture`, closed-enum garde skip) · S5 CLI-arguments row (`preconditions --for`, the
  `--live` selector) · S6 NEW model-output-ingest row for the capture (+S7 dependent, the data-dir spawn row's reader
  list) · S8 env-handle row: `CONDUCTOR_RUNS_DIR` test-binary reader with an unguarded join (an unrecorded
  residual, same shape as `journal_conformance.rs`; `live_suite.rs:60` predates) · S9 Data Protection — the P4
  ratification (+S10/S11 dependents, applied only if ratified). All of S1/S3/S4/S5/S6/S9 are plan Expected
  amendments; S8 is new (a residual the code carries). D-security-subprocess / -deps / D-platform-claim: no hit.
- **test-plan** — 14 proposals (raw twin `.raw-fanout-test-plan.md`, condensed): GROUP A D-tests-obs-harness ×3 (T1 §3
  :155 `--live` selector set; T2 §9 :465; T3 §2 :124) — plan Expected amendments §3/§9. GROUP B D-tests-derived-count
  ×8 (T4 §7 :394 primary + T5–T11 dependents at :39, :71, :128, :201, :214, :151, :154) — retire "one scenario per
  P-ID" (P-018 now named by two) and make the P-ID selector's indeterminacy explicit — NOT in the plan's list (a real
  derived-count consequence; validate against the report's Counts bullet). GROUP C D-tests-coverage ×3 ESCALATE (T12
  §1 Path 7 :81 both-surface parity scoped to deterministic-posture scenarios; T13 :94; T14 §6 :374) — narrows a
  Creator Brief Must-Work: the operator's decision. NOTE: the plan's Expected amendment "test-plan §6 — the
  real-model harvest leg" was NOT proposed → validate check 5 must raise it.
- **architecture** — 19 proposals (raw twin `.raw-fanout-architecture.md`, heavily condensed): D-arch-resources ×15
  (A1–A11 primaries — posture-aware readiness gate, `--for` probe, "2 of 37", run-contract kinds + SEVEN terms,
  live-suite artifacts, posture-doc entry ×3, L4 env entry, NEW `RUST_LOG` env entry; A12–A15 dependents) and
  D-arch-decisions ×4 (A16 warning = plan D3 [Probabilistic-Assertion Policy]; A17–A19 ESCALATE: retire "NO read-back
  field varies with the emitted payload" — the grounded `fingerprint_refs` union at 83d4060, a code reading, not
  live-measured; same fact as the obs agent's out-of-detector note on obs-plan :131/:314/:315). D-platform-claim: no hit.

## Fan-out complete — 7/7 returned (2026-09-23 ~08:45Z). Totals: 49 proposals (arch 19 · security 11 · test 14 ·
## layouts 4 · obs 1 · design 0 · a11y 0). Escalations to resolve: security S1–S11 (detector severity), test T12–T14,
## arch A17–A19. Validate/apply/cascade NOT started — the session stopped at the context ceiling (91 %).
