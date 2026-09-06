# Report — 2026-09-06-coverage-completeness-gate

**Chunk:** Coverage completeness gate — the zero-gap classification claim raised from a crate-internal unit
test to a real gate surface with obs-plan §4's observable contract, red in CI on violation; carries the
coverage-matrix.md artifact decision, the load-envelope dispatch-vs-span rate term and Pulse's eighth P-047
category
**Date:** 2026-09-06
**Commits:** none since `last_wrap` — this chunk's work is uncommitted at authoring (HEAD `b2f60c8`)

## Changes (structured — detectors read this)

- **Files:**
  - modified: `crates/conductor-core/src/phase_spec.rs` · `crates/conductor-core/src/load_envelope.rs` ·
    `crates/conductor-report/src/coverage.rs` · `.github/workflows/ci.yml`
  - new: `crates/conductor-report/tests/coverage_gate.rs` · `coverage-matrix.md` · `.gitattributes`
  - basis: `git status --short` on the implement tree (3 source `M`, 1 config `M`, 3 `A`/`??`). No formatting
    cascade — no crate root was touched, so every changed file is semantic.

- **Symbols / APIs:**
  - **NEW** `EmissionSpec::max_spans_per_dispatch(&self) -> u64` (`phase_spec.rs:156`, 40 lines to its closing
    brace = `:156-195`; basis: `grep -n` + `awk` line count). An upper BOUND on the OTLP records one dispatch
    puts on the wire, one arm per `EmissionShape` variant.
  - **CHANGED (private, crate-local)** `phase_rate_exceeds` gained a `spans_per_dispatch: u64` parameter
    (`load_envelope.rs:180`). **Sole caller** — `phase_breach` (`load_envelope.rs:210`); basis: the P3
    code-graph impact query, trace `.andromeda/runs/2026-09-06T14-36-59-phase/tree-query-*.json` Q2, rows 81,
    `probe_hits: null`, which returned exactly 1 caller. No cross-crate blast radius (the fn is private).
  - **UNCHANGED signatures with kept callers:** `phase_breach` keeps both its callers — `classify`
    (`load_envelope.rs:246`) and `check_load_envelope` (`:349`); `classify` keeps its one production caller
    `classify_run @ crates/conductor-run/src/envelope.rs:78`. Basis: same graph query, 15 and 8 call sites.
  - **CHANGED (behaviour, not signature)** `conductor_report::coverage_rollup` now emits one `tracing::info!`
    boundary line; its RETURN value is unchanged and still a pure function of its arguments.
  - New test target `conductor-report --test coverage_gate` (3 tests) — `conductor-report`'s FIRST `tests/`
    directory.
  - No new IPC method, endpoint, export, port/socket, or env var.

- **Crates / modules:** none added or removed. Changed: `conductor-core` (`phase_spec`, `load_envelope`),
  `conductor-report` (`coverage`, + a new integration-test target). **No new cross-seam dependency edge** —
  the gate lands in `conductor-report`, which already depends on `conductor-core`.

- **Dependencies:** none added, none bumped. `Cargo.lock` un-drifted (basis:
  `git status --porcelain Cargo.lock` empty). `conductor-report`'s dev-deps are unchanged (`assert_fs` alone).

- **Schema / config:** none. No migration, no config key, no violation schema, no scrub/redaction shape.
  `contracts/pulse-load-envelope.toml` is UNCHANGED — the term keeps its name and its value
  (`max_sustained_rate_spans_per_s = 10000`); only what the code counts against it changed.

- **Spec-master edits:** none applied by implement (implement is read-only on the masters). Two are OWED and
  dispositioned below.

- **Counts / qualifiers moved:**
  1. **The load-envelope rate term's BASIS** moved from dispatches to wire records. Old (stated at
     `architecture.md:174`): "computed `occurrences / gap_ms`". New: `occurrences ×
     max_spans_per_dispatch() × 1000 > max_rate × gap_ms`.
  2. **The catalog's headroom figure** moved `~200×` → **≈43×**, and its worst-case scenario moved
     `latency-regression` (50/s) → **`halo-breathing-encoding` (232/s)**. Basis: a parse of every
     `scenarios/*.toml` phase block under the shipped mapping — the ramp phase declares `from_rate=5,
     to_rate=50, windows=20, occurrences=1, gap_ms=5000` (`scenarios/halo-breathing-encoding.toml:22-29`),
     so the bound is `20 × ceil(50 × 1.15) = 1160` records per dispatch over 5 s = **232/s**, and
     `10000 / 232 = 43.1`.
  3. **Docs stating the old values:** `architecture.md:174` ONLY. Basis:
     `grep -nE 'occurrences / gap_ms|samples./.windows|DISPATCHES|200×|50/s' .andromeda/*.md` → 1 hit
     (that line), and the same grep over `.claude/` + `CLAUDE.md` → **0 hits** (derived tier clean).

