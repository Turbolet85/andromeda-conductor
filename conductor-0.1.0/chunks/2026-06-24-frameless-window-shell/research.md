# Codebase Research — 2026-06-24-frameless-window-shell

## Scope
- **Depth:** moderate · **Reads:** 11 (obs.rs, cli main.rs, cli paths.rs, workspace Cargo.toml, tauri Cargo.toml, tauri main.rs, ui App.tsx, ui main.tsx, ui index.html, vite.config.ts, deny.toml) · **Globs/Greps:** 2 (tauri tree, ui/styles) · **code-graph queries:** 3

## Files inspected
- `crates/conductor-tauri/src/main.rs` (full) — STUB: `fn main(){ conductor_core::init_observability("conductor-tauri", None, ObsSink::Stderr); }`. No `tauri::Builder`, no event loop. This chunk replaces it with a real Tauri app + the File sink.
- `crates/conductor-tauri/Cargo.toml` (full) — only dep is `conductor-core.workspace = true`. No `tauri`, no `tauri-build`, no `build.rs`.
- `crates/conductor-core/src/obs.rs` (full) — `pub enum ObsSink { Stderr, AgentFile(PathBuf) }` (`Debug,Clone`). `init_observability(name, run_id, sink) -> ServiceIdentity`. `resolve_writer` (89) maps `AgentFile`→`open_agent_file` (101, **create_dir_all + truncate** = `-latest` semantics) → `ObsWriter::File(Arc<Mutex<File>>)`. The file sink **inherits processor-stage redaction** (test `agent_file_sink_writes_redacted_json_to_the_file`, 500) and **falls back to stderr** when unopenable (infallible, test 517). `ServiceIdentity::resolve` (37) reads `$CONDUCTOR_SERVICE_NAME` (default arg), `$CONDUCTOR_ENV`, `service.version` from `env!("CARGO_PKG_VERSION")`.
- `crates/conductor-cli/src/main.rs` (full) — the ObsSink consumer: `obs_sink(agent_mode)` (45) = `paths::agent_log_path().map(ObsSink::AgentFile).unwrap_or(ObsSink::Stderr)`. Calls `init_observability("conductor", Some(run_id), …)` (29). Pattern to mirror in tauri main.
- `crates/conductor-cli/src/paths.rs` (full) — `agent_log_path()` (71): `logs/agent-latest.jsonl` as a **sibling of the runs dir** (`resolve_handle(CONDUCTOR_RUNS_DIR,"runs").parent()/logs`), through `conductor_core::resolve_under` (traversal guard). The exact resolution pattern to mirror for `conductor-tauri.jsonl`.
- `Cargo.toml` (workspace, full) — `[workspace.dependencies]` **already has `tauri = "2.10.3"`** (line 72, "Epoch 9 … unused until then"). **No `tauri-build`** entry — must be added. `resolver = "3"`, edition 2024, MSRV 1.94.1.
- `crates/conductor-tauri/ui/src/App.tsx` (full) — a design-token showcase placeholder: a `<main>` with status-tier lamps + type tiers + surface swatches, all `var(--token)`-bound. Already a `main` landmark; this chunk adds a `banner` titlebar above it.
- `crates/conductor-tauri/ui/src/main.tsx` (full) — imports the 5 Fontsource WOFF2 faces + `./styles/tokens.css` + `App`, mounts in `StrictMode`. Standard.
- `crates/conductor-tauri/ui/index.html` (full) — minimal `<div id="root">` + module script. No CSP meta (Tauri 2 CSP lives in `tauri.conf.json` `app.security.csp`).
- `crates/conductor-tauri/ui/vite.config.ts` (full) — `plugins:[react(), tailwindcss()]`, `build.outDir:'dist'`. No `server` block → Vite default port. Tauri dev needs a fixed `server.port` + `strictPort` + `clearScreen:false` aligned with `build.devUrl`.
- `crates/conductor-tauri/ui/src/styles/tokens.css` (present, via Glob) — the 34 `:root` tokens + 6 type-role classes (design-token-bundle chunk). Titlebar binds these by name.
- `deny.toml` (full) — `[licenses] allow` is only `MIT, Apache-2.0, Unicode-3.0, BSL-1.0, BSD-3-Clause, Zlib`. `[advisories] ignore` = `[RUSTSEC-2025-0119]`. `bans.multiple-versions="warn"`, `wildcards="deny"`. **The Tauri tree will add licenses + crates not yet allowlisted.**

## Graph impact (from the code-graph query)
- **`ObsSink`** — referenced ONLY at `crates/conductor-cli/src/main.rs:9,44,46,48` (import + the `obs_sink` fn; `AgentFile`@46, `Stderr`@46/48). **Zero references in any other crate.** → generalizing/renaming the `AgentFile` variant has a **1-file** caller blast radius (cli main.rs) + obs.rs's own tests; no seam crate touches it.
- **`init_observability`** — called at exactly two bin sites: `conductor-cli/src/main.rs:28` + `conductor-tauri/src/main.rs:3` (+ the `conductor-core/src/lib.rs:34` re-export). Confirms only the two shells init obs.
- **`conductor-tauri` crate edges** — single outbound `conductor-tauri → conductor-core`; **no inbound** consumers (it is a leaf bin). An additive change here has zero cross-crate blast beyond the core `ObsSink` API.

