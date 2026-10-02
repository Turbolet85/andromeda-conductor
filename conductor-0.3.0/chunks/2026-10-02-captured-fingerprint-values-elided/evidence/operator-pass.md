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
