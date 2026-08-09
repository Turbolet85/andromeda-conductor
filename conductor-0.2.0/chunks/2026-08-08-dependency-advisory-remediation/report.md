# Report — 2026-08-08-dependency-advisory-remediation

**Chunk:** Dependency advisory remediation — lock-only bumps clearing RUSTSEC-2026-0194/-0195 (quick-xml→0.41.0, lever is plist 1.10.0 — no Tauri bump), -0204 (crossbeam-epoch→0.9.20, a dev-dep path) and the deny-only unsound -0190 (anyhow→1.0.104), restoring cargo-audit + cargo-deny green (Cargo.lock)
**Date:** 2026-08-09T11:22:56Z
**Commits:** none since `last_wrap` — this chunk's commit is wrap P7 (HEAD is `0088133`, the prior chunk)

## Changes (structured — detectors read this)

- **Files:** `Cargo.lock` (8 insertions / 8 deletions) · `Cargo.toml` (2 edits)
- **Symbols / APIs:** **none** — zero Rust source delta. No new or changed public fn, IPC method / `#[tauri::command]`, endpoint, export, port, socket, or env var. `git status --short crates/` is empty.
- **Crates / modules:** none added, removed, or changed. Workspace membership unchanged (9 members).
- **Dependencies:**
  - **Bumped (lock):** `plist` 1.9.0 → **1.10.0** · `quick-xml` 0.39.4 → **0.41.0** · `crossbeam-epoch` 0.9.18 → **0.9.20** · `anyhow` 1.0.102 → **1.0.104**. Verified as the complete delta: 559 → 559 name+version pairs, exactly 4 swapped, none added or removed.
  - **Manifest floor raised:** `[workspace.dependencies] anyhow` `"1.0.102"` → `"1.0.104"`. **This is the one arch-registered version string this chunk moves.**
  - **Manifest declaration removed:** `rmcp = "1.7.0"` (+ its `# MCP read-back client` comment) — an orphan: no member referenced it and it was absent from `Cargo.lock`, so the resolved graph is unchanged. Aligns the manifest with the already-recorded 2026-06-27 rmcp-removal decision.
  - **Unchanged (asserted):** tokio 1.52.3 · rusqlite 0.38.0 · tonic 0.14.6 · tonic-prost 0.14.6 · prost 0.14.4 · opentelemetry-proto 0.32.0 · rand_chacha 0.9.0 · rand_core 0.9.5 · garde 0.22.1 · thiserror 2.0.18 · tauri 2.11.3 · tauri-build 2.6.3 · libsqlite3-sys 0.36.0 · indicatif 0.17.11.
  - **`deny.toml` untouched** — no ignore added, no license allow added. All four advisories are actionable, so they were fixed by bump, not exception.
- **Schema / config:** none — no migration, no config key, no violation-schema change. `.github/workflows/ci.yml`, `.config/nextest.toml` and `deny.toml` are byte-unchanged.
- **Coverage of new surfaces:** **none — this chunk introduces no external surface, hot-path operation, or UI element.** No validation / instrumentation / PII / a11y / design-token flag applies. Existing surfaces were re-proven, not extended.

## Deviations from intent

1. **Ran an unlisted producer to obtain the obs artifact.** The plan's obs acceptance criterion asserts properties of `logs/agent-latest.jsonl`, but no command in its `## Test Commands` emits that file — `scripts/agent-run.sh` gates the agent-mode scenario leg on a non-empty `$SCENARIO` (`agent-run.sh:69-71`). **Justification:** the criterion is only checkable against the artifact, so the same producer CI's obs-conformance job uses was run directly (`cargo run -q -p conductor-cli --bin conductor -- run error-baseline-spike --seed 424242 --agent-mode`). It exited 0 with `[BLOCKED] error-baseline-spike` and all three conformance assertions passed. No plan or scope file was edited.

No other deviation. The matrix no-op (zero capabilities claimed) was planned, not a deviation.

## Decisions & corrections