## Patterns detected
- **Truncating file sink + redaction inheritance** (`obs.rs:89-106` + test `:500`): `AgentFile(PathBuf)` already does create-parent + truncate-open + `Arc<Mutex<File>>` `MakeWriter`, inheriting the `JsonObsLayer` processor redaction. The Tauri sink reuses this verbatim — **no new redaction policy** (obs-plan §3 / scope CARRY).
- **Runs-dir-sibling log path via `resolve_under`** (`paths.rs:71-76`): `logs/<name>.jsonl` resolved as a sibling of `CONDUCTOR_RUNS_DIR` through the traversal guard. Mirror for `conductor-tauri.jsonl`.
- **Bin obs bootstrap** (`cli main.rs:28-29`): `mint_run_id()` (or `None`) → `init_observability(service, run_id, sink)` once before logic. Tauri mirrors with `"conductor-tauri"`.
- **Frontend tokens-by-name** (`App.tsx`, `main.tsx`): every color/space/radius is `var(--token)`; fonts + `tokens.css` imported in `main.tsx`. Titlebar follows — no hardcoded hex/px.
- **Workspace dep inheritance** (`Cargo.toml`): seams use `<dep>.workspace = true`. The crate adds `tauri.workspace = true` + a workspace `tauri-build`.

## Conventions to follow
- **Core runtime ≠ Tauri runtime** (architecture §Async Runtime Flavor): Tauri owns its own event loop/runtime as the GUI shell; never run core timeline work on it. This chunk only starts the shell — no timeline wiring — so the two never meet yet.
- **Service identity**: `init_observability("conductor-tauri", …)` ⇒ `service.name="conductor-tauri"` (obs.rs:37), overridable by `$CONDUCTOR_SERVICE_NAME` (obs-plan §3).
- **Deny-by-default capabilities** (security-plan §Tauri GUI): a `capabilities/*.json` with a minimal allowlist; `core:window:allow-start-dragging` is required for `data-tauri-drag-region` to function; later command perms are added with their commands.
- **`tauri ≥2.10.3`** (CVE-2026-42184) — already satisfied by the workspace pin.
- **Status never color-alone / semantic-HTML-first** (a11y.md): `banner` landmark; window controls are native `<button>` with `aria-label` + visible `--color-focus` ring.

## New files to create
- `crates/conductor-tauri/build.rs` — `fn main(){ tauri_build::build() }`.
- `crates/conductor-tauri/tauri.conf.json` — `productName`, `identifier` (`com.andromeda.conductor`), `build` (`frontendDist:"ui/dist"`, `devUrl`, `beforeDevCommand`/`beforeBuildCommand` driving `ui/` npm), `app.windows[0]` (`decorations:false`, title, size), `app.security.csp`, `bundle` (icon set).
- `crates/conductor-tauri/capabilities/default.json` — deny-by-default ACL: window `"main"`, permissions = `core:window:allow-start-dragging` (+ `allow-minimize`/`allow-close` iff controls are in scope — D2).
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx` — the `banner` drag-region titlebar (`data-tauri-drag-region`) + optional window-control `<button>`s.
- `crates/conductor-tauri/icons/` — a minimal app-icon set referenced by `bundle.icon` (Tauri context generation expects icons; provide placeholders).

## Files to modify
- `Cargo.toml` (workspace) — add `tauri-build = "2"` to `[workspace.dependencies]` (sibling of the existing `tauri` pin).
- `crates/conductor-tauri/Cargo.toml` — add `tauri = { workspace = true, features = [...] }` + `[build-dependencies] tauri-build = { workspace = true }`.
- `crates/conductor-tauri/src/main.rs` — replace stub with the `tauri::Builder` app; resolve the tauri log path; `init_observability("conductor-tauri", None, <File sink>)`; add `#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]`.
- `crates/conductor-core/src/obs.rs` — **D1**: generalize `ObsSink::AgentFile(PathBuf)`→`ObsSink::File(PathBuf)` (+ rename `open_agent_file`→`open_log_file`) and update the 2 tests, OR add a parallel variant. (See Open questions.)
- `crates/conductor-cli/src/main.rs` — track the D1 rename (`ObsSink::AgentFile`→`ObsSink::File` at line 46) — the only non-core caller.
- `crates/conductor-tauri/ui/src/App.tsx` — restructure to render `<Titlebar/>` (banner) above the existing `<main>` console body.
- `crates/conductor-tauri/ui/vite.config.ts` — add `server:{port,strictPort:true}` + `clearScreen:false` aligned with `tauri.conf.json` `devUrl`.
- `crates/conductor-tauri/ui/package.json` — add `@tauri-apps/api` (iff window controls — D2) + optional `@tauri-apps/cli` devDep; regenerate + commit `package-lock.json`; `npm audit` clean.
- `crates/conductor-tauri/ui/index.html` — optional: suppress context-menu / set `html,body{overflow:hidden}` for the frameless shell.
- `deny.toml` — extend `[licenses] allow` with each NEW license the Tauri tree introduces (with justifying comments) + any non-actionable `[advisories] ignore`; **never a silent skip**.
- `Cargo.lock` (+ `package-lock.json`) — regenerated by the Tauri tree; commit un-drifted.

## Open questions
- **D1 — ObsSink shape:** generalize `AgentFile(PathBuf)`→`File(PathBuf)` (DRY; 1 cli call-site + 2 obs tests to track; recommended) vs add a redundant `TauriFile`/`BackendFile` variant (cli untouched, but two variants with identical behavior). → P4 AskUserQuestion.
- **D2 — Window controls scope:** include minimize+close `<button>`s (adds `@tauri-apps/api` npm dep + `allow-minimize`/`allow-close` perms; operable shell) vs drag-region-only titlebar (no npm dep; window closable only via OS). → P4 AskUserQuestion.
- **D3 — App icon:** `tauri::generate_context!` / `bundle.icon` expects an icon set; provide a minimal placeholder (or Tauri default) so `cargo build -p conductor-tauri` succeeds — resolve at implement (mechanical).
