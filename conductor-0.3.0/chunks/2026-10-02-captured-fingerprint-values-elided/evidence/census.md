# Census — captured fingerprint values elided

Chunk `2026-10-02-captured-fingerprint-values-elided`, base `31f9d92`. This record quotes neither residual
value. Every value-bearing count below was derived by extracting the values from the two capture sites at
`31f9d92`, then printing only paths and counts.

## Red before the fix (step 1)

- **Run:** `cargo nextest run -p conductor-run --test real_model_harvest --profile ci`, after the population arm
  was added and before any capture or the elider changed.
- **Result:** 76/101 tests ran, 75 passed and 1 failed. The `ci` profile's fail-fast cancelled the rest. The
  failure was `every_committed_capture_carries_no_un_elided_fingerprint_value`, and it named exactly two files:
  - `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt`: 2 un-elided keyed
    values. It was not a fixed point of the elision, because its values matched the unkeyed shape rule.
  - `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/evidence/rm-capture-d3.txt`:
    2 un-elided keyed values. It was a fixed point, because its all-digit values passed the unkeyed rule.
- **Population:** the arm found the pinned count of 14 files before it failed on these two. The other twelve
  passed.

## Green after the fix (step 8)

- **`real_model_harvest`:** 102/102 tests ran and passed (implement run `2026-10-02T15-37-06`, gate entry 1).
  That includes:
  - `every_committed_capture_carries_no_un_elided_fingerprint_value`;
  - `un_elided_keyed_values_counts_a_planted_value`, the inverse control. Its planted value is built at test time
    and never committed.
  - the extended `a_fingerprint_is_elided_and_a_stamp_a_seed_and_a_det_prefix_are_not`;
  - `the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key`, now with no d3 exception;
  - `the_frozen_capture_is_elided_in_place_and_equals_the_graded_copy`;
  - `the_later_scrub_stages_leave_the_placeholder_intact` and
    `a_path_valued_workspace_leaks_neither_its_key_nor_its_path`;
  - every existing 2026-10-01 grade, rule-record and digest arm, over the moved d3 pin.
- **Other test gates:** `cargo test -p conductor-run` passed 340 with 0 failed. The workspace nextest run passed
  1178/1178.

## Census per class, before and after

| Class | Before (`31f9d92`) | After (working tree) |
|---|---|---|
| Real-model captures: `conductor-0.3.0/chunks/*/evidence/rm-capture*.txt` (14 files) | 2 files carry a value: the frozen 2026-09-22 capture (2 hex) and the 2026-10-01 d3 capture (2 all-digit) | 0 files, 0 values |
| Prose quotes of the frozen value | 3 files: 2026-09-22 `report.md` ×1, 2026-09-30 `plan.md` ×3, 2026-09-30 `research.md` ×1 | 0 |
| Gate trails quoting the frozen value | 2 files: the 2026-09-30 implement trail ×8 and the 2026-09-30 wrap trail ×2 | 0 |
| Quotes of the d3 value outside its capture | 0 | 0 |
| Outside class: conductor-0.2.0 live-leg evidence | 14 files, 35 values | unchanged |
| Outside class: harvest test source | 4 files, 8 values: `baseline_harvest.rs` 1, `pii_harvest.rs` 2, `severity_harvest.rs` 3, `storm_harvest.rs` 2 | unchanged |

The outside-class values are Pulse's fingerprints of Conductor's own synthetic exceptions. They sit outside the
real-model capture exception's scope, and several of them carry verified graders (scope.md §Census). They stay as
they are, per the P4 subject answer.

## Derivations

- **Capture population:** `git ls-files 'conductor-0.3.0/chunks/*/evidence/rm-capture*.txt' | wc -l` = 14 at
  `31f9d92`. The test's filesystem walk finds the same 14, and the count is pinned in `COMMITTED_CAPTURES`. All 14
  are `i/lf w/lf` (`git ls-files --eol`).
- **Per-file values:** read through a scratch census script (session scratchpad `census.py`) over
  `git ls-tree -r 31f9d92` (before) and `git ls-files --cached --others --exclude-standard` (after). It excludes
  `.andromeda/runs/` and the friction ledger. The script classifies each value after a `fingerprint_hex` key as
  elided, hex or all-digit. It prints paths and class counts, never a value.
- **Prose and trail counts:** a scratch replace script (session scratchpad `replace_quotes.py`) extracts each
  residual value from its site at `31f9d92` and asserts one distinct value per site. It then counts each value per
  record file with `bytes.count`, plus a word-bounded count. The counts were 1/3/1/8/2, all bounded, for the
  frozen value, and 0 everywhere for the d3 value. The same script performs the in-place replacement and re-counts
  0 afterwards.
- **conductor-0.2.0 evidence:** a pattern of `fingerprint_hex`, at most 6 separator characters, then 8 lowercase
  hex digits, counted per file over `git ls-files conductor-0.2.0`. It counts 14 files and 35 values. A pattern
  limited to `=`, `"`, `:` and space as separators finds 13, because one `leg-verdict.md` separates with a
  backtick.
- **Test source:** the same wider pattern over `crates/` finds 4 files and 8 values. The values the new harvest
  arms use are built at test time from string pieces, so none sits in the source as a literal.
- **The value-absence sweep:** gate entry 9 extracts the two values from `31f9d92` and runs
  `git grep --untracked -l -F` per value over the tree. It exits 123 with no output, behind entry 8's
  anti-vacuity count of 2.

## The frozen-evidence exception

`conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt` was elided in place on
2026-10-02. This is an exception to the standing rule "frozen evidence is never edited". It is recorded against
the founder's ruling of 2026-10-02, «ничего не переносим» ("nothing gets deferred"), relayed by the overseer. The
ruling made the residual FIXED rather than ratified, and the overseer's P4 note answered the frozen-file fork as
"elide it in place".

- **What changed:** exactly lines 256-257. Each line's `fingerprint_hex` value became `<fingerprint>`. No other
  byte moved (`git diff --numstat 31f9d92` reads 2/2).
- **The result:** the file is byte-identical to the graded 2026-09-30 copy,
  `rm-capture-2026-09-22-elided.txt` (`cmp` exit 0). Its sha256 equals that copy's pin, `d57c2613…`. The copy
  stays the pinned, graded form.
- **Citations:** every citation of the path keeps resolving, including the 2026-09-22 `plan.md` at `:460`, `:481`,
  `:484` and `:498`.

The graded 2026-10-01 d3 capture was re-elided the same way, on lines 514-515 only. Its `SERIES_2026_10_01` pin
moved to `b49bfe68…` in the same change, and its grade is unchanged.

Beside the exception sit the two 2026-09-30 gate trails under `.andromeda/runs/`:
`2026-09-30T04-48-49-implement` and `2026-09-30T07-22-03-wrap`. They are in scope as the founder ruled at the P5
approval of 2026-10-02. They were edited for the same reason as the prose records, by byte-level in-place
replacement, and both still parse as JSON (gate entry 13).

## History

Git history was not rewritten and nothing was force-pushed. The original values remain in past commits, which the
overseer accepted. The subject of this chunk is the committed tree.
