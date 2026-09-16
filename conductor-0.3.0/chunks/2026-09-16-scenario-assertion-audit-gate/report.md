# Report — 2026-09-16-scenario-assertion-audit-gate

**Chunk:** Scenario-assertion audit gate — one mechanical re-runnable check establishing both epoch-2
outcomes (no committed scenario declares an unsatisfiable assertion · every scenario's tier is
accounted for), registered as a build gate and demonstrated able to fail as well as to pass.
**Date:** 2026-09-16
**Commits:** none since `last_wrap` — the tree carries this chunk uncommitted; HEAD is
`05a7c3f feat(2026-09-15-scenario-tier-honesty)`, the prior wrap's commit, still unpushed (1 ahead of
`origin/build/conductor-0.3.0` at this write; this wrap's commit leaves it 2).

## Changes (structured — detectors read this)

- **Files:** 4 new, 4 source/config modified, 5 pipeline-bookkeeping.
  - NEW: `contracts/scenario-audit-ledger.toml` (91 L) · `crates/conductor-core/src/scenario_audit.rs`
    (266 L) · `crates/conductor-core/tests/scenario_audit_gate.rs` (240 L) ·
    `crates/conductor-core/tests/fixtures/wrapped-gloss.toml` (20 L) — 617 L total.
  - MODIFIED: `crates/conductor-core/src/lib.rs` (+7/−1, module decl + two re-export lists) ·
    `crates/conductor-core/src/scenario_catalog.rs` (+16) · `crates/conductor-core/src/error.rs` (+6) ·
    `.github/workflows/ci.yml` (+11).
  - BOOKKEEPING: `master-route.md` (+1, the promotion record) · `working-route.md` (±1, the freeze
    stamp) · `verification-matrix.json` · `friction-log.ndjson` · `session-handoff.md`.
  - Basis: `git diff --stat HEAD` → 9 files changed, 68 insertions(+), 6 deletions(−) over tracked
    files; `wc -l` over the four untracked new files → 617.

- **Symbols / APIs:** six new public items, all `mod`-private and re-exported by explicit list from
  `crates/conductor-core/src/lib.rs` (the crate root exports by list — a crate-local `tests/` binary
  sees only what the root exports):
  - `check_scenario_audit(&ScenarioAuditLedger, &[Scenario]) -> crate::Result<()>` — the gate.
  - `ScenarioAuditLedger` (+ `default_path() -> PathBuf`, `load(&Path) -> crate::Result<Self>`).
  - `LiveAssertion { scenario, ground, discriminates, weakness }` · `OverTier { scenario, reason }`.
  - `carries_retired_gloss(&str) -> bool` — the comment-marker-stripped, wrap-tolerant sweep.
  - `load_catalog(&Path, &CapabilityManifest) -> crate::Result<Vec<Scenario>>` — an ADDITIVE sibling of
    `list_scenarios`, which returns identity-only summaries carrying no phases.
  - `CoreError::ScenarioAudit(String)` — a new named variant beside `SutDrift` / `UnbackedCoverage` /
    `LoadEnvelope`.
  - **Remaining-caller fact:** `list_scenarios` is UNCHANGED in signature and KEEPS all **six** of its
    call sites — `drift.rs:362`, `lib.rs:69`, `scenario_catalog.rs:81`, `commands.rs:148`,
    `commands.rs:390`, `main.rs:23`. Not a sole-caller change and not a threading change; nothing was
    re-plumbed. Basis: the bare code-graph query `callee_name = 'list_scenarios' AND callee_kind =
    'fn'` on the `rust` plane, `rows: 6` (trace
    `.andromeda/runs/2026-09-16T07-13-13-phase/tree-query-2026-09-16-scenario-assertion-audit-gate.json`).

- **Crates / modules:** `conductor-core` gains one module, `scenario_audit`. No workspace crate added,
  removed or renamed; no cross-seam dependency edge added (every member already depends on
  `conductor-core`).

