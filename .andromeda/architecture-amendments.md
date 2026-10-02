# Architecture — Amendments

_Append-only changelog of amendments to `architecture.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1
**Section:** §Stack and Technologies (+ §Infrastructure Patterns Build system · §Inherited Defaults · directory-tree comment)
**Change:** MSRV pinned to 1.94.1 (was 1.88.0) across all occurrences; the workspace pins build toolchain 1.95.0 with `rust-version = "1.94.1"` as the MSRV floor.
**Why:** implements security-plan §Dependency Security's required bump (≥1.94.1, tar-rs CVE-2026-33056), which superseded arch's stated MSRV 1.88.0.
**Ref:** .andromeda/runs/2026-06-14T23-46-47-wrap/

## 2026-06-14-cargo-workspace-scaffold — self-observation stack row added
**Section:** §Stack and Technologies
**Change:** added a "Self-observation | tracing 0.1.44 + tracing-subscriber 0.3.23" row.
**Why:** `tracing` + `tracing-subscriber` were pinned in `[workspace.dependencies]` as the obs-plan §3 self-obs stack, which arch's Stack table did not list. NOT an OTel SDK — OTLP remains the PRODUCT emission.
**Ref:** .andromeda/runs/2026-06-14T23-46-47-wrap/

## 2026-06-15-conductor-core-shared-types — serde_json 1.0 registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "Serialization (JSON) | serde_json 1.0" row.
**Why:** `serde_json = "1.0"` joined `[workspace.dependencies]` (canonical-name round-trip tests now; the run-report envelope + per-run JSONL journal consume it at runtime) and arch's §Stack registry did not list it; audit-green and allowed by security-plan §Dependency Security.
**Ref:** .andromeda/runs/2026-06-15T00-36-47-wrap/

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Stack and Technologies (Validation row) · §Established Decisions [Validation Library] · §Inherited Defaults (Validation)
**Change:** garde version 0.23.0 → 0.22.1 across all three; §Established Decisions now records that garde 0.22.1's `#[garde(custom)]` is field-level only (no container-level custom) — cross-field invariants spanning distinct fields use garde's `Context` pattern.
**Why:** garde's `derive` feature was needed, but `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + `derive` is unbuildable; the user authorized the downgrade. The no-duplicate-P-IDs validator consequently landed field-level, not struct-level.
**Ref:** .andromeda/runs/2026-06-15T15-45-15-wrap/

## 2026-06-15-structured-logging-stack — CONDUCTOR_SERVICE_NAME / CONDUCTOR_ENV registered
**Section:** §Occupied Resources (Environment variables)
**Change:** added `CONDUCTOR_SERVICE_NAME` (self-obs `service.name` override) and `CONDUCTOR_ENV` (self-obs `deployment.environment`, default `local`) to the reserved `CONDUCTOR_*` env-var list.
**Why:** `init_observability` reads both (obs-plan §3) to populate `ServiceIdentity`; the arch env-var inventory listed only RUNS_DIR/SCENARIOS_DIR/CONTRACT_MANIFEST/SEED. No OTel SDK introduced.
**Ref:** .andromeda/runs/2026-06-15T17-46-44-wrap/

## 2026-06-15-design-token-typography-bundle — frontend stack + ui/ asset subtree registered
**Section:** §Stack and Technologies (new Desktop-frontend row) · §Occupied Resources (Frontend asset subtree) · §Inherited Defaults (Frontend bullet)
**Change:** recorded the optional GUI's realized frontend toolchain — React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide via `@tailwindcss/vite`) + Fontsource WOFF2, package manager npm (committed `package-lock.json` + `npm audit` gate); registered the `crates/conductor-tauri/ui/` asset subtree (not a Cargo member; `node_modules/` + `dist/` git-ignored) and the npm/frontend default.
**Why:** the first frontend code landed under `crates/conductor-tauri/ui/`; arch §Stack pinned no Vite major (Vite 8.0.16 is the npm-audit floor clearing esbuild GHSA-gv7w-rqvm-qjhr), and §Occupied Resources / §Inherited Defaults did not record the frontend asset layer. User-approved at the wrap escalation.
**Ref:** .andromeda/runs/2026-06-15T22-05-00-wrap/

## 2026-06-16-test-framework-fixtures-coverage-tooling — test/coverage toolchain registered in Build system
**Section:** §Infrastructure Patterns — Build system
**Change:** the test clause now names cargo-nextest as the pinned runner (zero-retry `ci` profile in `.config/nextest.toml`) + `cargo test --doc`, the dev-test stack (rstest/proptest/insta/assert_cmd/assert_fs/predicates), and `cargo-llvm-cov` line coverage requiring the `llvm-tools-preview` toolchain component; exact versions deferred to test-plan §4.
**Why:** the chunk landed exactly this toolchain, all gates green; arch §Infrastructure named only `cargo test` / cargo-nextest loosely. Routine spec→sound-impl alignment.
**Ref:** .andromeda/runs/2026-06-16T16-46-23-wrap/

## 2026-06-16-seeded-phase-scheduler — seedable RNG (rand_chacha/rand_core) registered in §Stack + §Established Decisions
**Section:** §Stack and Technologies (new Determinism RNG row) · §Established Decisions (new [Determinism RNG] entry)
**Change:** added a "Determinism RNG | rand_chacha 0.9 (`ChaCha8Rng`) + rand_core 0.9 (`SeedableRng`)" Stack row and a [Determinism RNG] established decision locking `ChaCha8Rng` + `seed_from_u64` (platform/version-stable) as the timeline scheduler's sole non-determinism source.
**Why:** rand_chacha 0.9 / rand_core 0.9 joined `[workspace.dependencies]` for deterministic per-gap jitter; arch §Stack listed no RNG and §Established Decisions did not record the RNG/seeding choice. arch §Cross-cutting already mandated a "seeded RNG"; this records its concrete algorithm-stable realization (routine spec→sound-impl alignment).
**Ref:** .andromeda/runs/2026-06-16T19-34-21-wrap/

