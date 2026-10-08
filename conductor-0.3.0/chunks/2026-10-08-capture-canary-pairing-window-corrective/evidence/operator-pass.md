# Operator pass — entries 19, 20, 21

Performed by the agent on the operator's explicit word, given in this session on 2026-10-08: "Operator pass: go,
entries 19 (hygiene), 20 (pre-CI commit, guarded push) and 21 (CI read). […] Your three deviations are accepted.
After the CI read STOP: do not start the wrap, I send it." The commit and the push are the operator's acts, made
on that word, never a skill bypass.

The same message states what the operator verified first: "my own harvest run reads 139 of 139; the contract diff
is 12 added, 0 removed; no other chunk folder, pin, the rule file or the lockfile moved against 39e197b;
failing-first.md holds both readings (43/139 under fail-fast, then 137 passed 2 failed)."

## Entry 19 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`

Fired bare, 2026-10-08, before the pre-CI commit. Exit 0.

```
gate v1.12 · bdf1c88a
root . · case exact · planes rust, ts · base HEAD (no --marker)
control: P1 drive · P1 device · P1 home · P1 users · P1 root · P1 msys · P1 wsl · P1 tmp · P2 · P3 rust · P3 ts — each fired on its synthetic known positive
hygiene: clean — read 40 (runs 35 · evidence 1 · inputs 4) · trails 13 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Atoms `exit 0` and `contains hygiene: clean` hold. No row was printed, so nothing was rewritten or removed.

Re-read once after this record was written, so the record itself is in the set the commit carries: exit 0,
`hygiene: clean — read 41 (runs 35 · evidence 2 · inputs 4)`, the other counts unchanged.
