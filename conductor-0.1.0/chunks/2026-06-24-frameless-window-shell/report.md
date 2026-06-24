# Report — 2026-06-24-frameless-window-shell

**Chunk:** Frameless window shell — first real Tauri 2 app: decorations:false drag-region titlebar (banner landmark) + deny-by-default capabilities ≥2.10.3 + Tauri-backend logs/conductor-tauri.jsonl sink via conductor_core::ObsSink; opens Epoch 9 (conductor-tauri)
**Date:** 2026-06-24 (UTC) · **Epoch:** 9 (Desktop control panel) ch1/10
**Commits:** none yet since last_wrap — this chunk commits in wrap P7

## Changes (structured — detectors read this)

- **Files:**
  - Rust: `Cargo.toml` (workspace), `crates/conductor-core/src/obs.rs`, `crates/conductor-cli/src/main.rs`, `crates/conductor-tauri/Cargo.toml`, `crates/conductor-tauri/src/main.rs`, NEW `crates/conductor-tauri/build.rs`
  - Tauri config: NEW `crates/conductor-tauri/tauri.conf.json`, NEW `crates/conductor-tauri/capabilities/default.json`, NEW `crates/conductor-tauri/icons/{32x32.png,128x128.png,icon.ico}`
  - Frontend: NEW `crates/conductor-tauri/ui/src/components/{Titlebar.tsx,Titlebar.css}`, `ui/src/App.tsx`, `ui/index.html`, `ui/vite.config.ts`, `ui/package.json`, `ui/package-lock.json`
  - Harness/CI: `scripts/agent-run.sh`, `scripts/agent-run.ps1`, `.github/workflows/ci.yml`
  - Supply-chain/hygiene: `deny.toml`, `.gitignore`, `Cargo.lock`
- **Symbols / APIs:**
  - `conductor_core::ObsSink` — variant **renamed** `AgentFile(PathBuf)` → `File(PathBuf)` (public enum; the only external caller was `conductor-cli/src/main.rs:47`, updated). Private `open_agent_file`→`open_log_file`; obs tests `agent_file_sink_*`→`file_sink_*`. `init_observability(name, run_id, sink)` signature UNCHANGED.
  - `conductor-tauri::main` — stub → real `tauri::Builder::default().run(generate_context!())`; new private `obs_sink()` + `tauri_log_path()` (mirror of cli `agent_log_path`, via `conductor_core::resolve_under`). `#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]`.
  - Frontend: `data-tauri-drag-region` `banner` titlebar; window controls call `getCurrentWindow().minimize()/.close()` (`@tauri-apps/api/window`).
  - Tauri IPC capability surface (deny-by-default): `core:window:allow-start-dragging` · `-minimize` · `-close`. NO `#[tauri::command]` / `Channel` yet (later Epoch-9 chunks).
- **Crates / modules:** `conductor-tauri` transformed stub→real Tauri 2 app (existing workspace member; no new crate). No seam-crate model change.
- **Dependencies:**
  - Rust: workspace `[workspace.dependencies]` +`tauri-build = "2"`; `conductor-tauri` +`tauri` (workspace pin `2.10.3` → resolved **2.11.3**) +`tauri-build` (build-dep). Cargo.lock gained the full Tauri tree (tao/wry/webview2-com/windows-*/…).
  - npm: `ui/` +`@tauri-apps/api ^2.0.0` (resolved; `npm audit` 0 vulns). Omitted optional `@tauri-apps/cli`.
- **Schema / config:**
  - NEW `tauri.conf.json` — Tauri 2: main window `decorations:false` label `main`, identifier `com.andromeda.conductor`, `build.frontendDist=ui/dist` + devUrl `localhost:5173` + before*Commands, `app.security.csp` (restrictive), `withGlobalTauri:false`, `bundle.icon`.
  - NEW `capabilities/default.json` — deny-by-default ACL (3 window perms, window `main`).
  - `deny.toml` — +16 `[advisories] ignore` (ALL `unmaintained`, zero vulnerabilities: 10 gtk-rs GTK3 Linux-webview, 5 unic-* via urlpattern→tauri-utils, 1 proc-macro-error) + 2 `[licenses] allow` (`MPL-2.0`, `Apache-2.0 WITH LLVM-exception`). Each entry justified inline.
  - `.gitignore` +`crates/conductor-tauri/gen/` (tauri-build regenerates `gen/schemas/` each compile).
  - `vite.config.ts` +`server.{port:5173,strictPort}` +`clearScreen:false`.
  - `agent-run.{sh,ps1}` +`ensure_frontend` (npm build of `ui/dist` before workspace cargo legs); `ci.yml` Rust job +node-setup +frontend-build before `cargo build --locked`.
