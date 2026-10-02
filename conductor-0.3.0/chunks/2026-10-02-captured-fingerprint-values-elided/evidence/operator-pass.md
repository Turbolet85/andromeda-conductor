# Operator pass — captured fingerprint values elided

The operator pass of the approved plan (Test Commands entries 19-21). Performed by the agent on the
overseer's word of 2026-10-02 ("Run the operator pass in the approved plan: entry 19 hygiene, the pre-CI
commit, entry 20 push, entry 21 CI read"). The commit and the push are the operator's acts, made on that
word.

## Entry 19 — hygiene

- Command: the entry's exact `run` (`gate.py hygiene` from the andromeda-tools scripts dir), fired by hand
  before the pre-CI commit, and fired again after this record was written so that its read covers it.
- Exit: 0 on both firings.
- Atoms: `exit 0` held; `contains hygiene: clean` held, on both firings. The first summary line read
  `hygiene: clean — read 32 (runs 29 · evidence 3) · trails 14 not read · binary 0 not read by P1`, with every
  control fired on its synthetic known positive. The second, which covers this record, read `read 33 (runs 29 ·
  evidence 4)`, also clean.

## The pre-CI commit

- The commit is `6e7a345`: `chore(2026-10-02-captured-fingerprint-values-elided): operator pre-CI commit, for
  the run this chunk's verdict reads`. It stages the whole tree, as the 2026-10-02 precedent `2a49480` did.
- It is the operator's act, made on the overseer's word.
- The commit message quotes neither residual value.

## Entry 20 — the guarded push

- Command: the entry's exact `run`. That is the clean-tree guard, `git push origin HEAD`, then the `PUSHED_SHA=`
  echo.
- Exit: 0. It pushed `31f9d92..6e7a345` to `build/conductor-0.3.0`.
- Atoms: `exit 0` held, and `contains PUSHED_SHA=` held
  (`PUSHED_SHA=6e7a34526686333a293517487d4b97a30ca4edc9`).

## Entry 21 — the CI read

- Command: the entry's exact `run` (`ci.py conclusion --sha HEAD --wait 1200`), bounded at the entry's
  `timeout = 1380`.
- Exit: 0. It polled 21 times over 622 s.
- Atoms: `exit 0` held, and `contains verdict: green` held. The tool printed `6e7a34526686 verdict: green · checks
  3/3 · wall 598 s · runs CI#37032414148 completed/success`.
- The run this chunk's CI acceptance names is **CI#37032414148** (push, completed/success).
