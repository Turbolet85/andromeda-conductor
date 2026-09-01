# Codebase Research — 2026-09-01-webview-self-verify-windows-host

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 14 · **Graph queries:** 1 (ts plane, 28 rows, `db_state: fresh`)
- The chunk's gating question is a **live probe**, so this research executed one — `tauri-driver --help`
  against the installed Windows prebuilt — rather than reasoning from documentation. That execution is the
  finding the entry assigned to research.

## Files inspected
- `crates/conductor-tauri/ui/wdio.conf.ts` (full, 70 lines) — the display gate and the driver bracket. Header
  `:3-6` asserts "DISPLAY-GATED … does NOT run on the Windows dev host (no display)". `onPrepare` (`:41-43`)
  spawns the **bare name** `tauri-driver` with no path and no platform branch; `onComplete` (`:45-47`) kills it.
  Capability is `{ browserName: 'wry', 'tauri:options': { application } }`, and `application` (`:16-25`)
  already branches on `process.platform === 'win32'` → `target/release/conductor-tauri.exe`. **So the config
  is Windows-aware about the BUNDLE and Windows-blind about the DRIVER.**
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (full, 66 lines) — five `it()` blocks over the
  four accessible paths: zero-axe-violations on the run report, a Blocked row never rendered red, token-pair
  contrast (4.5:1 text / 3:1 status+focus), the operator-pause `alertdialog` (focus trap, `Escape` resolves
  NoGo, focus restores), and the operator-checklist (`role=status` count, `Space` toggles a row). These are
  already real click/keypress affordance legs — nothing needs authoring.
- `crates/conductor-tauri/ui/package.json` — `a11y` script is `wdio run wdio.conf.ts`; devDeps carry
  `@crabnebula/tauri-driver ^2.0.9`, `@axe-core/webdriverio ^4.12.1`, wdio 9.29.1, `axe-core ^4.12.0`.
- `node_modules/@crabnebula/tauri-driver/{index.js,main.js,cli.js,package.json}` — NAPI loader; the win32
  branch (`index.js:62-72`) requires `tauri-driver.win32-x64-msvc.node` locally or the sibling package.
- `scripts/agent-run.sh` (`:53-54`, `:84-104`) + `scripts/agent-run.ps1` (`:50-51`, `:94-114`) — the stage
  selectors and `ensure_frontend` / `Invoke-EnsureFrontend`.
- `crates/conductor-tauri/capabilities/default.json` (full) — deny-by-default ACL, exactly three window
  permissions (`allow-start-dragging` / `allow-minimize` / `allow-close`).
- `crates/conductor-tauri/src/{commands.rs,main.rs,pause.rs}` (grep) — the `#[tauri::command]` surface.
- `Cargo.lock` (grep) — `tauri 2.11.3`.

## Graph impact (ts plane — `tree-query-2026-09-01-webview-self-verify-windows-host.json`, 28 rows, `fresh`)
- **`wdio.conf.ts` symbols** — `timeout0:` `@:38`, `onPrepare0:` `@:41`, `stdio0:` `@:42`, `onComplete0:`
  `@:44`. The config's members are indexed; no other workspace TS symbol references them (a config consumed
  by the `wdio` runner at runtime, which the graph cannot see — per cookbook, **not** dead-code evidence).
- The spec file's symbols are test-local. **Blast radius of editing `wdio.conf.ts` is zero inside the
  workspace** — its only consumer is the runner.

## Patterns detected
- **The driver bracket is a lifecycle hook pair, not a dependency** (`wdio.conf.ts:41-47`): a `spawn` in
  `onPrepare` + `kill` in `onComplete`, with wdio connecting to `127.0.0.1:4444`. A Windows path changes
  *what* is spawned and *with which arguments*, not the shape.
- **Platform branching already exists in this file** (`wdio.conf.ts:16-25`, `process.platform === 'win32'`) —
  the precedent for a driver-side branch lives in the same object.
- **`ensure_frontend` is stage-selective** (`agent-run.sh:84-88`): `--unit` and `--integration` call it,
  `--e2e` does not. Any webview leg attached to `--e2e` must gain that ordering, per architecture's
  `generate_context!` compile-time coupling.
- **Deny-by-default ACL holds** (`capabilities/default.json`): three window permissions, and the file's own
  comment records that app-defined commands need no ACL entry. Driving the window adds no permission.

## Conventions to follow
- **Bare-name spawn is the sanctioned shape** (`wdio.conf.ts:41`; security-plan §Security Anti-Patterns →
  Input): a fixed hard-coded program name resolved through inherited `PATH`. A Windows fix must stay a fixed
  name or a **repo-derived** constant — never an operator- or config-supplied path.
- **Selectors are role / text / `data-testid`** (`accessibility.e2e.ts:21`, `browser.execute` reading
  `document.activeElement?.getAttribute('data-testid')`) — matching test-plan §6 Selector strategy.
- **Stage-selector semantics are duplicated across two shells** (`agent-run.sh` / `.ps1`) and test-plan §3
  requires them identical — every stage edit is a two-file edit.

