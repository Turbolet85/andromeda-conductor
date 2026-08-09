# Report — 2026-08-09-sut-drift-check

**Chunk:** SUT-drift check — loud failure naming the manifest ids the coverage classification does not cover, built over the shipped `CapabilityManifest::load`/`accepts` (no re-sourcing); a Pulse release can no longer leave Conductor's universe silently stale (conductor-core, v2-02)
**Date:** 2026-08-09T12:40Z
**Commits:** none yet — this wrap creates the chunk commit. (`2cdec9e` predates this chunk; it is the prior chunk's commit, landed after the last wrap wrote `state.yaml`.)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-core/src/drift.rs` — **new** (198 lines incl. tests)
  - `crates/conductor-core/src/error.rs` — modified (+4)
  - `crates/conductor-core/src/lib.rs` — modified (+2)
  - `conductor-0.2.0/verification-matrix.json` — modified (v2-02: `chunk` + refined `acceptance` + `ref` + `status`)
  - `conductor-0.2.0/working-route.md` — modified (freeze stamp only, at promotion)
  - `.andromeda/master-route.md` — modified (promotion append only)
  - `conductor-0.2.0/chunks/2026-08-09-sut-drift-check/` — **new** (scope · research · plan · report)
  - `.andromeda/runs/2026-08-09T11-52-59-phase/` — **new** (7 extracts, 2 raw twins, graph trace)

- **Symbols / APIs** (all additive; `conductor-core` is the workspace dependency root with **6 inbound crate edges and zero outbound**):
  - `pub fn conductor_core::check_sut_drift(&CapabilityManifest, &[CapabilityRow], &[&str]) -> Result<()>` — **new public fn**
  - `pub const conductor_core::KNOWN_UNCLASSIFIED: &[&str]` — **new public const** (22 ids, `P-061`..`P-082`)
  - `CoreError::SutDrift(String)` — **new enum variant**; the enum is `#[non_exhaustive]`, so this is non-breaking for downstream `match`
  - `fn drift_message(..)` — private
  - No IPC method, no endpoint, no port/socket, no env var added.

- **Crates / modules:** `conductor-core` gains module `drift`. No crate added or removed. **No new crate edge** — the module consumes only same-crate items (`CapabilityManifest`, `CapabilityRow`, `CoreError`).

- **Dependencies:** none added, none bumped. `Cargo.toml` and `Cargo.lock` **untouched**.

- **Schema / config:** none. `contracts/pulse-capabilities.toml` is read-only input and was **not** edited. The run-report envelope, the JSONL journal line schema, and the `runs.db` schema are byte-unchanged.

- **Coverage of new surfaces:**
  - `conductor_core::check_sut_drift` (library fn; **not** an external surface — no network, no CLI verb, no IPC, no webview) → validation **n/a** (both inputs are already-validated in-process artifacts; `CapabilityManifest::validate` enforced shape + no-duplicates before `load` returned, and the classification is a compile-time `static`) · instrumentation **n/a by decision** (no span, no metric, no log line — obs-plan §11 bans over-instrumenting non-critical paths and a unit-tier check needs none) · PII **n/a** (no user data; the message carries capability ids + the manifest's Pulse release and capture date only) · tests **unit ×7** · a11y **n/a** (no rendered surface; a11y distiller returned `No domain coverage`) · tokens **n/a** (no operator-facing output)

## Deviations from intent

**None.** All 9 Implementation Steps landed as planned; every acceptance criterion is met; no gate deferral was invoked (real compiled-source delta, so the expensive workspace gates ran); nothing was surfaced as a spec↔reality gap.

Two in-plan judgment calls worth recording:
- **`BTreeSet` for the set differences** rather than an explicit sort pass. Satisfies the plan's "sort explicitly rather than relying on either input's incidental order" note — ordering is a property of the set type, not of the inputs — and is asserted directly by `drift_message_is_byte_identical_for_identical_inputs`, which feeds a manifest ordered `P-003, P-001, P-002` and asserts the message reports `P-002, P-003`.
- **`let ... else { panic!() }`** to bind the error payload in three tests. The neighbouring convention is `assert!(matches!(..))`, which cannot extract the string for a `contains` assertion. A style extension, not a replacement.

## Decisions & corrections

1. **Gate shape (operator decision, phase P4).** A plain live zero-drift assertion would be RED on the committed tree (manifest 82 vs classification 60) and would fail this chunk's own wrap. Chosen instead: the check is **live and gating from day one against a pinned known-gap ledger** of the 22 currently-unclassified ids. Green today; fails the moment an accepted id appears outside the pin, a pinned id gets classified (pin rot), or a classified row loses manifest backing. `v2-03` empties the pin and it degenerates to the plain assertion.
2. **Surface (operator decision, phase P4).** Core unit tier only — matching `v2-02`'s declared `method: unit`; CI enforcement rides the existing `agent-run run` dogfood step; operator-facing surfacing stays with the Epoch-6 *Coverage completeness gate* entry that already owns it. No cli/design/layouts constraint binds this chunk.
3. **Operator correction at the phase P5 review.** The plan claimed `conductor-core` has "five inbound" crate edges. The trace's result set showed **6 rows, all inbound** (`conductor-run` was the row outside a truncated console view), **zero outbound**. Corrected at 4 sites across `plan.md` and `research.md`. Root cause: the row *count* was taken correctly from the trace, but the *composition* was reconstructed from a `tail`-ed view — honoring the trace for counts but not for content.
4. **Matrix acceptance refined (phase P5).** `v2-02`'s route-authored acceptance ("when manifest and classification are in sync it passes") would have been literally unsatisfiable by the shipped chunk, since the pinned design passes while 22 uncovered ids exist. Refined to name the pin, the exact-set match, and the pin-rot failure, so wrap's flip to `verified` tests shipped behavior rather than a condition the chunk deliberately does not meet.
5. **Reverse direction added.** The check also reports a classified row with no manifest entry (a retired capability). Not required by `v2-02`; one extra set difference; reports nothing today. A free latent guard.
6. **`scope.md` amended under the intent-incomplete rule** (phase P5 validation-1) rather than re-planning: the pinned-ledger option that neither original scope option named, the resolved CLI surface, the reverse direction, and the `error.rs` touchpoint.

## Outcome

**All acceptance criteria met.** Gates, all green on first run (**0 fix-loop iterations**):

| command | result |
|---|---|
| `cargo nextest run -p conductor-core` | 188/188 (181 → 188) |
| `cargo nextest run --workspace --profile ci` | **435/435**, exactly +7 from the 428 baseline, zero retries |
| `cargo test --workspace --doc` | ok |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean, all 9 crates |
| `cargo audit` | exit 0 (18 allowed warnings — the known gtk/unmaintained set) |
| `cargo deny check advisories bans sources licenses` | advisories ok · bans ok · licenses ok · sources ok |
| `bash scripts/agent-run.sh run` | exit 0 |

**Smoke:** fired on the harness-listed condition and recorded from the P2 gate rather than re-run (`agent-run.sh run`, exit 0, under a 570s bound; no timeout, no SIGKILL, no leftover processes). The boot-path itself did not change — this is a library-only chunk with no operator surface by decision 2 — so the smoke was supplemented with a direct check that `mod drift` + the re-export are genuinely linked and that both `CARGO_MANIFEST_DIR` uses sit inside `#[cfg(test)]`, never the shipped message.

**Verification matrix:** `v2-02` → `status: implemented` with 5 named refs, one per acceptance clause (in-sync passes · unknown id fails · pin rot fails · retired capability fails · empty-pin zero-drift passes). No `by-construction` fallback needed.

**Invariants held:** drift is `Err(CoreError::SutDrift)`, never a `Verdict`/`ReportState` (verdict/error wall) · the accepted set enters only via `CapabilityManifest::load`/`accepts`, with no second parse of `contracts/pulse-capabilities.toml` and no id-range expression anywhere · the pin is an explicit id list documented as a residual ledger with `v2-03` named as its retirement owner, never a reference universe · the failure message carries ids + release + date and no host path or internal type name (test-asserted) · no OTel SDK, exporter, metric, or new span name.