- **Dependencies:** **none.** `Cargo.lock` untouched, 562 packages before and after. Basis:
  `git status --porcelain Cargo.lock` prints nothing; `grep -c '^\[\[package\]\]' Cargo.lock` → 562.

- **Schema / config:** one new committed TOML, `contracts/scenario-audit-ledger.toml` — header
  (`sut_version`, `sut_head`, `captured_at`, `provenance`) plus two arrays: `[[live_assertion]]`
  (`scenario`, `ground`, `discriminates`, `weakness`) with **2** rows, and `[[over_tier]]` (`scenario`,
  `reason`) with **11** rows. No migration, no config key, no violation-schema or scrub/redaction shape
  change.

- **Spec-master edits:** none — this report is authored before P2.

- **Counts / qualifiers moved:** the workspace test count moved **979 → 986** (+7, exactly this
  chunk's new arms; basis: entry 3's summary line `986 tests run: 986 passed, 0 skipped`, against the
  prior wrap's recorded 979). **No spec master states it** — `grep -c '979'` returns **0** on each of
  the seven (`architecture`, `security-plan`, `design-system`, `layout-templates`, `test-plan`,
  `obs-plan`, `a11y-plan`); the single repo hit is `master-route.md:143`, the prior chunk's
  append-only historical record, which is correct as history and must not be edited. No scenario count
  moved (36 before and after) and no live-`[[expected]]` count moved (2 before and after) — this chunk
  added no scenario and changed no declaration. **none — verified.**

- **Dev-tool versions:** none — no host tool was installed, upgraded, or read at a changed version this
  chunk.

- **Harness / gate surface:** `.github/workflows/ci.yml` gains ONE named step,
  `Scenario-assertion audit gate`, in the `rust` job (22 → 23 steps), placed after
  `Coverage-completeness gate`. Its `run` opens with a presence guard on
  `contracts/scenario-audit-ledger.toml` emitting a `::error::` annotation and exiting 1, then invokes
  `cargo nextest run -p conductor-core --test scenario_audit_gate --profile ci`.
  **Registration is VERIFIED, not asserted** — the workflow parses clean after the edit; the step's
  keys are exactly `{name, run, shell}` with `shell: bash`, no `if:` and `continue-on-error: false`, so
  it genuinely fails the build. Basis: the operator's wrap directive, which pre-verified this reading;
  independently corroborated at implement by a `yaml.safe_load` of the file plus a key-set read of the
  step. **No change to** `scripts/agent-run.{sh,ps1}` (no sixth command), no xtask verb, no
  status/verdict shape, no `[[gate]]`-block change. **The step has never executed in its target
  environment** — its first run necessarily follows the operator's push (see Outcome, and the P5 pin).

- **Cross-project / external claims:** YES. The ledger's grounds are readings of `andromeda-pulse` at
  HEAD **`83d4060`**, recorded in the file's own `sut_head` so a SUT rendering change rots them
  visibly. The claim: under the deterministic interpretation mode, the canned L4 payload is returned
  regardless of prompt and pins severity `autonomous`, which maps to incident severity `Error` and
  renders the label `error` into every report — so both surviving `Contains "error"` assertions are
  satisfiable but non-discriminating. Basis read in that repo:
  `pulse-app/src/deterministic_inference.rs:18,42,126` · `pulse-app/src/inference_runtime.rs:697-703,
  784, 847-868` · `crates/interpretation/src/schema.rs:121-128` ·
  `crates/interpretation/src/markdown.rs:134, 257, 359-366` · `crates/triage/src/contract.rs:239-244`.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:** **none.** The one mechanism claim this chunk leaned on
  (the carried finding that both surviving assertions match a fixture-pinned token) was re-derived
  TRUE at SUT HEAD, not disproved. The plan's forecast of 11 `over_tier` rows was measured and
  **agreed** (11). `verification-matrix.json#v3-05`'s existing dated note — that the post-change
  over-tier set is eleven rather than the nine its acceptance appositive names — is CORROBORATED by
  this chunk's independent computation; it was already written by the prior wrap and needs no new
  disposition here.

- **Expected amendments (from plan):** three entries, each CARRIED with its fact stated in Changes
  above and its owner located by search.
  - `architecture.md` §Occupied Resources — register `contracts/scenario-audit-ledger.toml` (new
    on-disk artifact), the new `conductor-core::scenario_audit` module, and the new `ci.yml` gate step
    in the CI gate-set enumeration. **Carried**; fact in *Files*, *Crates / modules*, *Schema /
    config* and *Harness / gate surface*. Owner located: `grep -c 'cargo fmt --all --check\|Formatting
    gate\|fmt gate' .andromeda/architecture.md` → **4** hits (the gate-set enumeration the 2026-09-09
    fmt-gate fan-out touched); `grep -c 'coverage-completeness\|Coverage-completeness'` → **2**;
    `grep -c 'contracts/pulse-load-envelope.toml'` → **1** (the Occupied-Resources artifact row the new
    ledger joins).
  - `security-plan.md` §Input Validation — record the gate test binary as a new READER of a committed
    manifest resolving from `CARGO_MANIFEST_DIR`, joining `conductor-report --test coverage_gate` in
    that row. **Carried**; fact in *Symbols / APIs* and *Schema / config*. Owner located:
    `grep -c 'CARGO_MANIFEST_DIR' .andromeda/security-plan.md` → **1** hit (the reader row).
  - `test-plan.md` §6 — record the scenario-audit gate beside the coverage-completeness gate as the
    second static gate over committed data; §9 takes NO stage-table row, following the
    `2026-09-06-coverage-completeness-gate` operator resolution. **Carried**; fact in *Harness / gate
    surface* and Outcome. Owner located: `grep -c 'Coverage-matrix completeness gate\|coverage-
    completeness' .andromeda/test-plan.md` → **2** hits.

- **Coverage of new surfaces:**
  - `contracts/scenario-audit-ledger.toml` (committed data, read by one test binary) → validation
    **✓** (serde typed parse + hard `CoreError` on absent/malformed; `default_path()` is a hard-coded
    relative constant with no `CONDUCTOR_*` override, and `load()` takes an already-resolved path —
    garde `n/a`: not a scenario-config struct) · instrumentation **n/a** (no runtime operation) · PII
    **n/a** (Conductor owns no PII; host-path sweep over the file returns 0 hits) · tests **unit ✓**
    (the 7-arm gate) · a11y **n/a** · tokens **n/a**.
  - `conductor_core::check_scenario_audit` + `load_catalog` + `carries_retired_gloss` (new public API,
    library-internal, no external input of their own) → validation **n/a** (pure over already-validated
    values) · instrumentation **n/a** (static gate; no span name minted, per obs-plan §11's bounded
    set) · PII **n/a** · tests **unit ✓** · a11y **n/a** · tokens **n/a**.
  - `.github/workflows/ci.yml` step `Scenario-assertion audit gate` (new gate surface) → validation
    **n/a** · instrumentation **n/a** (job-log readable, nothing routed into a telemetry artifact) ·
    PII **n/a** · tests **✓** (the step RUNS the unit gate) · a11y **n/a** · tokens **n/a**.
    Its first execution in the target environment is **unrunnable-here** — a CI run follows the push.

## Deviations from intent

**None.** All nine Implementation Steps executed as written, within the plan's touchpoint lists; no
gray-area scope judgment ran and no step was reordered against its stated build order (the plan's own
order-sensitivity note directs building `load_catalog` before the gate function that consumes it, and
that is what happened).

## Decisions & corrections

- **Operator directive at this wrap.** The CI-registration reading was PRE-VERIFIED by the operator and
  recorded here on that basis rather than re-derived. The first-CI-run ownership item was directed to
  P5 route-resolve, with the operator's lean being a PREREQ on the next markerless entry; `plan.md` was
  explicitly not to be re-opened (implement is finished and the light gate re-runs entries as written).
- **Operator sharpening at P4.** A proposed "flag it as a route candidate" note for the carried
  mutation-gate item was rejected as a carry, and the pipeline's armed-orphan form named instead
  (`route-resolve.md:24` — four dispositions with a lean), so the halt arrives armed rather than as
  prose someone must interpret.
- **Operator correction at P4, recorded.** The mutation-gate repair is NOT small in full: reading
  `timeout.txt` is a patch, but rostering the fifteen timeouts is a per-item disposition — judgment,
  not a patch — and folding it here would mix that work's outcome with this gate's own measurement.
- **Sweep hazard found this chunk (curation raw material).** A fixture that exists to prove a sweep
  form must DESCRIBE the token it guards, never spell it: this chunk's control fixture explained the
  wrapped-gloss trick by writing the gloss on one line, which made the control's own plain-form
  assertion fail and reddened three gate entries. The rule already exists in `.claude/rules/host-win32.md`
  (2026-09-12) for host-path gates; it did not transfer to a different pattern's artifact.
- **TOML literal strings have no escape mechanism (found at phase P5).** A `baseline` value written
  with a doubled apostrophe inside a `'…'` literal string broke the gate fence (`UNPARSED` at line 5);
  the contract's own rule is that a prose key holding an apostrophe takes a basic `"…"` string.
- **A prose roll-up contradicted its own list (found at phase P4).** Research prose said
  `list_scenarios` has "4 callers" beside a list of five sites; the bare per-symbol graph query returns
  **6**. Corrected in three places; the first corrective sweep then missed a fourth site because its
  pattern keyed on "four of its" while the text read "all four call sites".

## Outcome

**Acceptance criteria, each re-asserted against the DIFF rather than the plan's text:**

1. *(tests)* One re-runnable check establishes BOTH outcomes and is registered as a CI step that fails
   the build — **MET.** `cargo nextest run -p conductor-core --test scenario_audit_gate --profile ci`
   → 7 tests, 7 passed; the ci.yml step exists with `continue-on-error: false`. `v3-06`.
2. *(tests)* SIX in-suite negative arms, each mutating the REAL committed ledger and naming the
   offending scenario — **MET.** Three test functions cover the six conditions (unpinned / rotted /
   lost_subject × the two arrays), each loading the committed ledger and mutating it in memory.
3. *(tests)* No predicate keyed on a literal count; exact-set equality only — **MET.** `Findings::grade`
   uses `BTreeSet` difference/intersection; the only non-emptiness assertions are the positive arm's
   vacuity guards, which bound the subject rather than predicating on a corpus size.
4. *(tests)* The sweep control proves detection where the plain single-line form misses — **MET**, and
   it demonstrated its own sharpness by failing on a defective fixture before it passed.
5. *(tests)* Green under both runners, no retries configured — **MET** (`cargo test -p conductor-core`
   green; `.config/nextest.toml` untouched).
6. *(arch)* Existing member, no new crate, no sixth `agent-run` command, red surfaces as a named
   `CoreError` — **MET.** The diff adds no crate and touches neither harness script;
   `CoreError::ScenarioAudit` is the failure type.
7. *(arch)* No listener, no sidecar, no non-loopback target — **MET.** The module performs
   `std::fs::read_to_string` only; the diff introduces no bind, spawn or network call.
8. *(security)* Hard-coded relative `default_path()`, `load()` on an already-resolved path, no
   `CONDUCTOR_*` override, `e.kind()` only — **MET**; verified against the diff, and the module never
   calls `resolve_under`.
9. *(security)* The one new reader is recorded — **ROUTED**, not yet met: it is an Expected amendment
   owned by P2 (`security-plan.md` §Input Validation). Stated here so the detector sees it.
10. *(security)* Zero absolute host paths and zero internal seam-crate struct names in failure output
    and committed artifacts — **MET.** Sweep: word-anchored `\b[A-Za-z]:[\\/]` plus `/home/`,
    `/Users/`, `%APPDATA%`, `.cargo`, `.rustup` over all four new committed files → **0 hits each**;
    the read fault formats `e.kind()` via `{:?}` and never the path.
11. *(security)* Fixed-program / fixed-argv `run:` step, no shell string, no `Invoke-Expression`, no
    `continue-on-error`, so rule (b)'s six governed spawn forms do not move — **MET** (pre-verified).
12. *(obs)* The tier term consumes `SloTier::deadline_ms()` rather than re-declared thresholds —
    **MET**; the literals 5000/20000/90000 appear nowhere in the new module.
13. *(obs)* Machine-parseable verdict; no host path into `logs/agent-latest.jsonl` or `runs/**.jsonl` —
    **MET**; the gate writes no telemetry artifact at all.
14. *(a11y)* The `a11y` job remains declared, unconditioned, retaining `CONDUCTOR_A11Y_STRICT` —
    **MET**; measured post-edit: no `needs`, no `if`, `runs-on: windows-2025`, the env handle present.
15. *(design/layouts)* No new per-P-ID bracket label, no seventh lamp, no sixth `ReportState`, no new
    ANSI/token entry, no `conductor` verb — **MET**; the diff adds no CLI surface whatsoever.

**Gates** (`[[gate]]` entries by `run` text, in order; the gate tool's own outcome words):

- `cargo nextest run -p conductor-core --test scenario_audit_gate --profile ci` — **green** (exit 0;
  `exit 0` held). 7 tests run, 7 passed, 0 skipped. `new = true`, P5 baseline `red — exit 101, no such
  test target`; the red was absence, not a failing property.
- `cargo test -p conductor-core` — **green** (exit 0). The runner-portability gate.
- `cargo nextest run --workspace --profile ci` — **green** (exit 0). 986 tests run, 986 passed.
- `cargo clippy --workspace --all-targets -- -D warnings` — **green** (exit 0), 0 warnings.
- `cargo fmt --all -- --check` — **green** (exit 0), empty output.
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` — **green** (exit 0, `no
  output` held). The local advisory-db copy is current, so the audit below is trusted.
- `cargo audit` — **green** (exit 0). 1246 advisories · 562 crate dependencies · 7 allowed warnings
  (6 `unmaintained` + 1 `unsound`). Signature UNCHANGED against the 2026-09-10 reading of 7 allowed;
  the advisory count moved 1243 → 1246 on external database movement over a byte-unchanged lockfile.
- `cargo deny check advisories bans licenses sources` — **green** (exit 0).

No entry was deferred, skipped, `recorded`, env-gated, `leg`-gated or timed out; no red was carried.
The gate tool measured `rust` delta 5, so no language was untouched and the source-delta deferral arm
was never available.

**Smoke:** skipped — no boot-path and no UI-surface change. `conductor-core` declares no `[[bin]]`;
every touchpoint is library source, committed data, a test, a fixture or CI config; the plan lists no
`role = 'smoke'` and no `role = 'self-verify'` entry and states that absence in its prose.

**Outcome basis:** implement's P4 report as given (this session held the implement conversation
directly), plus the operator's wrap directive between implement and this report, whose contribution is
the pre-verified CI-registration reading recorded under *Harness / gate surface* and the first-CI-run
ownership item routed to P5. No post-implement artifact was substituted for either.

**Process hygiene:** re-measured here, not recalled. `Get-Process` enumerated **246** host processes,
**0** matching `conductor|pulse|cargo|nextest|msedgedriver|tauri|node` — nothing this chunk's runs
started survives. Implement's own census recorded the same: cargo/nextest/rustc started by its gate
entries, all terminated; nothing else started.
