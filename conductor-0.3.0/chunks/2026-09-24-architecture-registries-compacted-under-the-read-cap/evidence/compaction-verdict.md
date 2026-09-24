# Compaction verdict — 2026-09-24-architecture-registries-compacted-under-the-read-cap

Measured 2026-09-24 at /implement against HEAD `f0d92bd` (the pre-compaction tree), with
`scripts/arch-registry-check.py` over the drafts in `../compaction/`. The master is not edited here:
wrap P2 applies the drafts, and its P7 light gate re-runs the same entries after the apply.

## BEFORE — `measure --rev HEAD` (exit 1, the can-fail control on the real artifact)

```
threshold 38115 B (60% of 25000 tokens at 2.541 B/token)
  §Established Decisions: 49134 B · ~19336 tokens · 77.3% of cap · OVER target
  §Occupied Resources: 48859 B · ~19228 tokens · 76.9% of cap · OVER target
registries: OVER target
```

The section byte counts include each section's trailing blank line. That is why they are one byte above
research M5's figures (49 133 / 48 858 B), which ended at the last text line.

## AFTER — `check --before-rev HEAD --drafts …` (exit 0)

```
  AFTER §Established Decisions: 37658 B · ~14820 tokens · 59.3% of cap · within target
  AFTER §Occupied Resources: 37929 B · ~14927 tokens · 59.7% of cap · within target
judgment rows: 151 (rewritten 150 · span-less in-sidecar 1)
arch-registry-check: PASS
```

Margins under the threshold: 457 B (§Established Decisions) and 186 B (§Occupied Resources). The plan's
forecast was ≈31-35 KB and ≈27-33 KB, and the measured sizes are above both. History removal alone did not
reach the threshold for §Occupied Resources. The last ~1.5 KB came from condensing verbose
reader-registration prose, which is current truth restated more briefly. Every such sentence is a
`rewritten` row.

## `selftest --before-rev HEAD --drafts …` (exit 0)

```
  a size: detected (1 new findings)
  b labels: detected (3 new findings)
  c registry: detected (4 new findings)
  d span floor: detected (2 new findings)
  e hygiene: detected (1 new findings)
  f ledger: detected (1 new findings)
  f ledger: detected (1 new findings)
  f ledger: detected (1 new findings)
selftest: every arm detected
```

The three arm-(f) lines are: a deleted row, a non-AFTER row flipped to `kept`, and an `in-sidecar` row
repointed to a marker whose entries lack its span.

## `status --drafts …` (exit 0, report-only)

```
§Established Decisions: master 49134 B · draft 37658 B · applied: no
§Occupied Resources: master 48859 B · draft 37929 B · applied: no
```

`applied: no` is by design at /implement. The wrap's light gate must read `applied: yes` for both sections.

## Disposition ledger tally (as measured)

| section | kept | moved | in-sidecar | rewritten | rows |
|---|---|---|---|---|---|
| §Established Decisions | 175 | 44 | 5 | 44 | 268 |
| §Occupied Resources | 177 | 24 | 4 | 106 | 311 |
| total | 352 | 68 | 9 | 150 | 579 |

The splitter counts 579 distinct sentences. The P5 prototype's 282 / 328 was a different regex and
was recorded as a forecast only.

How the dispositions were assigned:
- A scratch helper proposed one disposition per row:
  - `kept`: the sentence is verbatim in AFTER;
  - `rewritten`: a long common run with AFTER;
  - `in-sidecar`: every span is inside one marker's entries, with ≥ 85 % word overlap;
  - `moved`: otherwise.
- Every non-kept row was then reviewed by hand, and 63 proposals were overridden.
- The overrides do three things:
  - they re-classify restated current truth from `moved` to `rewritten`, with a chosen anchor;
  - they replace anchors the heuristic caught on a path or a mid-word fragment;
  - they refuse span-less `in-sidecar` matches unless the sentence is verbatim in the named entry. Two
    such matches were nonsense: a bare "added 2026-09-02)." matched to a scaffold entry, and "it is
    neither." matched to an unrelated one.

`../compaction/moved-history.md` holds 174 verbatim passages in 28 groups (49 814 B), and each group uses
the sidecar's entry shape. It holds two kinds of passage:
- every `moved` row (68);
- the BEFORE wording of each `rewritten` row that dropped content the HEAD sidecar does not already hold
  (106).

So a rewrite that drops a dated clause loses nothing to the sidecar either. Rows the sidecar already holds
are not repeated: that is the 9 `in-sidecar` rows, plus the rewrites whose history the sidecar carries.

## Rust gates — deferral voided, run by hand

`gate.py delta --defer-check rust` voided the deferral, because one uncommitted file name (a phase
extract, `security.md`) matched a doc comment in `crates/conductor-verify/tests/jsonrpc_line_bound.rs`.
That comment names `.claude/rules/security.md`, a different file the test does not read, so the hit is a
basename false positive. The letter's one-hit rule was followed anyway. The gate tool does not fire a
`defer`-keyed entry, so both `run` strings were driven by hand, with each exit read from the bare command:
- `cargo nextest run --workspace --profile ci`: exit 0, `1067 tests run: 1067 passed, 0 skipped`;
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0, 0 warning/error lines.

## Wrap addendum (2026-09-24, wrap P2)

The faithfulness review corrected 4 of the 151 judgment rows before apply (see the report's Decisions &
corrections). After the corrections, `check` → PASS: §Established Decisions 37 907 B (margin 208 B) and
§Occupied Resources 37 929 B (margin 186 B). The figures above are /implement's record and stand as that run's
reading.

## Owed at wrap P2, not proven by any gate

The faithfulness review covers the 151 judgment rows that the `ledger-show … --judgment` entry renders:
- 150 `rewritten` rows: does the AFTER anchor restate the BEFORE sentence without re-deciding or
  de-scoping it?
- 1 span-less `in-sidecar` row: does the named entry carry the content?

This item is OWED, and nothing here counts it as met.
