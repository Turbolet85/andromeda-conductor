# Scope — Coverage completeness gate

**Marker:** `2026-09-06-coverage-completeness-gate`
**Working entry (verbatim title):** _Coverage completeness gate — zero-gap classification over the current
SUT set with every CI gate green_
**Version:** conductor-0.2.0 · Epoch 6b — Polish & ship

---

## 1. What this chunk builds

A **gate surface** for the zero-gap classification claim: the assertion that every capability the SUT
capability manifest accepts is classified into exactly one `CoverageMode`, raised from a crate-internal unit
test to a real gate with the observable contract obs-plan §4 already specifies, and wired so CI is red on
violation while every existing CI gate stays green.

The claim's *logic* already ships. What does not ship is the gate: a surface that fails, an observable that
records what it measured, and a CI step that runs it.

### 1.1 Measured starting state (basis stated per line; measured at HEAD `b2f60c8`, 2026-09-06)

- **`check_sut_drift` has ZERO production callers.** Every call site is inside `drift.rs`'s own
  `#[cfg(test)]` module — `crates/conductor-core/src/drift.rs:250` is the one real assertion
  (`check_sut_drift(&m, coverage_matrix(), KNOWN_UNCLASSIFIED)`); `:257`–`:480` are constructed-fixture
  arms. Basis: `grep -rn "check_sut_drift" crates/ --include=*.rs` — 8 call sites, all in that module;
  the remaining hits are doc-comment references and the `lib.rs:40` re-export. Graph-confirmed at P3:
  9 call sites, 8 test + 1 re-export.
- **[P3 SHARPENING — test-only does NOT mean un-gated, and this reframes the chunk.]** The zero-gap
  assertion is **already executed by CI and already green**. CI runs `.\scripts\agent-run.ps1 run`
  (`ci.yml:68-73`) → `cargo nextest run --workspace --profile ci` (`agent-run.sh:187`), which includes
  `conductor-core`'s lib tests; and `KNOWN_UNCLASSIFIED = &[]` (`drift.rs:38`), so `check_sut_drift` has
  already degenerated to the plain zero-drift assertion. Measured at P3: `committed_artifacts_match_the_known_gap`,
  `matrix_covers_every_accepted_capability`, `the_committed_catalog_matches_the_unbacked_ledger` and
  `the_committed_catalog_matches_the_committed_envelope` all PASS. So this chunk does **not** build
  verification the tree lacks — it decides whether an already-green assertion becomes a NAMED gate
  surface, reconciles three specs that describe a gate the tree does not have, and lands the freight.
  See `research.md` §THE HEADLINE FINDING.
- **`conductor coverage [--write]` is not a gate.** `crates/conductor-cli/src/commands/coverage.rs:12-20`
  renders the table + summary, optionally writes the artifact, and returns `ExitCode::SUCCESS`
  unconditionally — there is no drift assertion on the CLI path. Basis: whole-file read.
- **obs-plan §4's named observables are ALL ABSENT from the tree.** The three spans
  (`report.coverage_matrix_generate` · `db.query_all_p_ids` · `report.validate_coverage`, obs-plan `:135`)
  and the fields (`missing_p_ids` · `coverage_percent` · `p_id_count_expected` · `p_id_count_found`,
  obs-plan `:357-360`) return zero hits across `crates/**/*.rs`. Basis: per-token grep, each verified
  individually.
- **`coverage-matrix.md` does not exist at the repo root and is NOT gitignored.** Basis: `ls` (absent) +
  `git check-ignore -v coverage-matrix.md` (no match). CARRY 2's premise holds unchanged.
- **CI is GREEN at HEAD.** Run `34037552968`, conclusion `success`, 7m20s, `headSha b2f60c8`,
  2026-09-06T13:55:12Z. Basis: `gh run list`. The `rust` job carries 17 named steps and `frontend` 3; the
  entry's "with every CI gate green" therefore starts from a green baseline, and the clause binds this
  chunk's ADDITION — the new gate must be green and must not turn any existing step red.
- **`check_scenario_backing` is shipped and live** (`drift.rs:155`, re-exported `lib.rs:40`, asserted at
  `drift.rs:368` against `UNBACKED_AUTO`). Per CARRY 1 this chunk does NOT rebuild it and does NOT fold it
  into this gate's acceptance.

### 1.2 Directive-supplied fact — re-verified, both directions (overseer, 2026-09-06)

