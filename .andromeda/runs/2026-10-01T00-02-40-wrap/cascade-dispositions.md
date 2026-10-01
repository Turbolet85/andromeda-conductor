# Cascade dispositions — 2026-09-30-full-gate-regression-over-the-moved-surfaces

The pass's amendments: test-plan §3 `run` (`:151`) and Test selection (`:154`); security-plan §Input Validation CLI
arguments row (`:123`). One retired claim, C3's: a P-ID target resolves to the FIRST scenario naming it in directory
order. The pattern set (`cascade-patterns.toml`, written after the last body edit) keys the claim's wording AND its
mechanism verbs: `first scenario naming` · `directory order` · `determinate(ly)? ( run| only where)` · `find_by_pid` ·
`making \`run\` do the same` · `(take|takes|picks?|resolves? to) the first` (case-insensitive). Every pattern's
known-positive control fired on the pre-pass `test-plan.md:151`.

## First sweep (after the three body edits, before the leaves)
Masters: `new 0 · standing 0` on every pattern — `test-plan.md:151` / `:154` and `security-plan.md:123` carry no
retired wording. Curation homes 0, judgment bases 0. Leaf rows (7 rows, 4 sites):
- `.claude/rules/verification-harness.md:19` (first-naming · dir-order · take-first) — stale, the test-plan §3 `run`
  leaf → **re-derived**: an ambiguous P-ID is refused before any load, naming them, exit 1.
- `.claude/rules/testing.md:38` (determinate) — stale, the test-plan Running leaf → **re-derived**.
- `.claude/docs/commands.md:12` (first-naming · dir-order · take-first) — stale, the harness `SCENARIO=` line; a site
  the plan's expected-amendment list did not name → **re-derived**.
- `.claude/docs/commands.md:18` (dir-order · take-first) — stale, the `conductor run` line → **re-derived**.

## Second sweep (after the leaves)
0 rows on every pattern, every control fired.

## Leaves recomputed by provenance with no change
- `.claude/docs/tests-summary.md` — carries no P-ID-resolution statement (`:41` "no scenario without a P-ID" stands).
- `.claude/docs/security-summary.md` · `.claude/rules/security.md` — neither restates the CLI-arguments row (the
  `preconditions --for` mention at `security.md:17` concerns the L4 handle's real-model posture, unchanged).
- CLAUDE.md `GENERATED:setup:warnings` — states no P-ID-resolution claim.

## Seen and left (true claims sharing a token)
- `architecture.md:244`, `test-plan.md:39`, `:71`, `:128`, `:201`, `:396`, `layout-templates.md:188` — "a P-ID may be
  named by several" — still true after C3 (the census names seven such P-IDs).
- `test-plan.md:124` "a 50 ms utterance-to-stamp window" — still true: the parser is unchanged; the SR stamp fix records
  a declared partner at its first row's instant (curation's channel, P3).

## Not looked for here
- The knip / rustdoc / E0-10 tallies — no master states them (report Counts bullet, grep basis); the one leaf stating the
  knip residual (`frontend.md:54`) is a `## Session Additions` entry → P3 curation, never a cascade edit.
