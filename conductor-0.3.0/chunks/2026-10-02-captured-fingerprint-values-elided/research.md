# Codebase Research — 2026-10-02-captured-fingerprint-values-elided

## Scope
- **Depth:** moderate · **Reads:** 7 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/testing.md` and `.claude/rules/verification-harness.md` were read IN FULL, Session Additions included; both auto-loaded on the `crates/**/tests/**` reads. No live leg in this chunk.
  - Applied: the companion sweep greps all of `crates/`, never only `tests/` (testing.md 2026-06-22 as extended 2026-09-10).
  - Applied: the inverse control for an arm green on both sides (testing.md 2026-09-09 as extended 2026-09-12).
  - Applied: no positional nextest filter (testing.md 2026-09-07).
  - Applied: rstest is absent from `conductor-run` (testing.md 2026-09-23).
- **Platform issues consulted:** none. There is no runner-only bullet and no CI-reading entry outside the operator leg; Setup 5a read `31f9d92` green.

## Files inspected
- `crates/conductor-run/tests/real_model_common/mod.rs` (:140-219, :15-21)
  - `elide_fingerprints` is defined at `:157`. A token is fingerprint-shaped when it is a run of ≥8 of `[0-9a-f]`, bounded by non-alphanumerics, holding at least one LOWERCASE LETTER (`:169`).
  - So an all-digit run is never elided. That rule is what lets d3's all-digit `fingerprint_hex=` value survive.
  - `mask_workspace_key` (`:197`) is the in-tree CONTEXT-KEYED precedent: every line starting `workspace=` has its whole value masked "whatever it holds".
  - The module is test-tier and shared. It is `mod real_model_common;` in `real_model_harvest.rs:46` and in `real_model_live.rs:30`.
- `crates/conductor-run/tests/real_model_live.rs` (:150-170, :636-680)
  - The capture PRODUCER `emit_block` (`:157-164`) runs mask → `redact_value` → `mask_host_paths` → `elide_fingerprints` over every printed line. So a fix to the elider also governs every FUTURE capture.
  - The storm line is rendered `fingerprint_hex={value}` (`:660-662`) as `k=value` text, never JSON. Its `scenario_storm=` bool is computed from the RAW hex BEFORE elision (`:666-670`), so a keyed elision cannot change it.
  - The file is `#![cfg(feature = "live-pulse")]` (`:27`); the feature is `live-pulse = []` at `crates/conductor-run/Cargo.toml:28`. It compiles only under that feature.
- `crates/conductor-run/tests/real_model_harvest.rs` (:1420-1450, :1580-1740, :2160-2300)
  - `a_fingerprint_is_elided_and_a_stamp_a_seed_and_a_det_prefix_are_not` (`:1430-1450`) is the elider's synthetic arm. It covers an elided 32-hex and 8-hex, and five kept forms: a nanosecond stamp, a seed, a run id, a short word and an embedded run. It also covers the `det-` prefix.
  - `FROZEN_CAPTURE` (`:1592-1593`) is read by exactly one test, `the_elided_copy_is_the_frozen_capture_through_the_rule` (`:1713-1722`). It asserts `copy == elide(frozen)`, `elide(copy) == copy` and `copy != frozen`.
  - The graded 2026-09-23 form is `ELIDED_CAPTURE` (`:1587`), pinned `d57c2613…` (`:1588-1589`).
  - `the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key` (`:2242-2271`) is the arm that holds the d3 residual EXACTLY: `residual = if d3 { 2 } else { 0 }` (`:2268`). It also asserts `elide(committed) == committed` (`:2251`), which passes on d3 only because the elider passes all-digit runs.
  - The rule section is `:64-452` (`// ---- rule: begin/end ----`). `each_2026_10_01_drive_recorded_the_current_rule_before_it_fired` (`:2224-2237`) holds every 2026-10-01 capture's recorded rule byte-equal to it. Neither the residual arm nor the elider sits inside it.
- `crates/conductor-run/tests/real_model_series/mod.rs` (full)
  - The digest pins. The 2026-10-01 d3 pin is `148da3c5…` at `:94-95`; the 2026-09-30 d3 pin is `93b03a9e…` at `:71-72`. These are the only places they live.
  - `git grep -e 148da3c5 -e 93b03a9e -e d57c2613 -- . ':!.andromeda/runs'` returns these three sites only. No ledger or report records a capture's own digest.
- `conductor-0.3.0/chunks/2026-10-01-…/evidence/attempt-ledger.md` (:50, :106)
  - The ledger's pre-registration digest (`:50`) is of the CONTRACT SECTION, not of a capture, so re-eliding d3 leaves it true.
  - `:106` describes the d3 residual in prose, without the value.
- `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt` (:256-257) and `…/2026-10-01-…/evidence/rm-capture-d3.txt` (:514-515)
  - These are the two residual sites, re-read. `git ls-files --eol` shows all three files `i/lf w/lf attr/text=auto eol=lf`: the two residual captures and the 2026-09-30 elided copy.

## Graph impact
- **elide_fingerprints** — 13 call sites, all in `crates/conductor-run/tests/real_model_harvest.rs`: `:56` (the `use`), `:1433`, `:1443`, `:1446`, `:1719`, `:1720`, `:1831`, `:1834`, `:2055`, `:2251`, `:2466`, `:2467`, `:2480`. The editor lines are the graph's 0-indexed `line + 1`; trace `tree-query-2026-10-02-captured-fingerprint-values-elided.json`, `rows: 13`.
  - The graph does NOT carry `real_model_live.rs:161`. The file is `live-pulse`-gated and the index was built without the feature. Grep finds it (`grep -n elide_fingerprints crates/conductor-run/tests/real_model_live.rs` → `:41` the `use`, `:161` the call), so the true caller set is 14 sites in 2 files.
  - Every `elide(committed) == committed` arm is a caller: `:1834` (the 2026-09-29 series), `:2055` (2026-09-30), `:2251` (2026-10-01) and `:1720` (the elided copy). A keyed widening must therefore be idempotent over the placeholder. A value that already reads `<fingerprint>` must stay `<fingerprint>`, or all four arms go red over captures that hold nothing to elide.
  - `:2466`/`:2467`/`:2480` are the scrub-ordering arms. The elider is the LAST stage, so a keyed branch keeps the order.

## Patterns detected
- **Keyed value masking** (`real_model_common/mod.rs:197-209`): `workspace=` lines keep their key and get their value replaced whatever it holds. The keyed fingerprint elision mirrors it, scoped to the `fingerprint_hex=` key the capture renders.
- **Elided copy + digest pin, frozen original untouched** (`real_model_harvest.rs:1583-1601`, `:1713-1722`): this is the 2026-09-30 remedy route for the frozen file. Under it the frozen file was never edited, which is exactly why site 1 still stands.
- **Exact-count residual pin** (`real_model_harvest.rs:2257-2269`): this is the shape that turns red on a fix. It is re-expressed as "zero un-elided keyed values", never re-pinned to a new count.

## Conventions to follow
- **Digest pins live only in `real_model_series/mod.rs` and the harvest's own consts**, and are graded through `pinned()` → `check_digest` (`real_model_harvest.rs:1564-1567`). A moved pin is a one-literal edit beside its drive.
- **No capture text in test source** (`no_committed_capture_text_sits_in_test_source`). A new arm must not quote a capture line or either residual value.
  - The residual values must not appear in test source at all, and today neither does. The per-value
    `git grep --untracked -l -F {value} -- .` sweep (each value extracted from the two sites at `31f9d92`, never
    printed) lists no path under `crates/`.
- **Plain `#[test]` functions in `conductor-run`.** rstest is not a `conductor-run` dev-dependency (testing.md 2026-09-23), so `#[case]` rows would need a manifest edit.
- **Committed evidence is LF** (`* text=auto eol=lf`). A rewritten capture is written LF, and its pin is computed over the LF-normalized content (`real_model_harvest.rs:1526-1531`).

## New files to create
- `conductor-0.3.0/chunks/2026-10-02-captured-fingerprint-values-elided/evidence/census.md` — the chunk's own census record: the probe, its class definition, its exclusions and per-file counts before and after.

## Files to modify
- `crates/conductor-run/tests/real_model_common/mod.rs` — `elide_fingerprints` gains the keyed branch: a `fingerprint_hex=` value is elided whatever its character class, and an existing `<fingerprint>` is left as it is. Its doc comment is updated to match.
- `crates/conductor-run/tests/real_model_harvest.rs` — three changes:
  - the 2026-10-01 arm's exact residual count becomes zero un-elided keyed values across every capture;
  - the synthetic elider arm gains the keyed all-digit and unkeyed all-digit cases;
  - the frozen-file test is re-expressed per the P4 fork.
- `crates/conductor-run/tests/real_model_series/mod.rs` — the 2026-10-01 d3 sha256 pin moves to the re-elided file's digest.
- `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/evidence/rm-capture-d3.txt` — re-elided: lines 514-515's keyed all-digit value becomes `<fingerprint>`, and every other byte is unchanged.
- `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt` — site 1, elided in place (P4 fork, answered).
- `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/report.md` — its one quote of the frozen site's value becomes `<fingerprint>`.
- `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/plan.md` — its three quotes of the same value become `<fingerprint>`.
- `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/research.md` — its one quote of the same value becomes `<fingerprint>`.
- `.andromeda/runs/2026-09-30T04-48-49-implement/gate-2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir.json` — its quotes of the same value (in recorded gate output) become `<fingerprint>`; the file stays valid JSON.
- `.andromeda/runs/2026-09-30T07-22-03-wrap/gate-2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir.json` — the same edit as the implement trail.

## Open questions
- none. Both plan-decision questions P3 carried were answered at P4 (2026-10-02, AskUserQuestion; the overseer answered for the founder):
  - **Subject: captures plus the residual quotes.**
    - The two capture sites are fixed, and every committed quote of either residual value is replaced. That covers the three prose files and the two gate trails listed above, so the values leave the tree. The per-value sweep lists exactly those files, plus the two capture sites and this chunk's own drafts (since corrected).
    - The outside-class values stay. They are Conductor's own synthetic-exception fingerprints, and several are load-bearing:
      - `severity_harvest.rs:351` asserts cross-line fingerprint identity;
      - `storm_harvest.rs` reads and compares the field (`:25`, `:58`, `:68`);
      - arch [Read-Back Dependency Posture] cites the 2026-09-10 envelopes as its basis.
    - The overseer's note: a verbatim prose quote of the residual value is the same Data Protection residual. History is not rewritten and nothing is force-pushed; git history keeping the original is accepted.
  - **Frozen file: elided in place.**
    - It becomes byte-identical to the 2026-09-30 elided copy, so `copy != frozen` (`real_model_harvest.rs:1721`) inverts.
    - Every citation of its path keeps resolving: the 2026-09-22 `plan.md` names it at `:460`, `:481`, `:484` and `:498`.
    - The frozen-evidence exception is recorded against the founder ruling of 2026-10-02 in the chunk record (overseer's note).
