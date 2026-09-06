# Codebase Research — 2026-09-06-coverage-completeness-gate

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11 · **Graph queries:** 3 (`rust` plane, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — **read in full** (60 lines / 45 KB;
  performed as a structural extraction per the host rule for multi-KB single-line entries: headers `:1-38`,
  then the `## Session Additions` entry index `:40-60` by introducer, then offset-bounded full reads of the
  three entries that bind this chunk's Test Commands). No live-**Pulse** leg fires here, but the `--e2e` arm
  boots external processes, which the file governs.
  - **L58 (2026-09-02, operator directive)** — binding on the `--e2e` line. Firing form
    `bash scripts/agent-run.sh run --e2e`, guarded by `CONDUCTOR_MSEDGEDRIVER` (unset ⇒ skip at exit 0).
    **Its exit code cannot distinguish a full pass from a total skip**, because the guard is designed to
    skip at 0 — so the asserted signal is the SPEC LIST: the driven session banner `[webview2 <version>
    windows]` plus per-spec ticks, with passing/skipped tallies reconciled (today exactly two driven-arm
    specs skip, their subject needing a live Pulse; **a third skip means a seed failed**). Every leg that
    boots an external process takes a `tasklist` census **before and after** — the pre-leg baseline is what
    makes the post-leg reading mean anything — and names its STOP form: self-terminating via wdio's
    `onComplete` → `tauriDriver.kill()`; on an INTERRUPTED run,
    `taskkill /F /IM msedgedriver.exe /IM conductor-tauri.exe` plus the node CLI. **Retrieval caveat the
    entry states about itself:** its `paths:` do not cover `crates/conductor-tauri/ui/test/`, so this rule
    does not auto-load for a ui-touching chunk — it is retrieved only by this research-time read.
  - **L49 (2026-08-11, extended ×4)** — an artifact-backed check names the FULL firing invocation and
    verifies freshness for THIS run; a bare harness verb proves nothing. Applied: the artifact and self-obs
    producers below are listed in their firing forms, not as bare verbs.
  - **L42 (2026-06-21)** — for a no-boot-path-change chunk the agent-run smoke is `run`, never
    `status`/`logs` (which need a `run_id` from a prior live run). Applied: no `status` line is listed.

## THE HEADLINE FINDING — the claim is already true, already asserted, and already running in CI

The entry reads as "build a coverage completeness gate." Measured at HEAD, the **assertion exists and is
green**, and CI **already executes it**:

```
cargo nextest run -p conductor-core -E 'test(committed_artifacts_match_the_known_gap) or …'
  PASS  drift::tests::committed_artifacts_match_the_known_gap
  PASS  coverage::tests::matrix_covers_every_accepted_capability
  PASS  drift::tests::the_committed_catalog_matches_the_unbacked_ledger
  PASS  load_envelope::tests::the_committed_catalog_matches_the_committed_envelope
  Summary  4 tests run: 4 passed
```

- `KNOWN_UNCLASSIFIED = &[]` (`drift.rs:38`), so `check_sut_drift` has **already degenerated to the plain
  zero-drift assertion** its own doc-comment describes as the end state.
- CI runs `.\scripts\agent-run.ps1 run` (`ci.yml:68-73`), whose `run` verb executes
  `cargo nextest run --workspace --profile ci` (`agent-run.sh:187`) — a `conductor-core` lib test is inside
  `--workspace`. **The zero-gap equality is therefore CI-enforced today.**

**This sharpens scope §1.1 rather than falsifying it.** "Zero production callers" is exact (9 call sites:
8 in `drift.rs`'s own `#[cfg(test)]`, 1 the `lib.rs:40` re-export). What scope did not state — and P4 must
consume — is that *test-only* does not mean *un-gated* here. So the chunk's real subject is **not** building
verification. It is: (a) whether an already-green assertion should become a *named* gate surface, (b) three
specs that describe a gate the tree does not have, and (c) the absorbed freight.

## Files inspected
- `crates/conductor-core/src/drift.rs` (`:38-160`, `:221-320`) — `KNOWN_UNCLASSIFIED = []`; `UNBACKED_AUTO` = 8 ids (`P-031/033/034/039/041/042/043/044`); `check_sut_drift` at `:79` (editor), `check_scenario_backing` at `:156`. Three drift conditions; message is identity-only, no host paths.
- `crates/conductor-core/src/redact.rs` (`:21-68`) — `ALLOWLISTED_FIELDS`, the decisive obs constraint (below).
- `crates/conductor-core/src/load_envelope.rs` (`:19`, `:63-64`, `:163-170`, `:188`) — `phase_rate_exceeds(occurrences, gap_ms, max_rate)` compares `occurrences * 1000 > max_rate * gap_ms`; exact integer math, no division. Reached only from `phase_breach` (`:189` editor).
- `crates/conductor-emit/src/pii.rs` (`:32-78`, `:88-108`) — 7-variant enum, `all() -> [PiiCategory; 7]`, `PiiCorpus { values: [String; 7] }`, `seeded()`'s 7-element array literal, `field_key()` match.
- `crates/conductor-report/src/coverage.rs` (`:100-140`) — `coverage_rollup` computes `in_scope = rows.len() − out_of_scope`; `summary_line` renders `total · in_scope (breakdown) · out_of_scope`.
- `crates/conductor-cli/src/commands/coverage.rs` (full, 20 lines) — renders, optionally writes, returns `ExitCode::SUCCESS` unconditionally.
- `.github/workflows/ci.yml` (`:68-73`, `:155-172`) — the test step and the prior chunk's gate precedent.
- `crates/conductor-run/tests/journal_conformance.rs` (existence + its CI step) — the gate-placement precedent.
- `contracts/pulse-load-envelope.toml` — `max_sustained_rate_spans_per_s = 10000`, `max_sustained_storm_ms = 600000`; `[[exempt]]` ledger **empty** (zero `^\[\[exempt\]\]` matches).
- `crates/conductor-cli/Cargo.toml` · `conductor-{core,report,emit,run}/Cargo.toml` — the `tracing` dependency roster.

## Graph impact (rust plane, `db_state: fresh`, `probe_hits: null` on every query — no 0-row conclusion drawn)
- **`check_sut_drift`** — 9 call sites: 8 inside `drift.rs`'s `#[cfg(test)]` (`:251`, `:258`, `:271`, `:288`, `:305`, `:312`, `:478`, `:481`) + `crate @ lib.rs:41`. **No production caller.**
- **`check_load_envelope`** — 8 call sites, **the same shape**: 7 tests in `load_envelope.rs` + `crate @ lib.rs:46`. Also production-callerless.
- **`LoadEnvelope::classify`** — 15 call sites: 14 tests + **one production caller, `classify_run @ crates/conductor-run/src/envelope.rs:78`**. This asymmetry is load-bearing for CARRY 4: the *gate* half is test-only, the *caption* half ships.
- **`phase_rate_exceeds`** — 1 call site (`phase_breach @ load_envelope.rs:189`). Narrow blast radius.
- **`coverage_matrix`** (core; disambiguated by `callee_file` from `conductor-tauri`'s same-named command) — 31 call sites across `conductor-cli` (8), `conductor-core` (10), `conductor-report` (8), `conductor-tauri` (2), plus `conductor-cli/tests/cli_smoke.rs:408` and `conductor-run/tests/lamps_fixture.rs:75`.
- **`coverage_rollup`** — 4 call sites: `render.rs:23`, `render.rs:300`, `summary_line @ report/coverage.rs:136`, `crate @ report/lib.rs:9`.
- **`PiiCategory::all`** — 13 call sites: 10 in `pii.rs` tests, 2 in `conductor-emit/tests/pii_payload_corpus.rs` (`:101`, `:140`), **1 cross-crate: `conductor-run/tests/pii_harvest.rs:185`**.
- **`crate_edges`** — `conductor-cli → {core, report, run, verify}`; `conductor-report → core`. **No `conductor-cli → conductor-emit` edge.**

## Patterns detected
- **Gate-as-its-own-CI-step** (`ci.yml:160-168`): the prior chunk runs `cargo nextest run -p conductor-run --test journal_conformance --profile ci` as a named step, preceded by a guard that fails loudly when the subject is absent. Its inline comment states the reason a Rust test was chosen over `jq`: re-listing the schema would be "a third copy" that drifts from the structs and obs-plan. Directly transferable.
- **Identity-only failure messages** (`drift.rs`, `load_envelope.rs`): `drift_message` / the envelope's message are pinned by `drift_message_names_identity_without_host_paths_or_type_names` (`:312`) and `envelope_message_names_identity_without_host_paths_or_type_names` (`:767`). A new gate's message inherits this test shape.
- **Pure-function rollup with the split already implemented** (`report/coverage.rs:107-131`): `in_scope = rows.len() − count(NotConductors)`, `unbacked` passed in rather than read from the const "so this stays a pure function of its arguments". obs-plan §4 `:359`'s denominator split is **already the shipped basis**; a gate need only read it, not re-derive it.
- **Arity fixed by array types, not by convention** (`pii.rs:53`, `:90`): both `[PiiCategory; 7]` and `[String; 7]` are compile-time arities, so an eighth category is a type change, not a data change.

## Conventions to follow
- **Zero-drift assertion already at its end state**: `KNOWN_UNCLASSIFIED = &[]` (`drift.rs:38`) — pass an empty ledger, per `check_sut_drift`'s own doc (`drift.rs:76-77`).
- **Manifest-derived counts, never literals**: `coverage_rollup` derives every number from `rows`; no literal appears (`report/coverage.rs:107-131`).
- **`conductor-cli` raises no telemetry**: it declares **no `tracing` dependency** (`conductor-cli/Cargo.toml` — the only one of the five crates checked without it). Every span on the CLI path comes from the libraries it drives.

## Files to modify
Provisional pending the P4 forks; the caller-threading enumeration below is from the graph result, not memory.

- `crates/conductor-core/src/drift.rs` — if the gate gains a named surface or a distinct message.
- `crates/conductor-core/src/load_envelope.rs` — CARRY 4: `phase_rate_exceeds` (+ its 1 caller `phase_breach`), `check_load_envelope`, `LoadEnvelope::classify`; the doc at `:19`/`:63-64` states the term's meaning and moves with it.
- `crates/conductor-run/src/envelope.rs` — CARRY 4 threading: `classify_run @ :78` is `classify`'s **only production caller**; a signature or semantics change surfaces here.
- `crates/conductor-core/src/redact.rs` — only if the gate's attributes must be allowlisted (see Open question 1).
- `crates/conductor-emit/src/pii.rs` — CARRY 5: enum, `all()` array type, `field_key()` match, `PiiCorpus.values` array type, `seeded()` array literal, and the test **`exposes_exactly_seven_distinct_categories` (`:325`) whose NAME pins the arity**.
- `crates/conductor-emit/tests/pii_payload_corpus.rs` (`:101`, `:140`) and `crates/conductor-run/tests/pii_harvest.rs` (`:185`) — CARRY 5 cross-crate test callers of `all()`; a data-pin, not a call-signature change, so a caller query alone would under-enumerate them.
- `.github/workflows/ci.yml` — one added step in the existing `rust` job, after the test step and not disturbing `ensure_frontend` ordering.
- `contracts/pulse-load-envelope.toml` — CARRY 4: only if the term is renamed; the `[[exempt]]` ledger stays empty either way.
- **Not required:** no manifest edit, no new crate, no new `CONDUCTOR_*` handle, no `Cargo.toml` dependency addition (unless Open question 2 resolves toward a CLI-placed observable, which arch's acceptance forbids).

## Load-bearing equalities — verified, not assumed

1. **Zero-gap holds at HEAD.** `check_sut_drift(manifest(82 ids), coverage_matrix(), [])` → `Ok(())`, measured by the passing run above. The gate would be green on the day it lands.
2. **CARRY 4's mechanism, re-derived at HEAD** (parsed all `scenarios/*.toml` phase blocks): bound = `10000`. Max **dispatch** rate across the catalog = **3.33/s** (`orthogonal-health-domains`, 10 occ / 3000 ms). Max **wire-span** rate = **50.00/s** (`latency-regression`, 180 occ / 180000 ms × 50 samples). For `latency-regression` specifically the gate counts **1.0/s** against a real **50/s** — **exactly the 50× divergence the CARRY states**, and the catalog stays **200× under** the bound on the corrected basis. Both halves of the CARRY verified; **no verdict moves**, so the correction is a truthfulness fix, not a red-gate fix.
3. **CARRY 5's blast radius, verified and EXTENDED.** The CARRY says "the enum, the array type and the corpus, not just a doc line" — confirmed, and it also moves `field_key()`'s match, a test whose *name* pins the arity, and a cross-crate test caller. **One mechanism the CARRY does not name:** `PiiCorpus::seeded` draws all seven values sequentially from one `ChaCha8Rng` (`pii.rs:97-106`), and `all()` is documented as discriminant order = storage order = **draw order**. Appending an eighth variant **last** leaves the first seven draws byte-identical; inserting it mid-order **changes every subsequent corpus value**, breaking determinism against every committed expectation. Append-last is therefore a correctness constraint, not a style preference.

## Open questions
1. **obs-plan §4's Critical Path 6 is unbuildable as written — on three independent axes.** Measured: (a) of its six named fields only `p_ids` is in `ALLOWLISTED_FIELDS`; `missing_p_ids`, `coverage_percent`, `p_id_count_expected`, `p_id_count_found`, `missing_count` are **all absent**, so as attributes they would emit nothing (the `degraded_mode_requested` disqualifier); (b) none of `report.coverage_matrix_generate` / `db.query_all_p_ids` / `report.validate_coverage` is in §11's bounded span-name set; (c) §4's own `conductor-report` row forbids `db.*` widening, contradicting its own `db.query_all_p_ids`. → blocks: **plan-decision** — P4 must choose between allowlisting + widening §11, riding the `message` field, or re-basing §4; all are amendment-class and belong on the P5 review card.
2. **Gate placement is narrowed to one option by two independent constraints.** `conductor-cli` declares no `tracing` dependency, and arch's acceptance forbids a new cross-seam edge — so a CLI-placed gate cannot carry §4's observables without breaking an arch criterion. The `journal_conformance` precedent (a Rust test as its own named CI step) is the only placement satisfying both. → blocks: **plan-decision**, but the answer is effectively determined; P4 states the lean rather than asking.
3. **Does an already-CI-enforced assertion warrant a second gate surface at all?** Given the headline finding, "commit `coverage-matrix.md` + gate on it" (CARRY 2 option a) is the only fork option that adds verification the tree lacks; the other two are spec-reconciliation. → blocks: **plan-decision** — this is the operator's call at P5 and reframes CARRY 2 from an artifact-housekeeping question into the chunk's central one.