The directive asserts `contracts/pulse-capabilities.toml` is COMPLETE against Pulse HEAD `83d4060`. Folded
as a hypothesis and **re-verified against the artifacts themselves**, since the SUT repo is on disk:

| Coordinate | Directive | Measured | Verdict |
|---|---|---|---|
| Manifest id count / max | 82 / P-082 | 82 distinct / P-082 | ✓ |
| Manifest `captured_at` / `sut_version` | 2026-08-08 | `2026-08-08` / `v0.3.0` | ✓ |
| Pulse HEAD | `83d4060` | `83d4060` | ✓ |
| `docs/v0_2_0/pulse-capability-spec.md` heading ids | 60, unchanged since 2026-06-12 | 60 distinct (P-001..P-060); last touched `21d4de5` 2026-06-12 | ✓ |
| `andromeda-pulse-0.3.0/verification-matrix.json` | P-061..P-082, 21 verified / 1 planned | 22 caps, 21 verified / 1 planned; 22 ids above P-060 | ✓ |
| Conductor `refs/` dir | absent at HEAD | absent | ✓ |
| **Set-diff (the actual claim)** | no id either side lacks | Pulse union = 82; `pulse − manifest` = ∅; `manifest − pulse` = ∅ | ✓ |

**Consequence for scope:** "zero-gap classification over the CURRENT SUT set" is measured against a manifest
that IS current. The gate's subject is **Conductor's own classification**, not a stale transcription — so no
manifest re-transcription is in scope, and a red gate here would mean Conductor's classification drifted, not
that the SUT record aged.

---

## 2. Folded freight — the five CARRYs on this entry

All five arrived via route-resolve on this working line. Each named coordinate was re-verified against the
artifact before it shaped scope; results below.

### CARRY 1 (from `2026-08-09-interpretation-correctness-posture`) — two axes, this entry owns one
- **Verified:** `check_sut_drift` (`drift.rs:79`) and `check_scenario_backing` (`drift.rs:155`) are distinct
  functions on distinct axes; `UNBACKED_AUTO` at `drift.rs:61`. `check_scenario_backing` is live in nextest.
- **Scope effect:** this gate's subject is `check_sut_drift` (classification↔manifest set-equality) ONLY.
  Do not rebuild `check_scenario_backing`; do not fold it into this gate's acceptance.
- **Roll-up constraint (verified at obs-plan `:359`):** the auto term's derived `(N unbacked)` qualifier is
  NOT a fifth summand — a gate reading those fields must keep the four per-mode counts summing to the row
  total.

### CARRY 2 (from `2026-08-09-current-sut-coverage-classification`) — the artifact decision
- **Verified:** `architecture.md:171` registers `coverage-matrix.md` under §Occupied Resources and `:239`
  calls the complete file "the definition of done"; `:226` places it in the directory tree.
  `test-plan.md:80` (Critical Path 6) names it as the gate's surface, and `:39`/`:250`/`:361-362` name it
  again. The file has never existed; `conductor coverage --write` generates it on demand.
- **Scope effect:** **this entry owns the call.** Three options, unresolved here — resolved at P4:
  (a) commit it + gate on it · (b) keep it on-demand and re-word the two specs · (c) gitignore it.
- Only the artifact question is open — the count/mode staleness on that test-plan line was already amended.

### CARRY 3 (from `2026-08-09-out-of-scope-classification-treatment`) — recurring cost + the denominator
- **Verified:** obs-plan `:359` states the split verbatim — `coverage_percent` is computed over the
  **in-scope** count (manifest set minus out-of-scope rows) while `p_id_count_expected` stays the FULL
  manifest count, and the shipped roll-up (`conductor_report::coverage::summary_line` /
  `conductor_cli::render::coverage_summary`) already renders it, so the gate must adopt the same denominator
  or the two silently disagree. Both counts stay manifest-derived; neither is ever a literal.
- **Scope effect:** build the gate to that split, not to a single total. The create-inspect-delete cost is
  recurring, not incidental — whichever way CARRY 2 resolves should address it rather than pay it a third time.

### CARRY 4 (from `2026-08-20-latency-regression-re-proof`) — the rate term measures dispatches
- **Verified at HEAD:** `phase_rate_exceeds(occurrences, gap_ms, max_rate)`
  (`crates/conductor-core/src/load_envelope.rs:166-170`) compares `occurrences * 1000 > max_rate * gap_ms` —
  exact integer math over **`occurrences`**, i.e. dispatches. A `Latency`/`Ramp` phase puts `samples`/`windows`
  spans on the wire per dispatch. The `[[exempt]]` ledger has **zero entries** (grep for `^\[\[exempt\]\]`
  returns none; only explanatory comments remain), so the "re-verified still-empty" precondition holds today.
