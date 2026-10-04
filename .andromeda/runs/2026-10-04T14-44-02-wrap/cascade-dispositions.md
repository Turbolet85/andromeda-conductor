# Cascade dispositions — 2026-10-04-second-test-surface-corrective

**Search:** `cascade.py sweep --patterns-file cascade-patterns.toml` over the seven masters (pre-pass baseline `dab66dca`,
the pre-CI commit's parent), every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and
the leaf bodies. Six patterns, each derived from a claim this pass retires, by wording AND by assertion:
`nonblocking` · `ci-annotations` · `clippy-warn-soft` (clippy warnings being soft / annotations / "visible to agent") ·
`hardfail-only` (non-zero meaning only a hard `Fail`) · `psnative` (the caller-preference mechanism) · `caller-preempt`
(caller / invoking environment preempting native-command exits). Every pattern's known-positive control fired on the
pre-pass masters. `ci-annotations` and `clippy-warn-soft`: 0 rows after the pass (their only pre-pass sites, obs-plan
`:443` / `:490`, are the amended lines). Long lines were read by bounded window (`cascade.py window`), never by the grep
view. Sections read beyond the rows: obs-plan §1 `:42` (names clippy as a build-time step — no change), §9, §10;
test-plan §1 `:64`, the `5-command implementation` key file `:11-12` and `:15`.

## Rows (21 printed, every one dispositioned)

| row | pattern | disposition |
|---|---|---|
| `.andromeda/test-plan.md:432` @c45 | nonblocking | no change — "Mutation-survivor disposition (audit tier, non-blocking)": the mutation audit, a true claim sharing the token |
| `.claude/docs/session-learnings.md:366` | nonblocking | no change — curation home, "non-blocking UI" (a Tauri command returning immediately); unrelated |
| `.andromeda/test-plan.md:64` | hardfail-only | amended (T3 raise) — the hard-`Fail` clause stands and the bundled default's red build/doctest/clippy line is added beside it; re-read: no intra-line duplicate |
| `registries/contracts/test-plan/5-command-implementation.md:12` | hardfail-only | amended (T2 raise) — same form; re-read: no intra-line duplicate |
| `.claude/rules/testing.md:42` | hardfail-only | no change — its scope is the reported envelope STATES ("`blocked`/`ManualCheck`/… are reported envelope states, NOT non-zero exits — only a hard `Fail` exits non-zero"); among the states it stays true — a clippy red is not a report state |
| `.claude/rules/verification-harness.md:19` @c824 | hardfail-only | re-derived (leaf) — the `run` verb's own exit sentence: now adds the bundled default's first-non-zero-line stop in both shells, the measured 101, the ps1 per-line check, and the caller distinction |
| `.andromeda/obs-plan.md:490` | psnative | new — this pass's amended §10 line |
| `5-command-implementation.md:11` @c1314 | psnative | new — this pass's amended `run` body |
| `5-command-implementation.md:15` @c3455 | psnative · caller-preempt | no change — the `--e2e` capture-then-print caveat (a caller setting the preference preempts the printed verdict); still true, the `--e2e` arm is unchanged |
| `registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:6` | psnative · caller-preempt | no change — "a caller that preempts native-command exits diverges the `.ps1` leg with zero script delta": still true (under the preference a red line throws, step exit 1; without it the line's own cargo exit) |
| `.claude/docs/commands.md:30` @c1407/@c1448 | psnative · caller-preempt | no change — the same `--e2e` caveat, leaf restating it truly |
| `.andromeda/layout-templates.md:196` @c136 | caller-preempt | no change — the same divergence claim, true (the layout-templates doc-agent's refinement) |
| `.claude/rules/verification-harness.md:17` | caller-preempt | no change — the 5-command discipline's "a caller preempting native-command exits diverges the `.ps1` leg" (measured 2026-09-07), still true |

(The listing prints the per-pattern count lines twice, before and after the rows; every row above is one printed row,
`psnative` + `caller-preempt` co-hits on one line counted once per pattern.)

## Leaf re-derivation (step 3)

Changed sources: `obs-plan.md` (§9, §10) · `test-plan.md` (§1) · the test-plan key `5-command implementation`.
Leaves enumerated by the table AND by provenance header (`.claude/rules/{observability, verification-harness, testing}.md`
name the two plans): recomputed —
- `.claude/docs/tests-summary.md` §Harness contract (§3) — RE-DERIVED: the 5-command bullet states the bundled default's
  both-shells first-non-zero-line stop.
- `.claude/rules/verification-harness.md` `run` bullet — RE-DERIVED (row above).
- `.claude/rules/testing.md` — no change (states scope, row above).
- `.claude/docs/obs-summary.md` — no change: its §10 table carries the runtime SLO metrics only, never the build failure
  conditions; its §3 harness section carries no exit rule. `.claude/rules/observability.md` — no change (no clippy / CI
  failure-condition clause).
- `CLAUDE.md` `GENERATED:setup:warnings` — no change: no clause states clippy as non-blocking or the run exit rule.
- `.claude/docs/gotchas.md:54` (arch-derived; read because it states exits) — no change: "the five report states are
  distinct; only a hard `Fail` is a non-zero exit" scopes to report states, true.

Binds: test-plan §3 ↔ obs-plan §3 — unchanged on both sides (no envelope / status / log-format change); a11y ↔ obs
schema — untouched. Judgment bases (`playbook.md`, `drift-base.md`): 0 rows. Curation homes: 1 row, unrelated.
