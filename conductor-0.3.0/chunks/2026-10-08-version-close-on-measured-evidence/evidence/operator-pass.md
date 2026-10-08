# Operator pass — entries 28, 29, 30

Performed by the agent on the operator's explicit word, given in this session on 2026-10-08: "Operator pass: go,
entries 28 (hygiene), the pre-CI commit, 29 (guarded push of the build branch) and 30 (CI read). […] Do NOT drive
pwsh scripts/agent-run.ps1 run: no unplanned leg at the close; the wrap amends the stale reason. After the CI read
STOP: do not start the wrap, I send it (a pipeline deploy must land first)." The commit and the push are the
operator's acts, made on that word, never a skill bypass.

The same message states what the operator read first: "I read version-close.md and next-version-direction.md
whole, the contract block and both comment diffs: accepted, including your pwsh correction and the three
deviations."

## Entry 28 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`

Fired bare, 2026-10-08, before the pre-CI commit. Exit 0.

```
gate v1.13 · 3870f1a7
root . · case exact · planes rust, ts · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P1 in-root home · P1 in-root drive · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 46 (runs 37 · evidence 2 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Atoms `exit 0` and `contains hygiene: clean` hold. No row was printed, so nothing was rewritten or removed.