- **[VERIFIED at P3 — was `[inferred]`; causal-mechanism claim, marker text kept verbatim]:** _"No verdict
  moves today (the catalog sits ~200x under the bound) and the gate/caption shared basis is unaffected, but
  the term as named tells a reader it bounds wire load when it bounds dispatch rate, and a denser scenario
  could breach the real bound while passing the gate."_ **Re-derived at HEAD over all `scenarios/*.toml`
  phase blocks:** bound `10000`; max dispatch rate `3.33/s` (`orthogonal-health-domains`), max wire-span
  rate `50.00/s` (`latency-regression`, 180 occ / 180000 ms × 50 samples) — so `latency-regression` is
  counted `1.0/s` against a real `50/s`, **exactly the 50× the CARRY states**, and the catalog remains
  **200× under** the bound on the corrected basis. No verdict moves. The correction is a truthfulness fix,
  not a red-gate fix.
- **[P3 addition] The gate/caption halves are ASYMMETRIC**, which shapes how CARRY 4 must land:
  `check_load_envelope` has no production caller (7 tests + the `lib.rs` re-export), while
  `LoadEnvelope::classify` has exactly one — `classify_run @ crates/conductor-run/src/envelope.rs:78`.
  Moving them "together" therefore means one test-only symbol and one shipped path.
- **Scope effect:** the fix is Conductor-side — count `occurrences × samples`, or rename the term to what it
  measures. Whichever lands, `check_load_envelope` and `LoadEnvelope::classify` **must move TOGETHER** (they
  read one shared basis) with the `[[exempt]]` ledger re-verified still-empty.

### CARRY 5 (from the 2026-08-22 boundary adaptation) — Pulse's P-047 catalog is now EIGHT
> **[P4 OPERATOR DECISION — DEFERRED OUT OF THIS CHUNK.]** Verified at HEAD (below) but not executed here:
> different crate, different domain, and a determinism constraint of its own. `crates/conductor-emit/src/pii.rs`
> is explicitly NOT a touchpoint. **The verbatim re-CARRY text wrap's route-resolve must append to a later
> markerless entry is authored in `plan.md` §Implementation notes** — folded freight that no chunk executes
> runs in no chunk, so the re-CARRY is the mechanism that keeps it alive. The verification below stands as
> the evidence that entry inherits.
- **Verified at CURRENT Pulse HEAD `83d4060`** (the CARRY cites `70344d5`; re-checked against HEAD, not the
  cited commit): `andromeda-pulse/crates/security/src/scrubber.rs:3` reads "Pattern catalog covering the 8
  P-047 categories" with a live `provider_key` arm (`:107-108`) and its case matrix at `:149-152`
  (`sk_live_…` · `sk-proj-…` · `ghp_…` · `AKIA…`).
- **Verified Conductor-side:** `crates/conductor-emit/src/pii.rs:32` still documents "The seven P-047 PII
  categories Pulse's scrubber must detect and redact" over a seven-variant `PiiCategory`, with
  `all() -> [PiiCategory; 7]` (`:53`) fixing the arity.
- **[VERIFIED and EXTENDED at P3 — was `[inferred]`; causal-mechanism claim, marker text kept verbatim]:**
  _"so an eighth arm moves the enum, the array type and the corpus, not just a doc line."_ Confirmed:
  `PiiCategory` (7 variants), `all() -> [PiiCategory; 7]` (`pii.rs:53`), `PiiCorpus { values: [String; 7] }`
  (`:90`), `seeded()`'s 7-element array literal (`:97-106`), `field_key()`'s match — **plus** the test
  `exposes_exactly_seven_distinct_categories` (`:325`) whose NAME pins the arity, and a cross-crate test
  caller `conductor-run/tests/pii_harvest.rs:185` (13 `all()` call sites total).
- **[P3 addition — a determinism constraint the CARRY does not name]** `PiiCorpus::seeded` draws all seven
  values **sequentially from one `ChaCha8Rng`**, and `all()` is documented as discriminant order = storage
  order = draw order. Appending the eighth variant **LAST** leaves the first seven draws byte-identical;
  inserting it mid-order changes **every subsequent corpus value**, breaking determinism against every
  committed expectation. Append-last is a correctness constraint, not a style preference.
