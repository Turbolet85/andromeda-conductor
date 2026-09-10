# Report — 2026-09-09-port-occupier-test-hygiene

**Chunk:** Port-occupier test hygiene — `cargo test -p conductor-faults` green under plain parallel libtest, the process-global observability dependency removed at the CAUSE, plus a measured result for the crates the short-circuiting gate never reached.
**Date:** 2026-09-10
**Commits:** none since `last_wrap` (2026-09-09T14:22:25Z) — HEAD is `266da34` (the prior wrap's commit); this chunk is uncommitted until P7.

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-faults/tests/port_occupier.rs` (modified, −48) · `crates/conductor-faults/tests/port_occupier_span_witness.rs` (new, 60 lines)
- **Symbols / APIs:** **none added or changed.** No `src/` file touched; no public fn, IPC method, endpoint, export, port, socket or env var added or changed. The test fn `the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines` was RELOCATED between test targets, byte-identically in its body (basis: the plan's byte-identity `diff` gate, exit 0 / 0 bytes output). No signature changed, so there are no callers to re-thread; `conductor_core::{init_observability, ObsSink}` and `conductor_faults::PortOccupier` are READ, not modified.
- **Crates / modules:** none added or removed. `conductor-faults` gains one crate-local **test target** (`tests/port_occupier_span_witness.rs`), taking that crate from 3 to 4 test targets under `cargo test -p` (lib unittests · `port_occupier` · the new witness · `Doc-tests`).
- **Dependencies:** none. `Cargo.lock` and `crates/conductor-faults/Cargo.toml` byte-unchanged (basis: `git diff --quiet HEAD -- Cargo.lock crates/conductor-faults/Cargo.toml`, exit 0). `serde_json` was already a `[dev-dependencies]` entry and dev-deps apply to every test target in the crate.
- **Schema / config:** none.
- **Spec-master edits:** none — this report is authored before P2; no master has been touched.
- **Counts / qualifiers moved:**
  - Workspace nextest binaries **55 → 56**; test total **902 → 902** (the move relocates a test rather than adding one). Basis: gate 8 `Starting 902 tests across 56 binaries` / `902 tests run: 902 passed`; the 902/55 prior from the last wrap's report. Swept for a baked value — `grep -rnE '\b55 binaries|across 55|\b902\b'` over the seven masters + `.claude/rules/*.md` + `.claude/docs/*.md` + CLAUDE.md → **0 hits** outside `master-route.md:129/:130` (historical chunk descs, not spec claims).
  - `tests/port_occupier.rs` test count **7 → 6** (basis: `grep -c '^#\[test\]'` = 6), the seventh now alone in the witness binary (basis: same grep = 1).
  - `conductor-faults` doctests: the inherited record says **0** (the `2026-06-19-port-occupier-fault` report, carried into the route entry's CONTEXT); **measured 3 at HEAD** — `gap.rs:35`, `silence.rs:23`, `train.rs:42`, all green — and `conductor-faults` is the only workspace crate carrying any (basis: gate 1's `Doc-tests conductor_faults` block = 3 passed; the P3 smoke's workspace `--doc` arm shows every other crate at 0). Swept for a baked figure — `grep -rniE '0 doctests|zero doctests'` over the masters + leaves + CLAUDE.md → **0 hits**; the doctest-STAGE statements (`test-plan.md:151`, `:458`, `:597`) name the command and are unaffected.
- **Dev-tool versions:** none.
- **Harness / gate surface:** **none.** `scripts/agent-run.sh`, `scripts/agent-run.ps1`, `.github/workflows/ci.yml`, `.config/nextest.toml` all byte-unchanged (basis: `git diff --quiet HEAD -- <each>`, exit 0 on all four). The chunk's manifest-derived portability sweep lives in `plan.md`'s `[[gate]]` fence — a plan artifact, not a shipped harness surface.
- **Cross-project / external claims:** none.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - *The `--doc` arm "records running 0 doctests deliberately."* **Measured false at HEAD** (3 doctests, above). **Where stated:** the chunk's own frozen working-route entry CONTEXT (`working-route.md:131`) and the prior wrap's report — **no spec master states it** (sweep above, 0 hits). **Disposition:** no master amendment is owed; the frozen entry is inviolate by the freeze contract, so the correction rides the master-record desc at the P7 flip, carrying the measured actuals with their date (operator wrap directive 4). **DISPOSED — P7.4.**
  - No other spec, plan or contract assertion was measured false. In particular, nothing in the seven masters claims the per-crate runner-portability gate currently HOLDS or currently FAILS — `test-plan` §4 and §12:614 state it as a requirement and `§11:538` states the own-binary remedy generically, enumerating no crates (basis: `grep -rniE 'runner-portab'` → requirement statements only; `grep -nE 'readback_shape_witness|canary_obs_witness|obs_span|port_occupier' .andromeda/test-plan.md` → 0 hits). A fourth application of an existing rule amends nothing.
- **Expected amendments (from plan):** **none listed** — `plan.md` §Implementation notes states no `Expected amendments (wrap)` entry, with its reason and the sweep that established it (the 0-hit instance-enumeration grep above).
- **Coverage of new surfaces:** **none** — the delta adds one crate-local test target and introduces no external surface, hot-path operation or UI element. No validation boundary, no new instrumentation, no PII path, no a11y-bearing element, no design token.

## Deviations from intent
- **The P3 smoke driven was `bash scripts/agent-run.sh run`, not the plan's listed `role = 'smoke'` entry `bash scripts/agent-run.sh status`.** *Cause — a P4 plan defect, not a harness finding:* `verification-harness.md:42` (2026-06-21) states that for a no-boot-path-change chunk the smoke is `run` (the release-gate path) and **explicitly not** `status`, because `status` needs a `run_id` from a prior live run; the plan I authored listed `status` anyway. Measured: the listed entry exited 0 while printing an envelope from run `2026-09-07T09-06-34-068`, two days stale — the documented exits-0-while-proving-nothing behaviour. *Justification for driving the other form:* the prescribed command satisfies the rule by execution rather than leaving it noted as a miss; its content is a superset of gates 6/7/8 plus the workspace `--doc` arm. The listed entry still ran, as a P2 gate, exit 0. Typed in the evolve ledger at `2026-09-09T20:16:56Z-b`.

## Decisions & corrections
- **Operator wrap directive 1 — masked-target count corrected 2 → 1.** My implement report said the red gate "was masking two further test targets". Measured: `cargo test -p` runs the lib target first (23 unit tests — error 6 · gap 6 · silence 3 · train 8, all green), then the failing `port_occupier` binary aborts the invocation. The witness binary is created by this chunk and could not have been masked, so the one masked target was **Doc-tests** (3). The finding stands — a red masks every later target of the same invocation — only its count moves.
- **Operator wrap directive 2 — census citation re-aimed, basis unchanged.** My report cited "the 2026-09-08 harness entry"; there is no standalone entry of that date. Re-measured with ERE (`grep -rniE`): `census|process family`, `msedgewebview2`, and `StartTime` each return exactly one hit, `verification-harness.md:58`. That line is **5,687 chars** carrying several dated extensions, and the text I relied on is `**Extended 2026-09-08 (hosted-runner-webview2-session)**` at offset 4624 within it. So both readings were true: one grep hit dated 2026-09-02, and a real 2026-09-08 clause inside it. The citation is now `verification-harness.md:58` — the 2026-09-02 census rule **as extended 2026-09-08**. StartTime remains the basis.
- **A grep-granularity trap worth keeping:** a multi-KB single-line rules entry collapses every dated extension it carries into ONE line hit, so a line-granular grep cannot date a clause inside it and "one hit, dated X" does not refute "a clause dated Y exists here". P3 curation candidate.
- **A deliberate non-escalation:** `test-plan` §4 calls the plain per-crate form "(full, non-doc)" while that form in fact runs the crate's doctests. The phrase reads equally as "the full suite, as opposed to the doc-only arm", so it was measured and recorded rather than raised as a spec↔reality gap — escalating an ambiguity risks authoring content into a spec that was already correct.
- **Operator wrap directive 3** — the markerless *Release build and bundle* CARRY's panic-hook coordinates are repaired in place at P5 (mechanism unchanged, two sites take and restore the global hook).
- **Operator wrap directive 4** — the master-record desc at the P7 flip carries the measured doctest actuals with their date rather than the inherited figure.

## Outcome
**Acceptance criteria, re-asserted against the DIFF** (modify-set: two test files; no `src/`, no manifest, no CI, no nextest config):
- *(tests) both runners green on the subject crate* — **MET.** `cargo test -p conductor-faults` exit 0 (baseline 101); `cargo nextest run -p conductor-faults` 30/30.
- *(tests) the sweep reports every member measured, none short-circuited or omitted* — **MET**, and widened at your P5 review: `measured 9 crates, red 0`, list derived from `cargo metadata --no-deps`.
- *(tests) no retry, no `--test-threads=1`, no in-test `sleep` in the delta* — **MET.** The diff is a relocation plus two import removals; none of the three appears.
- *(obs) the probe still reads real emitted span-lifecycle lines and can still FAIL if `port` leaves the allowlist* — **MET**, two ways: the byte-identity `diff` gate (exit 0, 0 bytes) shows the assertion body unchanged, and the one-shot control removed `"port"` from `ALLOWLISTED_FIELDS`, observed the witness go red, and reverted to green.
- *(obs) span attribute set and name unchanged; no `RUST_LOG` set* — **MET.** No `obs-plan` §4 surface is touched by the diff; no env var appears in it.
- *(arch) no new cross-seam dependency edge, no new occupied resource, `ci` profile still zero-retry* — **MET.** `Cargo.toml`/`Cargo.lock`/`.config/nextest.toml` byte-unchanged; no port, handle or on-disk artifact added.
- *(security) no operator-steerable bind target; no shipped binary gains a listener; lockfile unchanged* — **MET.** The occupier's port stays a private builder parameter (untouched `src/`); lockfile probe exit 0.
- *(a11y) no a11y spec, assertion or gate skipped, weakened or deleted; no retry on a shared lane* — **MET.** The diff touches no a11y surface and no CI lane.

No criterion is contradicted by the diff.

**Gates** (the `[[gate]]` entries by `run`, in order):
1. `cargo test -p conductor-faults` — **exit 0** (`expect 'exit 0'`); baseline red → green.
2. `cargo nextest run -p conductor-faults` — **exit 0**; 30/30.
3. `crates=$(cargo metadata …)` sweep — **exit 0** + `last line measured 9 crates, red 0` ✓.
4. `diff <(git show 266da34:…) <(…port_occupier_span_witness.rs)` — **exit 0** + `no output` ✓ (0 bytes).
5. `git diff --quiet HEAD -- Cargo.lock crates/conductor-faults/Cargo.toml` — **exit 0**.
6. `cargo clippy --workspace --all-targets -- -D warnings` — **exit 0**.
7. `cargo fmt --all --check` — **exit 0**.
8. `cargo nextest run --workspace --profile ci` — **exit 0**; 902/902 across 56 binaries.
9. `bash scripts/agent-run.sh status` — **exit 0** (printed a 2026-09-07 envelope; see Deviations).

No `defer` entry; no `leg` entry; no gate deferred under the source-delta rule (the chunk has `.rs` delta, so every compiled-language gate was mandatory and all ran). **Smoke:** `agent-run.sh run` exit 0 — nextest 902/902, `--doc` 3 passed, clippy 0.

**Outcome basis:** implement's P4 report as given, **plus the operator's wrap directive**, which corrected two of its measurements (the masked-target count 2 → 1; the census citation) and added the route repair and the master-desc premise. No post-implement artifact supersedes anything else in it.

**Process hygiene:** implement's run started no external process — cargo/rustc/test binaries are transient children, all terminated. Census taken twice and keyed on **StartTime**, per `verification-harness.md:58` (2026-09-02, as extended 2026-09-08):

| Process | Started by | Final state |
|---|---|---|
| cargo / rustc / test binaries | this chunk's runs | `terminated` |
| `msedgewebview2` ×6 | **not this chunk** | `left running` — StartTime `2026-08-13 18:06` at implement (2026-09-09 20:18Z); **re-measured at this wrap: six at `2026-09-10 08:02:05`**, i.e. the set turned over between the two readings. Neither generation is this chunk's; both attributions rest on StartTime, not on the census pattern. Owner: whoever owns the host browser session. |
| every other family in the census pattern | — | `none started` |
