# Fan-out results — 2026-10-11-conductors-window-retired (wrap run 2026-10-11T08-28-34-wrap, resumed)

Each doc: its verdict line, then the parsed list (extracted programmatically from the doc-agent's handback, HTML
entities decoded, `entities=0` probed after decode, parsed as YAML). Dispositions follow in § Validate.

## architecture

verdict: 57 proposal(s) · entities after decode 0 · parse ok · stripped: 0 preamble line(s), 0 trailing line(s)

```yaml
proposals:
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Workspace / Core Structure]'
    change: 'Runtime-agnostic core library + the one CLI bin (`conductor-cli`): engine logic lives in a core that `agent-run` calls; the Tauri GUI bin was retired 2026-10-11, so one binary crate remains and the headless/CLI split is enforced at compile time.'
    sidecar: '2026-10-11 conductors-window-retired: [Workspace / Core Structure] drops the Tauri bin, leaving one binary crate (conductor-cli).'
    rationale: 'Report Changes → Crates/modules: `conductor-tauri` (the GUI bin) removed, "One binary crate remains (`conductor-cli`)". The locked decision "core library + CLI bin + Tauri bin" now contradicts the tree.'
    basis: 'Cargo.toml [workspace] members'
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Module Boundaries]'
    change: 'The workspace is eight members: `conductor-core` + the seam crates + `conductor-run` + the `conductor-cli` bin. The standalone sweep takes the eight members each on its OWN targets: `--lib` for the seven library crates and `--bins` for `conductor-cli`, which has no lib target.'
    sidecar: '2026-10-11 conductors-window-retired: [Module Boundaries] goes from nine members to eight and from two bins to one.'
    rationale: 'Report Counts/qualifiers: "Workspace members 9 → 8; binary crates 2 → 1", which the report says is stated at architecture.md:55. Outcome gate: per-seam sweep `for p in conductor-core … --lib --locked …; cargo build -p conductor-cli --bins --locked` green.'
    basis: '.andromeda/architecture.md:55'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Design Philosophy'
    change: '"Headless-drivable core, thin shell": all engine logic lives in runtime-agnostic library crates that `scripts/agent-run.sh` (the source of truth) drives through `conductor-cli`. Remove the "optional Tauri 2 GUI" and "GUI is convenience only" clauses. The real-model sentence reads "while `conductor suite` keeps the deterministic gate", with no GUI.'
    sidecar: '2026-10-11 conductors-window-retired: Design Philosophy no longer names a Tauri GUI shell.'
    rationale: 'Report Crates/modules: the GUI bin was removed and one binary crate remains.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies (Backend framework row)'
    change: 'N/A, no web framework. Conductor exposes no HTTP/network service of its own (gRPC client + MCP client only); drop "+ Tauri IPC".'
    sidecar: '2026-10-11 conductors-window-retired: Stack Backend-framework row drops Tauri IPC.'
    rationale: 'Report Symbols/APIs: the window''s eight IPC handlers were removed with the crate.'
    basis: 'crates/conductor-tauri/src/main.rs:22-31@321dc8f'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies (Hashing / digest row)'
    change: '`sha2 0.10` is locked by its own `conductor-run` dev-dependency edge. Replace "already locked via `tauri-codegen` / `wry`".'
    sidecar: '2026-10-11 conductors-window-retired: sha2''s lock basis moves from the Tauri tree to its conductor-run dev edge.'
    rationale: 'Report Dependencies: "`sha2` stays (its `conductor-run` dev edge now locks it)", and no window-toolkit package remains in Cargo.lock.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies (Push / real-time row)'
    change: 'Retire the row. No in-app push surface remains, because the Tauri 2 IPC `Channel` left with the window crate.'
    sidecar: '2026-10-11 conductors-window-retired: Stack Push/real-time row (Tauri Channel) retired.'
    rationale: 'Report Symbols/APIs: "the live-counter `Channel`" was removed with the crate.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies (Desktop shell row)'
    change: 'Retire the row. No desktop shell remains; the `tauri` crate, `tauri-build` and the host `tauri-cli` bundler are gone.'
    sidecar: '2026-10-11 conductors-window-retired: Stack Desktop-shell row (Tauri 2) retired.'
    rationale: 'Report Dependencies: workspace deps `tauri = "2.10.3"` and `tauri-build = "2"` left Cargo.toml, and no window-toolkit package remains. Dev-tool versions: `tauri-driver` / `tauri-cli` leave with the jobs and arms that used them.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies (Desktop frontend (webview) row)'
    change: 'Retire the row (React 19 + Vite + Tailwind + Fontsource + `@tauri-apps/api`, npm). No npm tree remains.'
    sidecar: '2026-10-11 conductors-window-retired: Stack Desktop-frontend row retired with the ui/ tree.'
    rationale: 'Report Files: 47 files under `ui/` were deleted. Dependencies: "no npm tree remains (`git ls-files ''*package.json''` → 0)".'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies (Build / packaging row)'
    change: 'Cargo workspace with `cargo build --release`, producing the local release binary (the source of truth). Drop "+ Tauri 2 bundler" and "optional GUI bundle".'
    sidecar: '2026-10-11 conductors-window-retired: Stack Build/packaging row drops the Tauri bundler.'
    rationale: 'Report Crates/modules and Dev-tool versions: the GUI bin and `tauri-cli` are gone.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Async Runtime Flavor]'
    change: 'tokio `current_thread`, core-owned: `#[tokio::main(flavor = "current_thread")]` on the CLI path. Drop the Tauri arm (the hand-built `Builder::new_current_thread()` under Tauri and Tauri''s own multi_thread runtime).'
    sidecar: '2026-10-11 conductors-window-retired: [Async Runtime Flavor] Tauri arm retired.'
    rationale: 'Report Crates/modules: the GUI bin that owned the Tauri runtime arm was removed. The report''s Expected amendments name "[Async Runtime Flavor]''s Tauri arm".'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Backend Framework]'
    change: 'Conductor is a gRPC client + MCP client, not a server. Drop "+ Tauri IPC host".'
    sidecar: '2026-10-11 conductors-window-retired: [Backend Framework] drops the Tauri IPC host.'
    rationale: 'Report Symbols/APIs: the eight IPC handlers were removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Accepted Capability Set]'
    change: 'The binary edge that takes a `&CapabilityManifest` is `conductor-cli::paths` alone. Drop `conductor-tauri::commands`.'
    sidecar: '2026-10-11 conductors-window-retired: [Accepted Capability Set] load-site list drops conductor-tauri::commands.'
    rationale: 'Report Files: `crates/conductor-tauri/src/{main,commands,pause}.rs` deleted.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Error Handling]'
    change: 'anyhow lives only in `conductor-cli`. Drop the `#[tauri::command]` edges.'
    sidecar: '2026-10-11 conductors-window-retired: [Error Handling] anyhow edge set is now conductor-cli only.'
    rationale: 'Report Symbols/APIs: the `#[tauri::command]` handlers were removed with the crate.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Real-time Strategy]'
    change: 'Retire the decision (Tauri 2 IPC `Channel` for in-app live updates). No in-app live-update surface remains.'
    sidecar: '2026-10-11 conductors-window-retired: [Real-time Strategy] retired with the Channel.'
    rationale: 'Report Symbols/APIs: "the live-counter `Channel`" was removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Established Decisions → [Deployment]'
    change: 'Local `cargo build --release` + `scripts/agent-run.sh`. Headless is the source of truth, co-located with Pulse (loopback `:4317`). Drop the Tauri 2 bundler / `tauri-cli` GUI-artifact clause. No cloud / container / serverless.'
    sidecar: '2026-10-11 conductors-window-retired: [Deployment] drops the Tauri bundler GUI artifact.'
    rationale: 'Report Crates/modules and Dev-tool versions: the GUI bin and `tauri-cli` are gone.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Conventions (Internal core↔UI)'
    change: 'Retire the bullet (`#[tauri::command]` request/response + `Channel`). No UI surface remains.'
    sidecar: '2026-10-11 conductors-window-retired: Conventions Internal core↔UI bullet retired.'
    rationale: 'Report Symbols/APIs: the IPC handlers and the `Channel` were removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Conventions (Error handling)'
    change: '`anyhow` only at `conductor-cli` edges. Drop `#[tauri::command]`.'
    sidecar: '2026-10-11 conductors-window-retired: Conventions Error-handling edge set is conductor-cli only.'
    rationale: 'Report Symbols/APIs: the `#[tauri::command]` handlers were removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Standard Contracts (Readiness gate)'
    change: '`preflight()` remains what `conductor suite` and `drive_run` call. Drop "the GUI''s run thread". `drive_run` keeps test callers only until `Panel-shaped types retired`.'
    sidecar: '2026-10-11 conductors-window-retired: Readiness gate caller list drops the GUI run thread.'
    rationale: 'Report Symbols/APIs, remaining-caller facts: `drive_run` keeps test callers and its `pub use` re-export only, because the window was its sole production caller.'
    basis: 'conductor-run/src/drive.rs:53'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Standard Contracts (Live update channel)'
    change: 'Retire the paragraph (backend→frontend Tauri 2 IPC `Channel`). There is no live-update surface and no SSE/WebSocket/polling HTTP surface.'
    sidecar: '2026-10-11 conductors-window-retired: Live update channel contract retired.'
    rationale: 'Report Symbols/APIs: the live-counter `Channel` was removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Ports: no inbound listener in the shipped binaries)'
    change: 'No inbound listener of Conductor''s own in the shipped binary (no API port, no DB port, since SQLite is in-process). Drop the frontend dev-server / Tauri webview `custom-protocol` / `devUrl` (`localhost:5173`) clause.'
    sidecar: '2026-10-11 conductors-window-retired: Ports bullet drops the Tauri webview devUrl clause.'
    rationale: 'Report Crates/modules and Harness/gate: the GUI bin and its `--features tauri/custom-protocol` build are gone.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Tauri commands (internal IPC))'
    change: 'Retire the entry. The eight IPC handlers and the `Channel` left with `conductor-tauri`.'
    sidecar: '2026-10-11 conductors-window-retired: Tauri commands IPC registry entry retired (8 handlers).'
    rationale: 'Report Symbols/APIs lists the eight removed handlers (`start_run` … `resolve_operator_hold`).'
    basis: 'crates/conductor-tauri/src/main.rs:22-31@321dc8f'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Route prefixes)'
    change: 'The consumed MCP tool names are the complete route-equivalent set. Drop "and the Tauri command names above".'
    sidecar: '2026-10-11 conductors-window-retired: Route-prefix set drops Tauri command names.'
    rationale: 'Report Symbols/APIs: the IPC handlers were removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Service / process names)'
    change: 'Retire "Conductor Tauri app (GUI bin), optional control panel". `conductor-cli` is the one process of Conductor''s own.'
    sidecar: '2026-10-11 conductors-window-retired: GUI-bin process name retired.'
    rationale: 'Report Crates/modules: `conductor-tauri` (the GUI bin) removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Service / process names: launched Pulse MCP child)'
    change: 'The `CREATE_NO_WINDOW` sentence reads "so a launched sidecar raises no console pane". Drop "GUI-launched", because no GUI launcher remains.'
    sidecar: '2026-10-11 conductors-window-retired: sidecar console-suppression rationale no longer names a GUI launcher.'
    rationale: 'Report Crates/modules: the GUI bin was removed and one binary crate remains.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Crate names (workspace members))'
    change: 'Eight members: `conductor-core`, `-timeline`, `-emit`, `-faults`, `-verify`, `-report`, `-run` (sitting above the seams and below the `conductor-cli` bin), and `conductor-cli`. Drop "plus the `conductor-tauri` bin" and "below both bins, shared by them".'
    sidecar: '2026-10-11 conductors-window-retired: crate-name registry 9 → 8, conductor-tauri removed.'
    rationale: 'Report Crates/modules lists the eight members from `Cargo.toml [workspace] members`.'
    basis: 'Cargo.toml [workspace] members'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (Frontend asset subtree)'
    change: 'Retire the entry. `crates/conductor-tauri/ui/` (the Vite + React SPA, `tokens.css`, `package-lock.json`) is deleted; no npm tree remains, and the `ui/logs/`, `node_modules/`, `ui/dist/` and `gen/` ignore rules left `.gitignore`.'
    sidecar: '2026-10-11 conductors-window-retired: frontend asset subtree retired; no npm tree remains.'
    rationale: 'Report Files (47 `ui/` files deleted), Dependencies (npm tree left whole) and Schema/config (`.gitignore` rules left).'
    basis: '.gitignore:19'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (artifact paths: `logs/conductor-tauri.jsonl`)'
    change: 'Retire the entry and its per-suite a11y landing sites. The Tauri backend self-obs sink (`tauri_log_path()`) is gone, and the `.gitignore` self-obs comment no longer names `conductor-tauri.jsonl`.'
    sidecar: '2026-10-11 conductors-window-retired: logs/conductor-tauri.jsonl artifact path retired with tauri_log_path().'
    rationale: 'Report Symbols/APIs: "the self-obs sink helper `tauri_log_path()` (`logs/conductor-tauri.jsonl`)" was removed. Schema/config: the `.gitignore` self-obs comment changed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Occupied Resources (env vars: `CONDUCTOR_SERVICE_NAME`)'
    change: 'Overrides the self-observation `service.name` field (default `conductor`). Drop the `conductor-tauri` default.'
    sidecar: '2026-10-11 conductors-window-retired: CONDUCTOR_SERVICE_NAME loses the conductor-tauri default.'
    rationale: 'Report Crates/modules: the `conductor-tauri` bin, whose identity was that default, was removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Cross-cutting Patterns (Development Style)'
    change: 'agent-driven: the headless `scripts/agent-run.sh` path is the source of truth and the release gate. Drop "the GUI is a thin shell over the same core commands".'
    sidecar: '2026-10-11 conductors-window-retired: Development Style drops the GUI shell.'
    rationale: 'Report Crates/modules: the GUI bin was removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Project Intent (Product type)'
    change: 'Headless CLI harness (`conductor-cli`) over a Rust core, a scenario-driven OTLP fault-injection and verification harness … Replace "Desktop control-panel app (Tauri 2)".'
    sidecar: '2026-10-11 conductors-window-retired: Product type is now a headless CLI harness, not a Tauri desktop app.'
    rationale: 'Report Crates/modules: "One binary crate remains (`conductor-cli`)", and the GUI bin was removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Inherited Defaults (Backend framework)'
    change: 'None: headless Rust core, no HTTP/network service (gRPC client + MCP client). Drop "+ Tauri IPC".'
    sidecar: '2026-10-11 conductors-window-retired: Inherited Defaults backend framework drops Tauri IPC.'
    rationale: 'Report Symbols/APIs: the IPC handlers were removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Inherited Defaults (Interface style)'
    change: 'Drop "internal Tauri 2 commands + `Channel`". OTLP/gRPC egress + MCP read-back remain the interface set; no REST/GraphQL/tRPC.'
    sidecar: '2026-10-11 conductors-window-retired: Inherited Defaults interface style drops Tauri commands + Channel.'
    rationale: 'Report Symbols/APIs: the IPC handlers and the `Channel` were removed.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Inherited Defaults (Deployment)'
    change: 'Local `cargo build --release` + `scripts/agent-run.sh` (source of truth). Drop the optional Tauri 2 GUI bundle / `tauri-cli` clause. No cloud/container/serverless.'
    sidecar: '2026-10-11 conductors-window-retired: Inherited Defaults deployment drops the Tauri GUI bundle.'
    rationale: 'Report Crates/modules and Dev-tool versions: the GUI bin and `tauri-cli` are gone.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Inherited Defaults (Frontend (optional GUI))'
    change: 'Retire the bullet (React/Vite/Tailwind under `crates/conductor-tauri/ui/`, npm + `npm audit --omit=dev` gate). No frontend and no npm tree remain.'
    sidecar: '2026-10-11 conductors-window-retired: Inherited Defaults frontend bullet retired; npm audit gate gone.'
    rationale: 'Report Dependencies: no npm tree remains. Harness/gate: the `frontend` job, including the npm audit gate, was removed whole.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Build system'
    change: 'Retire the `conductor-tauri` / `tauri-build` / `generate_context!` / `ensure_frontend` / `custom-protocol` paragraph. Replace the "Tauri tree adds many unmaintained advisories + MPL-2.0 / LLVM-exception" sentence with: `deny.toml` `[advisories] ignore` is 0 and `[licenses] allow` is 8 (MPL-2.0 retired), and `Apache-2.0 WITH LLVM-exception` is KEPT for its 16 current holders (`blake3`, `rustix`/`linux-raw-sys`, the `wasi`/`wit-bindgen`/`wasm-*` tree). The current reading is `cargo audit` with 1296 advisories, 268 crates and no allowed warnings. Keep the 2026-09-05 reading as dated history.'
    sidecar: '2026-10-11 conductors-window-retired: Build system drops the Tauri build chain; deny.toml ignores 16 → 0 and allows 9 → 8.'
    rationale: 'Report Schema/config (deny.toml counts and holders), Counts (`Cargo.lock` 562 → 268, cargo audit 268 crates / 1296 advisories), and Harness/gate (`ensure_frontend` / `Invoke-EnsureFrontend` left both shells).'
    basis: 'deny.toml:14-18'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Deployment model'
    change: 'Local-only: `cargo build --release` produces the `conductor-cli` binary, run beside Pulse via `scripts/agent-run.sh`. Retire the Tauri 2 bundler / `tauri-cli` / installer-set (nsis/msi) sentences. No Docker/Compose/Kubernetes/serverless.'
    sidecar: '2026-10-11 conductors-window-retired: Deployment model drops the Tauri bundler and its installer set.'
    rationale: 'Report Crates/modules and Dev-tool versions: the GUI bin and `tauri-cli` are gone.'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: '§Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace)'
    change: 'Remove the `conductor-tauri/` row. `conductor-core` reads "runtime-agnostic engine library", with no "(usable outside Tauri)". `conductor-run` reads "used by the `conductor-cli` bin", not "shared by both bins". The LICENSE row reads "every Cargo manifest declares `MIT OR Apache-2.0`", with no npm.'
    sidecar: '2026-10-11 conductors-window-retired: directory tree drops crates/conductor-tauri/ (eight members).'
    rationale: 'Report Files (`crates/conductor-tauri/` deleted whole) and Crates/modules (eight members, one bin, no npm tree).'
    dependent-of: D-arch-decisions
  - detector: D-platform-claim
    severity: warning
    section: '§Established Decisions → [CI/CD]'
    change: 'GitHub Actions runs one job, `rust`: fmt + build + test + clippy + the static gates (over committed data · repository hygiene) + supply-chain + coverage + obs/run-journal conformance. Remove the third `a11y` job, the `Pin msedgedriver …` gate, the routine webview a11y leg and its verdict sentences ("the arm is GREEN in CI on the measured configuration — hosted `windows-2022` …", "Hosted-runner runnability is MEASURED and PROVEN for that configuration and unproven elsewhere", "The leg also RUNS green on the Windows dev host through tauri-driver", "The measured SET is now: …"). Those verdicts retired 2026-10-11 with the desktop-webview a11y surface, which was removed and not re-measured elsewhere. Dynamic proof requiring a live Pulse stays an operator/local gate.'
    sidecar: '2026-10-11 conductors-window-retired: [CI/CD] goes from 3 jobs to 1 (`rust` alone); the webview a11y leg''s runner/driver verdicts (measured set: hosted windows-2025, windows-2022, Windows dev host) retired with the surface.'
    rationale: 'Report Harness/gate surface: "Verdicts RETIRED … the desktop-webview a11y leg''s runner and driver verdicts — the `--e2e` / `a11y:driven` / `sr*` legs, the tauri-driver + msedgedriver stack, the WebView2 `windows-2022` image pin — retire with the surface itself". Counts: CI jobs 3 → 1. CI#38124278202 has 1/1 checks.'
    basis: '.github/workflows/ci.yml:86'
  - detector: D-platform-claim
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'Remove the `a11y` job clause and the verdict "All three stages now EXECUTE in CI on the measured configuration: hosted `windows-2022` …", plus the `windows-2025` run-34162118841 reading. The webview leg retired 2026-10-11 with its surface (removed, not re-measured). The `rust` job alone carries the gates. Dynamic live-Pulse proof stays an operator/local gate.'
    sidecar: '2026-10-11 conductors-window-retired: CI/CD approach retires the a11y job and its hosted-runner verdict.'
    rationale: 'Report Harness/gate surface: the `a11y` job was removed whole and the leg''s runner/driver verdicts retired with the surface.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Infrastructure Patterns → Build system'
    change: 'The LF rationale reads "every CI job runs on a Windows runner: the one job, `rust`, on `windows-latest`". Retire the `a11y` job''s `windows-2022` pin rationale, the "The image no longer decides the driver" / pin-gate / runtime-floor sentences, and "The 2026-09-08 exit condition … is now MET", all with the job.'
    sidecar: '2026-10-11 conductors-window-retired: Build system drops the a11y job runner pin and msedgedriver pin-gate narrative.'
    rationale: 'Report Harness/gate surface: the `a11y` job (msedgedriver pin + Authenticode gate, six WebView2 diagnostics) was removed whole. Counts: CI jobs 3 → 1.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Stack and Technologies (CI/CD row)'
    change: 'GitHub Actions (`cargo fmt --all --check` / `cargo build` / nextest / clippy + the static gates). Drop "+ the a11y webview e2e gate" and "plus the Pulse-free routine a11y webview leg (2026-09-07)". Dynamic proof requiring a live Pulse remains a local operator gate.'
    sidecar: '2026-10-11 conductors-window-retired: Stack CI/CD row drops the a11y webview gate.'
    rationale: 'Report Harness/gate surface: the `a11y` job was removed. Counts: CI jobs 3 → 1.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (Ports: `https://msedgedriver.microsoft.com` CI-job-scoped egress)'
    change: 'Retire the entry. The non-loopback egress count is now ZERO, and the loopback-only client model holds for every shipped binary and every harness leg.'
    sidecar: '2026-10-11 conductors-window-retired: msedgedriver CI egress retired; non-loopback egress count 1 → 0.'
    rationale: 'Report Counts: "the registered network exception (the `a11y` job''s HTTPS fetch from `msedgedriver.microsoft.com`) 1 → 0".'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (Ports: `127.0.0.1:4444` + `127.0.0.1:4445`)'
    change: 'Retire the entry, including "live Pulse, never CI" and the three-suite-family stack description. The dev-only driver ports lost every binder 2026-10-11 (`wdio.conf.ts` left). `:4317` is untouched.'
    sidecar: '2026-10-11 conductors-window-retired: ports 4444/4445 retired (no binder remains).'
    rationale: 'Report Symbols/APIs, Ports: "`4444` / `4445` lose every binder (they lived in `wdio.conf.ts`). `:4317` (the port-occupier) is untouched."'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (artifact paths: `runs/a11y/<run_id>.jsonl`)'
    change: 'Retire the a11y violation-record entry. The webview gate that emitted it, the a11y violation-JSON conformance gate and the record upload left with the `a11y` job.'
    sidecar: '2026-10-11 conductors-window-retired: runs/a11y violation record retired with the webview gate.'
    rationale: 'Report Harness/gate surface: "the a11y violation-JSON conformance gate, `Upload a11y violation record`" were removed.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `CONDUCTOR_SCENARIOS_DIR`)'
    change: 'Overrides the default `scenarios/` config directory. Drop "`wdio.conf.ts` sets it for `driven`/`sr-empty`/`sr-error`".'
    sidecar: '2026-10-11 conductors-window-retired: CONDUCTOR_SCENARIOS_DIR loses its wdio setter.'
    rationale: 'Report Files: `wdio.conf.ts` deleted with the `ui/` tree.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `CONDUCTOR_MSEDGEDRIVER`)'
    change: 'Retire the entry. No reader remains.'
    sidecar: '2026-10-11 conductors-window-retired: CONDUCTOR_MSEDGEDRIVER retired (0 readers).'
    rationale: 'Report Symbols/APIs, env vars losing every reader: `CONDUCTOR_MSEDGEDRIVER` (`git grep` printed nothing).'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `CONDUCTOR_NVDA`)'
    change: 'Retire the entry. No reader remains.'
    sidecar: '2026-10-11 conductors-window-retired: CONDUCTOR_NVDA retired (0 readers).'
    rationale: 'Report Symbols/APIs, env vars losing every reader: `CONDUCTOR_NVDA`.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `CONDUCTOR_E2E_SEED_DIR`)'
    change: 'No setter remains. It is READ only by `crates/conductor-run/tests/envelope_fixture.rs` as an unset no-op. Drop the webview `--e2e` arm / `wdio.conf.ts` `onPrepare` setter clause.'
    sidecar: '2026-10-11 conductors-window-retired: CONDUCTOR_E2E_SEED_DIR loses its wdio setter; its test reader stays as an unset no-op.'
    rationale: 'Report Symbols/APIs: "`CONDUCTOR_E2E_SEED_DIR` keeps its `conductor-run` test reader as an unset no-op (its setter `wdio.conf.ts` left)".'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `CONDUCTOR_A11Y_STRICT`)'
    change: 'Retire the entry. Its readers (`wdio.conf.ts`, `assert_a11y_verdict` / `Assert-A11yVerdict`) are gone.'
    sidecar: '2026-10-11 conductors-window-retired: CONDUCTOR_A11Y_STRICT retired (0 readers).'
    rationale: 'Report Symbols/APIs (env vars losing every reader) and Harness/gate (the verdict checks left both shells).'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `EDGEWEBDRIVER`)'
    change: 'Retire the entry. The `a11y` job''s pin gate that set and read it is gone, and the Workflow env-context gate comment no longer names it.'
    sidecar: '2026-10-11 conductors-window-retired: EDGEWEBDRIVER registration retired.'
    rationale: 'Report Symbols/APIs (`EDGEWEBDRIVER` loses every reader) and Harness/gate (the env-context gate comment was re-worded generically).'
    basis: '.github/workflows/ci.yml:86'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `WEBVIEW2_USER_DATA_FOLDER` · `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`)'
    change: 'Retire the entry. The WebView2 diagnostic steps that set them left with the `a11y` job.'
    sidecar: '2026-10-11 conductors-window-retired: WEBVIEW2_* registrations retired.'
    rationale: 'Report Symbols/APIs: both lose every reader/setter. Harness/gate: six WebView2 diagnostic steps were removed.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `RUNNER_TEMP`)'
    change: 'Retire the entry. The `a11y` job steps that read it are gone.'
    sidecar: '2026-10-11 conductors-window-retired: RUNNER_TEMP registration retired.'
    rationale: 'Report Symbols/APIs: "(and the `a11y` job''s `RUNNER_TEMP` / `TEMP` reads)" lose every reader.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Occupied Resources (env vars: `TEMP` · `LOCALAPPDATA`)'
    change: 'Retire the entry. Its only basis was the `a11y` job''s `WebView2 session isolation (diagnostic)` step, which is removed.'
    sidecar: '2026-10-11 conductors-window-retired: TEMP / LOCALAPPDATA registrations retired with the a11y diagnostics.'
    rationale: 'Report Symbols/APIs (the a11y job''s `TEMP` reads lose every reader) and Harness/gate (the six WebView2 diagnostic steps were removed).'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Cross-cutting Patterns (Trust boundary)'
    change: 'Remove the dev-only webview a11y legs sentence (tauri-driver / native WebDriver on `4444`/`4445`, NVDA, the window-activation and `send-keys.ps1` spawns) and the "THIRD outbound surface exists in CI ONLY" msedgedriver clause. The outbound surfaces are OTLP/gRPC to `127.0.0.1:4317` + the MCP child, and the port-occupier stays the single deliberate bind.'
    sidecar: '2026-10-11 conductors-window-retired: Trust boundary drops the webview driver spawns and the CI-only msedgedriver egress.'
    rationale: 'Report Counts: rule (b) forms 1-4, 7 and 8 left with their files, and the network exception went 1 → 0. Symbols/APIs: ports 4444/4445 lose every binder.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Cross-cutting Patterns (Scope law)'
    change: 'Remove the "driving Conductor''s OWN webview is in-scope and ships as THREE suite families (four `wdio` suites) over ONE WebdriverIO + tauri-driver stack … NVDA''s own speech log as the driver on the Windows dev host … never a second automation stack" clause. No webview surface remains. Keep the "no automation of PULSE''s UI" non-goal.'
    sidecar: '2026-10-11 conductors-window-retired: Scope law drops the webview suite-family / driver clause.'
    rationale: 'Report Harness/gate surface: the `--e2e` / `a11y:driven` / `sr*` legs and their driver verdicts retired with the surface.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Inherited Defaults (CI/CD)'
    change: 'GitHub Actions fmt + build + test + the static gates (coverage-completeness · scenario-assertion audit; secret scan · workflow env-context). Drop "+ the Pulse-free a11y webview e2e gate". Dynamic proof requiring a live Pulse is a local operator gate.'
    sidecar: '2026-10-11 conductors-window-retired: Inherited Defaults CI/CD drops the a11y webview gate.'
    rationale: 'Report Harness/gate surface: the `a11y` job was removed. Counts: CI jobs 3 → 1.'
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: '§Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace)'
    change: 'Remove the `scripts/webview2-cause-probe.ps1`, `scripts/a11y-limited-token-launch.ps1` and `scripts/a11y-token-witness.ps1` rows. The `.github/workflows/` comment reads "fmt + build + test + clippy + the static gates (over committed data · repository hygiene)", with no a11y webview e2e gate.'
    sidecar: '2026-10-11 conductors-window-retired: directory tree drops the three a11y/WebView2 scripts and the a11y gate.'
    rationale: 'Report Files: those three scripts were deleted. Harness/gate: the `a11y` job was removed.'
    dependent-of: D-platform-claim
