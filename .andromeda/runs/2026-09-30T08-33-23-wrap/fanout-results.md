# Fan-out results — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

Seven Explore doc-agents, one batch. Returns carried no HTML entities in YAML values (none of `<` `>` `&` appear in the
proposals below); no raw twin warranted (no `proposals: []` return was changed by stripping beyond its `#` commentary, and
every return parsed).

## Verdicts
- a11y-plan — `proposals: []` (stripped: `#` notes — no interactive UI; no schema change; no platform verdict retired)
- obs-plan — `proposals: []` (stripped: `#` notes — operator stdout, stdlib only, `:72` path now repo-relative, no CI step)
- design-system — `proposals: []` (stripped: `#` notes — no UI; 0 hits for the moved values)
- security-plan — `proposals: []` (stripped: `#` notes — both new operator inputs validated; no dep; no sidecar spawn touched)
- layout-templates — `proposals: []` (stripped: `#` notes — operator instrument is not a product surface; 0 hits)
- architecture — 2 proposals (stripped: `#` notes — decisions/collision/registry-size/platform no hit)
- test-plan — 9 proposals (stripped: `#` notes — framework/obs-harness/platform no hit; a note that §12 edits touch existing entries only)

## architecture
- A1 · D-arch-resources · §Stack and Technologies → "Operator instruments (host runtime)" row (`:39`) — the mutation-tally
  gate's surface gains the `selftest [--fixtures DIR]` verb, the committed `scripts/fixtures/mutation-gate/` tree and the
  roster's required `tally` key; the no-CI-step / no-sixth-command sentence kept.
  **Disposition: apply** — check 1: playbook "Accurate this-chunk addition" (routine; the report's Changes carry all three);
  playbook :285 is for a NEW instrument and does not govern (its "NEW committed operator-local instrument" clause is false —
  the gate exists since 2026-09-13). Check 5: plan expected amendment (architecture §Stack row).
- A2 · D-arch-resources (dependent-of D-arch-resources) · §Infrastructure Patterns → Directory structure — `scripts/` tree
  gains `mutation-gate.py`, `mutation-roster.toml` and `fixtures/mutation-gate/`.
  **Disposition: apply** — check 1: "Accurate this-chunk addition"; check 5: plan expected amendment (Directory structure).
  Observed, not amended: the same tree also lacks `arch-registry-check.py`, `code-graph.py`, `scip_pb2.py`,
  `requirements.txt` and the cookbook/views files — pre-existing, not this chunk's Changes (playbook "Not this chunk's drift").

## test-plan
- T1 · D-tests-derived-count · §4 Mutation instrument (`:228`) — the gate grades every tally (presence, per-class counts vs
  `outcomes.json`, conservation, the `missed` and `timeout` multisets per the row's `tally` class; `unviable` counts only),
  proven by `selftest` over `scripts/fixtures/mutation-gate/` with no `cargo mutants` run.
  **Disposition: apply** — check 1 "Accurate this-chunk addition"; check 5 (§4 expected amendment).
- T2 · dependent-of D-tests-derived-count · §10 Mutation-survivor disposition (`:506`) — the roster parenthetical: the gate
  grades a run's tallies, not `missed.txt` alone. **Disposition: apply** (group of T1).
- T3 · dependent-of D-tests-derived-count · §12 `conductor-emit` member (`:619`) — the "never reads `timeout.txt` / no gate
  can hold them" claim retired; rows owed to the epoch-boundary code audit, the unit failing closed until then.
  **Disposition: apply** — check 5 (§12 expected amendment); owner per the report's Decisions and the operator's wrap
  directive. §12 existing entries are amended in place by precedent (10 prior sidecar entries name §12); no new log entry.
- T4 · dependent-of D-tests-derived-count · §12 Roster member IDENTITY (`:621`) — the multiset match covers `missed.txt` or
  `timeout.txt` per the row's `tally`; a zero-row unit passes when every graded tally agrees.
  **Disposition: apply** — check 5 (§12's twin; the report names `:621`).
- T5 · D-tests-derived-count · §7 committed fixture family (`:403`) — the SET of committed fixture trees; gains
  `scripts/fixtures/mutation-gate/`, the first member outside a crate-local `tests/fixtures/`.
  **Disposition: apply** — check 5 (§7 site 1).
- T6 · dependent-of D-tests-derived-count · §7 fixture meaning (`:404`) — the round-trip SET gains the Python `selftest` pin.
  **Disposition: apply** — check 5 (§7 site 2).
- T7 · D-tests-coverage · §9 Scoped mutation audit (`:469`) — "runs per-chunk" → at the epoch-boundary code audit only,
  never per chunk (founder ruling 2026-09-30).
  **Disposition: apply** — check 1: "Accurate this-chunk addition" excludes a reversal, so NO MATCH; the operator's RECORDED
  direction settles it (wrap directive: "test-plan section 9 still says mutation runs per chunk, which the founder ruling
  … reverses"; the founder ruling itself in plan Metadata and the report's Decisions) → apply, and propose the rule at the
  wrap card. Check 5 (§9 expected amendment).
- T8 · dependent-of D-tests-coverage · §10 Mutation-survivor disposition (`:506`) — "where a chunk runs" → "where the
  epoch-boundary code audit runs". **Disposition: apply** (group of T7).
- T9 · dependent-of D-tests-coverage · §12 Roster member IDENTITY (`:621`) — "gain one when a chunk that TOUCHES their crate
  re-runs the tier" → when the epoch-boundary code audit re-runs it. **Disposition: apply** (group of T7).

## Validate summary
- Check 2 cross-contradiction: T2 and T8 both edit `:506`, T4 and T9 both edit `:621`, in compatible directions (different
  clauses) — no conflict.
- Check 3 intent-consistency: the report matches the working-route entry + plan acceptance; scope record empty (gate.py scope
  clean). No escalation.
- Check 4 absence: the five `[]` returns each cite a 0-hit grep of their own master for the moved tokens; the report's own
  site census (`grep -n 'mutation-gate\|mutation-roster' .andromeda/*.md` → test-plan 5, architecture 1, others 0) agrees.
- Check 5 expected amendments: all six covered (§4 T1 · §9 T7 · §12 T3+T4 · §7 T5+T6 · arch §Stack A1 · arch tree A2).
- Check 6 disproved claims: the report lists none.
- Escalations: 0. Applies: 11. Rejects: 0.
