# Report — 2026-09-14-emit-scrubber-and-percentile-math-under-test

**Chunk:** Emit scrubber and percentile math under test — the `exception.rs` host-path scrubber trio and
`latency.rs` percentile math brought under discriminating test, the ten timeouts classified, and the
survivor floor re-scoped from the audit's `--shard 1/4` to the two files whole
**Date:** 2026-09-15
**Commits:** none of this chunk's — the only commit since `last_wrap` is `d6105af chore(route):
operator-requested adaptation — 0-pending wrap`, which is the prior 0-pending wrap's own bookkeeping. The
chunk's work is uncommitted at report time and rides this wrap's commit.

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-emit/src/exception.rs` (+182/−0 — +181 test-only from implement, +1 this
  wrap's doc-comment re-stamp) · `crates/conductor-emit/src/latency.rs` (+65/−15) ·
  `crates/conductor-emit/tests/pii_payload_corpus.rs` (+4/−36) · `crates/conductor-emit/Cargo.toml` (+2/−0)
  · `Cargo.lock` (+2/−0) · `scripts/mutation-gate.py` (+40/−18) · `scripts/mutation-roster.toml` (+31/−0).
  Basis: `git diff --numstat HEAD` over the seven paths — **+325/−69** at implement's end; the doc-comment
  re-stamp below makes `exception.rs` +182 and the total +326/−69. **Two files are NEW**, both chunk-folder
  artifacts rather than source: `evidence/disposition-ledger.md` (implement) and this `report.md` (wrap).
  No new file lands under `crates/` — notably `crates/conductor-emit/tests/common/mod.rs` was in the plan's
  modify-set but is untouched (see *Deviations* 7).

- **Symbols / APIs:** THREE public methods **REMOVED** from `LatencyProfile` in `conductor-emit`:
  `p50_ms()`, `p95_ms()`, `p99_ms()` (all `pub fn … -> u64`). **No callers remain anywhere in the
  workspace** — basis: `grep -rnE '\.(p50_ms|p95_ms|p99_ms)\(\)' --include=*.rs .` → **0 hits**. This is a
  claim about call sites, not about the identically-named STRUCT FIELDS, which stay and are still read
  in-crate (3 field-access hits, same grep without `()`). No symbol added: every other edit is test-only.
  No IPC method, endpoint, export, port/socket or env var added or changed.

- **Crates / modules:** none added, removed or renamed. `crates/conductor-emit/tests/common/mod.rs` is
  **pre-existing** (last written by `8e268c1`, the previous chunk) and already exported
  `pub async fn start_stub()`; `pii_payload_corpus.rs` now declares `mod common;` and consumes it instead
  of its own copy. No new file was created by this chunk.

- **Dependencies:** `proptest` and `rstest` added to `crates/conductor-emit`'s `[dev-dependencies]`, both as
  `.workspace = true` (already `[workspace.dependencies]` members used by sibling crates). **Lockfile
  package count UNCHANGED at 562** — basis: `grep -c '^\[\[package\]\]' Cargo.lock` → 562. The `Cargo.lock`
  delta is +2/−0 and is EDGES only: `"proptest",` and `"rstest",` appended to the `conductor-emit` package's
  own dependency list. Stated as a package count, never as lockfile byte-identity, per the 2026-09-02
  security Session Addition.

- **Schema / config:** `scripts/mutation-roster.toml` gains a NEW top-level table kind — `[[unit]]`, which
  REGISTERS a gated unit independently of its row count, with an optional `files` list scoping the tier.
  Three units are now declared (`conductor-cli`, `conductor-verify`, `conductor-emit`); `conductor-emit`
  carries a two-path `files` scope (`crates/conductor-emit/src/exception.rs`,
  `crates/conductor-emit/src/latency.rs`). One `[[entry]]` added: `member-emit-1`, `class = "ratified"`,
  `citation_home` → this chunk's `evidence/disposition-ledger.md`. No migration, no violation schema, no
  scrub/redaction shape changed.

- **Spec-master edits:** none at report time. One is EXPECTED and owned by this wrap's P2 — `test-plan.md`
  §12 (see *Expected amendments* below).

- **Counts / qualifiers moved:**
  - **Timeout count 10 → 15.** The plan's acceptance criterion reads "All **ten timeouts** are accounted
    for". The ten were the 2026-09-14 audit's `--shard 1/4` reading (115 of the two files' 227 mutants).
    This chunk's population is **the two files WHOLE (221 mutants)**, which produced **fifteen**. Basis:
    `grep -c . timeout.txt` → 15 in run 2's `mutants.out`. The number moved because the POPULATION moved,
    not because anything regressed. Discharged by the ledger's by-class classification table (six
    `skip_absolute_path`, four `skip_line_number_suffix`, four `skip_hex_address`, one `normalize_frame`
    = 15), not by the plan's wording.
  - **Survivor floor 57 → not comparable.** The audit's 57 survivors are a reading of a strict subset
    (a contiguous shard of 115 of 227), so it is not a baseline this run differences against. Stated in
    the ledger §"Why the population is the two files, not the audit's shard".
  - **Unviable 1 → 3** (see the caveat under *Decisions & corrections* — two of the three are a host link
    race, not a code fact).
  - Docs stating the old values: none outside this chunk's own plan/research and the audit run dir
    (`.andromeda/runs/2026-09-14T18-51-54-code-audit/`), which is a run record and is not edited.