```

## security-plan

verdict: 35 proposal(s) · entities after decode 0 · parse ok · stripped: 0 preamble line(s), 0 trailing line(s)

```yaml
proposals:
  # Scope note: the chunk adds no new input surface, dependency or sidecar change. Every proposal below retires
  # doc text that asserts a surface, dependency or spawn form which the report's Changes bullets say was REMOVED.
  # Each one sits in its detector's own section. Not touched: the `1239` at security-plan :181. It is an advisory
  # count, not the test count (the report's Decisions flag it as a sweep hazard). Rule (a) (the MCP sidecar spawn)
  # is also not touched: the report does not change the sidecar spawn, the preflight or the data-dir, so it still
  # matches §Security Anti-Patterns.
  - detector: D-security-deps
    severity: escalate
    section: §Dependency Security → Accepted exceptions
    change: "As of 2026-10-11 the set is 0 `[advisories] ignore` entries + 8 `[licenses] allow` entries. The 16 ignores (RUSTSEC-2024-0411..0420 gtk-rs GTK3, RUSTSEC-2025-0075/0080/0081/0098/0100 rust-unic, RUSTSEC-2024-0370 proc-macro-error) left with the Tauri tree. `MPL-2.0` was retired with 0 holders. `Apache-2.0 WITH LLVM-exception` is kept, with 16 holders (`blake3`, `rustix`/`linux-raw-sys`, the `wasi`/`wit-bindgen`/`wasm-*` tree). Replace the closing \"the 16 + 9 counts above hold unchanged at 2026-09-29\" with the new 0 + 8 stamp."
    sidecar: "2026-10-11 conductors-window-retired: deny.toml reconciliation stamp 16 ignore + 9 allow → 0 + 8 (the Tauri-tree ignores and MPL-2.0 retired, LLVM-exception kept with its current holders)."
    rationale: "Report Changes → Schema/config: `[advisories] ignore` 16 → 0 and `[licenses] allow` 9 → 8. Counts/qualifiers moved names `.andromeda/security-plan.md:185` as the stating site."
    basis: "deny.toml:14-18 · deny.toml:38-41; conductor-0.4.0/chunks/2026-10-11-conductors-window-retired/evidence/supply-chain.md"
  - detector: D-security-deps
    severity: escalate
    section: §Dependency Security → Update policy
    change: "Retire the `tauri` ≥ 2.10.3 floor (CVE-2026-42184) and its host-tool bundler twin (`tauri-cli`/`tauri-bundler` measured 2.11.4). `tauri` and `tauri-build` left the workspace deps 2026-10-11 and no window-toolkit package remains in `Cargo.lock`, so the floor has no subject. Keep the `libsqlite3-sys` currency line and the rmcp note."
    sidecar: "2026-10-11: the tauri ≥ 2.10.3 floor and the tauri-cli bundler host-tool floor retire, because tauri left the tree (lock 562 → 268, 0 window-toolkit packages)."
    rationale: "Report Changes → Dependencies: \"Workspace deps `tauri = \\\"2.10.3\\\"` and `tauri-build = \\\"2\\\"` left `Cargo.toml`\". The gate's window-toolkit `grep -c` on Cargo.lock read 0. Dev-tool versions: `tauri-cli` leaves with the jobs and arms that used it. The doc still says \"the tree resolves 2.11.3\"."
    basis: "conductor-0.4.0/chunks/2026-10-11-conductors-window-retired/evidence/lock-delta.md"
  - detector: D-security-deps
    severity: escalate
    section: §Security Anti-Patterns → Universal (pinned-dependency bump ban)
    change: "The standing research floor is the toolchain ≥ 1.94.1 alone (met). The `tauri` ≥ 2.10.3 floor retired 2026-10-11 with the crate: no tauri package is in `Cargo.lock`. Drop \"the tree resolves `tauri` 2.11.3\"."
    sidecar: "2026-10-11: the Universal bump ban drops the retired tauri floor; the toolchain floor stands."
    rationale: "This restates the tauri floor from Update policy, falsified by the same Dependencies bullet (tauri left Cargo.toml; 0 window-toolkit packages in the lock)."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Threat Model Summary → Infrastructure (Hosting)
    change: "Local-only: `cargo build --release` produces the `conductor-cli` binary (the sole binary crate), run beside Pulse via `scripts/agent-run.sh`. Remove the optional Tauri 2 GUI bundle / host `tauri-cli` / nsis+msi sentence, which retired 2026-10-11 with `conductor-tauri`."
    sidecar: "2026-10-11: Hosting no longer names the Tauri GUI bundle or the tauri-cli host tool."
    rationale: "Report Changes → Crates/modules: `conductor-tauri` (the GUI bin) removed; one binary crate remains (`conductor-cli`). Dev-tool versions: `tauri-cli` leaves."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: Preamble → Stack adaptability note
    change: "Conductor is a headless CLI Rust workspace (one binary, `conductor-cli`; eight members), not a web/API service. Drop \"Tauri 2 GUI +\"."
    sidecar: "2026-10-11: the stack note no longer names a Tauri 2 GUI."
    rationale: "Report Changes → Crates/modules: workspace 9 → 8 members, binary crates 2 → 1 (`conductor-cli`)."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Dependency Security → Critical CVE response SLA
    change: "Immediate bump for any advisory that touches the `cargo build` extraction path (tar-rs class), the live build boundary. Drop \"or the Tauri IPC surface\": no IPC surface remains."
    sidecar: "2026-10-11: the CVE SLA drops the retired Tauri IPC surface."
    rationale: "Report Changes → Symbols/APIs: the window's eight IPC handlers and the live-counter `Channel` were removed with the crate (`crates/conductor-tauri/src/main.rs:22-31@321dc8f`)."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Dependency Security → third dependency class
    change: "RETIRED 2026-10-11. The CI-time-fetched-binary class holds 0 members: the `a11y` job, its version-pinned msedgedriver fetch and its pre-execution Authenticode gate were removed whole. The audited surface is again the Rust crate graph alone (`Cargo.lock`, scanned by `cargo-audit` + `cargo-deny`). The npm tree is gone too. Keep the 2026-09-08 → 09-17 driver-pin / ≥152-floor / windows-2025 runner history only as dated record, never as a live claim."
    sidecar: "2026-10-11: the third dependency class goes 1 member → 0 (msedgedriver + Authenticode gate leave with the a11y job); the registered CI network exception goes 1 → 0."
    rationale: "Report Counts/qualifiers moved: \"The third dependency class (the CI-fetched msedgedriver) 1 member → 0; the registered network exception … 1 → 0\". Harness/gate surface: the `a11y` job was removed whole, including `Pin msedgedriver to the image's WebView2 runtime (gate)`. That step also retires the paragraph's windows-2025/windows-2022 runner verdicts, which the report lists under Verdicts RETIRED."
    basis: ".github/workflows/ci.yml lines 243-783 at 321dc8f (report Harness/gate surface)"
  - detector: D-security-deps
    severity: escalate
    section: §Dependency Security → Supply chain integrity (SKIP note)
    change: "Remove the claim that consumption-side signature verification is an active control. The `a11y` job's Authenticode check on the fetched msedgedriver retired 2026-10-11 with the job, and no fetched-binary control remains. The SKIP plus the optional `cargo-auditable` note stand."
    sidecar: "2026-10-11: the supply-chain SKIP note no longer cites the retired Authenticode control as active."
    rationale: "This is the same third-class claim. The report's Harness/gate surface removes the msedgedriver pin + Authenticode gate."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Threat Model Summary → Infrastructure (Networking)
    change: "Local only: loopback OTLP/gRPC to `127.0.0.1:4317` plus a stdio MCP session. No non-loopback egress remains in the project; the CI-only msedgedriver HTTPS fetch retired 2026-10-11 with the `a11y` job (count 1 → 0)."
    sidecar: "2026-10-11: the project's one non-loopback egress (the a11y job's msedgedriver fetch) retired; count 1 → 0."
    rationale: "Report Counts: the registered network exception (the `a11y` job's HTTPS fetch from `msedgedriver.microsoft.com`) goes 1 → 0."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Threat Model Summary → Infrastructure (CI/CD)
    change: "GitHub Actions runs one job, `rust`: the formatting, secret-scan and workflow env-context gates, build, nextest, clippy, the `cargo audit` + `cargo deny` supply-chain steps and coverage. The `a11y` job (its webview leg, fetched signed binary and Authenticode gate) and the `frontend` job (with `npm audit`) were removed 2026-10-11. Dynamic end-to-end proof stays an operator/local gate."
    sidecar: "2026-10-11: the CI/CD line now reads rust-only (3 jobs → 1); the a11y/frontend jobs and the fetched-binary surface are gone."
    rationale: "Report Counts: CI jobs 3 → 1 (`rust` alone). Harness/gate surface: the `frontend` and `a11y` jobs were removed whole; the kept list holds both supply-chain steps."
    basis: ".github/workflows/ci.yml:86"
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Secret Management → Secret scanning in CI
    change: "Replace \"would have been a second third-dependency-class member and a second CI egress\" with \"would re-open the (now empty) third dependency class and add a CI non-loopback egress where none remains\"."
    sidecar: "2026-10-11: the secret-scan rationale re-worded for an empty third dependency class and zero CI egress."
    rationale: "Report Counts: the third dependency class goes 1 → 0 and the network exception 1 → 0, so \"second\" is false."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Secret Management → Secret-scan gate shape
    change: "Replace \"would add a second member to the third dependency class … and a second CI non-loopback egress\" with \"would re-open the empty third dependency class … and add a CI non-loopback egress where none remains\"."
    sidecar: "2026-10-11: the secret-scan gate-shape rejection rationale re-counted (third class 0 members, 0 CI egress)."
    rationale: "Same third-class / egress counts as above (report Counts: 1 → 0 each)."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Dependency Security → Frontend (npm) supply chain
    change: "RETIRED 2026-10-11. No npm tree remains: `crates/conductor-tauri/ui/package-lock.json` left whole and `git ls-files '*package.json'` → 0. The `npm audit --omit=dev` gate left with the `frontend` job. Cargo is the sole audited ecosystem."
    sidecar: "2026-10-11: the npm frontend supply-chain paragraph retires; the npm tree and the npm audit gate left with the window crate and the frontend job."
    rationale: "Report Changes → Dependencies: \"The npm tree … left whole; no npm tree remains\". Harness/gate surface: the `frontend` job, including the npm audit gate, was removed whole."
  - detector: D-security-deps
    severity: escalate
    section: §Security Anti-Patterns → Code Patterns (Tauri `shell-open` plugin ban)
    change: "Retire the bullet: no Tauri GUI or plugin remains in the tree (2026-10-11). If a GUI is ever re-introduced, re-derive the ban then."
    sidecar: "2026-10-11: the Tauri shell-open ban retires with the GUI crate."
    rationale: "This ban assumes a shipped Tauri GUI. Report Crates/modules: `conductor-tauri` removed. Dependencies: tauri left; 0 window-toolkit packages."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Security Anti-Patterns → Code Patterns (remote-origin iframe ban)
    change: "Retire the bullet: no bundled Tauri webview or IPC surface remains (2026-10-11)."
    sidecar: "2026-10-11: the Tauri webview iframe ban retires with the GUI crate."
    rationale: "Report Symbols/APIs: the IPC handlers and `Channel` were removed. Files: the `ui/` webview was deleted whole."
    dependent-of: D-security-deps
  - detector: D-security-deps
    severity: escalate
    section: §Security Anti-Patterns → Code Patterns (Tauri capabilities-file ban)
    change: "Retire the bullet: no Tauri commands ship (the eight handlers and `capabilities/default.json` were deleted 2026-10-11)."
    sidecar: "2026-10-11: the Tauri capabilities ban retires; no Tauri command ships."
    rationale: "Report Files: `capabilities/default.json` deleted. Symbols/APIs: the eight IPC handlers were removed (`crates/conductor-tauri/src/main.rs:22-31@321dc8f`)."
    dependent-of: D-security-deps
  - detector: D-security-subprocess
    severity: escalate
    section: §Security Anti-Patterns → Code Patterns (spawn rule (b))
    change: "Rule (b) governs ONE live form: the fifth, the teardown `conductor cleanup <run_id>` on the routine `scripts/agent-run.{sh,ps1}` path, behind `valid_run_id` / `Test-RunId`. Forms 1-4 and 8 (the driver stack, `wdio.conf.ts` + `send-keys.ps1`) and form 7 (`a11y-limited-token-launch.ps1`) retired 2026-10-11 with their files. Form 6 retired 2026-09-17; ordinals are not reused. The driver-stack and CI-workflow loci hold no governed form, and the network-arriving-PROGRAM property has no carrier. Rule (a) is unchanged."
    sidecar: "2026-10-11: spawn rule (b) goes from eight governed forms to one live form (form 5); forms 1-4, 7, 8 retire with wdio.conf.ts, send-keys.ps1 and a11y-limited-token-launch.ps1."
    rationale: "Report Counts/qualifiers moved: \"forms 1-4 and 8 … and 7 … left with their files; form 5 … is the only one left (form 6 was retired 2026-09-17). Stated in security-plan (`.andromeda/security-plan.md:370`, 'eight governed forms')\". The sidecar spawn (rule a) is not touched by the report and still matches."
    basis: ".andromeda/security-plan.md:370"
  - detector: D-security-subprocess
    severity: escalate
    section: §Security Anti-Patterns → Universal (inbound-listener ban)
    change: "Drop the carve-out for the dev-only driver stack's loopback ports 4444/4445: they lost every binder 2026-10-11 (`wdio.conf.ts` deleted). The ban now has no dev-harness exception; `:4317` (port-occupier) is untouched."
    sidecar: "2026-10-11: the 4444/4445 driver-stack carve-out leaves the inbound-listener ban."
    rationale: "Report Symbols/APIs → Ports: \"the dev-only loopback driver ports `4444` / `4445` lose every binder (they lived in `wdio.conf.ts`). `:4317` … is untouched.\""
    dependent-of: D-security-subprocess
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → `#[tauri::command]` IPC arguments row
    change: "Retire the row: the `conductor-tauri` IPC boundary was removed 2026-10-11 (its eight handlers, the `run_id`-class `run_report`/`run_envelope` arguments and the `resolve_under` + `sanitize_error` edge in `commands.rs`)."
    sidecar: "2026-10-11: the #[tauri::command] IPC row retires with the window crate."
    rationale: "Report Coverage of new surfaces: \"`conductor-tauri` IPC + webview (removed) → validation n/a\". Symbols/APIs: eight handlers removed. Files: `src/commands.rs` deleted."
    basis: "crates/conductor-tauri/src/main.rs:22-31@321dc8f"
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Attack surface (Tauri IPC vector)
    change: "Retire the Tauri IPC vector: no webview→backend IPC remains (2026-10-11)."
    sidecar: "2026-10-11: the Tauri IPC attack-surface vector retires."
    rationale: "Same removal: report Symbols/APIs (IPC handlers, live-counter `Channel` removed)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → CLI arguments / stdin row
    change: "Replace \"neither the canonicalize duty above nor the Tauri row's `resolve_under` duty attaches to it\" with \"the canonicalize duty above does not attach to it\": the Tauri row is retired."
    sidecar: "2026-10-11: the CLI-args row drops its pointer to the retired Tauri row."
    rationale: "The row cites the `#[tauri::command]` row's duty, which the report retires (Coverage of new surfaces: IPC removed)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Error Handling → External responses / Error format
    change: "The external surface is `conductor-cli` stderr alone, and `thiserror` enums collapse to `anyhow` only at the `conductor-cli` boundary. Drop the `#[tauri::command]` return values / bundled webview / GUI surface."
    sidecar: "2026-10-11: Error Handling's external surface narrows to conductor-cli stderr."
    rationale: "Report Symbols/APIs: the IPC handlers were removed; one binary crate remains (`conductor-cli`)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Bootstrap phases → error-sanitization-wire
    change: "Collapse to `anyhow` only at the `conductor-cli` edge; drop the `#[tauri::command]` edge."
    sidecar: "2026-10-11: error-sanitization-wire names the conductor-cli edge only."
    rationale: "Same IPC-edge removal (report Symbols/APIs)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Security Anti-Patterns → Logging (stderr / IPC-return disclosure ban)
    change: "NEVER expose stack traces, absolute file paths or internal struct/field names via `conductor-cli` stderr. Drop \"or `#[tauri::command]` returns\"."
    sidecar: "2026-10-11: the disclosure ban drops the retired #[tauri::command] return surface."
    rationale: "Same IPC-edge removal (report Symbols/APIs)."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → `CONDUCTOR_MSEDGEDRIVER` row
    change: "Retire the row: the handle lost every reader 2026-10-11. Its reader `wdio.conf.ts` and its CI producer (the `a11y` job's `EDGEWEBDRIVER` derivation) were removed."
    sidecar: "2026-10-11: the CONDUCTOR_MSEDGEDRIVER row retires (0 readers)."
    rationale: "Report Symbols/APIs → env vars losing every reader: `CONDUCTOR_MSEDGEDRIVER`, `EDGEWEBDRIVER`. The gate's `git grep` printed nothing."
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → `CONDUCTOR_NVDA` row
    change: "Retire the row: the handle lost every reader 2026-10-11 (`wdio.conf.ts` `nvdaExe()` deleted)."
    sidecar: "2026-10-11: the CONDUCTOR_NVDA row retires (0 readers)."
    rationale: "Report Symbols/APIs → env vars losing every reader: `CONDUCTOR_NVDA`."
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → `CONDUCTOR_A11Y_STRICT` row
    change: "Retire the row: the handle lost every reader 2026-10-11 (`wdio.conf.ts` and both shells' `assert_a11y_verdict` / `Assert-A11yVerdict` removed)."
    sidecar: "2026-10-11: the CONDUCTOR_A11Y_STRICT row retires (0 readers)."
    rationale: "Report Symbols/APIs: `CONDUCTOR_A11Y_STRICT` lost every reader. Harness/gate: the verdict-check functions left `agent-run.{sh,ps1}`."
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → `CONDUCTOR_E2E_SEED_DIR` row
    change: "The handle has NO setter since 2026-10-11 (`wdio.conf.ts` deleted). Its one reader, `crates/conductor-run/tests/envelope_fixture.rs`, runs as an unset no-op, so no seed write occurs. The residual \"discipline lives at the wdio caller\" has no caller left and is re-stated as: the test still applies no guard of its own; that is sound only while nothing sets the handle and no shipped binary reads it."
    sidecar: "2026-10-11: CONDUCTOR_E2E_SEED_DIR loses its wdio setter; the test reader stays as an unset no-op and the residual is re-stated."
    rationale: "Report Symbols/APIs: \"`CONDUCTOR_E2E_SEED_DIR` keeps its `conductor-run` test reader as an unset no-op (its setter `wdio.conf.ts` left)\"."
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Attack surface (CLI input vector, `CONDUCTOR_*` entry point)
    change: "List only `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST` and `CONDUCTOR_SEED`, plus `CONDUCTOR_E2E_SEED_DIR` as a setter-less test-binary handle. Drop the wdio-read `CONDUCTOR_MSEDGEDRIVER` · `CONDUCTOR_NVDA`, the flag `CONDUCTOR_A11Y_STRICT` and the `--e2e` / `a11y:driven` / `sr*` legs."
    sidecar: "2026-10-11: the CLI-input vector drops the three reader-less dev-harness handles and the retired driver legs."
    rationale: "Report Symbols/APIs → env vars losing every reader; Harness/gate: `--e2e` and `a11y:driven` were removed from both shells."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Security Anti-Patterns → Input (env path-handle canonicalize ban)
    change: "Drop the `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` wdio-edge class, which now has 0 readers. Re-state `CONDUCTOR_E2E_SEED_DIR` as a setter-less test-binary handle (an unset no-op), without \"the wdio caller that sets it\". The per-READER duty and the four `CONDUCTOR_RUNS_DIR` test readers stand."
    sidecar: "2026-10-11: the Input canonicalize ban drops the reader-less wdio handle class and re-states the E2E seed residual."
    rationale: "Report Symbols/APIs: these env vars lost every reader; `CONDUCTOR_E2E_SEED_DIR`'s setter `wdio.conf.ts` left."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Bootstrap phases → input-validation-library-install
    change: "Drop \"`CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` never reach Rust … validated at the wdio harness edge\". Only `CONDUCTOR_E2E_SEED_DIR` (test-binary-only, now setter-less) stays outside this phase's scope."
    sidecar: "2026-10-11: input-validation-library-install no longer scopes out the reader-less wdio handles."
    rationale: "Report Symbols/APIs: `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` lost every reader."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Secret Management → Storage (Development)
    change: "Drop `CONDUCTOR_A11Y_STRICT`, `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` from the non-secret `CONDUCTOR_*` list (0 readers since 2026-10-11). Re-word `CONDUCTOR_E2E_SEED_DIR` as a setter-less test-binary handle."
    sidecar: "2026-10-11: the dev env-handle list drops the three reader-less handles."
    rationale: "Report Symbols/APIs → env vars losing every reader."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → Screen-reader speech-log ingest row
    change: "Retire the row: the NVDA speech-log ingest (`parse-nvda-log.ts`, `writeNvdaPass`, the `sr*` legs) was deleted 2026-10-11, and no committed record is produced any more."
    sidecar: "2026-10-11: the speech-log ingest row retires with the screen-reader suites."
    rationale: "Report Files: `ui/test/a11y/screen-reader/*` deleted, including `parse-nvda-log.ts`. Harness/gate: the `sr*` legs left with the crate."
  - detector: D-security-input
    severity: escalate
    section: §Security Anti-Patterns → Logging (artifact host-path leak ban)
    change: "Keep the ban over run-report artifacts and the per-chunk `evidence/` tree. Drop \"incl. `nvda-pass.json`, whose `heard` text is transcribed third-party speech …\": no new speech record is produced."
    sidecar: "2026-10-11: the artifact-leak ban drops the retired nvda-pass.json speech-ingest clause."
    rationale: "Same removal as the speech-log ingest row (report Files: the screen-reader tree deleted)."
    dependent-of: D-security-input
  - detector: D-platform-claim
    severity: warning
    section: §Threat Model Summary → Attack surface (port bind `:4317` vector, trust boundary)
    change: "Keep \"the sole case where a SHIPPED Conductor binary opens a port …\" and drop the whole dev-only webview driver-legs sentence. That includes its stated runner verdict \"(host-gated on Windows + an NVDA build named by `CONDUCTOR_NVDA`, skipping at exit 0 elsewhere; never a CI stage …)\" and the 4444/4445 listeners. The surface was removed, not re-measured, so no platform literal replaces it."
    sidecar: "2026-10-11: the retired driver-leg Windows host-gate verdict and the 4444/4445 harness listeners leave the :4317 vector."
    rationale: "Report Harness/gate surface → Verdicts RETIRED: \"the desktop-webview a11y leg's runner and driver verdicts — the `--e2e` / `a11y:driven` / `sr*` legs, the tauri-driver + msedgedriver stack … retire with the surface itself (removed, not re-measured elsewhere)\". Stating sentence quoted: \"the operator-local `sr` / `sr-empty` / `sr-error` screen-reader suites (host-gated on Windows + an NVDA build named by `CONDUCTOR_NVDA`, skipping at exit 0 elsewhere; never a CI stage, no `agent-run` verb)\". This one edit also covers the Ports bullet (4444/4445 lose every binder) at this site."
