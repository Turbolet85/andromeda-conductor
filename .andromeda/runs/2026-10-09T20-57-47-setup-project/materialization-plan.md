# Materialization plan — setup-project re-run, upgrade form

**Run:** `2026-10-09T20-57-47-setup-project` · **Project:** Conductor · **Development Style:** agent-driven
(`architecture.md:220`) · **Stack:** rust (`architecture.md:13`, the language row) · **Host:** linux
**Form:** re-run — upgrade form. No upstream body read. No structural change named at invocation.

**Operator's words at invocation:** a re-run between chunks, for registry U02 (the two guard entries in
`.claude/settings.json` swapped for the installed hook calls) and U49; on the card, `regenerate host-linux.md`; the
card shown before anything is written; nothing written in tracked files beyond what those two entries name; stop
before the commit. The card was shown from read-only calls; the operator answered `yes`.

## Tier 1 — CLAUDE.md
- `overview` · `modules` · `warnings` · `pointer-table` · `workflow` · `architecture`: not re-derived —
  cascade-maintained, each standing byte for byte.
- `imports`: the template's (U01 `ok`) — no Edit.
- `pointer-table`: U48 `ok` (4 of 4 indexes named) — no Edit.
- `deeper-topics`: recomputed from the files that exist — the docs list (5 specialist summaries, 5 core,
  `session-learnings.md`) and the rules list (`security` · `testing` · `observability` · `a11y` · `frontend` ·
  `verification-harness` · `host-linux`) both match the tree — no Edit.
- CLAUDE.md is NOT written (138 lines). Backed up per Setup 5.

## Tier 2 — .claude/rules/
| file | disposition |
|---|---|
| `a11y.md` | preserve |
| `frontend.md` | preserve |
| `observability.md` | preserve |
| `security.md` | preserve |
| `testing.md` | preserve |
| `verification-harness.md` | preserve |
| `host-linux.md` | regenerate — the operator's word at the card (U49); `upgrade.py apply --id U04 --regenerate` |

Absent: none.

## Tier 3 — .claude/docs/
`stack.md` · `conventions.md` · `commands.md` · `gotchas.md` · `workflow.md` · `security-summary.md` ·
`design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md` · `session-learnings.md` — each
`preserve`. Absent: none. No services leaves.

## Agent harness
`scripts/agent-run.sh` · `scripts/agent-run.ps1` · `.claude/rules/verification-harness.md` — present, each
`preserve`. No fresh render made, nothing compared (upgrade form).

## Code reviewer
`.claude/agents/code-reviewer.md` — present, `preserve` (rust).

## Hooks — .claude/settings.json
- U02 `behind` (`write inline · bash inline`): the two PreToolUse entries' `command` strings are replaced by the
  matrix's calls of the install's scripts —
  `bash ~/.claude/skills/andromeda-tools/hooks/write-guard.sh` (matcher `Edit|MultiEdit|Write|NotebookEdit`) and
  `bash ~/.claude/skills/andromeda-tools/hooks/bash-guard.sh` (matcher `Bash`). Matchers and timeouts stand.
- `env` block present (`PYTHONUTF8` · `PYTHONIOENCODING`) — stands.
- PostToolUse formatter entry equals the matrix's Rust row byte for byte — stands.
- Formatter config: `rustfmt.toml` present (U05 `ok`) — nothing written.

## Gitignore · .gitattributes
U07 `ok` · U06 `ok` — nothing written.

## Code-graph pipeline
Every seeded file present; U03 `ok` — lines behind `code-graph.py` 0 · `code-graph-views.sql` 0 ·
`code-graph-cookbook.md` 0. Nothing written, nothing proposed.

## Session / operational seeds
`state.yaml` · `session-handoff.md` · `drift-base.md` · `playbook.md` · `session-learnings.md` — present; nothing
seeded.

## Upgrade
Setup 5b's record: HEAD `9e8b30f9f61c00ad19140daa45fed1a61c30f712` · branch `build/conductor-0.4.0`, level with its
upstream · path set: ` M .claude/session-handoff.md` (expected-transient bookkeeping), nothing else.
`route.py cursor`: `records 165 · complete 165 · pending 0 · gated 0` · `half-promote 0 of 0 stamped lines vs 165
master records`.

`upgrade.py detect --root .` (read-only, no `--run-dir`), verbatim:

```
upgrade v1.8 · 30b07c54
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write inline · bash inline
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
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
upgrade: for setup 1 (U02) · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0 · 17 detectors of 45 registry entries
```

Acting rows: **U02** (P5). **U49** carries no detector (class `operator`); its act is the card's
`regenerate host-linux.md`, the operator's word given at invocation and confirmed at the card.
Noted, not written: U09 (`workflow.md` lacks `it never commits`) · U10 (`agent-run.sh` lacks
`ensure_fresh_artifacts`) · U36 (`security-summary.md` lacks `cascade re-derives it`, one leaf standing for the five
headers).

## Pre-write measurements (read-only, before the card)
- The install's two guard scripts, each run as the new entry would call it: Bash guard 6 of 6 arms as expected
  (heredoc deny 2 / allow 0 · `cd .andromeda && ls` 2 · `cd . && ls` 0 · `ls && cd .andromeda` 2 · subshell `cd` 0);
  write guard 3 of 3 (`src/x.rs` 0 · two backslash `target` paths 2).
- `upgrade.py apply --id U04 --regenerate --dry-run`: `would refresh .claude/rules/host-linux.md above
  `## Session Additions` (50 → 47 lines); 3 line(s) below it byte-identical`.
- U49's premise on this host: a backslash pair survives the transport single-quoted and in a quoted heredoc
  (`od -c` shows two backslashes in both).

## Stop
The operator's word: stop before the commit. Phase 9 does not run in this session; the writes stay uncommitted and
`detect` reads U02 `ok-uncommitted` until a commit carries them.
