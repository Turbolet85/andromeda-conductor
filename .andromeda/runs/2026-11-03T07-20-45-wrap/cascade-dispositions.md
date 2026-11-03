# Cascade dispositions — 2026-11-03-linux-only-base-ci (wrap 2026-11-03T07-20-45-wrap)

## The search
One call (cascade step 2), after every amendment of the pass landed (tx 57–64):
`cascade.py sweep --patterns-file` ×8 — the seven apply writers' `## Patterns` (`apply-architecture.md` arch-01..10,
`apply-design-system.md` ds-01, `apply-layout-templates.md` lt-01..03, `apply-obs-plan.md` obs-01..05,
`apply-security-plan.md` sec-01..08, `apply-test-plan.p1.md` tp1-01..17, `apply-test-plan.p2.md` tp2-01..14) and main's
`cascade-patterns.toml` (main-pulse-pin, the edits-main citation) — baseline `8e450f37` (the parent of the oldest
pre-CI commit `48b42a1f`). Every pattern's control fired on the pre-pass masters. Each pattern spans its claim's own
phrasings (the writers': the retired twin's spellings `agent-run.{sh,ps1}` / `both shells` / `.ps1` / `Test-RunId` /
`$LASTEXITCODE` / `PSNativeCommandUseErrorActionPreference`; `windows-latest` / `dev OS target`; `tokio 1.52.3`; the
capture locus `conductor-0.3.0/chunks/…/evidence`; `verification-matrix.json#v2-15`; `test-held`; `Windows / macOS /
Linux terminal`).

Counts, copied from the tool: `total (59 patterns) · 31 rows over 12 files` · `per class · new 0/0 · standing 2/2 ·
leaf 23/7 · curation 5/4 · base 1/1`.