- **Ledger rule (verified, not re-litigated):** **`v2-14` stays VERIFIED** — it measured a state that
  genuinely held against the then-current SUT, and a SUT change invalidates only FUTURE legs. This is owned
  debt, never a re-opened chunk.
- **Scope effect:** without the eighth category Conductor's P-047 claim is structurally narrower than the SUT
  it verifies.

---

## 3. Boundaries

**In scope**
- The `check_sut_drift` zero-gap claim raised to a gate surface with obs-plan §4's observable contract.
- A CI step running it, red on violation, with every existing CI gate staying green.
- The `coverage-matrix.md` artifact decision (CARRY 2) and the recurring create-inspect-delete cost (CARRY 3).
- The load-envelope rate-term correction (CARRY 4), as absorbed freight.

**Out of scope**
- **CARRY 5 (Pulse's eighth P-047 category)** — deferred at P4 by operator decision; re-CARRY text authored in
  `plan.md` §Implementation notes for wrap's route-resolve. `crates/conductor-emit/src/pii.rs` is not touched.
- `check_scenario_backing` / the `UNBACKED_AUTO` ledger — shipped; explicitly excluded by CARRY 1.
- Any manifest re-transcription or SUT re-survey — the manifest is measured current (§1.2).
- Re-opening `v2-14`, or any re-verification of a capability whose leg measured a then-true SUT state.
- New scenarios, new P-IDs, or any live-Pulse leg — this gate is static and needs no running SUT.
- `KNOWN_UNCLASSIFIED` — already retired to `[]`; not this chunk's to revive.

---

## 4. Open forks for P4

**[P3-updated — the headline finding reframes forks 1 and 2.]**

1. **The `coverage-matrix.md` artifact (CARRY 2) — now the chunk's CENTRAL question, not housekeeping.**
   Commit+gate · on-demand+re-word the two specs · gitignore. Contestable: architecture `:239` calls the
   complete file the definition of done, but the file has never existed, so one of the two must move. Given
   that the zero-gap assertion is already CI-green (§1.1 sharpening), **option (a) is the only fork option
   that adds verification the tree lacks**; the other two are spec-reconciliation. Carries CARRY 3's
   recurring-cost consideration.
2. **Gate placement — effectively DETERMINED by measurement, so P4 states a lean rather than asking.**
   `conductor-cli` declares no `tracing` dependency (the only one of five crates checked without it) and
   arch's acceptance forbids a new cross-seam edge, so a CLI-placed gate cannot carry obs-plan §4's
   observables without breaking an arch criterion. The `journal_conformance` precedent — a Rust test as its
   own named CI step — is the only placement satisfying both.
3. **obs-plan §4's Critical Path 6 is unbuildable as written, on three independent axes [P3, NEW].**
   Five of its six named fields are absent from `ALLOWLISTED_FIELDS` (only `p_ids` is present), so as
   attributes they emit nothing; none of its three span names is in §11's bounded set; and §4's own
   `conductor-report` row forbids the `db.*` widening its own `db.query_all_p_ids` requires. Allowlist +
   widen §11 · ride the `message` field · re-base §4 — all amendment-class, all belonging on the P5 card.
4. **Freight coherence (CARRY 4 + CARRY 5)** — both are real, both verified at HEAD, and folded here by
   route-resolve, but neither is a coverage-classification concern. P4 proposes sequencing; P5 is the
   operator's call on whether they ride this chunk or split to their own entries.

---

## 5. Surfaces and contracts touched

- `conductor-core` — `drift.rs` (`check_sut_drift`), `coverage.rs` (`coverage_matrix`), `load_envelope.rs`
  (CARRY 4: `phase_rate_exceeds`, `check_load_envelope`, `LoadEnvelope::classify`).
- `conductor-report` — `coverage.rs` (`CoverageMatrix::render` / `::write` / `coverage_rollup`).
- `conductor-cli` — `commands/coverage.rs`, `render.rs` (`coverage_table` / `coverage_summary`).
- `conductor-emit` — `pii.rs` (CARRY 5: `PiiCategory`, `all()`, `PiiCorpus`).
- `contracts/pulse-capabilities.toml` (read-only subject) · `contracts/pulse-load-envelope.toml` (CARRY 4's
  `[[exempt]]` ledger, re-verified empty).
- `.github/workflows/ci.yml` — one added step in the existing `rust` job.
- Obs contract: obs-plan §4 `:135` (span chain) + `:357-360` (fields + denominator semantics).
- Test contract: test-plan §1 Critical Path 6 (`:80`) + `:250`.
