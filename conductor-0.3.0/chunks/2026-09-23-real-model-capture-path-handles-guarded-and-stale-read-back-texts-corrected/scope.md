# Scope — Real-model capture path handles guarded and stale read-back texts corrected

**Marker:** `2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected`
**Version:** conductor-0.3.0 · Epoch 4 — Live proof against a real Pulse
**Working entry:** `working-route.md:46`. Its title reads: "the three recorded path residuals and three stale texts closed, no live leg"

## What this chunk builds

Two bodies of work, both carried verbatim from the Diagnostic-quality cluster (`working-route.md:50`) when the
2026-09-23 0-pending adaptation split that cluster (operator relay item 1a). **No live leg**: nothing here
drives Pulse, and every proof is a hermetic test, a compile, or a text read-back. Sized for one window, because
the chunk that shipped the real-model leg carried ONE capability and took three windows at effort max
(operator-measured).

### A. The three unguarded capture-path joins (security-plan §Input Validation residuals)

Every coordinate below was re-verified at HEAD `e799b9e` during the fold:

1. `crates/conductor-run/tests/real_model_live.rs:160` `runs_dir()` joins `CONDUCTOR_RUNS_DIR` under the
   workspace root (`CARGO_MANIFEST_DIR/../..`) with no `resolve_under` / canonicalize. An absolute value REPLACES
   the root instead of being rejected. It has two callers (`:76`, `:226`).
2. `crates/conductor-run/tests/real_model_live.rs:670` `pulse_log()` turns `ANDROMEDA_PULSE_DATA_DIR`'s VALUE
   into a path (`Path::new(&data_dir).join("logs")`) with no canonicalize.
3. `crates/conductor-run/tests/live_suite.rs:58` `runs_dir()` has the same join, and its capture-read panic
   (`:69`) prints the resolved path (`path.display()`) into the test's own output.

Outcome: each join is guarded, so an absolute or `..`-escaping `CONDUCTOR_RUNS_DIR` is rejected rather than
honoured, `ANDROMEDA_PULSE_DATA_DIR`'s value is canonicalized before it becomes a read path, and no failure
message in either capture prints a resolved host path.

### B. The three stale read-back texts

1. The comment at `crates/conductor-verify/src/preflight.rs:202-205` says the canary is attributed by the
   emitted fingerprint ("assert the fingerprint reads back from its telemetry slice"). `assert_canary`
   (`:387-407`) actually attributes by FRESHNESS: any incident whose `opened_at` is after
   `canary.emitted_at_unix_nano` is `Ok`, otherwise `NotYet(StaleCorpus)`.
2. The header of `crates/conductor-run/tests/storm_harvest.rs` (`:3-5`) says `fingerprint_refs` "is populated
   from the L4 model's `evidence_refs`, which the deterministic fixture pins to `[]`". At Pulse `83d4060` the
   field also carries the triggering cue's computed fingerprint (Pulse's grounded union; measured 2026-09-10,
   architecture [Read-Back Dependency Posture]). CLAUDE.md's 2026-08-09 entry carries the same correction,
   dated 2026-09-23.