```

## design-system

verdict: 0 proposal(s) · entities after decode 0 · parse ok · stripped: 0 preamble line(s), 0 trailing line(s)

## layout-templates

verdict: 5 proposal(s) · entities after decode 0 · parse ok · stripped: 9 preamble line(s), 0 trailing line(s)
strict YAML parse failed (raw twin `.raw-fanout-layout-templates.md`); list below is the field-by-field parse, re-serialised
stripped text, verbatim:
> # Drift check for layout-templates against the 2026-10-11-conductors-window-retired report.
> # D-layout-surface: no hit. The report's "Coverage of new surfaces" bullet says "none new — the chunk only removes surfaces".
> # D-layout-derived-count: no hit. None of the old values in "Counts / qualifiers moved" (9 members, 2 bins, 562 packages,
> #   16/9 deny entries, 3 CI jobs, 8 continue-on-error, 11 if:, eight governed forms, 1 msedgedriver class, 65 binaries, 1239 tests)
> #   appears in any caption, sample, wireframe or label. "9 measured" in the HOLD wireframe is a sample matrix tally, not one of these values.
> # Outside these three detectors: the plan's Expected amendments for layout-templates also list the whole §Surface: desktop-webview
> #   section, "all three surfaces" (twice in the §cli Primary screens `conductor coverage` bullet, one naming the removed `unbacked_auto`
> #   command), and the Tauri >= 2.10.3 mentions (desktop-webview Tooling context and IA notes). A removed surface fits none of these
> #   detectors, so these are not proposed here. The orchestrator should route them through the expected-amendments path.

```yaml
proposals:
- detector: D-platform-claim
  severity: warning
  section: '§Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}`'
  change: 'Remove the sentence "`--e2e` is the cross-surface stage: it builds the release Tauri bundle and drives the desktop-webview console through the tauri-driver webview leg, gated on a Conductor-owned driver handle whose absence skips the stage at exit 0 … and a non-zero exit when `CONDUCTOR_A11Y_STRICT` is affirmatively declared (2026-09-07)." Say instead that the `--e2e` webview stage left with the window (2026-10-11), that the token now hits the existing unknown-token `usage:` exit 2 in both shells, and that no driver-gated stage remains.'
  sidecar: 2026-10-11 conductors-window-retired — retired the `--e2e` tauri-driver webview stage's driver-gated skip/strict verdict; `--e2e` is now an unknown token (usage exit 2).
  rationale: Report Harness/gate surface says the `--e2e` arm (the tauri-driver `wdio run` leg and its release `--features tauri/custom-protocol` build) is gone from both shells. "`--e2e` is gone from both headers and both usage lines, so it falls to the existing unknown-token `usage:` exit 2." The "Verdicts RETIRED" bullet retires the desktop-webview leg's runner and driver verdicts, which go away with the surface and are not re-measured anywhere else. The quoted sentence is that driver verdict in the doc's own voice.
  basis: scripts/agent-run.sh; scripts/agent-run.ps1
