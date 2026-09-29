# Consolidation record — the 2026-09-29T05-42-49-wrap 0-pending wrap

Door: U13, the sidecar consolidation (`upgrade.py detect` read it `awaiting a door` at `67e8cb1`). Operator-requested
via the overseer relay `conductor-wrap-0pending-2026-09-29`. The seven sidecars had never been consolidated, so this
run is their backfill: every entry was re-worded. Tool: `sidecar.py v1.1 · 1c094949`, `--marker no-marker`.

## Before the consolidation — three severed entries restored (relay step 1)

Commit `06db2f9` (the `2026-09-07-sr-findings-fixed` wrap) cut the `**Why:**` line of the
`## 2026-09-07-a11y-ci-gate` entry in three sidecars. All three were intact at `00181df`.

| sidecar | Why line at HEAD → restored |
|---|---|
| security-plan | 150 → 832 chars |
| architecture | 146 → 697 chars |
| test-plan | 150 → 598 chars |

- Each entry was restored from `git show 00181df:{path}`, and each restored block is byte-identical to the `00181df`
  block (sha256 prefixes `24bb488a7798f228` · `03e54305f40cf523` · `cec57263f0e80cda`, identical on both sides).
- Form deviation from the relay's letter: at `00181df` each entry was the file's LAST block, so its block ends at EOF.
  At HEAD a one-line blank gap follows it. The gap was kept, so the restored entry does not abut the next heading.
- **Premise corrected:** the relay said the rest of each line was LOST. It was DISPLACED. Each severed tail sat whole
  as an orphan line (preceded by its own blank line) at the end of the following `2026-09-07-sr-findings-fixed` entry.
  The security-plan and test-plan rewriters each flagged it independently as a stray copy. On the operator's word, each
  orphan was removed only after asserting it byte-equal to the restored tail (682 · 551 · 448 chars); each tail now
  occurs once per file. The three sidecars were then re-split (`consolidate/{doc}-2`), and the finished rewrites were
  carried over by heading. Each doc's one changed source was its `sr-findings-fixed` row, and none of those three
  rewrites carries the orphan text.

## Per doc

sidecars: architecture 142 re-worded · 1 pruned · 237560→172509 B
sidecars: security-plan 59 re-worded · 0 pruned · 94474→79680 B
sidecars: design-system 16 re-worded · 0 pruned · 17583→15438 B
sidecars: layout-templates 32 re-worded · 0 pruned · 41059→37768 B
sidecars: test-plan 81 re-worded · 0 pruned · 134975→112620 B
sidecars: obs-plan 47 re-worded · 1 pruned · 69983→58156 B
sidecars: a11y-plan 21 re-worded · 0 pruned · 38819→29682 B

The bytes-before figures are HEAD's (`67e8cb1`); the tool's own lines read the tree after step 1. In total,
634 453 → 505 853 B and 398 → 396 entries. All seven now read `off-form 0`. Only `architecture` (172 509 B) stays
OVER the 120 000 B whole-read bound, so phase reads it through its index. Each archive
`.andromeda/{doc}-amendments-archive.md` holds that doc's originals verbatim under this run's heading.

## Ref derivation (W177, `sidecar.py` v1.1)

381 of 398 Refs were derived from the chunk's OWN wrap (flip commit 302 · both anchors 79 · by name 0 · anchors
disagree 0), with 0 derived from a text mention. 17 are `NOT DERIVED`: headings with no marker the tool can anchor
(date-only `## 2026-09-16 —` / `## 2026-09-17 —` forms, `0-pending adaptation` entries), plus
`2026-06-23-isatty-gated-operator-pause`, `2026-06-24-frameless-window-shell` and
`2026-08-21-delegated-timing-budgets-proven` rows. These are facts, not edits.

## Supersedes (W178)

The rewriters wrote 9 Supersedes claims, and a read-only reviewer judged each pair from the ORIGINAL texts:
- **2 WHOLE, kept and pruned:**
  - architecture `2026-09-04-sr-findings-remediation — the precondition probe cannot exit 0 …`, retired by `2026-09-04-preconditions-probe-reads-path-handles-by-presence`.
  - obs-plan `2026-08-16-fingerprint-storm-live-proof — fingerprint-storm \`fingerprints\` no longer "populated"`, retired by `2026-08-18-error-baseline-spike-live-proof — fingerprints MAY-BE-EMPTY-under-L4 superseded (three sites)`.
- **7 PARTIAL, claim dropped:** the later entry retires only part of the earlier one, which still carries standing
  claims.
  - architecture 42←46 · 46←47 · 68←72
  - layout-templates 10←12 · 25←26
  - obs-plan 19←23 · 32←34

  Pulse's run on 2026-09-28 found 3 of 5. The remaining `UNRESOLVED 1` in architecture and in obs-plan is each WHOLE
  superseder naming a target that now lives in its archive.

## Read of the DROPPED rows

1 509 backticked spans went missing from the first rewrites:
- 168 survive as plain text.
- 597 are locators the entry form sends out (file:line, evidence and run paths, SHAs, run ids, detector ids).
- 744 spans in 171 rows needed judging. Eight restorer agents judged them against the originals and re-added 217 of
  them (rules, identifiers, counts, thresholds) to 41 outputs, each output kept under 2 900 B.
- The 527 left out were spot-read on the fourteen heaviest rows: sweep counts, cascade targets, grep patterns, CI
  diagnostic readings and report-section names, all of the leave-out class.

ADDED spans were read against their sources:
- layout-templates rows 8/10, `(11 unbacked)` / `(10 unbacked)`, are the retired captions the sources state as
  "shrank 11 → 10" / "10 → 9". They stand.
- architecture row 79's `declares()` accepting only `"true"`/`"1"` is NOT in its source. It was replaced with the
  source's own retired terms (UNSATISFIABLE · permanently unmet · short-circuited before every preflight).
- architecture row 129's `boot` is not in its source. It was reworded to name the two scripts only.

## Hygiene

`gate.py hygiene` read the run dir clean. A direct probe still found 50 files (the tool's batch lists and manifests,
and this run's review lists) carrying the repo root as an absolute path. All 50 were rewritten repo-relative, and 0
remain.
