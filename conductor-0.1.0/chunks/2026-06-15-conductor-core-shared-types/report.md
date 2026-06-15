# Report — 2026-06-15-conductor-core-shared-types

**Chunk:** conductor-core shared types — Verdict/ReportState enums, scenario model, verdict/error wall
**Date:** 2026-06-15 (UTC)
**Commits:** none yet — the implement work is uncommitted; this wrap authors the chunk commit. (Prior: `36dc324` scaffold · `2bf847f` setup-project.)

## Changes (structured — detectors read this)
- **Files:** new `crates/conductor-core/src/{verdict,report_state,scenario,error}.rs`; modified `crates/conductor-core/src/lib.rs` (module decls + re-exports), `crates/conductor-core/Cargo.toml` (+deps), root `Cargo.toml` (+serde_json pin); regenerated `Cargo.lock`.
- **Symbols / APIs:** new public exports from `conductor_core` — `Verdict { Pass, Fail, CalibrationRegion }`, `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`, `Scenario { name, p_ids, seed, slo_tier }`, `PId(String)`, `SloTier { Tier5s, Tier20s, Tier90s }`, `CoreError` (thiserror, `#[non_exhaustive]`), `Result<T>` alias. Accessor methods `label()` + `status_prefix()` on `Verdict` and `ReportState`. No IPC methods / endpoints / sockets / ports / env vars.
- **Crates / modules:** no new crates. `conductor-core` gains 4 private modules (`verdict`, `report_state`, `scenario`, `error`) with flat public re-exports. No seam→seam edge — `conductor-core` stays a workspace leaf.
- **Dependencies:** `conductor-core` now depends on `serde` (workspace, `derive`) + `thiserror` (workspace) — both already pinned in `[workspace.dependencies]` per arch §Stack; this is their first use. Dev-dep `serde_json` (workspace). **New workspace pin:** `serde_json = "1.0"` added to root `[workspace.dependencies]` (serde companion — used for canonical-name round-trip tests now; the JSONL journal + run-report seam consume it at runtime in Epoch 6). No version bumps. `Cargo.lock` regenerated (+13 transitive: serde/serde_core/serde_derive/serde_json/thiserror/thiserror-impl/syn/quote/proc-macro2/itoa/memchr/unicode-ident/zmij). `cargo audit` not run this chunk (it is its own Foundation gate chunk).
- **Schema / config:** canonical serde serialization fixed to the run-report envelope (arch §Standard Contracts) — `Verdict`/`ReportState` serialize as their PascalCase variant names (`"Pass"`/`"CalibrationRegion"`/`"ManualCheck"`/…); `SloTier` serde-renamed to wire forms `<5s`/`<20s`/`<90s`; `Scenario` serializes `p_ids` as a bare string array via `#[serde(transparent)]` `PId`. No migrations, no `runs.db` schema (Epoch 6).
- **Coverage of new surfaces:**
  - `Verdict` / `ReportState` / `SloTier` / `Scenario` / `PId` (pure data types; no external-input boundary wired this chunk) → validation {n/a — garde deferred to the Config-validation chunk per plan; nothing deserializes from external input yet} · instrumentation {n/a — no operations/spans} · PII {n/a — no PII fields, no logging} · tests {unit✓ — 10 tests: serde round-trip + canonical names + `SloTier` wire forms + accessors + Scenario round-trip} · a11y {n/a — no UI} · tokens {n/a — no color/glyph values in core; `status_prefix()` is ASCII; state→color/glyph mapping documented in doc-comments, rendering deferred to cli/GUI}
  - `CoreError` (harness-fault type) → validation {n/a} · instrumentation {n/a} · PII {n/a — message carries no host path / struct name} · tests {unit✓ — Display + verdict/error-wall assertion} · a11y {n/a} · tokens {n/a}

## Deviations from intent
- **`serde_json` added to workspace pins** — not in `scope.md` originally, but resolved in `plan.md` step 1 + research's Open-Question: a serde round-trip test needs a concrete format, and `serde_json` is the standard companion the JSONL journal/report seam needs anyway. Justification: necessary + forward-looking single-source-of-truth pin.
- **Clippy run with `--all-targets`** (plan listed `-- -D warnings`) — stricter, lints the new `#[cfg(test)]` modules too; passed clean.
- **garde validation NOT added — by design.** The plan + the security extract defer garde (`#[derive(Validate)]`, non-empty `p_ids`, range/cross-field invariants) and `CONDUCTOR_*` path handling to the next chunk ("Config-validation surface"). `Scenario`/`PId` derive serde only; **no external-input boundary is wired this chunk** (no config loading / CLI parsing), so the validation surface does not yet exist — the boundary and its garde validation arrive together next chunk. Not drift.
- Otherwise none — `CoreError` landed exactly as specced (`#[non_exhaustive]` + a single `Config(String)` variant; garde `#[from]` deferred to the next chunk).

## Decisions & corrections
- No user corrections or "from now on" conventions this chunk — straight phase → implement → wrap execution.
- **Design point resolved (per plan):** keep design-token color/glyph VALUES out of `conductor-core` (runtime-agnostic core); expose only `label()` + ASCII `status_prefix()`; the state→color/glyph mapping lives in doc-comments for downstream cli/GUI surfaces to bind. Reconciles the design/layouts extracts' color asks with the runtime-agnostic-core invariant.
- **cli status prefixes:** extended CLAUDE.md's named four (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) with `[MANUAL]`/`[RESIDUAL]` for `ReportState::ManualCheck`/`KnownResidual`; `[HOLD]` maps to `Verdict::CalibrationRegion`.

## Outcome
- **All 10 acceptance criteria met.** Gates green: `cargo build -p conductor-core` ✓ · `cargo test -p conductor-core` ✓ (10 passed, 0 failed) · `cargo clippy -p conductor-core --all-targets -- -D warnings` ✓ clean · `cargo build --workspace` ✓ (all 8 crates compile).
- **Smoke:** skipped — no boot-path change (library-only chunk; no bin/entry-point touched; no `agent-run.sh` gate).
