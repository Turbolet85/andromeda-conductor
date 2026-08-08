# Report — 2026-08-08-sut-capability-manifest

**Chunk:** SUT capability manifest — versioned `contracts/` artifact as the accepted P-ID source replacing the compile-time `001..=060` bound; malformed/absent surfaces a named precondition, so a Pulse release is a data update not a code change (conductor-core, v2-01)
**Date:** 2026-08-08
**Commits:** none yet — this wrap authors the chunk commit (last_wrap 2026-06-27T23:44:19Z)

## Changes (structured — detectors read this)

- **Files:**
  - NEW `contracts/pulse-capabilities.toml` — versioned SUT capability set (82 ids, `P-001`..`P-082`)
  - NEW `crates/conductor-core/src/capability_manifest.rs` — loader + validator + 6 tests
  - MOD `crates/conductor-core/src/scenario.rs` — `pid_format` shape-only; `from_toml_str_with`; `check_capabilities`; tests re-keyed
  - MOD `crates/conductor-core/src/lib.rs` — module registration + `pub use CapabilityManifest`
  - MOD `crates/conductor-core/src/scenario_catalog.rs` — `list_scenarios` takes the manifest
  - MOD `crates/conductor-core/src/pause.rs` — one test fixture value (`P-999` → `P-99`)
  - MOD `crates/conductor-cli/src/paths.rs` — `capability_manifest_path` + manifest-checked loads
  - MOD `crates/conductor-cli/tests/cli_smoke.rs` — `copy_capability_manifest` helper
  - MOD `crates/conductor-tauri/src/commands.rs` — `capabilities()` + manifest-checked loads

- **Symbols / APIs:**
  - NEW public: `conductor_core::CapabilityManifest` (`default_path`, `load`, `accepts`; fields `sut_version`, `captured_at`, `capabilities`)
  - NEW public: `Scenario::from_toml_str_with(&str, &CapabilityManifest)`, `Scenario::check_capabilities(&CapabilityManifest)`
  - CHANGED signature: `conductor_core::list_scenarios(dir)` → `list_scenarios(dir, &CapabilityManifest)`
  - CHANGED field set: `conductor_cli::paths::Paths` gains `capability_manifest_path`
  - UNCHANGED (deliberate): `Scenario::from_toml_str(&str)`, `PId` shape/derives, `CoreError` variants
  - CHANGED behavior: `pid_format` validates the `P-NNN` shape only; the `001..=060` range is gone
  - Ports / sockets: none. **Env vars: none added** (a `CONDUCTOR_CAPABILITY_MANIFEST` handle was written then reverted — see Deviations)

- **Crates / modules:** no crate added/removed. Modules changed in `conductor-core` (+1 new module), `conductor-cli`, `conductor-tauri`. No new cross-seam `Cargo.toml` edge.

- **Dependencies:** **none added, none bumped.** `Cargo.lock` un-drifted (verified `git diff --quiet`). `toml`/`serde` were already `conductor-core` deps.

- **Schema / config:** new on-disk config artifact `contracts/pulse-capabilities.toml` (TOML: `sut_version`, `captured_at`, `capabilities[]`). No DB migration, no envelope/violation-schema change, no `runs.db` column change.