- **Dev-tool versions:** none installed or upgraded.

- **Harness / gate surface:**
  1. **NEW CI step** "Coverage-completeness gate" in `.github/workflows/ci.yml`'s existing `rust` job, placed
     between "Test + lint (dogfood agent-run)" and "Supply-chain — cargo audit". Guard (`test -f
     coverage-matrix.md`, loud on absence) → `cargo nextest run -p conductor-report --test coverage_gate
     --profile ci` → a PASS echo counting the classified rows. `ensure_frontend` ordering and every existing
     step untouched. Modelled on the prior chunk's "Run-journal conformance gate" (`ci.yml:160-168`).
  2. **NEW `.gitattributes` — the repo's FIRST** (basis: `grep -rn gitattributes .andromeda/*.md .claude/` →
     0 hits; the file did not exist at HEAD). ONE rule: `coverage-matrix.md text eol=lf`. A **host-portability
     control**: `CoverageMatrix::render()` is `writeln!`-built (LF) and `write()` stores it verbatim, so the
     gate's byte-equality arm only holds where checkout preserves LF. Measured need: `core.autocrlf=true` on
     this host and `git diff` warns `LF will be replaced by CRLF` on every tracked text file; both CI jobs run
     `windows-latest` (`ci.yml:18`, `:178`). Verified in force: `git check-attr text eol -- coverage-matrix.md`
     → `text: set` / `eol: lf`, against an unpinned sibling's `unspecified`; artifact on disk 0 CRLF / 88 LF.
     **Scope note for the operator:** the file governs exactly one path — no repo-wide attribute policy is
     asserted or implied here.
  3. No change to `scripts/agent-run.{sh,ps1}`, no xtask verb, no status/verdict shape.

