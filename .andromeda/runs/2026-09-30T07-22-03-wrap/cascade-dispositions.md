# Cascade dispositions — 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (12 patterns; baseline the pre-CI parent `9785405b`), run
AFTER every body amendment of this pass (S1-S2, A1-A4, T1-T4, O1-O4) and the architecture registry trims. Patterns
cover each retired claim by its name AND its mechanism: the byte-equal literal pins (`byte-equal`, `pinned literal`,
`string literals`, `PINNED_CAPTURE`, `real_model_series`), the recorded breach and its pending route-owned remedy
(`recorded breach|2026-09-29 breach`, `owned by the new v3-09 series|and its route owner`), the three-stage scrub list
(`` `redact_value` + `mask_host_paths` + `elide_fingerprints` ``, `passes `redact_value`, then`), the pending key mask
and the basename premise (`masking it in the capture|workspace key however`, `workspace basename`), the moving tally
(`12 passing`). A 13th pattern (`never a hash|would need a dependency`) was DROPPED: its control never fired over the
masters — the claim lived only in a code doc comment, fixed in the chunk's own diff. Every kept pattern's control fired.
A separate by-hand read over CLAUDE.md, every `.claude/docs/*` and `.claude/rules/*` leaf for `blake3|sha2`,
`breach`, `workspace key|basename`, `Page/Frame|readiness`, `mask_host_paths` fed the leaf set below.

## Master rows (standing / new)
| row | disposition |
|---|---|
| security-plan.md:335 `byte-equal` / `string-literals` / `series-mod` / `breach`(new) / `scrub-3`(new) | amended (S2) — the words now describe the REMEDIED breach and the four-stage list |
| security-plan.md:121 `ws-basename` | amended (S1) — "never prints the workspace basename" is still true and kept; the new text adds the mask and witness |
| test-plan.md:364 `byte-equal` | no change — the coverage-matrix render's byte-equality, a different claim |
| architecture.md:113 `breach` | amended (A4) — the clause now reads remedied |
| architecture.md:60 `tally-12` @c4164 | no change — "12 passing … as measured at run 35208593666", a dated record true of its run |
| architecture.md:250 `tally-12` | no change — the same dated run record |
| a11y-plan.md:115 `tally-12` | no change — "It IS green as of 2026-09-17 — 12 passing … run 35208593666", dated |
| a11y-plan.md:517 `tally-12` ×2 | no change — @c603 the 2026-09-16 local narrative; @c2540 "as measured at run 35208593666" |
| `pinned-literal` · `PINNED_CAPTURE` · `scrub-then` · `masking-it` — 0 rows | the retired wording no longer stands in any master (control fired on the pre-pass text) |

## Leaf rows — re-derived from the amended masters
| leaf | disposition |
|---|---|
| `.claude/rules/security.md:10` (`byte-equal`, `series-mod`, `route-owner`, `scrub-3`) | re-derived — the exception's scrub list leads with `mask_workspace_key`; the BREACH line now reads remedied with the frozen-file residual |
| `.claude/rules/security.md:18` (`scrub-3`) | re-derived — the real-model ingest names the key mask |
| `.claude/docs/security-summary.md:11` (`scrub-3`) + `:24` (by-hand `breach`) | re-derived — four-stage list, key masked since 2026-09-30; the breach remedied |
| `.claude/docs/stack.md:22` (by-hand `blake3`) | re-derived — a `sha2 0.10` test-only line follows blake3 (arch §Stack amended, A1) |
| `.claude/docs/a11y-summary.md:11` (`tally-12`) | re-derived — the job's standing tally in the SET form (mirrors a11y-plan `:465`); a readiness anti-pattern line added under Universal anti-patterns (O1) |
| `.claude/docs/a11y-summary.md:14` (`tally-12`) | no change — "GREEN since 2026-09-17 … (12 passing …, run 35208593666)", the dated record a11y-plan `:115` keeps |
| `.claude/docs/tests-summary.md:46` (`tally-12`) | re-derived — SET form (mirrors test-plan `:56`) |
| `.claude/rules/a11y.md:38` (`tally-12`) | re-derived — SET form; a new Testing bullet carries the readiness wait and the probe-bound stall arm (O1) |
| `.claude/rules/host-win32.md:56` (`string-literals`) | no change — Rust string literals in a transport note, unrelated |
| CLAUDE.md `GENERATED:setup:*`, `docs/commands.md`, `docs/conventions.md`, `docs/gotchas.md`, `obs-summary.md`, `rules/testing.md`, `rules/verification-harness.md`, `rules/observability.md` | recomputed by reading against the amended sections — none states a moved fact (0 hits for every pattern above); `gotchas.md:23` already describes the key as the app's detected root, falling back to the data dir |

## Curation homes and judgment bases
- No sweep row in `USER:session-learnings`, any `## Session Additions`, `docs/session-learnings.md`, `playbook.md` or
  `drift-base.md`. By hand: `frontend.md:51` (Session Additions, 2026-09-01 BiDi `Page/Frame is not ready`) and
  `security.md:56` (Session Additions, 2026-09-29 breach rule) are both still true — not stale, not routed.
