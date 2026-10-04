# Cascade dispositions — 2026-10-04-real-model-test-surface-corrective

**The search.** `cascade.py sweep` (v1.1) over `cascade-patterns.toml` in this run dir, written after the pass's LAST
body amendment (T1 keyed contract `5-command-implementation.md:11`; T2 `test-plan.md:382`; T3 `test-plan.md:391`; T4
`test-plan.md:291`; R2 `test-plan.md:149`; R1 `security-plan.md:262`; O2-raise `obs-plan.md:447`). Baseline `1208ca55`
(the pre-CI commit's parent). Eight patterns, every control fired on the pre-pass masters:
- `default-clippy` (fixed `default \`nextest\` / \`clippy\``) and `default-lint` (fixed `default lint pass`) — the T3 retired
  wording: 0 rows after the pass (control `test-plan.md:391`) — the retired phrasing survives nowhere.
- `owes-lint` (the owed-lint mechanism, however worded) · `ws-clippy` (`clippy --workspace --all-targets` — any site
  enumerating the bundled run) · `feat-sets` (`stub-server|--features live-pulse`) — T1/T2/T3.
- `secret-scan` (`secret_scan_gate|[Ss]ecret-scan gate`) · `empty-subject` (`empty subject|has no subject|lists no
  tracked files`) — T4/R1/O2.
- `mutants-copy` (`copy-vcs|copy mode|unmutated (tree|baseline)`) — R2.
Long-line hits read by offset (`cascade.py window`): security-plan:370 @9916 · ci-cd-approach.md:1 @705 ·
tests-summary.md:46 @2959 · security.md:27 @2629 · test-plan:391 @4566.

**Every row** (31):
- `test-plan.md:391` owes-lint ×2 (edited) — @4566 the span_landing target's "own lint line" (true: that line now runs in
  the bundle) · @5226 "such a leg OWES its own lint command" (kept by design; the amended sentence follows it). no change.
- `test-plan.md:382` ws-clippy (edited) — amended (T2).
- `5-command-implementation.md:11` ws-clippy (edited) — amended (T1).
- `CLAUDE.md:80` ws-clippy leaf — the Key commands list names the workspace lint command; a true claim, not an
  enumeration of the bundled run. no change.
- `.claude/rules/verification-harness.md:19` ws-clippy leaf — the `run` body enumeration → **re-derived** (the two
  feature lines added).
- `.claude/docs/commands.md:33` ws-clippy leaf — same as CLAUDE.md:80, a true command line. no change.
- `test-plan.md:382` feat-sets ×2 new — amended (T2). `test-plan.md:391` feat-sets ×4 (edited) — amended (T3).
  `5-command-implementation.md:11` feat-sets ×3 new — amended (T1).
- `.claude/rules/testing.md:52` feat-sets curation — the 2026-06-21 entry as extended 2026-10-03 ("a feature-gated test
  target that no CI job or harness verb builds rots silently…"): the rule stays TRUE (now satisfied for both feature
  sets); a curation home, never a cascade edit → routed to P3 (an in-place extension naming the harness verb that now
  builds them).
- `.claude/rules/verification-harness.md:18` feat-sets leaf — `boot`'s CI stub behind `stub-server` (true). no change.
- `.claude/docs/tests-summary.md:46` feat-sets leaf — "owes its own clippy leg" → **re-derived** (the line now runs in
  the bundle; never run).
- `architecture.md:60` secret-scan ×2 — the CI gate list and subject wording; true. no change.
- `security-plan.md:225` ×3 (REALIZED record) · `:260` ×2 (Secret scanning in CI) — true inside a repository / in CI's
  checkout. no change.
- `security-plan.md:262` ×3 (edited) — amended (R1).
- `security-plan.md:370` @9916 — the test-binary spawn record (fixed argv, no operator value): still the one spawn form;
  the skip adds no spawn. no change.
- `test-plan.md:149` secret-scan new — amended (R2). `test-plan.md:173` — names the gate file; true. no change.
- `test-plan.md:291` ×3 (edited) — amended (T4). `obs-plan.md:447` ×3 (edited) — amended (O2-raise).
- `obs-plan.md:492` — the §10 failure condition (a secret-shaped hit) is unchanged by a skip that is not a failure.
  no change.
- `registries/contracts/architecture/ci-cd-approach.md:1` @705 — the gate's subject wording; true. no change.
- `.claude/rules/security.md:8` leaf ×2 — the gate summary → **re-derived** (the skip clause added).
- `.claude/rules/security.md:27` leaf @2629 — the test-binary spawn record; true. no change.
- `.claude/docs/security-summary.md:46` leaf ×3 — the gate summary → **re-derived** (the skip clause added).
- `security-plan.md:262` empty-subject (edited) — the CI presence guard's "never passes over an empty subject" stays
  true (CI always has a checkout); amended beside it (R1).
- `test-plan.md:291` empty-subject new — amended (T4).
- `registries/contracts/a11y-plan/ci-integration.md:4` empty-subject — a HOLD dialog "has no subject": a true claim
  sharing the token. no change.
- `.claude/rules/testing.md:75` · `.claude/rules/verification-harness.md:63` empty-subject curation — unrelated
  (a webview lamp subject; an SR record's `empty` subject). no change.
- `test-plan.md:149` mutants-copy ×3 (edited) — @461 "unmutated tree" (the `--test-tool=nextest` discipline, true);
  @3435/@3587 the R2 addition. amended (R2).

**Leaves by provenance** (beyond the rows): test-plan → `tests-summary.md` (re-derived above), `rules/testing.md` body
(its Mutation bullet carries no copy-mode clause; the R2 addition is a §4 detail it does not distil — no change),
`rules/verification-harness.md` (re-derived above), CLAUDE.md `GENERATED:setup:warnings` (no harness enumeration —
no change). security-plan → `security-summary.md`, `rules/security.md` (re-derived above). obs-plan → `obs-summary.md`,
`rules/observability.md`: neither restates the §9 Repository-hygiene row (`grep -i 'hygiene|secret-scan'`: 0) — no change.

**Not looked for:** the `copy-vcs` token in non-master files beyond the leaves above (no master stated it pre-pass);
the obs §10 clippy "non-blocking" wording (O1 rejected — routed to a CARRY, not a cascade edit).