- **Dev-tool versions:** none — `cargo-mutants` **re-read at 27.1.0**, unchanged. Basis:
  `cargo mutants --version` → `cargo-mutants 27.1.0`, matching the floor registered in `test-plan.md:228`.
  No host tool installed, upgraded or moved this chunk.

- **Harness / gate surface:** `scripts/mutation-gate.py` changed in four ways, all operator-local (no CI
  step invokes it; `scripts/agent-run.{sh,ps1}` keep their five-command surface, untouched this chunk):
  1. **Registration is the `[[unit]]` declaration, not the row count.** Previously `if not rows: FAIL — no
     roster rows for this unit`; now `if declaration is None and not rows:` — so a declared unit whose
     survivors are all killed passes on an empty `missed.txt`. A roster predating the `[[unit]]` block
     still registers by rows, so the change is backward-compatible.
  2. **`files` scoping** — each declared path becomes a `-f {path}` argument (workspace-root-relative, per
     `.claude/rules/testing.md`); a declared path that does not exist is a named FAIL before the tier runs.
  3. **0-mutant guard** — `total_mutants == 0` is now an explicit FAIL. `cargo mutants` exits 0 with only a
     WARN on `Found 0 mutants to test`, so without this a mis-declared scope path would read as a clean
     sweep. `complete()` now returns the total alongside its verdict to make this readable.
  4. Verdict semantics otherwise unchanged: the tally decides, never `cargo mutants`' exit.

