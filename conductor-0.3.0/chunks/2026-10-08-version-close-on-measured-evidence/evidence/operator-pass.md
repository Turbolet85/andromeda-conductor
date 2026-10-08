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

Re-read once after this record was first written, so the record itself was in the set the commit carries: exit 0,
`hygiene: clean — read 47 (runs 37 · evidence 3 · inputs 7)`, the other counts unchanged. What follows from here
was appended after the push, so it is not in the pre-CI commit.

Pre-CI commit (the operator's act, on the word above): `git add -A`, then
`chore(2026-10-08-version-close-on-measured-evidence): operator pre-CI commit, for the run this chunk's verdict reads`.
It landed **`42daf3e`** on the first attempt: 57 files, of which three sit outside the pipeline's own folders — the
two scripts and the contract. The tree read clean after it (`git status --short`, 0 rows).

## Entry 29 — `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=$(git rev-parse HEAD)"`

Exit 0. `fb48cee..42daf3e  HEAD -> build/conductor-0.3.0`, printed
`PUSHED_SHA=42daf3e47e3052aa36a56984871600b5a005ad98`. Atoms `exit 0` and `contains PUSHED_SHA=` hold. Read back
from the remote afterwards: `git ls-remote origin refs/heads/build/conductor-0.3.0` prints the same sha. The entry
pushed the build branch and nothing else: no tag, no other branch.

## Entry 30 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1500`

Exit 0 (fired under `timeout 1680`, the entry's own bound):

```
ci v1.0 · a1c08692
repo Turbolet85/andromeda-conductor (the push remote `origin`) · polled 24× over 713 s
42daf3e47e30 verdict: green · checks 3/3 · wall 708 s · runs CI#37763841102 completed/success
runs: CI#37763841102 push completed/success
```

Atoms `exit 0` and `contains verdict: green` hold. The run the CI acceptance names is **CI#37763841102**. Read
from the run itself (`gh run view 37763841102 --json headSha,conclusion,status,jobs`): head sha
`42daf3e47e3052aa36a56984871600b5a005ad98`, status completed, conclusion success, and each of its three jobs
(`rust`, 32 steps; `frontend`, 9 steps; `a11y`, 21 steps) concluded success with no step reading anything but
success.

The two edited scripts' executions, read from that run:

- `scripts/agent-run.ps1`: the `rust` job's step "Test + lint (dogfood agent-run)" concluded success.
- `scripts/a11y-token-witness.ps1`: the `a11y` job's step "A11y routine arm (tauri-driver + axe + contrast)"
  concluded success. Read from the `a11y` job's own log (the jobs API, never the combined view), the step echo
  filtered out: `[a11y] verdict asserted - 0 failed | 2 skipped (expected 2) | driven session present`, on
  `webview2 131.0.2903.86`, 19 passing and 2 skipped. The skip tally is the expected 2, unchanged.

The `rust` and `frontend` jobs' logs were not read.

The pass ends here on the operator's word: the wrap is not started, and `pwsh scripts/agent-run.ps1 run` was not
driven.
