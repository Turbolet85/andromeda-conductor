# Codebase Research — 2026-08-09-sut-load-envelope

## Scope
- **Depth:** moderate · **Reads:** 6 files · **Globs/Greps:** 4 · **Graph queries:** 5 (trace: `.andromeda/runs/2026-08-09T20-58-29-phase/tree-query-2026-08-09-sut-load-envelope.json`)

## Files inspected
- `crates/conductor-core/src/phase_spec.rs` (full) — **the decisive read.** `PhaseSpec { name, gap_ms, emission }` and `EmissionSpec { signal }`. There is **no occurrence-count, rate, or intensity field anywhere in the committed model.** `EmissionSpec` is `#[non_exhaustive]` with a doc comment explicitly reserving it for later per-signal detail. `MAX_GAP_MS = 3_600_000` (one hour) per phase; `MAX_JITTER_MS = 60_000`.
- `crates/conductor-core/src/drift.rs:1-175` — the integrity-gate template: module doc explaining the axis, a `pub const` ledger with the retiring-owner named, `pub fn check_*(…) -> crate::Result<()>` doing `BTreeSet` set-algebra in both directions, and a private `*_message` renderer that is **identity-only** ("capability ids + the manifest's release/date, never a filesystem path and never an internal type name").
- `crates/conductor-core/src/run_record.rs:1-80` — the envelope: **exactly eleven fields in declaration order**, schema owned by test-plan §3, five measurement fields `Option` for the blocked-row null rule.
- `scenarios/activity-floor.toml` (full) — 6 phases: 5min active / 10min quiet / 5min / 10min / 5min / **30min lunch**. Fifty of its sixty-five minutes are deliberately *quiet*.
- `scenarios/fingerprint-storm.toml` (full) — records in-file that "`EmissionSpec` carries no occurrence-count field, so the Epoch-8 driver realizes the per-phase count + variant mix **from the phase names**."
- `contracts/` + `.andromeda/refs/` (grep) — `refs/` holds `pulse-capability-spec.md`, `pulse-mcp-contract.md`, `pulse-v0_2_0-capability-audit-2026-06-12.md`. **None mentions the DuckDB append stall, 10k/s, or a 10-minute ceiling.**

## Graph impact
- **`ReportState`** — **204 references across 16 files in 6 crates** (`conductor-core` 107 · `conductor-report` 32 · `conductor-verify` 32 · `conductor-cli` 12 · `conductor-run` 9 · `conductor-tauri` 3), plus the non-Rust webview mirror `ui/src/lamp.ts`. A sixth variant is a wide, cross-seam change, not a local one.
- **`conductor-core` crate edges** — six inbound (`cli`, `report`, `run`, `tauri`, `timeline`, `verify`), **zero outbound**. It is the root; anything added there is visible everywhere and can depend on nothing.
- **Gate/artifact symbols exist and are reusable** — `drift/check_sut_drift()` @ `drift.rs:78`, `drift/check_scenario_backing()` @ `drift.rs:152`, `Scenario::check_capabilities()` @ `scenario.rs:140`, `Scenario::from_toml_str_with()` @ `scenario.rs:129`, `CapabilityManifest::{default_path,load,accepts,validate}` @ `capability_manifest.rs:30/35/53/57`, `config_path/resolve_under()` @ `config_path.rs:16`.
- **Name space is free** — a `LIKE '%Envelope%' OR '%LoadEnv%' OR '%environment%' OR '%Suspect%'` query returns exactly one row, `obs/ServiceIdentity#deployment_environment` (unrelated). No collision for a new `LoadEnvelope` type or an `environment-suspect` token.

## Patterns detected
- **Committed-artifact static gate** (`drift.rs:79-136`): pure set comparison over committed data, `Result::Err(CoreError::…)` for the fault, message names ids + release, never a path. Reads committed artifacts only, "so the same tree always yields the same result."
- **Ledger-as-pinned-debt** (`drift.rs:38`, `drift.rs:61-63`): `KNOWN_UNCLASSIFIED = &[]`, `UNBACKED_AUTO = &[10 ids]`, each held to exact-set equality in both directions so it can only shrink under compulsion, with the retiring owner named in the doc comment.
- **Two-layer validation** (`scenario.rs:129/140`): garde asserts shape; an explicit `check_*` asserts membership against committed data, applied via `from_toml_str_with` at the binary edges rather than threaded through garde's `Context`.
- **Blocked-row null rule** (`run_record.rs:51-71`): a non-measured state populates identity + `slo_tier` only; the five measurement fields are `None`.

## Conventions to follow
- **Envelope schema is owner-first**: `run_record.rs:11` states the schema is owned by test-plan §3 / obs-plan §3 — "exactly these eleven fields, serialized in this declaration order." Any new field originates in test-plan §3 and is reproduced into obs §3 + a11y §3 + arch §Standard Contracts in the same chunk (the 2026-06-16 `read_back_observed_at` precedent).
- **Fault messages are identity-only** (`drift.rs:100-101`): ids and released versions, never a filesystem path, never an internal type name.
- **`EmissionSpec` is the designed extension point** (`phase_spec.rs:39-49`): `#[non_exhaustive]`, doc-committed to growing "without reshaping `PhaseSpec` or the scenario model."

## New files to create
- `contracts/pulse-load-envelope.toml` — the committed envelope artifact (terms + provenance), mirroring `contracts/pulse-capabilities.toml`'s shape.
- `crates/conductor-core/src/load_envelope.rs` — the loader + the `check_*` gate, beside `drift.rs`.

## Files to modify
- `crates/conductor-core/src/lib.rs` — module + re-exports.
- `crates/conductor-core/src/error.rs` — a named `CoreError` variant for the envelope fault.
- `crates/conductor-core/src/phase_spec.rs` — **only if** the rate axis is made first-class (extend `EmissionSpec`).
- `scenarios/*.toml` — comment header carrying the envelope constraint at the authoring surface.
- `conductor-0.2.0/verification-matrix.json` — `chunk` linkage at P5.

## Open questions
1. **The catalog is not currently inside a naive duration bound — and flagging it would be wrong.** Two scenarios exceed 10 minutes of wall-clock: `activity-floor` at **3900s** and `incident-auto-resolution` at **731s**. But `activity-floor` is 50 of its 65 minutes *deliberately quiet* (10+10+30 min gaps), and the SUT's failure mode is **sustained 10k/s storm**, not elapsed time. A duration-only assertion would produce a false positive on the one scenario whose whole point is idleness. The envelope predicate needs the rate axis to be honest — and the model cannot express it (`phase_spec.rs` has no count/rate field). **This is the plan's central decision, and it is upstream of the `ReportState` fork.**
2. **The envelope's terms are second-hand.** Nothing in `.andromeda/refs/` (Pulse's capability spec, MCP contract, or v0.2.0 audit) records the DuckDB append stall. The only record in Conductor's tree is intent §Theme 1 F4; `input.md:152` separately names Pulse's `crates/ingest/examples/load_profiles.rs` as the 10k-spans/s prior art. The envelope artifact must therefore cite intent §F4 as its provenance and say plainly that it is a transcribed SUT record, not a Conductor measurement.
3. **`MAX_GAP_MS` is 6× the ceiling.** A single phase may already be one hour, so garde's existing bounds provide no envelope protection whatsoever — whatever gate lands is the first one.
