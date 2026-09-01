# Report — 2026-09-01-webview-self-verify-windows-host

**Chunk:** Webview self-verify on the Windows host — whether an agent can drive the real Tauri window here,
measured rather than inherited from the CI matrix's silence
**Date:** 2026-09-01
**Commits:** (uncommitted at authoring — this wrap's commit carries the chunk)

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-tauri/ui/wdio.conf.ts` · `scripts/agent-run.sh` · `scripts/agent-run.ps1` ·
  `crates/conductor-run/tests/lifecycle_harvest.rs` (comment-only) · `.gitignore`
- **Symbols / APIs:**
  - No Rust public API changed; no `#[tauri::command]`, IPC method, or exported symbol added or altered.
  - New **env handle `CONDUCTOR_MSEDGEDRIVER`** (reserved `CONDUCTOR_*` namespace) — read by
    `wdio.conf.ts` only, never written by Conductor, never reaching argv as a shell string.
  - `wdio.conf.ts` internal members: `here` · `driverCli` · `UNSAFE_PATH` · `nativeDriver()` ·
    `reportSkip()` · a `before` hook. Capability object gains `wdio:enforceWebDriverClassic: true`.
  - **Ports (dev-only, harness-lifetime):** the driver stack binds `127.0.0.1:4444` (tauri-driver
    intermediary, pre-existing in config) and `127.0.0.1:4445` (msedgedriver, newly observed). Both are
    test-harness listeners inside the leg, distinct from `127.0.0.1:4317`; the shipped
    `conductor-tauri` / `conductor-cli` binaries gain **no** inbound listener.
  - **Remaining-caller fact:** `wdio.conf.ts` has ZERO workspace consumers (code-graph ts plane, 28 rows,
    `db_state: fresh`) — it is read by the `wdio` runner at runtime only.
- **Crates / modules:** none added, removed, or re-scoped.
- **Dependencies:** **none added or bumped.** No `package.json` change, no `Cargo.toml` change;
  `Cargo.lock` byte-unchanged; `package-lock.json` unchanged. Zero package nodes admitted to either tree.
- **Schema / config:** the `--e2e` stage's meaning changed in both harness shells (below). No migration, no
  config key, no violation-schema or redaction-shape change.
- **Spec-master edits:** none at authoring — this wrap's P2 applies them.
- **Counts / qualifiers moved:** **the `--e2e` stage mapping.** Previously `--e2e` ⇒
  `cargo nextest run -p conductor-cli --profile ci` in both shells; now ⇒ ensure-frontend → release build
  with `--features tauri/custom-protocol` → `npm run a11y` (the tauri-driver `wdio run` leg). Docs stating
  the OLD mapping: `layout-templates.md` §Surface: cli — Primary screens (the `agent-run` enumeration),
  `test-plan.md` §3 (`:152`) and §9 (`:456`) — note those two already DESCRIBE the new mapping and the
  shipped script was the divergent side.
- **Dev-tool versions:** none installed or upgraded by this chunk. Host preconditions **observed, not
  changed**: `msedgedriver` 151.0.4129.101 (operator-owned, `D:\…` outside both audited trees),
  WebView2 Runtime 151.0.4129.107, Edge 152.0.4191.53, `@crabnebula/tauri-driver-win32-x64-msvc` 2.0.9
  prebuilt (installed 2026-06-27, unchanged).
- **Harness / gate surface:**
  - `agent-run.{sh,ps1} run --e2e` re-pointed to the webview leg, **identical semantics in both shells**
    (measured: both exit 1 with the handle set, both exit 0 on the skip arm).
  - **New guard contract:** `CONDUCTOR_MSEDGEDRIVER` unset or not naming an existing file ⇒ the leg
    **SKIPS at exit 0**, printing a host-path-free precondition plus a fetch recipe; never a hard failure.
  - The removed `cargo nextest -p conductor-cli` leg is a strict subset of `--unit`'s `--workspace` run;
    no coverage lost.
- **Cross-project / external claims:**
  - `tauri` 2.11.3 `build.rs` computes `let dev = !custom_protocol` and emits `cargo:dev`, which
    `tauri-build` 2.6.0 reads back via `DEP_TAURI_DEV` (`is_dev()`). Basis: the crate sources in this
    host's cargo registry, plus `target/release/build/conductor-tauri-*/output` carrying
    `cargo:rustc-cfg=dev` under `--release`.
  - `andromeda-pulse` `pulse-app/src/inference_runtime.rs:811` — the `(kind, scope, scope_id)` dedupe
    predicate over `registry.list_active(&digest.workspace)`, at HEAD `83d4060`. Basis: read at source.
- **Reverted / negative API facts:** no PATH-fallback for the native driver was shipped — `plan.md` step 2
  proposed one and the operator's directive superseded it with skip-on-unset. Nothing was added then removed.
- **Spec claims disproved by measurement:**
  1. **`.claude/rules/verification-harness.md`** (Session Additions, 2026-08-22 entry (b)): _"only a release
     build reads the bundle, so `npm run build` is necessary-but-insufficient"_ — the MECHANISM is false.
     What decides bundle-embedding is the **`custom-protocol` feature**, not the profile: a bare
     `cargo build --release -p conductor-tauri` produced a binary loading `devUrl`
     (`http://localhost:5173`, a Chromium error page); the same build with
     `--features tauri/custom-protocol` loaded `http://tauri.localhost/` with the app mounted. The entry's
     operational advice (a debug `cargo run` shows devUrl) remains true. **Directive item 3 routes the fix
     through the master that feeds this derived rule.**
  2. **`test-plan.md`** §3 (`:207`), §6 (`:304`, `:369`), §9 (`:456`, `:465`), §12 (`:594`) and
     **`a11y-plan.md`** §3 (Configuration · CI integration · Bootstrap `a11y-ci-gate-wire`), §9, §11, §12:
     the webview E2E is recorded as **Linux + `xvfb` only**, justified by "no WKWebView WebDriver on
     macOS" with Windows never named. Measured false as a platform verdict: a real WebView2 151.0.4129.107
     session drove the release bundle on this Windows host.
  3. **`crates/conductor-tauri/ui/wdio.conf.ts` header** (pre-existing): _"does NOT run on the Windows dev
     host (no display)"_ — false, and **already corrected in-code this chunk**.
  4. **This chunk's own `research.md` open question** claimed "zero `#[tracing::instrument]` across all 7
     `#[tauri::command]` sites … against obs-plan §4". **Retracted — the finding was wrong.** All 7 sites
     carry manual `tracing::info_span!("tauri.command.*").entered()` (`commands.rs` ×6, `pause.rs` ×1),
     which `observability.md` prescribes *because* the attribute does not stack with `#[tauri::command]`.
     The grep tested the wrong token. **No obs gap exists; no route entry is owed (directive item 2).**
     `plan.md` still carries the stale note and is immutable to implement.
- **Coverage of new surfaces:**
  - `--e2e` harness stage (webview leg) → validation `n/a` · instrumentation `n/a` (a shell dispatch arm) ·
    PII `n/a` · tests `e2e` (it IS the test leg; both shells exercised) · a11y `WCAG/focus/kbd✓`
    (2 of 5 specs pass live; 3 fail — see Outcome) · tokens `design-token✓` (the passing contrast spec
    reads `:root` tokens by name from the running webview).
  - `CONDUCTOR_MSEDGEDRIVER` env handle → validation `✓` (existence + `isFile` + shell-metacharacter
    rejection before use) · instrumentation `n/a` · PII `n/a` · tests `e2e` (both guard arms exercised in
    both shells) · a11y `n/a` · tokens `n/a`.

## Deviations from intent

1. **Plan step 2's PATH-fallback was not shipped.** The step said "when unset, fall back to PATH
   resolution"; the operator's mid-turn directive specified "unset or not-a-file ⇒ SKIP at exit 0 with the
   recipe". Implemented the directive (later, explicit, and the shape the guard mirrors). The two cannot
   both hold — a PATH fallback means an unset handle does not skip.
2. **Plan step 6's build command was insufficient.** It prescribed `cargo build --release -p conductor-tauri`;
   that binary loads `devUrl`. Both shells now build with `--features tauri/custom-protocol`. Justification:
   the plan could not have known the mechanism — it is `tauri`'s `dev = !custom_protocol`, read at source
   only after the leg failed.
3. **Two harness behaviours were added that the plan did not name** — a `before` readiness wait and
   `wdio:enforceWebDriverClassic`. Both are in the plan's own modify-set (`wdio.conf.ts`) and both were
   required for the leg to measure anything: without the wait, specs run against an unmounted document;
   without classic enforcement, wdio v9's BiDi script channel answers "Page/Frame is not ready"
   indefinitely against the wry/WebView2 context while the identical calls over classic WebDriver return
   normally (measured over both transports on one session).
4. **A pre-existing blocker was fixed that the plan did not anticipate.** The committed `wdio.conf.ts` used
   `__dirname` in a `"type": "module"` package — it could not LOAD on any host, Linux included. In-scope
   file; blocking; `typecheck:e2e` passed throughout because tsc accepts `__dirname` under the node types.
5. **`.gitignore` was edited — outside the plan's file list.** The leg launches the app with the ui package
   as cwd, so the Tauri backend's self-obs sink lands at `crates/conductor-tauri/ui/logs/`, which the
   root-anchored `/logs/` rule does not cover. Without the entry this wrap's `git add -A` would commit an
   ephemeral log. Hygiene forced by the chunk's own byproduct.
6. **The leg does not reach green: 2 passing, 3 failing.** Justification: of the three failures one is a
   GENUINE axe violation (exactly 1) and two assert a HOLD dialog and an operator-checklist that exist only
   during a driven run against a live Pulse. Fixing either is the _Desktop a11y sweep_ entry's scope
   (v2-22's acceptance names a live Pulse); this chunk authors no specs and weakens no gate. Recorded as a
   surfaced outcome, not a silent skip — see Outcome.

## Decisions & corrections

- **Operator (mid-turn, msedgedriver fork):** REUSE the host binary rather than acquire one — resolved via a
  Conductor-owned env var, `setx`-persisted by the chunk, mirroring the proven guard shape (unset or
  not-a-file ⇒ skip at exit 0 with the recipe, never a hard failure). Auto-fetch survives only as the RECIPE
  inside the skip message, and must name its nuance: the `edgedriver` package keys its download on the
  **Edge browser** version while the driver must match the **WebView2 Runtime** version. Measured on this
  host the same day: Edge 152.0.4191.53 vs WebView2 Runtime 151.0.4129.107 — a full major apart, so the
  auto-fetch route would have acquired a mismatched driver.
- **Operator (P5 plan review):** a correction dispositioned in prose but given no EXECUTOR is not
  dispositioned. The CARRY's source-side twin was added to Files-to-modify with its own step; wrap's cascade
  greps masters and curation homes, never source comments, so nothing downstream would have caught it.
- **Self-correction:** the obs-gap finding (zero `#[tracing::instrument]`) was wrong and is retracted —
  7/7 sites carry the prescribed manual span. Independently re-verified by the operator (directive item 2).
  No route entry is minted.
- **Operator (wrap directive item 1):** route pre-direction — _Desktop a11y sweep_ moves ABOVE _Live
  per-P-ID verdict lamps_, so the harness is calibrated to zero before UI chunks develop against it.
- **Operator (wrap directive item 3):** the `verification-harness.md` correction is routed through its
  feeding master, not patched in the derived rule.
- **Operator (wrap directive item 4):** the arch CARRY keeps the tuple boundary in the body text and
  re-sweeps its site list before applying.

## Outcome

**Acceptance criteria: met, with one deliberately unmet and surfaced.**

- ✅ (tests) `cargo nextest run --workspace --profile ci` → **759/759 passed** (run twice: standalone and
  inside `agent-run.sh run`). `cargo test -p conductor-run` → all suites ok (runner portability).
  `tsc --noEmit` ✓ · `tsc -p test/tsconfig.json` ✓ · `vite build` ✓ · `npm audit --omit=dev` → **0
  production vulnerabilities** · `package-lock.json` unchanged.
- ✅ **(affordance)** A real wdio session attached to the release-built `conductor-tauri.exe`:
  `POST /session` → id `ae3aed84…`, worker `[webview2 151.0.4129.107 windows]`, axe-core injected and
  analyzed, `browser.execute` reading `:root` tokens, colorjs.io contrast computed from the live webview.
  Interactions are genuine driver-level calls, not a process proxy.
- ✅ (tests) `--e2e` invokes the wdio leg with identical semantics in `.sh` and `.ps1`, building `ui/dist`
  and the release bundle first — matching test-plan's own §3/§9 definition.
- ✅ (a11y) One WebdriverIO + `@crabnebula/tauri-driver` session with `@axe-core/webdriverio` injected; no
  second automation stack; **no WCAG conformance claimed** from this chunk.
- ✅ (security) The driver spawn resolves a repo-derived constant
  (`require.resolve('@crabnebula/tauri-driver/cli.js')`); the env value is passed as a separate argv element
  of an array-form spawn after shell-metacharacter rejection; **no absolute host path is committed**.
- ✅ (security) `cargo deny check advisories bans licenses sources` → true exit **0**
  ("advisories ok, bans ok, licenses ok, sources ok"); `cargo audit` → true exit **1**, first diagnostic
  `duplicate advisory ID: RUSTSEC-2026-0244` — the pinned signature reproduced byte-identically, exit codes
  captured BEFORE any pipe. **41st consecutive**; `Cargo.lock` un-drifted and zero package nodes admitted,
  so the basis is cleaner than the 40th's. No floor raise, no `deny.toml` ignore, no CI edit.
- ✅ (arch) All artifacts under `crates/conductor-tauri/ui/`, `scripts/agent-run.{sh,ps1}`, one test-file
  comment, and `.gitignore`; no new Cargo member; no inbound listener in the shipped app; the driver's
  loopback ports (4444/4445) are harness-lifetime and distinct from `:4317`; the one new env var is inside
  the reserved `CONDUCTOR_*` namespace.
- ✅ (design) Skip arm measured in both shells: **exit 0**, named host-path-free precondition, `error:`/`hint:`
  shape, no new lamp or report state.
- ✅ (obs) No OTel JS SDK, browser exporter, or `:4318`/`:4317` self-obs endpoint introduced; the driven
  app's own sink is now gitignored rather than committed.
- ✅ (layouts) Every driven affordance is one the run-console wireframes name; the leg reaches state by
  driving the app, never a route or URL.
- ⚠️ **SURFACED — the `--e2e` leg is red (exit 1) and correctly so.** 2 specs pass (Blocked-vs-Fail token
  distinctness; WCAG AA contrast at 4.5:1 / 3:1). 3 fail: `zero axe violations` returns **1 genuine
  violation**, and the operator-pause + operator-checklist specs assert a HOLD dialog and checklist that
  exist only in a driven live-Pulse run. Per directive item 1 this is EXPECTED until the _Desktop a11y
  sweep_ entry (now re-ordered ahead of the lamps) calibrates it to zero; the reading until then is
  **skip-arm green without the handle; driven red = expected per this record, never a silent skip**.

**Smoke:** the headful self-verify is listed in the plan's Test Commands and ran as a P2 gate (five sessions
across both shells); `agent-run.sh run` (the aggregate release-gate path: nextest + doctest + clippy) →
**exit 0**. Bounded-subprocess discipline held: zero stray `conductor-tauri.exe` / `msedgedriver.exe`
processes after the final leg, verified by `tasklist`.

**Verification matrix:** this chunk claims **no capability** — every unclaimed id belongs to a later entry,
and making v2-22 runnable is a partial advance that stays `chunk: null` per the contract's status lifecycle.