- detector: D-platform-claim
  severity: warning
  section: '§Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}`'
  change: Remove the sentence "It also asserts the runner's PRINTED verdict rather than its exit code — the `Spec Files:` failed count, the per-spec skip markers against the expected-skip set, and the `[webview2 … windows]` driven-session banner — because the exit code alone cannot tell a full pass from a total skip." The printed-verdict check left with the `--e2e` arm.
  sidecar: 2026-10-11 conductors-window-retired — removed the `--e2e` printed-verdict check (`[webview2 … windows]` banner, skip-marker set), which left with the arm.
  rationale: Report Harness/gate surface says the `--e2e` arm "and its printed-verdict check (`assert_a11y_verdict` / `Assert-A11yVerdict`, `A11Y_EXPECTED_SKIPS`, `CONDUCTOR_A11Y_STRICT`) left". The sentence asserts that a WebView2-on-Windows driven session is what the leg verifies, which is part of the retired driver verdict.
  basis: scripts/agent-run.sh; scripts/agent-run.ps1
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: '§Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}`'
  change: In "the 5-command `boot`/`run`/`status`/`cleanup`/`logs` harness; `run` takes `--unit`/`--integration`/`--e2e`/`--live` stage flags", drop `--e2e` so the stage-flag set reads `--unit`/`--integration`/`--live`, with the bundled gate still the default.
  sidecar: 2026-10-11 conductors-window-retired — took `--e2e` out of the `run` stage-flag set.
  rationale: This restates the same retired claim, that `--e2e` is a `run` stage. Report Harness/gate surface says `--e2e` is gone from both headers and both usage lines, and that "The five verbs, `--live` / `--live real-model` … are unchanged."
  basis: scripts/agent-run.sh; scripts/agent-run.ps1
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: '§Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}`'
  change: 'Remove "That refusal is distinct from a HOST-TOOL handle''s absence (`CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA`), which SKIPS its own leg at exit 0 with a fetch recipe — an incomplete live-Pulse environment is a refusal, a missing host tool is a skip." Say instead that the `--live` suite has no host-tool leg: its driven a11y step left with the window, and the suite now runs H → B1 → B2 → quiet window → A, so the preconditions refusal (exit 1) is the only way it stops before a leg fires.'
  sidecar: 2026-10-11 conductors-window-retired — retired the `--live` suite's host-tool (msedgedriver/NVDA) skip verdict; the driven a11y step left with the window.
  rationale: Report Harness/gate surface says "the `--live` suite's driven a11y step (`npm run a11y:driven`) and its wdio / `taskkill` stop-form lines left … The live suite now composes H → B1 → B2 → quiet window → A". Symbols/APIs says `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` lost every reader. "Verdicts RETIRED" names the `a11y:driven` / `sr*` legs and the msedgedriver stack. The quoted sentence states that driver verdict (the leg needs a host driver and skips when it is missing).
  basis: scripts/agent-run.sh; scripts/agent-run.ps1
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: '§Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}`'
  change: 'Remove "That host-tool skip is the LAX arm: under an affirmative `CONDUCTOR_A11Y_STRICT` (the CI posture, 2026-09-07) the same unresolved handle exits non-zero instead, so the two-class contrast holds for the default arm and CI adds a third outcome rather than replacing either." No strict/lax host-tool arm remains, and the CI posture no longer reads `CONDUCTOR_A11Y_STRICT`.'
  sidecar: 2026-10-11 conductors-window-retired — retired the `CONDUCTOR_A11Y_STRICT` strict/lax host-tool arm and its CI posture.
  rationale: Report Symbols/APIs says `CONDUCTOR_A11Y_STRICT` lost every reader, and the gate's `git grep … CONDUCTOR_(MSEDGEDRIVER|NVDA|A11Y_STRICT)` printed nothing. Harness/gate surface says the `a11y` CI job was removed whole (CI jobs 3 → 1). The sentence restates the retired host-tool verdict and adds a CI outcome that no longer exists.
  basis: .github/workflows/ci.yml; scripts/agent-run.sh; scripts/agent-run.ps1
  dependent-of: D-platform-claim
