# Report — 2026-08-17-fingerprint-semantics-token-leading

**Chunk:** Fingerprint semantics re-aligned to token-leading — the same-fp membership redrawn onto the axes
that survive after path variation proved unrepresentable as a same-fp axis, both path variants collapsed into
one different-fp truth, and the storm's threshold arithmetic restored.
**Date:** 2026-08-17
**Commits:** (none since last_wrap — this is the chunk's own first commit)

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-emit/src/exception.rs` (+173/−…, the core) · `crates/conductor-core/src/phase_spec.rs`
  (7 lines, doc comments only) · `scenarios/fingerprint-storm.toml` (31 lines, prose + two variant lists).
  Source delta: 3 files, 118 insertions / 93 deletions.
- **Symbols / APIs:**
  - ADDED `is_token_boundary(bytes, i) -> bool` (private, `conductor-emit::exception`) — the transcribed guard.
  - REMOVED `leading_path_segment(file) -> &str` (private) — it modelled only the superseded rule.
  - CHANGED `is_absolute_path_start` (private) — now guarded by `is_token_boundary`.
  - **SEMANTIC change, no signature change:** `FingerprintVariant::PathVariant` moved from a SAME-fingerprint
    to a DIFFERENT-fingerprint variant. `fingerprint()`'s public signature is unchanged, but **its returned
    value changes for any frame path containing `/`**.
  - No public API added or removed. No IPC method, endpoint, export, port, socket, or env var touched.
- **Crates / modules:** none added / removed. Changed: `conductor-emit` (derivation + variant model),
  `conductor-core` (doc comments on `FingerprintVariantSpec`).
- **Dependencies:** none added, none bumped. `Cargo.toml` / `Cargo.lock` untouched — **zero dependency delta**.
- **Schema / config:** `scenarios/fingerprint-storm.toml` `[phases.emission].variants` changed at both phases
  from `["identical","path","line"]` to `["identical","line"]`. No new config key, no serde wire-enum member
  added or removed (`FingerprintVariantSpec` keeps all six). No migration. No violation-schema change.
- **Spec-master edits:** none — no `.andromeda/` master was touched by /implement (correct; the P-017 clause
  rides this report as an Expected amendment below).
- **Counts / qualifiers moved:**
  - The storm's same-fingerprint variant set: **TRIPLE → PAIR** (`identical`/`path`/`line` → `identical`/`line`).
    Stated in `scenarios/fingerprint-storm.toml` prose (updated) and possibly in `test-plan.md` §6 / §1 and
    `obs-plan.md` §4 — flagged for the sweep.
  - The P-017 identity narrowings: **TWO → ONE**. The first-`NORMALIZED_FRAMES` bound survives; the
    leading-path-segment narrowing is retired. Stated in `architecture.md` §Read-Back Dependency Posture
    (and check §Standard Contracts for the duplicate).
  - Workspace test count **610 → 611** (−3 retired unit tests, +4 new).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** the hex-address same-fp axis was designed and **deliberately not built** —
  it needs a `render_stacktrace` shape change and works only appended at line end, which would put new
  synthetic content on the wire. Operator-decided at P4. `Frame` keeps `{function, file, line}`.
- **Spec claims disproved by measurement:**
  - **`architecture.md` §Read-Back Dependency Posture, the P-017 clause.** It states that of a frame's `file`
    only the LEADING PATH SEGMENT is identity-significant — "`is_absolute_path_start` fires on ANY `/`" — so
    `src/worker.rs` and `src/anything/else.rs` are one identity. **Measured FALSE at Pulse HEAD `efabe8e`:**
    Pulse guards that arm with `is_token_boundary`, so a relative path is preserved in full and is significant
    at EVERY depth, while a token-leading absolute path is stripped to nothing. Evidence: Pulse's own
    `crates/buffer/src/fingerprint.rs` tests `compute_differs_for_relative_paths_differing_below_leading_segment`,
    `normalize_stacktrace_preserves_relative_paths_in_full`, `normalize_stacktrace_strips_only_absolute_paths`;
    reproduced Conductor-side by the new `relative_paths_are_significant_at_every_depth`, which fails against
    the pre-fix scanner.
  - **Consequence not previously recorded anywhere:** because Conductor transcribed the pre-guard scanner, its
    `fingerprint()` had been computing a DIFFERENT value than Pulse for every slash-bearing path — including
    the committed base fixture `src/worker.rs`. This was a live derivation drift, not only stale prose.
- **Coverage of new surfaces:**
  - `conductor-emit::exception::is_token_boundary` (derivation hot path, no external input) → validation n/a
    (pure byte-scan over an internally-rendered string) · instrumentation n/a (obs-plan §11 names fingerprint
    generation an inner-loop hot path that must NOT carry a span) · PII n/a (operates on synthetic frames;
    `stacktrace_carries_no_absolute_host_path` still green) · tests unit ✓ (`relative_paths_are_significant_at_every_depth`,
    `token_leading_absolute_paths_normalize_to_the_same_empty_form`) · a11y n/a (no UI) · tokens n/a (no UI).

## Deviations from intent

- **Two of five planned touchpoints needed no edit.** `crates/conductor-run/src/dispatch.rs` — `wire_variant`
  is a 1:1 mapping over all six `FingerprintVariantSpec` members, so its shape survives the semantic change;
  plan step 8 said "confirm rather than assume", and confirming returned clean.
  `crates/conductor-run/tests/dispatch_wire.rs` — its three construction sites sit in
  `the_exception_family_cycles_its_declared_variant_mix`, which asserts occurrence counts and exception-event
  presence only (identity-agnostic), and its three-member mix exercises the modulo better than two would.
  Justification: both were listed for verification, and verification is what they got; neither is a research error.
- **Test set −3/+4 rather than a straight re-aim.** Retired `only_the_leading_path_segment_reaches_the_preimage`,
  `the_path_variant_differs_below_an_unchanged_leading_segment`, `relative_path_is_fingerprint_significant`;
  added `both_path_variants_are_fingerprint_significant`, `path_variants_emit_a_changed_relative_path`,
  `relative_paths_are_significant_at_every_depth`, `token_leading_absolute_paths_normalize_to_the_same_empty_form`.
  Justification: the retired three encoded the below-vs-at-leading-segment split, which the new semantics
  dissolves; re-pointing them would have preserved a distinction that no longer exists.
- **The scenario leg proved the artifact-hygiene criterion only in part.** `SCENARIO=fingerprint-storm
  SEED=4317017 bash scripts/agent-run.sh run` fired and produced fresh artifacts (journal, `runs.db` row,
  `logs/agent-latest.jsonl`, all stamped after leg start) with **zero** host-path matches. But with no live
  Pulse the run took the Blocked spine and emitted nothing (0 `emit.` lines), so no stacktrace ever reached
  those files — the emission half of that criterion was not exercised by the leg. Non-vacuous proof is at unit
  tier: `stacktrace_carries_no_absolute_host_path` and `path_variants_emit_a_changed_relative_path` assert on
  the rendered stacktrace directly and pass. Recorded rather than smoothed over because a surface that cannot
  carry the asserted content is exactly the vacuous-pass class the previous chunk was burned by. The live
  equality lands with the next family leg's harvest by construction.

## Decisions & corrections

- **Operator decision (P4 dialogue): same-fp set is `{Identical, Line}`, not a triple.** The phase directive
  sketched "identical / line-variant / address-variant"; research then measured that the address axis needs a
  `render_stacktrace` shape change and works only trailing. Operator chose the pair — no render change, nothing
  new on the wire, and the storm arithmetic restores exactly.
- **Both path variants kept, as two witnesses at two depths.** `Path` and `RelativePath` now both diverge; they
  differ only in WHERE the path changes, and together they assert significance at every depth. No wire-enum
  member renamed or removed, so no scenario config migration.
- **The debt is repaid.** This chunk closes the cross-project re-alignment obligation opened by the 2026-08-17
  Pulse visit. Pulse's P-075 **blocker 2 (the drive-half) dissolves**; its decline note stays until an actual
  claim attempt, and **blocker 1 (P-025/P-027/P-045 have no timing surface on either side) still stands**. The
  next Pulse visit inherits a P-075 with ONE blocker, not two.
- **Correction to the arch extract, applied at P4.** It stated the P-017 clause edit was `/implement`'s work;
  a spec master is never a touchpoint (routing one through implement skips the sidecar, the playbook validation
  and the cascade). `research.md` was corrected and `scope.md` records the correction.
- **Standing `cargo audit` deferral: 26th pin, basis re-verified unchanged.** True exit 1 with the byte-identical
  `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories bans licenses sources` true exit 0
  across all four classes as the named overlap; `Cargo.lock` un-drifted with zero dependency delta this chunk.
  Compact form carries forward.

## Outcome

**Met.** All 18 plan acceptance criteria satisfied, with the one scoped limit recorded above.

Gates — green on **iteration 1**, zero fixes:
- `cargo nextest run -p conductor-emit` → 76/76
- `cargo nextest run -p conductor-core` → 253/253
- `cargo nextest run -p conductor-run` → 46/46
- `cargo nextest run -p conductor-timeline` → 27/27
- `cargo nextest run --workspace --profile ci` → **611/611, zero retries**
- `cargo test --doc` → 0 (unchanged)
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `cargo deny check advisories bans licenses sources` → true exit 0
- `SCENARIO=fingerprint-storm SEED=4317017 bash scripts/agent-run.sh run` → fired, `[BLOCKED] fingerprint-storm`
  (expected Pulse-less spine)

No gate deferrals — this chunk has real Rust source delta, so the workspace gates all ran.

**Goldens byte-identical, and the assertions demonstrably RAN:** `the_committed_storm_fixture_stream_is_frozen`,
`the_committed_storm_fixture_stream_is_frozen_at_an_alternate_seed`, `fixture_emission_stream_is_frozen`,
`fixture_emission_stream_is_frozen_at_an_alternate_seed` all PASS, driving the real `storm_fixture()`; git shows
`crates/*/tests/snapshots/` unmodified and no `.snap.new` written. Research had measured the projections
variant-independent in advance and that held under execution.

**Smoke ✓** — `bash scripts/agent-run.sh status` exit 0, reading back `run_id 2026-08-17T22-03-43-655`,
seed 4317017, scenario `fingerprint-storm`, state `Blocked` — matching the journal this leg wrote.

**Coverage:** this chunk claims zero capabilities (verified: no `verification-matrix.json` entry carries this
marker). The matrix is untouched.

## Expected amendments (wrap)

1. **`architecture.md` §Established Decisions [Read-Back Dependency Posture]** — the P-017 clause: replace the
   leading-path-segment narrowing with token-leading wording (a relative path is identity-significant at EVERY
   depth; a token-leading absolute normalizes to the empty form; line and hex addresses remain the insensitive
   axes). The first-`NORMALIZED_FRAMES` narrowing survives, so the count of narrowings goes 2 → 1. Record SUT
   HEAD `efabe8e`. **Per-occurrence sweep required**, including the `§Standard Contracts` duplicate.
2. **`test-plan.md` §6 + §1** — check whether either states the same-fp variant triple; it is now a pair.
3. **`obs-plan.md` §4** — check whether it states the triple; `fingerprints_in_batch`'s NAME is unchanged.