## Files to modify
- `crates/conductor-tauri/ui/wdio.conf.ts` — the driver spawn must resolve on Windows (the bare name is
  unresolvable here; the executable form is `node <pkg>/cli.js`, or the npm-local `.bin` shim), and must
  pass the native WebDriver via `--native-driver`. The stale display-gate header (`:3-6`) is corrected in the
  same file. **Zero workspace consumers** (graph), so the edit's blast radius is the runner only.
- `crates/conductor-tauri/ui/package.json` (+ `package-lock.json`) — only if a driver-acquisition dependency
  or a Windows script lands; re-enters the `npm audit --omit=dev` + `tsc --noEmit` + `vite build` gate.
- `scripts/agent-run.sh` + `scripts/agent-run.ps1` — **both**, identically, if the webview leg is wired: today
  `--e2e` runs `cargo nextest run -p conductor-cli` and skips `ensure_frontend`.
- `crates/conductor-run/tests/lifecycle_harvest.rs` — the module doc at `:23` carries the CARRY's
  over-scoped "at most one can be active per workspace" claim, verified present at source. A **comment-only**
  re-scope to the `(kind, scope, scope_id)` tuple; no behavior, assertion, or test-name change. Listed here
  because the amendment flow cannot reach a source comment — wrap's cascade sweeps the seven masters and the
  curation homes only, so without an in-repo executor the falsified claim would outlive the correction.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — **expected untouched**; the specs already
  exist and this chunk does not author specs.

## New files to create
- None required by the measurement. A Windows driver-resolution helper, if extracted, belongs beside
  `wdio.conf.ts` under `crates/conductor-tauri/ui/` (arch §Occupied Resources — the non-Cargo npm subtree).

## The probe — measured, not inferred

| Question | Measurement | Verdict |
|---|---|---|
| Does a Windows `tauri-driver` binary exist here? | `node_modules/@crabnebula/tauri-driver-win32-x64-msvc/tauri-driver.win32-x64-msvc.node`, **4,030,976 bytes**, installed 2026-06-27 | **YES** — the entry's "no driver binary" premise is false |
| Does it execute? | `node node_modules/@crabnebula/tauri-driver/cli.js --help` → **exit 0**, prints its usage | **YES** |
| Does it support a Windows native driver? | Advertises `--native-driver PATH` ("path to the native WebDriver binary") + `--native-port`; `--native-host` is explicitly "Linux only" | **YES** |
| Is `tauri-driver` on PATH? | `command -v tauri-driver` → unresolved | **NO** — but `.bin` shims + a direct `node cli.js` both work |
| Is `msedgedriver` available? | Absent from PATH; Edge (151.0.4129.78 · 152.0.4191.53) bundles none; `edgedriver@^6.1.2` is installed transitively but has **downloaded no binary** (it fetches on first use) | **NOT YET** — one acquisition step |
| Is the drive target built? | `target/release/conductor-tauri.exe` **absent**; `crates/conductor-tauri/ui/dist` **present** | **NO** — one release build |
| Is `tauri` past the Windows origin-confusion floor? | `Cargo.lock` → `tauri 2.11.3` vs required ≥ 2.10.3 | **YES** |
| Does the bundle satisfy deny-by-default? | `capabilities/default.json`: three window permissions, no remote iframe, no `shell-open` | **YES** |

**Conclusion.** The Windows webview-drive path is **not blocked by the platform**. It is blocked by two
ordinary, recordable steps — acquiring `msedgedriver` and building the release bundle — plus one code defect
(`wdio.conf.ts` spawns a name that cannot resolve on this host). The "Linux+xvfb only" framing across
test-plan (six sites) and a11y-plan (six sites) is a CI-runner assumption that measurement now contradicts at
the driver layer.

## Seam facts
- `crates/conductor-tauri/ui/` is an **npm subtree, not a Cargo member** — a driver dependency is a devDep in
  the already-registered non-shipping dev tree (`npm audit --omit=dev` gate), never a `Cargo.lock` node. It
  therefore does **not** admit a package to the cargo audit basis, which keeps the carried 41st `cargo audit`
  PREREQ's re-pin basis intact.
- A **host-installed** `msedgedriver` sits in neither audited tree (security-plan §Dependency Security), so
  provenance + version recording is the only available control.

## Open questions
- **`msedgedriver` acquisition route** — let the installed `edgedriver` devDep fetch it (wdio's standard
  mechanism, network at first use) or acquire it explicitly and record provenance/version? → blocks:
  **plan-decision** (it decides whether the plan prescribes a dependency-driven or an operator-visible step,
  which is the scope's stated boundary against hidden host installs).
- **Where the webview leg attaches** — repurpose `--e2e` (today a `conductor-cli` nextest leg) versus a
  driver-availability-gated addition alongside it. → blocks: **plan-decision** (test-plan §3 requires the two
  shells identical either way; layout-templates §cli owns the enumeration).
- **Zero `#[tracing::instrument]` across all 7 `#[tauri::command]` sites** (`commands.rs` · `main.rs` ·
  `pause.rs`), against obs-plan §4's requirement of a `tauri.command.*` handler span with `run_id`
  correlation. → blocks: **implementation-scope** — it does not block the probe, but any leg asserting on a
  driven run's self-obs lines would find no span. Recorded here as a measured gap; whether it is this chunk's
  to close or a separate obs entry is a P5/operator call, not a research verdict.
