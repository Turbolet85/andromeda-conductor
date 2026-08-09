# Report — 2026-08-09-in-lane-sut-scenarios

**Chunk:** In-lane SUT scenarios — the first catalog entries above P-060: live-service-truth + Investigate-result as drive+observe operator-checklist scenarios and the single-sourced workspace key as an auto scenario, shrinking UNBACKED_AUTO by P-079 (conductor-core scenarios, P-067/P-072/P-079, v2-04 partial)
**Date:** 2026-08-09
**Commits:** none yet (this wrap authors the chunk commit; prior HEAD is `68b7282 feat(2026-08-09-interpretation-correctness-posture)`)

## Changes (structured — detectors read this)

- **Files:**
  - NEW `scenarios/live-only-service-truth.toml` (P-067)
  - NEW `scenarios/investigate-actions-functional.toml` (P-072)
  - NEW `scenarios/constellation-severity-live-wiring.toml` (P-079)
  - MOD `crates/conductor-core/src/drift.rs`
  - MOD `crates/conductor-core/src/scenario.rs`
- **Symbols / APIs:** no new or changed public function, IPC method, endpoint, export, port, socket or env var.
  `conductor_core::UNBACKED_AUTO` (existing `pub const`) changed **value only**: 11 → 10 entries, `"P-079"`
  removed; its type and every call site are unchanged. Its doc comment was corrected (it named `P-073`/`P-074`/
  `P-079` as owned by one route entry; only `P-073`/`P-074` remain owed and their owners are different
  entries).
- **Crates / modules:** none added, removed or changed. The pin edit is a data change inside `conductor-core`;
  no new workspace member and no new dependency edge.
- **Dependencies:** none added, none bumped. **Zero dependency delta** — `Cargo.lock` and
  `crates/conductor-tauri/ui/package-lock.json` both un-drifted (verified via `git status`).
- **Schema / config:** three new declarative scenario-config files under `scenarios/`, all conforming to the
  existing `Scenario` serde+garde shape — **no schema change**: no new field, no new `ComparisonKind`, no new
  `ClaimClass`, no new `slo_tier` value. All three `slo_tier` values are drawn from the existing closed enum
  (`<20s`, `<90s`, `<20s`). No migration, no violation-schema change, no config key added.
- **Counts / qualifiers this chunk moved** (a documented derived value changed):
  - `UNBACKED_AUTO.len()`: **11 → 10**. The coverage roll-up caption therefore renders
    `82 capabilities · 66 in scope (43 auto (10 unbacked) · 16 drive+observe · 7 static-only) · 16 not-conductors`
    on all three surfaces (verified live via `conductor coverage`). Per-mode summands unchanged
    (43 + 16 + 7 = 66 in scope; + 16 = 82) — the qualifier remains a qualifier, not a fifth summand.
  - Scenario-catalog size: 31 → **34** committed scenarios; first entries above P-060.
- **Coverage of new surfaces:**
  - `scenarios/live-only-service-truth.toml` (P-067, DriveObserve) → validation garde✓ (loads via
    `from_toml_str` + manifest membership) · instrumentation n/a (config data, no span — catalog load is not a
    must-trace path) · PII n/a (synthetic phase names only, no host path) · tests unit✓
    (`in_lane_sut_fixtures_load_and_validate`, `in_lane_drive_observe_members_are_operator_checklist`) ·
    a11y n/a (no UI delta) · tokens n/a (no UI delta)
  - `scenarios/investigate-actions-functional.toml` (P-072, DriveObserve) → validation garde✓ ·
    instrumentation n/a · PII n/a · tests unit✓ (same two) · a11y n/a · tokens n/a
  - `scenarios/constellation-severity-live-wiring.toml` (P-079, Auto) → validation garde✓ ·
    instrumentation n/a · PII n/a · tests unit✓ (`in_lane_sut_fixtures_load_and_validate`,
    `constellation_severity_live_wiring_asserts_incident_visibility_via_hard_count_floor`) · a11y n/a ·
    tokens n/a
  - `conductor_core::UNBACKED_AUTO` value change → validation n/a · instrumentation n/a (the existing
    `tauri.command.unbacked_auto` span is unchanged) · PII n/a · tests unit✓
    (`the_committed_catalog_matches_the_unbacked_ledger` re-proves exact-set backing over the committed
    catalog; `the_unbacked_qualifier_is_not_a_fifth_summand` re-proves the roll-up arithmetic) · a11y n/a ·
    tokens n/a
  - **No UI surface touched** — zero `.tsx`/`.ts`/`.css` delta; the webview roll-up updates through the
    existing `unbacked_auto` IPC prop with no markup change.

