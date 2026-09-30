# Operator pass — entries 18, 19, 20

Fired by /implement on the operator's (overseer's) explicit word, 2026-09-30, each by hand in its plan form (the gate
tool refuses `leg = 'operator'` entries); exits read from the bare commands.

| entry | command | exit | reading |
|---|---|---|---|
| 18 | `python -X utf8 …/andromeda-tools/scripts/gate.py hygiene` | 0 | `hygiene: clean — read 35 (runs 31 · evidence 4) · trails 13 not read · binary 0 not read by P1` (10:58:56Z) |
| — | operator pre-CI commit (`git add -A`, 57 files) | 0 | `4bab3c6` `chore(2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed): operator pre-CI commit, for the run this chunk's verdict reads`; tree clean after |
| 19 | `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=…"` | 0 | `7ee2fea..4bab3c6  HEAD -> build/conductor-0.3.0` · `PUSHED_SHA=4bab3c6c2ce1a5d9544e550694ab8a532d232b2b` |
| 20 | `python -X utf8 …/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1200` | 0 | `4bab3c6c2ce1 verdict: green · checks 3/3 · wall 667 s · runs CI#36705777679 completed/success` (polled 23× over 684 s) |

CI run **36705777679** on `4bab3c6` is green on all three jobs: the `a11y` job ran the routine arm with the three new
`it()` blocks, and the `rust` job ran the secret-scan gate over the new evidence files.

Carried to the wrap, not resolved here (operator directive): the SR host silence (entry 12 red, the base bundle
reproduces it — `driven-leg.md` Findings 2) and the unsatisfiable `Spec Files: 1 passed, 1 total` literal in entries
8/10/11 (wdio prints `Spec Files:` + TAB + space; the operator confirmed the tab in the routine arm's captured log).
