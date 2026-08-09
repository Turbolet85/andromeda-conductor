# Codebase Research — 2026-08-08-dependency-advisory-remediation

## Scope
- **Depth:** moderate · **Reads:** 8 · **Globs/Greps:** 11 (incl. 12 live cargo tool invocations — the advisory state IS this chunk's codebase)

## Files inspected
- `Cargo.toml` (full) — `[workspace.dependencies]`: `anyhow = "1.0.102"` (line 56) is the only one of the three bumped crates declared directly; `plist`/`quick-xml`/`crossbeam-epoch` are transitive and unlisted, confirming the arch constraint that they get no §Stack row. `rmcp = "1.7.0"` (line 45) sits in the block but **no member references it and it is absent from `Cargo.lock`** — dead declaration, invisible to both audit tools.
- `Cargo.lock` (targeted) — `crossbeam-epoch 0.9.18` has exactly one dependent (`crossbeam-deque 0.8.6`); `glib 0.18.5` is present.
- `deny.toml` (full) — `[advisories] ignore` holds 17 justified entries (number_prefix, 5× `unic-*`, 10× gtk-rs, proc-macro-error), each with a comment. `[licenses] allow` holds 8. **No entry for any advisory in this chunk**, and none for RUSTSEC-2024-0429.
- `.github/workflows/ci.yml` (~lines 60-95) — the gate steps: `Test + lint (dogfood agent-run)` → `.\scripts\agent-run.ps1 run`, then two separate steps `cargo audit` and **bare** `cargo deny check`, then the llvm-cov coverage gate (`--fail-under-lines 60`).
- `.config/nextest.toml` (full) — `retries = 0` in **both** `ci` and `default` profiles; CI separately asserts a non-zero retries setting fails the build.
- `scripts/agent-run.sh` (lines 28-75) — `ensure_frontend()` = `cd ui && (node_modules || npm ci) && npm run build`; the bare `run` verb calls `ensure_frontend` → `nextest run --workspace --profile ci` → `test --workspace --doc` → `clippy --workspace --all-targets -- -D warnings`. `--e2e` deliberately skips `ensure_frontend` (it runs `-p conductor-cli` only).
- `crates/conductor-core/src/redact.rs` (lines 90-124) — `sanitize_error(&dyn std::error::Error) -> String` (line 99): `Display` text → `redact_value` → whitespace-collapsed to one line. Deliberately **`anyhow`-free** (takes `&dyn Error`, which `anyhow::Error` derefs to), so the anyhow bump cannot change this function's signature or behavior.
- `crates/conductor-tauri/ui/dist/` — **present** (`index.html` + `assets/`), so the `tauri-build` compile path is satisfiable right now.

## Graph impact
Query (canonical shape 4, existence): `SELECT symbol, crate, file, def_line FROM symbol WHERE symbol LIKE '%redact%' ORDER BY file, def_line` → **25 rows**, trace at `.andromeda/runs/2026-08-08T19-29-55-phase/tree-query-2026-08-08-dependency-advisory-remediation.json` (`db_state: fresh`).

- **`redact/sanitize_error()`** — defined at `crates/conductor-core/src/redact.rs:98`; the anyhow-edge half obs-plan §11 names. Its behavior is already pinned by `redact/tests/sanitize_error_is_single_line_and_host_path_free()` (`redact.rs:201`) against a local `SyntheticError` with a hand-written `Display` impl (`redact.rs:187-190`) — **the anyhow bump's obs-relevant surface is covered by existing tests**, so the acceptance criterion is satisfied by the standard nextest run; no new test is owed.
- **Redaction coverage is broad, not localized** — 25 symbols across `conductor-core` (`redact.rs` implementation + 8 tests, `obs.rs:473/482/500` sink+panic-hook tests, `tests/operator_pause.rs:52`, `tests/toolchain_smoke.rs:28`) and `conductor-verify` (`src/verdict.rs:90`, `tests/expected_slo.rs:74`). Any regression in the anyhow error edge surfaces in the existing workspace run, in multiple crates.
- **Zero workspace symbols are modified by this chunk** — no `calls`/`crate_edges` impact query is meaningful, because the change set is `Cargo.lock` only. That is the finding, not a skipped query.

## Patterns detected
- **Dual-tool supply-chain gate, already wired** (`.github/workflows/ci.yml:74-78`): `cargo audit` and `cargo deny check` as two separate steps — matching security-plan's "both tools, not one". Restoration is making them exit 0, not adding workflow surface.
- **Bare `cargo deny check` ≡ the explicit subcommand list** (verified empirically, not assumed): `cargo deny check` reports **4 errors** and `cargo deny check advisories bans sources licenses` reports **4 errors**; the bare form's own output shows `advisories FAILED · bans ok · licenses ok · sources ok`. cargo-deny runs all check classes when none is named.
- **Frontend-bundle-before-cargo, enforced by the harness** (`scripts/agent-run.sh:28-30, 63-66`): `agent-run.sh run` is exactly the "ensure_frontend-preceded path" the arch acceptance criterion requires — running it satisfies that criterion with no extra step.
- **Zero-retry is doubled** (`.config/nextest.toml`): both `ci` and `default` set `retries = 0`, so even a local re-run cannot mask a bump-induced flake.
- **`anyhow` is quarantined to the binary edges** (`redact.rs:93-99` doc comment, citing arch §Error handling): the reusable core takes `&dyn std::error::Error`, so an anyhow minor bump has no seam-crate API surface to break.

## Conventions to follow
- **Justified `deny.toml` entries carry a comment** (`deny.toml:8-10` header: "WITH a justifying comment — never a silent skip"). If any entry proves necessary, it follows that form; none is expected here.
- **Lock-drift discipline** (security-plan §Dependency Security; `Cargo.lock` committed at the workspace root): `git diff --exit-code Cargo.lock` must be clean after the final gate run.
- **Harness-first verification** (`scripts/agent-run.sh` `run`, test-plan §3): drive the re-proof through the 5-command harness rather than bespoke cargo invocations, so local and CI agree.

## New files to create
- (none)

## Files to modify
- `Cargo.lock` — the three `cargo update -p {plist,crossbeam-epoch,anyhow}` bumps (4 packages move: plist 1.9.0→1.10.0, quick-xml 0.39.4→0.41.0, crossbeam-epoch 0.9.18→0.9.20, anyhow 1.0.102→1.0.104).
- `Cargo.toml` — **conditional, P4 Decision 1 only**: raise the `anyhow` floor (line 56); and **conditional, P4 Decision 2 only**: delete the orphan `rmcp` line (45).

## Open questions
1. **P4 Decision 1 — raise the `anyhow` manifest floor, or lock-only?** Arch flags that `anyhow 1.0.102` is registered in three arch sections, so a floor raise widens the wrap-time amendment; leaving it means the manifest keeps naming a version with a known unsound advisory while the committed lock carries the fix.
2. **P4 Decision 2 — delete the orphan `rmcp = "1.7.0"`?** Both arch and security extracts independently flagged it as contradicting the recorded 2026-06-27 removal decision. Zero-risk (absent from the lock, referenced by no member), but it is hygiene, not advisory remediation.
3. **RUSTSEC-2024-0429 (glib 0.18.5) — disposition established, root cause not.** Empirically it **blocks neither tool**: `cargo audit` classifies it `informational: unsound` and counts it among the 19 *allowed warnings* (exit 0 on warnings), and `cargo deny check` does **not report it at all** — 4 errors, 0 warnings, no glib mention — even though glib 0.18.5 is in `Cargo.lock` within the advisory's affected range (patched `>=0.20.0`, unaffected `<0.15.0`) and carries no `deny.toml` ignore. So **no `deny.toml` entry is owed and there is no latent fifth error**; why deny's graph omits it is unestablished and does not need to be resolved, because the acceptance criterion is empirical (both tools exit 0 after the bumps).