3. `contracts/pulse-real-model-leg-posture.md` §The launch posture (`:56`) calls the deterministic-L4-absent
   term a `shell-declaration`. It shipped as `shell-absence` in the `l4-real-model` set
   (`contracts/pulse-run-contract.toml:49`). [premise-corrected: the MCP-enabled bullet at `:61-62` ("the second
   `shell-declaration` the run contract observes") is NOT stale. `mcp-enabled` is the contract's second
   `shell-declaration` in file order (`pulse-run-contract.toml:41`, `:57`), and `architecture.md:197` uses the
   same ordinal. Only `:56` is corrected, and any reword of `:61-62` must keep the contract-wide ordinal.]

Outcome: each text states the mechanism the code and the SUT at `83d4060` actually have.

## Boundaries

- No live leg, no Pulse launch, no emission. Nothing in the shipped run path changes behaviour. The only
  changes are test-binary path hygiene, comments, and one contract document's prose.
- No spec master is edited: security-plan §Input Validation / §Security Anti-Patterns still describe the three
  residuals as open. Closing them makes those rows stale, and the rows are amended through wrap's amendment
  flow, not by this chunk.
- Not in scope, still carried from the handoff with no sanctioned writer: `test-plan.md:335` ("permanently
  `degraded_mode`") and `.andromeda/residuals.md:11` ("payload fidelity stays unattainable").
- No capability is expected to be claimed. The entry names none, and the Diagnostic-quality cluster's
  capabilities (`v3-10`) stay with the blocked entry at `:50`.

## Premises closed at P3 (research.md §Scope premise closure)

- `live_suite.rs` has two MORE path-printing panics: `:76` and `:80` print `journal.display()`, and that
  journal path is built by `journal_of` → `runs_dir()`. The CARRY names only the capture-read panic. The outcome
  "no failure message prints a resolved host path" covers all three.
- `crates/conductor-run/tests/journal_conformance.rs:216` is a FOURTH unguarded test-binary reader of
  `CONDUCTOR_RUNS_DIR`. It runs in the default suite and is CI-driven with `runs/a11y`, and its panics
  interpolate the raw handle VALUE. security-plan records it as a separate residual, and the CARRY does not
  name it. **IN SCOPE (operator ruling at P4, 2026-09-23):** the shared guard covers it, and its two panics
  stop printing the handle value.
- The guard is `conductor_core::resolve_under` (`lib.rs:39` `pub use`). `conductor-core` is a normal dependency
  of `conductor-run`, and a candidate that does not exist yet resolves (`config_path.rs:36-38`).
- `pulse_log()` needs different handling. `ANDROMEDA_PULSE_DATA_DIR` is legitimately ABSOLUTE
  (`architecture.md:197-198`), so its guard is canonicalize plus is-dir, with failure reported without the path.
- The proof is hermetic tests. Both capture files are `live-pulse`-gated, so the tests live in a
  default-suite target over a shared `tests/` module. Neither capture is re-run live.
- The stale claim is a FAMILY, not three sites. The sweep by what the claim SAYS (research.md, companion
  sweep) finds the retired "no read-back field varies with the emitted payload / computed fingerprint reaches
  no read-back surface" claim at seven sites beyond the two the CARRY names: `preflight.rs:11-14`, `:49-54`,
  `:381-385`, `canary.rs:233-234`, `conductor-verify/tests/common/mod.rs:75-77`, `lifecycle_harvest.rs:4-6`
  and `scenarios/fingerprint-storm.toml:73-74`. All are comments. **IN SCOPE (operator ruling at P4,
  2026-09-23): the whole family is corrected.** The P4 probe, a multi-line grep validated on the untouched
  tree (13 matches across 6 files), found a tenth site, `crates/conductor-run/src/canary.rs:6-7` (module doc,
  "no read-back field varies with what Conductor emitted"). It falls under the same ruling.
  `scenarios/pii-scrub.toml:21` ("fingerprints were the payload-invariant det-* triple") is a dated record of
  the 2026-08-19 run, in the past tense, and is NOT changed.

## Surfaces touched

`crates/conductor-run/tests/{real_model_live.rs, live_suite.rs, journal_conformance.rs}` plus a NEW shared
module `tests/capture_paths/mod.rs` and its default-suite target `tests/capture_paths_guard.rs` · comments only
in `crates/conductor-verify/src/preflight.rs`, `crates/conductor-run/src/canary.rs`,
`crates/conductor-verify/tests/common/mod.rs`, `crates/conductor-run/tests/{storm_harvest,lifecycle_harvest}.rs`
and `scenarios/fingerprint-storm.toml` · `contracts/pulse-real-model-leg-posture.md` (§The launch posture prose).

## CI fold

CI on the last-shipped sha `e799b9e0` shows all three gates green (Rust · A11y · Frontend). There is no red
to disposition.
