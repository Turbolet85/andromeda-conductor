# Report — 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected

**Chunk:** capture-path joins guarded, three stale read-back texts corrected, no live leg
**Date:** 2026-09-23
**Commits:** none since `last_wrap` 2026-09-23T16:24:38Z beyond `e799b9e` (the 0-pending adaptation). This chunk lands in the wrap commit.

## Changes (structured — detectors read this)

- **Files:**
  - NEW `crates/conductor-run/tests/capture_paths/mod.rs`
  - NEW `crates/conductor-run/tests/capture_paths_guard.rs`
  - modified: `crates/conductor-run/tests/{real_model_live,live_suite,journal_conformance,storm_harvest,lifecycle_harvest}.rs`, `crates/conductor-run/src/canary.rs`, `crates/conductor-verify/src/preflight.rs`, `crates/conductor-verify/tests/common/mod.rs`, `scenarios/fingerprint-storm.toml`, `contracts/pulse-real-model-leg-posture.md`
  - Basis: `git status --short` at wrap Setup. Every `src/` edit is comments-only: `canary.rs` doc `:6-7` and `:231-234`; `preflight.rs` `:11-14`, `:49-54`, `:202-205`, `:381-385`.

- **Symbols / APIs:**
  - **Test-only helpers.** The new test module `capture_paths` holds `workspace_root()`, `runs_dir_from(root, Option<&str>) -> Result<PathBuf, String>` and `pulse_logs_dir_from(Option<&OsStr>) -> Result<PathBuf, String>`. These are test-binary helpers, not a shipped API.
    - `runs_dir_from` calls the existing shipped `conductor_core::resolve_under`. It is unchanged, and its callers stay the cli/tauri/run edges (research.md graph query, 19 ref rows).
    - Error texts: `CONDUCTOR_RUNS_DIR rejected: {fixed resolve_under reason}` · `ANDROMEDA_PULSE_DATA_DIR unset` · `ANDROMEDA_PULSE_DATA_DIR rejected: {io::ErrorKind}` · `ANDROMEDA_PULSE_DATA_DIR rejected: not a directory`. None interpolates the value or a path.
  - **`real_model_live.rs`:**
    - `runs_dir()` routes through the guard and panics on rejection with the path-free reason (callers `:76`, `:226`).
    - `pulse_log()` now returns `Result<…, String>` and canonicalizes the data dir, requiring a directory, before joining `logs/`. Its single caller, `print_pulse_witnesses`, emits `pulse-log: none ({reason})` through the existing `redact_value` → `mask_host_paths` scrub.
  - **`live_suite.rs`:** `runs_dir()` routes through the guard. The panics in `capture()` (`:69`) and `envelope()` (`:76`, `:80`) now name the FILE NAME (via a new local `file_name_of`) plus `e.kind()`, never `.display()`.
  - **`journal_conformance.rs`:** `every_journal_in_a_pointed_at_runs_dir_conforms` resolves `CONDUCTOR_RUNS_DIR` through the guard before any read. Its `{target}` interpolations are unchanged and path-free by construction, because the guard admits only relative, `..`-free values.

- **Crates / modules:** none added or removed. `conductor-run` gains one integration test target (`capture_paths_guard`) and one shared `tests/` module (`capture_paths`).

- **Dependencies:** none. `git diff --quiet HEAD -- Cargo.lock` exits 0 and `crates/conductor-run/Cargo.toml` is untouched. `rstest` was deliberately NOT added (see Deviations).

- **Schema / config:** none. No env handle added. `CONDUCTOR_RUNS_DIR` and `ANDROMEDA_PULSE_DATA_DIR` keep their registered meanings. No span, field, or `ALLOWLISTED_FIELDS` entry was added.

- **Spec-master edits:** none. No master was touched by implement.

- **Counts / qualifiers moved:**
  - `CONDUCTOR_RUNS_DIR` test-binary readers joining with no `resolve_under` (`journal_conformance.rs`, `real_model_live.rs`, `live_suite.rs`) went from 3 unguarded to 0 unguarded.
  - `ANDROMEDA_PULSE_DATA_DIR` VALUE readers with no canonicalize (`real_model_live.rs::pulse_log`) went from 1 to 0.
  - Path-printing panics in `live_suite.rs` went from 3 to 0. Basis: gate `grep -vhE '^[[:space:]]*//' … | grep -c 'display()'` reading `last line 0`; the P5 baseline counted 3.
  - `conductor-run` integration test targets rose by 1 (`capture_paths_guard`, 8 tests).
  - The retired read-back claim went from 13 matches across 6 files to 0. Basis: the multi-line `grep -rzoE` gate at P5 baseline versus post-change.