## Deviations from intent

**None.** All seven plan Implementation Steps executed as written; no out-of-scope edit; no plan step
underdetermined or conflicting.

Two intent refinements were made *before* implementation and are already recorded in the chunk artifacts,
so they are not deviations here:
- `scope.md` deliverable 3 was amended at phase P5 (validation-1, intent-incomplete): it assumed three
  roll-up render edits, but all 13 `UNBACKED_AUTO` consumers derive the count, so the pin edit propagates
  with no render change. Confirmed live this chunk — the caption moved to `(10 unbacked)` with zero renderer
  edits.
- `v2-04` was deliberately left `chunk: null` at phase P5 (partially advanced, not claimed): its acceptance
  requires a live-Pulse non-blocked verdict via MCP read-back, which depends on `v2-08`/`v2-09`/`v2-10` in
  Epoch 2. Cited in the plan's Provenance instead.

## Decisions & corrections

- **Operator decision (phase P4, Q1):** P-079's `[[expected]]` asserts `CountAtLeast "1"` / `Hard` — incident
  *visibility through read-back at all*, which is exactly what a diverged workspace key destroys. Deliberately
  **not** `Contains "RetryStorm"`: that token belongs to `fingerprint-storm.toml`, and sharing it would make
  P-079 go red for storm-detection reasons unrelated to the key (the one-outcome-per-token discipline).
- **Operator decision (phase P4, Q2):** P-072 stays empty-`expected` / operator-checklist. Three signals
  converged — its `DriveObserve` classification, the standing "no UI automation of Pulse" non-goal (triggering
  Investigate is a Pulse UI action), and the `v2-05` deferral that hands the L4 result's *quality* to
  conductor-0.3.0. The `retrieve_report` read-back leg is recorded in the TOML as declare-only with its owner.
- **`cargo audit` — bounded-wait deferral extends to a FOURTH consecutive chunk.** Re-run this chunk on
  cargo-audit **0.22.2** (the latest published): byte-identical
  `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`. Confirms the
  standing classification as an advisory-**DATABASE** fault, not a tool fault — there is nothing to raise a
  floor to. Remedy applied: the bounded wait alone, with `cargo deny check advisories bans licenses sources`
  **observed** green (exit 0, all four classes) as the overlapping signal rather than assumed. No floor raise,
  no `deny.toml` ignore, no CI edit, no silent accept.
- **Seed convention confirmed and followed:** `4317<PPP>` encodes the primary P-ID. New seeds 4317067 /
  4317072 / 4317079, each verified collision-free before use.
- **Process correction (self-observed):** the `/andromeda-implement` P1 evolve checkpoint was passed without
  executing it; the `code` and `fix-loop` checkpoints both ran after P2 instead. Reading the playbooks after
  their steps completed preserves the no-influence property, but the checkpoint ordering was not followed.
  Recorded in the `code` step's deviation block.

## Outcome

**All acceptance criteria met.** Gates green in **zero fix iterations** — every gate passed first run:

| Gate | Result |
|---|---|
| `cargo nextest run -p conductor-core` | 206/206 (+7 new) |
| `cargo nextest run --workspace --profile ci` | 463/463, zero retries |
| `cargo test --doc` | ok |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `cargo deny check advisories bans licenses sources` | advisories ok · bans ok · licenses ok · sources ok (exit 0) |
| `cargo audit` | **red — bounded-wait deferral, 4th consecutive** (advisory-DB parse fault; see Decisions) |
| `bash scripts/agent-run.sh status` | exit 0 |

**Smoke ✓** (boot-path / harness-listed condition — the chunk extends the catalog loaded at boot):
`bash scripts/agent-run.sh run` exit 0 under a 600s bound (no timeout, no SIGKILL). Beyond the harness verbs,
the new Auto scenario was driven directly — `conductor run P-079 --seed 4317079` loaded the manifest
(82 capabilities), resolved `constellation-severity-live-wiring`, hit `preflight blocked: MCP read-back path
unreachable` with no live Pulse, and reported `[BLOCKED]` at **exit 0** — the reported-envelope-state rule
holding (Blocked is a state, not a non-zero exit). `conductor coverage` confirmed the three new P-IDs render
with the right modes (P-067 drive+observe · P-072 drive+observe · P-079 auto) and the caption reads
`43 auto (10 unbacked)`.

**Verification matrix:** no capability carries `chunk == 2026-08-09-in-lane-sut-scenarios` — this chunk
claimed none, so `/implement`'s matrix write was a no-op and the P7 coverage gate is a no-op. Version coverage
stays 4/32.
