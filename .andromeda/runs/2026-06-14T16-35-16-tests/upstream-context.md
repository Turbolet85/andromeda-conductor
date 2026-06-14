# Upstream Context — Conductor (tests Phase 0 distillation)

_Mapper-reduce distillation of architecture.md + security-plan.md + design-system.md + layout-templates.md + input.md. Tests Phases 1-7 read THIS composite instead of the full upstreams. Five sections in fixed order: Architecture → Security Plan → Design System → Layout Templates → Creator Brief._

---

## 1. Architecture Excerpt

### Stack (testable surfaces)
- **Rust 2024 (cargo 1.85, MSRV 1.88.0)** — primary implementation language; serves every seam crate and both binary edges (CLI + Tauri).
- **tokio 1.48.x (`current_thread` flavor)** — deterministic single-threaded scheduler for the timeline engine; serves the seeded phase-scheduling surface (`conductor-timeline`).
- **opentelemetry-proto 0.32.0 (`gen-tonic` + trace/metrics/logs)** — raw hand-built OTLP message structs for byte-level fault control; serves the emission surface (`conductor-emit`).
- **tonic 0.14.6 + tonic-prost 0.14.6 + prost 0.14** — generated `TraceServiceClient`/`LogsServiceClient` over loopback gRPC; serves the OTLP egress transport.
- **rmcp 1.7.0 (`client` feature)** — official Rust MCP SDK doing `serve_client()` over `TokioChildProcess` stdio with version negotiation + typed tool calls; serves the read-back verification surface (`conductor-verify`).
- **rusqlite 0.38.0 + libsqlite3-sys 0.38.0 (`bundled` → SQLite 3.51.1, JSON1)** — synchronous embedded run-metadata index (`runs.db`); serves the storage/report surface (`conductor-report`).
- **Tauri 2 (bundler v2.10.x) + Tauri 2 IPC `Channel`** — optional GUI control-panel artifact + in-app backend→frontend streaming of live counters/target status; serves the desktop shell surface (`conductor-tauri`).
- **serde 1.0.x + garde 0.23.0** — `#[derive(Validate)]` range rules + struct-level cross-field invariants; serves the scenario-config validation surface.
- **thiserror 2.0.18 + anyhow 1.0.102** — typed per-seam error enums in library crates; type-erased `Result` at binary edges; serves the error-handling surface across all seams.

### Workspace / Modules
- **conductor-core** — runtime-agnostic engine library that every other crate depends on (makes "headless-drivable" real).
- **conductor-timeline** — deterministic seeded phase scheduler built on `tokio::time`.
- **conductor-emit** — OTLP raw-type emission primitives (opentelemetry-proto + tonic).
- **conductor-faults** — fault helpers (ramps, silence, port-occupier, fingerprints).
- **conductor-verify** — MCP read-back client (rmcp), preflight gate, and verdict logic.
- **conductor-report** — JSONL journal + Markdown report + `runs.db` (rusqlite) storage seam.
- **conductor-cli** — the `agent-run` binary (headless source of truth; `#[tokio::main(flavor="current_thread")]` + anyhow).
- **conductor-tauri** — Tauri 2 GUI binary (commands + Channel; owns its own runtime).

### Standard Contracts
- **MCP `initialize` preflight readiness gate** (preflight contract, run once at suite start) — asserts negotiated protocol version (pinned `2024-11-05`) + required-tool presence against the pinned manifest, then round-trips a canary incident to prove data-dir wiring; `ready: false` forces dependent scenarios to report `blocked`.
- **query_incident_list** (MCP tool consumed) — lists incidents from Pulse's corpus; filtered by `workspace_root = data_dir`; used by the canary round-trip.
- **retrieve_report** (MCP tool consumed, with `degraded_mode`) — retrieves a Pulse incident report; a `degraded_mode` result maps to the `KnownResidual` state.
- **retrieve_telemetry_slice** (MCP tool consumed) — retrieves a telemetry slice for verification.
- **mark_incident_resolved** (MCP tool consumed) — marks an incident resolved (lifecycle verification).
- **Run report envelope** (artifact/JSON schema, per scenario check) — canonical per-check shape shared by the Markdown report and `runs.db` row (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, `slo_tier`, `fingerprints`); `verdict ∈ {Pass, Fail, CalibrationRegion}`, `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`.
- **OTLP egress liveness check** (transport contract) — confirms the gRPC channel to `127.0.0.1:4317` is connectable before emission; refused transport surfaces as a harness error (`Result::Err`), not a verdict.
- **Pinned MCP contract manifest** (schema file under `contracts/`) — fixes the expected protocol version (`2024-11-05`) and the required-tool set the preflight gate checks against.
- **Tauri 2 `#[tauri::command]` request/response + `Channel`** (IPC contract) — internal core↔UI start/stop/picker actions and live-counter streaming; no REST/GraphQL/tRPC.

