# Fan-out results — 2026-08-20-read-back-seam-survivors-closed

7 Explore doc-agents, one batch. **6 clean · 1 doc with proposals (2, a `dependent-of` group).**
Raw twin kept only for `test-plan` (the one return carrying proposals): `.raw-fanout-test-plan.md`.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | `proposals: []` |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | `proposals: []` |
| design-system | D-design-tokens · D-design-derived-count | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count | `proposals: []` |
| **test-plan** | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count | **2 proposals** (D-tests-framework primary + its `dependent-of`) |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` |

## Absence evidence (validate check 4)

- **arch** — no new IPC/endpoint/event/socket/port/env-var/workspace-crate; the two additions are test
  *binaries* inside the existing `conductor-verify` crate, not workspace members, so §Occupied Resources
  "Crate names (workspace members)" stays accurate. `assert_fs` is already named in §Infrastructure Patterns
  → Build system as part of the pinned dev-test stack, so it is not a new library.
- **security-plan** — Coverage bullet states no new external surface; `Schema / config: none`; no `src/`
  change, `serve_stub`'s signature unchanged, both new `StubConfig` fields default `None`, so the fixed-name
  spawn / `.env(...)` data-dir / `2024-11-05` negotiate-down are all unperturbed. §Dependency Security's
  admitting-a-dependency-under-a-red-audit rule satisfied on its own terms (deny true exit 0 over the POST-
  change lock, zero `[[package]]` nodes).
- **design-system** — grepped for each OLD literal (669, 92, 33, 5, 51, 38, 11, 2, `assert_fs`, `nextest`,
  `mutant`, `survivor`) across palette rows, the `:root` token block, the ANSI map and reuse tallies: zero
  hits. Noted the coincidental `11 of 34` token count is a design value, unrelated to `11 unviable` (which
  did not move).
- **layout-templates** — grepped the same OLD literals plus the tooling vocabulary: zero hits. The doc's
  caption literals (`82 loaded`, `seed 424242`, the `77 Pass · 1 Calib` roll-up, the 6-/4-column shapes) are
  on a different axis and none was moved by this chunk.
- **obs-plan** — returned the sanctioned bare `proposals: []`. Independently consistent with the report: no
  new must-trace operation, no OTel SDK (`assert_fs` is a temp-dir crate), and the witness READS the existing
  post-redaction self-obs stream rather than writing a new artifact.
- **a11y-plan** — `Schema / config: none`; the one artifact-touching test adds no span, field, or SDK, so the
  violation schema and the obs §6 envelope both stand, including their two verbatim reproductions.

## Considered and deliberately NOT proposed

`test-plan` names cargo-nextest **0.9.137** at six sites while the report's Dev-tool bullet reads
**0.9.133**. Correctly not proposed: the bullet is `none` (nothing was installed or upgraded this chunk), and
§4's Tool-version policy already governs it — the named versions are reference floors, not exact pins, so any
install running the gate green satisfies it. Substituting a fresh literal would only re-stale. Matches the
standing playbook rule (2026-06-15) for a dev CLI tool running green a patch behind the named version.

## Validation outcome (main)

1. **Playbook check** — no rule covers "a chunk measures a tool's semantics and the plan records it"; the
   change is neither structural nor surprising (operator-directed, report-substantiated, same class as the
   prior wrap's registrations), so: **routine, applied**. The `dependent-of` applies atomically with it.
2. **Cross-contradiction** — none; a single doc, primary + dependent, same direction.
3. **Intent-consistency** — report matches the working-route entry + plan acceptance. No divergence.
4. **Absence needs evidence** — satisfied above; the three derived-count detectors each cited their grep.
5. **Expected-amendments reconciliation** — the plan's list said "none anticipated" plus one CONDITIONAL
   (a package admitted ⇒ arch §Stack row + `deny.toml` justification). The conditional did **not** fire —
   the lock delta admitted zero packages — so nothing under-ran. The report had pre-declared the test-plan §4
   edit as EXPECTED, and the detector proposed exactly it.
6. **Disproved-claims disposition** — both report entries disposed: (a) the plan's timeout-conversion
   speculation measured FALSE — a PLAN speculation, not a spec-master claim, so no master owes an edit;
   recorded plainly in the report and operator-ratified as a non-event that cannot fail the chunk;
   (b) the CARRY's correlation premise — already resolved at phase P4 (keep-the-goal-change-the-mechanism),
   recorded in `scope.md` and the friction ledger.

## Escalation (1) — raised, resolved with the operator

**`playbook.md`'s dependency-under-red-audit rule contradicted the route PREREQ's own signature.** The rule's
form-clause triggered on "a real `Cargo.lock` delta" ⇒ pin returns to FULL form; the PREREQ (authored later)
narrows it to "a dependency delta that **ADMITS a package**". This chunk landed the gap: an edge-only delta,
zero packages admitted, operator-ratified compact record. Left standing, the next identical case would
re-litigate — possibly the other way.
**Resolution (operator, "Sharpen the rule to match"):** the form now keys on package admission, not on
whether the lock moved; the compact form stands for an edge-only delta **provided** the deny-green
verification was actually run over the new lock. The rule's substantive control is untouched and was
satisfied here (true exit 0). Appended via the sanctioned propose→approve→append channel — `playbook.md` sits
outside the re-derivation DAG and is never cascade-edited.

## Applied (3 bodies + 2 sidecar entries + 2 cascade leaves)

- `test-plan.md` §4 Mutation instrument — the exit-code-carries-no-verdict clause (primary).
- `test-plan.md` §12 Decisions Log — run-discipline bullet extended to both directions (`dependent-of`).
- `playbook.md` — dependency-under-red-audit form-clause sharpened (resolved escalation).
- `test-plan-amendments.md` — 2 entries, appended after re-reading the edited bodies.
- **Cascade:** `.claude/rules/testing.md` Mutation bullet + `.claude/docs/tests-summary.md` Mutation
  instrument bullet re-derived from the amended §4. Cross-master sweep for the amended wording found no
  citation in any other master, nor in the three preserve-verbatim curation homes.
