# arch extract

## Relevance
relevant — the chunk deletes a workspace member, two CI jobs, registered resources (ports, egress, env handles) and Tauri-only supply-chain exceptions, all of which arch registers. The plan still describes the window as present, so its window passages are what the wrap amends (B6). They are not mandates for this chunk.

## Constraints
- The `Cargo.toml` dependency edges are the architecture. Removing `conductor-tauri` removes a member and its edges and adds none. `conductor-run` is described as "shared by both bins", so after the removal `conductor-cli` is its only bin consumer. The standalone per-seam build is a measured property that no CI step enforces: `--lib` for each library crate and `--bins` for `conductor-cli`. Per architecture §Established Decisions [Module Boundaries].
- All engine logic lives in runtime-agnostic library crates, and the headless `conductor-cli` / `scripts/agent-run.sh` path is the release gate. Whether any function the window offers lives only inside `conductor-tauri` and not in core/run, so that it would lose its only entry point (B4), is research's question. Per architecture §Design Philosophy ("Headless-drivable core, thin shells").
- The runtime decision has two arms: `#[tokio::main(flavor = "current_thread")]` on the CLI path, and a hand-built `Builder::new_current_thread()` "owned by the core under Tauri". The CLI arm must stay. Whether the Tauri arm's builder sits in a crate that survives (and becomes dead code under `-D warnings`, the B1 question) is research's question. Per architecture §Established Decisions [Async Runtime Flavor].
- The registered resources that exist only for the webview stack are:
  - ports: loopback `4444`/`4445` (tauri-driver + native WebDriver);
  - egress: the CI-only `https://msedgedriver.microsoft.com` fetch, the single non-loopback egress;
  - env handles: `CONDUCTOR_MSEDGEDRIVER` · `CONDUCTOR_NVDA` · `CONDUCTOR_E2E_SEED_DIR` · `CONDUCTOR_A11Y_STRICT` · `EDGEWEBDRIVER` · `WEBVIEW2_USER_DATA_FOLDER` / `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` · `RUNNER_TEMP` · `TEMP`/`LOCALAPPDATA`.

  The plan registers each one by its readers or writers in `wdio.conf.ts`, the `a11y` job or `envelope_fixture.rs`. `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_RUNS_DIR`, `RUST_LOG` and `GHA_ENV_CONTEXT_PROBE` have non-window readers per the same section. Research measures whether each handle keeps a consumer. Per architecture §Occupied Resources (Ports · Environment variables).
- The Tauri tree accounts for these `deny.toml` exceptions: the gtk-rs Linux-webview advisories, `unic-*` via `urlpattern`, `proc-macro-error`, `MPL-2.0` and `Apache-2.0 WITH LLVM-exception`. `BSD-2-Clause` (for `arrayref`, via `conductor-emit`→`blake3`) is named as the first cargo-side exception from outside that tree. The `ensure_frontend` step in `agent-run.{sh,ps1}` and in the CI `rust` job exists because `tauri::generate_context!` resolves `ui/dist` at compile time. Per architecture §Infrastructure Patterns → Build system.
- The `rust` job keeps all of these, each as its own named step with no `continue-on-error` and no `if:`:
  - the `cargo fmt --all --check` gate;
  - build/nextest/clippy;
  - the coverage-completeness and scenario-assertion audit gates;
  - the secret-scan and workflow env-context gates (with the `GITHUB_ENV context probe` pair).

  The `a11y` job, its pin gate and its `journal_conformance`-over-`runs/a11y` stage are what leave. Per architecture §Infrastructure Patterns → CI/CD approach.
- The trust boundary is loopback-only with no inbound listener in shipped binaries. The `:4317` port-occupier is the sole deliberate bind, and it stays. The dev-only driver-stack listeners and the CI-only third outbound surface are the exceptions this removal retires. Per architecture §Cross-cutting Patterns (Trust boundary · Scope law).

