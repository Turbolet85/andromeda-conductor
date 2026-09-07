# Architecture — Amendments

_Append-only changelog of amendments to `architecture.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1
**Section:** §Stack and Technologies (+ §Infrastructure Patterns Build system · §Inherited Defaults · directory-tree comment)
**Change:** MSRV pinned to 1.94.1 (was 1.88.0) across all occurrences.
**Why:** the cargo-workspace-scaffold chunk implemented the security-plan §Dependency Security required bump (≥1.94.1, tar-rs CVE-2026-33056); arch's stated MSRV 1.88.0 was superseded. The workspace pins build toolchain 1.95.0 with `rust-version = "1.94.1"` as the MSRV floor.

## 2026-06-14-cargo-workspace-scaffold — self-observation stack row added
**Section:** §Stack and Technologies
**Change:** added a "Self-observation | tracing 0.1.44 + tracing-subscriber 0.3.23" row.
**Why:** the chunk pinned `tracing` + `tracing-subscriber` in `[workspace.dependencies]` (the obs-plan §3 self-obs stack); arch's Stack table did not list them. NOT an OTel SDK — OTLP remains the PRODUCT emission.

## 2026-06-15-conductor-core-shared-types — serde_json 1.0 registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "Serialization (JSON) | serde_json 1.0" row.
**Why:** the conductor-core-shared-types chunk added `serde_json = "1.0"` to `[workspace.dependencies]` (canonical-name round-trip tests now; the run-report envelope + per-run JSONL journal consume it at runtime in Epoch 6); arch's §Stack registry did not list it. Cleared by the security drift-detector (audit-green, allowed by §Dependency Security). Cascaded to `.claude/docs/stack.md`. The design proposal (record an implementation note in the Decisions Log) was rejected as non-drift — design-system.md §cli already prescribes the implemented prefixes.

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Stack and Technologies (Validation row) · §Established Decisions [Validation Library] · §Inherited Defaults (Validation)
**Change:** garde version 0.23.0 → 0.22.1 across all three; §Established Decisions now records that garde 0.22.1's `#[garde(custom)]` is field-level only (no container-level custom) — cross-field invariants spanning distinct fields use garde's `Context` pattern.
**Why:** the config-validation-surface chunk needed garde's `derive` feature, but `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + `derive` is unbuildable; user authorized the downgrade to 0.22.1. The no-duplicate-P-IDs validator consequently landed field-level (not struct-level). Cascaded to `.claude/docs/stack.md`. Cleared by the security + arch drift-detectors (§Dependency Security carried no garde pin).

## 2026-06-15-structured-logging-stack — CONDUCTOR_SERVICE_NAME / CONDUCTOR_ENV registered
**Section:** §Occupied Resources (Environment variables)
**Change:** added `CONDUCTOR_SERVICE_NAME` (self-obs `service.name` override) and `CONDUCTOR_ENV` (self-obs `deployment.environment`, default `local`) to the reserved `CONDUCTOR_*` env-var list.
**Why:** the structured-logging-stack chunk's `init_observability` reads both (obs-plan §3) to populate `ServiceIdentity`; the arch env-var inventory listed only RUNS_DIR/SCENARIOS_DIR/CONTRACT_MANIFEST/SEED. No cascade — CLAUDE.md / stack.md do not enumerate env vars (grep-confirmed). D-arch-decisions cleared (tracing/tracing-subscriber already in §Stack; no OTel SDK introduced).

## 2026-06-15-design-token-typography-bundle — frontend stack + ui/ asset subtree registered
**Section:** §Stack and Technologies (new Desktop-frontend row) · §Occupied Resources (Frontend asset subtree) · §Inherited Defaults (Frontend bullet)
**Change:** recorded the optional GUI's realized frontend toolchain — React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide via `@tailwindcss/vite`) + Fontsource WOFF2, package manager npm (committed `package-lock.json` + `npm audit` gate); registered the `crates/conductor-tauri/ui/` asset subtree (not a Cargo member; `node_modules/` + `dist/` git-ignored) and the npm/frontend default.
**Why:** the design-token-typography-bundle chunk landed the first frontend code under `crates/conductor-tauri/ui/`; arch §Stack pinned no Vite major (Vite 8.0.16 was the npm-audit floor clearing esbuild GHSA-gv7w-rqvm-qjhr), and §Occupied Resources / §Inherited Defaults did not record the frontend asset layer. User-approved at the wrap escalation (all-4 spec recordings). Cascaded to `.claude/docs/stack.md`. D-arch-resources + D-arch-decisions cleared (React 19 / Tailwind 4.1 / Fontsource already allowed by §Stack; Vite major was unpinned).

## 2026-06-16-test-framework-fixtures-coverage-tooling — test/coverage toolchain registered in Build system
**Section:** §Infrastructure Patterns — Build system
**Change:** the test clause now names cargo-nextest as the pinned runner (zero-retry `ci` profile in `.config/nextest.toml`) + `cargo test --doc`, the dev-test stack (rstest/proptest/insta/assert_cmd/assert_fs/predicates), and `cargo-llvm-cov` line coverage requiring the `llvm-tools-preview` toolchain component; exact versions deferred to test-plan §4.
**Why:** the test-framework chunk landed exactly this toolchain (report Changes + Outcome — 46/46 nextest, 91.97% llvm-cov, all gates green); arch §Infrastructure named only `cargo test` / cargo-nextest loosely. Routine per playbook rule #5 (spec→sound-impl alignment). Cascaded to `.claude/docs/stack.md`. D-arch-resources cleared (no new crate/port/env var); D-arch-decisions resolved by this registration.

## 2026-06-16-seeded-phase-scheduler — seedable RNG (rand_chacha/rand_core) registered in §Stack + §Established Decisions
**Section:** §Stack and Technologies (new Determinism RNG row) · §Established Decisions (new [Determinism RNG] entry)
**Change:** added a "Determinism RNG | rand_chacha 0.9 (`ChaCha8Rng`) + rand_core 0.9 (`SeedableRng`)" Stack row and a [Determinism RNG] established decision locking `ChaCha8Rng` + `seed_from_u64` (platform/version-stable) as the timeline scheduler's sole non-determinism source.
**Why:** the seeded-phase-scheduler chunk added rand_chacha 0.9 / rand_core 0.9 to `[workspace.dependencies]` for deterministic per-gap jitter (report §Dependencies); arch §Stack listed no RNG and §Established Decisions did not record the RNG/seeding choice. Routine per playbook rule #5 (spec→sound-impl alignment) — arch §Cross-cutting already mandated a "seeded RNG"; this records the concrete algorithm-stable realization. Cascaded to `.claude/docs/stack.md`. D-arch-resources + D-arch-decisions resolved by this registration. D-security-deps escalate was a verified false positive (tauri already 2.10.3, untouched by this chunk; real new deps audit/deny-green) — dismissed with the user; a playbook rule was added for the misfire class.

## 2026-06-16-scenario-config-model — toml 0.9 registered in §Stack + [Scenario Config Format] decision
**Section:** §Stack and Technologies (new Scenario config (TOML) row) · §Established Decisions (new [Scenario Config Format] entry)
**Change:** added a "Scenario config (TOML) | toml 0.9" Stack row and a [Scenario Config Format] established decision locking declarative TOML scenario config (serde-deserialized + garde-validated via `Scenario::from_toml_str`), chosen over JSON for hand-author ergonomics + inline comments across the 60 per-P-ID files.
**Why:** the scenario-config-model chunk added `toml = "0.9"` to `[workspace.dependencies]` + conductor-core for the declarative per-phase emission spec (report §Dependencies + §Decisions — the P4 user format decision); arch §Stack listed only serde_json and §Established Decisions recorded no scenario config format. Routine per playbook rule #5 (spec→sound-impl alignment) + the serde_json / rand_chacha precedents (a new audit-green dep → §Stack row, a locked format choice → §Established Decisions). Cascaded to `.claude/docs/stack.md` (CLAUDE.md no delta — its overview Stack line is high-level). D-arch-decisions resolved by this registration; D-security-deps cleared by the security detector (toml audit/deny-green, Cargo.lock committed); the other 5 docs returned `proposals: []`.

## 2026-06-18-exception-events-fingerprint-control — fingerprint primitive placed in conductor-emit
**Section:** §Infrastructure Patterns (directory-tree crate comments) — cascaded to CLAUDE.md §Modules
**Change:** the per-exception fingerprint PRIMITIVE (`fingerprint()` + the exception-event builder, identical/path/line/type/frame variants) is recorded in `conductor-emit` (co-located with the exception content it derives from); `conductor-faults`' "fingerprint generation" is narrowed to the fingerprint-STORM fault (Epoch-7), which will depend on emit and compose this primitive.
**Why:** the exception-events-fingerprint-control chunk landed `fingerprint()` + `exception_trace_request()` in `conductor-emit` per the crate-seam the user ratified in /andromeda-phase (AskUserQuestion — the error-spans "builder-in-emit" precedent; faults is an empty Epoch-4 stub). arch's dir-tree comment had attributed all "fingerprints" to faults. Routine per playbook rule #5 (spec→sound-impl alignment) + the explicit prior user ratification — documentation alignment, no behavioral change. Cascaded to CLAUDE.md §Modules (stack.md has no crate-ownership mention — grep-confirmed; no delta). The other 6 docs returned `proposals: []`.

## 2026-06-21-run-report-envelope-serializer — ManualCheck widened + Verdict→ReportState default mapping
**Section:** §Read-Back Dependency Posture · §Probabilistic-Assertion Policy
**Change:** ManualCheck's definition broadened from "operator-checklist / no-programmatic-read-back only" to ALSO include an auto-measured model-interpretive (calibration-region) check; recorded the default `Verdict → ReportState` mapping (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`, via `Verdict::default_report_state`), with `verdict`/`state` kept independent and the run-report lamp chosen verdict-first (a calibration row renders HOLD, not Manual).
**Why:** the chunk shipped `Verdict::default_report_state` (P4 decision, user-approved) + `RunRecord::measured`; arch's narrow ManualCheck definition was stale. The arch drift-detector false-negatived (claimed arch already prescribed it — it did not); the orchestrator authored the amendment, cross-confirmed by the a11y detector. Verdict-first lamp precedence resolves the a11y §6 six-lamp conflict (escalated + user-confirmed 2026-06-21). Cascade no-op (CLAUDE.md / stack.md carry no ManualCheck-definition detail — grep-confirmed).

## 2026-06-21-runs-db-index — SQLite/libsqlite3-sys versions corrected to the shipped lock
**Section:** §Stack and Technologies (Database row) · §Inherited Defaults (Database)
**Change:** `libsqlite3-sys 0.38.0 → 0.36.0` and bundled `SQLite 3.51.1 → 3.50.4` across both spots (rusqlite 0.38.0 unchanged).
**Why:** the runs-db-index chunk landed the first real `rusqlite 0.38.0 bundled` compile; rusqlite 0.38.0 transitively pins `libsqlite3-sys 0.36.0`, which bundles SQLite 3.50.4 (verified from the bundled `sqlite3.h` + `Cargo.lock`) — arch's stated 0.38.0 / 3.51.1 were assumed, not the resolved lock. The functional invariant holds (bundled, JSON1 proven by the `json_array_length` test, the §Established-Decisions ≥3.38 floor satisfied). Routine per playbook spec→sound-impl alignment; audit-green (cargo-audit exit 0 / cargo-deny ok). Cascaded to `.claude/docs/stack.md`. Mirrored in security-plan §Infrastructure (same correction).