- **Dev-tool versions:** none. No host tool installed or changed.

- **Harness / gate surface:** none changed in `scripts/` or `ci.yml`. The CI journal-conformance step's `CONDUCTOR_RUNS_DIR=runs/a11y` still resolves (relative, in scope).

- **Cross-project / external claims:**
  - Pulse HEAD `83d4060` (no re-read this chunk; the basis is the committed 2026-09-10 envelopes cited at `architecture.md:93`): `fingerprint_refs` carries the triggering cue's computed fingerprint beside the L4 model's constant `det-*` `evidence_refs`.
  - CI on `e799b9e0` (the last-shipped sha, read at phase Setup 5a via `gh api …/commits/e799b9e0…/check-runs`): Rust gate · A11y gate · Frontend gate all `completed success`.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  1. **The retired claim family.** Ten comment sites still asserted that "no read-back field varies with the emitted payload / the computed fingerprint reaches no read-back surface / `fingerprint_refs` is `[]` or payload-invariant". At `83d4060`, `fingerprint_refs` carries the cue fingerprint (`architecture.md:93`, measured 2026-09-10). Where stated, all in-repo code comments and none in a spec master:
     - `crates/conductor-verify/src/preflight.rs:11-14`, `:49-54`, `:202-205` (stale in the opposite direction: it claimed fingerprint attribution while `assert_canary` attributes by freshness, `preflight.rs:397-407`), `:381-385`
     - `crates/conductor-run/src/canary.rs:6-7`, `:231-234`
     - `crates/conductor-verify/tests/common/mod.rs:75-77`
     - `crates/conductor-run/tests/storm_harvest.rs:3-5`
     - `crates/conductor-run/tests/lifecycle_harvest.rs:4-6`
     - `scenarios/fingerprint-storm.toml:73-74`

     Disposition: corrected in this chunk (operator ruling at P4). Sweep basis: `grep -rzoE` over `crates scenarios contracts`, 13 matches across 6 files → 0. Sweep dispositions:
     - `scenarios/pii-scrub.toml:21` is a dated past-tense record of the 2026-08-19 run: no change.
     - The `span_refs` / fixture-severity hits are a different, still-true claim: no change (research.md companion sweep, 19 line hits dispositioned).
  2. **`contracts/pulse-real-model-leg-posture.md:56`** named the deterministic-L4-absent term a `shell-declaration`. It is `shell-absence` (`contracts/pulse-run-contract.toml:47-51`, `l4-real-model`). Corrected.
  3. **The working-route entry's premise that posture-doc `:61-62` ("the second `shell-declaration` the run contract observes") is stale was FALSIFIED at P3.** `mcp-enabled` is the contract's second `shell-declaration` in file order (`pulse-run-contract.toml:41`, `:57`), and `architecture.md:197` uses the same ordinal. Left unchanged. This is a premise of an entry freight, not of a spec master.

