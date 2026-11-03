# P5 controls — 2026-11-03-linux-only-base-ci (check 4 (9))

Clone: `git clone --shared` of the project at `8e450f3`, `$TMPDIR/{scratchpad}/clone-p5`; inputs applied by
`$TMPDIR/{scratchpad}/p5_controls.py` (modes `build` · `neg-roster` · `neg-pin` · `neg-sh`); each entry fired as itself,
`gate.py run --chunk 2026-11-03-linux-only-base-ci --docs-root . --entry {n} --project-root {clone}`.

## Unmodified clone (must equal the tree's baseline)
- 5 green (0) · 6 red (exit 1 expected 0, 3195 B) · 7 red (1212 B) · 8 red (exit 1) · 9 red (exit 0, hits) · 11 red (5131 B)
  · 12 red (exit 1) · 13 green · 14 red (the 57-path row) · 15 green — each equal to the tree's reading.

## The subject as the plan builds it (`build`: steps 2-7 and the row as the reconcile is to leave it)
- 5 green · 6 green (exit 1, 0 B) · 8 green (last line 2) · 9 green (exit 1, last line 0) · 11 green (0 B)
  · 12 green (last line 46) · 13 green (re-pointed constants move no pin line) · 14 green (65 B, the pair) · 15 green
- 7 red, 658 B: its only hits are 5 lines inside `scripts/agent-run.ps1` itself (`git grep … | cut -d: -f1 | uniq -c`
  → `5 scripts/agent-run.ps1`), the file step 8's `git rm` removes; no removal was run, the clone included.

## Planted negatives (a green guard must read red)
- `neg-roster` (one `citation_home` set to an absent path) → 5 red, last line 1.
- `neg-pin` (one sha256 pin character flipped in `real_model_series/mod.rs`) → 13 red, exit 0 (a pin line counted).
- `neg-sh` (a comment line appended to `scripts/agent-run.sh`) → 15 red, exit 1.

## Tree baselines (`gate.py run --entry {n} --run-dir`, untouched tree)
- 3 green, 3.45 s · 4 green · 5 green · 13 green · 15 green · 6, 7, 8, 9, 10 (last line 59), 11, 12, 14 red — every
  red on this chunk's own subject (its paths, its files, its counts).
