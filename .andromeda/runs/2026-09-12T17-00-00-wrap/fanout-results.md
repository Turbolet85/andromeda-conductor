# Fan-out results — 2026-09-12-ledger-gate-id-space-generalised

7 doc-agents, one per spec source, one parallel batch. **All seven returned `proposals: []`.**
0 proposals · 0 escalations · drift = 0.

**Twin disposition.** No `.raw-fanout-{doc}.md` twin was written. Each return carried its YAML payload
followed by supplementary per-detector evaluation notes; the YAML parsed clean (`proposals: []`) in all
seven and the stripping was the trivial "take the fenced block". The notes are the audit value, so they are
preserved VERBATIM in this consolidated file rather than duplicated into seven near-identical twins that
would each carry an empty payload. Entity-decode probe: `entities=0` on all seven (no `&lt;`/`&gt;`/`&amp;`
survived into any saved text). Recorded here rather than left implicit, because the letter of
`amendment-flow.md` §Fan-out ties a twin to "needed stripping".

---

## arch — `proposals: []`

- **D-arch-resources** — no drift. The sole new symbol `is_version_capability_id` is a file-private helper
  in the crate-local `tests/` target — not `pub`, not exported, no crate surface, so nothing is
  registerable. Its host crate `conductor-report` is already registered in §Occupied Resources — Crate
  names (`architecture.md:162`) and the §Infrastructure Patterns tree (`:227`).
- **D-arch-decisions** — no drift. Dependencies "none added, none bumped", 562 → 562. The chunk's governing
  decision (shape, not substituted literal) is settled *by* §Established Decisions [Accepted Capability
  Set]'s de-hardcoding ruling — it affirms a locked decision rather than contradicting one.
- **D-platform-claim** — no trigger. Both source bullets are an explicit "none".

## security-plan — `proposals: []`

- **D-security-input** — no hit. No new external-input boundary; only the pre-existing
  `env!("CARGO_MANIFEST_DIR")`, no `std::env`, no `CONDUCTOR_*`.
- **D-security-subprocess** — no hit. No sidecar spawn, preflight or data-dir surface touched; process
  census records no external process started.
- **D-security-deps** — no hit. 562 → 562, lock un-drifted, `rstest` rejected specifically to avoid moving
  it; audit exit 0 and deny exit 0 with advisory-db currency established first, which is what §Dependency
  Security requires. The plan's "16 ignore + 9 allow" line (`security-plan.md:182`) is a dated
  reconciliation stamp against `deny.toml`, not a count this chunk moved.
- **D-platform-claim** — no hit.

## design-system — `proposals: []`

No UI rendered; no token, palette, ANSI-map or reuse-tally value moved.

## layout-templates — `proposals: []`

- **D-layout-surface** — no hit. No user-facing surface or region; smoke independently re-derived as
  "no boot-path / UI-surface change".
- **D-layout-derived-count** — no hit, and swept anyway: `906`, `905`, `562`, `v2-`, `v3-`, `ledger`,
  `requirement_ids`, `matrix_ledger` over `layout-templates.md` → zero matches. The doc's own baked counts
  (the `82 capabilities …` roll-up at `:188`, the `step 82/82` sample at `:244`) are untouched baseline.
- **D-platform-claim** — no hit.

## test-plan — `proposals: []`

- **D-tests-coverage** — no hit. The new helper is unit ✓ (8 direct accept/reject assertions + 3
  fixture-driven cases), at the crate-local `tests/` tier the doc already mandates for this crate's
  artifact assertions (`test-plan.md:250`, `:234`). Determinism holds. The Deviations control is evidence
  for criterion 4, not a non-deterministic test.
- **D-tests-framework** — no hit. §4 (`:226`) names cargo-nextest as primary AND `cargo test -p <crate>` as
  the standing runner-portability gate — both of the chunk's invocations are the doc's own runners, and the
  Gate-3 four-target observation is that portability gate working as §4 describes. `rstest` was rejected,
  not replaced with something off-spec; plain `#[test]` is permitted at `:130` and `:235`.
- **D-tests-obs-harness** — no hit. Harness surface "none"; §3 ↔ obs-plan §3 untouched.
- **D-tests-derived-count** — no hit. No `matrix_ledger` token anywhere in the doc, and the two nearby
  id-space passages already name the SET rather than a literal (`:80`, `:363`) — the very passage this
  chunk's shape-predicate decision cites as its baseline.
- **D-platform-claim** — no hit. The doc's platform verdicts at `:469` and `:123` are untouched.

## obs-plan — `proposals: []`

- **D-obs-instrumentation** — no hit. `instrumentation n/a` matches the `cargo build / CI-CD pipeline →
  Not-instrumentable` row at `obs-plan.md:42`; criterion 13 measures 0 tracing/span tokens.
- **D-obs-stack** — no hit. No dependency moved; §3's tracing-JSON-only stack (`:187`, `:200`) unchallenged.
- **D-obs-redaction** — no hit. Both re-worded messages render `{label}` + a file name, no path, no struct
  name (`:604`, `:617`). Test-assertion panic text is not a log line, `runs.db` row or run-report artifact.
- **D-platform-claim** — no hit.

## a11y-plan — `proposals: []`

- **D-a11y-surface** — no hit. No interactive UI element; `a11y n/a`; the four must-be-accessible paths
  untouched.
- **D-a11y-obs-schema** — no hit. Schema/config "none"; the violation schema (`:228-248`, mirrored
  `:94-110`) and the obs §6 envelope both stand at baseline.
- **D-platform-claim** — no hit.
- **Deliberately NOT proposed, and correctly so:** the agent identified the `a11y-plan.md:115` dittography
  and declined it as out of its channel — it is a text duplication, not a claim any of its three invariants
  guards. That routing matches the operator directive: the defect is pinned as a CARRY on `v3-03` at P5
  (owned + scheduled), which `route-resolve.md` §Drift = 0 states explicitly is consistent with drift = 0.

---

## Validate (orchestrator)

| check | outcome |
|---|---|
| 1. Playbook | n/a — no proposals to classify |
| 2. Cross-contradiction | n/a — no proposals |
| 3. Intent-consistency | **aligned** — the report's outcome matches the working-route entry (generalise the id space; clear the red) and all 13 plan acceptance criteria are MET |
| 4. Absence needs evidence | **satisfied** — every absence claim in the report cites its search: the one-file class (bare per-path counts 1/0/0), the no-master-describes-this-gate claim (grep + hit count, the single hit read and identified), and `Counts moved: none — verified` (the `9[0-9]{2}` sweep over the seven masters) |
| 5. Expected-amendments reconciliation | **satisfied** — the plan's list is an explicit "none", re-verified at report time rather than inherited; no entry for the orchestrator to raise |
| 6. Disproved-claims disposition | **satisfied** — the report's `Spec claims disproved by measurement` is "none"; nothing to dispose |

**Result: 0 amendments applied · 0 escalations · no spec body edited · no sidecar written · drift = 0.**
No cascade ran (nothing changed to cascade from).
