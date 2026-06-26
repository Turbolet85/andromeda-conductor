# security extract

## Relevance
partial — the chunk introduces a Tauri GUI control surface (commands, IPC, UI dependencies) that crosses three security boundaries (Tauri IPC, CLI input validation, frontend supply chain); the core run execution DRIVES the existing pipeline unchanged

## Constraints
1. Deny-by-default `capabilities/*.json` — extend with ONLY the `list_scenarios`/`list_suites`/`start_run`/`stop_run` allowlist entries, no blanket permission (security-plan §Code Patterns; Tauri GUI forward-guardrail ban on permissive defaults)
2. No `shell-open` plugin with scenario-derived strings — if the GUI ever opens paths/URLs, set explicit allow-scope; the unscoped plugin enables RCE via dangerous protocols (security-plan §Code Patterns, CVE-2025-31477; forward-guardrail for this chunk's surface)
3. `tauri` ≥2.10.3 required in `[workspace.dependencies]` — current declared version 2.10.1 is vulnerable to origin-confusion CVE-2026-42184 on Windows (security-plan §Dependency Security §Update policy + §Anti-Patterns)
4. `npm audit` clean (0 vulnerabilities) before merging frontend changes + committed `package-lock.json` — new Radix/cmdk/shadcn/utility deps under `crates/conductor-tauri/ui/` are not covered by `cargo-audit`/`cargo-deny` (security-plan §Dependency Security §Frontend (npm) supply chain)
5. Scenario selection validated against known P-ID catalog BEFORE command execution — never interpolate user-selected scenario strings into argv/shell or pass to subprocess without pre-validation (security-plan §Input Validation §Scenario config files + §Anti-Patterns §Input)
6. The `start_run` selection and any new scenario-enumeration API are trusted-local (operator-controlled `CONDUCTOR_SCENARIOS_DIR`), not remote; the scenario string must not flow to the spawned MCP sidecar's argv (security-plan §Threat Model Summary §Attack surface §Vector: Tauri IPC)

## Patterns to follow
1. Use Tauri `#[tauri::command]` typed return values (not panics) for all list/start/stop commands; route errors through `anyhow`-sanitized responses (no absolute paths, struct names, or stack traces to the webview) (security-plan §Error Handling)
2. `start_run` executes on the core-owned `current_thread` runtime in a managed Tauri async task; Stop requests cancellation; treat `tonic::Status` codes and MCP errors as typed verification inputs routed through the verdict wall (security-plan §Error Handling + §Threat Model Summary §Vector: MCP read-back)
3. Observe all commands via the existing `conductor-tauri` `tracing` sink (`logs/conductor-tauri.jsonl`); redaction reuses the processor-stage policy (no new obs boundary) (security-plan §Logging & Monitoring; obs-plan §3)

## Anti-patterns to avoid
1. NEVER spawn the MCP sidecar or anything via shell/eval with scenario-derived input — fixed program path + `.env(...)` only, no config-into-argv (security-plan §Code Patterns §Anti-Patterns, rmcp STDIO design flaw CVE-2026-30623)
2. NEVER pre-add later-chunk command permissions (`run-report`, `operator-pause`, live-`Channel` allowlist) to `capabilities/*.json` in this chunk — extend only with the list/start/stop commands needed here (security-plan §Code Patterns §Security Anti-Patterns — ship deny-by-default, permit only the actual surface)
3. NEVER embed remote-origin iframes in the bundled webview or skip Tauri IPC origin guards — in-process boundary assumption is part of the threat model (security-plan §Code Patterns, GHSA-57fm-592m-34r7 + CVE-2026-42184 combined re-exposure)

## Contract bindings
- **Tests harness** ↔ acceptance-criteria gates: `ui/` `tsc` strict + `npm audit` clean + workspace `cargo nextest` + `cargo audit`/`cargo-deny` green + `agent-run.sh run` exit 0 (security-plan §Dependency Security §CI integration)
- **Obs** ↔ self-observation: list/start/stop commands emitted to `logs/conductor-tauri.jsonl` via existing `tracing` sink + processor-stage redaction (no new policy required) (obs-plan §3)
- **A11y static rules** (not test harness yet): picker fully keyboard-operable (Radix/cmdk roving focus), buttons semantic + visible focus rings, selection/state not-color-alone (a11y-plan §Keyboard/Forms-controls/Focus/Visual)

## Acceptance criteria contributions
1. (security) `capabilities/*.json` deny-by-default with ONLY `list_scenarios`/`list_suites`/`start_run`/`stop_run` allowlist entries; grep verifies no unintended permission pre-added
2. (security) Scenario selection pre-validated against known P-ID catalog in `start_run` before execution (grep + test trace confirms no unchecked string flow)
3. (security) `npm audit` clean (0 vulnerabilities) + `package-lock.json` committed (frontend supply-chain gate passes)
4. (security) `cargo audit` green + `cargo deny check advisories bans sources licenses` green with any new advisory justified in `deny.toml` (workspace supply-chain gate passes)
5. (security) `tauri` ≥2.10.3 pinned in `[workspace.dependencies]` (origin-confusion CVE-2026-42184 mitigated on Windows)

## Relevant amendment history
- **2026-06-15-dependency-audit-gate** (§Dependency Security): audit tool versions (cargo-audit 0.22.1 / cargo-deny 0.19.4) are minimum floors; RustSec advisory DB fetched fresh each run; toolchain floor 1.94.1 confirmed satisfied (1.95.0 current). Tauri ≥2.10.3 remains a forward **required bump** — dormant until Epoch-9 GUI chunk (this chunk); audit remit + body reconciliation belong here
- **2026-06-15-design-token-typography-bundle** (§Dependency Security §Frontend (npm) supply chain): npm/frontend supply-chain control gate added — `npm audit` clean (0 vulns) + committed `package-lock.json` + vendored fonts (no runtime CDN). Applies to all npm changes in `crates/conductor-tauri/ui/` (Radix/cmdk/shadcn/utility deps introduced in this chunk); no rigid Minimal-tier dependency allowlist; npm advisories drive same floor discipline as cargo
