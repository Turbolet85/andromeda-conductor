# Report — 2026-08-16-fingerprint-storm-live-proof

**Chunk:** Fingerprint-storm live proof — the identity triple re-derived against the shipped blake3 derivation, the storm cue thresholds driven past Pulse's Suggested/Autonomous bands and exactly-one-incident coalescing asserted on a live leg; P-074 backed and the two falsified path-insensitivity claims retired
**Date:** 2026-08-16
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:** 14 tracked changed (+233/−97) + 1 new source file + the chunk folder.
  - `crates/conductor-emit/src/exception.rs` (+87/−23) · `crates/conductor-core/src/scenario.rs` (+19/−35)
    · `crates/conductor-core/src/phase_spec.rs` (+4/−1) · `crates/conductor-core/src/drift.rs` (+5/−6)
    · `crates/conductor-run/src/dispatch.rs` (+1) · `crates/conductor-run/Cargo.toml` (+2)
    · `scenarios/fingerprint-storm.toml` (+45/−17) · `scenarios/fingerprint-distinct.toml` (+17/−9)
    · `Cargo.lock` (+1) · `conductor-0.2.0/verification-matrix.json` (+5/−5)
    · route/state/handoff/friction bookkeeping
  - NEW: `crates/conductor-run/tests/storm_harvest.rs` (248 lines, test-only)
  - NEW: `conductor-0.2.0/chunks/2026-08-16-fingerprint-storm-live-proof/` (scope · research · plan · report
    · `evidence/` — leg verdict + 4 harvest/envelope artifacts)

- **Symbols / APIs:**
  - `conductor_emit::FingerprintVariant` gains **`RelativePathVariant`** (public enum variant); `PathVariant`'s
    semantics CHANGED — it now varies the path BELOW its leading segment (was: a relative-path rewrite).
  - `conductor_emit::exception::leading_path_segment` (new private fn) — replaces the briefly-added
    `ABSOLUTE_VARIANT_ROOT` const (see Reverted facts).
  - `conductor_core::FingerprintVariantSpec` gains **`RelativePath`** (serde `snake_case` → `relative_path`);
    `Path`'s doc semantics changed to match.
  - `conductor_run::dispatch::wire_variant` gains the `RelativePath → RelativePathVariant` arm.
  - No new ports, sockets, env vars, IPC methods, or endpoints. No new `CONDUCTOR_*` handle.

- **Crates / modules:** none added or removed. Changed: `conductor-emit`, `conductor-core`, `conductor-run`.

- **Dependencies:** `serde_json` added as a **dev-dependency of `conductor-run`** (parses Pulse's JSONL in
  the test-only harvest). Already a workspace dep — `Cargo.lock` moved **+1 line, a dependency EDGE inside
  `conductor-run`'s existing entry, ZERO new `[[package]]`**. No production dep added, none bumped.

- **Schema / config:**
  - `scenarios/fingerprint-storm.toml` — `p_ids` += `P-074`; `slo_tier` `<20s` → **`<90s`**;
    `[[expected]] Contains "RetryStorm"` **removed** (declare-only); header prose rewritten.
  - `scenarios/fingerprint-distinct.toml` — `slo_tier` `<20s` → **`<90s`**;
    `[[expected]] Absent "RetryStorm"` **removed** (declare-only); two header-prose corrections.
  - `conductor_core::UNBACKED_AUTO` shrinks 9 → 8 (`P-074` removed, same commit as the `p_ids` add).
  - No migrations, no violation-schema change, no run-report envelope field change.

- **Spec-master edits:** none applied yet — P2 owns them. The chunk's Expected amendments (plan
  §Implementation notes) are: `architecture.md` §Established Decisions [Read-Back Dependency Posture] ·
  `test-plan.md` §6 · `obs-plan.md` §4.

- **Counts / qualifiers moved:**
  - `UNBACKED_AUTO` **9 → 8** — the `conductor coverage` roll-up's `(N unbacked)` auto qualifier moves with
    it. Docs stating it: `layout-templates.md` §Surface: cli — Primary screens (the only doc that has held a
    literal sample historically); `obs-plan.md` §4 denominator semantics names the mechanism, not a literal.
  - `slo_tier` for both fingerprint scenarios **`<20s` → `<90s`** — any doc quoting these scenarios' tier is
    stale. Tier SET unchanged (still the closed `<5s`/`<20s`/`<90s`).
  - Workspace test count **597 → 610**.

- **Dev-tool versions:** none installed or upgraded.

- **Reverted / negative API facts:**
  - `ABSOLUTE_VARIANT_ROOT: &str = "/opt/build"` was written, then REMOVED before the gates went green. It
    encoded the plan's absolute-path mechanism, which measurement falsified (below). Replaced by
    `leading_path_segment`. Nothing absolute reaches the wire — Conductor's frames stay relative and
    `stacktrace_carries_no_absolute_host_path` still holds.
  - No shipped log-harvest seam: the harvest is **test-only** (operator decision at phase P4). Conductor's
    production code reads no SUT log; a tenth workspace crate was considered and rejected.

