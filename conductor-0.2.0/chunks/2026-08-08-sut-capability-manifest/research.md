# Codebase Research — 2026-08-08-sut-capability-manifest

## Scope
- **Depth:** moderate · **Reads:** 7 · **Globs/Greps:** 9 · **Code-graph queries:** 3

## Files inspected
- `crates/conductor-core/src/scenario.rs` (1–120 full head + test module 170–230) — holds the exact defect: `PId` (`:18-20`) with `#[garde(custom(pid_format))]`, and `pid_format` (`:23-34`) hardcoding `(1..=60).contains(&n)` with the message `"expected P-NNN with NNN in 001..=060"`. `Scenario::from_toml_str` (`:107-112`) is the load boundary: TOML parse failure → `CoreError::Config(sanitize_error(..))`, garde failure → `CoreError::Validation`.
- `crates/conductor-verify/src/manifest.rs` (full, 115 lines) — **the direct precedent for a second pinned manifest.** `default_path()` → workspace-relative `PathBuf`; `load(&Path)` read → parse → private `validate()`; failures are `VerifyError::Manifest { reason }`, documented "never a verification verdict"; the read error deliberately carries only `e.kind()` with the comment *"never the path itself — io::Error's Display leaks it (artifact hygiene)"*. Its 4-test shape is the model: loads-the-committed-artifact / missing-file-is-a-fault / rejects-empty-field / rejects-a-dropped-required-entry.
- `crates/conductor-core/src/coverage.rs` (1–60 + test tail) — `static COVERAGE: [CapabilityRow; 60]` with **`&'static str`-backed fields**, documented "`static` with no allocation and **no runtime IO**". Tests `matrix_has_exactly_sixty_capabilities` (`:141`) and `p_ids_are_contiguous_p001_to_p060_zero_gaps_no_dups` (`:146`, `expected: (1..=60)`).
- `crates/conductor-core/src/config_path.rs` (1–45) — `resolve_under(base, candidate)`; its doc names `CONDUCTOR_CONTRACT_MANIFEST` among the handles it guards; rejects absolute / `..` / symlink-escape, "never silently clamped", returns `CoreError::Config`.
- `crates/conductor-core/src/error.rs` (15–27) — `CoreError` has exactly **two** variants: `Config(String)` and `Validation(#[from] garde::Report)`.
- `crates/conductor-core/Cargo.toml` — deps are `serde`, `serde_json`, **`toml`**, `thiserror`, `garde` (derive), `tracing`, `tracing-subscriber`. **`toml` is already present — no new dependency is required.**
- `contracts/` — contains exactly one file, `mcp-contract.toml` (597 B). This is arch's registered runtime-read config root.
- `.andromeda/refs/` — 4 files (`pulse-capability-spec.md`, `pulse-v0_2_0-capability-audit-2026-06-12.md`, `pulse-mcp-contract.md`, `capability-verification-matrix.json`). All are **Pulse-authored planning reference material; none is parsed at runtime.**

## Graph impact (code-graph, `tree-query-2026-08-08-sut-capability-manifest.json`; all queries `db_state=fresh`)
- **`pid_format`** — **1 reference total** (q3), `crates/conductor-core/src/scenario.rs:19` (its own `#[garde(custom(..))]` attribute). The validator body is entirely contained: changing *how it decides* has no external call sites. This is the narrow part of the change.
- **`PId`** — **82 references across 16 files in 6 crates** (`conductor-core`, `-cli`, `-report`, `-tauri`, `-timeline`, `-verify`); 50 of them outside `scenario.rs` — `run_record.rs` ×8, `core/pause.rs` ×5, `report/db.rs` ×5, `report/report.rs` ×5, `cli/render.rs` ×4, `lamp.rs` ×3, `run_journal.rs` ×3, `verify/record.rs` ×3, and 7 more files. The **type** is pervasive across the workspace, so its shape must not change; only the validation rule behind it may.
- **`Scenario::from_toml_str`** — **41 call sites** (q4 `calls`, q5 `refs` agree) across 6 files in 5 crates; the production ones are `conductor-cli/src/paths.rs:78`, `conductor-tauri/src/commands.rs:78`, `conductor-core/src/scenario_catalog.rs:46` (graph line numbers are **0-indexed**; grep reports these as 79/79/47). A signature change here is expensive.
- **`crate_edges` → `conductor-core`** — 6 inbound crates (q2). Combined with the above: the change is both widely depended on *and* widely referenced. **Keep `PId`'s shape and `from_toml_str`'s signature fixed; change only the rule inside `pid_format`.**

_Correction note: an earlier revision of this section claimed `PId` had no references outside its own test module and that call-site churn was near-zero. That was wrong — it read a query output truncated by a `LIMIT 40` (q1) and by shell `head` filtering as if it were the full result set. The trace's own `rows` field is the authority; see the durable lesson in `plan.md` §Implementation notes._