## 2026-06-16-scenario-config-model — toml 0.9 registered in §Stack + [Scenario Config Format] decision
**Section:** §Stack and Technologies (new Scenario config (TOML) row) · §Established Decisions (new [Scenario Config Format] entry)
**Change:** added a "Scenario config (TOML) | toml 0.9" Stack row and a [Scenario Config Format] established decision locking declarative TOML scenario config (serde-deserialized + garde-validated via `Scenario::from_toml_str`), chosen over JSON for hand-author ergonomics + inline comments across the 60 per-P-ID files.
**Why:** `toml = "0.9"` joined `[workspace.dependencies]` + conductor-core for the declarative per-phase emission spec (the user's P4 format decision); arch §Stack listed only serde_json and §Established Decisions recorded no scenario config format. Precedent it follows and sets: a new audit-green dep → a §Stack row; a locked format choice → an §Established Decisions entry.
**Ref:** .andromeda/runs/2026-06-16T20-48-47-wrap/

## 2026-06-18-exception-events-fingerprint-control — fingerprint primitive placed in conductor-emit
**Section:** §Infrastructure Patterns (directory-tree crate comments) — cascaded to CLAUDE.md §Modules
**Change:** the per-exception fingerprint PRIMITIVE (`fingerprint()` + the exception-event builder, identical/path/line/type/frame variants) is recorded in `conductor-emit` (co-located with the exception content it derives from); `conductor-faults`' "fingerprint generation" is narrowed to the fingerprint-STORM fault (Epoch-7), which will depend on emit and compose this primitive.
**Why:** `fingerprint()` + `exception_trace_request()` landed in `conductor-emit` per the crate seam the user ratified at /andromeda-phase (the error-spans "builder-in-emit" precedent; faults was an empty stub); arch's dir-tree comment had attributed all "fingerprints" to faults. Documentation alignment, no behavioral change.
**Ref:** .andromeda/runs/2026-06-18T00-45-24-wrap/

## 2026-06-21-run-report-envelope-serializer — ManualCheck widened + Verdict→ReportState default mapping
**Section:** §Read-Back Dependency Posture · §Probabilistic-Assertion Policy
**Change:** ManualCheck's definition broadened from "operator-checklist / no-programmatic-read-back only" to ALSO include an auto-measured model-interpretive (calibration-region) check; recorded the default `Verdict → ReportState` mapping (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`, via `Verdict::default_report_state`), with `verdict`/`state` kept independent and the run-report lamp chosen verdict-first (a calibration row renders HOLD, not Manual).
**Why:** the chunk shipped `Verdict::default_report_state` (a user-approved P4 decision) + `RunRecord::measured`, and arch's narrow ManualCheck definition was stale. Verdict-first lamp precedence resolves the a11y §6 six-lamp conflict (user-confirmed).
**Ref:** .andromeda/runs/2026-06-21T18-23-37-wrap/

## 2026-06-21-runs-db-index — SQLite/libsqlite3-sys versions corrected to the shipped lock
**Section:** §Stack and Technologies (Database row) · §Inherited Defaults (Database)
**Change:** `libsqlite3-sys 0.38.0 → 0.36.0` and bundled `SQLite 3.51.1 → 3.50.4` across both spots (rusqlite 0.38.0 unchanged).
**Why:** rusqlite 0.38.0 transitively pins `libsqlite3-sys 0.36.0`, which bundles SQLite 3.50.4; arch's stated 0.38.0 / 3.51.1 were assumed, not the resolved lock. The functional invariant holds (bundled, JSON1 proven by the `json_array_length` test, the §Established Decisions ≥3.38 floor satisfied); audit-green. Mirrored in security-plan §Infrastructure (same correction).
**Ref:** .andromeda/runs/2026-06-21T19-09-44-wrap/

## 2026-06-21-runs-db-index — runs.db instant columns are TEXT RFC-3339, not integer-ms offsets
**Section:** §Data model conventions · §Standard Contracts (Timestamp formats)
**Change:** the `journal_emitted_at`/`read_back_observed_at` `runs.db` columns are stored as TEXT RFC-3339 (the JSONL envelope's wire form); the integer-millisecond value the SLO math consumes is the separate `latency_ms` INTEGER column (was: "stored as the same integer-millisecond journal offsets … not ISO strings").
**Why:** the user's P4 decision. The envelope carries only second-precision RFC-3339 instants + a precomputed `latency_ms`; storing the two instants as epoch-ms would add a date-parser dep for no SLO-math gain, since `latency_ms` already IS the journal-relative delta the math consumes. The SLO-math invariant (integer-ms via `latency_ms`) is preserved; obs §3/§6 already use RFC-3339 TEXT.
**Ref:** .andromeda/runs/2026-06-21T19-09-44-wrap/

## 2026-06-23-conductor-run-suite-report-verbs — clap 4 registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "CLI argument parsing | clap 4 (`derive`)" Stack row.
**Why:** `clap = { version = "4", features = ["derive"] }` joined `[workspace.dependencies]` + conductor-cli for the `run`/`suite`/`report` verb surface, and arch §Stack listed no CLI arg parser; audit-green (a new audit-green dep → a §Stack row).
**Kept:** no §Established Decisions entry — clap is a standard CLI parser, not a consequential fork. The CLI-level `scenario.run` root span (obs §4) is a carried code follow-up, not doc drift; obs §4 stays target-state.
**Ref:** .andromeda/runs/2026-06-23T18-00-59-wrap/

## 2026-06-23-5-command-agent-run-harness — CONDUCTOR_PREFLIGHT_TIMEOUT registered
**Section:** §Occupied Resources (Environment variables)
**Change:** added `CONDUCTOR_PREFLIGHT_TIMEOUT` (preflight readiness-gate timeout in seconds, default 30; read by `scripts/agent-run.{sh,ps1}` `boot`) to the reserved `CONDUCTOR_*` env-var list.
**Why:** the `conductor preflight` verb (the `agent-run boot` entrypoint) is gated by this timeout, which `agent-run.{sh,ps1}` read as `PREFLIGHT_TIMEOUT_SEC`. It predates this chunk (Epoch-1 skeleton) but was never registered, and this chunk made the preflight path it gates load-bearing.
**Ref:** .andromeda/runs/2026-06-23T19-20-31-wrap/

## 2026-06-23-line-oriented-output-rendering — terminal-rendering stack (owo-colors/indicatif/comfy-table) registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "Terminal output rendering | owo-colors 4 + indicatif 0.17 + comfy-table 7" Stack row (the `conductor-cli` presentation layer — tty-gated status-line color, run/suite progress spinner, results + 60-P-ID coverage tables).
**Why:** owo-colors 4.3.0 / indicatif 0.17.11 / comfy-table 7.2.2 joined `[workspace.dependencies]` + conductor-cli for line-oriented output, and arch §Stack listed no terminal-rendering layer. `cargo audit` + `cargo deny check` green, with the indicatif→number_prefix advisory and the pre-existing foldhash Zlib license recorded as accepted `deny.toml` exceptions (security-plan §Dependency Security).
**Kept:** no §Established Decisions entry (standard presentation crates). The `conductor coverage` verb and the render module's public fns stay out of §Occupied Resources — the verb is layout-templates' concern, and a library API is library-symbol over-reach.
**Ref:** .andromeda/runs/2026-06-23T20-40-17Z-wrap/

## 2026-06-23-isatty-gated-operator-pause — inquire 0.9 registered in §Stack (terminal-rendering row)
**Section:** §Stack and Technologies (Terminal output rendering row)
**Change:** appended `+ inquire 0.9` to the terminal-rendering Stack libs (owo-colors/indicatif/comfy-table) + the role gained "isatty-gated interactive operator-pause prompts (inquire confirm; headless never blocks)".
**Why:** `inquire = "0.9"` (→0.9.4) joined `[workspace.dependencies]` + conductor-cli for the CLI interactive `PauseResolver`, and the row listed no interactive-prompt lib. `cargo audit` + `cargo deny check` green with its transitives (crossterm 0.29 / fuzzy-matcher 0.3.7 / derive_more 2.1.1) needing no new `deny.toml` exception.
**Kept:** no §Established Decisions entry (a standard CLI-prompt crate, not a consequential fork). The `CliResolver`/`PromptResolver`/`hold_line` symbols stay out of §Occupied Resources (library-symbol over-reach).
**Ref:** NOT DERIVED

## 2026-06-24-sanitized-stderr-agent-mode-logging — registered CONDUCTOR_AGENT_MODE env var + logs/agent-latest.jsonl artifact
**Section:** §Occupied Resources (Environment variables + On-disk artifacts)
**Change:** registered two new occupied resources — the `CONDUCTOR_AGENT_MODE` env var (a read-only agent-mode trigger; `flag || env-set`, main never writes it) and the `logs/agent-latest.jsonl` on-disk artifact (the self-obs `tracing` JSON stream in agent mode; a SEPARATE artifact + schema from the emission journal, a sibling of the runs dir moving with `CONDUCTOR_RUNS_DIR`).
**Why:** arch §Occupied Resources explicitly tracks env vars and on-disk artifacts, and neither was registered. No new dep — a std-only file sink; the `ObsWriter` enum-dispatch mirrors the shipped `CliResolver` precedent.
**Kept:** the `--agent-mode`/`--debug` CLI FLAGS stay layout-templates' concern (its §cli "Error output" + pipe discipline), not arch.
**Ref:** .andromeda/runs/2026-06-24T20-38-37-wrap/

## 2026-06-24-frameless-window-shell — registered logs/conductor-tauri.jsonl artifact + @tauri-apps/api/tauri-build + the generate_context! build-order coupling
**Section:** §Occupied Resources (On-disk artifacts) · §Stack and Technologies (Desktop frontend row) · §Infrastructure Patterns (Build system)
**Change:** registered `logs/conductor-tauri.jsonl` (the Tauri backend self-obs `tracing` JSON stream via `ObsSink::File`, sibling of the runs dir — the GUI shell's analogue of `agent-latest.jsonl`); added `@tauri-apps/api` (window/IPC client) to the Desktop frontend stack row; documented `tauri-build`'s `generate_context!` resolving `build.frontendDist` (`ui/dist`) at COMPILE time → the webview bundle must be built before any workspace cargo compile of `conductor-tauri` (the `ensure_frontend` step wired into `agent-run.{sh,ps1}` + the CI Rust job), plus the Tauri tree's justified `deny.toml` additions (unmaintained gtk/unic/proc-macro-error advisories + `MPL-2.0` / `Apache-2.0 WITH LLVM-exception` licenses).
**Why:** the first real Tauri 2 app. `logs/conductor-tauri.jsonl` is a genuine new on-disk artifact (the `agent-latest.jsonl` precedent); `@tauri-apps/api` + `tauri-build` are genuinely added deps; the frontend-before-cargo coupling is a real build-system constraint affecting CI and the harness. tauri 2.11.3 meets the ≥2.10.3 floor; the Tauri sink inherits the unchanged `init_observability` redaction layer.
**Kept:** the three `core:window:*` capability perms stay out of §Occupied Resources (user-confirmed) — framework ACL perms in the deny-by-default `capabilities/` file are security-plan's domain, not Conductor IPC methods; the Conductor command surface is unchanged.
**Ref:** NOT DERIVED

## 2026-06-26-live-counter-channel-stream — registered the conductor-run library crate (9th workspace member)
**Section:** §Occupied Resources (Crate names) · §Infrastructure Patterns (directory tree)
**Change:** added `conductor-run` to the workspace-member registry + the directory tree — the run composition root library (preflight + execute_scenario + persist + the live-counter drive_run) sitting above the seams and below both bins, shared by conductor-cli + conductor-tauri.
**Why:** conductor-cli's bin-local `pipeline.rs` was extracted into a NEW workspace crate, and a workspace member IS an occupied resource (the crate-names list) — unlike the library-symbol / command-name / module / config-file / capability-perm over-reach that stays out. The core-owned `current_thread` runtime under Tauri matches §Async Runtime Flavor, the live-counter Channel was already pinned in §Real-time Strategy, and no external crate was added.
**Ref:** .andromeda/runs/2026-06-26T22-49-24-wrap/

## 2026-06-27-desktop-a11y-harness-setup — npm-audit gate one-liners made dev-aware (`--omit=dev`)
**Section:** §Stack and Technologies (Desktop frontend row) · §Inherited Defaults (Frontend)
**Change:** the two npm-audit gate one-liners `npm audit` → `npm audit --omit=dev` (production-dep strict; dev-only test-tooling advisories accepted at dev-tree grain) — kept consistent with the authoritative security-plan §Dependency Security amendment.
**Why:** the substantive amendment is security-plan's (the a11y harness's dev-only transitive advisories → a dev-aware gate, user-decided); arch carries derived one-liner summaries of it, updated in lockstep.
**Kept:** the a11y test devDeps (axe-core / webdriverio / lighthouse / colorjs.io / @crabnebula/tauri-driver) are not registered in arch §Stack / §Inherited Defaults — arch summarizes the frontend stack at React/Vite/Tailwind/npm grain and never enumerates test tooling, which lives in a11y-plan §3.5 and the stack.md distillation.
**Ref:** .andromeda/runs/2026-06-27T12-09-32-wrap/

## 2026-06-27-mcp-read-back-result-shape-adapter — [MCP Read-Back Client] REVERSED: rmcp removed → hand-rolled JSON-RPC
**Section:** §Stack and Technologies (MCP read-back row) · §Established Decisions [MCP Read-Back Client] · §Read-Back Dependency Posture · §Conventions (Inbound verification + Error handling) · §Inherited Defaults · directory-tree comment
**Change:** the read-back client decision flips from "rmcp 1.7.0 (official SDK; typed `list_all_tools()`/`call_tool()`; version negotiation via `peer_info()`)" to "hand-rolled line-delimited JSON-RPC over the sidecar's stdio" — `initialize`/`tools/list`/`tools/call` returning the RAW `serde_json::Value`; version negotiation reads the `initialize` result's `protocolVersion`; rmcp removed from conductor-verify. The hardened spawn + the `2024-11-05` manifest pin + the preflight gate are unchanged.
**Why:** the superseded rationale (preserved here per the body's pointer) read "rmcp 1.7.0 — official SDK with version negotiation + typed tool calls; hand-rolled JSON-RPC and third-party rust-mcp-sdk were rejected because re-deriving version negotiation is the silent-mismatch risk class the preflight exists to prevent" (with a caveat that Pulse's server is itself hand-rolled `2024-11-05`). Sound at design time but contradicted by the live SUT: Pulse's `andromeda-pulse-mcp` is non-MCP-compliant for `tools/call` — it returns the raw tool payload as `result` (no `{content:[…]}` envelope), which rmcp's typed `call_tool` deserializes into `ServerResult` and rejects as `UnexpectedResponse` on EVERY live call, and rmcp exposes no raw-result escape. So the "rejected hand-rolling" clause is reversed: hand-rolling is the faithful match to a hand-rolled non-compliant server, and version negotiation reduces to reading one `initialize` field. The user confirmed the approach at /andromeda-phase P4 and the reversal at the wrap escalation. Standing rule set: a SUT that contradicts a locked decision → routine-apply.
**Ref:** .andromeda/runs/2026-06-27T20-59-40-wrap/

## 2026-06-27-live-pulse-e2e-proof — canary premise reversed (Pulse incident creation is LLM-in-the-loop non-deterministic) + corpus.db plaintext + read-back surface
**Section:** §Established Decisions [Read-Back Dependency Posture] · §Standard Contracts (Readiness gate + corpus access) · §Occupied Resources (`ANDROMEDA_PULSE_DATA_DIR`)
**Change:** the canary "emit one known incident → assert `query_incident_list` returns it" round-trip is superseded — Conductor emits a unique fingerprint-storm (telemetry) and the fidelity carrier is `retrieve_telemetry_slice.fingerprint_refs` (Pulse scrubs incident titles). The original premise is unattainable: Pulse's incident creation is NON-DETERMINISTIC + LLM-in-the-loop (OTLP → L1 → L2 RetryStorm cue [deterministic, ≥5 same fingerprint/30s] → L3 digest [20-60s cadence] → **L4 llama.cpp Llama-3.2-3B decides Dismiss/Severity** → incident), so deterministic incident-readback verification against the real LLM is impossible — an OPEN posture decision (leading: a deterministic test-L4 mode in Pulse). `corpus.db` is **plaintext SQLite** (the P-049 "encrypted at rest / `OsKeychainBackend`" assumption is WRONG → keychain-failure canary mode N/A); the live read-back surface is **8 tools** (only the 4 persistent-corpus tools work cross-process from a Conductor-spawned sidecar — the in-memory-buffer tools return empty); the incidents filter column is `workspace` (not `workspace_root`).
**Why:** verified live against Pulse (operator findings): ingest works, yet Pulse creates no incident, so the shipped canary fingerprint-fidelity bridge (Part A) makes live `conductor preflight` correctly return `Blocked: incident not found in corpus`. The live SUT contradicts a §Standard Contracts assumption, so the reversal is escalate-once-then-apply; user-confirmed in the wrap directive. The deterministic-verification posture is the chunk's PENDING follow-up, and Part B (the 2 live families + live `ready:true`) is deferred on the same open decision. Pulse run recipe for a future live pass: pulse-app needs `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`; the build needs `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (a triage/build.rs bug truncates the 9 MB tokenizer → L4 breaks); RetryStorm ≥5/≥10 same fingerprint/30s; L4 ≈4 s/inference.
**Ref:** .andromeda/runs/2026-06-27T23-24-58-wrap/

## 2026-08-08-sut-capability-manifest — Accepted capability set is manifest data (SUT-advance reversal)
**Section:** §Established Decisions (new [Accepted Capability Set]) · §Occupied Resources On-disk artifacts · §Infrastructure directory tree · §Conventions Naming · §Cross-cutting Scope law · §Project Intent
**Change:** the accepted Pulse P-ID set moved from the compile-time `(1..=60)` bound in garde's `pid_format` to the versioned runtime-read `contracts/pulse-capabilities.toml`; validation split into shape (garde) + membership (`Scenario::check_capabilities` via `from_toml_str_with`); a malformed/absent manifest is a `CoreError` harness fault, never Blocked. Every "all 60 P-IDs" / "P-001..P-060" claim de-hardcoded to name the manifest's accepted set.
**Why:** Pulse's ledger advanced to P-082 during a 41-day pause, and a scenario naming anything above P-060 failed garde validation at load, so the harness could not express what the SUT had shipped. Ratified at wrap (escalate-once); de-hardcoding rather than substituting 82 was the operator's decision, so the next Pulse release needs no doc edit.
**Ref:** .andromeda/runs/2026-08-08T16-05-00-wrap/

## 2026-08-08-dependency-advisory-remediation — anyhow pin 1.0.102 → 1.0.104
**Section:** §Stack and Technologies (Error handling row) · §Established Decisions [Error Handling] · §Inherited Defaults (Error handling)
**Change:** the three arch-registered `anyhow 1.0.102` pins refresh to `anyhow 1.0.104`. thiserror 2.0.18, the anyhow-only-at-binary-edges rule, and the verdict/error-wall prose are untouched — only the version value moves.
**Why:** one of four decayed advisories cleared by dependency bump was RUSTSEC-2026-0190 (`anyhow` 1.0.102, `unsound` in `Error::downcast_mut()`, patched `>=1.0.103`) — reported by `cargo audit` as an exit-0 allowed warning while `cargo deny` denies it, so it was invisible to an audit-only check. The operator's P4 decision raised the `[workspace.dependencies]` floor to `1.0.104` alongside the lock bump, so the manifest no longer names a version with a known unsound advisory; arch's pins followed. Spec-value → sound-impl reconciliation with the decision's invariants preserved.
**Kept:** security-plan carries no anyhow version (its `anyhow` mentions are error-sanitization edge references; §Dependency Security's version-bearing content is the cargo-audit ≥0.22 / cargo-deny ≥0.19 tool floors and the `tauri` ≥2.10.3 line), so no lockstep security-plan amendment. Pre-existing gaps this chunk did not introduce, left for follow-up: arch §Stack's `tokio 1.48.x` vs the resolved 1.52.3; arch §Stack's Tauri `v2.10.x / latest 2.10.1` vs the resolved 2.11.3; `deny.toml`'s 17 ignores / 8 allows vs security-plan §Accepted exceptions naming one of each.
**Ref:** .andromeda/runs/2026-08-09T11-22-56-wrap/

## 2026-08-09-current-sut-coverage-classification — Re-aiming is a manifest edit PLUS a classification row
**Section:** §Established Decisions [Accepted Capability Set] · §Occupied Resources (`contracts/pulse-capabilities.toml`) · §Stack and Technologies (Terminal output rendering row)
**Change:** the [Accepted Capability Set] decision's closing claim — "re-aiming at a newer Pulse release is now a manifest edit with no Rust change" — is qualified to current truth: the **accepted set** is data, but the **coverage classification** stays code-native (`coverage_matrix()`, a no-runtime-IO `static`), and `check_sut_drift` holds the two to set-equality. Re-aiming is therefore a manifest edit **plus** a matching classification row; an accepted id with no row, or a classified id the manifest no longer accepts, is a `CoreError::SutDrift`. The `KNOWN_UNCLASSIFIED` residual ledger is recorded as retired to `[]`. §Occupied Resources' echo of the same claim follows in lockstep. Separately, the §Stack terminal-rendering row's "60-P-ID coverage tables" reads "capability-coverage tables (the manifest's accepted set)".
**Why:** the chunk classified all 82 manifest-accepted capabilities and emptied the residual ledger, making the manifest↔classification coupling exact and unavoidable. The two-source shape is deliberate: the operator's P4 representation decision chose code-native classification so `check_sut_drift` stays a genuine two-source comparison rather than collapsing into a self-check. The decision's invariant (the accepted set is DATA, never a compile-time constant) is fully preserved; only its consequence-wording moved.
**Ref:** .andromeda/runs/2026-08-09T14-17-38-wrap/

## 2026-08-09-interpretation-correctness-posture — second coverage-integrity gate + interpretation-correctness deferral
**Section:** §Established Decisions [Accepted Capability Set] · [Read-Back Dependency Posture]
**Change:** [Accepted Capability Set] gains the second integrity gate on a distinct axis — `check_scenario_backing` holds the `UNBACKED_AUTO` pin (`Auto`-classified capabilities no scenario names) to exact-set equality against the committed catalog, failing on a new unbacked claim / pin rot / a pin that lost its `Auto` classification (`CoreError::UnbackedCoverage`); only `Auto` participates. [Read-Back Dependency Posture]'s "OPEN posture decision" is closed: the deterministic-L4 option landed Pulse-side, and its cost is recorded as a deferral — "Conductor green" does not mean interpretation is trustworthy — owned by a conductor-0.3.0 entry and pinned in `.andromeda/residuals.md`.
**Why:** the chunk shipped both the gate and the deferral; arch described `check_sut_drift` as the single integrity gate and still carried the posture question as open, so the body no longer matched current truth.
**Ref:** .andromeda/runs/2026-08-09T19-30-00-wrap/

## 2026-08-09-sut-load-envelope — load-envelope artifact + runs.db second table registered
**Section:** §Occupied Resources (On-disk artifacts) · §Infrastructure Patterns (directory tree) · §Stack (ORM row) · §Established Decisions [ORM]
**Change:** registered `contracts/pulse-load-envelope.toml` as a committed on-disk artifact (terms + provenance + exemption ledger; fixed `LoadEnvelope::default_path()` through `resolve_under`, no `CONDUCTOR_*` override; only the duration term asserted, rate terms declared-not-derivable). Extended the `runs.db` bullet to its two tables (`runs` per-check + the additive run-level `run_envelope`) and restated the [ORM] / §Stack qualifier from "~one indexed table" to a small fixed set of hand-written tables. The directory-tree `contracts/` comment now names three manifests.
**Why:** the chunk landed a third committed contracts artifact and a second runs.db table; §Occupied Resources enumerates committed artifacts, and the one-table qualifier no longer matched. The no-ORM decision itself is unchanged.
**Ref:** .andromeda/runs/2026-08-10T15-43-07-wrap/

## 2026-08-10-workspace-key-divergence-probe — readiness gate: a fourth named precondition
**Section:** §Standard Contracts — Readiness gate (the `ready:false` paragraph)
**Change:** the gate's named-precondition set is stated as FOUR (was three): protocol version-mismatch · required-tool absence · emitted fingerprint absent from an existing incident · **app/sidecar workspace-key agreement**. The fourth is emitted on a zero-incident `query_incident_list`, which is byte-identical on the wire to a workspace-key divergence (the sidecar keys on `ANDROMEDA_PULSE_DATA_DIR`, `pulse-app` on its detected workspace root), so the string names the key agreement AND "Pulse raised no incident" as the two candidate causes rather than claiming a measurement Conductor cannot make — read-back exposes no second key (`query_incident_list` takes no arguments). Every precondition string is host-path-free (`data_dir` redacted).
**Why:** the chunk replaced the opaque `canary round-trip failed: incident not found in corpus` with the named precondition, verified by test legs and live `conductor preflight --json` runs, with the Pulse-side keying mechanics confirmed first-hand.
**Ref:** .andromeda/runs/2026-08-10T19-49-03-wrap/

## 2026-08-10-pulse-run-contract — the run contract registered as the fourth `contracts/` manifest
**Section:** §Occupied Resources — On-disk artifacts (+ the `contracts/` line in the §Infrastructure Patterns directory tree)
**Change:** registered `contracts/pulse-run-contract.toml` — the pinned Pulse run contract (`sut_version` · `captured_at` · `provenance` · an `[incident_formation]` table · a `[[term]]` list), runtime-read and bounds-checked at load, resolved from a fixed `RunContract::default_path()` through `resolve_under` with deliberately NO `CONDUCTOR_*` override handle. The entry records what each `check` kind MEANS as a statement about what Conductor can honestly know — `shell-declaration` (observable in Conductor's own environment, the only kind that can block), `asserted` (satisfied by construction), `declared-not-observable` (true on the SUT's side with no read-back surface, recorded but never blocking, because blocking would claim a measurement). The directory-tree comment moved from three manifests to four.
**Why:** the chunk landed the artifact; §Occupied Resources enumerates each `contracts/` manifest individually (the `pulse-capabilities` / `pulse-load-envelope` precedent), so a fourth left unregistered would be an unregistered resource.
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/

## 2026-08-10-pulse-run-contract — preflight timeout floor + the L4 declaration Conductor reads
**Section:** §Occupied Resources — Environment variables
**Change:** `CONDUCTOR_PREFLIGHT_TIMEOUT` keeps its default of 30 but now carries a run-contract-derived effective FLOOR (`[incident_formation].min_canary_poll_seconds`) it cannot sit below, because the bare default is shorter than Pulse's own L3 digest cadence; the env handle still overrides upward. Added `ANDROMEDA_PULSE_L4_DETERMINISTIC` — Pulse-side, asserted not set by Conductor, but now READ in Conductor's own environment as the contract's shell-declaration proxy, with an absent declaration surfacing as an unmet term naming both candidate causes rather than a claimed measurement of `pulse-app`.
**Why:** the chunk made both changes real; the registry stated the bare 30s default and did not list the L4 var at all.
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/

## 2026-08-10-pulse-run-contract — readiness gate: a fifth named precondition
**Section:** §Standard Contracts — Readiness gate
**Change:** the gate's named preconditions are FIVE (was FOUR), adding **unmet run-contract terms**. That arm composes ONE string naming each unmet term individually (its condition and its candidate causes) and sits after the tool checks but BEFORE the canary arms, skipping the canary poll rather than paying it — an unmet launch condition explains a failed canary, so surfacing the canary symptom first sends the operator to the wrong cause, and under the raised poll floor it would spend the whole budget doing so.
**Why:** the chunk added the arm. The ordering rationale is recorded because the preceding workspace-key probe hit exactly that failure mode — it blocked for the no-incident reason while the real blocker sat upstream.
**Ref:** .andromeda/runs/2026-08-10T21-24-17-wrap/
## 2026-08-11-faithful-emission-dispatcher — load-envelope rate terms; error-fraction encoding; nested-spec `dive`
**Section:** §Occupied Resources (`contracts/pulse-load-envelope.toml`) · §Established Decisions [Validation Library] · §Conventions (Config conventions)
**Change:** the load envelope's rate terms move from *declared-not-derivable* to **derivable but deliberately not yet asserted** — `EmissionSpec::occurrences` is exactly the per-phase occurrence-count field whose absence was the old justification; the assertion surface and the `[[exempt]]` ledger are unchanged, and re-scoping to emitting-phase duration is recorded as carried chunk work rather than an amendment. [Validation Library] and Config conventions now name the shipped error-fraction encoding (`EmissionShape::Error { error_percent: u32 }` ∈ 0..=100, an integer percent rather than an f64 so `PhaseSpec`/`Scenario` keep `Eq`), state that a nested spec field must `dive` and never `skip`, and record that a phase's emission shape is declared data (`EmissionSpec { signal, occurrences, shape }`, `occurrences: 0` = a deliberate silence window).
**Why:** the per-phase emission dispatcher shipped: the declarative shape model landed in `conductor-core`, which falsified the load envelope's stated premise and replaced an aspirational `[0,1]` bound with a real integer-percent field. `PhaseSpec.emission` was `#[garde(skip)]`, so the nested rules the specs mandated had never executed.
**Ref:** .andromeda/runs/2026-08-13T16-43-31-wrap/

## 2026-08-13-dispatcher-determinism-goldens — load-envelope asserted terms inverted; exemption ledger retired
**Section:** §Occupied Resources — the `contracts/pulse-load-envelope.toml` bullet
**Change:** The two sustained terms (`max_sustained_storm_ms` + `max_sustained_rate_spans_per_s`) are now the ASSERTED ones, judged per emitting phase; `max_scenario_duration_ms` inverts to recorded-but-not-asserted. `check_load_envelope` and `LoadEnvelope::classify` read ONE shared basis, so the static gate and the run-level `[ENVIRONMENT-SUSPECT]` caption cannot diverge. The `[[exempt]]` ledger is retired to EMPTY (still exact-set in both directions). The bullet records that the artifact's own predicted landing — asserting SUMMED emitting-phase duration — was measured FALSIFIED, and that the shipped term bounds the longest single emitting window instead.
**Why:** Summed emitting-phase duration leaves `activity-floor` (900 000 ms) and `incident-auto-resolution` (610 000 ms) over the 600 000 ms ceiling across all 35 committed scenarios, so it retires no exemption and changes no gate verdict — summing disjoint bursts separated by quiet is not *sustained*. Under the per-phase joint bound every scenario passes unaided (longest single emitting phase 600 000 ms, exactly at the ceiling; peak rate 4.00 spans/s against 10 000), so the ledger genuinely empties. The re-scope the bullet carried as chunk work is discharged, but by a different term than it predicted.
**Ref:** .andromeda/runs/2026-08-13T18-25-00-wrap/

## 2026-08-13-first-live-green-preflight — [incident_formation] warm-up measured false
**Section:** Occupied Resources — `contracts/pulse-run-contract.toml`
**Change:** The claim that the warm-up pre-roll "carries the canary service out of Pulse's baseline bootstrap before the counted storm" is recorded as measured FALSE, with the mechanism: Pulse gates cue evaluation on `BootstrapState::Ready`, which needs `now − first_observed_unix_nanos ≥ BOOTSTRAP_WINDOW_SECONDS = 3_600` per service wall-clock, while `baseline_state` holds 0 rows so each launch resets the anchor. `warmup_ms = 45000` is short by 80x and unfixable by its own knobs; the `check = "asserted"` term cannot catch its own falsity. The underlying diagnosis (no baseline ⇒ the cue evaluator never considers the service) is unchanged — only the remedy is disproved, and the fix is Pulse-side.
**Why:** The first live leg's three probe arms all blocked for the no-incident cause, with no cue emitted, no service ready and no incident formed throughout.
**Ref:** .andromeda/runs/2026-08-13T22-48-53-wrap/

## 2026-08-14-canary-fingerprint-feed-capture — a second Pulse-side gap on the canary path
**Section:** §Occupied Resources (`contracts/pulse-run-contract.toml`)
**Change:** Recorded beside the warm-up falsification: Conductor's storm reaches the wire with its `exception` events intact and Pulse receives and counts all nine spans, yet the fingerprint table stays empty — so the gap lies inside Pulse between OTLP ingest receipt and the per-span-event fingerprint observer, a REGION rather than a named defect. Also records how to read that telemetry: `tracked_fingerprints_count` is a 60s-windowed gauge over DISTINCT fingerprints sampled after eviction (a working six-occurrence identical-fingerprint storm reads 1, never 6), and the window-immune discriminators are the cumulative `storms_detected_total` / `fingerprints_evicted_total`.
**Why:** The chunk's capture settled its fork, with `span_count` verified as a cumulative counter in the SUT's source rather than inferred from its shape. The finding bears directly on the run contract's `[incident_formation]` premise — the canary storm is emitted to raise an incident and the feed that would raise it never engages — so it belongs beside the warm-up falsification the section already carries.
**Ref:** .andromeda/runs/2026-08-14T16-51-43-wrap/

## 2026-08-15-canary-storm-autonomous-band — the bootstrap misattribution corrected, the second gap un-retired
**Section:** §Occupied Resources (`contracts/pulse-run-contract.toml`) · §Established Decisions [Read-Back Dependency Posture]
**Change:**
- §Occupied Resources (1): the `BootstrapState::Ready` gate is scoped to the **baseline-derived cue families** — it is the only such gate in `crates/triage/` and sits inside `evaluate_service_went_silent`, while the RetryStorm path consults no baseline.
- (2): was "reaching a live incident requires a Pulse-side change (bootstrap override / `baseline_state`)"; retired as measured-false and replaced by the **tier band** — `CANARY_STORM_COUNT = 6` sat in `5 <= 6 < DEFAULT_AUTONOMOUS_THRESHOLD = 10` with Tier-1 Autonomous-only, raised to 12 by this chunk.
- (3): the second Pulse-side gap is **un-retired and sharpened** from "a region, not a named defect" to producer-dependent and localized between OTLP ingest receipt and buffer span-event enumeration, on the `buffer.tick` trio (`span_events_seen` / `observer_invocations` / `fingerprints_computed` all 0 across 15 ticks, `rows_ingested: 1` against `span_count: 15`).
- (4): the windowed-gauge reading instruction no longer names the retired six-occurrence size, and the span-count figures are dated (nine at the 2026-08-14 capture; 15 = 3 warm-up + 12 storm at the 2026-08-15 leg).
- §Established Decisions [Read-Back Dependency Posture] (5): the write-path's `≥5 same fingerprint / 30s` now states that the floor raises a **Suggested** cue and only `≥10` reaches the Autonomous band Tier-1 requires — the duplicate occurrence of the claim (2) retires.
**Why:** The chunk's live leg measured all three. The `evaluate_service_went_silent` gate citation was correct throughout and only its cue-family attribution was wrong, so the doc taught a Pulse-side blocker that measurement disproves — while the gap that IS live had been predicted retired.
**Ref:** .andromeda/runs/2026-08-15T22-39-33-wrap/

## 2026-08-15-canary-spans-pulse-fingerprints — the fingerprint-derivation match claim measured false
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** Was "the fingerprint — computed to match Pulse's derivation — is the fidelity carrier, not a title echo"; retired. The body keeps the fidelity-carrier role (Pulse scrubs titles, so the fingerprint and not a title echo carries fidelity) and states the fingerprint is **NOT** currently computed to match Pulse's derivation, naming both: Conductor computes FNV-1a 64-bit rendered as 16 hex chars over `exception_type` + each frame's `function`; Pulse computes blake3 truncated to 16 bytes over `exception_type` + `\0` + `normalize_stacktrace(stacktrace)` and reads it back as an 8-char hex prefix of the first 4 bytes (Pulse HEAD `d090314`). Equality is impossible by WIDTH alone, so the canary round-trip's final precondition fails BY CONSTRUCTION even when every upstream stage succeeds; aligning the derivations is owned by the successor route entry.
**Why:** The chunk's live leg drove the canary through ingest, buffer append, fingerprinting, Autonomous-tier storm detection and incident formation, and still ended `ready:false` at `canary fingerprint not found in telemetry slice` — isolating the last precondition and making the mismatch measurable for the first time. The claim was unreachable until now: no canary span had ever survived to be fingerprinted.
**Ref:** .andromeda/runs/2026-08-16T08-29-25-wrap/

## 2026-08-15-canary-spans-pulse-fingerprints — the readiness gate's round-trip annotated (duplicate occurrence)
**Section:** §Standard Contracts (Readiness gate)
**Change:** The restated canary round-trip ("assert its fingerprint reads back … to prove the data-dir/workspace wiring end-to-end") now carries the caveat that this assertion cannot currently succeed for a reason that is not a wiring fault — the two derivations differ in algorithm, input and width — so the gate ends `ready:false` at this last precondition even when every upstream stage is proven, and a `canary fingerprint not found in telemetry slice` block must be read as a derivation mismatch rather than as broken data-dir/workspace wiring.
**Why:** A second, independent restatement of the round-trip's proving power; a single-site apply at §Established Decisions would have left §Standard Contracts teaching that a failed round-trip implicates the wiring — the precise misreading the 2026-08-16 leg disproves.
**Ref:** .andromeda/runs/2026-08-16T08-29-25-wrap/

## 2026-08-15-canary-spans-pulse-fingerprints — the second ingest-to-fingerprint gap CLOSED, cause Conductor-side
**Section:** §Occupied Resources (`contracts/pulse-run-contract.toml`)
**Change:** Was "SECOND, independent **Pulse-side** gap … **This gap is NOT closed**"; now the gap is recorded CLOSED (2026-08-16) and its cause reattributed to **Conductor's own side**: Pulse's `spans` table is `PRIMARY KEY (trace_id, span_id)` while Conductor's `ok_span` stamped a CONSTANT `vec![1; 16]` / `vec![1; 8]` identity on every call, so every warm-up span after the first violated the key and was logged-and-skipped — exactly the observed producer-dependence, since `inject_demo`'s per-sequence ids never collide. The shared builder carried the same defect through the scenario dispatcher, not the canary alone. The fixed leg's numbers are recorded (27 `duckdb.append` lines with zero `reject_reason`, trio `12/12/12`, `rows_ingested: 15`, `storms_detected_total: 2` with `severity_hint: "autonomous"` at `occurrence_count: 10`, incident formed), with one explicit limit: why the storm's 12 DISTINCT-id spans also appended zero rows on the prior leg was never observed, so the appender-poisoning reading stays **inferred, not proven**. The telemetry-reading sentence carries both directions — the same cumulative discriminators reading 0 on the broken path and `2`/`1` on the fixed one, with `tracked_fingerprints_count` still sampling 0 on a healthy late tick.
**Why:** The section asserted a Pulse-side gap that was neither Pulse-side nor open, which would have mis-aimed the successor chunk's research.
**Ref:** .andromeda/runs/2026-08-16T08-29-25-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — Stack gains a hashing row
**Section:** §Stack and Technologies
**Change:** New `Hashing / digest` row — blake3 1.8.6 (`blake3 = "1"` in `[workspace.dependencies]`), a NORMAL (non-dev) dep of `conductor-emit`, version-matched to the SUT's own pin under the Pulse-consistency mandate; Conductor's first hashing dependency, pulling `arrayref`/`arrayvec`/`constant_time_eq`/`cpufeatures`.
**Why:** The chunk adopts Pulse's own fingerprint derivation, so the algorithm is the SUT's choice rather than Conductor's; the registry carried no hashing row at all, and a test-scoped dep cannot back a shipped signature.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — Read-Back Dependency Posture reversed (canary carrier)
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** Was: the canary asserts the emitted fingerprint reads back via `retrieve_telemetry_slice.fingerprint_refs`; now it asserts Pulse opened an incident AFTER the storm's emission instant, and reached `ready:true` for the first time on 2026-08-16. Records why the old carrier could never work (the field is fed from the L4 model's `evidence_refs`, pinned `[]` under deterministic L4; Pulse's own fingerprint lands in a `span_events` column no MCP tool reads), that Conductor's fingerprint now IS Pulse's derivation (blake3, 32 hex, first 3 normalized frames), its two identity narrowings, and the causation-in-time limit of freshness. The superseded FNV-1a/width-mismatch rationale is retired from the body.
**Why:** Ratified as a locked-decision reversal (playbook 2026-06-27 rule) — the live SUT's field provenance contradicted the decision's mechanism. The decision's invariants (prove data-dir/workspace wiring before any scenario trusts read-back; never a silent downgrade) hold via the replacement; only the carrier changed.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — Readiness-gate contract re-based (duplicate occurrence)
**Section:** §Standard Contracts (Readiness gate paragraph)
**Change:** The round-trip description moves off the fingerprint assertion onto incident freshness, the by-construction-failure caveat is retired, and a staleness block is documented as "corpus reachable, this run raised nothing" rather than "wiring broken".
**Why:** The same retired claim restated at a second independent site; a single-site apply at §Established Decisions would have left §Standard Contracts teaching an unreachable gate — the duplicate-occurrence lesson applied in the inverse direction.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — third named precondition renamed (duplicate occurrence)
**Section:** §Standard Contracts (the gate's named preconditions are FIVE)
**Change:** The third of the five preconditions changes from "the emitted fingerprint absent from an existing incident" to "no incident opened after the canary storm was emitted"; the COUNT stays FIVE and the arms' ordering is unchanged. Mirrors `NotFound::FingerprintAbsent` → `NotFound::StaleCorpus` in the code.
**Why:** Third restatement of the retired precondition, in an enumeration a prose-only apply would not reach.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-canary-fingerprint-derivation-aligned — deny.toml exceptions no longer Tauri-only
**Section:** §Infrastructure Patterns (Build system)
**Change:** The accepted-license note records that `BSD-2-Clause` entered for `arrayref` (via `conductor-emit`→`blake3`), the first cargo-side exception from outside the Tauri tree, and separates `cargo deny` (green) from `cargo audit` (red on an external advisory-DB fault).
**Why:** The prose attributed every accepted license to the Tauri tree, which this chunk's dependency made false; conflating the two gates' states would also misread the supply-chain posture.
**Ref:** .andromeda/runs/2026-08-16T09-59-45-wrap/

## 2026-08-16-fingerprint-storm-live-proof — RBDP path narrowing corrected to the leading segment
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** The SECOND identity narrowing — was "`normalize_frame` strips absolute paths only, so a RELATIVE-path change is identity-significant" — is RETIRED by measurement and replaced: of a frame's `file` only the **LEADING PATH SEGMENT** is identity-significant, because `is_absolute_path_start` fires on ANY `/` followed by a path char (not merely a leading one) and `skip_absolute_path` then consumes everything from the first slash. So `src/worker.rs` ≡ `src/anything/else.rs` are ONE identity, `other/worker.rs` is another, and a leading `/` erases the segment entirely. The narrowing COUNT stays two; the P-017 clause (c) qualifier now runs through the leading segment, NOT through relative-vs-absolute. Pinned by `exception.rs::only_the_leading_path_segment_reaches_the_preimage`.
**Why:** The planned absolute-path mechanism measured false (base normalized to `at fn (src)`, the absolute variant to `at fn ()`), so the chunk reshaped `FingerprintVariant::PathVariant` to vary the path BELOW its leading segment, adding `RelativePathVariant` for the significant half. Byte-verified identical to Pulse's source at HEAD `d090314`, so this is the SUT's semantics, not a transcription drift.
**Ref:** .andromeda/runs/2026-08-16T14-06-03-wrap/

## 2026-08-17-fingerprint-semantics-token-leading — P-017 narrowings 2 → 1; normalization is token-leading
**Section:** §Established Decisions [Read-Back Dependency Posture]
**Change:** The leading-path-segment narrowing is RETIRED and the clause now states TOKEN-LEADING normalization — `is_absolute_path_start` guarded by `is_token_boundary`, so a relative path is preserved in full and is identity-significant at EVERY depth (`src/worker.rs`, `src/anything/else.rs`, `other/worker.rs` are three distinct identities) while a token-leading absolute path is stripped to nothing (`at handler(/usr/lib/thing.rs:10)` → `at handler()`), which differs from every surviving relative path. The insensitive axes are named as LINE and HEX ADDRESSES. The first-`NORMALIZED_FRAMES = 3` narrowing survives, so the count goes two → one. The pin moves from the retired `exception.rs::only_the_leading_path_segment_reaches_the_preimage` to `::relative_paths_are_significant_at_every_depth` + `::token_leading_absolute_paths_normalize_to_the_same_empty_form`. The transcription citations are de-literalized to function names and re-based from HEAD `d090314` to `efabe8e`.
**Why:** The SUT changed its normalization between the two HEADs, so the leading-segment wording measured FALSE at `efabe8e` (Pulse's own tests, reproduced Conductor-side by the new pinning tests). The clause records what nothing in this repo could have caught: having transcribed the PRE-guard scanner, Conductor's `fingerprint()` returned a different value than Pulse's for every slash-bearing path — including the committed base fixture — while every gate stayed green, because they assert Conductor against Conductor.
**Ref:** .andromeda/runs/2026-08-17T22-07-59-wrap/

## 2026-08-18-error-baseline-spike-live-proof — deterministic-L4 evidence_refs de-vacuumed; envelope sample re-tiered
**Section:** Established Decisions [Read-Back Dependency Posture] · Standard Contracts (readiness gate + run-report envelope sample)
**Change:** Was "fixture pins `evidence_refs` to `[]`", retired at both arch sites — the SUT's deterministic fixture now populates a constant `det-*` triple (measured live 2026-08-18, SUT HEAD `efabe8e`); the freshness-carrier conclusion is unchanged, its supporting fact re-based from emptiness to payload-invariance. The envelope sample re-tiered `<5s` → `<90s`, with a note that the error-baseline-spike family ships declare-only (live rows `verdict: null` / `state: KnownResidual`).
**Why:** The live leg's envelope carried the 3 `det-*` refs; both family TOMLs re-declared `<90s` and retired their checks under the family re-calibration clause.
**Ref:** .andromeda/runs/2026-08-18T19-10-05-wrap/

## 2026-08-18-restart-suppression-live-proof — canary service identity registered
**Section:** §Occupied Resources — Service / process names
**Change:** Registered the two emitted OTLP `service.name` identities: `conductor` (`DEFAULT_SERVICE_NAME`, the scenario dispatcher) and `conductor-canary` (`CANARY_SERVICE_NAME`, the preflight canary's warm-up + storm), with the rationale for the split (preflight runs inside every scenario leg; Pulse keys `persistence_seconds` = cumulative samples and error-rate EWMAs per service).
**Why:** The chunk split the canary off the dispatcher's identity so preflight traffic cannot age a scenario's young-sample suppression window; the wire-visible identity was previously unregistered.
**Ref:** .andromeda/runs/2026-08-18T21-55-43-wrap/

## 2026-08-19-pii-scrub-live-proof — declare-only family note gains pii-scrub
**Section:** Standard Contracts — Run report envelope (per scenario check)
**Change:** The declare-only note now names BOTH families — error-baseline-spike (retired 2026-08-18) and pii-scrub (retired 2026-08-19) — shipping zero [[expected]] checks because no read-back surface can carry them under deterministic L4, their live rows landing verdict: null / state: KnownResidual under the degraded read-back with the live claims graded at the harvest tier.
**Why:** The chunk retired all five pii-scrub checks after the live leg measured the vacuous-Absent / structural-fail-Contains behavior the sources predicted; the note previously named error-baseline-spike only, and this is its sole arch occurrence.
**Ref:** .andromeda/runs/2026-08-19T21-00-45-wrap/

## 2026-08-19-connection-lifecycle-live-proof — declare-only family list re-based to the full six
**Section:** Standard Contracts — Run report envelope (declare-only note)
**Change:** The note now names all SIX declare-only families (fingerprint-storm · error-baseline-spike · latency-regression · restart-suppression · pii-scrub · the connection family's four TOMLs) instead of the two it had grown incrementally.
**Why:** The connection family retired declare-only this chunk (all four `[[expected]]` checks → 0, measured structurally ungradeable — connection state reaches no MCP read-back surface); the incremental two-name list under-stated the standing set.
**Ref:** .andromeda/runs/2026-08-19T23-10-30-wrap/

## 2026-08-19-connection-lifecycle-live-proof — the per-phase fault model + driver registered
**Section:** Conventions — Config conventions · Established Decisions [Validation Library] · Occupied Resources — Crate names · Infrastructure Patterns — directory tree
**Change:** Registered the optional `[phases.fault]` declared-data table (`PhaseSpec.fault: Option<FaultSpec>`, closed `FaultKindSpec::PortOccupier`, garde dive, fault ⇒ occurrences 0 via `fault_phases_are_silent`); the cross-field-invariant enumeration gains the silence rule (field-level one altitude up, like `no_duplicate_pids`); conductor-run's registry entry + tree comment gain the fault-phase occupier guard and the new `conductor-run → conductor-faults` edge (conductor-faults' first consumer).
**Why:** The chunk shipped the port-occupier driver as declared phase data over the guard-generic timeline hook; the config surface, the invariant and the crate edge were unregistered.
**Ref:** .andromeda/runs/2026-08-19T23-10-30-wrap/

## 2026-08-20-latency-regression-re-proof — the second route to KnownResidual, and the rate term's real basis
**Section:** §Established Decisions [Read-Back Dependency Posture] · §Standard Contracts — Run report envelope · §Occupied Resources — `contracts/pulse-load-envelope.toml`
**Change:**
- [Read-Back Dependency Posture]: recorded the shipped read-back routing carve-out — after a GREEN preflight, an EMPTY active list routes a declare-only scenario (zero `[[expected]]`) to the pre-accepted auto-resolve residual (`KnownResidual` / `verdict: null`) instead of `Blocked`, because an emptied active list is Pulse's own auto-resolve lifecycle and a scenario grading nothing cannot pass falsely on an empty observation; checks-bearing scenarios and every read-back call failure still land `Blocked`. Names the mechanism (`route_read_back` / `ReadBack {Graded, AutoResolved, Blocked}`, crate-private in `conductor-run`) and the honest limit — the arm is unit-pinned only and did NOT fire on the 2026-08-20 live leg.
- Run report envelope: the six declare-only families' landing clause now names BOTH routes to `verdict: null` / `state: "KnownResidual"` (the degraded read-back, or the auto-resolve residual on an empty post-green-preflight read-back), was attributed solely to the degraded read-back; a checks-bearing scenario and every call failure still land `Blocked`. The six-family list is unchanged.
- Load envelope: `max_sustained_rate_spans_per_s` is computed `occurrences / gap_ms` — DISPATCHES per second, not wire spans — so a `Latency`/`Ramp` phase multiplies it by `samples`/`windows` (50 wire spans/s counted as 1/s at the shipped `latency-regression` shape, a 50× divergence, still ~200× under the bound); no verdict moves and the gate/caption shared basis is unaffected, but the term as named misdescribes what it bounds. Marked SURFACED-not-authored, the fix owned by a working-route entry.
**Why:** The carve-out was operator-ratified and shipped, while arch enumerated `KnownResidual` as the degraded/accepted-fingerprint residual only and routed every empty observation to `Blocked`; the envelope restated the same wording, so a single-site apply would have named one mechanism after a second shipped. The re-shape widened the rate divergence 50×; the fix (count `occurrences × samples`, or rename the term) stays owned rather than smuggled into a doc edit.
**Ref:** .andromeda/runs/2026-08-20T17-10-10-wrap/
## 2026-08-21-severity-lifecycle-live-proof — the AutoResolved arm fired live, and the declare-only registry grew to seven
**Section:** §Established Decisions [Read-Back Dependency Posture] · §Standard Contracts — Run report envelope (per scenario check)
**Change:**
- [Read-Back Dependency Posture]: `route_read_back`'s `AutoResolved` arm, was "unexercised live so far", first fired LIVE on 2026-08-21 (`ack-cooldown`): a scenario forming no incident of its own leaves only the preflight canary's, whose cues are `curious`, and the Tier-1 coordinator accepts Autonomous alone, so nothing refreshed it; it auto-resolved on schedule and left ~4 minutes of empty active list before read-back (`query_incident_list` → `result_count: 0`, envelope `fingerprints: []`).
- Run report envelope: the declare-only family registry reads SEVEN, adding the severity-lifecycle family (`incident-auto-resolution` · `severity-tier-autonomous` · `severity-tier-suggested` · `severity-tier-curious` · `ack-cooldown`, 2026-08-21).
**Why:** The 2026-08-20 leg inferred the list could not empty because the canary's error-rate cues kept refreshing its incident; measurement retires the inference while keeping the observation — cues refresh an incident only through a digest of the SAME identity, which a `curious` cue never triggers. The arm was exercised, not modified (zero production-source delta). All five family TOMLs now carry zero `[[expected]]`, each retirement against a measured ground (the det-L4 fixture pins one severity and no corpus tool renders a tier word; `query_incident_list` is active-only so a resolved incident leaves the surface; `CountAtLeast` grades `span_refs` the incident producer writes empty; no ack tool exists in the four-tool contract); all five live rows landed `verdict: null` / `state: KnownResidual`.
**Ref:** .andromeda/runs/2026-08-21T09-50-00-wrap/

## 2026-08-21-per-check-latency-measurement — runs.db is three tables
**Section:** Occupied Resources -> On-disk artifacts (runs.db)
**Change:** Was "two tables"; `runs.db` is now registered as THREE tables — `runs` (scenario grain, 11 columns, PK `(run_id, scenario)`), `run_envelope` (run grain), and NEW `run_check`, the per-check index keyed `(run_id, scenario, check_index)` with `latency_ms`/`deadline_ms` NOT NULL and `budget_ms` NULL on inherit; blocked/declare-only scenarios write zero rows.
**Why:** The chunk landed the `run_check` table, leaving the new persistence resource unregistered under the two-table wording.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — ORM table roster + per-check-index label
**Section:** Established Decisions -> [ORM] None — raw SQL
**Change:** Table roster updated to three, and the 'per-check index' label moved off `runs` onto `run_check`, whose grain it now actually is.
**Why:** The same roster is restated here; a single-site apply would have left the two-table claim and the mis-attributed label alive.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — CheckRecord registered as a second shared shape
**Section:** Standard Contracts -> Run report envelope
**Change:** Registered the per-check `CheckRecord` (9 keys) as a SECOND shared artifact shape beside the eleven-field envelope, riding the JSONL journal, the `run_check` table and the Markdown report's indented detail line; the envelope is byte-unchanged, and blocked/declare-only scenarios emit zero check records.
**Why:** Standard Contracts declares itself the registry of shared shapes and carried only the envelope; the chunk added a second one that three surfaces depend on.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — budget_ms registered as declarable config
**Section:** Conventions -> Config conventions
**Change:** Registered the optional `[[expected]].budget_ms` key (integer ms, `#[serde(default)]`, garde `range(min = 1, max = MAX_BUDGET_MS)` with the ceiling DERIVED from `SloTier::Tier90s.deadline_ms()`), cross-checked against the scenario's own `slo_tier` at load by `Scenario::check_budgets()`; no committed TOML declares one.
**Why:** This paragraph enumerates the declarable scenario-config surface; the chunk's new external input appeared nowhere in it.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Nullability qualified per table
**Section:** Conventions -> Data model conventions (SQLite / runs.db)
**Change:** Nullability qualified PER TABLE: `runs.latency_ms` stays NULL-for-blocked, while `run_check.latency_ms`/`deadline_ms` are NOT NULL (an ungraded check emits no row at all) and `run_check.budget_ms` is NULL on inherit.
**Why:** The unqualified 'NULL for blocked rows' claim contradicted the new table's NOT NULL columns once Occupied Resources registered it.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Sibling-spanning invariants cannot be garde validators
**Section:** Established Decisions -> [Validation Library] serde 1.0.x + garde 0.22.1
**Change:** Recorded the THIRD route and its structural boundary: the one-altitude-up `custom` covers only invariants contained WITHIN the lifted field, because a field-level `custom` receives `(&field, &())` and sees no SIBLING; a sibling-spanning invariant ships as a load-path method invoked from `from_toml_str` (`check_capabilities`, now joined by `check_budgets`).
**Why:** The plan's `#[garde(custom)]`-on-`Scenario::expected` design measured structurally impossible; the decision enumerated only the Context pattern or one-altitude-up, neither of which the shipped rule uses.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Stack validation row narrowed
**Section:** Stack and Technologies -> Validation row
**Change:** Was: garde carries all cross-field scenario-config invariants; the row is narrowed to the garde-EXPRESSIBLE cross-field invariants, naming the load-path `Scenario::check_*()` route for sibling-spanning ones.
**Why:** The Stack table restated the retired claim, which the [Validation Library] amendment corrects.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Load-error mapping is three-way
**Section:** Established Decisions -> [Scenario Config Format] TOML on disk
**Change:** Load-error mapping corrected from two-way to THREE-way: `CoreError::Config` on a parse failure OR a non-garde load-path check failure, `CoreError::Validation` only on a garde failure (it is `#[from] garde::Report` and cannot carry a hand-written message).
**Why:** The chunk's budget fault raises `CoreError::Config`, contradicting the two-way mapping as written.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-per-check-latency-measurement — Tier deadline recorded as a ceiling
**Section:** Established Decisions -> [Timing-Tolerance Model]
**Change:** The tier deadline recorded as a CEILING: a check may declare sub-tier `budget_ms` and is graded against a per-check effective deadline; `evaluate_slo` takes `deadline_ms`, `SloOutcome` carries it, and every check persists its own latency/deadline/verdict. Because the corpus is observed ONCE per scenario, per-check latencies are equal by construction and the DEADLINE is what separates their verdicts.
**Why:** The decision defined the deadline solely from the tier; the chunk proved the sub-tier budget round-trips and separates two checks sharing one observation instant.
**Ref:** .andromeda/runs/2026-08-21T11-05-00-wrap/

## 2026-08-21-delegated-timing-budgets-proven — Declare-only family count SEVEN→EIGHT
**Section:** Standard Contracts -> Run report envelope
**Change:** Count raised from SEVEN to EIGHT and the delegated-timing family registered (`halo-hue-encoding` · `service-constellation-discovery` · `report-render-surface`, 2026-08-21), each zero `[[expected]]` landing `KnownResidual` with its live claim graded at the harvest tier; `findings-counter-refresh` explicitly excluded because it carries one `[[expected]]`.
**Why:** The chunk live-proved the three as declare-only harvest-graded rows, making them an eighth family by that sentence's own definition, and this is the only arch section enumerating them.
**Ref:** NOT DERIVED

## 2026-08-21-delegated-timing-budgets-proven — Checks-bearing ⇒ Blocked narrowed to the auto-resolve route
**Section:** Established Decisions -> [Read-Back Dependency Posture]
**Change:** The confinement now binds the auto-resolve/empty-active-list route ALONE: a checks-bearing scenario is excluded from THAT route (an empty list leaves it `Blocked`) and every read-back call FAILURE still lands `Blocked` unconditionally — but the degraded-read-back route is NOT gated on declare-only, so a checks-bearing scenario reaches `KnownResidual` through it, an unmet `CountAtLeast` floor grading `CalibrationRegion` rather than failing.
**Why:** Measured false as written: `findings-counter-refresh` carries one `[[expected]]` and landed `verdict: CalibrationRegion` / `state: KnownResidual`, not `Blocked`.
**Ref:** NOT DERIVED

## 2026-08-21-delegated-timing-budgets-proven — Duplicate Blocked claim narrowed in step
**Section:** Standard Contracts -> Run report envelope (closing sentence)
**Change:** Restated so the unconditional `Blocked` guarantee covers read-back call failures only, while a checks-bearing scenario is excluded from the auto-resolve route alone and can still reach `KnownResidual` via the degraded read-back, cross-referencing [Read-Back Dependency Posture].
**Why:** The same retired claim was restated here, in a paragraph spanning BOTH routes to `KnownResidual`, so a single-site apply would have left it standing as a global guarantee.
**Ref:** NOT DERIVED

## 2026-08-22-operator-pause-and-checklist-live-firing — the `[[checklist]]` scenario-config key + the third load-path check
**Section:** Conventions → Config conventions · Established Decisions [Scenario Config Format] · Established Decisions [Validation Library]
**Change:** Registered `[[checklist]]` — an array-of-tables of `ChecklistItem { induced, observation }` (`#[serde(default)]`, garde `dive`, both halves `length(min = 1, max = MAX_CHECKLIST_TEXT)` with `MAX_CHECKLIST_TEXT = 200`), carried on `Scenario` and on `HoldPoint`, declared by exactly two committed scenarios (`halo-hue-encoding`, `halo-breathing-encoding`). Widened BOTH closed enumerations the new sibling-spanning rule joins: the [Scenario Config Format] `CoreError::Config` arm now reads `check_budgets`, `check_capabilities`, `check_checklist`, and [Validation Library]'s third-route list names `Scenario::check_checklist` beside the two it already carried.
**Why:** The chunk shipped a new declarative scenario-config surface and a third load-path `check_*()`; arch's config-key registry and its two closed method enumerations had no entry, so a shipped surface was unregistered and two lists read as complete while being false.
**Kept:** The `budget_ms`-scoped "No committed scenario TOML declares one" sentence — it remains true.
**Ref:** .andromeda/runs/2026-08-22T12-15-00Z-wrap/

## 2026-08-31-p-075-assert-round — Payload fidelity disproved on a second axis; runtime-STATE fidelity recorded as the stronger claim that DOES exist
**Section:** Established Decisions [Read-Back Dependency Posture] → the Honest-limit sentence
**Change:** Retired "under deterministic L4 no stronger claim exists" and the "concurrent unrelated incident inside the poll window" caveat. Added: the SECOND payload axis (`incident_events` has ZERO references in `crates/mcp-server`, written by triage, read only corpus-side — reaches no MCP tool at any width, Pulse HEAD `83d4060`); runtime-STATE fidelity as the stronger claim, live-proven (incident 6 resolved, active set emptied, `idle_seconds_at_resolve = 0.0` against a 120s idle threshold, corpus-recorded 45s life); the one-active-incident dedupe constraint (the producer dedupes against any OPEN incident regardless of fingerprint), which both forecloses a spared-control design and makes the concurrent-incident caveat unreachable; and the DECLINED arm's permanent stub-only status (monotonic-timestamp guard).
**Why:** The chunk measured all four facts first-hand against a live Pulse at HEAD `83d4060`.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-08-31-p-075-assert-round — Payload-fidelity carrier restated on both axes at the twin site
**Section:** Standard Contracts → Readiness gate (the "carrier is freshness, not payload identity" sentence)
**Change:** Extended the carrier sentence from a single axis to name BOTH measured axes (`span_events` unread + `incident_events` unreachable) and to record that runtime-STATE fidelity is attainable where payload identity is not.
**Why:** Duplicate occurrence of the single-axis claim retired in [Read-Back Dependency Posture]; the duplicate-occurrence precedent requires both move together or the retired reading survives in the twin.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-08-31-p-075-assert-round — Workspace-key divergence mechanism is now the published-key FALLBACK
**Section:** Standard Contracts → Readiness gate (app/sidecar workspace-key precondition)
**Change:** Was "the sidecar keys its query on `ANDROMEDA_PULSE_DATA_DIR` while `pulse-app` keys incidents on its detected workspace root"; now the measured mechanism: `pulse-app` PUBLISHES its key to `{data_dir}/run/workspace-key` and the sidecar reads it (`read_published_workspace_key`), falling back to `data_dir` only when that file is absent or invalid. Measured 2026-09-01: the published key matched the incidents' stamped `workspace` byte-for-byte and no divergence occurred.
**Why:** A SUT-side mechanism change, read first-hand at HEAD `83d4060`; the divergence is now the fallback case rather than the default, which changes how an operator should diagnose a zero-row read-back.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-08-31-p-075-assert-round — `workspace` column = data_dir qualified (third occurrence)
**Section:** Occupied Resources → Environment variables, `ANDROMEDA_PULSE_DATA_DIR`
**Change:** Qualified the `= data_dir` equality: the filter value is the published workspace key when present and `data_dir` only as fallback. The propagation obligation and the empty-`query_incident_list` failure mode stand unchanged.
**Why:** Third restatement of the retired sidecar-keys-on-data_dir claim; without moving it the corrected mechanism would survive at only two of three sites.
**Ref:** .andromeda/runs/2026-09-01T16-50-00Z-wrap/

## 2026-09-01-webview-self-verify-windows-host — the webview drive path, and the CARRY's tuple re-scope
**Section:** Established Decisions [Read-Back Dependency Posture] · Occupied Resources (Ports · Frontend asset subtree · `logs/conductor-tauri.jsonl` · Environment variables) · Infrastructure Patterns (Build system) · Cross-cutting Patterns (Trust boundary · Scope law)
**Change:**
- (1) Was "At most ONE incident is active per workspace"; re-scoped to "per DEDUPE TUPLE", the `(kind, scope, scope_id)` predicate (Pulse HEAD `83d4060`); "spared-control unattainable" and the freshness-caveat retirement both narrowed to WITHIN-tuple, with a cross-scope two-incident control recorded as a weighable route option, never a retirement. The originating leg's evidence stands — only the generalisation was unlicensed.
- (2) Registered `CONDUCTOR_MSEDGEDRIVER` (harness-only driver handle, skip-at-exit-0 guard) and the dev-only harness-lifetime ports `4444`/`4445`.
- (3) The `custom-protocol` FEATURE — not the profile — decides bundle embedding (`tauri` 2.11.3 `build.rs`: `let dev = !custom_protocol`, read back via `DEP_TAURI_DEV`), as measured.
- (4) Trust boundary's "only case where Conductor opens a port" scoped to SHIPPED binaries.
- (5) Scope law's "no UI automation" scoped to PULSE's UI.
- (6) `conductor-tauri.jsonl`'s CWD-relative landing site + `ui/logs/` added to the ignored enumeration.
**Why:** The chunk drove the real Tauri window for the first time, which measured three arch claims incomplete or over-scoped and landed two genuinely new resources. The CARRY was owed from `2026-08-31-p-075-assert-round`. The scope-law and port-ban qualifications were escalated and operator-ratified.
**Ref:** .andromeda/runs/2026-09-01T18-49-38Z-wrap/

## 2026-09-01-desktop-a11y-sweep — webview a11y leg is two arms; self-obs landing site moved; sidecar PATH recorded
**Section:** §Occupied Resources — Ports · Service/process names · Frontend asset subtree · On-disk artifacts (`conductor-tauri.jsonl`) · Environment variables (`CONDUCTOR_MSEDGEDRIVER`) · §Cross-cutting Patterns — Trust boundary · Scope law
**Change:**
- (1) The `4444`/`4445` driver stack, the trust-boundary spawn note, the scope-law self-verify clause and the `CONDUCTOR_MSEDGEDRIVER` handle now name BOTH arms — the unattended `--e2e` routine arm and the operator-local `npm run a11y:driven` driven arm — over ONE WebdriverIO + tauri-driver stack.
- (2) `logs/conductor-tauri.jsonl` lands at the WORKSPACE ROOT under the tauri-driver `cwd: repoRoot` spawn, covered by the root-anchored ignore rule; `crates/conductor-tauri/ui/logs/` is the retired pre-2026-09-01 site.
- (3) The sidecar's fixed-NAME-through-`PATH` resolution is recorded, with `PATH` named as a spawn-resolution input.
**Why:** (1) The chunk added a second suite to the same `wdio.conf.ts`; a single-arm attribution would leave the driven arm's identical listeners unregistered. (2) The landing site moved with the `cwd` spawn, as measured. (3) With the sidecar off `PATH`, preflight returns BLOCKED in ~2ms with all four tools `absent`, at row level indistinguishable from a genuine SUT-side gate failure.
**Ref:** .andromeda/runs/2026-09-01T22-22-12Z-wrap/

## 2026-09-01-live-per-p-id-verdict-lamps — self-obs sink landing site is per-arm
**Section:** Occupied Resources -> On-disk artifacts (`logs/conductor-tauri.jsonl`) + Occupied Resources -> Frontend asset subtree
**Change:** Was: the Tauri backend's self-obs stream has ONE landing site; now it is per-arm. `tauri_log_path()` resolves it as `runs_dir.parent()/logs`, so the routine `--e2e` arm's new `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` moves it to `runs/logs/conductor-tauri.jsonl`, while the operator-local `a11y:driven` arm leaves the handle unset and still lands at the workspace root. Both sites git-ignored; the frontend-subtree parenthetical restating the single-site verdict was qualified to match.
**Why:** The chunk gave the routine arm a seeded fixture runs dir, and the sink follows the handle — measured on disk, not derived.
**Ref:** .andromeda/runs/2026-09-02T00-58-00Z-wrap/

## 2026-09-02-screen-reader-manual-spec — `CONDUCTOR_NVDA`, three suite families over the one stack, the per-suite self-obs landing site
**Section:** Occupied Resources -> Ports (`4444`/`4445`) + On-disk artifacts (`logs/conductor-tauri.jsonl`) + Environment variables (`CONDUCTOR_MSEDGEDRIVER`, new `CONDUCTOR_NVDA`) + Cross-cutting Patterns -> Trust boundary + Scope law
**Change:**
- Registered `CONDUCTOR_NVDA` — the second host dev-tool handle, read only by `wdio.conf.ts`, validated and spawned like `CONDUCTOR_MSEDGEDRIVER`, skip-at-exit-0 when unset, value never committed. Was "one HOST dev-tool handle"; now two.
- The webview a11y legs are THREE suite families (four `wdio` suites) over the ONE stack (was two families): the `sr*` screen-reader leg joins the routine and driven arms in the Ports bullet, the `CONDUCTOR_MSEDGEDRIVER` bullet, the trust boundary (NVDA + a fixed-argv PowerShell activation script as leg-spawned children, no listener) and the scope law; "never a second automation stack" unchanged.
- The self-obs landing site is per-SUITE: `runs/logs/` (routine + `sr-empty`), `runs/driven/logs/` (driven), `runs/sr-leg/logs/` (`sr` / `sr-error`). The unset-handle root site stays the mechanism, retired only as an a11y landing site.
**Why:** `CONDUCTOR_NVDA` is the chunk's one new resource; the suite-family count moved 2 -> 3 and is restated at five sites. The chunk moved the driven arm's `CONDUCTOR_RUNS_DIR` to `runs/driven/runs`, so the earlier claim "the driven arm leaves the handle unset" is retired by construction, not by a mechanism change.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-02-cross-surface-envelope-parity — `CONDUCTOR_E2E_SEED_DIR` registered as the third handle class
**Section:** Occupied Resources — Environment variables
**Change:** An eleventh bullet in the reserved `CONDUCTOR_*` enumeration, after `CONDUCTOR_NVDA`: `CONDUCTOR_E2E_SEED_DIR`, the repo-relative fixture runs dir the webview `--e2e` arm seeds into — SET by `wdio.conf.ts` `onPrepare`, READ only by `crates/conductor-run/tests/envelope_fixture.rs`, never by a shipped binary, a no-op when unset. Recorded as the THIRD handle class: unlike `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` it names a path this document defines rather than a HOST dev-tool.
**Why:** The chunk's only new resource, absent from the enumeration. Operator-ratified as a genuinely new class rather than an instance of playbook rule 115, whose preconditions it fails on four of five clauses.
**Kept:** The "One of TWO handles … naming a HOST dev-tool" clause stays true and was left standing.
**Ref:** .andromeda/runs/2026-09-02T14-34-37Z-wrap/

## 2026-09-03-live-pulse-preconditions-probed — preconditions probe registered; run contract at six terms
**Section:** §Stack (CLI argument parsing) · §Standard Contracts (Liveness equivalent) · §Occupied Resources (Environment variables ×3, `contracts/pulse-run-contract.toml`)
**Change:**
- The `:4317` liveness check has TWO production callers with deliberately different dispositions: the timeline engine's pre-emission check surfaces a refusal as `Result::Err`; the scheduling-time `observe_preconditions` probe reports it as an unmet precondition and a non-zero exit, never an `Err` and never a verdict.
- `CONDUCTOR_PREFLIGHT_TIMEOUT`: `boot` runs `conductor preconditions` as a LEADING arm and short-circuits, so the preflight invocation and its timeout are SKIPPED rather than paid.
- The run contract's `[[term]]` list is at SIX (gaining `mcp-enabled`, `shell-declaration`, `env = ANDROMEDA_PULSE_MCP_ENABLED`); the observable set is TWO handles. `ANDROMEDA_PULSE_L4_DETERMINISTIC` loses its exclusive article; `ANDROMEDA_PULSE_MCP_ENABLED` gains its read role.
- `sidecar-built`'s `asserted` rationale is recorded as measurably FALSE (the `warmup_ms` failure shape), with the deliberate non-re-classification noted.
- The §Stack clap row's three-verb literal is de-literalized to the set `Commands` declares.
**Why:** The chunk landed a non-mutating precondition probe and `boot`'s leading arm.
**Kept:** The CLI VERB itself is not registered in arch — playbook dismisses that as over-reach (no port/socket/endpoint/IPC/event/env-var/crate added); its home is layout-templates §cli Primary screens.
**Ref:** .andromeda/runs/2026-09-03T19-20-00-wrap/
## 2026-09-04-preconditions-probe-reads-path-handles-by-presence — the probe that can now say yes
**Section:** §Standard Contracts — Liveness equivalent (primary) · §Occupied Resources — `CONDUCTOR_PREFLIGHT_TIMEOUT` · §Occupied Resources — `ANDROMEDA_PULSE_MCP_ENABLED`
**Change:** `observe_preconditions` now builds `declared` PER HANDLE through `conductor_core::handle_declared` — the PATH-valued `ANDROMEDA_PULSE_DATA_DIR` graded by presence-after-trim, every other name delegating to the value-only `conductor_core::flag_declared` — so all three subjects are meetable and a non-zero exit names a genuinely unmet subject again (was: UNSATISFIABLE — no subject could ever be met, the probe permanently unmet and short-circuited before every preflight).
- `CONDUCTOR_PREFLIGHT_TIMEOUT`: the skip is CONDITIONAL (was unconditional); the timeout is reached and paid (~47s per leg = warm-up 45s + poll).
- `ANDROMEDA_PULSE_MCP_ENABLED`: the CHECK is per-handle by kind; `=false` leaves `handles-declared` naming that handle alone.
- The name-only REPORTING / redaction property is unchanged throughout.
**Why:** The fix the document had named route-owned shipped in this chunk. Measured: `conductor preconditions` exit 0 printing `[PRECONDITION] every live-Pulse precondition is satisfied`, then `ReadyState` with `ready: true` / `canary_round_trip: "ok"` / `blocked_precondition: null` / 4-of-4 tools / `data_dir: "<redacted>"` in BOTH shipped shells (`agent-run.sh`, `agent-run.ps1`), zero `skipped preflight` lines, neither script edited.
**Supersedes:** 2026-09-04-sr-findings-remediation — the precondition probe cannot exit 0 (path handle graded by a boolean)
**Ref:** .andromeda/runs/2026-09-04T17-15-00-wrap/

## 2026-09-04-sidecar-spawn-without-a-console-window — the spawn stops naming a type it no longer uses, and stops promising a build it cannot do
**Section:** §Occupied Resources — Service/process names (primary) · §Cross-cutting Patterns — Trust boundary · §Infrastructure Patterns — Directory structure · §Established Decisions [Module Boundaries]
**Change:**
- The sidecar launch mechanism was the retired rmcp `TokioChildProcess`; now `conductor-verify/src/spawn.rs::build_command`'s `tokio::process::Command`, handed to `ReadbackClient::connect_command` over piped stdio, carrying the `CREATE_NO_WINDOW` creation flag under `#[cfg(windows)]`.
- §Trust boundary's restatement of the retired name corrected identically.
- The directory tree registers the new root `rustfmt.toml` (one key, `edition = "2024"`).
- [Module Boundaries] keeps its compiler-enforced forbidden-edge property unqualified but QUALIFIES the standalone per-seam BUILD claim, which measurement falsified.
**Why:** The chunk shipped the console-suppression fix (SR row S1-01 lost the `<host-path>` / `terminal blank` / `pane` utterances; 0 `security_finding` rows), leaving the document naming a type absent from every crate `src/` since 2026-06-27. The [Module Boundaries] qualifier rests on a separate, PRE-EXISTING, route-owned (*Dependency polish*) defect: `cargo check -p conductor-verify --lib` red on `tokio::time::sleep` with tokio's `time` feature only in `[dev-dependencies]`. Escalated and resolved by the operator in favour of qualifying the body now while the CARRY owns the fix. A decisions-log record is annotated, never rewritten.
**Kept:** The `tokio 1.48.x` version claim (3 sites) — stale and pre-existing, but not this chunk's drift (no dependency added or bumped).
**Ref:** .andromeda/runs/2026-09-04T20-15-00-wrap/

## 2026-09-05-audit-corrective — the supply-chain red was local, not an external advisory-DB fault
**Section:** Infrastructure Patterns — Build system
**Change:** Was "`cargo audit` is separately red on an external advisory-DB fault"; now BOTH runners green over the un-drifted lock — `cargo deny` exit 0, `cargo audit` exit 0 (1239 advisories, 564 packages, 18 `deny.toml`-adjudicated allowed warnings, measured 2026-09-05 against advisory-db HEAD `5a0ebedf` with `git status --porcelain` empty). The real cause is named: an untracked `crates/gettext-sys/RUSTSEC-2026-0244.md` left in this host's clone after upstream moved the file to `crates/gettext-rs/` on 2026-08-09, which a fetch into an existing copy never removes. The porcelain check now precedes classifying any parse failure as external.
**Why:** The chunk MEASURED the claim false, and its own probe closes the standing deferral.
**Kept:** The `toml`-crate "audit/deny-clean" lines — they assert a per-crate pass, not the gate state.
**Ref:** .andromeda/runs/2026-09-05T21-09-11Z-wrap/

## 2026-09-06-operator-gated-live-suite — the deterministic-L4 degraded universal retired, and the live-suite capture site registered
**Section:** §Established Decisions [Read-Back Dependency Posture] · §Occupied Resources — On-disk artifacts / database · §Infrastructure Patterns — Directory structure
**Change:**
- [Read-Back Dependency Posture]: was "under deterministic L4 every read-back returns `degraded_mode`"; now `degraded` is a PER-READ-BACK property, deterministic L4 measured BOTH ways (`findings-counter-refresh` degraded; `degraded-mode-report` non-degraded → the graded route → `ManualCheck`). Only the quantifier was withdrawn: the degraded route to `KnownResidual`, its ungated-on-declare-only property and `findings-counter-refresh`'s outcome are unchanged.
- §Occupied Resources gains `runs/live-suite/{leg}.jsonl`, the `run --live` per-leg self-obs captures: harness-owned by `scripts/agent-run.{sh,ps1}`, deliberately no `CONDUCTOR_*` handle of its own, moves with `CONDUCTOR_RUNS_DIR`, git-ignored, LEG-stemmed, cleared per invocation by a non-recursive `rm -f …/*.jsonl`.
- The `runs/` directory-tree gloss narrowed: "run_id-stemmed, never overwritten" scopes to the journal + report, with `live-suite/` the one exception.
**Why:** `state_for` returns `KnownResidual` iff `observation.degraded`, so a `ManualCheck` row proves a non-degraded read-back under deterministic L4. The capture site is a permanent artifact the chunk's harness now writes; the tree-gloss fix is its duplicate-claim site, which a single-site apply would have left standing.
**Kept:** Other `every read-back …` claims (architecture's read-back-failure rule, security-plan's unreachable path) state DIFFERENT claims and stand.
**Ref:** .andromeda/runs/2026-09-06T09-37-04-wrap/

## 2026-09-06 0-pending adaptation (subject: `2026-09-06-operator-gated-live-suite`) — the AutoResolved arm's firing condition is uptime-bound
**Section:** §Established Decisions [Read-Back Dependency Posture] (the leg-E first-fired-LIVE narrative)
**Change:** The leg-E narrative is QUALIFIED, not retired. "Nothing refreshed it" holds only while Pulse is inside the emitting service's ONE-HOUR bootstrap window (`BOOTSTRAP_WINDOW_SECONDS = 3_600`): past it the silence evaluator raises `service_went_silent` cues for exactly the quiet the arm depends on, those cues reach the Autonomous band and create NEW incidents during the silent phase, so the active set is never empty and the arm is unreachable at ANY window length. Inside the window the arm is reachable but still bounded by 120s idle + up to a full 30s observer tick measured from the LAST preflight incident, which can be more than one.
**Why:** A 0-pending adaptation applying a fact this session measured, not drift-derived: the day's first emitted silence cue fired one hour after the canary service's first span, and incidents then formed mid-silence (SUT HEAD `83d4060`). The same section had retired the deterministic-L4 degraded universal hours earlier; without this qualification the passage would read as an unqualified account of when the arm fires.
**Kept:** The route-trigger definition ("the auto-resolve residual when a post-green-preflight read-back finds an empty active list") remains true and is not a duplicate. `scenarios/auto-resolve-idle-window.toml`'s header still states the superseded margin model — CODE, out of scope, owned by the CARRY pinned to *Halo hue budget re-driven*.
**Ref:** NOT DERIVED

## 2026-09-06-run-report-envelope-conformance-gate — the storage seam's first row-removing path
**Section:** Established Decisions — [ORM] None — raw SQL
**Change:** The workload description was "append + a handful of cross-run SELECTs"; now also one run-scoped teardown: `RunsDb::delete_run`, three LITERAL `DELETE ... WHERE run_id = ?1` statements over `runs` / `run_check` / `run_envelope` in one transaction, rusqlite bound parameters, idempotent, never `format!`-assembled even over a hard-coded table list. The `conductor cleanup <run_id>` verb is its sole caller; the harness shells issue no SQL.
**Why:** The workload characterisation IS the payoff argument for raw SQL, and a three-table transactional delete is precisely the shape that would otherwise argue for an ORM — the entry under-described what ships. Routine: spec illustration reconciled to the sound shipped impl, invariant intact.
**Kept:** The new CLI verb is not registered — the verb set is "whichever `Commands` (`cli.rs`) declares, never a literal list here".
**Ref:** .andromeda/runs/2026-09-06T13-07-09-wrap/

## 2026-09-06-coverage-completeness-gate — rate term counts wire records; `.gitattributes` registered
**Section:** §Occupied Resources → On-disk artifacts → `contracts/pulse-load-envelope.toml` · §Infrastructure Patterns → Directory structure + Build system
**Change:**
- The DISPATCHES reading of `max_sustained_rate_spans_per_s` is retired. Now: `phase_rate_exceeds` judges `occurrences × EmissionSpec::max_spans_per_dispatch() × 1000 > max_rate × gap_ms`, an upper BOUND (rate curves carry seeded jitter; the static gate has no seed), with the per-dispatch count measured per `EmissionShape` arm. The retired `samples`/`windows` reading held for `Latency` alone. The "~200× under the bound" figure is replaced by the measured ~232 records/s at `halo-breathing-encoding` (≈43×). The contract file and the term's name/value are unchanged. The SURFACED-not-authored disposition is retired as authored by this chunk. The gate/caption shared-basis property holds BY CONSTRUCTION via the single `phase_rate_exceeds` → `phase_breach` → {`check_load_envelope`, `classify`} call chain.
- `.gitattributes` registered in the directory tree beside `rustfmt.toml`, plus a Build-system sentence for the gate-correctness dependency.
**Why:** The chunk shipped the fix this record had scoped as route-owned-not-shipped. `.gitattributes` was re-homed from §Occupied Resources (runtime artifacts) to the directory tree, where its siblings (`Cargo.lock`, `rust-toolchain.toml`, `rustfmt.toml`) live.
**Ref:** .andromeda/runs/2026-09-06T15-59-32-wrap/

## 2026-09-06-halo-hue-budget-re-driven — ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS registered
**Section:** Occupied Resources — Environment variables
**Change:** Registered the Pulse-side per-service cold-start window handle — the one entry Conductor neither SETS nor READS — because a SHIPPED artifact names it in its own output (the `run --live` banner in both shells). Recorded as a BOOT-TIME posture (`Thresholds::from_env()`, SUT HEAD 83d4060), hence boot-wide rather than per-leg; load-bearing for the auto-resolve leg and inert for the storm-path legs; explicitly neither a run-contract `shell-declaration` term nor one of the three `conductor preconditions` subjects.
**Why:** Arch's env registry carried only handles Conductor reads, so a reader meeting the name in the banner landed nowhere. Escalated (a registry of read handles gaining a never-read one) and operator-approved, with a bounding playbook rule minted: only a handle a SHIPPED artifact names is registered, never one mentioned solely in a report or plan.
**Ref:** .andromeda/runs/2026-09-07T08-32-30-wrap/

## 2026-09-06-halo-hue-budget-re-driven — The auto-resolve arm's uptime bound is the window in force at boot, not a fixed hour
**Section:** Established Decisions — [Read-Back Dependency Posture]
**Change:** The UPTIME-BOUND paragraph is qualified: `BOOTSTRAP_WINDOW_SECONDS = 3_600` is the DEFAULT, overridable once at Pulse's boot, so the arm's reachability is bounded by the window IN FORCE AT BOOT (was: a fixed hour). The past-the-window mechanism is unchanged and scoped to the default window. The stretched posture removes cause (a) and is NOT sufficient — cause (b) is untouched and still decides the leg — so the body says the posture makes the arm REACHABLE rather than RELIABLE, and records both 2026-09-07 runs.
**Why:** Dependent of the handle registration: registering it alone would leave this paragraph telling a reader the arm must race a fixed hour, which the 2026-09-07 leg measured false. Same claim, second site — the cross-master pair with test-plan section 9, conditioned identically.
**Ref:** .andromeda/runs/2026-09-07T08-32-30-wrap/

## 2026-09-07-dependency-polish — Stack versions reconciled to the resolved lock
**Section:** §Stack and Technologies · §Established Decisions [Language / Runtime] · §Established Decisions [Deployment] · §Infrastructure Patterns — Deployment model · §Inherited Defaults
**Change:** `indicatif 0.17` → `0.18` (resolves 0.18.6); `tokio 1.48.x` → `1.52.3` at all three sites; Tauri `bundler v2.10.x, latest 2.10.1` → `bundler 2.11.3` at all four sites.
**Why:** The indicatif value is this chunk's own bump. The tokio and Tauri values were pre-existing stale literals the chunk's folded CARRY routed here, to reconcile the docs to the resolved artifacts where the dependency surface is already open.
**Kept:** `inquire 0.9` on the same Stack row was already correct. Every `≥ 2.10.3` FLOOR statement stands — a floor is satisfied, not falsified, at 2.11.3.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-dependency-polish — OTLP emission row records the default-features trim
**Section:** §Stack and Technologies
**Change:** The opentelemetry-proto row now reads `default-features = false` at the workspace entry with `gen-tonic` + `trace`/`logs`; `metrics` is de-registered.
**Why:** The trim shipped at the workspace entry because cargo rejects a member disabling defaults on an inherited dep. `metrics` was previously enabled through `conductor-run`'s featureless dev-dep riding `default = [full]`; with defaults off at the workspace entry no member enables it, so the registry now matches the code.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-dependency-polish — [Module Boundaries] E1 qualifier retired on a clean nine-member sweep
**Section:** §Established Decisions [Module Boundaries]
**Change:** The standalone per-seam BUILD claim is no longer qualified as "an intent, not a guarantee". The `conductor-verify` manifest repair landed (tokio's `time` into `[dependencies]`) and the whole roster swept clean — nine members each on its OWN targets (`--lib` ×7, `--bins` ×2 for the two crates with no lib target), every one exit 0, measured twice. `--all-targets` is banned in the sweep because it re-unifies dev-dependencies. A narrower caveat survives: no CI job builds a member standalone, so the property is measured-at-a-chunk, not gate-enforced.
**Why:** The retired text named *Dependency polish* as the repair's owner, so leaving it would send a future planner to redo landed work. Retirement made unconditional by operator wrap directive (the sweep measured twice — builder and operator).
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-dependency-polish — Supply-chain measurement re-stated post-bump
**Section:** §Infrastructure Patterns — Build system
**Change:** `cargo audit` exit 0 re-measured: 562 packages scanned (was 564), 17 allowed warnings (was 18) = 16 `unmaintained` + 1 `unsound`. RUSTSEC-2025-0119 left the ignore list when `number_prefix` left the tree, taking `deny.toml`'s ignore entries 17 → 16. `1239 advisories loaded` unchanged, re-measured rather than carried.
**Why:** The lock moved 564 → 562 and audit warnings 18 → 17; the ignore entry's subject left the tree.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/

## 2026-09-07-a11y-ci-gate — CI gains a Pulse-free a11y gate; a fourth env-handle class; the a11y violation artifact registered
**Section:** §Stack and Technologies (CI/CD row) · §Established Decisions [CI/CD] · §Occupied Resources — Environment variables (`CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`, new `CONDUCTOR_A11Y_STRICT`) · §Occupied Resources — On-disk artifacts · §Infrastructure Patterns — Build system / CI-CD approach / Directory structure · §Inherited Defaults
**Change:**
- [CI/CD]: was build+test only; now a third job (`a11y`, `runs-on: windows-2025`) runs the Pulse-free routine webview a11y leg plus the reused `journal_conformance` gate and the artifact upload. The exclusion narrows to dynamic proof REQUIRING A LIVE PULSE, which is why the routine arm is gateable at all.
- Five consequential restatements reconciled in lockstep (Stack row · CI/CD approach · directory-tree comment · Inherited Defaults · Build-system job enumeration: "both CI jobs run `windows-latest`" became the three-job Windows set with `a11y` on the explicit `windows-2025` label).
- `CONDUCTOR_A11Y_STRICT` registered as a FOURTH handle class — flag-valued, read by `wdio.conf.ts` AND both harness shells, never by a shipped binary. The `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` skip-at-exit-0 clauses are qualified as the LAX arm; both skip sites route through one `exitUnresolvedHandle()`.
- `runs/a11y/<run_id>.jsonl` registered with its measured 13-key (15 under CI) shape and the reasons for its own directory.
**Why:** The chunk shipped the new job, handle and artifact. The [CI/CD] widening was escalated (the locked-decision-reversal rule's causal clause fails: no live SUT contradicted anything, and the route delivered planned work test-plan and a11y-plan already assigned here) and operator-ratified routine: the decision's live-Pulse invariant is preserved and asserted MET; only the unqualified framing went.
**Ref:** .andromeda/runs/2026-09-07T16-19-12Z-wrap/

## 2026-09-07-sr-findings-fixed — a11y job measured RED on the hosted runner; `EDGEWEBDRIVER` registered
**Section:** §Established Decisions [CI/CD] · §Infrastructure Patterns — CI/CD approach · §Occupied Resources — Environment variables
**Change:**
- [CI/CD]: retired "The a11y job's own first GitHub run is pending the operator's push, so the wiring is recorded here, never a green run". The job HAS run on the hosted `windows-2025` image (four runs, 2026-09-07) and is RED at WebView2 session creation, after resolving the driver from the image and getting its RED wdio output into the job log (both fixed and proven in CI). Still never a GREEN run; the build-branch push is still owed; the failure is owned by the route entry *Hosted-runner WebView2 session*. "Which is exactly why it is gateable" is qualified: Pulse-freedom is NECESSARY, not sufficient — hosted-runner runnability is measured-unproven, the dev host proven (12 passing / 2 skipped).
- CI/CD approach: the three enumerated a11y stages are the job's WIRING — the conformance gate and the record upload have never executed in CI.
- `EDGEWEBDRIVER` registered as the CI-side value source for `CONDUCTOR_MSEDGEDRIVER`, with the expression-context mechanism that made the original wiring fail and the handle-named path-free precondition; `wdio.conf.ts` remains the handle's only READER and only validating site.
**Why:** The chunk measured the job red where the body said pending. The registration escalated: the external-handle playbook rule covers handles Conductor "neither SETS nor READS", and `ci.yml` READS `$env:EDGEWEBDRIVER` at four sites, so it did not govern. Operator-ratified, with that rule widened by a new rule rather than stretched.
**Ref:** .andromeda/runs/2026-09-07T21-30-50-wrap/


## 2026-09-08-hosted-runner-webview2-session — WEBVIEW2_* diagnostic handles registered
**Section:** Occupied Resources — Environment variables
**Change:** Registered `WEBVIEW2_USER_DATA_FOLDER` + `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` as one DIAGNOSTIC-scoped pair, on a THIRD registration basis beside the two already present: a shipped artifact SETS them but never READS them. `.github/workflows/ci.yml`'s `a11y` job sets both for probe (a) of its `WebView2 session isolation (diagnostic)` step and clears both before probe (b); the WebView2 loader is the only reader, so neither is a spawn-resolution input nor carries a configuration contract, and their whole lifetime is that one `continue-on-error` step. The `EDGEWEBDRIVER` names-AND-reads bar does not apply, and no `CONDUCTOR_*` namespace claim is made.
**Why:** Architecture held zero `WEBVIEW2` occurrences, so a reader meeting either name in the committed workflow landed nowhere. Neither the neither-SETS-nor-READS rule nor the shipped-artifact-READS rule governed the class, so it escalated as an unruled sixth handle class and was operator-ratified, minting a new playbook rule.
**Ref:** .andromeda/runs/2026-09-08T14-20-00-wrap/

## 2026-09-08-webview2-runtime-152-installed-in-job — CI-scoped Evergreen egress registered; the runtime floated while the driver stays pinned
**Section:** Occupied Resources — Ports · Occupied Resources — Environment variables · Cross-cutting Patterns — Trust boundary · Established Decisions [CI/CD] · Infrastructure Patterns — Build system (five sections; all five applied)
**Change:**
- §Ports registers `https://go.microsoft.com/fwlink/p/?LinkId=2124703` as a CI-JOB-SCOPED egress — the only non-loopback outbound target and the only registered one no shipped binary reaches — fetched over HTTPS with certificate verification intact, admitted only through a pre-execution `Get-AuthenticodeSignature` gate (status `Valid` AND an `O=Microsoft Corporation` signer), binding no port.
- §Trust boundary's outbound-surface enumeration gains that CI-only third surface; the loopback-only claim now carries its scope — every shipped binary and every dev-host harness leg.
- §Environment variables registers `RUNNER_TEMP` on the `EDGEWEBDRIVER` basis (a SHIPPED artifact READS it, in the step SHELL rather than a `${{ env.* }}` expression); the install gate SETS no handle and claims no `CONDUCTOR_*` name; the `WEBVIEW2_*` pair's lifetime is unchanged.
- [CI/CD] records the in-job provisioning (fetch → Authenticode gate → fixed array-form `Start-Process … '/silent','/install'` → post-install major ≥ 152 assertion; no `continue-on-error`, no `if:`) and states that whether a 152+ runtime opens the endpoint is UNMEASURED until a CI run follows the shipping commit.
- §Build system distinguishes the still-pinned Edge DRIVER (fixed by the `windows-2025` label) from the deliberately FLOATED WebView2 RUNTIME, with the float's exit condition stated.
**Why:** Boundary widening — always a human's call, never a routine rule — operator-ratified. §Trust boundary and §Build system are duplicate-occurrence sites a single-site apply would have left standing (§Build system justifies pinning the image so it cannot shift the driver, while this chunk floats the runtime). Operator ruling: Evergreen now (the chunk probes the runtime major, so newest IS the measurement), pinned once the job actually gates.
**Ref:** .andromeda/runs/2026-09-08T20-30-00-wrap/

## 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate — fmt gate enters the CI enumeration; the 152-runtime question retired as measured
**Section:** Stack and Technologies (CI/CD row) · Established Decisions [CI/CD] · Infrastructure Patterns — Directory structure · Infrastructure Patterns — CI/CD approach · Inherited Defaults (CI/CD)
**Change:**
- The gate-set enumeration gains `cargo fmt --all --check` at every site stating what CI runs: the §Stack CI/CD row (technology cell and its "Formatting, build + test gating" rationale), [CI/CD] (the `rust` job's own step at index 2, after the toolchain install and before the cache restore, no `continue-on-error`/`if:`; `--all` over bare because the workspace declares no `default-members`), the `.github/workflows/` directory-tree comment, CI/CD approach, and Inherited Defaults CI/CD.
- [CI/CD]'s "Whether a 152+ runtime actually opens the remote-debugging endpoint is **unmeasured**" is retired as MEASURED: run `34280136892` installed Evergreen 152.0.4191.66 and the endpoint still never opened — the runtime-major hypothesis is FALSIFIED, the cause stays OPEN, ownership sits with the `v2-24` deferral note.
**Why:** The chunk shipped the fmt gate and its CI run measured the provisional claim. The retirement had no matching playbook rule, was escalated and operator-approved, minting a provisional-status-claim retirement rule. The same enumeration claim was amended in security-plan, obs-plan and a11y-plan in the same pass.
**Ref:** .andromeda/runs/2026-09-09T13-20-08-wrap/
## 2026-09-10-release-build-and-bundle — bundler version + installer size retired; CRATE pin untouched
**Section:** Stack and Technologies (Desktop shell row) · Established Decisions [Deployment] · Infrastructure Patterns — Deployment model · Inherited Defaults — Deployment
**Change:** One retired claim, four sites: was "the Tauri 2 bundler (2.11.3) produces an optional ~3 MB GUI installer".
- The BUNDLER is a HOST dev-tool (`tauri-cli` / `tauri-bundler`) absent from `Cargo.lock`, so the tree resolves no bundler version; the `2.11.3` it carried MIRRORED the `tauri` CRATE version, a different artifact. The floor is met by the installed CLI at 2.11.4.
- `~3 MB` is replaced by the measured installer SET, marked `as measured 2026-09-10`: nsis 4 418 544 B (4.21 MB) · msi 6 152 192 B (5.87 MB), both under gitignored `target/`.
- The §Stack row, [Deployment] and Inherited Defaults each drop their `(2.11.3)` bundler pin and name the host tool; the roster floor is pointed at test-plan §4.
**Why:** The chunk measured both halves false; the operator directed ONE amendment carrying the measured pair with its date. No playbook rule governed the class — a measured-SCALAR literal (a version, a size) is neither a set-enumerating literal nor a derived-value sample, and the reconcile-to-impl rule requires values preserved, which is what moved — so applied on the operator's directive, with the gap reported.
**Kept:** The CRATE sites (`tauri` the crate, 2.11.3 present in the lock) are correct and untouched.
**Ref:** .andromeda/runs/2026-09-10T20-36-29-wrap/

## 2026-09-10-release-build-and-bundle — installer byte-size de-literalized after the light gate re-measured it
**Section:** Infrastructure Patterns — Deployment model
**Change:** Was `as measured 2026-09-10: nsis 4 418 544 B (4.21 MB) · msi 6 152 192 B (5.87 MB)`; now the MB scale only, with the reason recorded: a `cargo tauri build` re-run over unchanged source produced an nsis installer of **4 414 280 B** (4 264 bytes off), while the msi reproduced byte-exactly (6 152 192 B). Both nsis readings round to 4.21 MB, so the scale survives the variance and the byte count does not.
**Why:** A spec body must hold current truth: a byte-exact size a re-build of unchanged source does not reproduce asserts more than the toolchain guarantees. It is the `~3 MB` defect class reappearing one layer down — replacing an approximate literal with a byte-exact one made the claim MORE precise than its subject.
**Kept:** The first build's reading stays true as history and is not edited in the chunk's evidence or the `v2-27` ledger `ref`; the evidence gains the second reading as a second row, not a correction.
**Ref:** .andromeda/runs/2026-09-10T20-36-29-wrap/

## 2026-09-11-hosted-runner-endpoint-cause-probed — hosted-image POLICY and SESSION candidates measured and retired; two ci.yml env reads registered; the probe script joins the tree
**Section:** Established Decisions [CI/CD] · Occupied Resources — Environment variables · Infrastructure Patterns — Directory structure
**Change:**
- §Established Decisions [CI/CD]: the closing clause "with a hosted-image policy or session property the leading unmeasured candidate" is retired — both halves measured at CI run `34586959536` and neither holds: all five probed Edge/EdgeUpdate/EdgeWebView policy keys ABSENT in both hives on the hosted runner AND the dev host; the runner is `SessionId 2` / `UserInteractive: True`, not session 0 and not a service context. The body names NO replacement candidate: the runner being elevated (the dev host is not) is an observed difference, NOT a demonstrated cause, since no probe varied it; the third candidate (the module version the host processes load) returned no reading because its step runs 0.645 s after the isolation step stops the app. Scope: that image at that run.
- §Occupied Resources — Environment variables: registers `TEMP` and `LOCALAPPDATA` on the `EDGEWEBDRIVER`/`RUNNER_TEMP` shipped-artifact-READS basis, reading sites named; a PRE-EXISTING gap (this chunk reads no environment variable).
- §Infrastructure Patterns directory tree: `webview2-cause-probe.ps1` listed beside `agent-run.sh` under `scripts/`, CI-only, wired into neither harness shell.
**Why:** (1) is a master's own explicitly-provisional claim retired by the measurement the sentence named as its precondition; the reading for the two NAMED candidates is determinate, so the partial-result escalation does not fire (the unread third candidate is carried in `v3-01`'s `observed_gap`). (2) is an external handle a SHIPPED artifact reads. (3) matched no rule and applied as not-surprising: the per-item-content-in-a-registered-directory rule fails its qualifier, because `scripts/` is tracked at per-FILE grain and the new file is an executable entry point, not config content.
**Ref:** .andromeda/runs/2026-09-11T10-27-30-wrap/

## 2026-09-11-hosted-runner-endpoint-cause-closed — the endpoint cause ESTABLISHED as elevation; the Evergreen install made floor-conditional
**Section:** Established Decisions [CI/CD] · Infrastructure Patterns — Build system · Occupied Resources — Ports · Cross-cutting Patterns — Trust boundary
**Change:**
- Claim A (cause), §Established Decisions [CI/CD]: retired "No replacement candidate is named: … an observed difference and NOT a demonstrated cause … a third candidate … returned NO reading … (0.645 s later)". Now: the module reading is taken inside the isolation step's live window (CI run `34645345201`; placement, not a longer timeout, was the defect; the later step's `no live process` lines persist by design); driver/runtime major SKEW retired as a cause by direct variation with a control (run `34654076633` — `msedgedriver`/Edge/runtime all `152.0.4191.66`, 152 modules loaded against 153 prior, isolation readings byte-identical, endpoint still absent); ELEVATION varied on the dev host reproduces `session not created: DevToolsActivePort file doesn't exist` against a green non-elevated control, the pair differing on `IsElevatedAdmin` alone. "the cause stays OPEN, owned by the `v2-24` deferral note" → "stayed OPEN until 2026-09-12". The MECHANISM is recorded, not established (correlation under single-variable variation only); scope is the hosted `windows-2025` image and the Windows dev host; the endpoint is still CLOSED; `v3-02` owns the remedy.
- Claim B (install posture), §Infrastructure Patterns — Build system: was "the WebView2 RUNTIME is deliberately FLOATED — installs always-latest Evergreen in-job, chosen at job time"; now FLOORED AT 152 with a conditional install, the always-latest rationale recorded EXHAUSTED (not contradicted), the pin-once-it-GATES exit condition explicitly UNMET. §Established Decisions [CI/CD] provisioning: conditional below 152, the Authenticode gate guarding the only fetching path, the `≥ 152` floor assertion OUTSIDE the conditional, running every job. §Occupied Resources — Ports: the fwlink fetch qualified conditional. §Cross-cutting — Trust boundary: the third outbound surface is a CONDITIONAL one-shot fetch.
**Why:** Claim A is routine: the sentence named its own precondition both for the absent reading and for the unestablished cause, and the chunk satisfied each. Claim B escalated and was resolved by operator directive: the provisional-claim-retirement rule does not govern it, because the sentence's own retire-condition ("once the `a11y` job actually GATES") is measurably unmet (A11y job `failure` at run `34654076633`, `DevToolsActivePort` never seen) and the posture changed because the probe's SUBJECT moved, which that rule excludes. No playbook rule proposed for Claim B.
**Ref:** .andromeda/runs/2026-09-12T10-08-17-wrap/

## 2026-09-13-audit-debt-retired-before-epoch-1-closes — LF pinned repo-wide; `conductor-core`'s universal edge narrowed; host-Python instruments registered
**Section:** Infrastructure Patterns — Build system · Infrastructure Patterns — Directory structure · Occupied Resources — Crate names · Stack and Technologies
**Change:**
- Build system: was "The repo's sole `.gitattributes` rule (`coverage-matrix.md text eol=lf`, added 2026-09-06) is a gate-correctness control … It governs that one path; no repo-wide attribute policy is asserted." Now `.gitattributes` pins LF at checkout repo-wide (`* text=auto eol=lf`, added 2026-09-13) — the wildcard IS the repo-wide policy — with the `coverage-matrix.md` rule kept beneath it as the NAMED gate-correctness control, keeping that path's guarantee independent of any later narrowing. Measured 2026-09-13: no renormalization (`git ls-files --eol` 3031 `i/lf` · 13 `i/none` · 4 `i/-text`, no `i/crlf`, no `i/mixed`, against a system `core.autocrlf=true`), and the "LF will be replaced by CRLF" warning went from every touched file to 0.
- Directory tree `.gitattributes` comment: "one rule: coverage-matrix.md text eol=lf" → "repo-wide LF pin (`* text=auto eol=lf`) + the named coverage-matrix.md control" — SET-named, not counted, so a later rule cannot re-stale it.
- Crate names: `conductor-core` was "the runtime-agnostic engine library every other crate depends on" → "the other members depend on — `conductor-emit` EXCEPTED since 2026-09-13, when its unused edge was dropped; the dependent set is whatever the members' `Cargo.toml` files declare, never a universal".
- Stack gains an "Operator instruments (host runtime)" row: host Python 3, resolved by no lockfile and pinned by no toolchain file, running the code-graph pipeline (`scripts/code-graph.py` + `scripts/scip_pb2.py` + `scripts/requirements.txt`) and the mutation-tally gate (`scripts/mutation-gate.py` + `scripts/mutation-roster.toml`); neither CI-invoked, neither a sixth `agent-run` command.
**Why:** The LF edits were settled by the operator's recorded wrap directive (the founder's boundary-#5 LF ruling); a direction settles the proposal, never the class, so no rule was proposed. The crate edge was a universal measured false for one named member, narrowed to the declared-edge SET rather than a fresh enumeration. The Stack row escalated: its premise that the chunk introduces a new runtime is false (the code-graph Python scripts shipped 2026-06-18); the operator chose to register accurately with no novelty implied, naming the code-graph pipeline as precedent.
**Kept:** registering `conductor_core::ENVELOPE_KEYS_SORTED` in §Standard Contracts was dismissed — a public library API symbol is not an arch registry resource, and the chunk adds no port, socket, endpoint, IPC, event or env var.
**Ref:** .andromeda/runs/2026-09-13T19-15-00-wrap/

## 2026-09-13-p-025-measurement-contract-for-pulse — the fifth `contracts/` member, and the first no Rust code reads
**Section:** §Occupied Resources — On-disk artifacts / database · §Infrastructure Patterns — Directory structure
**Change:** Retired claim: the `contracts/` set is four members.
- §Occupied Resources gains a fifth On-disk-artifacts bullet for `contracts/pulse-p025-measurement-contract.md` — the P-025 hue-shift measurement contract (the observable and its literal field names · the resolution · the window as two named Pulse-internal instants · the hard-grade comparison), carrying `sut_version` · `captured_at` · `pinned_at` · `provenance`. It is the FIRST member with no Rust reader — addressed OUTWARD to the SUT — so it has no `default_path()`, no `resolve_under` load path, no bounds check and no `CONDUCTOR_*` override handle; the runtime-read regime of the four manifests above does not attach. Its `provenance` is the INVERSE of the load envelope's and run contract's: a Conductor measurement, not a transcribed SUT record, with every cited Pulse coordinate marked measured at HEAD `83d4060` on 2026-09-13 and expiring when that moves.
- The directory-tree `contracts/` comment moves from four named manifests to five, the fifth qualified "the one member no Rust code reads".
**Why:** An unregistered on-disk artifact is drift; applied under recorded direction (the plan's expected amendments, ratified at P5 and reaffirmed by the operator at wrap). The runtime-parsed-`contracts/`-artifact rule fails its own qualifier because this member is read by nothing; the reader-less member is n=1, so no rule was proposed. Trap: the leaf tier (`CLAUDE.md`'s generated overview and `.claude/docs/conventions.md`) also enumerates the `contracts/` members and must be re-derived when the set moves.
**Kept:** security-plan's §Input Validation row for "Committed SUT-facing manifests read at a fixed path" correctly excludes a reader-less member and is unmoved, as is `.claude/rules/security.md`, which inherits it.
**Ref:** .andromeda/runs/2026-09-14T16-05-00-wrap/

## 2026-09-15-structurally-dead-assertion-class-retired — declare-only registry +4, and two duplicate sites of the same retired claim
**Section:** §Standard Contracts (Run report envelope) · §Established Decisions [Read-Back Dependency Posture] · §Established Decisions [Run-History Persistence] — all three applied.
**Change:**
- §Standard Contracts: the declare-only registry reads NINE families (was EIGHT). `findings-counter-refresh` joins the delegated-timing family (2026-09-15) and the parenthetical `findings-counter-refresh is NOT a member: it carries one [[expected]]` is struck; a new grouping registers the structurally-dead-assertion class (`constellation-severity-live-wiring` · `pulse-run-contract` · `cross-incident-recurrence`, 2026-09-15). No corpus tally literal is added — the registry names the SET.
- [Read-Back Dependency Posture]: the degraded-route example re-tensed to its 2026-08-21 reading ("CARRIED one [[expected]] at that time"), keeping the load-bearing claim — the degraded route is NOT gated on declare-only — and its leg-D evidence, plus a sentence that the scenario is declare-only since 2026-09-15 and no longer a live example of that shape.
- [Run-History Persistence]: `runs.db`'s purpose restated as indexing for P-036 cross-incident fingerprint recurrence, no longer asserting a live `"Previously seen"` check; the retirement and its measured cause recorded (Pulse emits `"## Previously Seen"`; `Contains` is case-sensitive). The index and its purpose are unchanged.
**Why:** The chunk retired four structurally-dead `[[expected]]` declarations to declare-only, falsifying three present-tense arch claims. Trap: two of the three sites escaped a line-granular token grep — one sits mid-line inside a multi-KB line, the other states the claim under a different token (`Previously seen`) — so the claim's wording, read by offset, is the sweep key.
**Ref:** .andromeda/runs/2026-09-15T12-44-09-wrap/

## 2026-09-15-remaining-structurally-dead-declarations-retired — declare-only registry: +5 members, count unmoved, anchored on the complement
**Section:** Standard Contracts (Run report envelope — the declare-only registry)
**Change:** The structurally-dead-assertion class gains `activity-floor` · `service-went-silent` · `high-severity-log-capture` · `exception-event-capture` · `threshold-hot-reload` (2026-09-15), noting the class was taken in two passes over one twelve-block population. The "NINE families" count is deliberately unchanged — the five join an EXISTING group; only the class's membership (3 → 8) and the enumerated scenario total (21 → 26) move. A durable anchor heads the registry: the COMPLEMENT, which shrinks rather than grows — as measured 2026-09-15 exactly TWO committed scenarios still declare live `[[expected]]` checks, `root-span-error-scope` and `span-status-error-detection` (2 of 36) — so declare-only is the corpus's default shape and the list records HOW each family reached it.
**Why:** The chunk retired six `[[expected]]` blocks across five scenarios (live blocks 8 → 2, declaring scenarios 7 → 2). The proposal to re-base the count rested on the false premise that it had moved; re-derived from the registry itself, the count was already correct — an amendment is re-derived from the invariant plus the report's fact, never pasted from the proposal.
**Kept:** The wholesale reframing ("every committed scenario ships declare-only EXCEPT the two") was declined: it would change the registry's subject from families-retired-to-declare-only to all declare-only scenarios, discarding the retirement history the registry exists to carry. The complement anchor keeps its sound half — a value that cannot re-stale upward.
**Ref:** .andromeda/runs/2026-09-15T15-07-04-wrap/

## 2026-09-16-scenario-assertion-audit-gate — the scenario-audit ledger, its gate, and the CI gate-set enumeration
**Section:** §Occupied Resources (On-disk artifacts) · §Infrastructure Patterns (directory structure ×2, CI/CD approach) · §Established Decisions ([Accepted Capability Set] load sites, [CI/CD]) · §Stack and Technologies (CI/CD row) · §Inherited Defaults (CI/CD)
**Change:**
- Registers `contracts/scenario-audit-ledger.toml` as an on-disk artifact: the two-axis exact-set grading `check_scenario_audit` performs, and the resolution that distinguishes it from the three SUT manifests above it — a hard-coded relative `default_path()`, `load()` on an already-resolved path, no `resolve_under`, no `CONDUCTOR_*` handle, and no shipped reader. The `contracts/` tree enumeration gains the same member.
- [Accepted Capability Set]: "the three binary-edge load sites" is de-literalized to the SET of what takes a `&CapabilityManifest` — the binary edges plus the `conductor-core` catalog loaders `list_scenarios` and the new `load_catalog` — never a count.
- The CI gate-set enumeration is brought current at all FIVE sites that state it (§Stack CI/CD row, §Established Decisions [CI/CD], the `.github/workflows/` tree comment, §Infrastructure CI/CD approach, §Inherited Defaults), each now naming the static-gates-over-committed-data SET rather than a list.
**Why:** The chunk landed the ledger, the gate and the named CI step `Scenario-assertion audit gate` (`rust` job 22 → 23 steps, `continue-on-error: false`, presence-guarded). `load_catalog` is a fourth site applying `Scenario::from_toml_str_with`, and all five CI enumerations were already stale by one gate (the coverage-completeness gate shipped 2026-09-06), so adding only this gate would have authored a fresh false enumeration; both were applied as set-naming.
**Kept:** Registering the six new public API symbols and `CoreError::ScenarioAudit` was dismissed — arch tracks ports, sockets, endpoints, IPC, events, env vars and crates, not per-crate API surface. The [Accepted Capability Set] "second integrity gate sits beside it on a different axis" sentence is not a stale enumeration of all integrity gates: it is scoped to the accepted-capability-set axis (`check_sut_drift` = classified? / `check_scenario_backing` = backed?); `check_load_envelope` and now `check_scenario_audit` belong at their own artifact rows.
**Ref:** .andromeda/runs/2026-09-16T08-43-09-wrap/

## 2026-09-16 — a11y-ci-gate-at-an-honest-terminal
**Section:** §Established Decisions [CI/CD] · §Infrastructure Patterns — Directory structure · §Infrastructure Patterns — CI/CD approach
**Change:**
- §Established Decisions [CI/CD]: the dev-host runnability verdict is de-literalized from the pinned "12 passing / 2 skipped" to the measured runtime SET — 12/2 at WebView2 152 on 2026-09-10; 10 passing / 2 failing / 2 skipped re-measured 2026-09-16 after the host floated to Evergreen 153.0.4234.32 unprompted — naming the two regressions as the hold-free Operable pair and stating that session creation itself came up on 153.
- Same decision: the a11y asserting step is recorded as launched THROUGH `scripts/a11y-limited-token-launch.ps1` (leg entry point `scripts/a11y-token-witness.ps1`) rather than invoking `agent-run.ps1` directly.
- Same decision: the remedy class named there (a limited-token launch + a re-measurement, owned by `v3-02`) is recorded MEASURED INSUFFICIENT on a three-leg basis, the successor entry owning the remainder.
- Directory tree: the two new CI-only scripts registered beside `webview2-cause-probe.ps1`, with the same qualifier.
- CI/CD approach: the second occurrence of the direct-invocation claim corrected with the first.
**Why:** The routine arm re-measured 10/2/2 against a cited 12/2; the dev-host WebView2 moved 152 → 153.0.4234.32 (CI unaffected at 152.0.4191.66); both limited-token mechanisms are correct and structurally unable to lower the mandatory integrity label. Trap: a count can be restated in a form a literal-token sweep cannot match (the governed-forms count stood bolded as `**SIX**` in `.claude/rules/security.md`) — read the hits rather than trust a zero.
**Ref:** NOT DERIVED

## 2026-09-17 — 2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm
**Section:** §Established Decisions [CI/CD] · §Infrastructure Patterns — Build system — 4 sites incl. the elevation-remedy bounding
**Change:** The hosted-runner endpoint verdict is retired as UNCONDITIONAL and restated as configuration-bound. Measured at CI run 35192876641: hosted `windows-2022` at a coherent 131.0.2903.86 msedgedriver+WebView2-runtime pair, High integrity — `DevToolsActivePort` in 1 s, WebDriver session created, routine arm 11 passing / 1 failing / 2 skipped, SC 2.4.3 passing and the single red a counting-basis defect in the assertion (12 visits / 6 distinct, bracket lists identical). The endpoint still does not open on `windows-2025` at runtime 152/153; corroborated externally by actions/runner-images#14738 on a byte-identical image and runtime for a plain Tauri/wry app with no token work. Retired phrasings, corrected in place: "opens no remote-debugging endpoint", "RED at WebView2 session creation", "never been green", "Still never a green run", "endpoint remains CLOSED", "runnability is measured-unproven". Integrity's SIGN is configuration-bound: Medium helped at runtime 153 on the dev host; High is REQUIRED at 131 on `windows-2022`. The medium-integrity launcher is recorded MEASURED-INSUFFICIENT: it lowers the label as designed (parent `S-1-16-12288` → child `S-1-16-8192`) and did not open the endpoint.
**Why:** The 2026-09-16 legs A/B/C are BOUNDED by this, never retired — they were correctly measured on what they measured.
**Kept:** Four escalations reached no operator ruling, so the probe-scoped `ci.yml` surfaces that would have motivated further amendments were removed instead of ratified (the `windows-2022` label, the `≥152` floor bypass, the `msedgedriver.microsoft.com` egress, and the launcher's removal from the asserting step). The shipped arrangement is unchanged and no arrangement row moved.
**Ref:** NOT DERIVED

## 2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration — a11y CI arrangement moved to the measured configuration; egress re-registered
**Section:** §Occupied Resources (Ports · Environment variables `EDGEWEBDRIVER` / `RUNNER_TEMP` · Directory structure) · §Cross-cutting Patterns (Trust boundary) · §Established Decisions [CI/CD] · §Infrastructure Patterns (Build system · CI/CD approach)
**Change:**
- The `a11y` job's runner label moves `windows-2025` → `windows-2022`. The `≥ 152` WebView2 runtime FLOOR and the conditional in-job Evergreen install are retired, replaced by a `Pin msedgedriver to the image's WebView2 runtime (gate)` step that derives the driver version from the image's `EdgeUpdate` reading, Authenticode-gates it, asserts driver major == runtime major and publishes `EDGEWEBDRIVER`.
- §Ports re-registers the sole non-loopback egress from the Evergreen fwlink (LinkId 2124703) to `https://msedgedriver.microsoft.com` — count ONE before and after; the Trust boundary's THIRD outbound surface is re-described, not retired.
- `EDGEWEBDRIVER` gains a PRODUCER fact (the pin gate writes it through `GITHUB_ENV`, deliberately overwriting the image's value); `RUNNER_TEMP`'s reader is re-pointed and its "that install gate SETS no environment handle" clause dropped.
- The asserting step becomes a DIRECT invocation of `scripts/a11y-token-witness.ps1` at native High integrity; the tree comment demotes the limited-token launcher to the driver-alone diagnostics.
- Four verdicts retired and restated as the measured SET: "still never a green run", "Hosted-runner runnability is measured-unproven", "the conformance gate and the upload have not yet executed in CI", and "the app must not run elevated" (the step now runs at native HIGH integrity and creates a session).
- The 2026-09-08 pin-on-gating exit condition is recorded MET by the route that exists (the DRIVER pinned to the runtime); the versioned Standalone Installer it originally named could never have served it.
**Why:** Run 35208593666 (headSha `fc4a9c2`) proved the configuration: a11y job and run `success`, 12 passing / 0 failing / 2 skipped, the `journal_conformance` gate run (8/8) and the violation record uploaded. The operator ruled the move at P4 and at P5 ratified the replacement egress on a measured NET NARROWING (version-derived where the fwlink floated always-latest). Every amended claim carries its configuration rather than a bare verdict, since the next image bump can move the label, runtime, driver or their coherence.
**Kept:** The playbook's citation of the retired 152 claim as a rule's provenance stands, and `.claude/rules/host-win32.md`'s `runas /trustlevel` clause stands — it states what the host IS, not what the project does.
**Ref:** .andromeda/runs/2026-09-17T10-34-20-wrap/

## 2026-09-18-real-model-leg-posture-and-grading-rule — the second reader-less `contracts/` member
**Section:** §Occupied Resources — On-disk artifacts · §Infrastructure Patterns — Directory structure
**Change:** Retired claim: the reader-less `contracts/` regime is held by a single member.
- §Occupied Resources gains a row for `contracts/pulse-real-model-leg-posture.md` — the real-model leg posture, emission profile, grading rule and per-leg quiet window, fixed before any such leg is driven — the SECOND member with no Rust reader, taking the P-025 regime unchanged (no `default_path()`, no `resolve_under` load path, no bounds check, no `CONDUCTOR_*` override handle). Its `provenance` is stated PER CLAUSE: the Pulse coordinates are transcribed SUT records read at HEAD `83d4060`; the ~110 s real-model formation figure is a carried Conductor measurement confirmed at the first drive.
- The directory tree's "(the one member no Rust code reads)" → "the two members".
**Why:** §Occupied Resources registers each `contracts/` member individually, so an unregistered on-disk artifact is drift. The P-025 precedent stood at n=1 with no playbook rule; the operator ratified both edits and minting the rule for reader-less members (playbook now 49 entries).
**Kept:** The P-025 row's "FIRST `contracts/` member" stays — a historical ordinal, still true, not a uniqueness claim.
**Ref:** .andromeda/runs/2026-09-18T09-43-21-wrap/

## 2026-09-22-interpretation-proven-live — the L4 posture made scenario data, and the payload-fidelity universal retired
**Section:** §Design Philosophy · §Established Decisions [Read-Back Dependency Posture] + [Probabilistic-Assertion Policy] · §Standard Contracts (Readiness gate · Liveness equivalent · Run report envelope) · §Occupied Resources (`runs/live-suite/` · `pulse-run-contract.toml` · `pulse-real-model-leg-posture.md` · env `ANDROMEDA_PULSE_MCP_ENABLED` · `ANDROMEDA_PULSE_L4_DETERMINISTIC` · NEW `RUST_LOG`) · §Infrastructure Patterns — Directory structure
**Change:**
- (1) Readiness: posture-selected terms (`preflight_for` / `RunContract::evaluate_for`; `conductor run` passes `l4_posture`; `preflight()` for suite/GUI/`drive_run`, `readiness()` for `boot` stay deterministic). Mismatch = scenario-level `Blocked`, never a sixth precondition.
- (2) Liveness: per-KIND grading is deterministic-posture; real-model `handle_declared_for` needs the L4 handle absent/falsy (union truthy rule). `--for` belongs in layout-templates §cli.
- (3) Envelope: "2 of 36" → "2 of 37" (escaped basis); `real-model-interpretation` declare-only, not a tenth family, success row `ManualCheck`.
- (4) Run-contract: `posture?` in term shape; `shell-absence` kind; "only kind that can block" → the two SHELL kinds; SIX → SEVEN terms (`l4-real-model` a Conductor posture term, `l4-deterministic` tagged); observed handles stay two.
- (5) `runs/live-suite/`: real-model `rm.jsonl` / `rm-capture.{txt,err}`, non-recursive clear.
- (6) Posture doc: creation-time attach; ~110 s is deterministic L4's (no real-model figure exists); leg ships as `run --live real-model`.
- (7) Env: L4 handle read under both postures; MCP-enabled true/1 posture-qualified; `RUST_LOG` on the `EDGEWEBDRIVER` basis (the real-model arm's two test runs drop it; `.ps1` restores).
- (8) [Probabilistic-Assertion Policy]: `Identified → (Pass, Pass)`, miss `CalibrationRegion → ManualCheck`, nothing to grade `(null, Blocked)`; test tier only; `classify()` unchanged.
- (9) [Read-Back Dependency Posture]: real-model leg ships, driven once (2026-09-23: `NoAttributableIncident → Blocked`, `v3-09` deferred); degraded-per-read-back gains its CREATION-time mechanism; `state_for` cited by NAME.
- (10) RETIRED "NO read-back field varies with the emitted payload": true at `efabe8e`, false at `83d4060` (`grounded_fingerprint_hashes` puts the cue's full-hex fingerprint in `fingerprint_refs`). Freshness stays the canary's carrier; payload fidelity is PARTIAL.
- (11) "call identically": deterministic posture only; real-model runs headless.
- (12) "one per P-ID" → each file names its P-IDs; several may name one.
- (13) Corpus restatement gains the ratified exception pointer (security-plan).
**Why:** (10)–(12): operator-resolved escalations; (10) rests on measured 2026-09-10 envelopes.
**Ref:** .andromeda/runs/2026-09-23T08-03-55-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — registries compacted under the read cap; the operator-instrument row gains a third member
**Section:** §Established Decisions and §Occupied Resources (each replaced whole) · §Stack and Technologies ("Operator instruments (host runtime)" row)
**Change:**
- (1) Both registry sections were replaced by the chunk's compacted drafts: §Established Decisions 49 134 → 37 907 B; §Occupied Resources 48 859 → 37 929 B. The limit is 38 115 B — 60 % of the 25 000-token Read cap at 2.541 B/token — measured by `scripts/arch-registry-check.py measure --file .andromeda/architecture.md` (`registries: within target`).
- Every label, registered name, port, crate and scoped qualifier is kept; dated narrative (chronologies, superseded readings, CI-run stories, elimination stories) left the body.
- The per-sentence disposition ledger holds 579 rows (kept 352 · rewritten 150 · moved 68 · in-sidecar 9). The `CONDUCTOR_MSEDGEDRIVER` skip row is anchored on its own bullet, not the NVDA one. [CI/CD] keeps the dev-host green tally with its two-configuration basis, and the three-leg basis of the integrity-label discriminator.
- The passages that left the body are the entries headed by this marker, one per decision label or sub-registry, verbatim: history, or the BEFORE wording of a sentence the body now states more briefly. History the sidecar already carried is not repeated.
- (2) §Stack row: `scripts/arch-registry-check.py` is registered as the third committed operator instrument; "Neither … neither" → "None … none".
**Why:** The body holds only current truth, yet it carried its own amendment log (132 ISO dates, 13 CI run ids and 35 sha-like tokens across the two sections) and was growing ≈1 KB a day at wrap toward the Read cap.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Database]: passages moved out of the body
**Section:** §Established Decisions — [Database]
**Change:** passages moved verbatim to the archive — the brief's stale `rusqlite 0.31 / SQLite ≥3.38` pin ratified to 0.38.0.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [MCP Read-Back Client]: passages moved out of the body
**Section:** §Established Decisions — [MCP Read-Back Client]
**Change:** passages moved verbatim to the archive — the hardened spawn (fixed program path + `.env(...)` data-dir + injection-reject) is unchanged.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Accepted Capability Set]: passages moved out of the body
**Section:** §Established Decisions — [Accepted Capability Set]
**Change:** passages moved verbatim to the archive — the original compile-time `001..=060` bound in `pid_format`, which the SUT outgrew, is superseded; the `KNOWN_UNCLASSIFIED` residual ledger that briefly bridged the two is retired to `[]`.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Module Boundaries]: passages moved out of the body
**Section:** §Established Decisions — [Module Boundaries]
**Change:** passages moved verbatim to the archive — the standalone per-seam BUILD now holds for every member, measured rather than gate-enforced. The retired caveat: workspace feature unification can mask a feature a crate uses but declares only in `[dev-dependencies]`, so it compiles in the workspace and in `cargo test -p` while failing alone — measured 2026-09-04 as `cargo check -p conductor-verify --lib` red on `tokio::time::sleep`, repaired 2026-09-07 with tokio's `time` declared in `[dependencies]`.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [CI/CD]: passages moved out of the body
**Section:** §Established Decisions — [CI/CD]
**Change:** passages moved verbatim to the archive — the `a11y` job's history. The claims they retire:
- `runs-on` moved from `windows-2025` to `windows-2022` (2026-09-17); the leg (`scripts/agent-run.ps1 run --e2e` under `CONDUCTOR_A11Y_STRICT`) runs through `scripts/a11y-token-witness.ps1` directly at native High integrity; `scripts/a11y-limited-token-launch.ps1` survives only on the driver-alone diagnostics.
- The `Install WebView2 Evergreen runtime 152+ (gate)` step and its `≥ 152` floor are retired (not necessary, not sufficient, destructive) for driver/runtime coherence: `Pin msedgedriver to the image's WebView2 runtime (gate)` reads the runtime from the `EdgeUpdate` keys, fetches that driver, gates it by `Get-AuthenticodeSignature` (`Valid` AND `O=Microsoft Corporation`), asserts equal majors and publishes `EDGEWEBDRIVER`; then `journal_conformance` over `runs/a11y`. Neither the pin nor the asserting step carries `continue-on-error` or `if:`.
- Falsified causes: the runtime major, a hosted-image policy, a session property, driver/runtime skew. "The app must not run elevated" is retired as a general remedy. The discriminator is the mandatory integrity label, which neither `RunLevel Limited` (the job account is the built-in Administrator with `FilterAdministratorToken` off) nor `runas /trustlevel` (strips the group, leaves the label) lowers; an explicit medium-integrity launch lowered it and did not open the endpoint.
- The dev-host Operable-pair regressions were assertion defects (a walk waiting on a `BODY` sentinel), not a platform property.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Timing-Tolerance Model]: passages moved out of the body
**Section:** §Established Decisions — [Timing-Tolerance Model]
**Change:** passages moved verbatim to the archive — `slo_tier` keeps its closed three-value set and the latency formula is unchanged.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Read-Back Dependency Posture]: passages moved out of the body
**Section:** §Established Decisions — [Read-Back Dependency Posture]
**Change:** passages moved verbatim to the archive — the gate reached `ready:true` for the first time on 2026-08-16.
**Why:** the body holds only current truth; its history lives in the archive and in git. The moved passages record measurements later chunks build on:
- `retrieve_telemetry_slice.fingerprint_refs` was filled from the L4 model's `evidence_refs` alone (a constant `det-*` triple under deterministic L4); Pulse's computed fingerprint sits in `span_events.fingerprint`, which no MCP tool reads. At `83d4060` the refs are the model's ∪ the triggering cue's fingerprint (`grounded_fingerprint_hashes`), so the envelope's `fingerprints` carries it beside `det-*`: "no stronger claim exists" is RETIRED, payload fidelity is PARTIAL.
- `incident_events` (the one non-L4-authored corpus table) is read only corpus-side, with zero references in `crates/mcp-server`.
- At most ONE incident is active per dedupe tuple `(kind, scope, scope_id)`; the workspace scopes only the candidate set.
- Conductor's `fingerprint()` differed from Pulse's for every slash-bearing path until the SUT's new normalization guard was transcribed; this supersedes the 2026-08-16 leading-path-segment narrowing.
- `degraded` is PER-READ-BACK: the universal "under deterministic L4 every read-back returns `degraded_mode`" is RETIRED (2026-09-06). `state_for` returns `KnownResidual` iff `observation.degraded`, so a `ManualCheck` row proves `degraded == false`; `degraded-mode-report` took `manual_record` → `ManualCheck`. `findings-counter-refresh` (then one `[[expected]]`, declare-only since 2026-09-15) landed `KnownResidual` on a degraded read-back, its unmet `CountAtLeast` floor grading `CalibrationRegion` (a sample floor never hard-fails).
- The firing condition is uptime-bound by the window in force at boot; `BOOTSTRAP_WINDOW_SECONDS = 3_600` is only its DEFAULT. Disarming the evaluator removes cause (a), not (b): the arm fires only when the window's last incident forms early enough to clear the 120s idle + 30s tick before read-back. Leg E reached the arm by running inside the window, not because a `curious` cue can never be refreshed; the 2026-08-20 reading that the canary's own error-rate cues keep the list non-empty is RETIRED.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Established Decisions [Run-History Persistence]: passages moved out of the body
**Section:** §Established Decisions — [Run-History Persistence]
**Change:** passages moved verbatim to the archive — its `Contains "Previously seen"` check was retired, because Pulse emits the token as `"## Previously Seen"` and `Contains` is case-sensitive, so the declared token never matched.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Ports: `https://msedgedriver.microsoft.com`: passages moved out of the body
**Section:** §Occupied Resources — Ports: `https://msedgedriver.microsoft.com`
**Change:** passages moved verbatim to the archive — it REPLACED `https://go.microsoft.com/fwlink/p/?LinkId=2124703`, the WebView2 Evergreen bootstrapper target admitted 2026-09-08 and retired with its install step.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Service / process names: `andromeda-pulse-mcp`: passages moved out of the body
**Section:** §Occupied Resources — Service / process names: `andromeda-pulse-mcp`
**Change:** passages moved verbatim to the archive — the Pulse MCP server (`andromeda-pulse-mcp`), spawned over PIPED stdio from a `tokio::process::Command` built by `conductor-verify/src/spawn.rs::build_command` and handed to `ReadbackClient::connect_command` (rmcp and its `TokioChildProcess` removed 2026-06-27, absent from every crate `src/` since); on Windows the command carries the `CREATE_NO_WINDOW` creation flag (`spawn::console_suppressing_flags()`, `#[cfg(windows)]`), so a GUI-launched sidecar raises no console pane and publishes no absolute exe path.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Crate names (workspace members):: passages moved out of the body
**Section:** §Occupied Resources — Crate names (workspace members):
**Change:** passages moved verbatim to the archive — the `conductor-run → conductor-faults` dependency edge, conductor-faults' first consumer, sitting above the seams and below, shared by, both bins: `conductor-cli` and the `conductor-tauri` bin.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Frontend asset subtree:: passages moved out of the body
**Section:** §Occupied Resources — Frontend asset subtree:
**Change:** passages moved verbatim to the archive — `node_modules/`, the build output `crates/conductor-tauri/ui/dist/` and `crates/conductor-tauri/ui/logs/` stay git-ignored. `ui/logs/` is the RETIRED pre-2026-09-01 self-obs landing site: under the repo-root launch cwd with `CONDUCTOR_RUNS_DIR` unset the stream lands at the workspace-root `logs/`; the routine `--e2e` arm sets the handle and lands beside its fixture runs dir instead (see the `conductor-tauri.jsonl` bullet).
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `logs/conductor-tauri.jsonl`: passages moved out of the body
**Section:** §Occupied Resources — On-disk artifacts / database: `logs/conductor-tauri.jsonl`
**Change:** passages moved out of the body verbatim:
- With `CONDUCTOR_RUNS_DIR` unset the path is CWD-relative; the a11y legs spawn tauri-driver with `cwd` = the workspace root (the app under test inherits it).
- The landing site is per-suite: the sink resolves as `runs_dir.parent()/logs` and the ONE tauri-driver spawn site chooses `CONDUCTOR_RUNS_DIR` per invoked suite — the routine `--e2e` arm (`runs/e2e-fixture`) and the `sr-empty` suite land at `runs/logs/conductor-tauri.jsonl`; the operator-local `a11y:driven` arm (`runs/driven/runs`) at `runs/driven/logs/conductor-tauri.jsonl`; the `sr` / `sr-error` suites (`runs/sr-leg/runs`) at `runs/sr-leg/logs/conductor-tauri.jsonl`.
- No a11y suite leaves the handle unset any more; a plain launch with it unset still lands at the workspace-root `logs/conductor-tauri.jsonl`, so that root site is retired only as an a11y landing site, not as the unset-handle mechanism.
- The pre-2026-09-01 landing site `crates/conductor-tauri/ui/logs/conductor-tauri.jsonl` is retired and stays git-ignored explicitly.
**Why:** the body holds only current truth; its history lives in the archive and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/pulse-load-envelope.toml`: passages moved out of the body
**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/pulse-load-envelope.toml`
**Change:** compacted under the read cap; these passages left the body verbatim:
- No phase declaring occurrences may run longer than `max_sustained_storm_ms` nor emit faster than `max_sustained_rate_spans_per_s` (`EmissionSpec::occurrences` made both computable).
- The rate term counts WIRE RECORDS as an upper BOUND (shipped 2026-09-06, superseding the DISPATCHES reading): `phase_rate_exceeds` judges `occurrences × EmissionSpec::max_spans_per_dispatch() × 1000 > max_rate × gap_ms`, exact integer math, no division.
- Per-dispatch count per `EmissionShape` arm: `Ramp`/`Breathing` `sum(window_counts)`, `Latency` its `samples`, `Error` `depth + 1`, `Pii` 2 spans on traces or one record per category on logs, `Topology` one span per service, every other shape 1 — the retired `samples` / `windows` reading held for `Latency` alone and understated a rate curve by its own rate.
- Catalog worst case: `halo-breathing-encoding` at ~232 records/s (a ramp bounded at 1160 records per dispatch over a 5 s window), ≈43× under `max_sustained_rate_spans_per_s = 10000` — superseding "~200× under", which held only for the `samples` reading; no verdict moves.
- The contract file is unchanged: the term keeps its name and value, since it transcribes a SUT record about wire load — the code was brought to the name. This retires the SURFACED-not-authored disposition; the fix is neither prior prediction (not `occurrences × samples`; no rename).
- `phase_rate_exceeds` has one caller, `phase_breach`, whose only callers are `check_load_envelope` (the static gate) and `LoadEnvelope::classify` (the per-run `[ENVIRONMENT-SUSPECT]` caption): one shared basis, so they cannot diverge, and the `[[exempt]]` ledger — exact-set equality both ways, so it can only shrink under compulsion — is now **empty**.
- The predicted landing (asserting SUMMED emitting-phase duration retires the exemptions) was falsified — disjoint bursts separated by quiet are not *sustained* — so the term bounds the longest single emitting window.
**Why:** the body holds only current truth; history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/pulse-run-contract.toml`: passages moved out of the body
**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/pulse-run-contract.toml`
**Change:** passages moved out of the body; they state:
- `[[term]]`: SIX on 2026-09-03 with `mcp-enabled` (`check = "shell-declaration"`, `env = "ANDROMEDA_PULSE_MCP_ENABLED"`, class of `l4-deterministic`); SEVEN on 2026-09-22 with `l4-real-model` (`check = "shell-absence"` on `ANDROMEDA_PULSE_L4_DETERMINISTIC`, `posture = "real-model"`, a Conductor posture term), `l4-deterministic` tagged `posture = "deterministic"`.
- `sidecar-built`'s `asserted` rationale is FALSE: a `PATH` miss short-circuits to `[BLOCKED]` in ~0s upstream of term evaluation, so it is satisfied by construction when false; not re-classified (operator-selected: that moves what the gate can block on).
- `warmup_ms = 45000` cannot clear Pulse's bootstrap: baseline-derived cues gate on `BootstrapState::Ready` (sole gate in `crates/triage/`, in `evaluate_service_went_silent`): `now − first_observed_unix_nanos ≥ BOOTSTRAP_WINDOW_SECONDS = 3_600` per service, wall-clock (a default; `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` overrides at boot); `baseline_state` holds 0 rows, so each `pulse-app` launch resets it; 45s is 80× short, which its `check = "asserted"` cannot catch. The canary rides no baseline (count-in-window + the `last_emitted` tier ladder), so the warm-up is harmless; "a live incident needs a Pulse-side change" is retired FALSE.
- The real gate: the storm must reach `DEFAULT_AUTONOMOUS_THRESHOLD = 10` (`>=`; Tier-1 takes Autonomous cues alone); `CANARY_STORM_COUNT = 6` sat in `5 <= 6 < 10`.
- Closed 2026-08-16, Conductor-side: `ok_span` stamped a constant `vec![1; 16]` / `vec![1; 8]` identity against `spans`' `PRIMARY KEY (trace_id, span_id)`, logged-and-skipped by `run_consumer`; `inject_demo` derives `trace_id(seq)` / `span_id(seq)`. Fixed by seeded per-call identity, disjoint warm-up/storm seeds; `exception` events had arrived intact. The storm's zero appended rows are inferred, not proven.
**Why:** the body holds only current truth. Telemetry traps: `span_count` is a cumulative `fetch_add`; `buffer.tick` counters localize loss; `tracked_fingerprints_count` is a 60s gauge over DISTINCT fingerprints sampled at 15s after eviction — a working storm reads `1`, a late one 0, never the `CANARY_STORM_COUNT`; never read it as absence.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/scenario-audit-ledger.toml`: passages moved out of the body
**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/scenario-audit-ledger.toml`
**Change:** passages moved verbatim to the archive — the ledger's grounds as transcribed readings of `andromeda-pulse` at its own `sut_head`, and its resolution differing from the three manifests (hard-coded relative `default_path()`, `load()` taking an ALREADY-RESOLVED path, no `resolve_under`, no `CONDUCTOR_*` override, no shipped reader — its only reader the crate-local gate test binary, resolving the workspace root from `CARGO_MANIFEST_DIR`).
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources On-disk artifacts / database: `contracts/pulse-real-model-leg-posture.md`: passages moved out of the body
**Section:** §Occupied Resources — On-disk artifacts / database: `contracts/pulse-real-model-leg-posture.md`
**Change:** passages moved verbatim to the archive — the launch posture and emission profile (its multi-digest premise corrected in place 2026-09-22), the per-clause `provenance` (the ~110 s figure a deterministic-L4 measurement, under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`, mis-carried as real-model, corrected 2026-09-23), and its dated in-place corrections.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_PREFLIGHT_TIMEOUT`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_PREFLIGHT_TIMEOUT`
**Change:** passages moved verbatim to the archive — the ~47 s measurements of `scripts/agent-run.sh` and `scripts/agent-run.ps1`, and the run contract's `[incident_formation].min_canary_poll_seconds` floor (the bare default being shorter than Pulse's L3 digest cadence).
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_MSEDGEDRIVER`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_MSEDGEDRIVER`
**Change:** passages moved verbatim to the archive — the handle read only by `crates/conductor-tauri/ui/wdio.conf.ts` (never a shipped binary), the one config all three suite families fire, so it gates the routine `--e2e` arm, the operator-local `a11y:driven` arm and the `sr*` suites alike; its skip-at-exit-0 when unresolved (non-zero under `CONDUCTOR_A11Y_STRICT`); and its being one of two host dev-tool handles with `CONDUCTOR_NVDA`.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_NVDA`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_NVDA`
**Change:** passages moved verbatim to the archive — the handle's harness-edge validation exactly like `CONDUCTOR_MSEDGEDRIVER` (existence + `isFile` + shell-metacharacter rejection) and its use only as the program of an array-form detached spawn with a fixed argv (`-m --no-sr-flag -c <leg config dir> -l 12 -f <leg speech log>`, then `-q`), never interpolated into a shell; the `sr` / `sr-empty` / `sr-error` skip-at-exit-0 when unresolved, and `exitUnresolvedHandle()` making it exit non-zero under `CONDUCTOR_A11Y_STRICT`.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_E2E_SEED_DIR`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_E2E_SEED_DIR`
**Change:** passages moved verbatim to the archive — the handle as the THIRD class in the namespace (not operator-supplied, no `isFile` / shell-metacharacter guard, never reaching the `conductor-cli` `canonicalize` edge since no shipped binary reads it, repo-relative discipline at the wdio caller), distinct from `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA`, which name HOST dev-tools the document does not define.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `CONDUCTOR_A11Y_STRICT`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `CONDUCTOR_A11Y_STRICT`
**Change:** passages moved verbatim to the archive — the handle as a FOURTH, flag-valued class (affirmative `"true"` / `"1"`, mirroring `conductor_core::flag_declared`), never a path, so neither the `std::fs::canonicalize` rule nor the wdio-edge `isFile` + shell-metacharacter guard applies; and its effect (unset keeps the skip-at-exit-0; affirmative makes an unresolved `CONDUCTOR_MSEDGEDRIVER` / `CONDUCTOR_NVDA` exit non-zero).
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `ANDROMEDA_PULSE_MCP_ENABLED`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `ANDROMEDA_PULSE_MCP_ENABLED`
**Change:** passages moved verbatim to the archive — the handle as the run contract's second `shell-declaration` term (`mcp-enabled`) and one of the three `ANDROMEDA_PULSE_*` subjects `conductor preconditions` observes; it and `ANDROMEDA_PULSE_L4_DETERMINISTIC` keep the truthy `"true"`/`"1"` gate (`conductor_core::flag_declared`) under the deterministic posture, beside the path-valued `ANDROMEDA_PULSE_DATA_DIR` met by presence-after-trim (`conductor_core::handle_declared`); and `ANDROMEDA_PULSE_MCP_ENABLED=false` leaving `handles-declared` naming it alone.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `EDGEWEBDRIVER`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `EDGEWEBDRIVER`
**Change:** passages moved verbatim to the archive — the `a11y` job's pin gate setting it through `GITHUB_ENV` since 2026-09-17 (overwriting the image's floating-driver value) and the asserting step resolving `CONDUCTOR_MSEDGEDRIVER` from it in the step shell (`$env:EDGEWEBDRIVER`), because `${{ env.* }}` never holds a runner-process variable; `crates/conductor-tauri/ui/wdio.conf.ts` the only reader and validator (`isFile` + shell-metacharacter rejection) before handing it to tauri-driver via `--native-driver`.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`
**Change:** passages moved verbatim to the archive — the handle's registration ground (named in the `run --live` banner of both `scripts/agent-run.{sh,ps1}`), its governing every leg of the operator's launch, and its distinctness from the two `shell-declaration` run-contract terms and the three `ANDROMEDA_PULSE_*` subjects `conductor preconditions` observes.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `WEBVIEW2_USER_DATA_FOLDER`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `WEBVIEW2_USER_DATA_FOLDER`
**Change:** passages moved verbatim to the archive — the pair read by the WebView2 loader alone (no shipped binary, not `crates/conductor-tauri/ui/wdio.conf.ts`; no spawn-resolution input, no configuration contract), registered on the discoverability ground the `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` entry states, added 2026-09-08 and operator-ratified.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `RUNNER_TEMP`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `RUNNER_TEMP`
**Change:** passages moved verbatim to the archive — `.github/workflows/ci.yml`'s `a11y` job reads it in the `WebView2 session isolation (diagnostic)` step and, since 2026-09-17, the `Pin msedgedriver to the image's WebView2 runtime (gate)` step; that gate sets `EDGEWEBDRIVER` through `GITHUB_ENV` but claims no `CONDUCTOR_*` name, so the `WEBVIEW2_*` pair's registered lifetime — that one `continue-on-error` diagnostic step — is unchanged.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-architecture-registries-compacted-under-the-read-cap — §Occupied Resources Environment variables: `TEMP`: passages moved out of the body
**Section:** §Occupied Resources — Environment variables: `TEMP`
**Change:** passages moved verbatim to the archive — the handle's registration on 2026-09-11 as a pre-existing gap.
**Why:** compacted under the read cap; the body holds only current truth, history lives here and in git.
**Ref:** .andromeda/runs/2026-09-24T08-36-40-wrap/

## 2026-09-24-secret-scanning-ci-gate — the static-gate family gains a second kind (repository hygiene)
**Section:** §Established Decisions [CI/CD] · §Stack and Technologies CI/CD row · §Infrastructure Patterns (the `.github/workflows/` tree comment, the CI/CD approach, the Build-system dev-test stack) · §Inherited Defaults — CI/CD
**Change:** the `rust` job's static gates now have two kinds. Beside the gates over committed data (coverage-completeness, scenario-assertion audit) sit two repository-hygiene gates:
- `Secret-scan gate` (`-p conductor-core --test secret_scan_gate`): no secret-shaped string or file name in `git ls-files --cached --others --exclude-standard`.
- `Workflow env-context gate` (`--test workflow_env_gate`): every `env.X` expression read names a declared or `GITHUB_ENV`-written key — an admission the `GITHUB_ENV context probe` pair re-measures on every run.
- Each is presence-guarded, with no `continue-on-error` and no `if:`. The four enumeration sites name both kinds.
- `regex` joins the dev-test stack as a `conductor-core` dev-dependency edge (already locked, still 562 packages).
Was: "the static gates over committed data" as the whole set.
**Why:** neither new gate asserts a committed artifact against a production source, so every "static gates over committed data" enumeration under-described the set; the text is derived by kind rather than a "repo gates" label.
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-24-secret-scanning-ci-gate — §Occupied Resources env vars: `GHA_ENV_CONTEXT_PROBE` registered; the stale `${{ env.* }}` rationale retired
**Section:** §Occupied Resources — Environment variables (new `GHA_ENV_CONTEXT_PROBE` bullet · `EDGEWEBDRIVER` · `RUNNER_TEMP`)
**Change:**
- `GHA_ENV_CONTEXT_PROBE` is registered on the `EDGEWEBDRIVER` names-AND-reads basis (playbook rule "a SHIPPED artifact READS"): `ci.yml`'s `rust` job writes it to `GITHUB_ENV` and reads it back through `${{ env.* }}`.
- `EDGEWEBDRIVER`: was "because GitHub's `${{ env.* }}` expression context holds only workflow/job/step declarations and never a runner-process variable"; now a `GITHUB_ENV` key also resolves through `${{ env.* }}` (measured at run 36006370951), so the shell read is a choice, not a necessity.
- `RUNNER_TEMP`: was "whose context holds only what a workflow, job or step declared"; now "whose context carries no image-set runner variable".
**Why:** a CI run on `windows-latest` measured that a `GITHUB_ENV`-written key resolves through `${{ env.* }}` (the `GITHUB_ENV context probe (assert)` step printed its success line), disproving the stale rationale. The same stale clause in `.github/workflows/ci.yml` (not a master) is carried as a route-resolve CARRY.
**Kept:** the WEBVIEW2 sets-never-reads basis was rejected for `GHA_ENV_CONTEXT_PROBE`, since the job reads the key back.
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-24-secret-scanning-ci-gate — passages moved out of the body (D-arch-registry-size remedy)
**Section:** §Established Decisions [CI/CD] · §Occupied Resources — Environment variables: `WEBVIEW2_USER_DATA_FOLDER`
**Change:** passages moved verbatim to the archive — the dev-host legs establishing the mandatory integrity label as the discriminator, with neither `RunLevel Limited` nor `runas /trustlevel` lowering it on the runner (§Established Decisions [CI/CD]), and the `WEBVIEW2_USER_DATA_FOLDER` · `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` registration ground (the `EDGEWEBDRIVER` names-AND-reads bar does not apply; no `CONDUCTOR_*` namespace claim).
**Why:** to keep both registries within the size target after this wrap's additions; the body holds only current truth, history lives here and in git. The decision the first passage supported stays in the body ("which is why the asserting step does not take it").
**Ref:** .andromeda/runs/2026-09-24T14-02-12-wrap/

## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the real-model series, the three-storm canary, the backed diagnostic-quality four
**Section:** §Established Decisions [Read-Back Dependency Posture] (the gate description and the interpretation passage) · §Standard Contracts — Readiness gate · §Standard Contracts (the data-dir / corpus paragraph) · §Occupied Resources — `contracts/pulse-real-model-leg-posture.md`
**Change:**
- [Read-Back Dependency Posture], the interpretation passage:
  - Was "That leg now SHIPS and was driven ONCE (2026-09-23)". Now the leg is driven only as the pre-stated series its posture contract fixes before the first drive, and each counted drive is graded against a rule committed before it fires.
  - The 2026-09-23 drive blocked model-side at the one-storm canary (`NoAttributableIncident → Blocked`).
  - The 2026-09-29 series cleared that block with the three-storm canary and graded one drive `NotIdentified`: its model input carried the workspace key as `[redacted: credit_card]`, because Pulse's scrubber matched the dir name. `v3-09` is not met.
  - Was "the four diagnostic-quality capabilities … sit in the `UNBACKED_AUTO` ledger". Now they are backed: the scenario names them, each is graded by its own harvest arm, and they are out of `UNBACKED_AUTO`. Backing is not verifying.
- The same decision's gate description: the canary is three storms, 90 s apart, under real-model L4.
- Readiness gate: under the real-model L4 posture the canary is `REAL_MODEL_CANARY_STORMS = 3` distinct storms of `CANARY_STORM_COUNT` 12, `REAL_MODEL_CANARY_STORM_GAP = 90 s` apart, with the freshness stamp taken before the first. The reason: the model's surface/dismiss decision is nondeterministic per decision (`k_A = 2`, confirmed `k_B = 2`). The deterministic canary stays one storm, byte-identical.
- The corpus paragraph now points at security-plan's recorded 2026-09-29 breach (series pins in test source) and its route owner.
- The posture-contract entry: the drive series (2026-09-29) joins what is fixed before the first drive, and the Pulse coordinates are read at HEAD `e98d838` (was `83d4060`).
**Why:** the chunk replaced "driven once" with a pre-stated series (overseer ruling D1, founder-delegated), landed the three-storm canary and moved the four ids off the pin (`v3-10`); the founder ruled `v3-09` recorded not met, owned by a new series entry.
**Kept:** history moved out so both registries stay within the size threshold (here and in git).
- The 2026-09-23 block's detail: the model dismissed the canary's one cue-bearing digest, so no incident formed and the preflight blocked after its 600 s poll.
- `blocked_precondition: null`, from the gate's live-reading parenthetical.
- From the posture entry: the "(2026-09-18)" pin date, the `conductor-run/tests/lifecycle_live.rs:20` / `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` locator, and "never a sixth command".
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/

## 2026-09-29-dual-license-mit-or-apache-2-0 — the dual license
**Section:** §Infrastructure Patterns → Directory structure · §Infrastructure Patterns → Build system
**Change:**
- Directory structure: the root-file set gains `LICENSE-MIT` and `LICENSE-APACHE`, one tree row naming them as a set — the dual license every Cargo + npm manifest declares, `MIT OR Apache-2.0`.
- Build system: `deny.toml`'s license policy now also covers the workspace's OWN `conductor-*` crates — no `private` exemption. Each passes as `MIT OR Apache-2.0`, inherited from `[workspace.package]` (`license.workspace = true` in every member), so a member without an allowed license fails `cargo deny check licenses`; `publish = false` exempts nothing.
**Why:** The repository went public and was dual-licensed on the founder's direction. The own-crate check was the overseer's founder-delegated ruling: an exempted gate proves nothing about our crates. The member set, every `name@version` and `Cargo.lock` stay unchanged.
**Ref:** .andromeda/runs/2026-09-29T19-16-23-wrap/

## 2026-09-29-hue-shift-budget-graded-hard — the P-025 contract row, satisfied and graded hard
**Section:** §Occupied Resources → On-disk artifacts → `contracts/pulse-p025-measurement-contract.md`
**Change:** was "what Pulse would have to emit for its delegated ≤2 000 ms hue-shift budget to become measurable at all", with provenance "a Conductor measurement, not a transcribed SUT record" and every coordinate as measured at HEAD `83d4060` on 2026-09-13. Now the row records the observable Pulse emits since `e98d838` (`hue_update_ms` = paint − `tier_effective_at`), with its field names, resolution, window and hard grade, and §The grading rule, stated before the drive. Under that rule the ≤2 000 ms budget grades HARD (PASS, worst 684.98 ms, 2026-09-29), and the retired `83d4060` instrument stays in the document as history. Provenance is now per clause (MIXED): the Pulse coordinates are transcribed SUT records read at HEAD `226554a`, and the readings are Conductor measurements. The no-Rust-reader regime is unchanged.
**Why:** Pulse's P-025 fix satisfied the contract's ask, and the one graded leg measured the budget at a real value. The row's own expiry clause ("expires when that moves") named this event. The row was trimmed to keep §Occupied Resources within the registry target: the per-leg HEAD pairing (2026-08-21 at `f0c38f5`, 2026-09-07 at `83d4060`, the graded leg at a checkout carrying `e98d838`) and the evidence path live in the contract and in this entry's Ref, not in the body.
**Ref:** .andromeda/runs/2026-09-29T21-30-19-wrap/

## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — sha2 test-only, breach remedied, second series
**Section:** §Stack and Technologies (Hashing / digest) · §Established Decisions · §Standard Contracts (Readiness gate, corpus access) · §Occupied Resources (the posture-contract row) · §Infrastructure Patterns (Build system)
**Change:**
- Hashing row: blake3 stays the only SHIPPED hashing dependency; `sha2 0.10` is added as a TEST-only `conductor-run` dev-dependency for the real-model harvest's sha256 digest pins. It was already locked via `tauri-codegen` / `wry`, so it adds no package.
- Build system: the dev-test stack list gains `sha2` beside `regex`.
- Corpus access: the 2026-09-29 breach now reads REMEDIED on 2026-09-30, with the frozen 2026-09-22 file's residual (was "and its route owner").
- Posture-contract row: the contract now names both drive series (2026-09-29, then 2026-09-30), each add-only and fixed before its first drive. The 2026-09-30 series is re-pinned to Pulse `fcc31b2`.
- Established Decisions: the dev-host clause now reads "0 failing, the expected-skip SET skipped" (was "12 passing / 0 failing / 2 skipped"). The routine arm gained its stall spec, and the run-anchored records at the same site are kept.
**Why:**
- Each line brings the body to what the chunk shipped. The registry-size check read OVER after the first apply (Established Decisions +76 B, Occupied Resources +15 B).
- Two clauses were trimmed to within target (38 111 B and 38 028 B against 38 115 B); the dropped detail lives here:
  - the 2026-09-30 section's digest `0091fe6f…` was recorded in its chunk's attempt ledger before `d1` and is held by `the_2026_09_30_series_rule_was_fixed_before_d1`;
  - the dev-host reading was taken at a coherent 154.0.4258.37 driver/runtime pair.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/

## 2026-09-30-mutation-gate-grades-every-tally-it-rests-on — the mutation gate's selftest and fixture tree registered
**Section:** §Stack and Technologies — "Operator instruments (host runtime)" row · §Infrastructure Patterns — Directory structure
**Change:**
- §Stack row: the mutation-tally gate's roster rows each carry a required `tally` (`missed` | `timeout`). Since 2026-09-30 a stdlib-only `selftest [--fixtures DIR]` verb grades the committed fixture tree `scripts/fixtures/mutation-gate/`, spawning no process and running no `cargo mutants`. The row's closing sentence stands: no CI step invokes any instrument, and none adds a sixth `agent-run.{sh,ps1}` command.
- Directory structure: the `scripts/` block gains `mutation-gate.py` (operator-local, `<unit>` | `selftest`, no CI step, neither harness shell, no 6th command), `mutation-roster.toml` (the expected `missed` / `timeout` multisets per unit) and `fixtures/mutation-gate/` (the `selftest` fixture tree).
**Why:** the chunk added the verb and the fixture tree to an already-registered operator instrument. The tree had no row for either gate file. Neither registry section was edited (38 111 / 38 028 B of 38 115, within target).
**Kept:** the tree still has no rows for the other operator instruments (`arch-registry-check.py`, `code-graph.py` and its companions). That gap predates this chunk and is left for the chunk that owns those files.
**Ref:** .andromeda/runs/2026-09-30T08-33-23-wrap/

## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — `wdio.conf.ts` sets `CONDUCTOR_SCENARIOS_DIR` for the driven suite too
**Section:** §Occupied Resources → Environment variables → `CONDUCTOR_SCENARIOS_DIR`
**Change:** the entry names its harness setter — `wdio.conf.ts` sets it for the `driven` / `sr-empty` / `sr-error` suites (was: the override alone). The driven suite's value is `runs/driven/scenarios` (repo-relative, harness-owned, git-ignored, re-created per run with exactly the two `[[checklist]]` scenarios); no new handle, no second owner — the app stays its sole reader, the one tauri-driver spawn site its setter.
**Why:** the driven arm now seeds its own two-scenario catalog so one run reaches two holds behind one canary. The line was kept to one clause because §Occupied Resources stood at 38 028 / 38 115 B (87 B of headroom) before this pass; after it, 38 087 B, within target.
**Kept:** §Established Decisions untouched (38 111 B, 4 B of headroom); `runs/driven/scenarios/` not registered as its own artifact line (per-item content inside the already-registered gitignored `runs/` tree).
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/

## 2026-09-30-the-sr-cause-isolated-on-this-host — host-tool handles: the only COMMITTED reader
**Section:** §Occupied Resources → Environment variables — the `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` rows
**Change:** Was "Read ONLY by `crates/conductor-tauri/ui/wdio.conf.ts`" and "`wdio.conf.ts`, the ONLY reader"; now "Its only COMMITTED reader is `crates/conductor-tauri/ui/wdio.conf.ts` (never a shipped binary)" and "the only COMMITTED reader". Every other clause of both rows stands: validation at the harness edge, array-form spawn, skip at exit 0 when unset.
**Why:** Gitignored, uncommitted session scripts also read both handles (guarded the wdio way) for the 2026-09-30 SR controls, so "only reader" was literally false. Narrowed by the overseer's ruling so the claim stays true going forward with no dated clutter. The arch registry carries STANDING committed readers and binders: a one-off session script is recorded in security rule (b) and chunk evidence, never here.
**Kept:** No arch row for `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` and no Ports or trust-boundary edit for the one-off `:4445` binder — rejected by the overseer (founder-delegated); a successor that COMMITS a reader or binder registers it then.
**Ref:** .andromeda/runs/2026-09-30T13-50-58-wrap/

## 2026-09-30-the-sr-pass-regrades-on-the-os-input-path — the SR leg's spawned children gain the key-send script
**Section:** §Cross-cutting Patterns → Trust boundary
**Change:** was "the screen-reader suites additionally spawn the host NVDA … and a fixed-argv PowerShell window-activation script, neither of which opens a listener (measured 2026-09-02)"; now they also spawn, per key, a fixed-argv PowerShell key-send script (`send-keys.ps1`, a closed `-Key` set, foreground-guarded — rule (b)'s eighth governed form), none of the three opening a listener (measured 2026-09-02; the key-send script 2026-09-30). No port, env handle or artifact path is added; the loopback-only claim is unchanged.
**Why:** the committed spawn the chunk shipped, registered where the leg's children are listed, on the founder's live ratification of the eighth form (relayed by the overseer). Per the standing registration rule (committed readers and binders register when committed), a committed spawn registers here now.
**Kept:** §Occupied Resources unchanged — the script binds nothing and reads no env handle.
**Ref:** .andromeda/runs/2026-09-30T15-22-00-wrap/

## 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix — the posture contract's third series and its re-pin
**Section:** §Occupied Resources → On-disk artifacts (`contracts/pulse-real-model-leg-posture.md`) · §Standard Contracts (the corpus-text exception's summary)
**Change:**
- The posture-contract row names three drive series — 2026-09-29, 2026-09-30, 2026-10-01, each an add-only section fixed before its first drive. Was: two ("2026-09-29, then 2026-09-30").
- Its provenance clause: Pulse coordinates read at HEAD `e98d838`, re-pinned `fcc31b2` 2026-09-30 and `a2addb3` 2026-10-01. Was: the 2026-09-30 re-pin alone ("the scrubber fix").
- The §Standard Contracts summary of security-plan's Data Protection record names a second residual beside the frozen 2026-09-22 file's: the graded 2026-10-01 d3 capture's all-digit synthetic prefix, which `elide_fingerprints` keeps by definition — overseer-ruled, founder ratification pending.
**Why:** the chunk added the 2026-10-01 series add-only to the contract against Pulse `a2addb3`; the summary must not cite security-plan as saying less than it now says. The row's edit is byte-negative so §Occupied Resources stays within the registry-size target.
**Ref:** .andromeda/runs/2026-10-01T20-39-22-wrap/

## 2026-10-01-per-run-span-identity-in-the-real-model-harness — per-execution span identity; the live-suite second writer
**Section:** §Cross-cutting Patterns → Determinism discipline · §Occupied Resources → `contracts/pulse-run-contract.toml` · §Occupied Resources → `runs/live-suite/{leg}.jsonl` · §Infrastructure Patterns → Directory structure (`live-suite/`)
**Change:**
- Determinism discipline: "same scenario + seed ⇒ same stream shape" now names what is seed-pure: timing, counts, order and content, all drawn from `emission_seed`. Span IDENTITY is the one emitted value a wall-clock read reaches. On the production path, `execute_scenario` passes its `std::time` `emitted_ms` as `Dispatcher::connect(…, identity_salt: Some(_))`, and `conductor_emit::rekey_trace_identity` re-keys every trace export's ids. The re-key is an XOR with masks from a salt-seeded `ChaCha8Rng`: a bijection per salt that preserves linkage and moves no content byte. The unsalted tier (`None`) stays seed-pure, as pinned by the `dispatch_wire__*` goldens. As measured at the chunk's `evidence/witness-ledger.md`: two same-seed drives 253 507 ms apart inside Pulse's 600 s retention, with zero refused appends.
- The run-contract canary-identity passage now cites Pulse's span key at `schema.rs:36` (read at `a2addb3`; it was `:38`). It retires "`run_consumer` logs-and-skips a colliding row": a replay rejects the WHOLE batch at flush. Pulse logs it as `duckdb.append` `reject_reason=append_failed` plus a `buffer.tick` `append_rejections` count (Pulse's own `appender.rs:996` test). That detail lives here, not in the body, which sits at the registry-size cap.
- `runs/live-suite/` gains a second, operator-local writer class: the span-landing operator pass's plain `cp` of `span-{a,b}.jsonl` after each drive. Only the `span_landing_live` witness reads them. They are not cleared before the pass's drives, and their move to a harness-owned subdir is route-owned. The entry's own wording was compressed without dropping a fact, to hold §Occupied Resources at 38112 B ≤ 38115 B. The tree line names the same second writer.
**Why:** the chunk ships per-execution span identity. The live witness measured the protection, and the Pulse source re-read at `a2addb3` falsified the log-and-skip consequence. The D-arch-collision escalation was ruled by the overseer at this wrap: "register the second writer truthfully now, and carry the move as a CARRY on the next entry".
**Kept:** "same scenario + seed ⇒ same stream shape" at `:3` and the Determinism bullet's own head: shape stays seed-pure.
**Ref:** .andromeda/runs/2026-10-01T23-55-00-wrap/

## 2026-10-02-p-075-assert-round-against-pulse — the span pair in its own dir; the read-back posture re-measured at S
**Section:** §Occupied Resources → `runs/live-suite/{leg}.jsonl` · §Occupied Resources → `runs/span-landing/span-{a,b}.jsonl` (new) · §Infrastructure Patterns → Directory structure (`live-suite/`, `span-landing/`) · §Established Decisions → [Read-Back Dependency Posture] (the canary-carrier passage)
**Change:**
- `runs/span-landing/span-{a,b}.jsonl` is registered. It holds the span-landing pass's drive journals, has no `CONDUCTOR_*` handle and is git-ignored. Its two named files are cleared by `rm -f` before drive A. `span_landing_live` reads it via `capture_paths`, and a stale pair is refused. The tree gains a `span-landing/` line.
- The `live-suite/` entry and its tree comment lose their second, operator-local writer. That writer was registered in the 2026-10-01 per-run span-identity entry's third bullet, the only part of that entry this retires. The suite's `rm -f …/*.jsonl` no longer reaches the pair.
- To hold §Occupied Resources within target, the entry's wording was compressed without dropping a fact: the "else a suite read sees only the last leg" gloss went, along with "(not `run_id`-stemmed)" and a shorter stale-file clause.
- The posture passage now reads that from `83d4060`, `fingerprint_refs` are the model's refs ∪ the cue fingerprint. That set is written at creation only, and a dedupe never updates it. At Pulse S `03ec944`, Conductor's emitted fingerprint is a member (4 refs, 3 `det-*`), and that incident's `retrieve_report` reads `degraded_mode: false`. The evidence is cited as measured at the chunk's `evidence/round-ledger.md`.
- The passage was "as measured at" the 2026-09-10 `leg1` envelope, quoting its 32-hex value, and named `inference_runtime.rs`. Both citations moved here, as history, to hold §Established Decisions within target.
- Bytes: §Established Decisions 38097 B and §Occupied Resources 38111 B, both ≤ 38115 B.
**Why:** this chunk moved the span pair out of `live-suite/`, which discharges the route-owned move. The P-075 round re-measured read-back content fidelity at a newer Pulse HEAD. §Established Decisions already states `degraded` per-read-back, so no contrary arch claim remained.
**Kept:** `:70`'s KnownResidual example "a `retrieve_report` result returned under `degraded_mode`" and the per-read-back clause, both true. The `incident_events` paragraph is unchanged: no MCP tool reads it.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/

## 2026-10-02-captured-fingerprint-values-elided — the Corpus-access residual clauses retired as fixed
**Section:** §Standard Contracts → Readiness gate ("Corpus access" — the `corpus.db` parenthetical's tail)
**Change:** The clause was "the frozen 2026-09-22 file keeps its one `fingerprint_hex` prefix as a stated residual; a second — the graded 2026-10-01 d3 capture's all-digit synthetic prefix, which `elide_fingerprints` keeps by definition — is overseer-ruled, founder ratification pending". It now reads that both stated residuals were fixed 2026-10-02 under the founder's ruling, the frozen file elided in place. The edit is byte-negative (−59 B), and both arch registries stay within target.
**Why:** Lockstep with security-plan §Security Anti-Patterns → Data Protection, which owns the elision chain. The founder ruled the residuals fixed, never ratified (2026-10-02, relayed by the overseer).
**Ref:** .andromeda/runs/2026-10-02T16-24-28-wrap/
