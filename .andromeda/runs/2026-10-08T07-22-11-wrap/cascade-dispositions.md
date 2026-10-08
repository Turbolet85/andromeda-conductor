# Cascade dispositions — 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09

Written from the listing of `cascade.py sweep` (cascade v1.1; baseline `902d12c8`, the parent of the pre-CI commit),
run AFTER every body of this pass was applied and BEFORE any sidecar entry landed. The pattern set is
`cascade-patterns.toml` (10 patterns, each control fired on the pre-pass masters); the rows are in the trail
`cascade-…json`.

## What was searched
- The retired standing sentence of `architecture.md:70`: `unverified` · `not met` · `v3-09`.
- The replaced latest pin of `architecture.md:184`: `f70be92`.
- The retired unmeasured-pickup clauses (`architecture.md:184`, `test-plan.md:391`): `UNMEASURED` (case-insensitive)
  · `pickup` · `real-model formation|formation figure|formation AND`.
- The retired canary-line sentence of `obs-plan.md:221`: `prints two|two \`canary:\`|two canary` · `tick falls
  after|emission instant` · `` `canary:` line ``.
- Scope of the listing: the seven masters and every `.andromeda/registries/**` file, the three curation homes, the two
  judgment bases, the leaf bodies. NOT searched: the posture contract, the plan and scope of any chunk, sidecars,
  archives, run dirs (none is a cascade subject).