- **Spec claims disproved by measurement:**
  1. **P-017 clause (c) / `architecture.md` §RBDP — "only ABSOLUTE paths are stripped, so a RELATIVE path is
     identity-significant".** MEASURED narrower: Pulse's `is_absolute_path_start` fires on **ANY `/`**
     followed by a path char, not only a leading one, so `skip_absolute_path` consumes everything from the
     first slash. Only the **leading path segment** is identity-significant — `src/worker.rs` and
     `src/anything/else.rs` are ONE identity; `other/worker.rs` is another. Byte-verified identical in
     `andromeda-pulse crates/buffer/src/fingerprint.rs:139-218` @ `d090314`. Evidence: probe measurement
     (base → `at fn (src)`, absolute variant → `at fn ()`), pinned by
     `exception.rs::only_the_leading_path_segment_reaches_the_preimage`. **The PRIOR chunk's own amendment
     wording states the superseded form and must be swept too.**
  2. **`test-plan.md` §6 Scenario: Fingerprint-storm — "envelope `fingerprints` field populated".**
     MEASURED impossible under deterministic L4: `observation.fingerprints` is fed solely by
     `fingerprint_refs` (`extract.rs:114` → `lib.rs:413`/`:456`), which the L4 fixture pins `[]`. Both live
     legs returned `fingerprints: []`.
  3. **The scenarios' read-back token checks are structurally ungradeable.** `retrieve_report` returns
     `degraded_mode: true` PERMANENTLY under deterministic L4 (computed `parsed_l4.is_none()`; the fixture is
     never parsed as an L4 output), so `markdown` carries no scenario-specific token. `Contains "RetryStorm"`
     always failed; `Absent "RetryStorm"` always passed **vacuously**. Both retired to declare-only, assertions
     moved to the harvest surface (`cue_kind = "retry_storm"`).
  4. **`slo_tier = "<20s"` unattainable by construction** for both scenarios: `latency_ms` spans
     `read_back_observed_at − journal_emitted_at` while the scenarios' own emission windows are ~24 s and
     ~30 s. Measured 24 257/24 258 ms and 30 192/30 187 ms. Re-calibrated to `<90s` under the acceptance's
     own re-calibration clause.
  5. **`v2-11` acceptance's ">60 s separation" premise INSUFFICIENT** for same-dir back-to-back legs — a
     second, independent mechanism blocks them (Pulse dedupes against any OPEN incident, **not** by
     fingerprint). Already recorded as a PREMISE-CORRECTION in the matrix `notes`; no text weakened.

- **Coverage of new surfaces:**
  - `crates/conductor-run/tests/storm_harvest.rs` (test-only harvest of a SUT-owned log) → validation
    `n/a` (test-only, no external input to the product) · instrumentation `n/a` (no shipped seam) · PII
    `redacted✓` (nothing harvested reaches any Conductor artifact — verified: no host path, struct name, or
    SUT log content in journal/report/`runs.db`) · tests `unit ✓ 12 predicates` · a11y `n/a` · tokens `n/a`
  - `FingerprintVariant::RelativePathVariant` / `FingerprintVariantSpec::RelativePath` (config surface) →
    validation `garde✓` (rides the existing `[phases.emission]` `dive`, non-empty variant list, bounded
    `occurrences`) · instrumentation `n/a` · PII `n/a` · tests `unit✓` · a11y `n/a` · tokens `n/a`

## Deviations from intent

1. **Plan step 1's mechanism was falsified and corrected; the goal was preserved.** The plan specified
   reshaping `PathVariant` to an ABSOLUTE path so normalization strips it. Measurement showed that yields a
   DIFFERENT preimage from a relative-path base (`at fn ()` vs `at fn (src)`), because the base's leading
   segment survives. Shipped mechanism: vary the path BELOW its leading segment. Justification: the plan's
   goal (the triple shares one fingerprint, product-faithful, golden-free) is met exactly; only the stated
   mechanism was wrong, and the corrected one is strictly more faithful to the measured SUT semantics. Both
   committed golden families stayed byte-identical, as the plan required.

2. **`crates/conductor-core/src/scenario.rs` edited though not in the plan's Files-to-modify.** Its rstest
   cases pin the fixtures' exact `p_ids`, `expected` shape and tier, so the intentional scenario changes
   necessarily moved them. Judged in-scope under the gray-area rule (a guard co-located in a crate already in
   the modify set, asserting the very fixtures the chunk changed) rather than soft-exiting.