- **Cross-project / external claims:** ONE. `fingerprint`'s doc comment in `exception.rs` cites
  `andromeda-pulse crates/buffer/src/fingerprint.rs` (`compute_exception_fingerprint`) as its source of
  truth. **Re-measured in this wrap against the SUT repo on disk**, not inherited: SUT HEAD is `83d4060`;
  `git log efabe8e..83d4060 -- crates/buffer/src/fingerprint.rs` → **0 commits**; `git merge-base
  --is-ancestor efabe8e 83d4060` → true; `git log 83d4060..HEAD -- <same file>` → 0 commits. So the
  transcription is CURRENT and the older sha was a **sha age, not a drift**. The doc comment is re-stamped
  accordingly (see *Deviations*). **Scope of the re-stamp: this one site only.** Two other production
  sites carry `efabe8e` (`crates/conductor-core/src/scenario.rs:691`,
  `crates/conductor-run/src/canary.rs:115`) and they cite DIFFERENT SUT files (scenario semantics;
  `crates/triage/src/cue/evaluate.rs`) whose currency across `efabe8e..83d4060` was NOT measured — they are
  deliberately left alone rather than re-stamped on a neighbouring file's evidence.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:** ONE, about the project's own mutation instrument, SURFACED by
  this chunk's run rather than introduced by it. It is not this chunk's debt; it is routed at P5 as a CARRY
  on the entry that next touches the gate.
  1. **The gate cannot enforce the timeout criterion at all.** `scripts/mutation-gate.py` computes its
     verdict from `missed.txt` against the roster (`:137-151`) and **never reads `timeout.txt`** — basis:
     `grep -n 'timeout' scripts/mutation-gate.py` → a single hit at `:3`, inside the module docstring,
     describing cargo-mutants' exit semantics. So a caught→timeout regression passes the gate silently.
     The fifteen timeouts here are classified in the ledger precisely because no gate can hold them.

  **NOT a disproved claim — a measured property of the run, recorded so it is not re-derived as one.** A
  cargo-mutants mutation DESCRIPTION is not unique within a file: **`replace < with <= in quantile` names
  THREE distinct mutants** in `latency.rs` — `:111:10` (the rostered survivor), `:113:17` and `:115:17`
  (both caught) — and the run 2 population holds 12+ such duplicate `(file, description)` pairs. This
  contradicts nothing: `test-plan.md` §12's identity note already states that the roster "matches a run's
  `missed.txt` on `(file, mutation)` as a MULTISET — two members may legitimately share that pair", which
  is exactly what `parse_tally` implements. The gate is sound here for a second reason too: `member-emit-1`
  is a provably equivalent mutant, so it always survives, the multiset count can never fall below 1, and
  any sibling that began surviving would raise the count to ≥2 and FAIL. Recorded, not routed.

- **Coverage of new surfaces:** no new external surface, hot-path op or UI element. The chunk adds tests
  over existing crate-private helpers and REMOVES three public accessors.
  - `LatencyProfile::{p50_ms,p95_ms,p99_ms}` (removed) → validation n/a · instrumentation n/a · PII n/a ·
    tests n/a (removal; the `p50 ≤ p95 ≤ p99` invariant stays enforced in `LatencyProfile::new` and
    test-held) · a11y n/a · tokens n/a
  - `exception.rs` scrubber helpers (`is_absolute_path_start`, `is_token_boundary`, `skip_absolute_path`,
    `skip_line_number_suffix`, `is_hex_address_start`, `skip_hex_address`, `is_path_char`,
    `normalize_frame`, `normalize_stacktrace`) → validation n/a (crate-private, no external input) ·
    instrumentation n/a · PII **redacted✓** (these ARE the host-path scrub path; the new cases cover the
    drive-letter, `%APPDATA%`, `/Users` and `/home` shapes and assert `target` module paths and `::` type
    tokens are NOT redacted) · tests **unit✓** · a11y n/a · tokens n/a
  - `latency.rs` percentile math (`quantile`, `lerp`, `sample_durations_nanos`) → validation n/a ·
    instrumentation n/a · PII n/a · tests **unit✓** (rstest exact-value table + two proptest properties) ·
    a11y n/a · tokens n/a

- **Expected amendments (from plan):** ONE listed, at `plan.md:378-381`.
  - **`.andromeda/test-plan.md` §12 — add the `conductor-emit` accepted-deliberate class.** CARRIED. Its
    motivating fact is stated in *Schema / config* above (the `member-emit-1` `[[entry]]` with its rule and
    `citation_home`) and in *Harness / gate surface* (the registration change). The amendment owes: one member
    (`member-emit-1`), its rule the equivalence argument, its `citation_home` this chunk's
    `evidence/disposition-ledger.md`; and in the same amendment's **Why**, the roster's registration change
    (a unit is DECLARED, never counted), because §12's readers consume the gate's semantics. Search that
    located the site: `grep -c 'accepted-deliberate'` over all seven masters → **`test-plan.md` 7 hits, every
    other master 0**, so no other master is named as owner. Within it, §12 `## 12. Test Decisions Log` opens
    at `test-plan.md:587` and the five existing ratified-class members run `:609-613`; this chunk's entry is
    the SIXTH roster member and appends to that run.