## Rows, by pattern
| Row | Disposition |
|---|---|
| `unverified` · `.claude/rules/frontend.md:51` · curation | no change — a different claim (a static gate does not verify an unexecuted path) |
| `unverified` · `.claude/rules/verification-harness.md:47` @c2889 · curation | STALE — the 2026-09-29 correction tag ends "the interpretation claim stays unverified"; read by offset (chars 2689-3089). A preserve-verbatim home: routed to P3 as a correction in place |
| `unverified` · `.claude/docs/session-learnings.md:238` · curation | no change — about an agent's remit |
| `not met` · `test-plan.md:81` · standing | no change — the control-panel requirement "stands OWED, not met" |
| `not met` · `test-plan.md:260` ×6 edited (@c3131, 3369, 3728, 4853, 5393, 6368) | no change — each is the dated verdict of its own earlier series or of the capture run ("stays not met on the 2026-10-07 series' record"), true of that record; the sixth series' record follows them on the same line |
| `not met` · `playbook.md:163` · base | no change — "the usual recurrence bar is not met" |
| `not met` · `.claude/docs/tests-summary.md:22` · leaf | STALE ("`v3-09` not met by any series") — re-derived from test-plan §6: the earlier series' standing scoped to those series, the sixth series' record added |
| `v3-09` · `architecture.md:70` edited | amended — this pass's new sentence |
| `v3-09` · `test-plan.md:260` ×10 edited | the dated records above, the new record's two test names and the two chunk markers; no further change |
| `v3-09` · `obs-plan.md:221` ×3 edited | chunk markers inside evidence pointers (the fifth series', the capture run's, the sixth's); states no standing |
| `v3-09` · `playbook.md:318` · base | no change — the rule's own exclusion clause names the verdict statement; it quotes no retired wording |
| `v3-09` · `.claude/docs/tests-summary.md:22` · leaf | re-derived (the row above) |
| `f70be92` · `security-plan.md:121` edited | no change beyond this pass's own edit — the derivation's pin list keeps `f70be92` and gains `9bfefb8` |
| `f70be92` · `test-plan.md:260` ×2 edited | the 2026-10-07 series' dated record, and this pass's sentence naming the capture run's build; both intended |
| `f70be92` · `obs-plan.md:221` ×2 edited | dated records of the 2026-10-07 series and the capture run; stand |
| `f70be92` · `.claude/docs/tests-summary.md:22` · leaf | the 2026-10-07 series' dated record in the leaf; stands (the line was re-derived) |
| `unmeasured` · `test-plan.md:231`, `:387` · `obs-plan.md:490` · `a11y-plan.md:424` · `registries/contracts/a11y-plan/screen-reader-test-pattern.md:3` · standing | no change — other subjects (a WebView2 patch cause, a Linux `xvfb` target state, a `.ps1` red path, a screen-reader defect) |
| `unmeasured` · `CLAUDE.md:135` · `.claude/rules/testing.md:90` · `.claude/rules/verification-harness.md:65` · `.claude/docs/session-learnings.md:1052` · curation | no change — other subjects |
| `unmeasured` + `formation` · `.claude/rules/verification-harness.md:60` @c1312 · curation | STALE — "real-model formation is UNMEASURED … so whether it lands inside the window is unknown"; read by offset (chars 1112-1512). A preserve-verbatim home: routed to P3 as a correction in place |
| `unmeasured` · `playbook.md:142`, `:149`, `:168` · base | no change — rule text about other measurements |
| `unmeasured` · `.claude/rules/verification-harness.md:19` · `.claude/docs/tests-summary.md:13` · leaf | no change — the `.ps1` red path, another subject |
| `pickup` · `architecture.md:184` edited | amended — "no real-model formation or pickup budget exists" |
| `pickup` + `formation` · `test-plan.md:260` @c2966 edited | no change — the 2026-09-23 drive's dated record ("neither a pickup nor a formation figure was measured"), true of that drive |
| `pickup` ×2 + `formation` · `test-plan.md:391` edited | amended — this pass's wording |
| `two-lines` | 0 rows; the control fired at `obs-plan.md:221` on the pre-pass text, so the retired wording stands nowhere in the swept set |
| `tick-after` · `architecture.md:62`, `:64`, `:93` · standing | no change — the preflight canary's freshness comparison (an incident opened AFTER the emission stamp), a different mechanism |
| `tick-after` ×2 · `obs-plan.md:221` edited | amended — this pass's new sentence |
| `` `canary:` line `` · `security-plan.md:121` ×2 edited | no change — the row says the capture prints the `canary:` lines and each line's trailing `skip_reason`; it states no count |
| `` `canary:` line `` · `test-plan.md:260` ×2 edited | the `skip_reason` clause (no count) and this pass's sentence on the third line of d1 and d2 |
| `` `canary:` line `` · `obs-plan.md:221` edited | amended |
| `` `canary:` line `` · `.claude/docs/tests-summary.md:22` · leaf | no change in that clause — "each `canary:` line carries Pulse's `skip_reason`", no count (the line was re-derived for the standing) |

Every amended line was re-read for an intra-line duplicate of its retired wording: none stands (`stays unverified`,
`pickup figure`, `both still UNMEASURED`, `prints two` each read 0 on its line after the apply).

## Leaves
- Re-derived: `.claude/docs/tests-summary.md:22` (from test-plan §6, the real-model leg's dated records).
- Read for a restatement and left: `.claude/docs/security-summary.md:11` ("a series' drive, or the 2026-10-07 capture
  run's" — names no series set), `.claude/docs/commands.md:12` (the selector's rule, no verdict), and — 0 hits for
  `interpretation claim|trustworthy|re-pinned|pickup|formation` — `CLAUDE.md`, `obs-summary.md`, `stack.md`,
  `conventions.md`, `gotchas.md`, `a11y-summary.md`, `design-summary.md`.
- The citation sweep's re-points (`test-plan.md:426`, digits only) change no claim and re-derive no leaf.

## Curation homes (never edited by the cascade)
Two rows go to P3 as corrections: `.claude/rules/verification-harness.md:47` (the 2026-09-29 tag's "the
interpretation claim stays unverified") and `:60` ("real-model formation is UNMEASURED").

## Judgment bases
No row quotes retired wording; nothing is proposed for `playbook.md` or `drift-base.md`.

## Tooling detector
`D-arch-registry-size`, run after the apply: §Established Decisions 38082 B, §Occupied Resources 38114 B, threshold
38115 B — `registries: within target`.