```

## test-plan

verdict: 36 proposal(s) · entities after decode 0 · parse ok · stripped: 0 preamble line(s), 0 trailing line(s)

```yaml
proposals:
  - detector: D-tests-derived-count
    severity: warning
    section: "§9 CI Integration → Matrix builds (OS bullet)"
    change: "OS: the SHIPPED CI arrangement is the job set `.github/workflows/ci.yml` declares, named by job and never by line coordinate. Today that set is `rust` alone: the `frontend` and `a11y` jobs retired with the window on 2026-10-11, and the gate's job-count awk read 1. No OS matrix exists. Drop the a11y-job / webview-E2E shipping narrative and its WebView2 / msedgedriver / integrity capability findings. They retired with the surface and were not re-measured, so they stay history in the chunk records. Keep a matrix only as a TARGET arrangement where a route entry still owns one, and keep the nextest `job_name` sentence."
    sidecar: "§9 Matrix builds: CI job set 3 → 1 (`rust` alone); the a11y/frontend jobs and the webview driver-capability narrative retired with the window."
    rationale: "Report Counts / qualifiers moved: 'CI jobs 3 → 1 (`rust` alone; the gate's job-count awk read 1). Stated in test-plan §9 (`.andromeda/test-plan.md:398`, measured as three jobs … `rust` and `frontend` on `windows-latest`, `a11y` on `windows-2022`)'. The same bullet also states a retired capability verdict: 'Windows does on the DEV HOST … and, since 2026-09-17, on the GitHub-hosted `windows-2022` image the `a11y` job now runs, where a coherent 131.0.2903.86 driver/runtime pair at High integrity drives a session'. Harness / gate surface 'Verdicts RETIRED' retires it with the surface. It is folded into this one edit so the bullet is not edited twice. Name the set and do not substitute a fresh count literal."
    basis: ".andromeda/test-plan.md:398"
  - detector: D-tests-derived-count
    severity: warning
    section: "§1 Test Scope Summary → Test tier justification"
    change: "Replace '9 workspace crates' with the set: 'the workspace's crate-per-seam members (the set `Cargo.toml [workspace] members` declares)'. Keep the rest of the justification. The Minimal-at-upper-boundary verdict does not turn on the literal."
    sidecar: "§1 tier justification: '9 workspace crates' → the `[workspace] members` set (members now 8)."
    rationale: "Report Counts / qualifiers moved: 'Workspace members 9 → 8; binary crates 2 → 1 (`Cargo.toml@321dc8f` vs HEAD)'. Crates / modules: 'The workspace holds eight members'. The tier justification bakes the old count. §1 is marked VERBATIM from test-scope.md, so the same literal there is the upstream twin of this edit."
    basis: "Cargo.toml [workspace] members"
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Surfaces under test (desktop-webview entry, plus the closing 'No web-spa…' parenthetical)"
    change: "Delete the desktop-webview surface entry: the Tauri bundled webview, its tauri-driver driver and its Windows WebView2 measured-set verdict. Re-word the closing parenthetical so it says no desktop surface remains. The surfaces under test are cli and ipc-internal (MCP read-back)."
    sidecar: "§1 Surfaces: the desktop-webview surface and its tauri-driver / WebView2 platform verdict retired with the window."
    rationale: "Stating sentence: 'Driver: tauri-driver (headful on the Windows WebView2 host — the MEASURED SET — the dev host, and CI's `a11y` runner since it moved to `windows-2022` on 2026-09-17, where a coherent 131.0.2903.86 driver/runtime pair at High integrity drives a session …; Linux + `xvfb` remains an unrun target arrangement and macOS is driver-less)'. Report Harness / gate surface, 'Verdicts RETIRED': 'the desktop-webview a11y leg's runner and driver verdicts — the `--e2e` / `a11y:driven` / `sr*` legs, the tauri-driver + msedgedriver stack, the WebView2 `windows-2022` image pin — retire with the surface itself (removed, not re-measured elsewhere)'. Files: `crates/conductor-tauri/` deleted whole, including the React 19 webview."
    basis: "report §Changes → Harness / gate surface (Verdicts RETIRED); crates/conductor-tauri/ deleted"
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope (`conductor-tauri` entity)"
    change: "Delete the `conductor-tauri` entity. The crate left the workspace and its 27 tests left with it."
    sidecar: "§1 Coverage scope: `conductor-tauri` entity removed with the crate."
    rationale: "Stating sentence: 'The Tauri command/Channel IPC layer is agent-drivable headless via tauri-driver'. Report Crates / modules: 'removed `conductor-tauri` (the GUI bin)'. Coverage of new surfaces: 'its 27 tests left with it'. The tauri-driver verdict retires with the surface."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Surfaces under test (ipc-internal Tauri 2 `#[tauri::command]` entry)"
    change: "Delete the ipc-internal (Tauri commands + Channel) surface entry, including its 'full webview leg via tauri-driver (desktop-webview)' driver."
    sidecar: "§1 Surfaces: the Tauri IPC surface removed with the eight IPC handlers."
    rationale: "Report Symbols / APIs: 'Removed with the crate: the window's eight IPC handlers … the live-counter `Channel`' (`crates/conductor-tauri/src/main.rs:22-31@321dc8f`). The entry's tauri-driver webview leg is a retired driver verdict."
    basis: "crates/conductor-tauri/src/main.rs:22-31@321dc8f"
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Untestable zones (screen-reader BROWSE-MODE rows)"
    change: "Delete the screen-reader browse-mode zone. The `sr*` legs, `send-keys.ps1`, `nvda.ini` and `nvda-pass-spec.md` left with the crate, so no NVDA / WebDriver-injected-key leg remains for the zone to bound. Its 2026-09-30 regrade stays history in that chunk's evidence."
    sidecar: "§1 Untestable zones: the SR browse-mode zone retired with the `sr*` legs."
    rationale: "Stating sentence: 'NVDA reaches browse mode … only through its browse-mode commands, which its OS keyboard hook sees and WebDriver-injected keys never deliver'. Report Files: deleted `test/a11y/screen-reader/{activate-window,send-keys}.ps1` and `nvda-config/nvda.ini`. Verdicts RETIRED names the `sr*` legs."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Untestable zones (applied Windows creation flag on the sidecar spawn)"
    change: "Keep the zone. Re-word its evidence clause: the APPLIED flag 'was evidenced by the operator-local `sr` leg's absence of the console-pane observable; that leg retired 2026-10-11 with the window, so no standing leg evidences the application now'. It stays unasserted and is never a manual smoke step. The flag VALUE stays unit-covered."
    sidecar: "§1 Untestable zones: the creation-flag zone's evidencing `sr` leg retired; the zone stands with no standing evidence leg."
    rationale: "The zone names the `sr` leg as its only evidence of application. Report Harness / gate surface: 'the `--e2e` / `a11y:driven` / `sr*` legs … retire with the surface itself (removed, not re-measured elsewhere)'."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: escalate
    section: "§1 Test Scope Summary → Critical paths (Path 7, Both-surface parity)"
    change: "Rescope Path 7 to the PROVEN half: the in-process `conductor_run::{preflight, drive_run}` composition against the headless `conductor run` gives the same envelope for the same deterministic-posture scenario+seed (surfaces: cli + in-process core + persistent-storage). The DEFERRED control-panel-LAUNCHED half 'which the tauri-driver leg (§5) owns' retires with the window: no control panel remains. Record it against the Creator Brief Must-Work as a narrowing by the 2026-10-11 retirement, and stop calling it OWED to a tauri-driver leg."
    sidecar: "§1 Critical Path 7: control-panel-launched half retired with the window; the in-process ↔ headless parity half stands."
    rationale: "Stating sentence: 'DEFERRED: that the run be LAUNCHED via the Tauri start command itself, which the tauri-driver leg (§5) owns … The brief's control-panel requirement therefore stands OWED, not met.' Report Symbols / APIs: the window was `drive_run`'s 'sole production caller'. They stay, for `Panel-shaped types retired` (`working-route.md:13`). Escalated because this moves a Creator Brief Must-Work from OWED to retired, which needs the founder's word on the disposition."
    basis: "conductor-run/src/drive.rs:53"
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Coverage triggers (cross-surface-coordination)"
    change: "Keep the in-process-core ↔ headless parity assertion. Replace 'the composition behind the Tauri `start_run` command' with 'the in-process composition'. Drop 'Asserting a control-panel-LAUNCHED run is the deferred half, owned by the tauri-driver leg (§5)': that half retired with the window."
    sidecar: "§1 cross-surface trigger: the tauri-driver-owned launched half retired."
    rationale: "It repeats Path 7's retired owner claim 'owned by the tauri-driver leg (§5)'. Report Verdicts RETIRED: the tauri-driver stack."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§2 Test Strategy → Agent-runnable invariants (no human-in-loop bullet)"
    change: "Delete the tauri-driver parenthetical, from 'tauri-driver runs `wdio run` NON-INTERACTIVELY only — headful on the Windows WebView2 host set …' through '…not Linux-bound'. The bullet keeps: no Percy / Chromatic, no `cargo insta review` interactive accept UI (insta CI/assert mode only), exit-code-readable."
    sidecar: "§2 invariants: the tauri-driver / WebView2 host-set platform clause retired."
    rationale: "Stating sentence: 'tauri-driver runs `wdio run` NON-INTERACTIVELY only — headful on the Windows WebView2 host set (the dev host AND the hosted `windows-2022` `a11y` runner at a coherent 131.0.2903.86 driver/runtime pair at High integrity …)'. Report Verdicts RETIRED. The wdio harness `wdio.conf.ts` was deleted."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§2 Test Strategy → Agent-runnable invariants (Deterministic bullet)"
    change: "Narrow the loopback set to the gRPC/MCP stub. Drop 'the harness-lifetime WebDriver pair ANY webview arm spawns … tauri-driver `:4444` + the native driver `:4445`'. Drop the operator-local `a11y:driven` arm and the three `sr*` suites (NVDA, readiness windows, clock calibration) from the real-wall-clock exceptions. Drop 'never the deterministic tier for the frontend paths they prove'. Keep the `--live` suite, `--live real-model`, `span_landing_live` and `p075_round_live` exceptions."
    sidecar: "§2 Deterministic: the :4444/:4445 WebDriver pair and the a11y:driven / sr* wall-clock exceptions retired."
    rationale: "Report Symbols / APIs, Ports: 'the dev-only loopback driver ports `4444` / `4445` lose every binder (they lived in `wdio.conf.ts`)'. Harness / gate surface: the `--live` suite's 'driven a11y step (`npm run a11y:driven`) … left'. Verdicts RETIRED names the `a11y:driven` / `sr*` legs."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests cover (`conductor-tauri/ui (frontend SPA)` bullet)"
    change: "Delete the `conductor-tauri/ui` bullet: the build gate, the ESM-loadability proving legs (`--e2e`, `a11y:driven`, `a11y:sr*`, `a11y:ownership`) and the Epoch-9 tauri-driver deferral. No frontend tree remains."
    sidecar: "§4 What unit tests cover: the frontend SPA bullet and its npm proving legs retired."
    rationale: "It states the proving legs '`--e2e` proves the routine suite, `npm run a11y:driven` proves the `driven` suite …' and 'frontend coverage (webview E2E via tauri-driver) is deferred to Epoch 9'. Report Files: `ui/` deleted, including `check-claim-ownership.ts`. Dependencies: 'no npm tree remains (`git ls-files '*package.json'` → 0)'. Harness: `--e2e` gone."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§5 Integration Test Strategy → Cross-module patterns ('Deferred to the GUI test-harness leg' bullet)"
    change: "Re-word: the GUI test-harness leg retired with the window on 2026-10-11, so the `Channel`-frame-sequence assertion and the webview RENDER leg have no surface. The in-process-core ↔ CLI parity half stands as landed (`crates/conductor-cli/tests/cross_surface_parity.rs`). The mock-runtime command-dispatch tier left with the eight commands. Drop the 'on whichever host supplies a WebDriver — Linux+xvfb or the Windows WebView2 host' clause."
    sidecar: "§5 GUI-leg deferral retired; the WebDriver-host verdict removed; the parity half stands."
    rationale: "Stating sentence: 'Deferred to the GUI test-harness leg (`tauri-driver`, on whichever host supplies a WebDriver — Linux+xvfb or the Windows WebView2 host)'. Report Verdicts RETIRED. Symbols / APIs: the eight IPC handlers and the `Channel` were removed."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§6 E2E Test Strategy (section note and Drivers per surface → desktop-webview row)"
    change: "Change the section note to '_[Included — the cli surface is agent-drivable.]_'. Delete the desktop-webview row: tauri-driver via `wdio run`, the Windows host SET and pair tables, the hosted-image findings, the three suite families (routine `--e2e`, driven `a11y:driven`, `sr*`) and their firing / stop forms."
    sidecar: "§6: desktop-webview driver row and its WebView2 / msedgedriver platform verdicts retired."
    rationale: "Stating sentences: 'HEADFUL WebView2 on the Windows host SET — the dev host with an operator-supplied `msedgedriver`; and CI's `a11y` runner, on `windows-2022` …' and '**no WKWebView WebDriver on macOS** — macOS alone is driver-less'. The note itself states 'cli + desktop-webview surfaces are agent-drivable'. Report Verdicts RETIRED. Env vars `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, `EDGEWEBDRIVER` lose every reader."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Both-surface parity (control panel AND headless)"
    change: "Retitle the scenario to in-process ↔ headless parity, with surfaces cli + in-process core. In step 2, drop the webview render leg ('tauri-driver `wdio run` — headful on the Windows WebView2 host set … Linux + `xvfb` is target-state') and the `get_ipc_response()` aside. In the verification signal, drop 'the tauri-driver leg (§5) owns it': the control-panel-launched half retired with the window."
    sidecar: "§6 parity scenario: webview render leg and its host-set verdict retired; the in-process ↔ headless signal stands."
    rationale: "Stating sentence: 'Webview render leg: tauri-driver `wdio run` — headful on the Windows WebView2 host set (the dev host AND the hosted `windows-2022` `a11y` runner …; Linux + `xvfb` is target-state)'. Report Verdicts RETIRED."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§7 Test Data & Fixtures (committed fixture family and fixture-meaning bullets)"
    change: "Drop `crates/conductor-tauri/tests/fixtures/scenarios/` and its loader test `commands::tests::the_committed_scenarios_fixture_stays_loadable` from the committed fixture family. Re-word the `conductor-run` envelope/lamps fixtures' purpose: remove 'to give the webview arm a subject' and the 'consuming e2e' / 'green webview assertion' rationale, and say what still consumes them. `CONDUCTOR_E2E_SEED_DIR` keeps only its test reader, as an unset no-op."
    sidecar: "§7 fixture family: the window's fixture tree removed; the webview-arm consumer clause retired."
    rationale: "Report Files: `crates/conductor-tauri/tests/fixtures/scenarios/*` deleted. Symbols / APIs: '`CONDUCTOR_E2E_SEED_DIR` keeps its `conductor-run` test reader as an unset no-op (its setter `wdio.conf.ts` left)'. The webview arm the fixtures fed is a retired leg."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§9 CI Integration → Pipeline structure (E2E (webview) row)"
    change: "Delete the E2E (webview) stage row: `run --e2e`, the `a11y` job on `windows-2022`, the msedgedriver pin gate, `a11y-token-witness.ps1`, the `continue-on-error` diagnostic step SET and the Linux + `xvfb` / dev-host verdict."
    sidecar: "§9 pipeline: the E2E (webview) stage and its a11y-job runner verdict retired."
    rationale: "Stating sentence: 'Linux + `xvfb` is unmeasured target-state, while the same leg also runs headfully on the Windows dev host (`CONDUCTOR_MSEDGEDRIVER`-gated, skip at exit 0 when unset)'. Report Harness / gate surface: the `a11y` job was removed whole (pin + Authenticode gate, routine arm gate, six WebView2 diagnostic steps, the violation-JSON gate). Deleted: `scripts/a11y-token-witness.ps1`, `scripts/webview2-cause-probe.ps1`. Counts: `continue-on-error` 8 → 0."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§11 Test Anti-Patterns (non-interactive-runner ban)"
    change: "Keep the ban on Selenium IDE / WebdriverIO codegen-recorder / `cargo insta review` interactive accept UI. Drop the '`wdio run` — headless where a display must be faked, headful on a real desktop host' clause. The non-interactive runner named is insta CI/assert mode with the `CI` env var."
    sidecar: "§11: the wdio headless/headful platform clause retired."
    rationale: "Stating clause: '`wdio run` — headless where a display must be faked, headful on a real desktop host'. Report: the wdio harness `wdio.conf.ts` was deleted. Verdicts RETIRED."
    dependent-of: D-platform-claim
  - detector: D-platform-claim
    severity: warning
    section: "§11 Test Anti-Patterns (real-network ban)"
    change: "Loopback only (gRPC/MCP stubs). Drop 'plus the WebDriver pair `:4444`/`:4445` spawned by ANY webview arm — routine, driven, or an `sr*` screen-reader suite'. The live Pulse leg is local-gate-only, never a CI network call. Drop 'including the `a11y:driven` arm and the `sr` subject'."
    sidecar: "§11 network ban: the :4444/:4445 WebDriver exception and a11y:driven / sr mentions retired."
    rationale: "Report Ports: '`4444` / `4445` lose every binder'. `:4317` untouched. The `a11y:driven` / `sr*` legs are retired per Verdicts RETIRED."
    dependent-of: D-platform-claim
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation (CI stage selectors item)"
    change: "Stage selectors are `--unit` / `--integration` plus the operator-gated `--live`. `--e2e` is gone from both shells' headers and usage lines and falls to the existing unknown-token `usage:` exit 2. Bare `--live` composes the precondition probe → legs H → B1 → B2 → quiet window → A (read from `live_leg_order`), with no driven a11y arm, `ensure_frontend` or `tauri/custom-protocol` release build. Delete the `--e2e ⇒ …` parenthetical: the printed-verdict assertion, the `runs/a11y/<run_id>.jsonl` violation record, the wdio fixture seeding, the tauri-driver cwd and mocha ceiling, and the xvfb/Windows clause. `--integration` no longer names the `tauri::test` mock-runtime. The five verbs, `valid_run_id` / `Test-RunId`, the `status` disk read and the envelope/JSONL shapes are unchanged, so §3 ↔ obs-plan §3 still agree once obs-plan drops its a11y record."
    sidecar: "§3 5-command: `--e2e` selector, its verdict check and a11y violation record retired; `--live` no longer runs the driven a11y arm."
    rationale: "Report Harness / gate surface, `scripts/agent-run.{sh,ps1}`: 'the `--e2e` arm … and its printed-verdict check (`assert_a11y_verdict` / `Assert-A11yVerdict`, `A11Y_EXPECTED_SKIPS`, `CONDUCTOR_A11Y_STRICT`) left; the `--live` suite's driven a11y step … left; `ensure_frontend` / `Invoke-EnsureFrontend` … left … `--e2e` is gone from both headers and both usage lines, so it falls to the existing unknown-token `usage:` exit 2 … The live suite now composes H → B1 → B2 → quiet window → A'. The `runs/a11y` record is a separate artifact shape paired with obs-plan §3/§9 ('§3 … the a11y record', '§9 the a11y artifact row' in the obs-plan expected amendments). Both sides must move together, or the change is one-sided."
    basis: "scripts/agent-run.sh; scripts/agent-run.ps1"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Pipeline structure (Integration row)"
    change: "Integration: `agent-run.{sh,ps1} run --integration` → the hand-rolled MCP stub + in-memory/`TempDir` `runs.db`. The `tauri::test` mock-runtime left with the window."
    sidecar: "§9 Integration stage: tauri::test mock-runtime removed from the `--integration` composition."
    rationale: "It repeats the `--integration` composition that the primary amends. Report Dependencies: workspace deps `tauri` / `tauri-build` left `Cargo.toml`. Crates / modules: `conductor-tauri` removed."
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Live-Pulse scenarios"
    change: "The composed `run --live` stage drives the feature-gated targets only (legs H → B1 → B2 → quiet window → A); it no longer drives the `a11y:driven` npm suite. Drop 'and never `a11y:driven`' from the real-model selector clause. Drop 'an NPM-SCRIPT WDIO SUITE (`npm run a11y:driven`)' from the gate forms, and 'and the driven a11y arm' from the unaffected list."
    sidecar: "§9 Live-Pulse: the a11y:driven suite left the `--live` composition and the operator-gate forms."
    rationale: "Stating sentence: 'its composed `run --live` stage (2026-09-06), which drives the feature-gated targets and the `a11y:driven` npm suite in ONE invocation'. Report Harness: the `--live` suite's driven a11y step (`npm run a11y:driven`) left. 'The live suite now composes H → B1 → B2 → quiet window → A.'"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§11 Test Anti-Patterns (live-Pulse-as-CI-gate ban)"
    change: "NEVER run the live-Pulse scenarios as a CI gate: operator/local only, via `workflow_dispatch`, `scripts/agent-run.sh` (including its composed `run --live` stage) or a cargo-feature-gated test invoked directly, per §9. Drop 'or an npm-script wdio suite such as `a11y:driven`'."
    sidecar: "§11 CI ban: the npm-script wdio suite form retired."
    rationale: "It repeats the §9 gate-form set the dependent above amends. Report Harness: `npm run a11y:driven` left. No npm tree remains."
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Test Scope Summary → Coverage triggers (security Vector 1 handle classes)"
    change: "Vector 1 per-reader classes, named by kind and not by ordinal. The harness-only host dev-tool class (`CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`) and the `CONDUCTOR_A11Y_STRICT` flag class lost every reader on 2026-10-11 and leave the trigger, along with their `--e2e` / `sr*` skip-arm and strict/lax e2e-tier coverage. `CONDUCTOR_E2E_SEED_DIR` keeps only its `conductor-run` test reader as an unset no-op (its setter `wdio.conf.ts` left), and its mandated coverage stays the seeder test's no-op path. The Rust-read path handles and the test-binary class are unchanged."
    sidecar: "§1 Vector 1: window-only handle classes (MSEDGEDRIVER, NVDA, A11Y_STRICT) retired; E2E_SEED_DIR reduced to an unset no-op reader."
    rationale: "The trigger's mandated coverage for these classes is the removed harness arms ('the `--e2e` skip arm in both shells', 'both shells' `--e2e` verdict assertions'). Report Symbols / APIs, env vars losing every reader: `CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, `CONDUCTOR_A11Y_STRICT`, `EDGEWEBDRIVER`, `WEBVIEW2_*`. '`CONDUCTOR_E2E_SEED_DIR` keeps its `conductor-run` test reader as an unset no-op'."
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Log format (Self-obs stream is a distinct artifact)"
    change: "The self-obs stream is cli only: stderr / `logs/agent-latest.jsonl`. Drop 'Tauri backend: `logs/conductor-tauri.jsonl`'. The base fields, the two line variants and the separation from the per-run emission journal are unchanged."
    sidecar: "§3 Log format: the Tauri backend self-obs sink `logs/conductor-tauri.jsonl` retired with `tauri_log_path()`."
    rationale: "Report Symbols / APIs: removed 'the self-obs sink helper `tauri_log_path()` (`logs/conductor-tauri.jsonl`)'. Schema / config: '.gitignore … the self-obs comment no longer names `conductor-tauri.jsonl`'. The paired obs-plan rows (key files `log-file-location`, `logging-stack`) are in the report's obs-plan expected amendments. The report also says 'the window log sink's row stays until `:13`'. The orchestrator should apply this together with obs-plan §3's sink rows, or hold both: a one-sided edit is the drift this detector forbids."
    basis: "report §Changes → Symbols / APIs (tauri_log_path removed)"
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test pyramid (Integration and E2E rows)"
    change: "Integration covers cross-module + IPC (the hand-rolled JSON-RPC stub over stdio) + the OTLP-egress loopback gRPC stub + DB (rusqlite `open_in_memory`), across the boundaries OTLP egress, MCP read-back and runs.db. No `tauri::test` mock runtime and no Tauri IPC. E2E covers the critical paths via the cli surface (assert_cmd). No desktop-webview / tauri-driver leg."
    sidecar: "§2 pyramid: tauri::test and tauri-driver left the framework set; integration/E2E name the cli + MCP-stub tiers only."
    rationale: "Report Outcome: the chunk's runner set is `cargo nextest` / `cargo test --workspace --lib --bins --tests` / `cargo llvm-cov nextest` (64 binaries · 1212 passed). Dependencies: `tauri` and `tauri-build` left the workspace. Dev-tool versions: '`tauri-driver` / `tauri-cli` leave with the jobs and arms that used them'. The pyramid still names two frameworks the workspace can no longer build or run, so the doc's framework set no longer matches what the chunk runs."
    basis: "report §Outcome → Gates (cargo test / nextest / llvm-cov)"
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Frontend unit runner"
    change: "Delete the Frontend unit runner paragraph. No webview remains, and no npm tree remains for a JS/TS runner to cover."
    sidecar: "§4: the Frontend unit runner non-adoption paragraph retired with the webview."
    rationale: "Report Dependencies: 'no npm tree remains (`git ls-files '*package.json'` → 0)'. Files: `ui/` deleted whole."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Tool-version policy"
    change: "The external-CLI tool roster is cargo-nextest, cargo-llvm-cov and cargo-mutants. Drop `tauri-cli` and its 2026-09-10 bundler sentence. The crate dev-deps sentence is unchanged."
    sidecar: "§4 Tool-version policy: tauri-cli left the roster."
    rationale: "Report Dev-tool versions: '`tauri-driver` / `tauri-cli` leave with the jobs and arms that used them; no reading of either was taken'."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → What unit tests do NOT cover"
    change: "Integration boundaries are MCP read-back stub stdio and `runs.db` cross-process (Section 5). E2E flows are cli subprocess critical paths (Section 6). Drop 'Tauri command/Channel' and 'desktop-webview'."
    sidecar: "§4: Tauri command/Channel and desktop-webview removed from the higher-level coverage list."
    rationale: "Report Crates / modules: `conductor-tauri` removed. Symbols: the IPC handlers and `Channel` were removed."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy (section note and Driver(s))"
    change: "Note: '_[Included — integration boundaries present: MCP read-back stdio, runs.db, cross-surface parity.]_'. In Driver(s), drop `tauri::test::mock_builder()` + `mock_context(noop_assets())` + `get_ipc_response()` (Tauri commands, `test` feature, `tauri` ≥ 2.10.3)."
    sidecar: "§5: the Tauri mock-runtime driver and the Tauri IPC boundary retired."
    rationale: "Report Dependencies: `tauri = \"2.10.3\"` left `Cargo.toml`. Crates / modules: `conductor-tauri` removed."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy → Boundary types covered (Module ↔ IPC (Tauri) row)"
    change: "Delete the Module ↔ IPC (Tauri) row (mock-runtime command invoke, the tauri 2.10.3 `test` feature)."
    sidecar: "§5 boundary table: the Tauri IPC row removed."
    rationale: "Report Symbols / APIs: the eight IPC handlers and the live-counter `Channel` were removed. Dependencies: `tauri` removed."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy → Cross-module patterns (Tauri 2 `#[tauri::command]` + `Channel` bullet)"
    change: "Delete the Tauri 2 command + Channel bullet: the mock-runtime command-shape assertion and the deny-by-default capabilities-file static assertion. Both subjects left with the crate."
    sidecar: "§5: the Tauri command/Channel/capabilities pattern retired."
    rationale: "Report Files: `capabilities/default.json` and `src/{main,commands,pause}.rs` deleted with the crate."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy → Cross-module patterns (Cross-surface parity bullet)"
    change: "Keep the in-process ↔ CLI parity leg (`crates/conductor-cli/tests/cross_surface_parity.rs`). Replace 'the same composition the Tauri `start_run` command's background thread runs' with 'the in-process `conductor_run::{preflight, drive_run}` composition'. Drop the closing sentence on the `tauri::test` mock runtime staying real for `conductor-tauri`'s `#[test]` fns."
    sidecar: "§5 parity bullet: the Tauri start_run / mock-runtime references retired; the leg stands."
    rationale: "Report Symbols / APIs: `drive_run` (`conductor-run/src/drive.rs:53`) keeps test callers and its `pub use` re-export only; the window was its sole production caller. The mock runtime left with `tauri`."
    basis: "conductor-run/src/drive.rs:53"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§6 E2E Test Strategy → Drivers per surface (ipc-internal (Tauri commands/Channel) row)"
    change: "Delete the ipc-internal (Tauri commands/Channel) row (the tauri 2.10.3 `test` mock runtime, `get_ipc_response()`)."
    sidecar: "§6 drivers: the Tauri mock-runtime row removed."
    rationale: "Report Dependencies: `tauri` removed. Symbols: the IPC handlers were removed."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Mocking libraries (Tauri command runtime row)"
    change: "Delete the Tauri command runtime row (tauri 2.10.3 `test` feature, `mock_builder()` / `mock_context` / `get_ipc_response()`)."
    sidecar: "§8 mocking libraries: the Tauri command runtime row removed."
    rationale: "Report Dependencies: 'Workspace deps `tauri = \"2.10.3\"` and `tauri-build = \"2\"` left `Cargo.toml`'. No window-toolkit package remains in `Cargo.lock`."
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Test data bootstrap (Per-test isolation)"
    change: "Replace 'so the CLI and the Tauri mock-runtime write to the same file `runs.db`' with 'so the CLI subprocess and the in-process `conductor_run` arm write to the same file `runs.db`'. The rest of the item is unchanged."
    sidecar: "§3 Test data bootstrap: Tauri mock-runtime writer replaced by the in-process conductor_run arm."
    rationale: "Report Dependencies: `tauri` removed, so no mock-runtime writer remains. The cross-process parity leg's second writer is the in-process `drive_run` composition (Symbols / APIs, `conductor-run/src/drive.rs:53`)."
    basis: "conductor-run/src/drive.rs:53"
    dependent-of: D-tests-framework