## Deviations from intent

1. **The plan's "ten timeouts" criterion is re-asserted against the DIFF as FIFTEEN.** Justification: the
   criterion's own text binds it to "this chunk's own run under the audit's timeout regime unchanged", and
   this chunk's run measured 15 because its population is the two files whole (221) rather than the audit's
   shard (115 of 227). The criterion is discharged by the ledger's by-class table covering all fifteen, not
   by matching the number ten. Reporting it met at ten would be a criterion met on a stale count.

2. **One production source line changed at WRAP time, outside implement's tests-only scope.** The
   `fingerprint` doc comment in `exception.rs` is re-stamped to name the verified-current SUT HEAD. The plan
   explicitly assigned this to wrap (`plan.md:375-377`: "re-stamping the doc comment's sha is wrap's to do
   or to carry, not a source edit here"), and the wrap directive item 5 directed it. Wording note: the
   comment now reads "transcribed at HEAD `efabe8e` and re-verified unchanged at HEAD `83d4060`
   (`git log efabe8e..83d4060 -- crates/buffer/src/fingerprint.rs` → 0 commits, 2026-09-15)" rather than
   replacing the sha outright — the transcription really was made at `efabe8e`, and only its CURRENCY was
   measured at `83d4060`. Both facts are now on the record; a bare replacement would have asserted a
   transcription event that did not happen.

3. **No verification-matrix entry is claimed or flipped.** The plan states the chunk claims NOTHING
   (`plan.md:288-294`), having read the nine unclaimed ids and found all of them about the scenario corpus,
   a11y, live/dynamic or secret-scanning. Not a deviation from the plan — recorded so the coverage gate's
   `claimed 0 — gate no-op` is expected rather than surprising.

**Implement-time deviations, with the builder's own justifications** (relayed by the operator; no implement
conversation exists in this session — see *Outcome basis*):

4. **Step 6's token set named `%APPDATA%` / `~/.cargo` / `.rustup`, which are NOT `exception.rs`'s remit.**
   That file strips only TOKEN-LEADING absolute paths; those three shapes belong to
   `conductor-core::redact`, the self-observation scrubber, on a surface `normalize_frame` never feeds.
   Justification: every shape actually in remit was covered, and the three that are not were PINNED as the
   deliberate non-scrub boundary (`normalize_frame_leaves_non_token_leading_forms_untouched`) naming the
   owning rule set — so a future widening here is a deliberate change rather than a silent one.
5. **A THIRD change to `scripts/mutation-gate.py` beyond the plan's two** — the `total_mutants == 0` FAIL
   guard. Justification: `Found 0 mutants to test` exits 0 with only a WARN, and `files` scoping — which
   this chunk introduces — is precisely what opens that false-pass path. The guard closes a hole the
   chunk's own change created.
6. **`[[unit]]` declarations added for `conductor-cli` and `conductor-verify`** beyond the
   `conductor-emit` one the chunk needed. Justification: registration moved off the row count, so without
   their declarations those two units' existing invocations would have changed behaviour; declaring them
   keeps every prior invocation identical.
7. **`crates/conductor-emit/tests/common/mod.rs` was listed in the plan's modify-set but is UNMODIFIED.**
   Justification: the shared module already exported `start_stub()`, exactly what the clone fold needed —
   the fold consumed it rather than extending it. Independently confirmed here: the file's last write is
   commit `8e268c1` (the previous chunk) and it is absent from this chunk's diff.
8. **The mutation tier was NOT re-run a third time for a cosmetically clean tally.** Justification: a
   re-run re-rolls which builds lose the host link race, and the evidence for both affected mutants is
   already in hand from run 1 (see *Decisions & corrections*). Recorded rather than hidden.