### Surfaces
**Product type:** Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims).

(Deferred surfaces: mobile framework is N/A — desktop-only, host-bound; the GUI shell itself is convenience only, the CLI/headless path being the release gate.)

### Test Harness Commitment
**Development Style:** **agent-driven** — the headless `scripts/agent-run.sh` path is the source of truth and the release gate; the GUI is a thin shell over the same core commands. (This signal is read downstream by tests/obs/setup-project specialists.)

### Project Intent Summary
- **Core functionality:** "a scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims)."
- **Target users:** "Personal — solo developer, local dev host, no cloud, no multi-tenancy; runs next to a real Pulse instance on the dev host."
- **Critical paths hint:** Three dynamic end-to-end scenarios are named verbatim in arch (CI/CD and Established Decisions): "error-baseline-spike", "fingerprint-storm", "restart-suppression". Beyond these, scenarios are keyed one-per-Pulse-P-ID (P-001..P-060) — "no scenario without a P-ID"; the complete `coverage-matrix.md` (all 60 P-IDs, zero gaps) is the definition of done. Broader user-facing flow enumeration: derive from input.md or design in Phase 1.

### CI/CD Platform
- **Platform:** GitHub Actions
- **Pipeline note:** Runs `cargo build` + `cargo nextest`/`cargo test` (incl. golden tests) + `cargo clippy` on the dev OS target — build + test gating only; dynamic end-to-end scenario proof requires a live Pulse (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`) and is an explicit operator/local gate, not a CI gate.

### Test-Relevant Conventions
- **Test runner:** cargo-nextest (`cargo nextest`) / `cargo test`, consistent with Pulse's static layer; golden tests run in CI.
- **Per-seam test isolation:** crate-per-seam workspace gives independent `cargo build -p` / `cargo test -p` per seam crate.
- **Scenario keying (test-entity naming):** scenarios are keyed by Pulse capability P-ID (P-001..P-060) — "no scenario without a P-ID"; declarative scenario config (serde + garde) lives one-per-P-ID under `scenarios/`.
- **Source/file naming:** snake_case Rust source files and identifiers; crates named `conductor-<seam>` (kebab-case names, snake_case module paths); on-disk artifacts use kebab-case Markdown filenames (e.g. `coverage-matrix.md`) and per-run JSONL journals stemmed by `run_id`.

---

## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal (0)
- **Justification:** "Every signal points to a minimal-tier local utility — a single-developer, local-only, no-cloud, no-multi-tenancy tool with no user accounts (auth "none"), where the only persisted data is self-generated synthetic test telemetry and run-metadata in an embedded SQLite index (`runs.db`) plus on-disk JSONL/Markdown artifacts (no PII/payment/health/credentials owned by Conductor), and there is zero network exposure — Conductor is a loopback gRPC/MCP client with no inbound listener of its own."

### Attack Vectors
- **Vector 1:** CLI input — seed/scenario flags and `CONDUCTOR_*` env-var overrides on the `agent-run` / `conductor-cli` path. Mitigation: "`std::fs::canonicalize` + explicit type/existence check in `conductor-cli`; reject before any `runs.db`/journal write or manifest read" (path handles sit outside garde struct validation).
- **Vector 2:** config files (local file parsing) — scenario config (`scenarios/`) + pinned MCP contract manifest (`contracts/`) read from disk. Mitigation: "garde validation at load (`range` rules: error fraction ∈ [0,1], non-negative durations, sane ramp factors; `#[garde(custom)]` cross-field: p50≤p95≤p99, severity-mix sums)".
- **Vector 3:** Tauri IPC — internal in-app commands (`#[tauri::command]` start/stop/picker/run-report/operator-pause) + one `Channel` for live counters, over the bundled webview→backend boundary. Mitigation: "in-process app boundary (backend↔bundled webview); not network-exposed" — guarded by a deny-by-default minimal capabilities file allowing only the actual commands.
- **Vector 4:** MCP read-back child-process stdout — Conductor spawns the Pulse MCP server (`andromeda-pulse-mcp`) via `TokioChildProcess` (stdio) and trusts that child's stdout as the read-back source. Mitigation: "preflight readiness gate — negotiated protocol pinned at `2024-11-05`, required-tool presence vs. manifest, data-dir canary round-trip; mismatch/empty ⇒ **blocked** state".
- **Vector 5:** OTLP/gRPC egress — outbound only to `127.0.0.1:4317` (Pulse's loopback ingest), tonic client. Mitigation: "loopback-only egress; a refused transport surfaces as `Result::Err` (harness fault), not a verdict".
- **Vector 6 (single deliberate exception):** port bind `:4317` — the port-occupier fault scenario in the `conductor-faults` seam intentionally binds `:4317` to exercise Pulse's reaction. Mitigation: "loopback only; the *sole* case where Conductor opens a port, and it is an intentional in-host fault, not a network listener accepting external input".

(Vectors explicitly enumerated as "none": public API, file upload, OAuth, WebSocket, UGC, webhook — no web framework, no HTTP surface, local-only deployment.)

### Anti-Patterns Rejected
- **Interactive login on the headless `scripts/agent-run.sh` path** — rejected because: it is agent-driven by design and an auth prompt there would silently break the release gate.
- **`CONDUCTOR_*` env path handles without `std::fs::canonicalize` + bounds-check** — rejected because: path traversal; these handles sit outside garde's struct validation.
- **Interpolating `ANDROMEDA_PULSE_DATA_DIR` (or any operator value) into the sidecar argv or a shell string** — rejected because: rmcp STDIO command/argument-injection class (CVE-2026-30623); must pass strictly via `.env(...)` after rejecting injection metacharacters.
- **Spawning the MCP sidecar from an operator-chosen command** — rejected because: rmcp STDIO design flaw; spawn `andromeda-pulse-mcp` as a fixed hard-coded program path only.
- **String concatenation / `format!` to build SQL for `runs.db`** — rejected because: must use rusqlite bound parameters even though content is self-generated synthetic data.
- **Deserializing scenario config without garde validation at load** — rejected because: garde's `range` + `#[garde(custom)]` cross-field rules are the trust boundary; an unvalidated serde deserialize bypasses it.
- **Decoding prost/protobuf from child stdout without bounded recursion** — rejected because: protobuf-decode DoS lineage (RUSTSEC-2020-0002 / RUSTSEC-2024-0437); an empty canary round-trip must become `blocked`, never a false pass-as-empty.
- **Deprecated crypto algorithms (MD5, SHA-1, DES, RC4, ECB mode)** — rejected because: any future fingerprint/hash added to `runs.db` or the journal must use a modern hash.
- **Reading, copying, or decrypting Pulse's encrypted `corpus.db` (P-049)** — rejected because: it is owned and encrypted by Pulse; Conductor only round-trips a canary incident via MCP read-back.
- **Persisting real secrets/credentials into the unencrypted `runs.db` / JSONL journals** — rejected because: they store only synthetic telemetry and must be treated as world-readable local files.
- **Disabling TLS verification / downgrading to plaintext on any surface promoted beyond loopback** — rejected because: a non-loopback target must not silently skip transport verification.
- **Exposing internal error details (stack traces, file paths, library versions, internal struct names) in any response** — rejected because: the `anyhow` edge and run-report artifacts must stay sanitized.
- **Promoting the `:4317` port-occupier fault bind into a general-purpose inbound listener** — rejected because: it is an intentional in-host fault, not an API surface.
- **Committing `.env` files or any secret-shaped string to the `conductor-*` workspace** — rejected because: Conductor owns no secrets and that invariant must hold.
- **Hardcoding credentials in source / putting credentials in URL query strings or argv** — rejected because: there are none to hardcode; the OS keychain reference is Pulse-side.
- **Introducing a `DATABASE_URL` or cloud-credential env var** — rejected because: the SQLite path is a local file and the tool is local-only.
- **Logging secrets, tokens, or `Authorization` headers** — rejected because: there are none in Conductor and this keeps it that way if a future integration introduces them.
- **Letting run-report artifacts (`<run_id>.md`, `runs.db` rows, JSONL journals) leak absolute host paths or internal seam-crate struct names** — rejected because: they are agent-parseable ground truth shared across hosts and must carry verdict/state/identity fields only.
- **Writing journal/report wall-clock stamps from tokio's virtual clock** — rejected because: a wrong clock source corrupts the ground-truth artifact; use `std::time::SystemTime`/`Instant`.
- **Spawning the MCP sidecar (or anything) via a shell / `eval`-equivalent with operator-supplied input** — rejected because: rmcp STDIO design flaw (CVE-2026-30623); fixed program path + `.env(...)`, no config-into-argv.
- **Using the Tauri `shell-open` plugin with scenario-config-derived strings** — rejected because: the unscoped plugin enables RCE via dangerous protocols (`file://`/`smb://`/`nfs://`, CVE-2025-31477 CVSS 9.3); prefer avoiding the plugin entirely.
- **Embedding remote-origin iframes in the bundled Tauri webview** — rejected because: iframes can bypass origin checks for IPC even in isolation mode (GHSA-57fm-592m-34r7), re-exposing the in-process-only IPC surface.
- **Shipping Tauri commands without a minimal capabilities file** — rejected because: a permissive default exposes unintended IPC; author deny-by-default capabilities for only the actual commands plus the one live-counter `Channel`.
- **Pinning the rmcp client to a strict newer protocol default** — rejected because: it must negotiate down to `2024-11-05` (Pulse's hand-rolled server version); a strict-newer default is the silent-mismatch class the preflight exists to prevent.
- **Following symlinks when changing permissions during crate extraction on a pre-1.94.1 toolchain** — rejected because: tar-rs symlink-chmod flaw in the `cargo build` extraction path (CVE-2026-33056 / RUSTSEC-2026-0033); bump `rust-toolchain.toml` to ≥ 1.94.1.
- **Running `cargo build --release` or merging without `cargo-audit` (and recommended `cargo-deny`) green** — rejected because: the dependency/supply-chain audit is the Minimal-tier residual-risk control for the `bundled`-SQLite-from-C + OTLP/gRPC/MCP tree.
- **Letting `Cargo.lock` drift or go uncommitted** — rejected because: it makes the audit scan non-deterministic and lets the bundled SQLite C version float past advisory tracking.
- **Adding a scenario without a Pulse P-ID, or introducing an inbound network listener of Conductor's own** — rejected because: both violate the architecture's scope law / trust boundary and silently widen the attack surface beyond the loopback-client model.
- **Letting a malformed child/transport input panic** — rejected because: `tonic::Status` codes and MCP error responses are first-class typed verification inputs routed through the verdict/error wall; a panic on the read-back path corrupts run classification.
- **Silently downgrading a failed preflight (version mismatch / missing tool / empty canary / keychain read-while-write fault) to pass/fail/manual-check** — rejected because: it must surface as the distinct `blocked` state with the named precondition string, never a false pass.
- **Adding `unsafe` across the OTLP/gRPC/SQLite FFI boundary without review** — rejected because: the bundled-SQLite C and raw-protobuf/tonic codegen are the FFI surface safe Rust cannot check.
- **Skipping a research-flagged pinned dependency bump** — rejected because: `tauri` ≥ 2.10.3 (origin-confusion CVE-2026-42184) and toolchain ≥ 1.94.1 (tar-rs) are required bumps over the current 2.10.1 / MSRV 1.88.0 pins.

### Data Classifications
- **config — declarative scenario config (serde + garde, no DSL)** (low) — stored in `scenarios/` directory (deserialized into owning seam crates) and `contracts/` pinned MCP contract manifest; testability hint: testable (load-time garde validation with `range`/`#[garde(custom)]` rules; transient at load).
- **test-telemetry / run-metadata (synthetic, not user data)** (low) — stored in `runs/` directory (per-run `<run_id>.jsonl` journal + `<run_id>.md` report) and embedded SQLite `runs.db` index (run_id · seed · scenario · P-IDs · verdict · fingerprints · timestamps); testability hint: testable (self-generated synthetic OTLP fault data, append-mostly on disk; golden tests via cargo-nextest).
- **credential — none owned by Conductor** (N/A) — stored in N/A for Conductor (the `OsKeychainBackend("com.andromeda.pulse")` reference is Pulse-side, exercised via the spawned sidecar's canary as the same OS user); testability hint: untestable in current harness (out of scope — Conductor never stores, generates, or manages any secret).
- **PII — none** (N/A) — stored in N/A (no user accounts, no user-supplied content; only synthetic incident/fingerprint data Conductor itself emits); testability hint: N/A.
- **payment — none** (N/A) — stored in N/A (no payment/billing SDK, no transactions); testability hint: N/A.
- **health — none** (N/A) — stored in N/A (no medical data in stack or intent); testability hint: N/A.

(Note: Pulse's encrypted-at-rest `corpus.db`, P-049, is out of scope — owned and encrypted by Pulse; Conductor only round-trips a canary incident through it via MCP read-back to prove wiring.)

---

## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (desktop / Tauri-2-bundled-webview) — A single-station, frameless, desktop-bound mission-control console (React 19 + Vite SPA + Tailwind v4.1 + shadcn/ui on Tauri 2, Windows/macOS/Linux, no responsive breakpoints) where the operator picks a scenario/suite, watches a live heartbeat count and phase line, answers operator-pause go/no-go holds, and reads the run-report.
- **cli** (cli / terminal) — A headless, line-oriented `conductor-cli` (`scripts/agent-run.sh`; clap + owo-colors + indicatif + comfy-table + inquire, ANSI 256-color, TTY-gated) that is the source-of-truth and release gate, mirroring the same run flow as colored stdout with the headless path never blocked on an interactive prompt.

### Layout Categories

(No `## Layout Templates` / `## Layout Categories` section exists in the upstream; the categories below are derived from each surface's `### Component Patterns`, which serve as purpose-built layout types — flagged for Phase 1 cross-validation.)

- **Titlebar / heartbeat readout** (the signature paused-count + phase line) — used in: desktop-webview, cli
- **Confirmation dialog / prompt** (operator-pause go/no-go) — used in: desktop-webview, cli
- **Dense list / coverage matrix** (single-row-per-P-ID, SLO table) — used in: desktop-webview, cli
- **Report / detail view** (run-report verdict summary, prose + mono) — used in: desktop-webview, cli
- **Picker / selection + controls** (scenario/suite picker, start/stop) — used in: desktop-webview, cli
- **Form / checklist** (operator-checklist for ManualCheck drive+observe) — used in: desktop-webview
- **Error output** (sanitized stderr) — used in: cli

### Brand Identity Anchors

- **Status text labels** (`Pass` / `Fail` / `HOLD` / `CalibrationRegion` / `Blocked` / `Manual` / `Residual`; CLI ASCII prefixes `[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]` / `[MANUAL]` / `[RESIDUAL]`) — drives selector for: per-P-ID verdict/report-state result rows (every status lamp/dot is mandatorily paired with a text label per the Color-Only a11y rule, so text/role selectors are reliable; color is never the sole signal).
- **Mono ID-cyan status tier** (`#7DCFFF` / ANSI 117, JetBrains Mono) — drives selector for: P-ID identifiers (`P-001`..`P-060`), `run_id`, SLO timings, latency_ms, and fingerprints (these mono-rendered IDs anchor selecting/asserting individual coverage-matrix rows and report lines by their P-ID text).
- **ARIA labels / live regions** (`aria-label` on icon-only titlebar + lamp controls; `aria-live` HOLD/verdict announcements; `role="alertdialog"` on the operator-pause dialog) — drives selector for: titlebar min/close controls, the operator-pause go/no-go dialog, and status-change assertions.

(Beyond the above, no decorative brand tokens are relevant to E2E selector strategy — Phase 3 will use role/text/data-testid selectors.)

---

## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** Run console (idle), Run console (live), Run console (HOLD), Run report (terminal)
- **cli:** `conductor run <scenario>`, `conductor suite`, `conductor report <run_id>`, `scripts/agent-run.sh`

### Signature Placements

- **Run console (HOLD) — desktop-webview:** signature element = Heading-role phase line carrying persistent text "HOLD — operator pause" at the frameless titlebar left zone (rationale: "the phase line beside it flips to 'HOLD — operator pause'").
- **Operator-pause go/no-go dialog — desktop-webview:** signature element = shadcn AlertDialog with persistent header text "HOLD — operator pause" + step index at the dialog header (rationale: "the dialog opens carrying a frozen snapshot of the held count value... so the hold-point is legible at the exact moment of the proceed/abort decision").
- **`conductor run <scenario>` / `conductor suite` (HOLD) — cli:** signature element = `[HOLD]` ASCII bracket prefix + persistent text "HOLD — operator pause" printed above the `inquire` confirm (rationale: "a bold amber... `HOLD — operator pause` line printed above the proceed/abort confirm... so the signal is never color-alone and survives `NO_COLOR` / piping / screen readers").

(Beyond the HOLD signature text above, layouts use generic role/text selectors — Phase 3 will use role/text/data-testid selectors per layout, e.g. `data-tauri-drag-region` on the titlebar, status text `[PASS]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` paired with each lamp, and P-ID Data-role tokens like `P-009`.)

---

## 5. Creator Brief Excerpt

### Must-Work Scenarios

- "End-to-end proof against a real Pulse: at minimum `error-baseline-spike`, `fingerprint-storm`, `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce verified expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle` pass observes auto-resolve + resolution summary."
- "Every catalog scenario runs from the control panel AND headless; deterministic under a fixed seed; emission journal written per run."
- "Run report distinguishes pass / fail / manual-check / known-residual; manual checklist renders with induced-state context; P-032 reports as known-residual (not as a surprise failure)."
- "`coverage-matrix.md` complete: all 60 P-IDs classified auto / drive+observe / static-only, zero gaps." ("A P-XXX missing from the matrix is a defect. '0.1.0 done' requires the matrix complete with zero unclassified entries.")
- Coverage classification the harness must honor: "**auto** — Conductor drives AND asserts (journal + MCP read-back + timing)"; "**drive+observe** — Conductor induces the state; the operator confirms the visual/UX claim via the generated checklist"; "**static-only** — no dynamic telemetry dimension ... these stay with Pulse's own test matrix — Conductor explicitly does NOT duplicate them."

### Risk Tolerance Hints

- Scope-law rigor (no feature creep, no untested claims): "Conductor implements **exactly enough** fault injection and scenario surface to verify each P-XXX claim — no more (feature creep), no less (untested claims). Every scenario cites the P-IDs it verifies; a scenario with no P-ID does not exist."
- Assertion-policy split (THE rigor boundary, creator-mandated): "deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) = hard pass/fail; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting — model-side per the amended spec) = calibration-region checks + report-for-human, never hard-failed on exact values."
- Determinism as a hard quality bar: "Deterministic under a seed: same scenario + seed ⇒ same stream shape. Wall-clock-stamped **emission journal** per run = ground truth of what was sent when (the left side of every SLO check)."
- Audience / scale (solo local, MVP-version 0.1.0 but production-grade verification rigor): "The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host."
- Harness-execution mandate: "Development Style: **agent-driven** (headless-drivable core + `scripts/agent-run.sh`; the UI is a thin shell over the same commands)."
- MCP read-back prerequisite that must never be silently assumed: "Operator prerequisite (documented, never silently assumed): Pulse built with the non-default `mcp-server` cargo feature AND runtime env `ANDROMEDA_PULSE_MCP_ENABLED` AND the sidecar spawned with `ANDROMEDA_PULSE_DATA_DIR` equal to the live Pulse's data-dir ... mismatch ⇒ read-back silently empty; preflight must canary-round-trip a known incident before any scenario trusts read-back."

### Test Anti-Patterns (creator's explicit asks)

- "NOT a load-tester: bounded 'typical / high' load profiles only (P-060 SLO checks); 50k+ spans/sec saturation regimes are explicitly out (the spec excludes them from Conductor scenarios; Pulse's own `perf_load_profiles.rs` suite owns saturation)."
- "NO Pulse process management — Conductor never starts/stops/restarts Pulse or edits its config; those are operator-pause steps. (Keeps Conductor honest: it validates Pulse from the outside, as a real workload.)"
- "NO UI automation of Pulse (Playwright/axe live in Pulse's own suites) — visual claims are operator checklist items."
- "NO scenario DSL — declarative config files + the built-in catalog; new behavior = new P-XXX first."
- "NO multi-target / distributed generation; no cloud; local dev host only."
- Reference prior art read-only (do not vendor): "Reference prior art in the Pulse repo (read, don't vendor): `crates/ingest/examples/inject_demo.rs` (minimal injector) and `crates/ingest/examples/load_profiles.rs` ... — proven emission patterns to mine."
- Environment requirement for two scenario families: "P-032/P-036 scenarios run inside a real git workspace with known recent commits (documented per-scenario environment requirements)."
