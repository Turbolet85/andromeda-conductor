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

Pre-CI commit (the operator's act, on the word above): `git add -A`, then
`chore(2026-10-08-capture-canary-pairing-window-corrective): operator pre-CI commit, for the run this chunk's verdict reads`.
It landed **`9b4a0b8`** on the first attempt: 52 files, of which four sit outside the pipeline's own folders — the
three test files and the contract. The tree read clean after it (`git status --short`, 0 rows).

## Entry 20 — `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=$(git rev-parse HEAD)"`

Exit 0. `39e197b..9b4a0b8  HEAD -> build/conductor-0.3.0`, printed
`PUSHED_SHA=9b4a0b847361b1a98996dbbb80d65b0da887b9d1`. Atoms `exit 0` and `contains PUSHED_SHA=` hold. Read back
from the remote afterwards: `git ls-remote origin refs/heads/build/conductor-0.3.0` prints the same sha.

## Entry 21 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1500`

Exit 0 (fired under `timeout 1680`, the entry's own bound):

```
ci v1.0 · a1c08692
repo Turbolet85/andromeda-conductor (the push remote `origin`) · polled 22× over 651 s
9b4a0b847361 verdict: green · checks 3/3 · wall 651 s · runs CI#37754365520 completed/success
runs: CI#37754365520 push completed/success
```

Atoms `exit 0` and `contains verdict: green` hold. The run the CI acceptance names is **CI#37754365520**. Read
from the run itself (`gh run view 37754365520 --json headSha,conclusion,status,jobs`): head sha
`9b4a0b847361b1a98996dbbb80d65b0da887b9d1`, status completed, conclusion success, and each of its three jobs
(`rust`, `frontend`, `a11y`) concluded success. The jobs' logs were not read.

The pass ends here on the operator's word: the wrap is not started.