- **Route order changed by the operator** — "Dependency advisory remediation" was promoted ahead of the first markerless entry ("SUT-drift check") via `--chunk`, because the supply-chain gate blocks every downstream chunk that ends in a release gate.
- **Take-up corrected three route-entry premises** (evidence: `.andromeda/runs/2026-08-08T19-29-55-phase/advisory-evidence.md`). The entry said "crossbeam-epoch ≥0.9.20 and quick-xml ≥0.41 **through the Tauri tree**" and the prior handoff listed 4 advisories across 2 packages. In fact: RUSTSEC-2026-0190 is **`anyhow`**, not crossbeam-epoch, and is an `unsound` advisory that `cargo audit` reports as an exit-0 warning while `cargo deny` denies it; **crossbeam-epoch is a dev-dependency** (`assert_fs` → globwalk → ignore → crossbeam-deque), never in the release binary; and **quick-xml needs no Tauri bump** — `plist` 1.10.0 absorbs the semver-major, `tauri-utils` was already latest. Intent (gate green) unchanged; mechanism corrected in `scope.md`.
- **P4 scope decisions (operator-approved).** (1) Raise the `anyhow` manifest floor rather than lock-only, so the file a human reads stops naming a version with a known unsound advisory. (2) Delete the orphan `rmcp` declaration here — its apparent later owner, capability **v2-28**, is scoped to three *documents*, so a manifest line would fall outside every acceptance and never be caught.
- **P5 review corrections (operator).** (a) **The wrap-time amendment surface is `architecture.md` only** — `anyhow 1.0.102` sits at `architecture.md:31` (§Stack, Error handling row), `:52` (§Established Decisions [Error Handling]) and `:234` (§Inherited Defaults). **security-plan carries no anyhow version**: its ten hits (`:26, :55, :115, :198, :251, :264, :318, :334, :366`) are all anyhow-*edge* / error-sanitization references, and §Dependency Security's version-bearing content is the cargo-audit ≥0.22 / cargo-deny ≥0.19 tool floors and the `tauri` ≥2.10.3 line. A lockstep security-plan amendment must **not** be proposed. (b) An acceptance criterion must cite the **artifact** for a count and the **spec** for a rule.
- **Pre-existing cross-document gap, explicitly NOT this chunk's drift.** `deny.toml` holds **17** `[advisories] ignore` + **8** `[licenses] allow` entries, while security-plan §Dependency Security's Accepted-exceptions paragraph names exactly **one** of each (`number_prefix` / RUSTSEC-2025-0119; the `Zlib` allow via `foldhash`). The other 16 ignores (5× `unic-*`, 10× gtk-rs, `proc-macro-error`) and 7 allows accreted without the spec following. **This chunk adds none and changes none of them** — per `.andromeda/playbook.md:46` (a cross-document gap the report shows the chunk did not introduce; gap pre-existing → routine dismiss) it belongs to a dedicated doc-reconcile pass and is carried as a handoff follow-up, not amended here.
- **Verified technical findings worth keeping.** Bare `cargo deny check` is **equivalent** to `cargo deny check advisories bans sources licenses` (both report the same error count; the bare form's output shows `advisories · bans · licenses · sources` all evaluated) — CI's invocation is correct and must not be "fixed". `RUSTSEC-2024-0429` (glib 0.18.5, `unsound`) blocks **neither** tool — `cargo audit` classes it informational among its allowed warnings and `cargo deny` does not report it — so no `deny.toml` entry is owed and there is no latent fifth error.

## Outcome

**All acceptance criteria met.** Gates run and green:

| Command | Result |
|---|---|
| `cargo audit` | exit 0 — **0 vulnerabilities** (was 3), 18 allowed warnings (was 19) |
| `cargo deny check advisories bans sources licenses` | exit 0 — `advisories ok, bans ok, licenses ok, sources ok` (was 4 errors) |
| `scripts/agent-run.sh run` | exit 0 — `ensure_frontend` → nextest → `test --doc` → `clippy -D warnings` |
| `cargo nextest run --workspace --profile ci` | **428/428 passed, 0 skipped** (baseline exact), zero retries |
| `cargo metadata --locked --format-version 1` | exit 0 — lock complete, un-drifted |
| `bash scripts/agent-run.sh status` | exit 0 — truthful Blocked envelope |

**Risk surface exercised, not assumed:** the clippy stage compiled `anyhow 1.0.104`, `quick-xml 0.41.0`, `plist 1.10.0`, `crossbeam-epoch 0.9.20`, then `tauri-build` → `tauri-codegen` → `tauri` → **`conductor-tauri`**, clean under `-D warnings`, through the `ensure_frontend`-preceded harness path (never a bare `cargo build`). The semver-major quick-xml move inside `tauri-build`'s build script is confirmed harmless. No `.snap.new` — no insta golden rewritten.

**Smoke ✓** (boot-path/harness condition). `agent-run.sh status` exit 0; the agent-mode producer exited 0 with `[BLOCKED]`. Obs conformance on `logs/agent-latest.jsonl`: 7 base fields on 5/5 lines · no absolute host path · zero `panicked` lines. The two `ERROR conductor_verify::client` lines carry empty message text — the designed field-allowlist drop (`error` is not in `ALLOWLISTED_FIELDS`, `crates/conductor-core/src/redact.rs:21-47`), pinned by the obs/redact tests inside the passing 428, not bump fallout.

**Zero fix-loop iterations** — every gate passed first run. **Verification matrix:** no capability claimed (`chunk == marker` set is empty), so no `ref`/`implemented` write; v2-27 is partially advanced and stays `chunk: null`. Coverage remains 1/32.
