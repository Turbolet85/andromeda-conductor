# Report — 2026-06-27-desktop-a11y-harness-setup

**Chunk:** Desktop a11y harness setup — tauri-driver + axe-core + Lighthouse + colorjs.io GUI/a11y test harness under conductor-tauri/ui; lands the deferred tauri::test mock-runtime + tauri-driver integration tests from ch4/ch7/ch8 (start_run/stop_run/run_report/resolve_operator_hold + Channel frames + test-plan Path-7 parity); SETUP not the verification sweep, zero engine/seam change (conductor-tauri)
**Date:** 2026-06-27
**Commits:** uncommitted at report time — this wrap creates the chunk commit (no commits since last_wrap 2026-06-27T10:44:49Z; the phase promotion + implement work are staged in the working tree)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-tauri/ui/wdio.conf.ts` · `ui/test/a11y/accessibility.e2e.ts` · `ui/test/tsconfig.json` · `ui/test/README.md`
  - MOD `crates/conductor-tauri/Cargo.toml` (+`[dev-dependencies]`) · `src/commands.rs` (+`#[cfg(test)]` mock-runtime + Path-7) · `src/pause.rs` (+`resolve_operator_hold` command test) · `ui/package.json` (+devDeps + 2 scripts) · `ui/package-lock.json` · `Cargo.lock` (benign drift)
- **Symbols / APIs:** NONE new public. All Rust additions are `#[cfg(test)]` (private test fns: mock-runtime command tests + the Path-7 parity test + the `resolve_operator_hold` command test). No new `#[tauri::command]`, IPC method, `Channel`, endpoint, port, socket, or env var. (The tests EXERCISE the existing `list_scenarios`/`coverage_matrix`/`run_report`/`start_run`/`stop_run`/`resolve_operator_hold` surface unchanged.)
- **Crates / modules:** none added/removed. `conductor-tauri` stays a leaf bin (code-graph: outbound→{conductor-core, conductor-run}, zero inbound).
- **Dependencies:**
  - Rust `[dev-dependencies]` (conductor-tauri): `tauri` `test` feature · `tokio` `macros` feature · `assert_fs` · `serde_json` — **all already workspace deps; NO new package**. `Cargo.lock` drifted (tauri `test` feature pulled transitive locked pkgs) — **`cargo audit` exit 0 (18 pre-justified deny.toml allowed-warnings, unchanged) · `cargo deny check` advisories/bans/licenses/sources ok**.
  - npm `devDependencies` (conductor-tauri/ui) ADDED: `axe-core@^4.12.0` · `@axe-core/webdriverio` · `lighthouse@^13.0.3` · `colorjs.io@^0.6.1` · `@crabnebula/tauri-driver@^2.0.9` · `webdriverio@^9` + `@wdio/{cli,local-runner,mocha-framework,spec-reporter,globals}@^9` · `tsx`. `package-lock.json` committed.
  - **`npm audit` = 20 (19 moderate · 1 high), ALL dev-only + transitive** (`@opentelemetry/core <2.8.0` ×18 via `lighthouse→@sentry/node`; `serialize-javascript <=7.0.4` ×2 via `@wdio/mocha-framework→mocha`); none fixable non-breaking. **`npm audit --omit=dev` = 0** (no production vulnerability). See Decisions.
