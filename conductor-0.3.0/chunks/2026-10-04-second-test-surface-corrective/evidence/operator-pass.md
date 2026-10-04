# Operator pass — gates 23, 24, 25

Performed by the agent on the overseer's explicit word (2026-10-04: "Run the operator pass now: gate 23 hygiene and the
pre-CI commit, gate 24 push, gate 25 CI read (the ps1 green-path proof). Then stop and report") — the operator's acts,
made on that word, never a skill bypass.

## Gate 23 — `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`

First run (2026-10-04, before the commit): `hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read 44 (runs 39 · evidence 5)`,
exit 0. The one row was `.andromeda/runs/2026-10-04T12-48-50-phase/.p5-dryrun.txt:4 ×3 · tmp,home` — the phase run's raw
P5 dry-run capture of `gate.py run --dry-run`, whose header `logs` line carried the OS temp-dir log path and whose two
operator-leg rows carried the skills directory under the home dir. No acceptance, report or evidence cites the file
(a fixed-string search for its run-dir path matched only this implement run's gate-trail delta list), so the letter
rewrote it in place: the temp-dir prefix became `<os-temp>/`, the home-dir skills prefix `~/.claude/`, and the repo
root `<repo-root>` (4 substitutions; nothing else changed).

Re-run: `hygiene: clean — read 44 (runs 39 · evidence 5) · trails 12 not read · binary 0 not read by P1`, exit 0 — the
atoms `exit 0` and `contains hygiene: clean` hold.

Pre-CI commit (the operator's act, on the word above): `git add -A` then
`chore(2026-10-04-second-test-surface-corrective): operator pre-CI commit, for the run this chunk's verdict reads`.
