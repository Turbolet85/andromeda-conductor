# Report — 2026-09-12-ledger-gate-id-space-generalised

**Chunk:** Ledger gate id-space generalised — the verification-ledger gate matches the version id space by
SHAPE rather than a baked `v2-` prefix, so coverage tracks every version on the same basis its directory scan
already does.
**Date:** 2026-09-12
**Commits:** none since `last_wrap` (2026-09-12T10:45:00Z) — this chunk's work is uncommitted until this wrap.

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-report/tests/matrix_ledger_gate.rs` (+61 / −6, `git diff --stat`). Sole source
  file touched; the full porcelain delta beyond it is wrap/phase bookkeeping and the chunk folder.
- **Symbols / APIs:**
  - NEW `is_version_capability_id(&str) -> bool` — a file-private test helper in
    `matrix_ledger_gate.rs`, sibling of the existing `is_pulse_p_id`. Not `pub`, not exported, no crate
    surface: the file is a crate-local `tests/` target, so nothing outside it can name the symbol.
  - CHANGED behaviour (not signature) `requirement_ids(&str) -> Vec<String>` — its filter moved from
    `starts_with("v2-")` to the shape predicate. **Remaining callers: 4, all inside this file** (2 production
    path via `ledgers()`, 2 fixtures in `the_gate_discriminates`) — basis: the P3 code-graph impact query,
    `.andromeda/runs/2026-09-12T10-52-00-phase/tree-query-…-impact.json` `rows: 25`, `db_state: fresh`, plane
    `rust`, every edge in this file. This is a sole-file claim measured, not assumed.
  - No public fn, IPC method, endpoint, export, port, socket or env var added or changed.
- **Crates / modules:** none added, removed or changed. The change lands inside the already-registered
  workspace member `conductor-report`.
- **Dependencies:** none added, none bumped. Package set **562** before and after
  (`grep -c '^\[\[package\]\]' Cargo.lock`); `Cargo.lock` and every `Cargo.toml` un-drifted
  (`git status --porcelain` over them → empty). `rstest` was considered and rejected precisely because it
  would have moved the lock (plan §Constraints).
- **Schema / config:** none. No migration, no config key, no violation schema, no scrub/redaction shape.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** **none — verified.** The workspace test count is **906 before and after**
  (the delta is 905→906 *passed*, i.e. a verdict change, not a selection change), and the file's own test-fn
  count is unchanged at 4 — the new assertions were added inside the existing `the_gate_discriminates`, not as
  new test fns (`grep -c '^#\[test\]'` → 4). No spec master bakes either value: basis
  `grep -rnE '\b9[0-9]{2} (tests|passing|run)\b' .andromeda/*.md` excluding `*-amendments.md` returns only
  `master-route.md` records, which are historical per-chunk facts and correctly immutable.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. No `scripts/agent-run.*` change, no xtask verb, no CI step, no status or
  verdict shape. The chunk adds one `role = 'probe'` entry to its OWN plan's gate fence (a plan artifact, not a
  project harness surface).
- **Cross-project / external claims:** none. The one external-artifact read is the advisory database's local
  currency (`git -C "$CARGO_HOME/advisory-db" status --porcelain` → 0 lines), which qualifies this run's
  `cargo audit`, and asserts nothing about another repo.
- **Reverted / negative API facts:** none shipped-then-removed. (The temporary predicate restoration was a
  measurement, not a candidate surface — recorded under Outcome as evidence and under Deviations.)
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. The gate's own module header claim — that resolving version
  directories by scan prevents a baked literal from silently dropping coverage — was measured CORRECT on its
  own axis; the defect was that the same reasoning had not been applied to the id space. No spec-master
  assertion was measured false.
- **Expected amendments (from plan):** the plan's list is an explicit **none**, and it is re-verified here
  rather than inherited: `grep -rn 'matrix_ledger_gate\|ledger gate\|ledger-gate' .andromeda/*.md` excluding
  `*-amendments.md` returns **1 hit, and it is this chunk's own `master-route.md` record** — no spec master
  describes this gate or its id space. `test-plan §1 Critical Path 6`, which the gate's header cites at `:2`,
  is about the SIBLING coverage-matrix completeness gate over `coverage-matrix.md` (read at
  `test-plan.md:80`), an artifact this chunk does not touch. No master owns a fact this chunk moved.
- **Coverage of new surfaces:**
  - `is_version_capability_id` (file-private test helper, no external surface) → validation n/a (takes no
    external input; its callers read a committed repo-path artifact) · instrumentation n/a (build-time
    verification is Not-instrumentable per obs-plan §1) · PII n/a · tests **unit ✓** (8 direct accept/reject
    assertions plus 3 fixture-driven cases in `the_gate_discriminates`) · a11y n/a · tokens n/a.

## Deviations from intent

- **One, additive: a control run the plan did not list.** After the gates were green I temporarily restored
  `starts_with("v2-")`, ran `the_gate_discriminates` alone, observed it fail, then restored the fix and
  re-ran the headline gate. Justification: the plan's acceptance claims the discrimination arm "fails LOUDLY
  on a mis-admitted input", and an arm passing after the fix is equally consistent with its being vacuous —
  this file's own history is the argument, since `the_gate_discriminates` was green at HEAD before this chunk
  for exactly that reason (its fixtures were `v2-`-shaped, so it exercised nothing the repair changed). The
  control is what separates the two readings. Its output is recorded under Outcome as evidence, not merely as
  a process note. No residue: probe back to `0`/exit 1, workspace 906/906, `cargo fmt --all --check` exit 0,
  porcelain showing one changed source file.
- Nothing else. All eight Implementation Steps executed as written; both plan-listed rejected approaches held
  and neither came under iteration pressure; no step was reordered or skipped.

## Decisions & corrections

- **The predicate is a SHAPE, not a substituted literal** — settled at P4 by the standing de-hardcoding ruling
  (architecture.md §Established Decisions [Accepted Capability Set]; test-plan §6 Selector strategy, "a new
  literal re-stales on the next SUT release"), surfaced independently by the arch and tests distillers. A
  `v3-` substitution and a `{v2-, v3-}` enumeration were both rejected as reinstating the class.
- **"Delete the filter" was falsified by measurement, not by preference** — `requirements.md` bolds prose
  titles beside declared ids (0.3.0: 14 bolded items for 11 ids; 0.2.0: 33 for 32; 0.1.0 bolds a title in all
  31), so an unfiltered read admits non-ids and fails set-equality against matrices that are correct.
- **`rstest` rejected on a measured cost** — it is a workspace dependency but absent from `conductor-report`'s
  `[dev-dependencies]`, so adopting the test-plan's suggested fixture pattern would have added a manifest line
  and moved the lock, against the zero-dependency-delta property three extracts assert.
- **A directive-supplied enumeration was two sites short, and the conclusion it supported still held** — the
  PHASE directive's grep basis anchored `v2-` to a preceding double-quote, returning 11 lines where a bare
  token sweep returns 13; the misses were `:2` (provenance) and `:218`, the fixture INPUT driving the
  assertion on the function under repair. The directive's *conclusion* (the class is one file) re-verified
  true by bare per-path counts. Planning from the stated enumeration would have left `:218` outside the
  modify-set.
- **A curated rule paid for the first time.** Gate 3's baseline red (`cargo test -p conductor-report`, exit
  101) had aborted at the failing integration binary, so the crate's doctest target never ran; the green run
  reaches all four targets. That is `testing.md:90` (2026-09-10) — "a red invocation tells you nothing about
  the targets behind the failure" — firing as designed, and it is what explains the target-count delta
  between the two runs instead of leaving it as an unexplained discrepancy.
- **Operator directive at wrap (4 items):** the `a11y-plan.md:115` duplicate is pinned as a CARRY on `v3-03`
  rather than floated as a residual (no detector covers it, no report fact behind it, so it has no routine
  channel and a residual would leave it ownerless); the discharged BLOCKING annotation is to be stated where
  the annotation lives; the control is to be carried as evidence beside the green; and the `testing.md:90`
  firing is to be noted as the rule working.

## Outcome

**Acceptance criteria — each re-asserted against the DIFF, not the plan's text:**

| # | criterion | verdict |
|---|---|---|
| 1 | `cargo nextest run --workspace --profile ci` exit 0, zero failures | **MET** — 906 run, 906 passed, 0 skipped |
| 2 | `cargo test -p conductor-report` exit 0 | **MET** — all 4 targets (51 + 3 + 4 + 0 doc) |
| 3 | predicate asserted by SHAPE; no version literal in the live path | **MET** — probe `0`/exit 1, and the unit gate passes over the 0.2.0 **and** 0.3.0 ledgers in one run |
| 4 | discrimination arm fails loudly on a mis-admitted input | **MET, and measured** — see the control below |
| 5 | anti-vacuity assertion survives unweakened | **MET** — `assert!(!required.is_empty(), …)` present, unconditional, no `if`/`#[cfg]` guard (read from the diff) |
| 6 | change inside `conductor-report`; no new member / env handle / port / artifact / dependency | **MET** — diff is one crate-local test file; the only env use is the pre-existing `env!("CARGO_MANIFEST_DIR")`; no `std::env`, no `CONDUCTOR_*` |
| 7 | `P-NNN` excluded; required set still derived from `requirements.md` independently of the matrix | **MET** — the predicate rejects `P-017` (asserted directly and via a bolded-P-ID fixture); the two sources remain separate |
| 8 | `cargo fmt --all --check` and `clippy -D warnings` exit 0 | **MET** — both exit 0, 0 warning/error lines |
| 9 | `cargo audit` + `cargo deny` exit 0, local advisory-db currency established first | **MET** — porcelain 0 lines, audit exit 0 (7 allowed), deny exit 0 (advisories/bans/licenses/sources ok) |
| 10 | package set unchanged, `Cargo.lock` committed + un-drifted | **MET** — 562 → 562, stated on the package-count basis |
| 11 | no `CONDUCTOR_*` or other env handle steering the predicate or the scan | **MET** (as #6) |
| 12 | every added/re-worded message renders ids and file names only | **MET** — both messages render `{label}` (the version directory's own name) and a file name; no path, no struct name |
| 13 | no span, no must-trace row, no `tracing` dependency | **MET** — `grep -cE 'tracing\|info_span\|instrument'` → 0 |

**Gates (by `run`, in order, as re-run at the light gate):**

1. `cargo fmt --all --check` — exit 0 ✓ (`exit 0`)
2. `cargo nextest run --workspace --profile ci` — exit 0, 906/906 ✓ (`exit 0`) · baseline red: 906 run / 905 passed / **1 failed**
3. `cargo test -p conductor-report` — exit 0, 4 targets ✓ (`exit 0`) · baseline red: exit **101**, aborted at the integration binary
4. `cargo clippy --workspace --all-targets -- -D warnings` — exit 0 ✓ (`exit 0`)
5. `grep -c "starts_with(\"v2-\")" …` — exit 1, last line `0` ✓ (both `expect` atoms) · baseline red: `1` / exit 0
6. `git -C "$CARGO_HOME/advisory-db" status --porcelain` — 0 bytes ✓ (`no output`)
7. `cargo audit` — exit 0, 7 allowed warnings ✓ (`exit 0`)
8. `cargo deny check advisories bans licenses sources` — exit 0 ✓ (`exit 0`)

No `defer` entry, none deferred under the source-delta rule (this chunk has `.rs` delta, so both expensive
workspace gates ran). No `leg` entry. **Smoke: skipped — no boot-path / UI-surface change**, re-derived
mechanically rather than inherited from the plan's prose: the modify-set is one crate-local test target, the
crate declares no `[[bin]]`, no frontend path is touched, and the plan carries neither a `smoke` nor a
`self-verify` role.

**The control, as evidence for criterion 4.** With `starts_with("v2-")` temporarily restored,
`the_gate_discriminates` fails:

```
assertion `left == right` failed: ids of different versions are extracted alike
  left: ["v2-01"]
 right: ["v2-01", "v3-11"]
test result: FAILED. 0 passed; 1 failed; 0 ignored; 3 filtered out
```

Exit 101. The green alone could not establish this: before this chunk the same test was green over
`v2-`-shaped fixtures and would have stayed green under any repair, correct or not. The pair — green with the
shape predicate, red with the prefix literal — is what makes the arm evidence rather than decoration, and a
reader a version from now needs both readings to trust the gate.

**Verification matrix:** no capability claimed. The unclaimed pool is 10 (`v3-02` … `v3-11`), read in full at
P3; none names the ledger gate. `matrix.py show --chunk` → `claimed … 0`; the coverage gate is a no-op for
this chunk.

**Outcome basis:** /implement's P4 report as given, plus an operator directive issued between implement and
this report. The directive changed what this report CARRIES (item 3 — the control recorded as evidence rather
than only as a deviation; item 4 — the `testing.md:90` firing noted) and where two facts are routed (item 1 —
the `a11y-plan.md:115` duplicate pinned as a CARRY on `v3-03` at P5, not fixed here and not floated as a
residual; item 2 — the discharged BLOCKING annotation stated at the annotation's own site). It changed no
measurement. No post-implement artifact other than the light-gate re-run supports any claim above.

**Process hygiene:** re-measured here, not recalled — `Get-Process` filtered on
`pulse|conductor|msedgedriver|tauri-driver|node|nvda|cargo|rustc|nextest` returns an empty table.

| process | started by | final state |
|---|---|---|
| cargo / rustc / test binaries | this run's gate commands (implement P2 + wrap P7 light gate) | terminated — census empty |
| external processes (pulse-app, drivers, NVDA, node) | none started — this chunk drives no external process | n/a |