## Patterns detected
- **Two-tier manifest failure handling** (`conductor-verify/src/manifest.rs:38-50` + the preflight gate): the *loader* returns a typed harness fault (`VerifyError::Manifest`, "never a verification verdict"), while *contract violation at gate time* is what surfaces as `Blocked` with a named precondition. The same manifest therefore legitimately produces **both** shapes — at different moments. This directly resolves the scope's open question 1 (see Open questions).
- **Path-free error text** (`manifest.rs:41-44`): io failures are reported via `e.kind()` only, never `Display`, so the path cannot leak into an artifact.
- **Load-boundary error mapping in `conductor-core`** (`scenario.rs:107-112`): parse → `CoreError::Config(sanitize_error(&e))`, validation → `CoreError::Validation`. A manifest loader in this crate has an established shape to match.
- **Compile-time static coverage table** (`coverage.rs:47-73`): `&'static str` + `[CapabilityRow; 60]` explicitly means "no runtime IO". Manifest-sourcing this table is a *representation change*, not a re-point — evidence that it belongs to `v2-03`, not here.
- **garde 0.22.1 context** (`scenario.rs:20,23` + `:115` comment "garde 0.22.1 has no container-level `custom`"): `pid_format(value: &str, _ctx: &())` — the unused `_ctx` is garde's Context slot, the sanctioned channel for passing the accepted set into a field validator without a global.

## Conventions to follow
- **Harness faults are `CoreError`, verdicts are values** — `error.rs:15-23`; `Result::Err` is harness-only (`error.rs:25-27`).
- **Manifest errors never embed the path** — `manifest.rs:42` (comment is normative).
- **`CONDUCTOR_*` handles resolve through `resolve_under` at the cli edge, never inside the seam** — `config_path.rs:1-7` doc.
- **Committed artifact + `default_path()` relative to workspace root** — `manifest.rs:32-36`.
- **Test module lives in-file under `#[cfg(test)]`** — both `manifest.rs:74` and `scenario.rs`.

## New files to create
- `contracts/pulse-capabilities.toml` *(location pending open question 2)* — the versioned SUT capability manifest: a Pulse version/date stamp plus the accepted capability id set.
- `crates/conductor-core/src/capability_manifest.rs` — the loader + validator mirroring `ContractManifest` (`default_path()` / `load(&Path)` / private `validate()`), plus its `#[cfg(test)]` module.

## Files to modify
- `crates/conductor-core/src/scenario.rs` — `pid_format` sources its accepted set from the manifest instead of `(1..=60)`; the rejection message names the manifest rather than a hardcoded range; the test cases `malformed_or_out_of_range_p_ids_are_rejected` (currently asserts `"P-061"` must fail) and `boundary_p_ids_are_accepted` (`P-001`/`P-009`/`P-060`) re-key to the manifest-sourced set.
- `crates/conductor-core/src/lib.rs` — register the new module.
- `crates/conductor-core/src/error.rs` — possibly one new `CoreError` variant for the manifest fault (or reuse `Config`).

**Not modified:** `crates/conductor-core/src/coverage.rs` and its two "exactly sixty" tests stay green and untouched — the classification universe is `v2-03`'s (and its `&'static str` static will need a representation change there, which is why it must not be smuggled in here).

## Open questions
1. **Verdict/error wall (scope Q1) — the evidence resolves it more precisely than the route line.** `ContractManifest` shows the two-tier split: the *loader* is a harness fault, and `Blocked` is what the *gate* reports. Arch adds the decisive structural point — a `Blocked` row requires `run_id`/`seed`/`scenario`/`p_ids`/`slo_tier`, none of which exist at manifest-load time. Recommend: load/parse/bounds failure → typed `CoreError` (`Err`); the route line's "reporting blocked" is satisfied at the preflight/run surface, which already renders `[BLOCKED]` + named precondition. **P4 AskUserQuestion.**
2. **Manifest location (raised by the arch distiller, not in the original scope).** `contracts/` is arch's registered runtime-read config root and already holds the only other runtime-parsed manifest; `.andromeda/refs/` (the intent's wording) is a planning-docs subtree absent from arch's directory tree and currently holds nothing runtime-parsed. Recommend: `contracts/`. **P4 AskUserQuestion.**
3. **How the accepted set reaches the field validator.** garde 0.22.1's `pid_format(value, _ctx)` context slot is the sanctioned channel, but using it means `Scenario` validation needs a context — which would touch `Scenario::from_toml_str`'s signature (the scope names signature stability as a concern). The alternative is a process-lifetime loaded set. Resolve at P4 as a plan decision; not a user question.
