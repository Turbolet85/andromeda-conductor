# Materialization plan — setup-project re-run, upgrade form

**Run:** `2026-10-10T08-56-03-setup-project` · **Project:** Conductor · **Development Style:** agent-driven
(`architecture.md:220`) · **Stack:** rust (`architecture.md:13`, the language row) · **Host:** linux
**Form:** re-run — upgrade form. No upstream body read. No structural change named at invocation (the command
arrived bare, no words after it).

## Tier 1 — CLAUDE.md
- `overview` · `modules` · `warnings` · `pointer-table` · `workflow` · `architecture`: not re-derived —
  cascade-maintained, each standing byte for byte (`warnings`: 11 bullets).
- `imports`: the template's (U01 `ok`) — no Edit.
- `pointer-table`: U48 `ok` (4 of 4 indexes named) — no Edit.
- `deeper-topics`: recomputed from the files that exist — the docs list (5 specialist summaries, 5 core,
  `session-learnings.md`) and the rules list (`security` · `testing` · `observability` · `a11y` · `frontend` ·
  `verification-harness` · `host-linux`) both match the tree — no Edit.
- CLAUDE.md is NOT written (138 lines, md5 `f59925f427792c232a31b17bdf61c198`). Backed up per Setup 5.

## Tier 2 — .claude/rules/
| file | disposition |
|---|---|
| `a11y.md` | preserve |
| `frontend.md` | preserve |
| `observability.md` | preserve |
| `security.md` | preserve |
| `testing.md` | preserve |
| `verification-harness.md` | preserve |
| `host-linux.md` | preserve — this host's leaf, U04 `ok` |

Absent: none.

## Tier 3 — .claude/docs/
`stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md` · `security-summary.md` ·
`design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md` · `session-learnings.md` — each
`preserve`. Absent: none. No services leaves.

## Agent harness
`scripts/agent-run.sh` · `scripts/agent-run.ps1` · `.claude/rules/verification-harness.md` — present, each
`preserve`. No fresh render made, nothing compared (upgrade form). Both scripts carry the five verbs
(`boot` · `run` · `status` · `cleanup` · `logs`: one arm each in each script).

## Code reviewer
`.claude/agents/code-reviewer.md` — present, `preserve` (rust).

## Hooks — .claude/settings.json
U02 `ok` (`write current · bash current · PostToolUse on the stdin prologue`) — no entry replaced, the file is not
written and not backed up. `env` block present. Formatter config: `rustfmt.toml` present (U05 `ok`).

## Gitignore · .gitattributes
U07 `ok` · U06 `ok` — nothing written.

## Code-graph pipeline
Every seeded file present (`code-graph.py` · `code-graph-views.sql` · `scip_pb2.py` · `requirements.txt` ·
`code-graph-cookbook.md`) — nothing seeded.

**U03 `behind` — the drift rule (P6).** Measured with the tool's own region rule (`health.template_region` +
`health.behind`, read-only):

| file | lines behind | live | template |
|---|---|---|---|
| `scripts/code-graph.py` | **27** | 387 lines | 407 lines |
| `scripts/code-graph-views.sql` | 0 | 64 | 64 |
| `scripts/code-graph-cookbook.md` | 0 | 107 | 107 |

The `code-graph.py` difference, four hunks, all on the indexer call:
1. The rust plane's spec gains `"config": {"cargo": {"features": "all"}}` and its `argv` lambda a fifth parameter
   `cfg`, appending `--config-path {cfg}` when one is given.
2. The ts plane's `argv` lambda takes the same fifth parameter (unused).
3. The refresh writes a transient `indexer.json` beside the SCIP dump, runs the indexer with it, falls back to a
   default-features index if the all-features one fails (the plane line then carries
   ` (default features - the all-features index failed)`), and removes the transient.
4. The `tree-refresh[{plane}]` print and the returned plane line carry that note.

Every `-` line of the diff is the prior form of a line the template replaces: the live file holds no
project-evolved line the fresh copy would lose (the 2026-10-09 run read the same file `0` behind the then-installed
template, so it IS the prior template).

What it changes for this project: two members declare Cargo features — `conductor-run` (`live-pulse`) and
`conductor-verify` (`stub-server`). Under the present script the targets behind them are modules with no symbols in
`tree.db`; under the fresh copy they are indexed. A "0 callers" reading over a symbol behind either feature is
therefore not comparable across the two forms.

Disposition: **proposed**, the operator's to accept at the card. Present file backed up to
`.claude/backup/code-graph.py.pre-setup-2026-10-10T08-56-03` (md5 `9f982a7fed402835edfaa32d85a5ca3d`, gitignored).
On acceptance: `scripts/code-graph.py` is written whole from the template's fenced body, LF. The views file is not
replaced, so no `built.views` rebuild is forced; the DB rebuilds at the next refresh (`tree.db.commit` holds
`9e8b30f`, already behind HEAD). Setup does not build the DB.

## Session / operational seeds
`state.yaml` · `session-handoff.md` · `drift-base.md` · `playbook.md` · `session-learnings.md` — present; nothing
seeded. `.andromeda/cache/` present.

## Upgrade
Setup 5b's record: HEAD `7a2b8e5e47b3ac57787489fe624e20a3f283509f` · branch `build/conductor-0.4.0` · path set:
` M .claude/session-handoff.md` (expected-transient bookkeeping), nothing else.
`route.py cursor` (`route v1.7 · f7323866`): `records 165 · complete 165 · pending 0 · gated 0` ·
`half-promote 0 of 0 stamped lines vs 165 master records`.

`upgrade.py detect --root .` (read-only, no `--run-dir`), verbatim:

```
upgrade v1.8 · ed684deb
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · behind · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph…
U04 · ok · setup · .claude/rules/host-{os}.md · every template line present above `## Session Additions`
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 61 markerless entries · 0 introducers behind markup or after t…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
U48 · ok · setup · CLAUDE.md pointer rows · 4 of 4 indexes named
upgrade: for setup 1 (U03) · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0 · 17 detectors of 46 registry entries
```

The tool's stamp is `upgrade v1.8` (the registry's §Tool). The install's digest moved since the 2026-10-09 run
(`30b07c54` → `ed684deb`, 45 → 46 registry entries) — the fresh template is what turned U03 from `ok` to `behind`.

Acting row: **U03** (P6, the drift rule — a proposal). No other setup-class row is `behind`; none is
`INDETERMINATE`; no entry awaits a door.
Noted, not written: U09 (`workflow.md` lacks `it never commits`, grep 0) · U10 (`agent-run.sh` lacks
`ensure_fresh_artifacts`, grep 0) · U36 (the five summary leaves each lack `cascade re-derives it`, grep 0 in
each). Each is a preserved leaf behind its template; `regenerate {leaf}` stays the operator's word.

## Phase steps that write nothing
P1 (no Edit) · P2 (no absent rule file; host leaf current) · P3 (no absent leaf) · P4 (scripts present, validated) ·
P5 (reviewer, hooks, `.gitignore`, `.gitattributes` all current) · P6 seeds (all present).