## Decisions & corrections

- **The unviable count moved 1 → 3, and two of the three are a HOST LINK RACE, not a code fact.** Run 2's
  `unviable.txt` holds `exception.rs:115:9 replace FingerprintVariant::derive -> ExceptionSpec with
  Default::default()` (genuine — `ExceptionSpec` has no `Default`, unviable in both runs), plus
  `exception.rs:286:9 replace && with || in is_hex_address_start` and `exception.rs:340:7 replace += with
  -= in skip_line_number_suffix`. Both build logs end `LINK : fatal error LNK1104: cannot open file
  …conductor_emit-….exe` alongside `Blocking waiting for file lock on package cache` — the Windows linker
  could not open its output because the prior mutant's test binary was still held. **Run 1 tested both and
  found them dead:** `:286:9` is in run 1's `caught.txt` and `:340:7` is in run 1's `timeout.txt` (verified
  directly against both runs' tallies this wrap). Without this line the next audit reads `unviable 1 → 3`
  as a property of the code. The tier was deliberately not re-run a third time for a cosmetically clean
  tally — a re-run re-rolls which builds lose the race, and the evidence for these two is already in hand.

- **Sweep hazard, measured this wrap: a cargo-mutants mutation DESCRIPTION is not unique within a file, so
  grepping one match and reading it as "the" mutant is wrong.** `is_hex_address_start` carries THREE `&&`
  operators (`:285:9`, `:286:9`, `:287:9`), all three sharing the description `replace && with || in
  is_hex_address_start`; `quantile` likewise yields three `replace < with <= in quantile` mutants. A first
  grep hit showed `:287:9` and read as a coordinate discrepancy against the ledger's `:286:9` claim;
  enumerating ALL matches per tally file showed run 1 caught all three and the ledger was exactly right.
  Enumerate every hit with its class before concluding anything about a mutant by description.

- **A dictated coordinate was off and the measured one was used.** The wrap directive cites the fingerprint
  doc comment at `exception.rs:158-160` (as do `plan.md:372` and `research.md:119`); the `efabe8e` token is
  on **line 161** at HEAD. The edit was anchored on the quoted text, not the dictated line number.

- **Decision: the ONE instrument gap is routed, not fixed here.** The timeout blind spot is a real hole in
  the project's own gate, surfaced by this chunk's measurement. Per the wrap directive's doctrine for it,
  it rides a P5 CARRY on the entry that next touches `scripts/mutation-gate.py`, with its measured basis,
  rather than widening this chunk.

- **Correction made during this wrap: a second "gap" was drafted and then withdrawn.** The report first
  claimed the gate's `(file, description)` identity contradicted `.claude/rules/testing.md` and routed it
  as a second CARRY. Reading `test-plan.md` §12's identity note (`:620`) dissolved it — the multiset match
  on `(file, mutation)`, and the fact that two members may legitimately share that pair, are *already the
  documented design*. The underlying measurement (three same-description `quantile` mutants) is true and is
  kept as a recorded property; the conclusion drawn from it was not. Logged because the false finding
  arrived beside a true one and was accepted on that company until the owning section was read.

- **Self-corrected inside the implement loop (relayed):** an assertion the builder first wrote claimed the
  exception event's `time_unix_nano` is at or after its span's `start_time_unix_nano`. Measured, the event
  stamps ~25 µs EARLIER — the order is an artefact of build order, not a guaranteed property — so the
  assertion was replaced with a same-clock PROXIMITY bound. Confirmed here in the shipped test:
  `the_exception_event_carries_a_wall_clock_stamp` asserts a plausible-epoch floor plus
  `abs_diff(...) < ONE_MINUTE_NANOS`, and its doc comment records the ~25 µs measurement and states
  outright that it is deliberately not an ordering assertion. A magnitude floor alone would not have
  separated a dropped field from a real reading; the proximity half is what pins the two stamps to one
  clock.

- **Not re-filed here:** the 14-vs-11 `quantile` survivor-count correction is already queued for the next
  `code-metrics.ndjson` record's `corrections[]` (wrap directive item 7).

## Outcome

**Acceptance criteria, each re-asserted against the diff:**

- (tests) Every survivor killed or ratified against a cited rule; `missed.txt` holds EXACTLY the ratified
  set as a multiset over `(file, mutation)` — **MET.** `missed.txt` holds one line,
  `latency.rs:111:10: replace < with <= in quantile`, which is `member-emit-1`. Gate printed
  `missed 1 · caught 202 · expected 1` and `MUTATION GATE conductor-emit: PASS`.
- (tests) The changed registration arm demonstrated able to FAIL as well as pass — **MET.** Both readings
  are recorded: the undeclared-unit FAIL is the gate's documented baseline (`plan.md` Test Commands
  `baseline`), and a declared unit with an empty roster set FAILs when `missed.txt` is non-empty.