## Rows
### standing (2) — no change
- `architecture.md:112` arch-04 fct-bjducqnw @c2366/2955 "`ReadyState` JSON in BOTH shipped shells" — no change: the dated
  2026-09-04 measured `boot` runs, a record of what was measured then (the report's Counts: no-change, history).
- `layout-templates.md:35` tp2-07 fct-g276wtgy "measured in the full shipped shell SET" — no change: the same dated
  2026-09-04 measurement, history (the report's Counts: no-change).

### leaf (23 rows over 7 files) — re-derived (step 3)
- `.claude/docs/stack.md:8` arch-01 · tp1-15 (tokio 1.52.3)
- `CLAUDE.md:16` arch-03 · `CLAUDE.md:36` lt-02
- `.claude/rules/security.md:16` arch-03 · sec-03 · `:27` arch-03 · `:40` sec-08
- `.claude/rules/verification-harness.md:17` arch-03 · tp2-14 · `:18` lt-02 · tp2-06 · `:19` lt-02 ×3 · obs-01 · tp2-02
  · tp2-10 · `:21` tp2-12
- `.claude/docs/commands.md:9` arch-03 · `:13` tp2-12
- `.claude/docs/conventions.md:9` arch-03
- `.claude/docs/tests-summary.md:10` arch-03 · `:13` lt-02 · tp2-02
Each leaf joins step 3's set; a row never stands in for the recompute.

### curation (5 rows over 4 files) — P3
Preserve-verbatim homes, never cascade-edited: routed to P3 as in-place extensions of the stale entry, or surfaced.
- `.claude/rules/frontend.md:16` arch-03 (`agent-run.{sh,ps1}`)
- `.claude/rules/security.md:54` arch-03 (Session Addition 2026-09-06, `scripts/agent-run.{sh,ps1}` expand
  `${CONDUCTOR_RUNS_DIR:-runs}`)
- `.claude/rules/testing.md:83` lt-02 ("the harness `status` verb in BOTH shells")
- `.claude/rules/verification-harness.md:58` lt-02 @c3475/5898 ("now MECHANIZED in both shells")
- `.claude/rules/verification-harness.md:49` tp2-05 @c3107/4025 ("identical `.sh`/`.ps1` semantics")

### base (1) — no proposal
- `.andromeda/playbook.md:139` lt-02 — a rule's `note:` naming the precedent it was minted on ("the `run --live` banner
  in both shells while arch's env registry carried only …"), a true record of that date; the rule's pattern and verdict
  do not depend on two shells. No propose→approve→append owed.

## Leaves
Set: `cascade.py leaves` (pass mode — Epoch 1 keeps 4 markerless entries, no epoch close) → `total · pass · 10 group(s)
· 15 leaves · 122546 B · unmapped 1`; every step-2 `leaf` row's file in a group, no `--add`. `unmapped
.claude/rules/host-linux.md (host)` — the host leaf, statements no master makes, no comparer. Batch `leaf` (10
comparers) → `check` 10 of 10 pass. Apply: `cascade.py leaves --apply` over nine files → `total · blocks 24 · applied 22
· preserved 2`.
- architecture · pass · 1 finding · applied 1 · by hand 0 · declined 0 (conventions.md:9).
- architecture+security-plan · pass · 2 findings · applied 2 (stack.md:8 tokio 1.54.1; stack.md:43 omission — the CI
  runner `ubuntu-latest`, applied: a reader writing a CI step would otherwise assume the retired pwsh runner) · by hand
  0 · declined 0.
- architecture+security-plan+design-system+layout-templates+test-plan+obs-plan · pass · 1 finding · applied 1
  (CLAUDE.md:16) · by hand 0 · declined 0. Step-2 row `CLAUDE.md:36` (lt-02, "in both shells") — no finding, no change:
  the GENERATED warning's dated 2026-09-04 measurement, history.
- architecture+design-system+layout-templates · pass · 0 findings (frontend.md, a retirement stub; its step-2 row :16 is
  a `## Session Additions` line — curation, P3).
- architecture+test-plan · pass · 3 findings · applied 2 (commands.md:13 ×2) · by hand 1 (commands.md:9, a heading the
  tool preserves — written by Edit as the finding's replace-with) · declined 0.
- security-plan · pass · 6 findings · applied 6 (security.md:16, :27, :23 ×2, :10, :40; the three omissions name the
  capture locus under the exception, the ban's scope and the no-leg flag value — each a fact a reader would otherwise
  act against) · by hand 0 · declined 0. security-summary.md: no finding.
- design-system+layout-templates · pass · 0 findings · its file's `read:` line names 35 lines for a 34-line leaf, so
  `leaves --apply` refused it (exit 3, "re-spawn"); `fanout.py render --respawn … --failed HEADER|RULED` refused the
  re-spawn (exit 2, "it failed none of …", "feeds only a judged check (none for leaf)") — the file holds no finding, so
  it was left out of the apply call and the leaf read by hand: `grep -n -i -E
  'windows|macos|platform|ps1|both shells|agent-run|pwsh' .claude/docs/design-summary.md` → exit 1, no line; the two
  source changes (Platform line, one harness) are stated nowhere in the leaf — 0 findings stand.
- test-plan · pass · 11 findings · applied 10 (tests-summary.md:10, :13; verification-harness.md:18 ×3, :19 ×3, :20,
  :21) · by hand 1 (verification-harness.md:17, a heading — written by Edit as the replace-with) · declined 0.
  testing.md: no finding (its step-2 row :83 is a `## Session Additions` line — curation, P3).
- test-plan+obs-plan · pass · 0 findings (observability.md).
- obs-plan · pass · 0 findings (obs-summary.md).
Binds: test-plan §3 ↔ obs-plan §3 / §6 consistent — the step-2 patterns swept every two-shell phrasing over all seven
masters and their key files; only the two dated-history `standing` rows remain.

## Notes
- The sweep's row labels name the masters by their pre-door paths (`architecture.md`, `layout-templates.md`, as the
  tool prints them, `cascade v1.21`): the store's renders, no file read or written — the masters stand at no path.
