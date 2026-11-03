# Materialization plan — conductor · re-run, upgrade form

Development Style: agent-driven · host: linux · stack: rust · invoked for U56 (overseer, founder-delegated).
No upstream body read (upgrade form); no structural change named.

## Tier 1
- overview · modules · warnings (10 bullets) · pointer-table · workflow · architecture: not re-derived — cascade-maintained, each standing byte for byte.
- imports: the template's (U01 ok) — no Edit.
- deeper-topics: recomputed from the files that exist — `.claude/docs/` (5 summaries + 5 core + session-learnings) and `.claude/rules/` (security · testing · observability · verification-harness · a11y · frontend · host-linux) match the block as it stands — no Edit.
- pointer-table: U48 ok — no Edit. CLAUDE.md is not written (backup only).

## Tier 2
Present, each `preserve`: security.md · testing.md · observability.md · verification-harness.md · a11y.md · frontend.md · host-linux.md (U04 noted: 9 template lines missing above `## Session Additions` — regenerate on the operator's word).

## Tier 3
Present, each `preserve`: stack · conventions · commands · gotchas · workflow (U09 noted) · security-summary (U36 noted) · design-summary · tests-summary · obs-summary · a11y-summary · session-learnings. Absent: none.

## Agent harness
`scripts/agent-run.sh` · `scripts/agent-run.ps1` · `.claude/rules/verification-harness.md` — present, `preserve` (U10 noted: agent-run.sh lacks `ensure_fresh_artifacts`).

## Code reviewer
`.claude/agents/code-reviewer.md` — present, `preserve` (rust).

## Hooks
U02 ok — `.claude/settings.json` not written.

## Code-graph pipeline
U03 ok — the triple 0 lines behind; scip_pb2.py · requirements.txt present. Nothing seeded.

## Gitignore
U56 behind — append `.andromeda/runs/*/prompts/` to the root `.gitignore`. U07 ok — no other base line. `.gitattributes`: U06 ok — not written. `rustfmt.toml`: U05 ok.

## Upgrade
HEAD at 5b: 4e5a69c9a838883c5b655a405a978fb2a5eccfcb
Path set at 5b: ` M .claude/session-handoff.md` (bookkeeping)

```
upgrade v1.21 · 58fca40c
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-gra… · lines behind: code-graph.py 0 · code-graph-views.sql 0 · code-graph-cookbook.md 0
U04 · noted · setup · .claude/rul… · 9 template line(s) missing above `## Session Additions` — regenerate on the operator's word
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts… · scripts/agent-run.sh lacks `ensure_fresh_artifacts`
U11 · ok · setup · code-metrics.ndjson line endings (and, until S1's deletion, the friction le… · code-metrics.ndjson LF
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendme… · masters in the store — every sidecar entry on the form held at write (U60)
U14 · ok · hand · working-route markerless i… · 59 markerless entries · 0 introducers behind markup or after the em-dash
U35 · ok · hand · masters' lo… · masters in the store — the logs and the keyed contracts out of the body, a key regrown refused held at write (U60)
U36 · noted · noted · .claude/docs/{security,design,te… · .claude/docs/security-summary.md lacks `cascade re-derives it`
U48 · ok · setup · CLAUDE.md p… · masters in the store — CLAUDE.md's pointer rows naming `rec.py contracts {plan}` held at write (U60)
U56 · behind · setup · .gitignore base ignore `.androm… · not ignored by the root .gitignore: .andromeda/runs/*/prompts/
U57 · ok · hand · the friction family in the store — `.andromeda/db/{moved, log/*.ndjson, reco… · friction: in the store
U58 · ok · hand · the spine in the store — `version` · `epoch` · `chunk` · `carry` · `capability`… · spine: in the store
U59 · ok · hand · a chunk's and a version's documents and files in the store — `doc` · `section` ·… · docs: in the store
U60 · ok · hand · the masters in the store — the seven `.andromeda/{doc}.md` bodies, their cate… · masters: in the store
upgrade: for setup 1 (U56) · blocks phase 1 (U56) · awaiting a door 0 · noted 4 (U04, U09, U10, U36) · INDETERMINATE 0 · 22 detectors of 54 registry entries
```

Validation: internally consistent — one write planned (`.gitignore`), one backup (CLAUDE.md), no Tier/harness/hook write.