```

## obs-plan

verdict: 5 proposal(s) · entities after decode 0 · parse ok · stripped: 0 preamble line(s), 0 trailing line(s)
strict YAML parse failed (raw twin `.raw-fanout-obs-plan.md`); list below is the field-by-field parse, re-serialised

```yaml
proposals:
- detector: D-obs-ci-gates
  severity: warning
  section: §9 CI Integration — Telemetry artifact handling
  change: Remove the `A11y violation record (runs/a11y/<run_id>.jsonl)` row. The `a11y` CI job, its a11y violation-JSON conformance gate and its `Upload a11y violation record` step left on 2026-10-11, so `rust` is the only CI job and no artifact is written under `runs/a11y`.
  sidecar: §9 — the a11y violation-record artifact row is retired along with the `a11y` job and its violation-JSON conformance gate (2026-10-11-conductors-window-retired).
  rationale: The report's Harness / gate surface bullet lists the `a11y` job as removed whole, including "the **a11y violation-JSON conformance gate**, `Upload a11y violation record`". Its Counts bullet records CI jobs going from 3 to 1 ("`rust` alone"). §9 still names the gate's artifact, its upload and its in-job `journal_conformance` check as live, so the site is orphaned. The Expected amendments bullet for obs-plan also lists "§9 the a11y artifact row".
  basis: .github/workflows/ci.yml@321dc8f:243-783
- detector: D-obs-ci-gates
  severity: warning
  section: §3 → Log format JSON schema
  change: The envelope's extension point was exercised from 2026-09-07 until 2026-10-11 by the a11y CI gate's violation record at `runs/a11y/<run_id>.jsonl` (the 13/15-key superset, as measured at the 2026-09-07 a11y-ci-gate chunk). That gate and its record left with the `a11y` job on 2026-10-11, so no artifact exercises the extension point now and `journal_conformance` serves the scenario record alone.
  sidecar: §3 Log format JSON schema — the a11y-gate violation record is moved to history, since the gate left with the `a11y` job (2026-10-11-conductors-window-retired).
  rationale: 'This is the same retired claim as the §9 row: that the a11y violation-JSON conformance gate exists and exercises the schema. The key file states it in the present tense ("as of 2026-09-07 it IS exercised — by the a11y CI gate''s violation record … which is what lets one gate serve both shapes"). The report''s Harness / gate surface bullet removes that gate along with the `a11y` job.'
  basis: .github/workflows/ci.yml@321dc8f:243-783
  dependent-of: D-obs-ci-gates
- detector: D-obs-ci-gates
  severity: warning
  section: §10 SLO Invariants & Telemetry Budgets — Build / deploy failure conditions
  change: In the `cargo clippy … -D warnings` failure-condition line, delete the closing sentence "The `--e2e` arm runs no clippy line, so a lint never fails the `a11y` job". The `--e2e` arm and the `a11y` job both left on 2026-10-11, and a clippy red now fails the `rust` job's dogfood step, which is the only CI job.
  sidecar: §10 — the orphaned `--e2e` / `a11y`-job clippy clause is dropped, since both left with the window (2026-10-11-conductors-window-retired).
  rationale: The report's Harness / gate surface bullet says the `--e2e` arm left `scripts/agent-run.{sh,ps1}` and the `a11y` job was removed whole. Its Counts bullet records CI jobs going from 3 to 1. The §10 failure-condition line still describes both as live, so the site is orphaned. The Expected amendments bullet for obs-plan names "§10 the `--e2e` clippy sentence".
  basis: .github/workflows/ci.yml@321dc8f:243-783
- detector: D-platform-claim
  severity: warning
  section: §11 Obs Anti-Patterns — Project-specific bans
  change: Replace the stating sentence "the applied flag itself is not assertable at the unit tier, so its effect is evidenced by the operator-local SR leg (test-plan §1 untestable zones)" with "the applied flag itself is not assertable at the unit tier; its effect was evidenced by the 2026-09-04 NVDA SR pass, and that leg retired with the window on 2026-10-11, so no live leg evidences it now — the `#[cfg(windows)]` unit test pins only the flag value".
  sidecar: §11 Project-specific bans — the operator-local SR-leg evidence verdict is retired; the leg left with the desktop-webview surface (2026-10-11-conductors-window-retired).
  rationale: The report's Harness / gate surface bullet "Verdicts RETIRED" retires the desktop-webview a11y leg's runner and driver verdicts, including "the `--e2e` / `a11y:driven` / `sr*` legs … retire with the surface itself (removed, not re-measured elsewhere)". The quoted sentence states, in the doc's own voice, a runner verdict for the SR leg ("operator-local") and treats that leg as the live evidence mechanism. The surface was removed, not moved to another platform, so the retirement names the removal rather than a substitute platform.
- detector: D-platform-claim
  severity: warning
  section: §11 Obs Anti-Patterns — PII Scrubbing
  change: Replace "The a11y leg's scrub is now a second line of defence rather than the only one" with "The a11y leg's scrub was a second line of defence until that leg retired with the window (2026-10-11); `CREATE_NO_WINDOW` now stands alone on this channel". Keep the 2026-09-02 and 2026-09-04 measurements as history.
  sidecar: §11 PII Scrubbing — the a11y leg's scrub is moved to history as a retired second line of defence (2026-10-11-conductors-window-retired).
  rationale: This restates the same retired claim, that the operator-local a11y/SR leg still runs and still guards the sidecar-pane channel. The report's "Verdicts RETIRED" bullet removes the `sr*` legs with the surface, and its Files bullet deletes the screen-reader suites (`test/a11y/screen-reader/*`, `nvda-config/nvda.ini`).
  dependent-of: D-platform-claim