- **Schema / config:** none. No new violation schema (the a11y violation-JSON→obs envelope is the Epoch-10 "A11y CI gate" chunk). New npm scripts `a11y` (run the sweep, display-gated) + `typecheck:e2e`.
- **Coverage of new surfaces:** this chunk adds **no new product surface / UI element / runtime hot-path op** — only test code + display-gated harness config. The relevant flags:
  - `conductor-tauri tauri::test mock-runtime tests` → validation n/a · instrumentation n/a · PII n/a · tests **unit/integ ✓** (10/10 conductor-tauri; coverage_matrix/run_report/stop_run/resolve_operator_hold via real `get_ipc_response`) · a11y n/a · tokens n/a
  - `test-plan Path-7 cross-surface parity` → tests **✓** (Tauri-persisted Blocked envelope == CLI's for one seed; obs §4 run_id-keyed envelope)
  - `a11y real-webview specs (wdio + axe + colorjs.io)` → a11y **authored + typecheck-clean, DISPLAY-GATED** (Linux+xvfb + live Pulse; not run on this host, surfaced with reason — test-plan §11) · tokens design-token✓ (colorjs.io reads `:root` pairs, zero literals)

## Deviations from intent
1. **coverage_matrix mock-runtime test asserts on `serde_json::Value` (array len 60), not `Vec<CapabilityRow>`** — `CapabilityRow` is `Serialize`-only; deserializing it would require an out-of-scope `Deserialize` derive on `conductor-core`. `run_report` DOES deserialize `Vec<RunRecord>` (it's `Deserialize` via `read_run_journal`). Justified: keeps the chunk conductor-tauri-only.
2. **Path-7 parity realized via the `conductor_run` library path** (`preflight→drive_run→persist` on a `current_thread` runtime + `TempDir`) with a **reviewed `unsafe { std::env::set_var(ANDROMEDA_PULSE_DATA_DIR,"pulse;injection") }`** — resolution (a) from plan §Implementation notes; nextest's per-process isolation makes it sound (SAFETY comment added). NOT via the `start_run` command's thread/Channel frame stream (the deferred display leg, per the P4 "deterministic-now, webview-deferred" decision). Exercises the exact composition `run_thread` runs, minus the spawn+Channel.
3. **`get_ipc_response` request URL = `http://tauri.localhost`** (the mock webview's real origin; `tauri://localhost` → "not allowed. Plugin not found"). API detail.
4. **`wdio.conf.ts`:** `tauri:options` vendor capability asserted `as unknown as WebdriverIO.Capabilities` (outside wdio's typed surface — justified comment); removed `autoCompileOpts` (wdio-v7, dropped in v9; tsx auto-detects TS).
5. **npm devDep caret ranges** (npm default; `package-lock.json` pins exact) — consistent with the package.json convention.

## Decisions & corrections
- **P4 (phase) decision — "Deterministic now, webview deferred":** the deterministic mock-runtime legs (command contracts + Path-7 persisted-envelope parity, wait-for-signal) land now; the real-webview axe/contrast/keyboard tauri-driver specs are authored + display-gated (Linux+xvfb + live Pulse, Epoch-10 CI), never run on this Windows/no-display host.
- **DECISION (implement, user-chosen) — npm-audit gate → dev-aware:** the a11y-plan §3.5-pinned tooling (lighthouse@13 + wdio@9) brings 20 dev-only transitive advisories that the bare `npm audit`-0 gate forbids, none fixable non-breaking. User chose **"Dev-aware gate (wrap amends)"**: make the npm-audit gate dev-aware (`npm audit --omit=dev`, or a justified suppression) — the npm analogue of the established `deny.toml` justified-ignore pattern for non-shipping transitive advisories. Keeps the full spec'd tooling. **This wrap is to amend security-plan §Dependency Security + the npm-audit gate (ci.yml) accordingly** (out of the chunk's implement file scope; deferred to wrap by the implement protocol).

## Outcome
- **Acceptance criteria met** (under the decided dev-aware gate): the 3 CARRY batches' mock-runtime tests landed + the Path-7 parity leg + the a11y tooling installed at the pinned versions + the real-webview specs authored/typecheck-clean/display-gated. The only open item is the gate-definition amendment (this wrap).
- **Gates (all green except the to-be-amended bare npm audit):**
  - `cargo nextest run --workspace --profile ci` → **417/417** (6 new conductor-tauri tests)
  - `cargo clippy --workspace --all-targets -- -D warnings` → clean
  - `bash scripts/agent-run.sh run` (smoke) → **EXIT 0** (workspace nextest + doctests + clippy + fresh `ui/dist`)
  - `cargo audit` exit 0 · `cargo deny check` all ok
  - `npm run build` (app) → green · `npm run typecheck:e2e` → green
  - `npm audit` (bare) → 20 dev-only · **`npm audit --omit=dev` → 0** (the decided gate is green)