## 2026-06-21-runs-db-index — runs.db instant columns are TEXT RFC-3339, not integer-ms offsets
**Section:** §Data model conventions · §Standard Contracts (Timestamp formats)
**Change:** the `journal_emitted_at`/`read_back_observed_at` `runs.db` columns are stored as TEXT RFC-3339 (the JSONL envelope's wire form); the integer-millisecond value the SLO math consumes is named as the separate `latency_ms` INTEGER column (was: "stored as the same integer-millisecond journal offsets … not ISO strings").
**Why:** the chunk's P4 user decision (AskUserQuestion → Option A). The envelope carries only second-precision RFC-3339 instants + a precomputed `latency_ms`; storing the two instants as epoch-ms would add a date-parser dep for no SLO-math gain (`latency_ms` already IS the journal-relative delta the math consumes, stored INTEGER). The original "integer-ms, not ISO strings" wording under-specified the columns given the envelope's actual shape; the SLO-math invariant (integer-ms via `latency_ms`) is preserved. User ratified at P4 ("records a doc-reconcile note for wrap"). No cross-doc conflict (obs §3/§6 already use RFC-3339 TEXT). Cascade no-op for this entry (CLAUDE.md/stack.md carry no per-column timestamp detail — grep-confirmed).

## 2026-06-23-conductor-run-suite-report-verbs — clap 4 registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "CLI argument parsing | clap 4 (`derive`)" Stack row.
**Why:** the conductor-run-suite-report-verbs chunk added `clap = { version = "4", features = ["derive"] }` to `[workspace.dependencies]` + conductor-cli for the `run`/`suite`/`report` verb surface (report §Dependencies); arch §Stack listed no CLI arg parser. Routine per the serde_json/toml/rand_chacha precedent (a new audit-green dep → §Stack row). `cargo audit` exit 0 (clap + transitives clean); `Cargo.lock` committed. Cascade no-op: CLAUDE.md overview Stack line is high-level (no clap — the toml precedent), and `.claude/docs/stack.md` already lists clap (line 34, cli-surface). D-arch-resources + D-arch-decisions resolved by this registration; clap needs no §Established Decisions entry (a standard CLI parser, not a consequential fork). The 3 security/obs escalations (D-security-input / D-security-subprocess / D-obs-redaction) were dismissed as consume-shipped-hardened-infra false positives (new playbook rule added); D-security-deps (tauri, untouched at 2.10.3) + D-tests-obs-harness (§3↔§3 pre-existing) dismissed via existing rules; layouts D-layout-surface satisfied by §Primary screens line 175 (report verb already documented); design + a11y clean. The CLI-level `scenario.run` root span (obs §4) is a carried code follow-up (this chunk is the Epoch-8 driver), not a doc drift — obs §4 stays target-state.

## 2026-06-23-5-command-agent-run-harness — CONDUCTOR_PREFLIGHT_TIMEOUT registered
**Section:** §Occupied Resources (Environment variables)
**Change:** added `CONDUCTOR_PREFLIGHT_TIMEOUT` (preflight readiness-gate timeout in seconds, default 30; read by `scripts/agent-run.{sh,ps1}` `boot`) to the reserved `CONDUCTOR_*` env-var list.
**Why:** the chunk's `conductor preflight` verb (the `agent-run boot` entrypoint) is gated by this timeout, which `agent-run.{sh,ps1}` read (`PREFLIGHT_TIMEOUT_SEC`); the arch env-var inventory omitted it. It predates this chunk (Epoch-1 skeleton) but was never registered, and this chunk makes the preflight path it gates load-bearing — an accurate gap the report names (it IS consumed + genuinely absent), NOT the not-added-dependency misfire. No cascade — CLAUDE.md / stack.md do not enumerate env vars (the CONDUCTOR_SERVICE_NAME/ENV precedent). D-arch-decisions cleared (no new stack/dep; the clap-4 `preflight` subcommand is an allowed-pattern addition). The other 6 docs: security/obs/tests/design/a11y returned `proposals: []`; layouts registered the verb in its own §cli Primary screens.

## 2026-06-23-line-oriented-output-rendering — terminal-rendering stack (owo-colors/indicatif/comfy-table) registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "Terminal output rendering | owo-colors 4 + indicatif 0.17 + comfy-table 7" Stack row (the `conductor-cli` presentation layer — tty-gated status-line color, run/suite progress spinner, results + 60-P-ID coverage tables).
**Why:** the chunk added owo-colors 4.3.0 / indicatif 0.17.11 / comfy-table 7.2.2 to `[workspace.dependencies]` + conductor-cli for line-oriented output (report §Dependencies); arch §Stack listed no terminal-rendering layer. Routine per the clap/serde_json/toml precedent (a new audit-green dep → §Stack row); `cargo audit` exit 0 + `cargo deny check` exit 0 (the indicatif→number_prefix advisory + the pre-existing foldhash Zlib license recorded as accepted deny.toml exceptions — security-plan §Dependency Security). Cascade: CLAUDE.md overview Stack line is high-level (no per-lib — the clap/toml precedent); `.claude/docs/stack.md` already lists the cli rendering libs (line 34) — corrected its `indicatif 0.18`→`0.17` to the resolved lock. D-arch-decisions resolved by this registration (standard presentation crates, no §Established Decisions fork). D-arch-resources (proposing the `conductor coverage` verb + the render module's public fns into §Occupied Resources) dismissed as over-reach — the verb is layout-templates' concern (registered there), the library API is the §Occupied-Resources library-symbol over-reach; a new playbook rule was added.

## 2026-06-23-isatty-gated-operator-pause — inquire 0.9 registered in §Stack (terminal-rendering row)
**Section:** §Stack and Technologies (Terminal output rendering row)
**Change:** appended `+ inquire 0.9` to the terminal-rendering Stack libs (owo-colors/indicatif/comfy-table) + the role gained "isatty-gated interactive operator-pause prompts (inquire confirm; headless never blocks)".
**Why:** the chunk added `inquire = "0.9"` (→0.9.4) to `[workspace.dependencies]` + conductor-cli for the CLI interactive `PauseResolver` (report §Dependencies); arch §Stack's terminal-rendering row listed no interactive-prompt lib. Routine per the clap / owo-colors-indicatif-comfy-table precedent (a new audit-green dep → §Stack row): `cargo audit` exit 0 + `cargo deny check` exit 0 (inquire + crossterm 0.29 / fuzzy-matcher 0.3.7 / derive_more 2.1.1 transitives clean — NO new deny.toml exception needed, unlike ch3's number_prefix/foldhash). Cascade: CLAUDE.md overview Stack line is high-level (no per-lib — the clap/toml/ch3 precedent), no-op; `.claude/docs/stack.md` already listed the cli prompt lib — corrected its `inquire 0.7`→`0.9` to the resolved lock (the ch3 indicatif-0.18→0.17 cascade pattern). No §Established Decisions entry (a standard CLI-prompt crate, not a consequential fork — the ch3 presentation-crates precedent). D-arch-decisions resolved by this registration; D-arch-resources clean (no new port/socket/endpoint/IPC/event/env-var/crate; the `CliResolver`/`PromptResolver`/`hold_line` symbols are the library-symbol over-reach arch omits). The other 6 docs returned `proposals: []`: security (inquire audit/deny-green, no new input boundary/spawn), obs (tracing-only, no OTel SDK; redaction preserved), tests (nextest/rstest on-spec; harness unchanged), design (amber via `lamp_code` token, never color-alone), layouts (the `[HOLD]` line + inquire confirm already in §cli wireframes/§Output structure), a11y (cli surface is "not-assertable" per §1; zero schema change).

## 2026-06-24-sanitized-stderr-agent-mode-logging — registered CONDUCTOR_AGENT_MODE env var + logs/agent-latest.jsonl artifact
**Section:** §Occupied Resources (Environment variables + On-disk artifacts)
**Change:** registered two genuinely-new occupied resources — the `CONDUCTOR_AGENT_MODE` env var (a read-only agent-mode trigger; `flag || env-set`, main never writes it) and the `logs/agent-latest.jsonl` on-disk artifact (the self-obs `tracing` JSON stream in agent mode; a SEPARATE artifact + schema from the emission journal, a sibling of the runs dir moving with `CONDUCTOR_RUNS_DIR`).
**Why:** D-arch-resources fired on the report's two new Changes-listed resources. Routine (NOT the flag/symbol/config-file/token over-reach the playbook dismisses): arch §Occupied Resources explicitly tracks env-vars + on-disk artifacts in its enumerated list, and neither was registered. The `--agent-mode`/`--debug` CLI FLAGS stayed layout-templates' concern (the already-documented §cli "Error output" + pipe discipline), not arch — per the CLI-verb/flag over-reach rule. Cascade: CLAUDE.md GENERATED:setup:* + stack.md enumerate neither env-vars nor per-artifact paths (high-level overview) → no-op. D-arch-decisions clean (no new dep — std-only file sink; the `ObsWriter` enum-dispatch mirrors the shipped `CliResolver` precedent).

## 2026-06-24-frameless-window-shell — registered logs/conductor-tauri.jsonl artifact + @tauri-apps/api/tauri-build + the generate_context! build-order coupling
**Section:** §Occupied Resources (On-disk artifacts) · §Stack and Technologies (Desktop frontend row) · §Infrastructure Patterns (Build system)
**Change:** registered `logs/conductor-tauri.jsonl` (the Tauri backend self-obs `tracing` JSON stream via `ObsSink::File`, sibling of the runs dir — the GUI shell's analogue of `agent-latest.jsonl`); added `@tauri-apps/api` (window/IPC client) to the Desktop frontend stack row; documented `tauri-build`'s `generate_context!` resolving `build.frontendDist` (`ui/dist`) at COMPILE time → the webview bundle must be built before any workspace cargo compile of `conductor-tauri` (the `ensure_frontend` step wired into `agent-run.{sh,ps1}` + the CI Rust job), plus the Tauri tree's justified `deny.toml` additions (unmaintained gtk/unic/proc-macro-error advisories + `MPL-2.0` / `Apache-2.0 WITH LLVM-exception` licenses).
**Why:** the first real Tauri 2 app (report §Changes). D-arch-resources fired; the GENUINE new occupied resource is the `logs/conductor-tauri.jsonl` artifact (arch §Occupied Resources tracks on-disk artifacts — the agent-latest.jsonl precedent registered the sibling). `@tauri-apps/api` + `tauri-build` are genuinely-added deps (report §Dependencies); the `generate_context!`-frontend-before-cargo coupling is a real build-system constraint affecting CI + the harness. DISMISSED (user-confirmed 2026-06-24): the 3 `core:window:*` capability PERMS into §Occupied Resources (framework ACL perms in the deny-by-default `capabilities/` file — security-plan's domain, NOT Conductor IPC methods; the Conductor command surface is unchanged) → new playbook rule. D-arch-decisions clean (tauri 2.11.3 ≥2.10.3 floor; React/Vite/Tailwind already in §Inherited Defaults). The escalate-severity detectors returned clean: security (`[]` — deny.toml inline-justified, tauri ≥floor, deny+audit green, no new input boundary/spawn), obs (no OTel SDK; the Tauri sink inherits the UNCHANGED `init_observability` redaction layer), a11y (`[]` — banner+button+focus-ring covered; the `ObsSink` rename is not a schema change). Cascade: stack.md gains `@tauri-apps/api` + the build-order note; gotchas.md gains the disk + build-order gotcha (P3 curation).

## 2026-06-26-live-counter-channel-stream — registered the conductor-run library crate (9th workspace member)
**Section:** §Occupied Resources (Crate names) · §Infrastructure Patterns (directory tree)
**Change:** added `conductor-run` to the workspace-member registry + the directory tree — the run composition root library (preflight + execute_scenario + persist + the live-counter drive_run) sitting above the seams and below both bins, shared by conductor-cli + conductor-tauri.
**Why:** D-arch-resources (warning). The chunk extracted conductor-cli's bin-local pipeline.rs into a NEW workspace crate (report §Changes — "ADDED conductor-run (9th workspace member)"); a workspace member IS an occupied resource arch §Occupied Resources tracks (the crate-names list), distinct from the library-symbol / command-name / module / config-file / capability-perm over-reach the playbook dismisses (those rules exclude new crates). Routine new-crate registration. D-arch-decisions clean (the core-owned current_thread runtime under Tauri matches §Async Runtime Flavor; the live-counter Channel was already pinned in §Real-time Strategy; no new external crate). The escalate-severity detectors returned clean: security (`[]` — start_run input validated via validate_selection+resolve_under, sidecar spawn unchanged, no new external dep — audit+deny green), obs (`[]` — tauri.command.* spans + run_id, no OTel SDK, RunEvent path-free), a11y (`[]` — the live counter drives the EXISTING titlebar aria-live region, already anticipated in a11y §1/§4/§5). Cascade: CLAUDE.md §Modules + §Key directories + stack.md gain conductor-run.

## 2026-06-27-desktop-a11y-harness-setup — npm-audit gate one-liners made dev-aware (`--omit=dev`)
**Section:** §Stack and Technologies (Desktop frontend row) · §Inherited Defaults (Frontend)
**Change:** the two npm-audit gate one-liners `npm audit` → `npm audit --omit=dev` (production-dep strict; dev-only test-tooling advisories accepted at dev-tree grain) — kept consistent with the authoritative security-plan §Dependency Security amendment.
**Why:** D-arch-decisions (warning) fired on the new npm devDeps + the gate change. The substantive amendment is security-plan's (the a11y harness's dev-only transitive advisories → dev-aware gate, user-decided); arch carries derived one-liner summaries of it, updated here in lockstep. DISMISSED: registering the a11y test devDeps (axe-core / webdriverio / lighthouse / colorjs.io / @crabnebula/tauri-driver) into arch §Inherited Defaults / §Stack — over-reach per the 2026-06-26 frontend-component-package rule (the test/harness tooling lives in a11y-plan §3.5 + the stack.md distillation, which already lists @crabnebula/tauri-driver + axe/lighthouse/colorjs.io; arch §Stack/§Inherited Defaults summarize the frontend stack at React/Vite/Tailwind/npm grain, never enumerated test tooling). No new workspace crate / port / socket / env-var. Cascade: stack.md (lines 32 + 39) carry the gate one-liner (updated in lockstep); the a11y tooling rows in stack.md (37/40) already existed.

## 2026-06-27-mcp-read-back-result-shape-adapter — [MCP Read-Back Client] REVERSED: rmcp removed → hand-rolled JSON-RPC
**Section:** §Stack and Technologies (MCP read-back row) · §Established Decisions [MCP Read-Back Client] · §Read-Back Dependency Posture · §Conventions (Inbound verification + Error handling) · §Inherited Defaults · directory-tree comment
**Change:** the read-back client decision flips from "rmcp 1.7.0 (official SDK; typed `list_all_tools()`/`call_tool()`; version negotiation via `peer_info()`)" to "hand-rolled line-delimited JSON-RPC over the sidecar's stdio" — `initialize`/`tools/list`/`tools/call` returning the RAW `serde_json::Value`; version negotiation reads the `initialize` result's `protocolVersion`; rmcp removed from conductor-verify. The hardened spawn + the `2024-11-05` manifest pin + the preflight gate are unchanged.
**Why (incl. the superseded rationale, preserved here per the body's pointer):** the original [MCP Read-Back Client] rationale read "rmcp 1.7.0 — official SDK with version negotiation + typed tool calls; **hand-rolled JSON-RPC and third-party rust-mcp-sdk were rejected** because re-deriving version negotiation is the silent-mismatch risk class the preflight exists to prevent" (with a caveat that Pulse's server is itself hand-rolled `2024-11-05`). That was sound at design time but **contradicted by the live SUT**: Pulse's `andromeda-pulse-mcp` is non-MCP-compliant for `tools/call` — it returns the raw tool payload as `result` (no `{content:[…]}` envelope), which rmcp's typed `call_tool` deserializes into `ServerResult` and rejects as `UnexpectedResponse` on EVERY live call (rmcp exposes no raw-result escape — verified in rmcp 1.7 `service/client.rs:282-327` + `service.rs:442`). So the "rejected hand-rolling" clause is reversed: hand-rolling is the faithful match to a hand-rolled non-compliant server; version negotiation reduces to reading one `initialize` field. Discovered implementing Live-Pulse E2E proof → re-planned as this prerequisite; user-confirmed the hand-rolled approach at /andromeda-phase P4 + the reversal at the wrap escalation (2026-06-27); a playbook rule was added (SUT-contradicts-a-locked-decision → routine-apply). DISMISSED clean: D-arch-resources (rmcp removed, no new crate/port/env), security D-security-subprocess (spawn hardening preserved) + D-security-deps (no new dep; rmcp removed; audit/deny green), obs/tests/design/layouts/a11y (`proposals: []` — spans/no-OTel/redaction/harness/envelope all hold). Cascaded to CLAUDE.md GENERATED:setup:* (§Modules / §Architecture) + `.claude/docs/stack.md`.

## 2026-06-27-live-pulse-e2e-proof — canary premise reversed (Pulse incident creation is LLM-in-the-loop non-deterministic) + corpus.db plaintext + read-back surface
**Section:** §Established Decisions [Read-Back Dependency Posture] · §Standard Contracts (Readiness gate + corpus access) · §Occupied Resources (`ANDROMEDA_PULSE_DATA_DIR`)
**Change:** the canary "emit one known incident → assert `query_incident_list` returns it" round-trip is superseded — Conductor emits a unique fingerprint-storm (telemetry) and the fidelity carrier is `retrieve_telemetry_slice.fingerprint_refs` (Pulse scrubs incident titles). The original premise is unattainable: Pulse's incident creation is NON-DETERMINISTIC + LLM-in-the-loop (OTLP → L1 → L2 RetryStorm cue [deterministic, ≥5 same fingerprint/30s] → L3 digest [20-60s cadence] → **L4 llama.cpp Llama-3.2-3B decides Dismiss/Severity** → incident), so deterministic incident-readback verification against the real LLM is impossible — an OPEN posture decision (leading: a deterministic test-L4 mode in Pulse). `corpus.db` is **plaintext SQLite** (the P-049 "encrypted at rest / `OsKeychainBackend`" assumption is WRONG → keychain-failure canary mode N/A); the live read-back surface is **8 tools** (only the 4 persistent-corpus tools work cross-process from a Conductor-spawned sidecar — the in-memory-buffer tools return empty); the incidents filter column is `workspace` (not `workspace_root`).
**Why:** verified live against Pulse this session (operator findings). Part A (the canary fingerprint-fidelity bridge) shipped + CI-green (nextest 420/420), but live `conductor preflight` correctly returns `Blocked: incident not found in corpus` — ingest works (`conductor-canary` in `service_registry`) yet Pulse creates no incident (`incidents` 0 rows; `baseline_state` 0; all services silent; config `mcp_server_enabled=false`; both dense + sustained storms tried). REVERSES a §Standard Contracts assumption because the live SUT contradicts it — escalate-once-then-apply (playbook 2026-06-27 locked-assumption-reversal rule); user-confirmed in the wrap directive (record Part A + defer Part B + apply this amendment; the deterministic-verification posture is the chunk's PENDING follow-up, NOT resolved this session). Part B (the 2 live families + live `ready:true`) deferred — blocked on the same open decision. Cascade: NO CLAUDE.md / stack.md edit (their canary/MCP distillations carry no stale claim — the "empty canary → Blocked" warning + the hand-rolled-JSON-RPC stack row both still hold). Pulse run recipe (future live pass): pulse-app needs `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`; build needs `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (a triage/build.rs bug truncates the 9 MB tokenizer → L4 breaks); RetryStorm ≥5/≥10 same fingerprint/30s; L4 ≈4 s/inference.

## 2026-08-08-sut-capability-manifest — Accepted capability set is manifest data (SUT-advance reversal)
**Section:** §Established Decisions (new [Accepted Capability Set]) · §Occupied Resources On-disk artifacts · §Infrastructure directory tree · §Conventions Naming · §Cross-cutting Scope law · §Project Intent
**Change:** The accepted Pulse P-ID set moved from the compile-time `(1..=60)` bound in garde's `pid_format` to the versioned runtime-read `contracts/pulse-capabilities.toml`; validation split into shape (garde) + membership (`Scenario::check_capabilities` via `from_toml_str_with`); a malformed/absent manifest is a `CoreError` harness fault, never Blocked. Every "all 60 P-IDs" / "P-001..P-060" claim de-hardcoded to name the manifest's accepted set.
**Why:** Pulse's ledger advanced to P-082 during a 41-day pause and a scenario naming anything above P-060 failed garde validation at load, so the harness could not express what the SUT had shipped. Ratified at wrap (escalate-once, playbook rule 27's shape); de-hardcoding rather than substituting 82 was the operator's decision, so the next Pulse release needs no doc edit.

## 2026-08-08-dependency-advisory-remediation — anyhow pin 1.0.102 → 1.0.104
**Section:** §Stack and Technologies (Error handling row) · §Established Decisions [Error Handling] · §Inherited Defaults (Error handling)
**Change:** the three arch-registered `anyhow 1.0.102` pins refresh to `anyhow 1.0.104`. thiserror 2.0.18, the anyhow-only-at-binary-edges rule, and the verdict/error-wall prose are untouched — only the version value moves.
**Why:** the chunk cleared four decayed advisories by dependency bump, one of which was RUSTSEC-2026-0190 (`anyhow` 1.0.102, `unsound` in `Error::downcast_mut()`, patched `>=1.0.103`) — an advisory `cargo audit` reports as an exit-0 allowed warning while `cargo deny` denies it, so it was invisible to an audit-only check. The operator's P4 decision raised the `[workspace.dependencies]` floor to `1.0.104` alongside the lock bump, so the manifest no longer names a version with a known unsound advisory; arch's pins followed. The amendment surface is **architecture.md only** — verified at the chunk's P5 review that security-plan carries no anyhow version (its ten `anyhow` mentions are all error-sanitization *edge* references; §Dependency Security's version-bearing content is the cargo-audit ≥0.22 / cargo-deny ≥0.19 tool floors and the `tauri` ≥2.10.3 line), so no lockstep security-plan amendment was proposed or made. Validated routine under playbook:28-30 (spec-value → sound-impl reconciliation; the decision's invariants preserved). Fan-out was otherwise clean: 6 of 7 docs returned `proposals: []`. Cascade: `.claude/docs/stack.md` §Error handling; no CLAUDE.md edit (its anyhow mention carries no version). NOT amended, as pre-existing gaps this chunk did not introduce (playbook:46, → handoff follow-ups): arch §Stack's `tokio 1.48.x` vs the resolved 1.52.3, arch §Stack's Tauri `v2.10.x / latest 2.10.1` vs the resolved 2.11.3, and `deny.toml`'s 17 ignores / 8 allows vs security-plan §Accepted exceptions naming one of each.

## 2026-08-09-current-sut-coverage-classification — Re-aiming is a manifest edit PLUS a classification row
**Section:** §Established Decisions [Accepted Capability Set] · §Occupied Resources (`contracts/pulse-capabilities.toml`) · §Stack and Technologies (Terminal output rendering row)
**Change:** The [Accepted Capability Set] decision's closing claim — "re-aiming at a newer Pulse release is now a manifest edit with no Rust change" — is qualified to current truth: the **accepted set** is data, but the **coverage classification** stays code-native (`coverage_matrix()`, a no-runtime-IO `static`), and `check_sut_drift` holds the two to set-equality. Re-aiming is therefore a manifest edit **plus** a matching classification row; an accepted id with no row, or a classified id the manifest no longer accepts, is a `CoreError::SutDrift`. The `KNOWN_UNCLASSIFIED` residual ledger is recorded as retired to `[]`. §Occupied Resources' echo of the same claim follows in lockstep. Separately, the §Stack terminal-rendering row's "60-P-ID coverage tables" reads "capability-coverage tables (the manifest's accepted set)".
**Why:** The chunk classified all 82 manifest-accepted capabilities and emptied the residual ledger, making the manifest↔classification coupling exact and unavoidable. The two-source shape is deliberate, not incidental: the operator's P4 representation decision chose code-native classification precisely so `check_sut_drift` stays a genuine two-source comparison rather than collapsing into a self-check — so the decision's invariant (the accepted set is DATA, never a compile-time constant) is fully preserved and only its consequence-wording moved. Validated routine under playbook:28-30 (spec-wording → sound-impl reconciliation). The §Stack row edit landed at /implement (see the process note in this chunk's report §Deviations 1); this entry is the history half that flow owed. Cascade: CLAUDE.md `GENERATED:setup:*` + `.claude/docs/stack.md`.

## 2026-08-09-interpretation-correctness-posture — second coverage-integrity gate + interpretation-correctness deferral
**Section:** §Established Decisions [Accepted Capability Set] · [Read-Back Dependency Posture]
**Change:** [Accepted Capability Set] gains the second integrity gate on a distinct axis — `check_scenario_backing` holds the `UNBACKED_AUTO` pin (`Auto`-classified capabilities no scenario names) to exact-set equality against the committed catalog, failing on a new unbacked claim / pin rot / a pin that lost its `Auto` classification (`CoreError::UnbackedCoverage`); only `Auto` participates. [Read-Back Dependency Posture]'s "OPEN posture decision" is closed: the deterministic-L4 option landed Pulse-side, and its cost is recorded as a deferral — "Conductor green" does not mean interpretation is trustworthy — owned by a conductor-0.3.0 entry and pinned in `.andromeda/residuals.md`.
**Why:** the chunk shipped both the gate and the deferral; arch described `check_sut_drift` as the single integrity gate and still carried the posture question as open, so the body no longer matched current truth.

## 2026-08-09-sut-load-envelope — load-envelope artifact + runs.db second table registered
**Section:** §Occupied Resources (On-disk artifacts) · §Infrastructure Patterns (directory tree) · §Stack (ORM row) · §Established Decisions [ORM]
**Change:** Registered `contracts/pulse-load-envelope.toml` as a committed on-disk artifact (terms + provenance + exemption ledger; fixed `LoadEnvelope::default_path()` through `resolve_under`, no `CONDUCTOR_*` override; only the duration term asserted, rate terms declared-not-derivable). Extended the `runs.db` bullet to its two tables (`runs` per-check + the additive run-level `run_envelope`) and restated the [ORM] / §Stack qualifier from "~one indexed table" to a small fixed set of hand-written tables. Directory-tree `contracts/` comment now names three manifests.
**Why:** The chunk landed a third committed contracts artifact and a second runs.db table; §Occupied Resources enumerates committed artifacts, and the one-table qualifier no longer matched. The no-ORM decision itself is unchanged.

## 2026-08-10-workspace-key-divergence-probe — readiness gate: a fourth named precondition
**Section:** §Standard Contracts — Readiness gate (the `ready:false` paragraph)
**Change:** The gate's named-precondition set is stated as FOUR (was three): protocol version-mismatch ·
required-tool absence · emitted fingerprint absent from an existing incident · **app/sidecar workspace-key
agreement**. The fourth is emitted on a zero-incident `query_incident_list`, which is byte-identical on the
wire to a workspace-key divergence (the sidecar keys on `ANDROMEDA_PULSE_DATA_DIR`, `pulse-app` on its
detected workspace root), so the string names the key agreement AND "Pulse raised no incident" as the two
candidate causes rather than claiming a measurement Conductor cannot make — read-back exposes no second key
(`query_incident_list` takes no arguments). Also states that every precondition string is host-path-free
(`data_dir` redacted).
**Why:** the chunk replaced the opaque `canary round-trip failed: incident not found in corpus` with the
named precondition, verified by four `conductor-verify` test legs and three live `conductor preflight --json`
runs. Pulse-side mechanics confirmed first-hand (`corpus/src/contract.rs:628` · `pulse-app/src/main.rs:688-693`
· `mcp-server/src/bin/andromeda-pulse-mcp.rs:74`).

## 2026-08-10-pulse-run-contract — the run contract registered as the fourth `contracts/` manifest
**Section:** §Occupied Resources — On-disk artifacts (+ the `contracts/` line in the §Infrastructure Patterns directory tree)
**Change:** registered `contracts/pulse-run-contract.toml` — the pinned Pulse run contract (`sut_version` · `captured_at` · `provenance` · an `[incident_formation]` table · a `[[term]]` list), runtime-read and bounds-checked at load, resolved from a fixed `RunContract::default_path()` through `resolve_under` with deliberately NO `CONDUCTOR_*` override handle. The entry records what each `check` kind MEANS as a statement about what Conductor can honestly know — `shell-declaration` (observable in Conductor's own environment, the only kind that can block), `asserted` (satisfied by construction), `declared-not-observable` (true on the SUT's side with no read-back surface, recorded but never blocking, because blocking would claim a measurement). The directory-tree comment moved from three manifests to four.
**Why:** the chunk landed the artifact; §Occupied Resources enumerates each `contracts/` manifest individually (the `pulse-capabilities` / `pulse-load-envelope` precedent), so a fourth left unregistered is an unregistered resource. Detector-raised (D-arch-resources) off the report's Files + Schema/config bullets.

## 2026-08-10-pulse-run-contract — preflight timeout floor + the L4 declaration Conductor reads
**Section:** §Occupied Resources — Environment variables
**Change:** `CONDUCTOR_PREFLIGHT_TIMEOUT` keeps its default of 30 but now carries a run-contract-derived effective FLOOR (`[incident_formation].min_canary_poll_seconds`) it cannot sit below, because the bare default is shorter than Pulse's own L3 digest cadence; the env handle still overrides upward. Added `ANDROMEDA_PULSE_L4_DETERMINISTIC` — Pulse-side, asserted not set by Conductor, but now READ in Conductor's own environment as the contract's shell-declaration proxy, with an absent declaration surfacing as an unmet term naming both candidate causes rather than a claimed measurement of `pulse-app`.
**Why:** the chunk made both changes real; the registry stated the bare 30s default and did not list the L4 var at all. Detector-raised (D-arch-resources) off the report's env-vars bullet.

## 2026-08-10-pulse-run-contract — readiness gate: a fifth named precondition
**Section:** §Standard Contracts — Readiness gate
**Change:** the gate's named preconditions are FIVE, adding **unmet run-contract terms**. That arm composes ONE string naming each unmet term individually (its condition and its candidate causes) and sits after the tool checks but BEFORE the canary arms, skipping the canary poll rather than paying it — an unmet launch condition explains a failed canary, so surfacing the canary symptom first sends the operator to the wrong cause, and under the raised poll floor it would spend the whole budget doing so.
**Why:** the chunk added the arm; the section stated FOUR. The ordering rationale is recorded because the preceding workspace-key probe hit exactly that failure mode — it blocked for the no-incident reason while the real blocker sat upstream. Detector-raised (D-arch-resources) off the report's Counts/qualifiers bullet + Deviation 4.
## 2026-08-11-faithful-emission-dispatcher — load-envelope rate terms; error-fraction encoding; nested-spec `dive`
**Section:** §Occupied Resources (`contracts/pulse-load-envelope.toml`) · §Established Decisions [Validation Library] · §Conventions (Config conventions)
**Change:** The load envelope's rate terms move from *declared-not-derivable* to **derivable but deliberately not yet asserted** — `EmissionSpec::occurrences` is exactly the per-phase occurrence-count field whose absence was the old justification; the assertion surface and the `[[exempt]]` ledger are unchanged, and re-scoping to emitting-phase duration is recorded as carried chunk work rather than an amendment. [Validation Library] and Config conventions now name the shipped error-fraction encoding (`EmissionShape::Error { error_percent: u32 }` ∈ 0..=100, an integer percent rather than an f64 so `PhaseSpec`/`Scenario` keep `Eq`), state that a nested spec field must `dive` and never `skip`, and record that a phase's emission shape is declared data (`EmissionSpec { signal, occurrences, shape }`, `occurrences: 0` = a deliberate silence window).
**Why:** The chunk shipped the per-phase emission dispatcher: the declarative shape model landed in `conductor-core`, which falsified the load envelope's stated premise and replaced an aspirational `[0,1]` bound with a real integer-percent field. `PhaseSpec.emission` was `#[garde(skip)]`, so the nested rules the specs mandated had never executed.

## 2026-08-13-dispatcher-determinism-goldens — load-envelope asserted terms inverted; exemption ledger retired
**Section:** §Occupied Resources — the `contracts/pulse-load-envelope.toml` bullet
**Change:** The two sustained terms (`max_sustained_storm_ms` + `max_sustained_rate_spans_per_s`) are now the ASSERTED ones, judged per emitting phase; `max_scenario_duration_ms` inverts to recorded-but-not-asserted. `check_load_envelope` and `LoadEnvelope::classify` read ONE shared basis, so the static gate and the run-level `[ENVIRONMENT-SUSPECT]` caption cannot diverge. The `[[exempt]]` ledger is retired to EMPTY (still exact-set in both directions). The bullet now also records that the artifact's own predicted landing — asserting SUMMED emitting-phase duration — was measured and FALSIFIED, and why the shipped term bounds the longest single emitting window instead.
**Why:** The chunk discharged the re-scope this bullet had recorded as carried chunk work, but not by the term it predicted: measured across all 35 committed scenarios, summed emitting-phase duration leaves `activity-floor` (900 000 ms) and `incident-auto-resolution` (610 000 ms) over the 600 000 ms ceiling, so it retires no exemption and changes no gate verdict. Summing disjoint bursts separated by quiet is not *sustained*. Under the per-phase joint bound every scenario passes unaided (longest single emitting phase 600 000 ms, exactly at the ceiling; peak rate 4.00 spans/s against 10 000), so the ledger genuinely empties.

## 2026-08-13-first-live-green-preflight — [incident_formation] warm-up measured false
**Section:** Occupied Resources — `contracts/pulse-run-contract.toml`
**Change:** The claim that the warm-up pre-roll "carries the canary service out of Pulse's baseline bootstrap before the counted storm" is recorded as measured FALSE by the first live leg, with the mechanism: Pulse gates cue evaluation on `BootstrapState::Ready` (`cue/evaluate.rs:164`), which needs `now − first_observed_unix_nanos ≥ BOOTSTRAP_WINDOW_SECONDS = 3_600` per service wall-clock (`baseline/activity_floor.rs:33,173-183`), while `baseline_state` holds 0 rows so each launch resets the anchor. `warmup_ms = 45000` is short by 80x and unfixable by its own knobs; the `check = "asserted"` term cannot catch its own falsity. The underlying diagnosis (no baseline ⇒ the cue evaluator never considers the service) is unchanged — only the remedy is disproved, and the fix is Pulse-side.
**Why:** Three arms of the live probe, all blocked for the no-incident cause with `cues_emitted: 0` and `services_ready: 0` throughout; `incidents` 0 rows in total. Evidence: `chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` §Re-run — 2026-08-13.

## 2026-08-14-canary-fingerprint-feed-capture — a second Pulse-side gap on the canary path
**Section:** §Occupied Resources (`contracts/pulse-run-contract.toml`)
**Change:** recorded beside the warm-up falsification: Conductor's storm reaches the wire with its `exception`
events intact and Pulse receives and counts all nine spans, yet the fingerprint table stays empty — so the gap
lies inside Pulse between OTLP ingest receipt and the per-span-event fingerprint observer, a REGION rather than
a named defect. Also records how to read that telemetry: `tracked_fingerprints_count` is a 60s-windowed gauge
over DISTINCT fingerprints sampled after eviction (a working six-occurrence identical-fingerprint storm reads
1, never 6), and the window-immune discriminators are the cumulative `storms_detected_total` /
`fingerprints_evicted_total`.
**Why:** the chunk's capture settled its fork. The finding bears directly on the run contract's
`[incident_formation]` premise — the canary storm is emitted to raise an incident, and the feed that would
raise it never engages — so it belongs beside the warm-up falsification the same section already carries.
Evidence: 31 tick lines across three arms, twelve inside the retention window, all counters zero; `span_count`
verified as a cumulative counter in the SUT's source rather than inferred from its shape.

## 2026-08-15-canary-storm-autonomous-band — the bootstrap misattribution corrected, the second gap un-retired
**Section:** §Occupied Resources (`contracts/pulse-run-contract.toml`) · §Established Decisions [Read-Back Dependency Posture]
**Change:** Five edits across two sections. §Occupied Resources: (1) the `BootstrapState::Ready` gate is scoped to the **baseline-derived cue families** — `cue/evaluate.rs:164` is the only such gate in `crates/triage/` and sits inside `evaluate_service_went_silent`, while the RetryStorm path consults no baseline (`pattern/storm.rs:245-285`); (2) "reaching a live incident requires a Pulse-side change (bootstrap override / `baseline_state`)" is retired as measured-false and replaced by the **tier band** — `CANARY_STORM_COUNT = 6` sat in `5 <= 6 < DEFAULT_AUTONOMOUS_THRESHOLD = 10` with Tier-1 Autonomous-only (`cadence/coordinator.rs:390`), raised to 12 by this chunk; (3) the second Pulse-side gap is **un-retired and sharpened** from "a region, not a named defect" to producer-dependent and localized between OTLP ingest receipt and buffer span-event enumeration, on the `buffer.tick` trio (`span_events_seen` / `observer_invocations` / `fingerprints_computed` all 0 across 15 ticks, `rows_ingested: 1` against `span_count: 15`); (4) the windowed-gauge reading instruction no longer names the retired six-occurrence size, and the span-count figures are dated (nine at the 2026-08-14 capture, 15 = 3 warm-up + 12 storm at the 2026-08-15 leg). §Established Decisions [Read-Back Dependency Posture]: (5) the write-path's `≥5 same fingerprint / 30s` now states that the floor raises a **Suggested** cue and only `≥10` reaches the Autonomous band Tier-1 requires — the duplicate occurrence of the claim edit (2) retires.
**Why:** The chunk's live leg measured all three. The citation `evaluate.rs:164` was correct throughout and only its cue-family attribution was wrong, so the doc taught a Pulse-side blocker that measurement disproves — while the gap that IS live had been predicted retired. Evidence: `conductor-0.2.0/chunks/2026-08-15-canary-storm-autonomous-band/report.md`; `andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/premise-check.md`.

## 2026-08-15-canary-spans-pulse-fingerprints — the fingerprint-derivation match claim measured false
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** The parenthetical "the fingerprint — computed to match Pulse's derivation — is the fidelity
carrier, not a title echo" is retired. The body now keeps the fidelity-carrier role (Pulse scrubs titles, so
the fingerprint and not a title echo is what carries fidelity) and states plainly that the fingerprint is
**NOT** currently computed to match Pulse's derivation, naming both: Conductor computes FNV-1a 64-bit
rendered as 16 hex chars over `exception_type` + each frame's `function`
(`conductor-emit/src/exception.rs:115-123`); Pulse computes blake3 truncated to 16 bytes over
`exception_type` + `\0` + `normalize_stacktrace(stacktrace)` and reads it back as an 8-char hex prefix of the
first 4 bytes (`andromeda-pulse crates/buffer/src/fingerprint.rs:79-110`, HEAD `d090314`). Equality is
impossible by WIDTH alone, so the canary round-trip's final precondition fails BY CONSTRUCTION even when
every upstream stage succeeds; aligning the derivations is owned by the successor route entry.
**Why:** this chunk's live leg (`run_id 2026-08-16T08-17-48-786`) drove the canary all the way through
ingest, buffer append, fingerprinting, Autonomous-tier storm detection and incident formation, and still
ended `ready:false` at `canary fingerprint not found in telemetry slice` — which isolated the last
precondition and made the mismatch measurable for the first time. The claim was unreachable until now: no
canary span had ever survived to be fingerprinted. Evidence:
`conductor-0.2.0/chunks/2026-08-15-canary-spans-pulse-fingerprints/evidence/leg-verdict.md` §3.

## 2026-08-15-canary-spans-pulse-fingerprints — the readiness gate's round-trip annotated (duplicate occurrence)
**Section:** §Standard Contracts (Readiness gate)
**Change:** The restated canary round-trip ("assert its fingerprint reads back … to prove the
data-dir/workspace wiring end-to-end") now carries the caveat that this assertion cannot currently succeed
for a reason that is not a wiring fault — the two derivations differ in algorithm, input and width — so the
gate ends `ready:false` at this last precondition even when every upstream stage is proven, and a
`canary fingerprint not found in telemetry slice` block must be read as a derivation mismatch rather than
as broken data-dir/workspace wiring.
**Why:** the per-occurrence sweep for the retired claim found this second, independent restatement of the
round-trip's proving power. A single-site apply at §Established Decisions would have left §Standard
Contracts still teaching that a failed round-trip implicates the wiring — the precise misreading the
2026-08-16 leg disproves. `dependent-of: D-arch-decisions`.

## 2026-08-15-canary-spans-pulse-fingerprints — the second ingest-to-fingerprint gap CLOSED, cause Conductor-side
**Section:** §Occupied Resources (`contracts/pulse-run-contract.toml`)
**Change:** The "SECOND, independent **Pulse-side** gap … **This gap is NOT closed**" passage is replaced.
The gap is recorded CLOSED (2026-08-16) and its cause reattributed to **Conductor's own side**: Pulse's
`spans` table is `PRIMARY KEY (trace_id, span_id)` (`andromeda-pulse crates/buffer/src/schema.rs:38`) while
Conductor's `ok_span` stamped a CONSTANT `vec![1; 16]` / `vec![1; 8]` identity on every call, so every
warm-up span after the first violated the key and was logged-and-skipped — exactly the observed
producer-dependence, since `inject_demo`'s per-sequence ids never collide. The shared builder carried the
same defect through the scenario dispatcher (`dispatch.rs:82`, `:95`), not the canary alone. The fixed leg's
numbers are recorded (27 `duckdb.append` lines with zero `reject_reason`, trio `12/12/12`,
`rows_ingested: 15`, `storms_detected_total: 2` with `severity_hint: "autonomous"` at `occurrence_count: 10`,
incident formed), together with one explicit limit: why the storm's 12 DISTINCT-id spans also appended zero
rows on the prior leg was never observed, so the appender-poisoning reading stays **inferred, not proven**.
The trailing telemetry-reading sentence is re-based to carry both directions — the same cumulative
discriminators reading 0 on the broken path and `2`/`1` on the fixed one, with `tracked_fingerprints_count`
still sampling 0 on a healthy late tick.
**Why:** the plan's `Expected amendments (wrap)` named this paragraph, and no detector proposed it — the
orchestrator raised it at Validate check 5 as the chunk's coverage floor. The section asserted a Pulse-side
gap that was neither Pulse-side nor open, which would have mis-aimed the successor chunk's research.
Evidence: `conductor-0.2.0/chunks/2026-08-15-canary-spans-pulse-fingerprints/evidence/leg-verdict.md`.

## 2026-08-16-canary-fingerprint-derivation-aligned — Stack gains a hashing row
**Section:** §Stack and Technologies
**Change:** New `Hashing / digest` row — blake3 1.8.6 (`blake3 = "1"` in `[workspace.dependencies]`), a NORMAL
(non-dev) dep of `conductor-emit`, version-matched to the SUT's own pin under the Pulse-consistency mandate;
Conductor's first hashing dependency, pulling `arrayref`/`arrayvec`/`constant_time_eq`/`cpufeatures`.
**Why:** the chunk adopts Pulse's own fingerprint derivation, so the algorithm is the SUT's choice rather than
Conductor's; the registry carried no hashing row at all, and a test-scoped dep cannot back a shipped signature.

## 2026-08-16-canary-fingerprint-derivation-aligned — Read-Back Dependency Posture reversed (canary carrier)
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** The canary no longer asserts the emitted fingerprint reads back via
`retrieve_telemetry_slice.fingerprint_refs`; it asserts Pulse opened an incident AFTER the storm's emission
instant, and reached `ready:true` for the first time on 2026-08-16. Records why the old carrier could never
work (the field is fed from the L4 model's `evidence_refs`, pinned `[]` under deterministic L4; Pulse's own
fingerprint lands in a `span_events` column no MCP tool reads), that Conductor's fingerprint now IS Pulse's
derivation (blake3, 32 hex, first 3 normalized frames), its two identity narrowings, and the causation-in-time
limit of freshness. The superseded FNV-1a/width-mismatch rationale is retired from the body to here.
**Why:** ratified as a locked-decision reversal (playbook 2026-06-27 rule) — the live SUT's field provenance
contradicted the decision's mechanism, measured at P3 and confirmed on the 2026-08-16 leg. The decision's
invariants (prove data-dir/workspace wiring before any scenario trusts read-back; never a silent downgrade)
hold via the replacement; only the carrier changed.

## 2026-08-16-canary-fingerprint-derivation-aligned — Readiness-gate contract re-based (duplicate occurrence)
**Section:** §Standard Contracts (Readiness gate paragraph)
**Change:** The round-trip description moves off the fingerprint assertion onto incident freshness, the
by-construction-failure caveat is retired, and a staleness block is documented as "corpus reachable, this run
raised nothing" rather than "wiring broken".
**Why:** the same retired claim restated at a second independent site; a single-site apply at §Established
Decisions would have left §Standard Contracts teaching an unreachable gate — the duplicate-occurrence lesson
this project recorded at the predecessor chunk, applied in the inverse direction.

## 2026-08-16-canary-fingerprint-derivation-aligned — third named precondition renamed (duplicate occurrence)
**Section:** §Standard Contracts (the gate's named preconditions are FIVE)
**Change:** The third of the five preconditions changes from "the emitted fingerprint absent from an existing
incident" to "no incident opened after the canary storm was emitted"; the COUNT stays FIVE and the arms'
ordering is unchanged.
**Why:** third restatement of the retired precondition, in an enumeration a prose-only apply would not reach.
Mirrors `NotFound::FingerprintAbsent` → `NotFound::StaleCorpus` in the code.

## 2026-08-16-canary-fingerprint-derivation-aligned — deny.toml exceptions no longer Tauri-only
**Section:** §Infrastructure Patterns (Build system)
**Change:** The accepted-license note records that `BSD-2-Clause` entered for `arrayref` (via
`conductor-emit`→`blake3`), the first cargo-side exception from outside the Tauri tree, and separates
`cargo deny` (green) from `cargo audit` (red on an external advisory-DB fault).
**Why:** the prose attributed every accepted license to the Tauri tree, which this chunk's dependency made
false; conflating the two gates' states would also misread the supply-chain posture.

## 2026-08-16-fingerprint-storm-live-proof — RBDP path narrowing corrected to the leading segment
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** the SECOND identity narrowing — recorded by the `2026-08-16-canary-fingerprint-derivation-aligned`
entry above as "`normalize_frame` strips absolute paths only, so a RELATIVE-path change is
identity-significant" — is RETIRED by measurement and replaced: of a frame's `file` only the **LEADING PATH
SEGMENT** is identity-significant, because `is_absolute_path_start` fires on ANY `/` followed by a path char
(not merely a leading one) and `skip_absolute_path` then consumes everything from the first slash. So
`src/worker.rs` ≡ `src/anything/else.rs` are ONE identity, `other/worker.rs` is another, and a leading `/`
erases the segment entirely. The narrowing COUNT stays two; the P-017 clause (c) qualifier now runs through
the leading segment, NOT through relative-vs-absolute.
**Why:** the chunk reshaped `FingerprintVariant::PathVariant` to vary the path BELOW its leading segment
(adding `RelativePathVariant` for the significant half) after the planned absolute-path mechanism measured
false — base normalized to `at fn (src)`, the absolute variant to `at fn ()`. Byte-verified identical in
`andromeda-pulse crates/buffer/src/fingerprint.rs:139-218` at HEAD `d090314`, so this is the SUT's semantics,
not a transcription drift; pinned by `exception.rs::only_the_leading_path_segment_reaches_the_preimage`. The
prior entry is left as written (sidecars are append-only) — this entry is the supersession record.

## 2026-08-17-fingerprint-semantics-token-leading — P-017 narrowings 2 → 1; normalization is token-leading
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** the leading-path-segment narrowing is RETIRED and the clause now states TOKEN-LEADING
normalization — `is_absolute_path_start` guarded by `is_token_boundary`, so a relative path is preserved in
full and is identity-significant at EVERY depth (`src/worker.rs`, `src/anything/else.rs`, `other/worker.rs`
are three distinct identities) while a token-leading absolute path is stripped to nothing
(`at handler(/usr/lib/thing.rs:10)` → `at handler()`), which differs from every surviving relative path. The
insensitive axes are named as LINE and HEX ADDRESSES. The first-`NORMALIZED_FRAMES = 3` narrowing survives,
so the count goes two → one. The pin moves from the retired
`exception.rs::only_the_leading_path_segment_reaches_the_preimage` to
`::relative_paths_are_significant_at_every_depth` + `::token_leading_absolute_paths_normalize_to_the_same_empty_form`.
The transcription citations are de-literalized to function names and re-based from HEAD `d090314` to `efabe8e`.
**Why:** the SUT changed its normalization between the two HEADs, so the wording recorded at
`2026-08-16-fingerprint-storm-live-proof` (immediately below) measured FALSE at `efabe8e` — evidence: Pulse's
own `compute_differs_for_relative_paths_differing_below_leading_segment`,
`normalize_stacktrace_preserves_relative_paths_in_full` and `normalize_stacktrace_strips_only_absolute_paths`,
reproduced Conductor-side by the new pinning tests. The consequence the clause now records is the one nothing
in this repo could have caught: because Conductor had transcribed the PRE-guard scanner, its `fingerprint()`
was returning a different value than Pulse's for every slash-bearing path — including the committed base
fixture — and every gate stayed green over it, because they assert Conductor against Conductor.

## 2026-08-18-error-baseline-spike-live-proof — deterministic-L4 evidence_refs de-vacuumed; envelope sample re-tiered
**Section:** Established Decisions [Read-Back Dependency Posture] · Standard Contracts (readiness gate + run-report envelope sample)
**Change:** The "fixture pins `evidence_refs` to `[]`" claim retired at both arch sites — the SUT's deterministic fixture now populates a constant `det-*` triple (measured live 2026-08-18, SUT HEAD `efabe8e`); freshness-carrier conclusion unchanged, its supporting fact re-based from emptiness to payload-invariance. The envelope sample re-tiered `<5s` → `<90s` with a note that the error-baseline-spike family ships declare-only (live rows `verdict: null` / `state: KnownResidual`).
**Why:** Leg A's envelope carried the 3 `det-*` refs (`runs/2026-08-18T18-47-32-786.jsonl`); both TOMLs re-declared `<90s` and retired their checks under the family re-calibration clause. Chunk report + `evidence/leg-verdict.md`.

## 2026-08-18-restart-suppression-live-proof — canary service identity registered
**Section:** §Occupied Resources — Service / process names
**Change:** Registered the two emitted OTLP `service.name` identities: `conductor` (`DEFAULT_SERVICE_NAME`, the scenario dispatcher) and `conductor-canary` (`CANARY_SERVICE_NAME`, the preflight canary's warm-up + storm), with the rationale for the split (preflight runs inside every scenario leg; Pulse keys `persistence_seconds` = cumulative samples and error-rate EWMAs per service).
**Why:** The chunk split the canary off the dispatcher's identity so preflight traffic cannot age a scenario's young-sample suppression window; the wire-visible identity was previously unregistered (report §Symbols/APIs, Deviation 2).

## 2026-08-19-pii-scrub-live-proof — declare-only family note gains pii-scrub
**Section:** Standard Contracts — Run report envelope (per scenario check)
**Change:** The declare-only note now names BOTH families — error-baseline-spike (retired 2026-08-18) and pii-scrub (retired 2026-08-19) — shipping zero [[expected]] checks because no read-back surface can carry them under deterministic L4, their live rows landing verdict: null / state: KnownResidual under the degraded read-back with the live claims graded at the harvest tier.
**Why:** The chunk retired all five pii-scrub checks after leg A measured the vacuous-Absent / structural-fail-Contains behavior the sources predicted (run 2026-08-19T20-37-25-933); the note previously named error-baseline-spike only. Sole arch occurrence (agent grep verified).

## 2026-08-19-connection-lifecycle-live-proof — declare-only family list re-based to the full six
**Section:** Standard Contracts — Run report envelope (declare-only note)
**Change:** The note now names all SIX declare-only families (fingerprint-storm · error-baseline-spike · latency-regression · restart-suppression · pii-scrub · the connection family's four TOMLs) instead of the two it had grown incrementally.
**Why:** The connection family retired declare-only this chunk (all four `[[expected]]` checks → 0, measured structurally ungradeable — connection state reaches no MCP read-back surface); the incremental two-name list under-stated the standing set.

## 2026-08-19-connection-lifecycle-live-proof — the per-phase fault model + driver registered
**Section:** Conventions — Config conventions · Established Decisions [Validation Library] · Occupied Resources — Crate names · Infrastructure Patterns — directory tree
**Change:** Registered the optional `[phases.fault]` declared-data table (`PhaseSpec.fault: Option<FaultSpec>`, closed `FaultKindSpec::PortOccupier`, garde dive, fault ⇒ occurrences 0 via `fault_phases_are_silent`); the cross-field-invariant enumeration gains the silence rule (field-level one altitude up, like `no_duplicate_pids`); conductor-run's registry entry + tree comment gain the fault-phase occupier guard and the new `conductor-run → conductor-faults` edge (conductor-faults' first consumer).
**Why:** The chunk shipped the port-occupier driver as declared phase data over the guard-generic timeline hook; the config surface, the invariant and the crate edge were unregistered (report §Symbols/APIs · §Crates/modules · §Schema/config).

## 2026-08-20-latency-regression-re-proof — the second route to KnownResidual, and the rate term's real basis
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** Recorded the shipped read-back routing carve-out: after a GREEN preflight, an EMPTY active list routes a declare-only scenario (zero `[[expected]]`) to the pre-accepted auto-resolve residual (`KnownResidual` / `verdict: null`) instead of `Blocked`, because an emptied active list is Pulse's own auto-resolve lifecycle and a scenario grading nothing cannot pass falsely on an empty observation; checks-bearing scenarios and every read-back call failure still land `Blocked`. Named the shipped mechanism (`route_read_back` / `ReadBack {Graded, AutoResolved, Blocked}`, crate-private in `conductor-run`) and carried the honest limit — the arm is unit-pinned only and did NOT fire on the 2026-08-20 live leg.
**Why:** The carve-out was operator-ratified at the chunk's P5 and shipped; arch's posture previously enumerated `KnownResidual` as the degraded/accepted-fingerprint residual only and routed every empty observation to `Blocked`, so the shipped route was unrecorded.

**Section:** §Standard Contracts — Run report envelope
**Change:** The six declare-only families' landing clause now names BOTH routes to `verdict: null` / `state: "KnownResidual"` (the degraded read-back, or the auto-resolve residual on an empty post-green-preflight read-back) instead of attributing it solely to the degraded read-back; restated that a checks-bearing scenario and every call failure still land `Blocked`. The six-family list itself is unchanged.
**Why:** Duplicate occurrence of the wording the [Read-Back Dependency Posture] amendment retires — a single-site apply would have left arch naming one mechanism after a second shipped.

**Section:** §Occupied Resources — `contracts/pulse-load-envelope.toml`
**Change:** Recorded that `max_sustained_rate_spans_per_s` is computed `occurrences / gap_ms` — DISPATCHES per second, not wire spans — so a `Latency`/`Ramp` phase multiplies it by `samples`/`windows` (50 wire spans/s counted as 1/s at the shipped `latency-regression` shape, a 50× divergence, still ~200× under the bound); no verdict moves and the gate/caption shared basis is unaffected, but the term as named misdescribes what it bounds. Marked SURFACED-not-authored with the fix owned by a working-route entry.
**Why:** The chunk's re-shape measured the divergence and widened it 50×; recording it keeps a known gap from being silently re-discovered, while the fix (count `occurrences × samples`, or rename the term) stays owned rather than smuggled into a doc edit.
## 2026-08-21-severity-lifecycle-live-proof — the AutoResolved arm fired live, and the declare-only registry grew to seven

**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** `route_read_back`'s `AutoResolved` arm is no longer recorded as "unexercised live so far" — it
first fired LIVE on 2026-08-21 (leg E, `ack-cooldown`): a scenario forming no incident of its own leaves only
the preflight canary's, whose cues are `curious`, and the Tier-1 coordinator accepts Autonomous alone, so
nothing refreshed it; it auto-resolved on schedule and left ~4 minutes of empty active list before read-back
(`query_incident_list` → `result_count: 0`, envelope `fingerprints: []`).
**Why:** the 2026-08-20 leg recorded an honest limit — the canary's error-rate cues kept refreshing its
incident — and inferred the list could not empty. Measurement retires the inference while keeping the
observation: cues refresh an incident only through a digest of the SAME identity, which a `curious` cue never
triggers. The arm was exercised, not modified (zero production-source delta this chunk).

**Section:** §Standard Contracts — Run report envelope (per scenario check)
**Change:** the declare-only family registry reads SEVEN, adding the severity-lifecycle family
(`incident-auto-resolution` · `severity-tier-autonomous` · `severity-tier-suggested` · `severity-tier-curious`
· `ack-cooldown`, 2026-08-21).
**Why:** all five family TOMLs now carry zero `[[expected]]`, each retirement against a measured ground (the
det-L4 fixture pins one severity and no corpus tool renders a tier word; `query_incident_list` is active-only
so a resolved incident leaves the surface; `CountAtLeast` grades `span_refs` the incident producer writes
empty; no ack tool exists in the four-tool contract). All five live rows landed `verdict: null` /
`state: KnownResidual`.

## 2026-08-21-per-check-latency-measurement — runs.db is three tables
**Section:** Occupied Resources -> On-disk artifacts (runs.db)
**Change:** `runs.db` registered as THREE tables — `runs` (scenario grain, 11 columns, PK `(run_id, scenario)`), `run_envelope` (run grain), and NEW `run_check`, the per-check index keyed `(run_id, scenario, check_index)` with `latency_ms`/`deadline_ms` NOT NULL and `budget_ms` NULL on inherit; blocked/declare-only scenarios write zero rows.
**Why:** The chunk landed the `run_check` table (`db.rs:37`); the bullet said 'two tables', leaving the new persistence resource unregistered. The smoke leg read back `tables: ['runs','run_envelope','run_check']`.

## 2026-08-21-per-check-latency-measurement — ORM table roster + per-check-index label
**Section:** Established Decisions -> [ORM] None — raw SQL
**Change:** Table roster updated to three, and the 'per-check index' label moved off `runs` onto `run_check`, whose grain it now actually is.
**Why:** The same roster is restated here; a single-site apply would have left the two-table claim and the mis-attributed label alive.

## 2026-08-21-per-check-latency-measurement — CheckRecord registered as a second shared shape
**Section:** Standard Contracts -> Run report envelope
**Change:** Registered the per-check `CheckRecord` (9 keys) as a SECOND shared artifact shape beside the eleven-field envelope, riding the JSONL journal, the `run_check` table and the Markdown report's indented detail line; the envelope is byte-unchanged, and blocked/declare-only scenarios emit zero check records.
**Why:** Section Standard Contracts declares itself the registry of shared shapes and carried only the envelope; the chunk added a second one that three surfaces depend on.

## 2026-08-21-per-check-latency-measurement — budget_ms registered as declarable config
**Section:** Conventions -> Config conventions
**Change:** Registered the optional `[[expected]].budget_ms` key (integer ms, `#[serde(default)]`, garde `range(min = 1, max = MAX_BUDGET_MS)` with the ceiling DERIVED from `SloTier::Tier90s.deadline_ms()`), cross-checked against the scenario's own `slo_tier` at load by `Scenario::check_budgets()`; no committed TOML declares one.
**Why:** This paragraph enumerates the declarable scenario-config surface; the chunk's new external input appeared nowhere in it.

## 2026-08-21-per-check-latency-measurement — Nullability qualified per table
**Section:** Conventions -> Data model conventions (SQLite / runs.db)
**Change:** Nullability qualified PER TABLE: `runs.latency_ms` stays NULL-for-blocked, while `run_check.latency_ms`/`deadline_ms` are NOT NULL (an ungraded check emits no row at all) and `run_check.budget_ms` is NULL on inherit.
**Why:** The unqualified 'NULL for blocked rows' claim contradicted the new table's NOT NULL columns once Occupied Resources registered it.

## 2026-08-21-per-check-latency-measurement — Sibling-spanning invariants cannot be garde validators
**Section:** Established Decisions -> [Validation Library] serde 1.0.x + garde 0.22.1
**Change:** Recorded the THIRD route and its structural boundary: the one-altitude-up `custom` covers only invariants contained WITHIN the lifted field, because a field-level `custom` receives `(&field, &())` and sees no SIBLING; a sibling-spanning invariant ships as a load-path method invoked from `from_toml_str` (`check_capabilities`, now joined by `check_budgets`).
**Why:** The chunk measured the plan's `#[garde(custom)]`-on-`Scenario::expected` design structurally impossible; the decision enumerated only the Context pattern or one-altitude-up, neither of which the shipped rule uses.

## 2026-08-21-per-check-latency-measurement — Stack validation row narrowed
**Section:** Stack and Technologies -> Validation row
**Change:** Row narrowed to the garde-EXPRESSIBLE cross-field invariants, naming the load-path `Scenario::check_*()` route for sibling-spanning ones.
**Why:** The Stack table restated the retired claim that garde carries all cross-field scenario-config invariants, which the [Validation Library] amendment corrects.

## 2026-08-21-per-check-latency-measurement — Load-error mapping is three-way
**Section:** Established Decisions -> [Scenario Config Format] TOML on disk
**Change:** Load-error mapping corrected to THREE-way: `CoreError::Config` on a parse failure OR a non-garde load-path check failure, `CoreError::Validation` only on a garde failure (it is `#[from] garde::Report` and cannot carry a hand-written message).
**Why:** The chunk's budget fault raises `CoreError::Config`; the two-way mapping as written was contradicted by it.

## 2026-08-21-per-check-latency-measurement — Tier deadline recorded as a ceiling
**Section:** Established Decisions -> [Timing-Tolerance Model]
**Change:** The tier deadline recorded as a CEILING: a check may declare sub-tier `budget_ms` and is graded against a per-check effective deadline; `evaluate_slo` takes `deadline_ms`, `SloOutcome` carries it, and every check persists its own latency/deadline/verdict. Because the corpus is observed ONCE per scenario, per-check latencies are equal by construction and the DEADLINE is what separates their verdicts.
**Why:** The decision defined the deadline solely from the tier; acceptance (1) and (3) prove the sub-tier budget round-trips and separates two checks sharing one observation instant.

## 2026-08-21-delegated-timing-budgets-proven — Declare-only family count SEVEN→EIGHT
**Section:** Standard Contracts -> Run report envelope
**Change:** Count raised to EIGHT and the delegated-timing family registered (`halo-hue-encoding` · `service-constellation-discovery` · `report-render-surface`, 2026-08-21), each zero `[[expected]]` landing `KnownResidual` with its live claim graded at the harvest tier; `findings-counter-refresh` explicitly excluded because it carries one `[[expected]]`.
**Why:** The chunk live-proved the three as declare-only harvest-graded rows (all four legs exit 0, `[RESIDUAL]`), making them an eighth family by that sentence's own definition, and this is the only arch section enumerating them.

## 2026-08-21-delegated-timing-budgets-proven — Checks-bearing ⇒ Blocked narrowed to the auto-resolve route
**Section:** Established Decisions -> [Read-Back Dependency Posture]
**Change:** The confinement now binds the auto-resolve/empty-active-list route ALONE: a checks-bearing scenario is excluded from THAT route (an empty list leaves it `Blocked`) and every read-back call FAILURE still lands `Blocked` unconditionally — but the degraded-read-back route is NOT gated on declare-only, so a checks-bearing scenario reaches `KnownResidual` through it, an unmet `CountAtLeast` floor grading `CalibrationRegion` rather than failing.
**Why:** Measured false as written on leg D — `findings-counter-refresh` carries one `[[expected]]` and landed `verdict: CalibrationRegion` / `state: KnownResidual`, not `Blocked`.

## 2026-08-21-delegated-timing-budgets-proven — Duplicate Blocked claim narrowed in step
**Section:** Standard Contracts -> Run report envelope (closing sentence)
**Change:** Restated so the unconditional `Blocked` guarantee covers read-back call failures only, while a checks-bearing scenario is excluded from the auto-resolve route alone and can still reach `KnownResidual` via the degraded read-back, cross-referencing [Read-Back Dependency Posture].
**Why:** The same retired claim was restated here, in a paragraph spanning BOTH routes to `KnownResidual`, so a single-site apply would have left it standing as a global guarantee.

## 2026-08-22-operator-pause-and-checklist-live-firing — the `[[checklist]]` scenario-config key + the third load-path check
**Section:** Conventions → Config conventions · Established Decisions [Scenario Config Format] · Established Decisions [Validation Library]
**Change:** Registered `[[checklist]]` — an array-of-tables of `ChecklistItem { induced, observation }` (`#[serde(default)]`, garde `dive`, both halves `length(min = 1, max = MAX_CHECKLIST_TEXT)` with `MAX_CHECKLIST_TEXT = 200`), carried on `Scenario` and on `HoldPoint`, declared by exactly two committed scenarios (`halo-hue-encoding`, `halo-breathing-encoding`). Widened BOTH closed enumerations the new sibling-spanning rule joins: the [Scenario Config Format] `CoreError::Config` arm now reads `check_budgets`, `check_capabilities`, `check_checklist`, and [Validation Library]'s third-route list names `Scenario::check_checklist` beside the two it already carried. The `budget_ms`-scoped "No committed scenario TOML declares one" sentence was left standing — it remains true.
**Why:** The chunk shipped a new declarative scenario-config surface and a third load-path `check_*()`; arch's config-key registry and its two closed method enumerations had no entry, so a shipped surface was unregistered and two lists read as complete while being false.

## 2026-08-31-p-075-assert-round — Payload fidelity disproved on a second axis; runtime-STATE fidelity recorded as the stronger claim that DOES exist
**Section:** Established Decisions [Read-Back Dependency Posture] → the Honest-limit sentence
**Change:** Retired "under deterministic L4 no stronger claim exists" and the "concurrent unrelated incident inside the poll window" caveat. Added: the SECOND payload axis (`incident_events` has ZERO references in `crates/mcp-server`, written by triage, read only corpus-side — reaches no MCP tool at any width, Pulse HEAD `83d4060`); runtime-STATE fidelity as the stronger claim, live-proven (incident 6 resolved, active set emptied, `idle_seconds_at_resolve = 0.0` against a 120s idle threshold, corpus-recorded 45s life); the one-active-incident dedupe constraint (the producer dedupes against any OPEN incident regardless of fingerprint), which both forecloses a spared-control design and makes the concurrent-incident caveat unreachable; and the DECLINED arm's permanent stub-only status (monotonic-timestamp guard, `crates/corpus/src/contract.rs:652-659`).
**Why:** The chunk measured all four facts first-hand against a live Pulse at HEAD `83d4060`; the report's Cross-project claims, Reverted/negative API facts and Outcome carry the evidence, and the plan named this section first in its Expected amendments.

## 2026-08-31-p-075-assert-round — Payload-fidelity carrier restated on both axes at the twin site
**Section:** Standard Contracts → Readiness gate (the "carrier is freshness, not payload identity" sentence)
**Change:** Extended the carrier sentence to name BOTH measured axes (`span_events` unread + `incident_events` unreachable) and to record that runtime-STATE fidelity is attainable where payload identity is not.
**Why:** Duplicate occurrence of the single-axis claim retired in [Read-Back Dependency Posture]; the duplicate-occurrence precedent requires both move together or the retired reading survives in the twin.

## 2026-08-31-p-075-assert-round — Workspace-key divergence mechanism is now the published-key FALLBACK
**Section:** Standard Contracts → Readiness gate (app/sidecar workspace-key precondition)
**Change:** Replaced "the sidecar keys its query on `ANDROMEDA_PULSE_DATA_DIR` while `pulse-app` keys incidents on its detected workspace root" with the measured mechanism: `pulse-app` PUBLISHES its key to `{data_dir}/run/workspace-key` and the sidecar reads it (`read_published_workspace_key`), falling back to `data_dir` only when that file is absent or invalid. Measured 2026-09-01: the published key matched the incidents' stamped `workspace` byte-for-byte and no divergence occurred.
**Why:** SUT-side mechanism change read first-hand at HEAD `83d4060`; the divergence is now the fallback case rather than the default, which changes how an operator should diagnose a zero-row read-back.

## 2026-08-31-p-075-assert-round — `workspace` column = data_dir qualified (third occurrence)
**Section:** Occupied Resources → Environment variables, `ANDROMEDA_PULSE_DATA_DIR`
**Change:** Qualified the `= data_dir` equality: the filter value is the published workspace key when present and `data_dir` only as fallback. The propagation obligation and the empty-`query_incident_list` failure mode stand unchanged.
**Why:** Third restatement of the retired sidecar-keys-on-data_dir claim; without moving it the corrected mechanism would survive at only two of three sites.

## 2026-09-01-webview-self-verify-windows-host — the webview drive path, and the CARRY's tuple re-scope
**Section:** Established Decisions [Read-Back Dependency Posture] · Occupied Resources (Ports · Frontend asset subtree · `logs/conductor-tauri.jsonl` · Environment variables) · Infrastructure Patterns (Build system) · Cross-cutting Patterns (Trust boundary · Scope law)
**Change:** (1) **The CARRY** — "At most ONE incident is active per workspace" re-scoped to "per DEDUPE TUPLE", citing the `(kind, scope, scope_id)` predicate as measured at `andromeda-pulse pulse-app/src/inference_runtime.rs:811` (HEAD `83d4060`); "spared-control unattainable" and the freshness-caveat retirement both narrowed to WITHIN-tuple, with a cross-scope two-incident control recorded as a weighable route option, never a retirement. The originating leg's evidence stands — only the generalisation was unlicensed. (2) Registered `CONDUCTOR_MSEDGEDRIVER` (harness-only driver handle, skip-at-exit-0 guard) and the dev-only harness-lifetime ports `4444`/`4445`. (3) Recorded that the `custom-protocol` FEATURE — not the profile — decides bundle embedding (`tauri` 2.11.3 `build.rs`: `let dev = !custom_protocol`, read back via `DEP_TAURI_DEV`), as measured. (4) Trust boundary's "only case where Conductor opens a port" scoped to SHIPPED binaries. (5) Scope law's "no UI automation" scoped to PULSE's UI. (6) `conductor-tauri.jsonl`'s CWD-relative landing site + `ui/logs/` added to the ignored enumeration.
**Why:** The chunk drove the real Tauri window for the first time, which measured three arch claims incomplete or over-scoped and landed two genuinely new resources. The CARRY was owed from `2026-08-31-p-075-assert-round` and applied here after a fresh re-sweep, which found a FIFTH site the CARRY's four-site list never named (`crates/conductor-run/tests/lifecycle_live.rs:128`, corrected in-code). The scope-law and port-ban qualifications were escalated and operator-ratified at this wrap.

## 2026-09-01-desktop-a11y-sweep — webview a11y leg is two arms; self-obs landing site moved; sidecar PATH recorded
**Section:** §Occupied Resources — Ports · Service/process names · Frontend asset subtree · On-disk artifacts (`conductor-tauri.jsonl`) · Environment variables (`CONDUCTOR_MSEDGEDRIVER`) · §Cross-cutting Patterns — Trust boundary · Scope law
**Change:** (1) the `4444`/`4445` driver stack, the trust-boundary spawn note, the scope-law self-verify clause and the `CONDUCTOR_MSEDGEDRIVER` handle now name BOTH arms — the unattended `--e2e` routine arm and the operator-local `npm run a11y:driven` driven arm — over ONE WebdriverIO + tauri-driver stack. (2) `logs/conductor-tauri.jsonl` lands at the WORKSPACE ROOT under the tauri-driver `cwd: repoRoot` spawn, covered by the root-anchored ignore rule; `crates/conductor-tauri/ui/logs/` is the retired pre-2026-09-01 site. (3) the sidecar's fixed-NAME-through-`PATH` resolution is recorded, with `PATH` named as a spawn-resolution input.
**Why:** (1) the chunk added a second suite to the same `wdio.conf.ts`; a single-arm attribution would leave the driven arm's identical listeners unregistered. (2) measured this wrap — root `logs/conductor-tauri.jsonl` 27163 B at 00:15 (post-cwd run) against `ui/logs/` 3188 B at 23:21 (pre-cwd). (3) measured 2026-09-01: with the sidecar off `PATH`, preflight returns BLOCKED in ~2ms with all four tools `absent`, at row level indistinguishable from a genuine SUT-side gate failure.

## 2026-09-01-live-per-p-id-verdict-lamps — self-obs sink landing site is per-arm
**Section:** Occupied Resources -> On-disk artifacts (`logs/conductor-tauri.jsonl`) + Occupied Resources -> Frontend asset subtree
**Change:** The Tauri backend's self-obs stream no longer has ONE landing site. `tauri_log_path()` resolves it as `runs_dir.parent()/logs`, so the routine `--e2e` arm's new `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` moves it to `runs/logs/conductor-tauri.jsonl`, while the operator-local `a11y:driven` arm leaves the handle unset and still lands at the workspace root. Both sites git-ignored; the frontend-subtree parenthetical restating the single-site verdict was qualified to match.
**Why:** The chunk gave the routine arm a seeded fixture runs dir, and the sink follows the handle. Measured 2026-09-02, not derived: `runs/logs/conductor-tauri.jsonl` (4741 B) carries the leg's own instant, matching `runs/e2e-fixture/runs.db`, while root `logs/conductor-tauri.jsonl` stayed stale. Surfaced by the arch drift detector; the orchestrator verified it on disk before applying.

## 2026-09-02-screen-reader-manual-spec — `CONDUCTOR_NVDA`, three suite families over the one stack, the per-suite self-obs landing site
**Section:** Occupied Resources -> Ports (`4444`/`4445`) + On-disk artifacts (`logs/conductor-tauri.jsonl`) + Environment variables (`CONDUCTOR_MSEDGEDRIVER`, new `CONDUCTOR_NVDA`) + Cross-cutting Patterns -> Trust boundary + Scope law
**Change:** (1) Registered `CONDUCTOR_NVDA` -- the second host dev-tool handle, read only by `wdio.conf.ts`, validated and spawned like `CONDUCTOR_MSEDGEDRIVER`, skip-at-exit-0 when unset, value never committed -- and retired the "one HOST dev-tool handle" singular. (2) The webview a11y legs are THREE suite families (four `wdio` suites) over the ONE stack: the `sr*` screen-reader leg joins the routine and driven arms in the Ports bullet, the `CONDUCTOR_MSEDGEDRIVER` bullet, the trust boundary (NVDA + a fixed-argv PowerShell activation script as leg-spawned children, no listener) and the scope law; "never a second automation stack" unchanged. (3) The self-obs landing site is per-SUITE -- `runs/logs/` (routine + `sr-empty`), `runs/driven/logs/` (driven), `runs/sr-leg/logs/` (`sr` / `sr-error`), each measured 2026-09-02; the unset-handle root site stays the mechanism, retired only as an a11y landing site.
**Why:** (1) the chunk's one new resource. (2) the report's count move (suite families 2 -> 3), restated at five arch sites. (3) the chunk moved the driven arm's `CONDUCTOR_RUNS_DIR` to `runs/driven/runs`, and the wrap measured the driven landing site itself (11 457 B at 11:55:29Z) beside the leg's `runs/sr-leg/logs/` (8 312 B) -- the previous entry's "the driven arm leaves the handle unset" is retired by construction, not by a mechanism change.

## 2026-09-02-cross-surface-envelope-parity — `CONDUCTOR_E2E_SEED_DIR` registered as the third handle class
**Section:** Occupied Resources — Environment variables
**Change:** Added an eleventh bullet to the reserved `CONDUCTOR_*` enumeration, after `CONDUCTOR_NVDA`:
`CONDUCTOR_E2E_SEED_DIR`, the repo-relative fixture runs dir the webview `--e2e` arm seeds into — SET by
`wdio.conf.ts` `onPrepare`, READ only by `crates/conductor-run/tests/envelope_fixture.rs`, never by a shipped
binary, a no-op when unset. Recorded as the THIRD handle class: unlike `CONDUCTOR_MSEDGEDRIVER` /
`CONDUCTOR_NVDA` it names a path this document defines rather than a HOST dev-tool, so the existing "One of
TWO handles … naming a HOST dev-tool" clause stays true and was left standing.
**Why:** the chunk's only new resource. The report's Symbols bullet declares the handle and states it was
absent from this enumeration; D-arch-resources proposed the row. Operator-ratified at the P2 escalation
(2026-09-02) as a genuinely new class rather than an instance of playbook rule 115, whose preconditions it
fails on four of five clauses.

## 2026-09-03-live-pulse-preconditions-probed — preconditions probe registered; run contract at six terms

**Section:** §Stack (CLI argument parsing) · §Standard Contracts (Liveness equivalent) · §Occupied Resources (Environment variables ×3, `contracts/pulse-run-contract.toml`)
**Change:** The `:4317` liveness check is recorded with TWO production callers and deliberately different dispositions — the timeline engine's pre-emission check still surfaces a refusal as `Result::Err`, while the scheduling-time `observe_preconditions` probe reports it as an unmet precondition and a non-zero exit, never an `Err` and never a verdict. `CONDUCTOR_PREFLIGHT_TIMEOUT` records that `boot` now runs `conductor preconditions` as a LEADING arm and short-circuits, so the preflight invocation and its timeout are SKIPPED rather than paid. The run contract's `[[term]]` list is recorded at SIX (gaining `mcp-enabled`, `shell-declaration`, `env = ANDROMEDA_PULSE_MCP_ENABLED`), making the observable set TWO handles; `ANDROMEDA_PULSE_L4_DETERMINISTIC` loses its exclusive article and `ANDROMEDA_PULSE_MCP_ENABLED` gains its read role. `sidecar-built`'s `asserted` rationale is recorded as measurably FALSE (as measured at `.claude/rules/verification-harness.md:54`, 2026-08-20) — the `warmup_ms` failure shape — with the deliberate non-re-classification noted. The §Stack clap row's three-verb literal is de-literalized to the set `Commands` declares.
**Why:** 2026-09-03-live-pulse-preconditions-probed landed a non-mutating precondition probe and `boot`'s leading arm. **NOT applied:** registering the CLI VERB itself in arch — playbook `:61` dismisses that as over-reach (no port/socket/endpoint/IPC/event/env-var/crate was added; the verb's home is layout-templates §cli Primary screens, where it landed).
## 2026-09-04-sr-findings-remediation — the precondition probe cannot exit 0 (path handle graded by a boolean)
**Section:** §Standard Contracts — Liveness equivalent (primary) · §Occupied Resources — `CONDUCTOR_PREFLIGHT_TIMEOUT` · §Occupied Resources — `ANDROMEDA_PULSE_MCP_ENABLED`
**Change:** All three sections now record that `observe_preconditions` passes the three `ANDROMEDA_PULSE_*` names through `declares()`, which accepts only `"true"`/`"1"` — so the PATH-valued `ANDROMEDA_PULSE_DATA_DIR` can never declare, `conductor preconditions` cannot exit 0 under any environment, and `agent-run boot`'s leading arm has short-circuited before every preflight since `480bc66`. The Liveness-equivalent passage no longer implies a non-zero exit means a refusal; the `CONDUCTOR_PREFLIGHT_TIMEOUT` skip is restated as unconditional rather than contingent; the `ANDROMEDA_PULSE_MCP_ENABLED` bullet separates the probe's name-only REPORTING (unchanged, the redaction property holds) from its value-gated CHECK. Each passage carries the evidence pointer and names the fix as route-owned, not shipped.
**Why:** Report §Spec claims disproved by measurement #1 (measured 2026-09-04T06:41Z under the full operator env: `egress-reachable` and `sidecar-resolvable` satisfied, `handles-declared` unmet naming DATA_DIR alone, exit 1; a child process independently confirmed to receive the value intact). Applied as a RECORD under playbook.md:118 — the operator's wrap directive names this defect (site `lib.rs:359`, mechanism, symptom) and mints the corrective route entry. **Search performed:** grep of architecture.md for `preconditions`, `observe_preconditions`, `declares`, `handles-declared`, `boot`, `ANDROMEDA_PULSE_` and for the mechanism's phrasings ("short-circuit", "unmet precondition", "presence-only") — three sites carried the claim; all three amended.

## 2026-09-04-preconditions-probe-reads-path-handles-by-presence — the probe that can now say yes
**Section:** §Standard Contracts — Liveness equivalent (primary) · §Occupied Resources — `CONDUCTOR_PREFLIGHT_TIMEOUT` · §Occupied Resources — `ANDROMEDA_PULSE_MCP_ENABLED`
**Change:** Retires the entry directly above. `observe_preconditions` now builds `declared` PER HANDLE through `conductor_core::handle_declared` — the PATH-valued `ANDROMEDA_PULSE_DATA_DIR` graded by presence-after-trim, every other name delegating to the value-only `conductor_core::flag_declared` — so all three subjects are meetable and a non-zero exit names a genuinely unmet subject again. `CONDUCTOR_PREFLIGHT_TIMEOUT`: the skip is restated as CONDITIONAL, and the timeout is reached and paid (~47s per leg = warm-up 45s + poll). `ANDROMEDA_PULSE_MCP_ENABLED`: the CHECK is per-handle by kind, and `=false` now leaves `handles-declared` naming that handle alone. The name-only REPORTING / redaction property is unchanged throughout.
**Why:** The fix this document had named route-owned shipped in this chunk. Measured at `conductor-0.2.0/chunks/2026-09-04-preconditions-probe-reads-path-handles-by-presence/report.md`: `conductor preconditions` exit 0 printing `[PRECONDITION] every live-Pulse precondition is satisfied` (2026-09-04T17:07Z, full operator env), then `ReadyState` JSON with `ready: true` / `canary_round_trip: "ok"` / `blocked_precondition: null` / 4-of-4 tools / `data_dir: "<redacted>"` in BOTH shipped shells (`agent-run.sh` 17:07:17→17:08:04Z · `agent-run.ps1` 17:11:16→17:12:03Z), zero `skipped preflight` lines in either, neither script edited. Supersedes the 2026-09-04T06:41Z `sr-findings-remediation` reading, which measured the pre-fix state. **Search performed:** grep of architecture.md for `UNSATISFIABLE`, `UNCONDITIONAL`, `short-circuited before every preflight`, `can ever be met`, `permanently unmet`, `route-owned`, plus `declares` / `boot` / `preconditions` read for the claim however worded — the same three sites (`:114`, `:184`, `:188`) carried it; four other hits (`Commands::declares`, scenario `declare`, the gate's own precondition strings, `bootstrap`/`BootstrapState`) are unrelated. All three amended; post-edit re-grep returns 0 for every retired phrase.

## 2026-09-04-sidecar-spawn-without-a-console-window — the spawn stops naming a type it no longer uses, and stops promising a build it cannot do
**Section:** §Occupied Resources — Service/process names (primary) · §Cross-cutting Patterns — Trust boundary · §Infrastructure Patterns — Directory structure · §Established Decisions [Module Boundaries]
**Change:** (1) The sidecar launch mechanism is corrected from the retired rmcp `TokioChildProcess` to what actually spawns it — `conductor-verify/src/spawn.rs::build_command`'s `tokio::process::Command`, handed to `ReadbackClient::connect_command` over piped stdio — and the same passage records the `CREATE_NO_WINDOW` creation flag the spawn now carries under `#[cfg(windows)]`. (2) §Trust boundary's restatement of that retired name is corrected identically. (3) The workspace directory tree registers the new root `rustfmt.toml` (one key, `edition = "2024"`). (4) [Module Boundaries] keeps its compiler-enforced forbidden-edge property unqualified but QUALIFIES the standalone per-seam BUILD claim, which measurement falsified.
**Why:** The chunk shipped the console-suppression fix and corrected the last in-code `TokioChildProcess` mention, leaving this document naming a type absent from every crate `src/` since 2026-06-27. Measured at `conductor-0.2.0/chunks/2026-09-04-sidecar-spawn-without-a-console-window/evidence/nvda-pass.json`: SR row S1-01's `heard` lost `<host-path>`, `<host-path> terminal blank` and `pane`, with 0 `security_finding` rows and 0 `<host-path>` placeholders where the prior record carried both. The [Module Boundaries] qualifier rests on a SEPARATE measurement — `cargo check -p conductor-verify --lib` red at HEAD on `tokio::time::sleep` (`preflight.rs:336`) with tokio's `time` feature in `[dev-dependencies]` (`Cargo.toml:31`) and not `[dependencies]` (`:10`) — which is PRE-EXISTING and route-owned (*Dependency polish*), not this chunk's defect; escalated as E1 and resolved by the operator on 2026-09-04 in favour of qualifying the body now while the CARRY owns the fix. **Search performed:** grep of all seven masters for `TokioChildProcess` — 5 sites across 3 docs (architecture ×2 amended here, security-plan ×2 amended in its own pass, test-plan ×1 at `:597` LEFT because it sits in §12 Test Decisions Log already carrying an inline dated `[2026-09-02 correction: rmcp was removed 2026-06-27 …]`; a decisions-log record is annotated, never rewritten) and 0 remaining in any crate `src/`. The one `TokioChildProcess` token still in this document is the historical note this amendment authored ("were removed 2026-06-27"), not a live claim. **Deliberately NOT amended:** the `tokio 1.48.x` version claim at §Stack / §Established Decisions [Language / Runtime] / §Inherited Defaults (3 sites), which a detector proposed moving to the resolved 1.52.3 — dismissed under playbook `:31` as not this chunk's drift, since the report's Dependencies bullet states "none added, none bumped" and `git diff HEAD` on `Cargo.toml`/`Cargo.lock` is empty. The staleness is real and pre-existing; it is recorded in this run's `fanout-results.md` so it is not lost.

## 2026-09-05-audit-corrective — the supply-chain red was local, not an external advisory-DB fault
**Section:** Infrastructure Patterns — Build system
**Change:** Retired "`cargo audit` is separately red on an external advisory-DB fault"; the paragraph now records BOTH runners green over the un-drifted lock (`cargo deny` exit 0, `cargo audit` exit 0 — 1239 advisories, 564 packages, 18 `deny.toml`-adjudicated allowed warnings, measured 2026-09-05 against advisory-db HEAD `5a0ebedf` with `git status --porcelain` empty) and names the real cause: an untracked `crates/gettext-sys/RUSTSEC-2026-0244.md` left in this host's clone after upstream moved the file to `crates/gettext-rs/` on 2026-08-09 (`e12b689b`), which a fetch into an existing copy never removes. The porcelain check now precedes classifying any parse failure as external.
**Why:** the chunk MEASURED the claim false (report §Spec claims disproved 2 + §Cross-project claims; `evidence/audit-probe.md`), and the chunk's own probe is what closes the standing deferral. Swept `architecture.md` for other statements of the retired verdict — none; the `toml`-crate "audit/deny-clean" lines at :31/:51 assert a per-crate pass, not the gate state.

## 2026-09-06-operator-gated-live-suite — the deterministic-L4 degraded universal retired, and the live-suite capture site registered
**Section:** §Established Decisions [Read-Back Dependency Posture] · §Occupied Resources — On-disk artifacts / database · §Infrastructure Patterns — Directory structure
**Change:** Three edits, all applied. (1) [Read-Back Dependency Posture]: the universal "under deterministic L4 every read-back returns `degraded_mode`" is RETIRED — `degraded` now reads as a PER-READ-BACK property, with deterministic L4 measured BOTH ways (`findings-counter-refresh` degraded, 2026-08-21; `degraded-mode-report` non-degraded → the graded route → `ManualCheck`, `latency_ms 6045`, run `2026-09-06T09-11-09-325`). The degraded route to `KnownResidual`, its ungated-on-declare-only property, and `findings-counter-refresh`'s own outcome are all unchanged — only the quantifier was withdrawn. (2) §Occupied Resources gains `runs/live-suite/{leg}.jsonl`, the `run --live` per-leg self-obs captures: harness-owned by `scripts/agent-run.{sh,ps1}`, deliberately no `CONDUCTOR_*` handle of its own, moves with `CONDUCTOR_RUNS_DIR`, git-ignored, LEG-stemmed, cleared per invocation by a non-recursive `rm -f …/*.jsonl`. (3) The directory-tree gloss for `runs/` was narrowed — "run_id-stemmed, never overwritten" now scopes to the journal + report, with the `live-suite/` child shown as the one exception.
**Why:** `2026-09-06-operator-gated-live-suite`. (1) disposes the chunk's `Spec claims disproved by measurement` entry: `state_for` (`conductor-run/src/execute.rs:233-235`) returns `KnownResidual` iff `observation.degraded`, so leg B1's `ManualCheck` row proves a non-degraded read-back under deterministic L4 — evidence at that chunk's `evidence/b1.jsonl` + `evidence/live-suite-verdict.md`. (2) registers a permanent artifact site the chunk's harness now writes (report §Harness / gate surface); (3) is (2)'s duplicate-claim fix — the tree comment asserted of ALL of `runs/` what holds only for the journal + report, and would have survived a single-site apply. **Sweep:** `every read-back returns` / `every read-back is degraded` / `all read-backs.*degraded` across all seven masters, each hit then read under a live-claim-vs-retirement-marker control — the retired wording survives only inside its own retirement marker; three other `every read-back …` hits (architecture's read-back-failure rule, security-plan ×2 on the unreachable path) state DIFFERENT claims and were left standing. `degraded_mode_response` → 0 hits in architecture.md. Not swept: whether any leaf outside the seven masters and the enumerated distillations restates the universal.

## 2026-09-06 0-pending adaptation (subject: `2026-09-06-operator-gated-live-suite`) — the AutoResolved arm's firing condition is uptime-bound
**Section:** §Established Decisions [Read-Back Dependency Posture] (the leg-E first-fired-LIVE narrative)
**Change:** The leg-E narrative is QUALIFIED, not retired. "Nothing refreshed it" is recorded as holding only while Pulse is inside the emitting service's ONE-HOUR bootstrap window: past it the silence evaluator raises `service_went_silent` cues for exactly the quiet the arm depends on, those cues reach the Autonomous band and create NEW incidents during the silent phase, so the active set is never empty and the arm is unreachable at ANY window length. Inside the window the arm is reachable but still bounded by 120s idle + up to a full 30s observer tick measured from the LAST preflight incident, which can be more than one.
**Why:** A 0-pending adaptation applying a fact this session itself measured (SKILL step 6's self-produced-fact door), not drift-derived. As measured at `pulse-legs/a11y-20260906-110201/logs/agent-latest.jsonl.2026-09-06`: the day's first EMITTED silence cue fired 10:11:09.723Z — one hour after the canary service's first span (~09:11Z) — and incidents then formed mid-silence at 11:00:33 and 11:02:02 after cues of magnitude 3.0 → 5.0 → 7.03; run `2026-09-06T10-58-18-536` landed `ManualCheck` / `latency_ms 200048` / 6 fingerprints. SUT coordinates re-verified at HEAD `83d4060`: `BOOTSTRAP_WINDOW_SECONDS = 3_600` (`crates/triage/src/baseline/activity_floor.rs:36`), the evaluator called each emit cycle (`crates/triage/src/cue/emitter.rs:186`), its post-bootstrap test at `:1104`. This matters here because the SAME section retired the deterministic-L4 degraded universal hours earlier — the two facts sit in one passage and would otherwise read as an unqualified account of when the arm fires. **Search performed:** `nothing refreshed it` / `auto-resolve on schedule` / `empty active list` across all seven masters → 2 hits, both architecture.md. `:69` is the amended passage; `:134` was READ rather than pattern-matched and is NOT a duplicate — it DEFINES the route's trigger ("the auto-resolve residual when a post-green-preflight read-back finds an empty active list"), which remains true, and already cross-references this section for detail. `AutoResolved` / `empty active` across `.claude/` + CLAUDE.md → 1 hit, `session-handoff.md`, rewritten at P6 by this same run. **Deliberately NOT edited:** `scenarios/auto-resolve-idle-window.toml:29-32`, whose header still states the superseded margin model — it is CODE, outside this path's scope, and is owned by the CARRY pinned to *Halo hue budget re-driven*.

## 2026-09-06-run-report-envelope-conformance-gate — the storage seam's first row-removing path
**Section:** Established Decisions — [ORM] None — raw SQL
**Change:** The seam's workload description widened from "append + a handful of cross-run SELECTs" to include one run-scoped teardown: `RunsDb::delete_run`, three LITERAL `DELETE ... WHERE run_id = ?1` statements over `runs` / `run_check` / `run_envelope` in one transaction, rusqlite bound parameters, idempotent, never `format!`-assembled even over a hard-coded table list. The `conductor cleanup <run_id>` verb is its sole caller and the harness shells issue no SQL.
**Why:** The entry's characterisation of the workload IS the payoff argument for keeping raw SQL, and a three-table transactional delete is precisely the shape that would otherwise argue for an ORM — so the entry under-described what ships. Proposed by D-arch-decisions; routine under playbook :28 (spec illustration reconciled to the sound shipped impl, invariant intact). D-arch-resources correctly did NOT propose registering the new CLI verb: architecture:33 states the verb set is "whichever `Commands` (`cli.rs`) declares, never a literal list here", and playbook :61 dismisses that class.

## 2026-09-06-coverage-completeness-gate — rate term counts wire records; `.gitattributes` registered

**Section:** §Occupied Resources → On-disk artifacts → `contracts/pulse-load-envelope.toml` · §Infrastructure Patterns → Directory structure + Build system
**Change:** (1) Retired the DISPATCHES reading of `max_sustained_rate_spans_per_s`. The body now states the shipped basis — `phase_rate_exceeds` judges `occurrences × EmissionSpec::max_spans_per_dispatch() × 1000 > max_rate × gap_ms`, an upper BOUND (the rate curves carry seeded jitter and the static gate has no seed), with the per-dispatch count measured per `EmissionShape` arm. Records that the retired `samples`/`windows` reading held for `Latency` alone, SUPERSEDES the "~200× under the bound" figure with the measured ~232 records/s at `halo-breathing-encoding` (≈43×), notes the contract file and the term's name/value are unchanged, retires the SURFACED-not-authored disposition as authored by this chunk, and preserves the gate/caption shared-basis property — now stated as holding BY CONSTRUCTION via the single `phase_rate_exceeds` → `phase_breach` → {`check_load_envelope`, `classify`} call chain. (2) Registered `.gitattributes` in the directory tree beside `rustfmt.toml`, plus a Build-system sentence for the gate-correctness dependency.
**Why:** The chunk shipped the fix this record itself scoped as route-owned-not-shipped (playbook: a master naming its own defect as route-owned, discharged by the chunk). Report Changes → Counts/qualifiers moved (1)(2) and Spec claims disproved (1)-(4), all four naming `architecture.md:174`. Both amendments applied at ONE site each: the report's sweep (`grep -nE 'occurrences / gap_ms|samples./.windows|DISPATCHES|200×|50/s' .andromeda/*.md`) returned `:174` only, and `grep -rn gitattributes .andromeda/*.md .claude/` returned 0 before this entry. The `.gitattributes` proposal arrived aimed at §Occupied Resources and was RE-HOMED by the orchestrator: that section holds runtime artifacts, while this file's siblings (`Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`) live in the directory tree.

## 2026-09-06-halo-hue-budget-re-driven — ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS registered
**Section:** Occupied Resources — Environment variables
**Change:** The Pulse-side per-service cold-start window handle is registered — the one entry Conductor neither SETS nor READS. It is registered because a SHIPPED artifact names it in its own output (the `run --live` banner in both shells), so a reader meeting it there can find it here. Recorded as a BOOT-TIME posture (`Thresholds::from_env()` at `pulse-app/src/main.rs:472`, measured 2026-09-07 at HEAD 83d4060), hence boot-wide rather than per-leg; load-bearing for the auto-resolve leg and inert for the storm-path legs; and explicitly neither a run-contract `shell-declaration` term nor one of the three `conductor preconditions` subjects.
**Why:** The chunk's `run --live` banner names the handle in both shells while arch's env registry carried only handles Conductor reads. ESCALATED at P2 — no playbook rule matched and the unease was precedent-shaped (a registry of read handles gaining a never-read one). The operator approved the pair and directed a bounding playbook rule, minted the same pass: only a handle a SHIPPED artifact names is registered, never one mentioned solely in a report or plan.

## 2026-09-06-halo-hue-budget-re-driven — The auto-resolve arm's uptime bound is the window in force at boot, not a fixed hour
**Section:** Established Decisions — [Read-Back Dependency Posture]
**Change:** The UPTIME-BOUND paragraph is qualified: `BOOTSTRAP_WINDOW_SECONDS = 3_600` is the DEFAULT, overridable once at Pulse's boot, so the arm's reachability is bounded by the window IN FORCE AT BOOT. The past-the-window mechanism is unchanged and now explicitly scoped to the default window. NARROWED at the light gate, before commit: the stretched posture removes cause (a) and is NOT sufficient — cause (b) is untouched by it and still decides the leg, so the body says the posture makes the arm REACHABLE rather than RELIABLE and records both 2026-09-07 runs.
**Why:** Dependent of the registry entry above: registering the handle alone would leave this paragraph telling a reader the arm must race a fixed hour, which the 2026-09-07 leg measured false. Same claim, second site — the cross-master pair with test-plan section 9, which this pass conditioned identically.

## 2026-09-07-dependency-polish — Stack versions reconciled to the resolved lock
**Section:** §Stack and Technologies · §Established Decisions [Language / Runtime] · §Established Decisions [Deployment] · §Infrastructure Patterns — Deployment model · §Inherited Defaults
**Change:** `indicatif 0.17` → `0.18` (this chunk's bump, resolves 0.18.6); `tokio 1.48.x` → `1.52.3` at all three sites; Tauri `bundler v2.10.x, latest 2.10.1` → `bundler 2.11.3` at all four sites. `inquire 0.9` on the same Stack row was already correct and was NOT touched.
**Why:** The indicatif value is this chunk's own dependency change (report Changes → Dependencies). The tokio and Tauri values were pre-existing stale literals the chunk's folded CARRY routes here ("reconcile the docs to the resolved artifacts here, where the dependency surface is already open"); every `≥ 2.10.3` FLOOR statement was deliberately left standing, since a floor is satisfied — not falsified — at 2.11.3.

## 2026-09-07-dependency-polish — OTLP emission row records the default-features trim
**Section:** §Stack and Technologies
**Change:** The opentelemetry-proto row now reads `default-features = false` at the workspace entry with `gen-tonic` + `trace`/`logs`; `metrics` is de-registered.
**Why:** The trim shipped at the workspace entry because cargo rejects a member disabling defaults on an inherited dep (report Deviations #1). `metrics` was previously enabled through `conductor-run`'s featureless dev-dep riding `default = [full]`; with defaults off at the workspace entry no member enables it, so the registry now matches the code. The report surfaced this mismatch by name rather than absorbing it.

## 2026-09-07-dependency-polish — [Module Boundaries] E1 qualifier retired on a clean nine-member sweep
**Section:** §Established Decisions [Module Boundaries]
**Change:** The standalone per-seam BUILD claim is no longer qualified as "an intent, not a guarantee". The `conductor-verify` manifest repair landed (tokio's `time` into `[dependencies]`) and the whole roster swept clean — nine members each on its OWN targets (`--lib` ×7, `--bins` ×2 for the two crates with no lib target), every one exit 0, measured twice. `--all-targets` is recorded as banned in the sweep because it re-unifies dev-dependencies. A narrower caveat survives: no CI job builds a member standalone, so the property is measured-at-a-chunk, not gate-enforced.
**Why:** Report Changes → Spec claims disproved by measurement #2. The retired text named *Dependency polish* as the owner of the repair, so leaving it would send a future planner to redo landed work. Retirement made unconditional by operator wrap directive item 4 (the sweep was measured twice — builder and operator).

## 2026-09-07-dependency-polish — Supply-chain measurement re-stated post-bump
**Section:** §Infrastructure Patterns — Build system
**Change:** `cargo audit` exit 0 re-measured: 562 packages scanned (was 564), 17 allowed warnings (was 18) = 16 `unmaintained` + 1 `unsound`; the note records that RUSTSEC-2025-0119 left the ignore list when `number_prefix` left the tree, taking `deny.toml`'s ignore entries 17 → 16. `1239 advisories loaded` is unchanged and was re-measured, not carried.
**Why:** Report Changes → Counts / qualifiers moved (lock 564 → 562; audit warnings 18 → 17) and Schema / config (the ignore entry's subject left the tree).

## 2026-09-07-a11y-ci-gate — CI gains a Pulse-free a11y gate; a fourth env-handle class; the a11y violation artifact registered
**Section:** §Stack and Technologies (CI/CD row) · §Established Decisions [CI/CD] · §Occupied Resources — Environment variables (`CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, new `CONDUCTOR_A11Y_STRICT`) · §Occupied Resources — On-disk artifacts · §Infrastructure Patterns — Build system / CI-CD approach / Directory structure · §Inherited Defaults
**Change:** [CI/CD] restated — CI is no longer build+test only: a third job (`a11y`, `runs-on: windows-2025`) runs the Pulse-free routine webview a11y leg plus the reused `journal_conformance` gate and the artifact upload; the exclusion narrows to dynamic proof REQUIRING A LIVE PULSE, which is why the routine arm is gateable at all. Five consequential restatements reconciled in lockstep (Stack row · CI/CD approach · directory-tree comment · Inherited Defaults · the Build-system job enumeration, whose "both CI jobs run `windows-latest`" became the three-job Windows set with `a11y` on the explicit `windows-2025` label). Registered `CONDUCTOR_A11Y_STRICT` as a FOURTH handle class — flag-valued, read by `wdio.conf.ts` AND both harness shells, never by a shipped binary — and qualified the `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` skip-at-exit-0 clauses as the LAX arm, since both skip sites now route through one `exitUnresolvedHandle()`. Registered `runs/a11y/<run_id>.jsonl` with its measured 13-key (15 under CI) shape and the reasons for its own directory.
**Why:** Report Changes → Harness/gate surface (the new job), Symbols/APIs → Env vars (the new handle + the two replaced `process.exit(0)` sites), Schema/config (the artifact), and Outcome (the criteria measured both ways). The [CI/CD] widening was ESCALATED at this wrap — rule `:97` (locked-decision reversal) fails its causal clause, since no live SUT contradicted anything and the route delivered planned work that test-plan `:469` and a11y-plan `:457` already named this entry as owning — and the operator ratified it routine under rule `:28` (wording → sound impl, invariant demonstrated): the decision's live-Pulse invariant is preserved and asserted MET, only the unqualified framing went.