```

## a11y-plan

verdict: 14 proposal(s) · entities after decode 0 · parse ok · stripped: 0 preamble line(s), 6 trailing line(s)
strict YAML parse failed (raw twin `.raw-fanout-a11y-plan.md`); list below is the field-by-field parse, re-serialised
stripped text, verbatim:
> No-hit notes (not proposals):
> - D-a11y-surface: no hit. The report adds no interactive UI element ("Coverage of new surfaces: none new — the chunk only removes surfaces"; "a11y n/a (no UI remains)").
> - D-a11y-obs-schema: no hit. The report removes the a11y violation-JSON conformance gate and the record upload, but it does not change the a11y violation schema or the obs log schema ("No scrub/redaction shape changed"; "Spec-master edits: none").
> - Sentences left out on purpose under the arrangement-versus-capability rule: the `windows-2022` job-placement sentences (§1, §9, and the ci-integration and tool-pick keys) and the target-state "rides the tests' `wdio run` webview-E2E job on `ubuntu-latest` + `xvfb`" lines (§9 Pipeline integration, ci-integration Command). These describe CI arrangement or a planned target, not capability. They still name the removed job and leg, so the expected-amendments sweep (report: a11y-plan pattern, 55 hits) should pick them up.
> - No line numbers are cited for a11y-plan.md or the key files, because the report states none for them.
> Files read: conductor-0.4.0/chunks/2026-10-11-conductors-window-retired/report.md, .andromeda/a11y-plan.md, .andromeda/runs/2026-10-11T08-28-34-wrap/a11y-plan-contracts.md, and all nine key files under .andromeda/registries/contracts/a11y-plan/

```yaml
proposals:
- detector: D-platform-claim
  severity: warning
  section: §1 A11y Scope Summary → A11y assertion harness specification (CI integration bullet)
  change: Replace the parenthetical capability verdict with a statement that the desktop-webview a11y leg (the routine `--e2e` arm, `a11y:driven`, the `sr*` legs) and its tauri-driver + msedgedriver stack were removed on 2026-10-11 (`2026-10-11-conductors-window-retired`) along with the window crate, so no platform, runner or driver capability verdict stands for them. The previously measured set (Linux under `xvfb`, headful on the Windows WebView2 host, hosted `windows-2022`) is history and was not re-measured anywhere else.
  sidecar: '2026-10-11 conductors-window-retired — §1 harness spec CI-integration bullet: the webview a11y leg''s "measured platform SET" capability verdict is retired along with the removed leg and stack (report Harness / gate surface, Verdicts RETIRED).'
  rationale: 'The report''s Changes → Harness / gate surface bullet "Verdicts RETIRED" says "the desktop-webview a11y leg''s runner and driver verdicts — the `--e2e` / `a11y:driven` / `sr*` legs, the tauri-driver + msedgedriver stack, the WebView2 `windows-2022` image pin — retire with the surface itself (removed, not re-measured elsewhere)". The doc states the verdict in its own voice: "the one webview-automation stack running on the measured platform SET — Linux under `xvfb` AND headfully on the Windows WebView2 host (...), macOS alone staying manual-pass-only; `ubuntu-latest`+`xvfb` is the CI ARRANGEMENT, not a capability bound". This is a capability claim about a leg that no longer exists.'
  basis: git show 321dc8f:.github/workflows/ci.yml lines 243-783 (the removed `frontend` and `a11y` jobs, per the report); scripts/agent-run.sh and scripts/agent-run.ps1 (the `--e2e` arm and the `a11y:driven` step removed)
- detector: D-platform-claim
  severity: warning
  section: §1 A11y Scope Summary → A11y surfaces & assistive tech reach (Screen-reader verification bullet)
  change: Say that the agent-driven NVDA (Windows) leg over the tauri-driver / WebdriverIO session was removed on 2026-10-11 along with the window, so no screen-reader platform verdict stands. NVDA/Windows agent-driven and VoiceOver/Orca declared-only are history.
  sidecar: '2026-10-11 conductors-window-retired — §1 Screen-reader verification: the NVDA/Windows agent-driven and VoiceOver/Orca not-runnable-on-this-host verdicts are retired along with the `sr*` leg.'
  rationale: 'The report''s "Verdicts RETIRED" bullet lists the `sr*` legs. The doc states: "NVDA (Windows) is AGENT-DRIVEN — its own speech log (`-l 12`) over the same tauri-driver / WebdriverIO session ... VoiceOver (macOS) / Orca (Linux) remain manual and declared not-runnable-on-this-host (no pass claimed)".'
  basis: conductor-0.4.0/chunks/2026-10-11-conductors-window-retired/report.md (Files — `test/a11y/screen-reader/{activate-window,send-keys}.ps1` and `nvda-config/nvda.ini` deleted)
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §1 A11y Scope Summary → A11y surfaces & assistive tech reach (Notes bullet)
  change: Drop the sentences "driven via the tests §6 `@crabnebula/tauri-driver` (WebDriver) session" and "VoiceOver macOS has no automated SR test — manual pass required" as standing driver verdicts. Say that the webview a11y tree has no driver since the 2026-10-11 removal.
  sidecar: '2026-10-11 conductors-window-retired — §1 Notes: the tauri-driver drive path and the macOS manual-only SR verdict are retired along with the removed leg.'
  rationale: 'The report''s "Verdicts RETIRED" bullet covers the tauri-driver stack and the `sr*` legs. The doc states: "The Tauri webview a11y tree is driven via the tests §6 `@crabnebula/tauri-driver` (WebDriver) session" and "VoiceOver macOS has no automated SR test — manual pass required."'
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §1 A11y Scope Summary → A11y assertion harness specification (Screen reader test pattern bullet)
  change: Retire "The NVDA (Windows) pass against the webview DOM is produced by the `sr*` leg over the one tauri-driver / WebdriverIO stack (measured 2026-09-02); VoiceOver (macOS) / Orca (Linux) are declared not-runnable-on-this-host" and "NVDA's built-in log is the automated SR driver for the stack". Say instead that the `sr*` leg and its stack left on 2026-10-11, so no SR platform or driver verdict stands.
  sidecar: '2026-10-11 conductors-window-retired — §1 Screen reader test pattern bullet: the `sr*`-leg NVDA/Windows driver verdict is retired along with the leg.'
  rationale: The report's "Verdicts RETIRED" bullet names the `sr*` legs and the tauri-driver + msedgedriver stack. The quoted sentences state where the SR leg runs and what drives it.
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §4 ARIA Patterns & Roles → Per-component pattern catalog (Run-level load-envelope banner row, Tool reach cell)
  change: Retire "Runnability is not platform-bound — it holds across the arm's measured platform set (Linux under `xvfb`, and headfully on the Windows WebView2 host, where this pass ran on WebView2 152.0.4191.53)". Say that the routine arm that ran the rendered-DOM axe check was removed on 2026-10-11, so no runnability verdict stands.
  sidecar: '2026-10-11 conductors-window-retired — §4 load-envelope banner row: the routine arm''s platform-set runnability verdict is retired along with the arm.'
  rationale: 'The report''s "Verdicts RETIRED" bullet says the `--e2e` routine arm "retire[s] with the surface itself". The doc states: "Runnability is not platform-bound — it holds across the arm''s measured platform set (Linux under `xvfb`, and headfully on the Windows WebView2 host ...)".'
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §6 Visual Design Verification → Motion tokens (Verification)
  change: Retire the per-driver verdicts "there is no reduced-motion scope under WebKitGTK/Linux, msedgedriver+WebView2/Windows or macOS alike", "the only preference-exercising path on EVERY platform" and "On WebKitGTK (Linux Tauri) where media-feature emulation may be unavailable, fall back to launching the webview with the preference set at the OS/GTK level". Say that no webview driver remains since the 2026-10-11 removal.
  sidecar: '2026-10-11 conductors-window-retired — §6 Motion tokens: the WebKitGTK / msedgedriver+WebView2 driver verdicts are retired along with the tauri-driver + msedgedriver stack.'
  rationale: The report's "Verdicts RETIRED" bullet lists "the tauri-driver + msedgedriver stack". The quoted sentences state, in the doc's own voice, what each platform's driver can or cannot do.
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §9 CI Integration → Platform
  change: Retire "(one webview-automation stack; that is the CI ARRANGEMENT, not the leg's only platform — the same stack drives headfully on the Windows WebView2 host, measured 2026-09-01)". Say that the stack and the leg left on 2026-10-11, so no capability verdict stands, and that the previously measured set (Linux+`xvfb`, the Windows WebView2 host, hosted `windows-2022`) is history.
  sidecar: '2026-10-11 conductors-window-retired — §9 Platform: the "same stack drives headfully on the Windows WebView2 host" capability verdict is retired along with the stack.'
  rationale: The report's "Verdicts RETIRED" bullet covers the stack and the `windows-2022` pin. The quoted sentence is a capability claim (where the leg CAN run), separate from the CI arrangement, and the leg it describes is removed.
  basis: git show 321dc8f:.github/workflows/ci.yml lines 243-783 (the removed `a11y` job, per the report)
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §9 CI Integration → Per-pipeline-stage a11y artifacts (E2E row)
  change: Retire "Windows + NVDA host only" for the SCREEN-READER leg. Say that the leg and its `CONDUCTOR_NVDA` handle left on 2026-10-11, so no host verdict stands.
  sidecar: '2026-10-11 conductors-window-retired — §9 E2E row: the screen-reader leg''s "Windows + NVDA host only" verdict is retired along with the leg.'
  rationale: 'The report''s "Verdicts RETIRED" bullet names the `sr*` legs, and the Symbols / APIs bullet says `CONDUCTOR_NVDA` loses every reader. The doc states: "the operator-only SCREEN-READER leg (... `CONDUCTOR_NVDA` as the host-tool handle ...; Windows + NVDA host only; tests §6)".'
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §11 A11y Anti-Patterns → CI (second browser-automation stack ban)
  change: Retire "(ONE webview-automation stack — on Linux+xvfb and, measured 2026-09-01, on the Windows WebView2 host). macOS (WKWebView, no WebDriver) stays manual-pass-only; NO assertion in this harness is platform-dependent". Say that the stack left on 2026-10-11, so no platform verdict stands.
  sidecar: '2026-10-11 conductors-window-retired — §11 CI: the one-stack platform-set and macOS manual-only verdicts are retired along with the tauri-driver stack.'
  rationale: The report's "Verdicts RETIRED" bullet covers the tauri-driver + msedgedriver stack. The quoted ban restates the leg's platform capability set in the doc's own voice.
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §3 → A11y testing tool pick
  change: Retire "tauri-driver wraps the platform WebDriver (WebKitWebDriver on Linux / msedgedriver on Windows, the latter measured working 2026-09-01), so the gate runs on the same target tests' webview E2E uses", "tauri-driver is cross-platform (WebKitWebDriver/Linux, msedgedriver/Windows)", "macOS (WKWebView, no WebDriver) stays manual" and "no assertion in this harness is platform-dependent". Say that the tauri-driver + msedgedriver stack left on 2026-10-11, so no driver or platform verdict stands.
  sidecar: '2026-10-11 conductors-window-retired — §3 A11y testing tool pick: the tauri-driver cross-platform driver verdicts are retired along with the stack.'
  rationale: The report's "Verdicts RETIRED" bullet lists "the tauri-driver + msedgedriver stack". The key file states the driver/platform verdicts quoted above.
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §3 → Focus management test harness
  change: Retire "Runs on the measured platform SET alongside tests' webview E2E — Linux under `xvfb` (the CI arrangement) AND headfully on the Windows WebView2 host, where its carrier, the operator-only driven arm, was re-measured 2026-09-02; macOS alone lacks a WebDriver". Say that the driver and both carrying arms left on 2026-10-11.
  sidecar: '2026-10-11 conductors-window-retired — §3 Focus management test harness: the measured-platform-SET verdict is retired along with the driven/routine arms.'
  rationale: The report's "Verdicts RETIRED" bullet names the `--e2e` and `a11y:driven` legs and the driver stack. The key file states the platform set quoted above.
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §3 → Screen reader test pattern
  change: 'Retire "The measured platform set: NVDA/Windows agent-driven with all three subjects attached (2026-09-02, v2-23) ..." and "VoiceOver (macOS) / Orca (Linux) declared not-runnable-on-this-host". Say that the `sr*` leg, `send-keys.ps1` and the tauri-driver + msedgedriver launch left on 2026-10-11, so no SR platform verdict stands. The dated configuration readings can stay as history.'
  sidecar: '2026-10-11 conductors-window-retired — §3 Screen reader test pattern: the NVDA/Windows measured-platform-set verdict is retired along with the `sr*` leg.'
  rationale: 'The report''s "Verdicts RETIRED" bullet names the `sr*` legs, and its Files bullet deletes `test/a11y/screen-reader/{activate-window,send-keys}.ps1`. The key file states: "The measured platform set: NVDA/Windows agent-driven ..." and "VoiceOver (macOS) / Orca (Linux) declared not-runnable-on-this-host".'
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §3 → CI integration
  change: Retire "Linux+`xvfb` stays a measured capability, not the shipped arrangement". Say that the webview leg it describes left on 2026-10-11, so no capability verdict stands.
  sidecar: '2026-10-11 conductors-window-retired — §3 CI integration: the "Linux+xvfb stays a measured capability" verdict is retired along with the leg.'
  rationale: The report's "Verdicts RETIRED" bullet says the leg's runner and driver verdicts "retire with the surface itself". The key file states the capability claim quoted above.
  dependent-of: D-platform-claim
- detector: D-platform-claim
  severity: warning
  section: §3 → Bootstrap phases (derive for route / setup-project)
  change: 'In a11y-ci-gate-wire, retire "The step is not Linux-bound — the measured SET is Linux+`xvfb`, the Windows WebView2 dev host, and the hosted Windows image ...; only macOS lacks a WebDriver: it runs on Linux+`xvfb` and, measured 2026-09-01, headfully on the Windows WebView2 host under `CONDUCTOR_MSEDGEDRIVER`". In screen-reader-test-spec-setup, retire "VoiceOver / Orca declared-only on this host". Say that the step, the leg and `CONDUCTOR_MSEDGEDRIVER` left on 2026-10-11.'
  sidecar: '2026-10-11 conductors-window-retired — §3 Bootstrap phases: the a11y-ci-gate-wire measured-SET verdict and the SR declared-only verdict are retired along with the leg.'
  rationale: The report's "Verdicts RETIRED" bullet covers the leg, the msedgedriver stack and the `windows-2022` pin. The Symbols / APIs bullet says `CONDUCTOR_MSEDGEDRIVER` loses every reader. The key file states the platform verdicts quoted above.
  dependent-of: D-platform-claim