- (tests) All timeouts accounted for — **MET at FIFTEEN, not ten.** See *Counts / qualifiers moved* and
  *Deviations* 1. All fifteen classified by class with their shared mechanism (a mutated loop advance
  leaving an index that never passes its bound; a hang IS detection). None rostered — and none CAN be, as
  the gate does not read `timeout.txt`.
- (tests) `cargo nextest run --workspace --profile ci` AND `cargo test -p conductor-emit` both green —
  **MET, re-run green at P7.1.** Workspace **982 tests run, 982 passed, 0 skipped** (909 → 982, +73);
  `conductor-emit` **150 passed** across its ten targets under the shared-process runner (77 → 150, +73 —
  the same delta, as expected, since every new test is in that crate). No `smoke` entry is owed: the chunk
  drives no external process, no scenario and no SUT, so there is no boot path and no `role = 'probe'`
  liveness entry — smoke n/a.
- (tests) Property test configured with seed control and a `proptest-regressions` path, no retry or
  serialisation knob — **MET.** The two `proptest!` properties draw `seed in any::<u64>()` and feed
  `ChaCha8Rng::seed_from_u64`; no nextest `retries` and no `--test-threads` knob is introduced anywhere in
  the diff. Verified by reading the configuration, not by a counterexample file's presence.
- (tests) Workspace line coverage ≥ 60 % — **MET with wide margin: 94.58 % lines measured** at P7.1
  (`cargo llvm-cov report --fail-under-lines 60 --ignore-filename-regex '[\\/]tests[\\/]'`, exit 0; TOTAL
  row 12139 lines / 658 missed → 94.58 %, regions 93.78 %, functions 91.53 %). **Divergence recorded:** the
  operator's relay carried 94.74 %; this wrap's own run of the plan's gate command measures 94.58 %. The
  criterion (≥ 60) is unaffected either way, and the measured figure is the one reported — neither number
  is silently adopted over the other.