- **Expected amendments (from plan):** Basis: `grep -c` per master for `real_model_live` / `live_suite.rs` / `journal_conformance` / `no canonicalize`.
  - **security-plan §Input Validation, env-var path-handles row (`:115`).** Carried by Counts moved bullet 1. The row records `journal_conformance.rs`, `real_model_live.rs` and `live_suite.rs` as unguarded test-binary readers, and `live_suite.rs`'s path-printing panic. All are now CLOSED through `resolve_under` via `tests/capture_paths`. Hits: security-plan `real_model_live` 4 · `live_suite.rs` 3 · `journal_conformance` 3.
  - **security-plan §Input Validation, real-model capture ingest row (`:121`) and `ANDROMEDA_PULSE_DATA_DIR` spawn row (`:122`).** Carried by Counts moved bullet 2. The "no canonicalize" residual is closed. Hits: security-plan `no canonicalize` 4 (`:115`, `:121`, `:122`, `:325`).
  - **security-plan §Security Anti-Patterns → Input (`:325`) and §Bootstrap phases → input-validation-library-install (`:221`).** Carried by Counts moved bullets 1 and 2. The per-READER residual list loses the three `CONDUCTOR_RUNS_DIR` test readers and the data-dir value reader. The `CONDUCTOR_E2E_SEED_DIR` residual is untouched.
  - **test-plan §1 Coverage triggers → Vector 1 (`:87`).** Carried by Counts moved bullet 4. The test-binary `CONDUCTOR_RUNS_DIR` reader class now HAS its negative test (`capture_paths_guard`). Hit basis: test-plan `:87` names no test-binary `CONDUCTOR_RUNS_DIR` reader (a Python scan of line 87 for `CONDUCTOR_RUNS_DIR` → 0 occurrences), so this is ADDITIVE coverage at most, never a stale-claim correction. test-plan `real_model_live` 2 and `live_suite.rs` 1 hits sit at `:155` and `:466` (the `--live` selector / Live-Pulse scenarios), which this chunk did not change.
  - Checked and not owners: architecture / obs-plan / a11y-plan `journal_conformance` hits (arch `:60`, `:173`, `:248`; obs `:225`, `:519`; a11y `:115`, `:248`, `:280`) describe the CI gate and the a11y record, not the reader's join, so they carry no residual claim.

- **Coverage of new surfaces:**
  - `tests/capture_paths` guards (test-binary only) → validation `resolve_under` + canonicalize ✓ · instrumentation n/a (test output only, no tracing) · PII n/a (path-free errors asserted) · tests unit (`capture_paths_guard`, 8 cases, both runners) · a11y n/a · tokens n/a

## Deviations from intent

1. **Guard cases are plain `#[test]` functions, not rstest `#[case]` rows** (plan step 2). This was an operator directive at `/andromeda-implement` invocation. `rstest` is not a `conductor-run` dev-dependency, and adding it would have edited `Cargo.toml` (not a touchpoint) and moved `Cargo.lock`, turning the plan's `git diff --quiet HEAD -- Cargo.lock` gate red. The plan's wording was conditional ("where the shape repeats").
2. **Gate `S='[[:space:]/#!*]+'; grep -rzoE …` was driven by hand.** `gate.py` reported it `not run — env S unset`: the tool reads the in-command shell variable `${S}` as a required env handle. It was run from a script file holding the exact `run` text under `bash -o pipefail`: exit 1, no output. A known-positive control file with one single-line and one line-wrapped phrase returned exit 0 with 2 matches. The wrap's light gate meets the same skip.
3. **Scope widened by operator rulings at P4.** The first ruling brought in `journal_conformance.rs` (a fourth reader). The second brought in the whole claim family: 10 sites instead of the CARRY's 3 texts. The tenth site, `canary.rs:6-7`, was found by the P4 multi-line probe and falls under the same ruling.
4. **Posture doc `:61-62` left unchanged**, against the working entry's text (premise falsified at P3; see Spec claims disproved 3).

## Decisions & corrections