- **Cross-project / external claims:** none NEW. The chunk's premise — that
  `contracts/pulse-capabilities.toml` is current against Pulse — was re-verified at phase P1 against the
  sibling repo `D:/dev/projects/andromeda-pulse` at HEAD `83d4060` (spec headings 60, 0.3.0 matrix 22 ids
  above P-060, set-diff empty both directions vs the manifest's 82). Unchanged by this chunk; recorded in
  `scope.md §1.2`.

- **Reverted / negative API facts:** the obs `tracing::info!` was first written into
  `CoverageMatrix::render()` and **deliberately moved out** before any gate ran — `render()` is documented
  pure/no-IO, and the CLI's no-`--write` path never calls it, so the line would have emitted nothing on the
  very invocation the plan's Test Command 4 uses. It now lives in `coverage_rollup`, which every path reaches.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  1. **`architecture.md:174` — "a `Latency` / `Ramp` phase puts `samples` / `windows` spans on the wire per
     dispatch".** MEASURED FALSE for `Ramp` (and `Breathing`): `rate_trace_request` emits
     `sum(window_counts)`, a nested loop over per-window COUNTS, not one span per window
     (`crates/conductor-emit/src/rate.rs:130-136`). Three further arms of the same reading are false:
     `Error` emits `depth + 1` (`span_tree.rs:47-48`), `Pii` emits 2 spans on traces / one record per
     category on logs (`pii.rs:139-160`, `:179-183`), `Topology` emits one per service. Same line's
     dependent figures are superseded: the catalog "sits ~200× under the bound" → ≈43×.
  2. **`architecture.md:174` — "Read the rate term as DISPATCHES per second, not wire spans … it is computed
     `occurrences / gap_ms`".** Retired by this chunk: the term now counts wire records.
  3. **`architecture.md:174` — "SURFACED, not authored — the fix … is owned by a working-route entry".**
     Now authored; the owning entry is this chunk.
  4. **`architecture.md:174` — the prescribed fix "count `occurrences × samples`, or rename the term".**
     Neither shipped as written: `occurrences × samples` is right only for `Latency`, and the term was not
     renamed. What shipped is a per-shape upper BOUND, because the rate curves' per-window counts carry
     seeded jitter (`rate.rs:23`, `JITTER = 0.15`) and the static catalog gate has no seed.

- **Expected amendments (from plan):**
  1. `obs-plan §4` — **carried**. Fact in Changes → Symbols/APIs (`coverage_rollup` now emits a `message`
     line) and Harness/gate surface. Search that located it: `grep -n 'coverage_matrix_generate\|
     query_all_p_ids\|validate_coverage\|coverage_percent' .andromeda/obs-plan.md` → hits at `:135` (the
     Critical Path 6 row) and `:357-360` (fields + denominator semantics); 2 sites, 1 master.
  2. `architecture.md §Occupied Resources → contracts/pulse-load-envelope.toml` — **carried**. Fact in
     Changes → Counts/qualifiers moved (1) and (2), and Spec claims disproved (1)-(4). Search:
     `grep -nE 'occurrences / gap_ms|DISPATCHES|200×' .andromeda/*.md` → 1 hit, `architecture.md:174`.
  3. `test-plan §1 Critical Path 6` — **carried**. Fact in Changes → Harness/gate surface (1) and Files
     (`coverage-matrix.md` now exists). Search: `grep -n 'coverage-matrix' .andromeda/test-plan.md` → hits at
     `:39`, `:80`, `:250`, `:361-362`; 5 sites, 1 master.
  4. `test-plan §9` — **carried, conditional**. The entry asked to add the CI step to the stage table *if the
     table enumerates steps individually*, and to confirm which before proposing. Fact in Changes →
     Harness/gate surface (1). The prior chunk's own gate recorded no stage-table change, so the detector
     decides; noted here so its absence is not read as an omission.

- **Coverage of new surfaces:**
  - `EmissionSpec::max_spans_per_dispatch` (pure core fn, no external input) → validation n/a · instrumentation
    n/a (a pure accessor beneath the envelope gate) · PII n/a · tests **unit ✓** (3 tests: one case per shape
    transcribed from the dispatcher's arm, a rate-curve case, an occurrence-independence case) · a11y n/a ·
    tokens n/a
  - `conductor-report --test coverage_gate` (reads two committed artifacts) → validation **n/a** (both reads
    go through shipped loaders / `std::fs`; the manifest via `CapabilityManifest::load`, which bounds-checks)
    · instrumentation n/a (a test target) · PII n/a · tests **is the test** ✓ · a11y n/a · tokens n/a
  - `coverage-matrix.md` (new committed artifact) → validation n/a · instrumentation n/a · PII **redacted ✓**
    (0 host-path hits, basis `grep -cnE '\b[A-Za-z]:[\\/]|/home/|/Users/|%APPDATA%|\.cargo|\.rustup'` → 0) ·
    tests **✓** (the gate's artifact arm byte-compares it to the production renderer) · a11y n/a · tokens n/a
  - `coverage_rollup`'s obs line (new self-obs write) → validation n/a · instrumentation **✓** (rides the
    allowlisted `message` field; adds NO field name and NO span name) · PII **redacted ✓** (0 of the five
    retired obs-plan §4 attribute names reach the stream; basis: the negative grep, 0) · tests **✓** (asserted
    by the plan's Test Command 4 against a fresh `logs/agent-latest.jsonl`) · a11y n/a · tokens n/a
  - CI step (gate surface) → validation n/a · instrumentation n/a · PII n/a · tests **✓** (the step runs the
    gate target; the target ran green locally) · a11y n/a · tokens n/a

## Deviations from intent

1. **Plan step 6's per-shape mapping was wrong on 5 of 9 arms; implemented the measured mapping instead.**
   Justification: the step itself instructed "verify each arm against that match before finalizing, do not
   infer from the field name alone" — the correction executes the step's instruction rather than departing
   from it. The false premise came from the CARRY (see Spec claims disproved 1).
2. **`EmissionShape::spans_per_dispatch()` → `EmissionSpec::max_spans_per_dispatch()`.** Two structural
   reasons the plan could not have had: the `Pii` arm needs `signal`, which a method on `EmissionShape`
   cannot see; and the rate curves' seeded jitter makes an exact per-dispatch count unattainable for a
   seedless static gate, so the value must be an upper bound (hence `max_`). Same touchpoint file, so in
   scope.
3. **The obs line moved from `render()` to `coverage_rollup()`** — see Reverted / negative API facts.
4. **`.gitignore` was listed as a touchpoint but needed no edit.** The plan predicted exactly this
   ("measured: `git check-ignore` returns no match at HEAD, so expect no edit").
5. **Plan Implementation-note figures superseded by measurement:** the note states "max wire rate is 50.00/s
   (`latency-regression`) … 200× headroom". Measured under the shipped mapping: 232/s at
   `halo-breathing-encoding`, ≈43×. No verdict moves either way.

## Decisions & corrections

- **Operator P4 forks (3, all recommendations accepted):** commit `coverage-matrix.md` + gate on it ·
  re-base obs-plan §4 onto the allowlisted `message` field · keep CARRY 4, defer CARRY 5.
- **Operator P5 review — three fixes, all verified at HEAD before applying:** (a) the positive drift arm must
  not use the CWD-relative `CapabilityManifest::default_path()`; (b) `grep -c '"message"'` proves nothing
  because every self-obs line carries that field — name the emitted line's stable prefix; (c) the byte-equality
  arm needs a line-ending pin. All three were real; (c)'s hypothesis was then confirmed empirically at
  implement (`git diff` warns LF→CRLF on every tracked text file).
- **Self-caught during implement:** the negative grep in Test Command 4 needed `|| true` — `grep -c` exits 1
  on zero matches, and zero is the healthy result, so as written the line would have aborted the gate run and
  read as a failure.
- **A vacuity found and stated rather than papered over:** Test Command 1's `git diff --exit-code
  coverage-matrix.md` cannot discriminate while the artifact is untracked (see Outcome).
- **Standing rule reaffirmed:** a token grep is evidence to read, not a verdict — it produced a false result
  twice this session (a P5 check matching a comment; an implement grep matching injected axe-core source).

## Outcome

**Acceptance criteria, re-asserted against the DIFF:**

| Criterion (abbreviated) | Verdict against the diff |
|---|---|
| Set-equality, no literal count / P-ID range; a removed row reproduces a non-zero-exit failure | **MET** — `coverage_gate.rs` loads the manifest at runtime, asserts `!capabilities.is_empty()`, and its negative arm removes **each of the 82 rows in turn**, asserting the drift message names the dropped id |
| Committed artifact pinned by a production-writer round-trip, not parseability | **MET** — the artifact arm byte-compares to `CoverageMatrix::render()` |
| Byte-equality is host-independent | **MET** — `.gitattributes` pins `eol=lf`; `git check-attr` confirms `text: set` / `eol: lf` |
| `nextest --workspace` 0; green under `cargo test -p` too; coverage ≥ 60% | **MET** — 896/896; `cargo test` exit 0; **94.24%** lines |
| No new workspace member, no new cross-seam dependency edge | **MET** — diff adds no crate and no `Cargo.toml` dependency line |
| A violation is `CoreError::SutDrift` on the `Err` side, never a Verdict/ReportState/Blocked | **MET** — the negative arm asserts `expect_err` on that variant |
| Two-source and handle-free; manifest at its conventional repo path, no `CONDUCTOR_*` override | **MET** — resolved via `CARGO_MANIFEST_DIR`; diff introduces no env handle |
| CI step static, existing `rust` job, no live Pulse, `ensure_frontend` untouched | **MET** — one step added; no `ANDROMEDA_PULSE_*` reference in it |
| Counts ride `message`; `ALLOWLISTED_FIELDS` and §11's span set gain nothing | **MET** — `redact.rs` is not in the diff; 0 span names added |
| `cargo audit` 0 · `cargo deny` 0 · `Cargo.lock` un-drifted | **MET** — 0 / 0 / clean (18 `deny.toml`-adjudicated warnings) |
| Zero host paths / struct names in gate output and artifact | **MET** — both greps 0 |
| `[[exempt]]` ledger still empty; `validate()` still rejects a reason-less entry | **MET** — 0 entries; `load_envelope.rs` validation unchanged |
| 4-column coverage table, no verdict column, roll-up caption intact; no new ANSI/palette/lamp | **MET** — `render.rs` and the lamp set are not in the diff |
| Roll-up shape: counts sum to the row total, `(N unbacked)` qualifies auto only | **MET** — artifact reads `82 capabilities · 66 in scope (43 auto (8 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`; 43+16+7 = 66 |
| Gate mints no bracket label / lamp / `ReportState` / token / ANSI entry, never prompts | **MET** |
| **a11y: the routine-arm banner spec + its axe run assert without reinstating a skip** | **MET** — both `an over-envelope run banners its standing…` and `the over-envelope banner state carries no axe violation` **passed**, not context-skipped |
| a11y: the CI step CI-wires none of the three a11y suite families | **MET** — the added step runs one Rust test target |

**Gates green (commands run):** `cargo run -p conductor-cli -- coverage --write` · `git diff --exit-code
coverage-matrix.md` (see caveat) · `cargo nextest run -p conductor-report --test coverage_gate --profile ci`
(3/3) · `cargo nextest run -p conductor-core -p conductor-report --profile ci` (380/380) · `cargo nextest run
--workspace --profile ci` (**896/896**) · `cargo test -p conductor-core -p conductor-report` (0) ·
`cargo run -p conductor-cli -- --agent-mode coverage` + its two greps (1 / 0) · `cargo clippy --workspace
--all-targets -- -D warnings` (0) · `cargo audit` (0) · `cargo deny check advisories bans licenses sources`
(0) · `cargo llvm-cov` collect + `--fail-under-lines 60` (**94.24%**) · `bash scripts/agent-run.sh run --e2e`.

**One Test Command is VACUOUS at this phase, and stays so at this wrap's light gate.** `git diff --exit-code
coverage-matrix.md` exits 0 because the artifact is UNTRACKED until this wrap commits it, and `git diff`
reports nothing for an untracked path — it cannot distinguish "committed copy already current" from "not in
the index at all". **The real guard is the artifact arm of `coverage_gate.rs`** (byte-equality to
`CoverageMatrix::render()`), which holds regardless of tracking and is what proves the artifact current.
The `git diff` line becomes meaningful **from the next chunk on**, once this wrap has committed the file.

**Smoke:** the boot-path did not change, but the plan's Test Commands include `agent-run.sh`, so P3 fired.
`bash scripts/agent-run.sh run` exit 0 — 896/896 nextest, doctests ok (0 in every crate), clippy clean; it
adds the doctest stage no other listed gate covers. The `--e2e` self-verify **ran at P2 and was
recorded-not-rerun at smoke**, per the skill's availability rule.

**The `--e2e` leg's asserted signal is its SPEC LIST, never its exit code** (harness rule L58: the guard is
designed to skip at exit 0, so `$?` cannot tell a full pass from a total skip). Recorded:
driven-session banner `[webview2 152.0.4191.66 windows]`, **10 passing / 2 skipped**, and the 2 skipped are
exactly the two driven-arm specs whose subject needs a live Pulse — a third skip would have meant a seed
failed.

**Outcome basis:** implement's P4 report as given, plus the overseer wrap directive of 2026-09-06 (which
added: the CARRY-5 re-CARRY obligation, the amendment wording constraint for the bound-not-count reading and
its 232/s · ≈43× restatement, the `git diff` vacuity statement, and the `.gitattributes` naming) — each
verified against the tree at authoring: the ramp arithmetic re-derived from
`scenarios/halo-breathing-encoding.toml:22-29`, the `architecture.md:174` hit re-grepped, the
`max_spans_per_dispatch` line range re-measured as `:156-195` (the directive said `:156-192`).

**Process hygiene** (implement P4's census, **re-measured at this wrap** — the host process list is readable
from this session):

| Process | Started by | Final state |
|---|---|---|
| `tauri-driver`, `msedgedriver.exe`, `conductor-tauri.exe`, `node.exe` | the `--e2e` leg (this run) | **terminated** — post-leg census matched the clean pre-leg baseline; no LISTENING socket on 4444/4445 |
| `conductor.exe` (coverage / `--agent-mode` / smoke invocations) | this run | **terminated** — single-turn, exited on their own |
| `pulse-app` | not started | **not started** — this chunk needs no live SUT |