- (security) At least one scrubber case per host-path shape the scrub token set names — **MET.**
  `is_absolute_path_start_fires_only_on_a_token_leading_absolute` and
  `normalize_frame_strips_host_varying_parts_and_keeps_the_rest` cover drive-letter (`C:\`, `C:/`),
  `/home/`, `/Users/`; `normalize_frame_leaves_non_token_leading_forms_untouched` covers `%APPDATA%` and
  `~/.cargo` as the deliberate NON-scrub boundary (that form is `conductor-core::redact`'s remit, on a
  surface this function never feeds) — pinned so a future widening here is deliberate.
- (security) No survivor inside the host-path scrub path or the blake3 `fingerprint` derivation is ratified
  — **MET.** The sole ratified member is in `latency.rs`; `exception.rs` contributes zero roster rows.
- (security) `Cargo.lock` committed and un-drifted, package count unchanged at **562**, `cargo audit`
  exit 0, `cargo deny check advisories bans licenses sources` green over the new lock — count verified
  (562); **all three MET at P7.1** — `cargo audit` exit 0 (1246 advisories · 562 packages · 7 allowed
  warnings), `cargo deny check advisories bans licenses sources` exit 0, and the local advisory-db
  porcelain probe exit 0 with ZERO output, which is what makes the audit green trustworthy rather than
  assumed (the corrected 2026-09-05 rule). The advisory count moved 1243 → 1246 on external database
  movement since 2026-09-10 — re-read here, never carried forward.
- (security) Roster rows and `evidence/disposition-ledger.md` carry zero absolute host paths — re-run at
  P7.1 (the scoped hygiene gate).
- (obs) Scrubber tests discriminate on the host-FILE-path anchor set and assert `target` module paths and
  `::` type tokens are NOT redacted — **MET.** `is_path_char_admits_exactly_the_path_bytes` pins the byte
  class; `normalize_frame_leaves_non_token_leading_forms_untouched` asserts
  `conductor_emit::latency::quantile` and `src/latency.rs` pass through unchanged.
- (obs) `quantile` / `lerp` / `sample_durations_nanos` hold the zero-unlogged-panics invariant over the
  declared domain — **MET.** `quantile_is_total_and_monotone_over_the_unit_interval` and
  `sample_durations_nanos_draws_exactly_n_without_panicking` assert totality (finite, monotone, exact
  length) across the generated domain.
- (obs) `p50 ≤ p95 ≤ p99` remains enforced and test-held after the accessors are retired — **MET.** The
  ordering is validated in `LatencyProfile::new`, which the diff does not touch; the removal is of
  accessors only, and the struct fields remain.
- (obs) No new test target or source edit initializes an OTel SDK, exporter or metrics instrument —
  **MET.** No such symbol appears in the diff.
- (arch) All new tests and the source edit land in `crates/conductor-emit`, no `conductor_core` edge
  introduced including via `[dev-dependencies]` — **MET** by inspection (`proptest` and `rstest` are the
  only dev-deps added); asserted mechanically by the `cargo doc --no-deps -p conductor-emit` gate's
  `lacks conductor_core` atom at P7.1.
- (arch) `cargo check -p conductor-emit --lib` exits 0 standalone on the crate's OWN target — re-run at
  P7.1.
- (arch) The mutation gate remains an operator-local host-Python instrument; no CI step invokes it and
  `scripts/agent-run.{sh,ps1}` keep their five-command surface — **MET.** Neither harness script is in the
  diff, and no workflow file is touched.
- (a11y) No WCAG or a11y coverage claim from this chunk — **MET.** The emit crate is a not-assertable
  surface; nothing in the diff touches a UI element.

**Gates (P7.1 light gate — the plan's 14 `[[gate]]` entries re-run in fence order, each exit read from the
BARE command into `%TEMP%/andromeda-gate/{marker}/wrap-2026-09-15T05-14-43/{n}.log`; ALL GREEN by their
`expect`):**

| # | `run` | verdict |
|---|---|---|
| 01 | `cargo fmt --all -- --check` | exit 0 |
| 02 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| 03 | `cargo check -p conductor-emit --lib` | exit 0 — the member builds standalone on its OWN target after the dev-dep change |
| 04 | `cargo doc --no-deps -p conductor-emit` | exit 0 · `conductor_core` **0 occurrences** ("lacks conductor_core") · **"generated 5 warnings"** — the pre-existing five, unmoved by the doc-comment re-stamp |
| 05 | `cargo nextest run --workspace --profile ci` | exit 0 — **982 run, 982 passed, 0 skipped** |
| 06 | `cargo test -p conductor-emit` | exit 0 — **150 passed** across ten targets (runner-portability gate) |
| 07 | `cargo test --workspace --doc` | exit 0 |
| 08 | `git -C "$CARGO_HOME/advisory-db" status --porcelain` | exit 0 · **no output** — the local checkout is clean, so 09's green measures the world |
| 09 | `cargo audit` | exit 0 — 1246 advisories · 562 packages · 7 allowed |
| 10 | `cargo deny check advisories bans licenses sources` | exit 0 |
| 11 | `python -X utf8 scripts/mutation-gate.py conductor-emit` | exit 0 · `scope: 2 file(s)` · `total_mutants 221 == len(mutants.json)` · `missed 1 · caught 202 · expected 1` · **`MUTATION GATE conductor-emit: PASS`** (`cargo mutants` exit 3, carrying no verdict) |
| 12 | host-path hygiene grep | **exit 1, last line 0** — which IS its `expect`; zero host-path hits in the roster and `evidence/` |
| 13 | `cargo llvm-cov nextest --workspace --profile ci --no-report` | exit 0 |
| 14 | `cargo llvm-cov report --fail-under-lines 60 …` | exit 0 — **94.58 % lines** |

No entry was deferred, none is `leg`-gated, and none is red — so P7.1's not-this-chunk's arm is not invoked.

**The tier reproduced on a THIRD independent run.** Gate 11 re-generated all 221 mutants over the same
two-file scope and landed the identical tally (1 missed / 202 caught / expected 1), with the single survivor
again `latency.rs:111:10 replace < with <= in quantile`. Two corroborations fall out of it. The equivalence
acceptance is not an artefact of one run — the mutant survived again, as an equivalent mutant must. And
**the unviable caveat is now confirmed by a third reading**: `skip_line_number_suffix`'s `+=` → `-=`, which
run 2 recorded UNVIABLE and run 1 recorded TIMEOUT, is a **TIMEOUT** here too (at `:341:7`, one line below
run 2's `:340:7` — the shift is this wrap's own doc-comment edit). A mutant that will not compile cannot
time out; the run-2 classification was the host link race, exactly as the ledger states.

**Outcome basis:** **implement's P4 report reached this wrap only as the operator's RELAY, after a
`/clear` — no implement conversation exists in this session**, so nothing here rests on a remembered trace.
The relay's every figure was re-derived against the post-implement artifacts before being written down, and
each reproduced exactly: `+325/−69` over seven files; lockfile 562; run 2's `1 / 202 / 15 / 3 = 221`; the
single survivor `latency.rs:111:10`; run 1's **12 missed, 11 of them in `exception.rs`** (so the new
boundary tests killed eleven); workspace **982** tests and `conductor-emit` **150**
(132+5+2+1+2+2+3+2+1+0, read from this wrap's own gate logs). Artifacts read directly — run 2's tallies
from
`target/mutation-gate/conductor-emit-20260914T232258Z/mutants.out/` (`missed` 1, `caught` 202, `timeout` 15,
`unviable` 3, `total_mutants` 221, `end_time` set), run 1's from
`conductor-emit-20260914T225338Z/mutants.out/`; the seven-file diffstat from `git diff --numstat HEAD`; the
lockfile count from `grep -c '^\[\[package\]\]'`; the SUT currency from the `andromeda-pulse` repo on disk.
One operator wrap directive arrived between implement and this report: it added the timeout-number
re-assertion, the unviable caveat, the instrument-gap routing, the §12 amendment framing and the
doc-comment re-stamp. Every figure it carried was re-derived here and reproduced exactly (+325/−69,
lockfile 562, 1/202/15/3 = 221, the single rostered survivor).

**Process hygiene:** this chunk's runs started `cargo`, `cargo mutants` (two tier invocations, `--jobs 2`),
`cargo nextest`, and host `python` for the gate — all synchronous, all exited. No sidecar, no SUT, no
driver, no listener: the chunk drives no external process and opens no port. The root `mutants.out/` and
`target/mutants-*` directories an earlier audit left behind are confirmed ABSENT (`ls` → no such file).
Tier output is confined to `target/mutation-gate/{unit}-{stamp}/`, which is gitignored under `target/`.