- **Operator directive (implement invocation):** before writing a test target, check that the plan's named fixture framework is actually a dependency of the target crate. A conditional "rstest where the shape repeats" did not license a manifest change the plan ruled out.
- **Operator rulings (P4):** guard `journal_conformance.rs` too; correct the whole stale-claim family.
- **Wrap audit trail written late (operator correction during P2):** `{run_dir}/fanout-results.md` was written AFTER the apply + cascade, not at fan-out. The raw twins `.raw-fanout-security-plan.md` / `.raw-fanout-test-plan.md` were added later still, when the operator flagged them missing. Both were written verbatim from the doc-agent returns held in this wrap's conversation, with no re-fan. The file is the fan-out's only audit trail and the key a wrap Setup-2a resume reads, so it belongs at P2 step 1, before validation.
- **Playbook rule `:134` extended (operator-approved at this wrap):** a residual the master recorded but never routed qualifies once an operator's recorded P4 ruling brings it into the chunk.
- **Sweep hazards found this chunk:**
  - A line-granular grep missed the tenth claim site and could reach the two line-WRAPPED sites (`preflight.rs:52-53` "reaches no / read-back surface", `tests/common/mod.rs:76-77` "never from its own / computed") only by alternate tokens. The multi-line `grep -rzoE` with a `[[:space:]/#!*]+` separator is the form that sees all ten.
  - `gate.py` treats any `${NAME}` inside a `run` string as an env handle, so a gate that defines a shell variable inline is silently skipped as "env unset". P5's `--dry-run` does not flag it.
  - A multi-file count probe (`grep -c A B C | grep -vc ':0$'`) reads GREEN when a file is missing. Stage 1 exits 2, but `pipefail` reports the rightmost non-zero stage (stage 2's exit 1). The P5 baseline measured it, and a leading `test -f` guard fixed it.
  - The code graph does not index the `live-pulse`-gated test files, so `runs_dir` / `pulse_log` resolved only to name collisions. Callers were settled by grep.

## Outcome

Acceptance criteria, re-asserted against the diff:
- (security) Absolute and `..` `CONDUCTOR_RUNS_DIR` values are rejected; unset and relative values resolve. **MET**: cases a–d, `capture_paths_guard` 8/8.
- (security) All four readers route through the guard. **MET**: each file carries `mod capture_paths;`; both clippy runs and workspace nextest are green.
- (security) `pulse_logs_dir_from` canonicalizes, requires a directory, and gives path-free failures. **MET**: cases e–h.
- (obs) No failure text renders a path. **MET**: the `display()` probe reads `last line 0`, and the guard tests assert no message echoes its input.
- (tests) Guard tests run in the default suite under both runners. **MET.**
- (arch) The retired claim appears at no site, the `83d4060` anchor is present, and `:202-205` states freshness. **MET**: the multi-line probe gives exit 1 with no output (by hand), and the `83d4060` probe reads `last line 0`.
- (arch) The posture doc names `shell-absence` and keeps the `:62` ordinal. **MET**: the three posture probes.
- (tests) Workspace gates are green. **MET.**
- (security) Supply chain is unchanged. **MET.**
- (tests) No live leg fired and no CI step added. **MET**: the diff touches no `scripts/` or `ci.yml`.
- No verification-matrix capability was claimed.

Gates (implement run, `.andromeda/runs/2026-09-23T20-40-06-implement`):

| Gate | Result |
|---|---|
| `git -C "$CARGO_HOME/advisory-db" rev-parse HEAD` | green (exit 0) |
| `git -C "$CARGO_HOME/advisory-db" status --porcelain` | green (no output) |
| `cargo audit` | green |
| `cargo deny check advisories bans licenses sources` | green |
| `git diff --quiet HEAD -- Cargo.lock` | green |
| `cargo fmt --check` | green |
| `cargo clippy --workspace --all-targets -- -D warnings` | green |
| `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` | green |
| `cargo nextest run -p conductor-run --test capture_paths_guard --profile ci` | green (8 tests run: 8 passed) |
| `cargo test -p conductor-run --test capture_paths_guard` | green (8 passed) |
| `cargo nextest run --workspace --profile ci` | green |
| `cargo test -p conductor-run` | green |
| `cargo test -p conductor-verify` | green |
| `S='[[:space:]/#!*]+'; grep -rzoE …` | `not run — env S unset` by the tool; driven by hand in its exact form: exit 1 · no output (atoms held), with a known-positive control |
| `grep -c 83d4060 … \| grep -c ":0$"` | green (exit 1 · last line 0) |
| `test -f …capture_paths/mod.rs && grep -vhE … \| grep -c 'display()'` | green (exit 1 · last line 0) |
| `grep -c "defining term, and a \`shell-declaration\`" …` | green (exit 1 · last line 0) |
| `grep -c "shell-absence" …` | green |
| `grep -c "second \`shell-declaration\` the run contract observes" …` | green (last line 1) |
| `test -f … && grep -cE "\b[A-Za-z]:[\\/]\|…" … \| grep -vc ":0$"` | green (exit 1 · last line 0) |

Smoke: skipped. The boot-path files `preflight.rs` and `canary.rs` changed in comments only; there is no smoke or self-verify entry and no UI surface.

Outcome basis: implement's P4 report, as given in this session's conversation, plus the operator's directive at implement invocation (rstest → plain `#[test]`).

Process hygiene: implement started no processes. The post-run census (`tasklist` filtered to conductor / sidecar / pulse-app / msedgedriver / tauri-driver / the capture test binaries) was empty at 2026-09-23T20:44:45Z.