## Patterns to follow
- A static gate's assertion lives in its Rust test, and the CI step is only that test's presence-guarded invocation. Where a gate's roster or allowlist names the window crate, edit the Rust test's data. Never re-list it in shell. Per architecture §Established Decisions [CI/CD].
- The capability manifest's load sites are "whatever takes that parameter, never a count". `conductor-tauri::commands` is one of the named binary-edge sites, so it leaves with the crate and no count in code changes with it. Per architecture §Established Decisions [Accepted Capability Set].
- Harness shells clear their own artifacts non-recursively by pattern (`rm -f …/*.jsonl`, the `cleanup` verb's idiom). The harness ships no recursive delete, and the arm removal in `agent-run.{sh,ps1}` must leave it that way. Per architecture §Occupied Resources (On-disk artifacts, `runs/live-suite/`).
- Measure supply-chain state with both runners over the new lock: `cargo deny check advisories bans licenses sources` and `cargo audit`, each exit 0, each remaining exception adjudicated with its crate and path named inline. Per architecture §Infrastructure Patterns → Build system.

## Anti-patterns to avoid
- Do not use `--all-targets` as the standalone sweep form. It re-unifies dev-dependencies and masks the very feature gap the sweep exists to catch. Per architecture §Established Decisions [Module Boundaries].
- Do not add a new port, egress target or `CONDUCTOR_*` handle, and do not repurpose one. That namespace covers only artifacts and concepts the plan already defines (B5). Per architecture §Occupied Resources (Environment variables).
- Do not drop a `deny.toml` entry because it sits beside the Tauri ones. `BSD-2-Clause`/`arrayref` is named as outside that tree. Per architecture §Infrastructure Patterns → Build system.

## Contract bindings
- arch §Occupied Resources (env handles `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` / `CONDUCTOR_A11Y_STRICT`, ports `4444`/`4445`) ↔ security-plan §Security Anti-Patterns: the harness-spawn rule (b) forms and the wdio-edge handle guards leave with what they spawn. ↔ a11y-plan §3: the routine, driven and `sr*` suite families.
- arch §Occupied Resources (`CONDUCTOR_E2E_SEED_DIR`, `runs/a11y/<run_id>.jsonl`) ↔ tests: the `conductor-run` seeding test `envelope_fixture.rs`, and `journal_conformance` run under `CONDUCTOR_RUNS_DIR=runs/a11y`. The conformance test itself also gates the run journal.
- arch §Occupied Resources (`logs/conductor-tauri.jsonl`, `CONDUCTOR_SERVICE_NAME` default `conductor-tauri`) ↔ obs-plan §3 dual-surface sink table. The window log sink stays per B1.
- arch §Conventions (Internal core↔UI) / §Standard Contracts (Live update channel) ↔ layout-templates desktop surface. The run-event channel payload stays per B1 (`Panel-shaped types retired`).
- arch §Infrastructure Patterns → Build system (`ensure_frontend`) ↔ `.claude/rules/verification-harness.md` (`agent-run run` verb) and test-plan §9/§10 (CI gates).

## Acceptance criteria contributions
- The workspace members are exactly the eight remaining `conductor-*` crates. A `cargo tree` / `Cargo.lock` reading after the removal holds no `tauri*`, `wry`, `tao`, `webview2-com` or gtk-rs package. Each of the seven library crates builds standalone with `--lib` and `conductor-cli` with `--bins`, each exit 0. Per architecture §Established Decisions [Module Boundaries].
- No committed code, script or workflow binds or reaches loopback `4444`/`4445` or `msedgedriver.microsoft.com`. No committed code, script or workflow reads or writes the window-only env handles that research measured as consumer-less. No new port, egress or `CONDUCTOR_*` handle appears. Per architecture §Occupied Resources.
- `.github/workflows/ci.yml` holds no `a11y` or `frontend` job and no frontend-bundle step. The `rust` job's fmt gate and four static gates are each present with no `continue-on-error` and no `if:`. The workflow env-context gate passes with the `a11y` job's `GITHUB_ENV` writer gone. Per architecture §Infrastructure Patterns → CI/CD approach.
- `cargo deny check advisories bans licenses sources` and `cargo audit` both exit 0 over the new lock. Every `deny.toml` entry that remains names a consumer in the post-removal graph. Per architecture §Infrastructure Patterns → Build system.