3. **The stale-doc surface was 4 sites, not the 1 the plan named.** Beyond `PathVariant`'s doc comment,
   `Frame`'s struct doc, `Frame.file`'s field doc and `FingerprintVariantSpec::Path`'s doc carried the same
   falsified claim. All corrected; the plan's enumeration was narrower than the defect.

4. **A second re-calibration round ran after the first live leg** (operator-approved): tier `<20s` → `<90s`
   and the read-back token checks retired to declare-only. Not in the original plan text — it is the
   acceptance's own "re-calibrated where live behavior proves them wrong" clause, executed once live
   behavior proved them wrong.

5. **Live legs used a FRESH DATA DIR PER LEG**, not the planned ">60 s quiet on one dir" (operator-approved).
   Justification: the plan's hazard analysis was correct but incomplete — see disproved claim 5.

## Decisions & corrections

- **Operator correction (recipe count).** My P3 research claimed the live-leg recipe is FIVE items, sourced
  from the working-route entry alone. The operator corrected it to SIX: the route text was authored
  2026-08-13, while `.claude/rules/verification-harness.md:47` carries the accumulated chain and adds a sixth
  at 2026-08-14 (paired `RUST_LOG=info,conductor_emit=debug`). **Rule: for the live-leg/harness domain read
  the path-scoped rules file, not the route entry — route text is a point-in-time citation, the rules file
  accumulates.** The correction direction was inverted (I was "correcting" a dictated fact that was right).
- **Operator decision (phase P4):** product-faithful variant reshape over dropping `path` from the mix or
  raising counts; log-harvest kept test-only rather than a shipped seam or a tenth crate.
- **Operator decision (phase P5 review):** fold `fingerprint-distinct` into the leg so `v2-11`'s distinctness
  clause is proven live rather than demoted to unit tier.
- **Operator decision (post-leg):** execute the acceptance's re-calibration clause, then re-run and record.
- **Correction to my own arithmetic premise (phase P3).** The operator's directive concluded the family leg
  "FAILS as declared"; every component fact verified true but the conclusion did not — both phases pace
  across their own 12 000 ms gap, so 18 occurrences land inside the 30 s window and the base fingerprint
  reaches 12, clearing Autonomous. The real defects were different (two fingerprints, boundaries off, an
  unnamed second cue).
- **`RUST_LOG` placement discipline.** The sixth recipe item may ride only invocations that run no test
  suite; `agent-run.sh run` executes nextest + doctest + clippy before its SCENARIO leg, so the scenario legs
  use the direct CLI form the script itself invokes.
- **Vacuous-green finding.** An `Absent` check against a surface that CANNOT carry the token passes
  vacuously — a false green, more dangerous than the `Contains` side's false red.
- **Pulse-visit candidate #4** recorded: incident dedupe is not keyed on fingerprint (an open incident
  absorbs a later, differently-fingerprinted storm). A product-design question, not a Conductor fix.

## Outcome

**Acceptance criteria: met.** All five `v2-11` clauses measured on fresh artifacts (fresh data dir per leg):
identity (one fingerprint `cbe26ad3` across 18 occurrences), distinctness (zero storm lines from sub-floor
variants), thresholds (`retry_storm/suggested@5` → `retry_storm/autonomous@10`), coalescing (exactly ONE
incident on a clean corpus; the scenario storm `created:false, deduped:true`), and the re-calibration clause.
`cbe26ad3` reproduced byte-identically across two data dirs and two Pulse instances — determinism confirmed
against the live SUT.

**Gates green** (`plan.md` §Test Commands): `cargo nextest run --workspace --profile ci` **610/610**, 0
retries · per-crate `-p conductor-{emit,core,run,timeline}` · `cargo test --workspace --doc` 0 ·
`cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo deny check advisories bans licenses
sources` **true exit 0** over the new lock · all four committed snapshots **byte-identical**, no `.snap.new`.

**`cargo audit` — 24th consecutive red**, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true
exit 1. Advisory-DATABASE fault; no released tool version parses it, so no floor exists to raise. **Basis
re-stated in FULL form because the lock MOVED:** +1 line, a dependency EDGE inside `conductor-run`'s existing
entry, **zero new `[[package]]`** ⇒ audit surface unchanged; `cargo deny` verified green as the overlap.

**Smoke:** Pulse-free mint-then-read on both edited scenarios (fresh `run_id`, read back as the same id,
`p_ids ["P-017","P-018","P-074"]` proving the backing survives config → garde → manifest membership →
envelope), then the two operator-gated live legs above. Pulse stopped, `:4317` released, SUT untouched at
`d090314`.

**Host residue:** three throwaway Pulse data dirs remain under `%LOCALAPPDATA%\Temp\` — the permission layer
denied both `rm` attempts. Inert, outside the repo, operator's to delete.