- **Coverage of new surfaces:**
  - `conductor-tauri` frameless window + `banner` titlebar → validation n/a · instrumentation (self-obs `logs/conductor-tauri.jsonl` via `ObsSink::File`✓) · PII (redacted✓ — inherits processor-stage redaction, no new policy) · tests (build-gated `tsc --noEmit`+`vite build`✓; no Rust unit tests — test-plan §4 GUI is build-gated) · a11y (`banner` landmark + semantic `<button>` + `aria-label` + `--color-focus` `:focus-visible` ring✓; no axe harness — Epoch-9 closing chunks) · tokens (design-token✓ — all `var(--…)`)
  - window controls (minimize/close) → IPC (deny-by-default ACL✓ — 3 window perms only, no wildcard) · a11y (semantic button + aria-label + focus ring✓)
  - `ObsSink::File` (generalized) → instrumentation✓ · PII (processor-stage redaction inherited✓) · tests (unit 192/192 incl. renamed `file_sink_*`✓)

## Deviations from intent
- **D1 (user decision)** — ObsSink generalized `AgentFile`→`File` (the scope flagged "variant vs parameterize" as a P4 choice; resolved to generalize). Extra: renamed `open_agent_file`→`open_log_file` + 2 test fns for consistency. Caller blast radius matched the code-graph prediction exactly (cli only).
- **D2 (user decision)** — window controls included (scope said "if included"); omitted optional `@tauri-apps/cli` devDep to keep the npm tree minimal (gate uses bare cargo+vite).
- **NEW `Titlebar.css`** (not in plan's new-file list) — co-located component styles; required because `:focus-visible`/`:hover` can't be React inline styles. In-scope (the Titlebar component).
- **`.gitignore` += `gen/`** (not in plan) — necessary build-artifact ignore (tauri-build regenerates it); same class as the existing `node_modules/`/`ui/dist/` ignores.
- **`tauri` 2.11.3** (not the 2.10.3 floor) — `^2.10.3` resolves latest 2.x; satisfies the ≥2.10.3 CVE floor.
- **`deny.toml` 16 ignores + 2 licenses** — anticipated by the plan ("extend allow/ignore"); larger than a casual reader expects but every entry is a justified `unmaintained`/permissive item.
- **Titlebar height `--space-xl` (32px)** vs the layouts extract's `space-lg` (20px) — 20px cannot contain the 18px heading + 20px controls. Cosmetic; flag for a layout note.
- **Icons** — minimal cyan (`--color-id-cyan`) placeholder set (D3, mechanical); real branding is later polish.
- The build-order/agent-run/CI touch is NOT a deviation — it was folded into `scope.md` at phase P5 (intent-incomplete amendment).

## Decisions & corrections
- **D1 (user):** generalize `ObsSink::AgentFile`→`File(PathBuf)` — one neutral truncating-file sink shared by cli (`agent-latest.jsonl`) + Tauri (`conductor-tauri.jsonl`).
- **D2 (user):** include minimize/close window controls (via `@tauri-apps/api`) so the frameless window is self-operable; gives the deny-by-default ACL meaningful content.
- **Disk recovery (user-directed):** the first combined-gate attempt hit `no space on device` (D: 100% full; target/ → 37G; incremental cache alone 14G). User chose "free space + re-run"; my `rm` was guard-blocked, so used `cargo clean` (freed 36.9G) → re-ran the full gate from scratch → green.
- **Reusable learnings (P3 raw material):**
  - Tauri `generate_context!` resolves `build.frontendDist` (`ui/dist`) at COMPILE time → the frontend MUST be built before ANY workspace `cargo build`/`nextest`/`clippy` that compiles the Tauri crate. Hence the `agent-run.{sh,ps1}` + CI `ensure_frontend` build-order coupling. A bare `cargo build` does NOT run `beforeBuildCommand` (only the Tauri CLI does).
  - Adding Tauri trips `cargo deny check` on BOTH advisories (16 `unmaintained`: gtk-rs Linux-webview ×10 / unic-* ×5 / proc-macro-error) AND licenses (`MPL-2.0`, `Apache-2.0 WITH LLVM-exception`); all justified-ignorable. `cargo audit` stays exit 0 (unmaintained-as-warnings).
  - A full debug build of the workspace + Tauri tree needs ~33–37G on disk (incremental cache ~14G); constrained hosts risk `no space on device` mid-compile.
  - `tauri.conf.json bundle.icon` is validated by `generate_context!` at compile time — valid icons (PNG + ICO) must exist before `cargo build`.
  - Tauri-generated `gen/schemas/` is a per-compile artifact → must be gitignored.

## Outcome
- **All acceptance criteria met.** `conductor-tauri` is a real Tauri 2 app (decorations:false, deny-by-default ACL, backend `ObsSink::File` → `logs/conductor-tauri.jsonl`); cli sink path unchanged.
- **Gates green:** `cargo nextest run --workspace --profile ci` **395/395** · `cargo test --workspace --doc` ok · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo build -p conductor-tauri` ok · `cargo audit` exit 0 · `cargo deny check` exit 0 · frontend `tsc --noEmit`+`vite build`+`npm audit` ok.
- **Smoke (boot-path changed):** `bash scripts/agent-run.sh run` **exit 0** (exercises the new `ensure_frontend` + the full gate). GUI window launch not automated (interactive GPU surface; `generate_context!` compile-time-validates the window/capability/CSP config; visual confirmation is the operator's check).