- **Coverage of new surfaces:**
  - `contracts/pulse-capabilities.toml` (new parsed config boundary) → validation **✓** (non-empty version/date/set, `P-NNN` shape per id, no duplicates, at load) · instrumentation **✓** (one `info` boundary line, `count` field allowlisted, message carries `sut_version`) · PII **n/a** (no user data; error text carries `e.kind()` only, never the path) · tests **✓ unit** (6 in `capability_manifest.rs`, incl. a no-path-leak assertion) · a11y **n/a** (no DOM surface) · tokens **n/a** (no rendered surface)
  - `Scenario::from_toml_str_with` (new load path) → validation **✓ garde + manifest** · instrumentation **n/a** (no new span; not one of obs §4's seven must-trace paths) · PII **n/a** · tests **✓ unit** (2 in `scenario.rs`) + **✓ e2e** (`cli_smoke` drives it through the real binary) · a11y **n/a** · tokens **n/a**
  - CLI error surface for a missing/rejected capability → validation **n/a** · instrumentation **✓** (sanitized `error:`/`hint:` at the anyhow edge) · PII **✓ redacted** (no absolute host path; asserted by test) · tests **✓ e2e** · a11y **n/a** (cli not-assertable) · tokens **✓** (reuses the shipped `error:`/`hint:` treatment, zero new tokens)

## Deviations from intent

1. **Membership moved out of `pid_format` into `Scenario::check_capabilities`.** Plan step 4 read literally ("re-source `pid_format` so the accepted set comes from the manifest") is unsatisfiable with the plan's own Implementation notes: garde's `dive` propagates `Context`, so giving `Scenario` a capability-set context forces it onto `PId`, `PhaseSpec`, `EmissionSpec` and `ExpectedCheck` plus ~20 call sites — changing `PId`'s public API, which the plan forbids. Surfaced to the operator before any code was written; resolved by splitting shape (garde) from membership (manifest). The plan's *intent* — the accepted set is data, not a constant — is fully met.

2. **Scope widened 3 → 6 files, then 6 → 8.** `conductor-core` never resolves a workspace-relative path (`Paths::resolve` does it at the binary edge), so `from_toml_str(&str)` cannot obtain the manifest and the three production load edges had to be wired. Operator approved the widening (option A of three, over compile-time `include_str!` embedding or shipping the split unwired). Two further files came from the fix-loop and were judged in-scope mechanical consequences rather than soft-exits: `pause.rs` (a `PId("P-999")` fixture that relied on the removed range — changed to a shape-malformed `P-99`, preserving exactly what the test asserts) and `cli_smoke.rs` (its `TempDir` root carried no `contracts/` — added `copy_capability_manifest`, mirroring the existing `copy_manifest`).

3. **`error.rs` listed as Files-to-modify but left untouched.** Plan step 7 made a new `CoreError` variant optional. `CoreError::Config` already carries every load fault in this crate (`run_journal.rs`, `scenario_catalog.rs`), so a variant would be an abstraction the plan did not require. No exhaustive `match` on `CoreError` exists anywhere, so adding one later stays cheap.

4. **A `CONDUCTOR_CAPABILITY_MANIFEST` env handle was written and reverted.** The plan says no override handle is planned and one must never be added quietly. The manifest resolves from the fixed `CapabilityManifest::default_path()`, still through `resolve_under`.

## Decisions & corrections

- **Operator decision (P4, phase):** malformed/absent manifest lands on the **harness-fault** side of the verdict/error wall (`CoreError`, `Result::Err`); `Blocked` stays the gate/run surface's job. Decisive argument: a `Blocked` row requires `run_id`/`seed`/`scenario`/`p_ids`/`slo_tier`, none of which exist at manifest-load time — and `ContractManifest` already demonstrates the same two-tier split for one manifest.
- **Operator decision (P4, phase):** the manifest lives in `contracts/` (arch's registered runtime-read config root, beside `mcp-contract.toml`), **not** `.andromeda/refs/` as the intent's wording suggested — `refs/` holds only planning documents, none parsed at runtime, and is absent from arch's directory tree.
- **Operator decision (P1, implement):** wire the three load edges rather than embed via `include_str!` — embedding would have undercut the very runtime-read rationale that chose `contracts/`.
- **Operator correction (route P4):** a11y violation JSON aligns to obs by **binding direction** (test-plan §3 → obs §6 → a11y), not by amendment recency; downstream aligns to upstream, so the a11y-plan amendment is mandatory, not optional.
- **Operator correction (phase P5):** a claim that the code-graph `calls` view under-reports was false — it was a self-truncated view (`head -30` on a 41-row result; a self-imposed `LIMIT 40` on another). The graph is the authority on call sites; grep over-counts (definition, test-function names, doc comments). Attribute a row count to the query that produced it; graph lines are 0-indexed, grep 1-indexed.
- **Convention observed:** `CoreError::Config` is this crate's established load-fault variant; manifest read errors carry `e.kind()` only, never the `io::Error` `Display` (the normative comment at `conductor-verify/src/manifest.rs:42`).

## Outcome

**Acceptance criteria: met**, with one pre-existing exception (below).

Gates run — `cargo nextest run -p conductor-core` (181/181) · `cargo nextest run --workspace --profile ci` (**428/428**, up from 420) · `cargo test --workspace --doc` (ok) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) · `bash scripts/agent-run.sh run` (exit 0) · `Cargo.lock` un-drifted.

**Smoke (real binary, beyond the harness gate):** `P-074` — a Pulse v0.3.0 capability 0.1.0 hard-rejected at load — now loads and runs; `P-083` is rejected with `P-ID "P-083" is not in the capability manifest for Pulse v0.3.0` (names the manifest, not a range); an absent manifest yields `could not read capability manifest (NotFound)` with no path leak; and appending `P-083` to the TOML made it load **with the same binary, mtime unchanged** — the data-update-not-code-change claim proven end-to-end.

**⚠ Pre-existing gate red, not caused by this chunk:** `cargo audit` / `cargo deny check advisories` fail on **RUSTSEC-2026-0190, -0194, -0195, -0204** (`crossbeam-epoch 0.9.18`, `quick-xml 0.39.4` — two rated 7.5 high). All published 2026-06-29 → 2026-07-06, during the 41-day pause. `Cargo.lock` is byte-identical to HEAD and this chunk added no dependency, so the gate went red with no code change. Fixes are transitive bumps through the Tauri tree (`quick-xml` 0.39 → 0.41 is a major), out of scope here and would drift the lock this chunk asserts un-drifted. Not silently ignored in `deny.toml` — these are actionable vulnerabilities with fixes available, not the unmaintained-transitive class `deny.toml` legitimately excepts. Needs its own chunk or an explicit operator decision.