```

total: 152 proposal(s) over 7 docs

## Validate — dispositions

Escalations, resolved WITH the operator in this wrap's question dialog (2026-10-11):
- E1 — the window retirement reverses locked architecture decisions and narrows three security boundaries → **ratified** ("Ratified by the operator on the founder ruling R1 of 2026-10-09 (both projects go to a console form). Every boundary only narrows.").
- E2 — the retirement form → **stub window-only sections, cut window clauses**; design-system, layout-templates and a11y-plan each say near their top, in plain words, that Conductor has no interface; each stub one or two lines.
- E3 — test-plan Critical Path 7's control-panel-launched half → **retired** (narrowed by R1, no longer owed).

Checks: 1 playbook (the plan's P5-approved `Expected amendments (wrap)` entries name each retirement — the recorded direction settles them; `Accurate this-chunk addition` routine; a reversal escalates once → E1) · 2 cross-contradiction: none in opposing directions · 3 intent-consistency: scope record clean (0 lines); deviations carry the founder's / operator's word (inputs#I3) · 4 absence claims: each "lost every reader" / "0 packages" claim rests on the report's gate readings · 5 expected amendments: see the raise list below · 6 disproved claims: the report's one entry is DISPOSED in the report (plan/research immutable, no spec owns it).

### architecture — 57
- architecture#1 D-arch-decisions · §Established Decisions → [Workspace / Core Structure] → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#2 D-arch-decisions · §Established Decisions → [Module Boundaries] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#3 D-arch-decisions · §Design Philosophy · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#4 D-arch-decisions · §Stack and Technologies (Backend framework row) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#5 D-arch-decisions · §Stack and Technologies (Hashing / digest row) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#6 D-arch-decisions · §Stack and Technologies (Push / real-time row) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#7 D-arch-decisions · §Stack and Technologies (Desktop shell row) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#8 D-arch-decisions · §Stack and Technologies (Desktop frontend (webview) row) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#9 D-arch-decisions · §Stack and Technologies (Build / packaging row) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#10 D-arch-decisions · §Established Decisions → [Async Runtime Flavor] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#11 D-arch-decisions · §Established Decisions → [Backend Framework] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#12 D-arch-decisions · §Established Decisions → [Accepted Capability Set] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#13 D-arch-decisions · §Established Decisions → [Error Handling] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#14 D-arch-decisions · §Established Decisions → [Real-time Strategy] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#15 D-arch-decisions · §Established Decisions → [Deployment] · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#16 D-arch-decisions · §Conventions (Internal core↔UI) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#17 D-arch-decisions · §Conventions (Error handling) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#18 D-arch-decisions · §Standard Contracts (Readiness gate) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#19 D-arch-decisions · §Standard Contracts (Live update channel) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#20 D-arch-decisions · §Occupied Resources (Ports: no inbound listener in the shipped binaries) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#21 D-arch-decisions · §Occupied Resources (Tauri commands (internal IPC)) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#22 D-arch-decisions · §Occupied Resources (Route prefixes) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#23 D-arch-decisions · §Occupied Resources (Service / process names) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#24 D-arch-decisions · §Occupied Resources (Service / process names: launched Pulse MCP child) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#25 D-arch-decisions · §Occupied Resources (Crate names (workspace members)) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#26 D-arch-decisions · §Occupied Resources (Frontend asset subtree) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#27 D-arch-decisions · §Occupied Resources (artifact paths: `logs/conductor-tauri.jsonl`) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#28 D-arch-decisions · §Occupied Resources (env vars: `CONDUCTOR_SERVICE_NAME`) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#29 D-arch-decisions · §Cross-cutting Patterns (Development Style) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#30 D-arch-decisions · §Project Intent (Product type) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#31 D-arch-decisions · §Inherited Defaults (Backend framework) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#32 D-arch-decisions · §Inherited Defaults (Interface style) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#33 D-arch-decisions · §Inherited Defaults (Deployment) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#34 D-arch-decisions · §Inherited Defaults (Frontend (optional GUI)) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#35 D-arch-decisions · §Infrastructure Patterns → Build system · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#36 D-arch-decisions · §Infrastructure Patterns → Deployment model · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#37 D-arch-decisions · §Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace) · dependent-of D-arch-decisions → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#38 D-platform-claim · §Established Decisions → [CI/CD] → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#39 D-platform-claim · §Infrastructure Patterns → CI/CD approach · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#40 D-platform-claim · §Infrastructure Patterns → Build system · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#41 D-platform-claim · §Stack and Technologies (CI/CD row) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#42 D-platform-claim · §Occupied Resources (Ports: `https://msedgedriver.microsoft.com` CI-job-scoped egress) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#43 D-platform-claim · §Occupied Resources (Ports: `127.0.0.1:4444` + `127.0.0.1:4445`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#44 D-platform-claim · §Occupied Resources (artifact paths: `runs/a11y/<run_id>.jsonl`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#45 D-platform-claim · §Occupied Resources (env vars: `CONDUCTOR_SCENARIOS_DIR`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#46 D-platform-claim · §Occupied Resources (env vars: `CONDUCTOR_MSEDGEDRIVER`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#47 D-platform-claim · §Occupied Resources (env vars: `CONDUCTOR_NVDA`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#48 D-platform-claim · §Occupied Resources (env vars: `CONDUCTOR_E2E_SEED_DIR`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#49 D-platform-claim · §Occupied Resources (env vars: `CONDUCTOR_A11Y_STRICT`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#50 D-platform-claim · §Occupied Resources (env vars: `EDGEWEBDRIVER`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#51 D-platform-claim · §Occupied Resources (env vars: `WEBVIEW2_USER_DATA_FOLDER` · `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#52 D-platform-claim · §Occupied Resources (env vars: `RUNNER_TEMP`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#53 D-platform-claim · §Occupied Resources (env vars: `TEMP` · `LOCALAPPDATA`) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#54 D-platform-claim · §Cross-cutting Patterns (Trust boundary) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#55 D-platform-claim · §Cross-cutting Patterns (Scope law) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#56 D-platform-claim · §Inherited Defaults (CI/CD) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1
- architecture#57 D-platform-claim · §Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for architecture (recorded direction), re-derived from the report at apply; E1 (reversal of locked decisions + boundary narrowings) resolved: ratified by the operator on the founder ruling R1

### security-plan — 35
- security-plan#1 D-security-deps · §Dependency Security → Accepted exceptions → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#2 D-security-deps · §Dependency Security → Update policy → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#3 D-security-deps · §Security Anti-Patterns → Universal (pinned-dependency bump ban) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#4 D-security-deps · §Threat Model Summary → Infrastructure (Hosting) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#5 D-security-deps · Preamble → Stack adaptability note · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#6 D-security-deps · §Dependency Security → Critical CVE response SLA · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#7 D-security-deps · §Dependency Security → third dependency class → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#8 D-security-deps · §Dependency Security → Supply chain integrity (SKIP note) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#9 D-security-deps · §Threat Model Summary → Infrastructure (Networking) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#10 D-security-deps · §Threat Model Summary → Infrastructure (CI/CD) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#11 D-security-deps · §Secret Management → Secret scanning in CI · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#12 D-security-deps · §Secret Management → Secret-scan gate shape · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#13 D-security-deps · §Dependency Security → Frontend (npm) supply chain → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#14 D-security-deps · §Security Anti-Patterns → Code Patterns (Tauri `shell-open` plugin ban) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#15 D-security-deps · §Security Anti-Patterns → Code Patterns (remote-origin iframe ban) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#16 D-security-deps · §Security Anti-Patterns → Code Patterns (Tauri capabilities-file ban) · dependent-of D-security-deps → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#17 D-security-subprocess · §Security Anti-Patterns → Code Patterns (spawn rule (b)) → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#18 D-security-subprocess · §Security Anti-Patterns → Universal (inbound-listener ban) · dependent-of D-security-subprocess → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#19 D-security-input · §Input Validation → `#[tauri::command]` IPC arguments row → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#20 D-security-input · §Threat Model Summary → Attack surface (Tauri IPC vector) · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#21 D-security-input · §Input Validation → CLI arguments / stdin row · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#22 D-security-input · §Error Handling → External responses / Error format · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#23 D-security-input · §Bootstrap phases → error-sanitization-wire · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#24 D-security-input · §Security Anti-Patterns → Logging (stderr / IPC-return disclosure ban) · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#25 D-security-input · §Input Validation → `CONDUCTOR_MSEDGEDRIVER` row → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#26 D-security-input · §Input Validation → `CONDUCTOR_NVDA` row → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#27 D-security-input · §Input Validation → `CONDUCTOR_A11Y_STRICT` row → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#28 D-security-input · §Input Validation → `CONDUCTOR_E2E_SEED_DIR` row → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#29 D-security-input · §Threat Model Summary → Attack surface (CLI input vector, `CONDUCTOR_*` entry point) · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#30 D-security-input · §Security Anti-Patterns → Input (env path-handle canonicalize ban) · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#31 D-security-input · §Bootstrap phases → input-validation-library-install · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#32 D-security-input · §Secret Management → Storage (Development) · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#33 D-security-input · §Input Validation → Screen-reader speech-log ingest row → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#34 D-security-input · §Security Anti-Patterns → Logging (artifact host-path leak ban) · dependent-of D-security-input → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1
- security-plan#35 D-platform-claim · §Threat Model Summary → Attack surface (port bind `:4317` vector, trust boundary) → apply — the plan's expected-amendments entry for security-plan (recorded direction), re-derived from the report at apply; E1 (boundary narrowings: listener carve-out, CI network exception 1 → 0, rule (b) eight forms → one) resolved: ratified by the operator on the founder ruling R1

### design-system — 5
- design-system#1 D-platform-claim · §Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}` → apply — the plan's expected-amendments entry for design-system (recorded direction), re-derived from the report at apply
- design-system#2 D-platform-claim · §Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}` · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for design-system (recorded direction), re-derived from the report at apply
- design-system#3 D-platform-claim · §Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}` · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for design-system (recorded direction), re-derived from the report at apply
- design-system#4 D-platform-claim · §Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}` · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for design-system (recorded direction), re-derived from the report at apply
- design-system#5 D-platform-claim · §Surface: cli → Primary screens (commands) → `scripts/agent-run.{sh,ps1}` · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for design-system (recorded direction), re-derived from the report at apply

### test-plan — 36
- test-plan#1 D-tests-derived-count · §9 CI Integration → Matrix builds (OS bullet) → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#2 D-tests-derived-count · §1 Test Scope Summary → Test tier justification → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#3 D-platform-claim · §1 Test Scope Summary → Surfaces under test (desktop-webview entry, plus the closing 'No web-spa…' parenthetical) → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#4 D-platform-claim · §1 Test Scope Summary → Coverage scope (`conductor-tauri` entity) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#5 D-platform-claim · §1 Test Scope Summary → Surfaces under test (ipc-internal Tauri 2 `#[tauri::command]` entry) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#6 D-platform-claim · §1 Test Scope Summary → Untestable zones (screen-reader BROWSE-MODE rows) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#7 D-platform-claim · §1 Test Scope Summary → Untestable zones (applied Windows creation flag on the sidecar spawn) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#8 D-platform-claim · §1 Test Scope Summary → Critical paths (Path 7, Both-surface parity) · dependent-of D-platform-claim → apply — E3 resolved: the panel-launched half retired by R1
- test-plan#9 D-platform-claim · §1 Test Scope Summary → Coverage triggers (cross-surface-coordination) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#10 D-platform-claim · §2 Test Strategy → Agent-runnable invariants (no human-in-loop bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#11 D-platform-claim · §2 Test Strategy → Agent-runnable invariants (Deterministic bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#12 D-platform-claim · §4 Unit Test Strategy → What unit tests cover (`conductor-tauri/ui (frontend SPA)` bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#13 D-platform-claim · §5 Integration Test Strategy → Cross-module patterns ('Deferred to the GUI test-harness leg' bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#14 D-platform-claim · §6 E2E Test Strategy (section note and Drivers per surface → desktop-webview row) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#15 D-platform-claim · §6 E2E Test Strategy → Scenario: Both-surface parity (control panel AND headless) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#16 D-platform-claim · §7 Test Data & Fixtures (committed fixture family and fixture-meaning bullets) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#17 D-platform-claim · §9 CI Integration → Pipeline structure (E2E (webview) row) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#18 D-platform-claim · §11 Test Anti-Patterns (non-interactive-runner ban) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#19 D-platform-claim · §11 Test Anti-Patterns (real-network ban) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#20 D-tests-obs-harness · §3 → 5-command implementation (CI stage selectors item) → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#21 D-tests-obs-harness · §9 CI Integration → Pipeline structure (Integration row) · dependent-of D-tests-obs-harness → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#22 D-tests-obs-harness · §9 CI Integration → Live-Pulse scenarios · dependent-of D-tests-obs-harness → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#23 D-tests-obs-harness · §11 Test Anti-Patterns (live-Pulse-as-CI-gate ban) · dependent-of D-tests-obs-harness → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#24 D-tests-obs-harness · §1 Test Scope Summary → Coverage triggers (security Vector 1 handle classes) · dependent-of D-tests-obs-harness → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#25 D-tests-obs-harness · §3 → Log format (Self-obs stream is a distinct artifact) → apply, re-derived — not the proposal's removal: the plan holds the window log sink row until `Panel-shaped types retired` (`working-route.md:13`, whose line names it), so the row stays, qualified "no writer since 2026-10-11", in lockstep with obs-plan §3 (the test-plan §3 ↔ obs-plan §3 bind)
- test-plan#26 D-tests-framework · §2 Test Strategy → Test pyramid (Integration and E2E rows) → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#27 D-tests-framework · §4 Unit Test Strategy → Frontend unit runner · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#28 D-tests-framework · §4 Unit Test Strategy → Tool-version policy · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#29 D-tests-framework · §4 Unit Test Strategy → What unit tests do NOT cover · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#30 D-tests-framework · §5 Integration Test Strategy (section note and Driver(s)) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#31 D-tests-framework · §5 Integration Test Strategy → Boundary types covered (Module ↔ IPC (Tauri) row) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#32 D-tests-framework · §5 Integration Test Strategy → Cross-module patterns (Tauri 2 `#[tauri::command]` + `Channel` bullet) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#33 D-tests-framework · §5 Integration Test Strategy → Cross-module patterns (Cross-surface parity bullet) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#34 D-tests-framework · §6 E2E Test Strategy → Drivers per surface (ipc-internal (Tauri commands/Channel) row) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#35 D-tests-framework · §8 Mocking & Stubbing Discipline → Mocking libraries (Tauri command runtime row) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply
- test-plan#36 D-tests-framework · §3 → Test data bootstrap (Per-test isolation) · dependent-of D-tests-framework → apply — the plan's expected-amendments entry for test-plan (recorded direction), re-derived from the report at apply

### obs-plan — 5
- obs-plan#1 D-obs-ci-gates · §9 CI Integration — Telemetry artifact handling → apply — the plan's expected-amendments entry for obs-plan (recorded direction), re-derived from the report at apply
- obs-plan#2 D-obs-ci-gates · §3 → Log format JSON schema · dependent-of D-obs-ci-gates → apply — the plan's expected-amendments entry for obs-plan (recorded direction), re-derived from the report at apply
- obs-plan#3 D-obs-ci-gates · §10 SLO Invariants & Telemetry Budgets — Build / deploy failure conditions → apply — the plan's expected-amendments entry for obs-plan (recorded direction), re-derived from the report at apply
- obs-plan#4 D-platform-claim · §11 Obs Anti-Patterns — Project-specific bans → apply — the plan's expected-amendments entry for obs-plan (recorded direction), re-derived from the report at apply
- obs-plan#5 D-platform-claim · §11 Obs Anti-Patterns — PII Scrubbing · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for obs-plan (recorded direction), re-derived from the report at apply

### a11y-plan — 14
- a11y-plan#1 D-platform-claim · §1 A11y Scope Summary → A11y assertion harness specification (CI integration bullet) → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#2 D-platform-claim · §1 A11y Scope Summary → A11y surfaces & assistive tech reach (Screen-reader verification bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#3 D-platform-claim · §1 A11y Scope Summary → A11y surfaces & assistive tech reach (Notes bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#4 D-platform-claim · §1 A11y Scope Summary → A11y assertion harness specification (Screen reader test pattern bullet) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#5 D-platform-claim · §4 ARIA Patterns & Roles → Per-component pattern catalog (Run-level load-envelope banner row, Tool reach cell) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#6 D-platform-claim · §6 Visual Design Verification → Motion tokens (Verification) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#7 D-platform-claim · §9 CI Integration → Platform · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#8 D-platform-claim · §9 CI Integration → Per-pipeline-stage a11y artifacts (E2E row) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#9 D-platform-claim · §11 A11y Anti-Patterns → CI (second browser-automation stack ban) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#10 D-platform-claim · §3 → A11y testing tool pick · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#11 D-platform-claim · §3 → Focus management test harness · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#12 D-platform-claim · §3 → Screen reader test pattern · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#13 D-platform-claim · §3 → CI integration · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply
- a11y-plan#14 D-platform-claim · §3 → Bootstrap phases (derive for route / setup-project) · dependent-of D-platform-claim → apply — the plan's expected-amendments entry for a11y-plan (recorded direction), re-derived from the report at apply

Totals: apply 152 (one re-derived as qualified, not removed) · reject 0 · rejected for a source the report does not carry 0.

## Check 5 — expected amendments the orchestrator raised (routine; the report substantiates each; recorded direction = the P5-approved plan)

- architecture: every §-site the entry names was reached by D-arch-decisions / D-platform-claim; nothing raised beyond the cascade's own fixes (`RUST_LOG`'s `EDGEWEBDRIVER` basis).
- security-plan: covered by D-security-* / D-platform-claim; raised the rule (a) SR-leg evidence clause and the preamble stack note's second line.
- design-system: no detector proposed (D-design-* blind to a removed surface) — raised whole: the preamble no-interface line, §Typography · §Spacing · §Depth Strategy · §Border Radius · §Motion · §Iconography · §Surface: desktop-webview stubs, the ANSI 246 `var(--status-residual)` clause, the webview per-surface bans, the font ban, the expression-level clause.
- layout-templates: raised §Surface: desktop-webview (stub + the no-interface line), "all three surfaces" ×2, the qualifier-set webview banner, the cli hero / expression / IA desktop mirrors.
- test-plan: raised the §8 Tauri mock-runtime row, the GUI-shell clause (§3 intro) and the obs-plan-bound log-format qualifier.
- obs-plan: raised §1 surfaces + identities, §3 heartbeat Tauri rows + frontend sinks, §4 span rows, the parity path/scenario, §6 log level, §11 `tauri.command.*`, and the sink "no writer" qualifiers.
- a11y-plan: raised the preamble no-interface line and the stubs of §1–§11 + every §3 key (beyond D-platform-claim's 14 sites).
- `matrix#v4-01` / `matrix#v4-09` notes — ledger-note form, dispositioned at P7.3 (operator: write both).

Escalations resolved at this wrap's question dialogs: E1 (ratify), E2 (form), E3 (Path 7), E4 (three drift-base detector edits — approved), E5 (the two matrix notes — write).
