# 1A tree — conductor-0.4.0

_Phase 1 sub-step 1A. Hierarchical outline per epoch; the sub-block tier is visible here and dropped in 1C. Each
leaf: candidate title + `(source: …)` — the requirement id it serves, then the master section it BUILDS ON or
RETIRES. A plan section is cited only where it was opened in this run (`plans-not-opened.md`). Pulse entries are
named as they stand in `andromeda-pulse-0.4.0/working-route.md` at `b3ac58a`._

## Epoch 1 — Foundation: a console program on Linux, read through the door

- Conductor's window retired (source: v4-01, v4-09 · RETIRES architecture §Stack — Desktop shell / Desktop frontend / Push, §Established Decisions [Real-time Strategy] and the `a11y` half of [CI/CD], §Occupied Resources — ports 4444/4445, the msedgedriver egress, the Tauri handles; security-plan §Dependency Security — frontend supply chain and the third dependency class, §Security Anti-Patterns → Code Patterns — the driver-stack, seventh and eighth spawn forms and the Tauri bans; obs-plan §1 — the desktop-webview and ipc-internal surfaces; test-plan §1 — the desktop-webview and Tauri ipc surfaces, §6 — the drivers table, §9 — the E2E (webview) stage and the three Windows jobs; the window subject of design-system.md, layout-templates.md and a11y-plan.md, whose window sections were NOT opened in this run — named as the masters this chunk's wrap re-subjects, not as cited sections)
- Panel-shaped types retired outside the window crate (source: v4-01, v4-09 · RETIRES architecture §Standard Contracts — Live update channel, §Conventions — Config conventions' dialog-row bound, §On-disk artifacts — `logs/conductor-tauri.jsonl`, `runs/a11y/`; obs-plan §1 — the Tauri logging sinks and log file locations, §4 — both-surface parity; test-plan §1 — Critical Path 7, §5 — cross-surface parity; BUILDS ON layout-templates §Surface: cli — Primary screens)
- Linux-only base CI and harness (source: v4-07 · RETIRES architecture §Established Decisions [CI/CD] — the Windows runners, `agent-run.ps1`; test-plan §9 — Matrix builds; layout-templates §Surface: cli — Primary screens' two-shell line; BUILDS ON `phase-1/synthesis-protocol.md` §Placement — fast feedback)
- Engine read through the door (source: v4-12 · REPLACES architecture §Established Decisions [MCP Read-Back Client], §Conventions — Inbound verification; security-plan §Input Validation — the MCP child stdout row; test-plan §1 — the ipc-internal read-back surface, §5 — the stub tier; follows Pulse's `Door inside the engine's process`)
- Engine-backed check pipe reachable (source: v4-12 · [GATE_REACHABILITY]; REPLACES architecture §Established Decisions [CI/CD] — "dynamic scenario proof requiring a live Pulse remains a local operator gate"; test-plan §2 — agent-runnable invariants, §9 — Live-Pulse scenarios)

## Epoch 2 — One way to describe a run

- World of named services (source: v4-13 · BUILDS ON architecture §Service / process names — emitted OTLP service identities, §Conventions — Outbound emission's shared vocabulary; input.md §Capabilities 2 — virtual topology)
- Schedule on one timeline (source: v4-15, v4-16 · BUILDS ON architecture §Design Philosophy — Determinism under a seed, §Established Decisions [Determinism RNG], §Cross-cutting — Determinism discipline)
- Declared truth per event (source: v4-17)
- Run description: a world plus a schedule (source: v4-16, v4-06 · BUILDS ON architecture §Established Decisions [Validation Library], [Scenario Config Format]; security-plan §Input Validation — the scenario-config row, §Security Anti-Patterns → Input)
- Discrimination decided at description time (source: v4-18 · REPLACES architecture §On-disk artifacts — `contracts/scenario-audit-ledger.toml`'s after-the-fact record)

## Epoch 3 — The reaction on the timeline

- Gate before a run, through the door (source: v4-12 · REPLACES architecture §Standard Contracts — Readiness gate, §Established Decisions [Read-Back Dependency Posture]; security-plan §Security Anti-Patterns → Universal — the five named preconditions)
- Run attribution (source: v4-12 · BUILDS ON architecture §Established Decisions [Read-Back Dependency Posture] — freshness as the carrier, runtime-state fidelity)
- Reaction log on the timeline (source: v4-19 · REPLACES security-plan §Security Anti-Patterns → Data Protection — the corpus-content ban and its one exception; architecture §Standard Contracts — Corpus access)
- Per-event grading (source: v4-20, v4-05 · REPLACES architecture §Established Decisions [Timing-Tolerance Model], [Probabilistic-Assertion Policy]; obs-plan §5, §10 — the tier budgets)
- Run record for the one form (source: v4-19, v4-20 · REPLACES architecture §Standard Contracts — Run report envelope, Per-check record, §On-disk artifacts — `runs.db`; obs-plan §6 — the envelope schema; test-plan §1 — Status endpoint shape (the §3 key file was not opened); residual `residuals.md:19`)
- Run read from the command line (source: v4-09, v4-20 · REPLACES layout-templates §Surface: cli — Output structure, results table, verdict lines; design-system §Surface: cli — Tokens, Component Patterns 3 and 4)

## Epoch 4 — Regression proofs through the door

- Accepted capability set re-based on Pulse's current record (source: v4-06 · REPLACES architecture §Established Decisions [Accepted Capability Set], §On-disk artifacts — `contracts/pulse-capabilities.toml`, `coverage-matrix.md`; `.andromeda/refs/`; follows Pulse's `Capability record re-based`)
- Short regression runs: rates, latency, error scope (source: v4-16, v4-18 · intent §3 — the kept subjects)
- Short regression runs: identity and capture (source: v4-16, v4-18 · intent §3)
- Short regression runs: quiet and lifecycle (source: v4-16, v4-18, v4-06 · intent §3; the three unclear subjects)
- Regression set driven against a live engine (source: v4-16 · intent §7 point 3; readiness for Pulse's `Theme 0 checked by the external harness`)

## Epoch 5 — What leaves with the old form

### Sub-block — the shared machine
- Log-file harvest and live suite retired (source: v4-04 · RETIRES test-plan §1 — the harvest-tier critical paths, §5 — the live legs, §9 — Live-Pulse scenarios; security-plan §Input Validation — the capture and span-landing ingest rows; architecture §On-disk artifacts — `runs/live-suite/`, `runs/span-landing/`)
- Spawned sidecar and shared-machine gate retired (source: v4-04 · RETIRES architecture §Service / process names — launched child process, §Environment variables — the `ANDROMEDA_PULSE_*` handles, §On-disk artifacts — `contracts/mcp-contract.toml`, `contracts/pulse-run-contract.toml`; security-plan §Security Anti-Patterns → Code Patterns rule (a), → Input — the sidecar bans)
- Port fault and shared-machine scenarios retired (source: v4-04 · RETIRES architecture §Occupied Resources — Ports, §Cross-cutting — Trust boundary's "single deliberate exception"; security-plan §Threat Model Summary — the port-bind vector; obs-plan §4 — `fault.port_occupier`)

### Sub-block — the subjects
- Local-model grading retired (source: v4-02 · RETIRES architecture §On-disk artifacts — `contracts/pulse-real-model-leg-posture.md`, §Established Decisions [Read-Back Dependency Posture] — the L4 postures; security-plan §Input Validation — the `l4_posture` key and the real-model capture row; obs-plan §4 — Real-model posture)
- Window grading retired (source: v4-03 · RETIRES architecture §On-disk artifacts — `contracts/pulse-p025-measurement-contract.md`, §Conventions — the `[[checklist]]` table; obs-plan §4 — the delegated-timing family; layout-templates §Surface: cli — the hold-point component; design-system §Surface: cli — Component Patterns 1 and 2)

### Sub-block — the form and its vocabulary
- Single-list scenario form retired (source: v4-16 · RETIRES architecture §Conventions — Config conventions, §On-disk artifacts — `contracts/scenario-audit-ledger.toml`, §Established Decisions [CI/CD] — the scenario-assertion audit gate; security-plan §Input Validation — the scenario-config row)
- Old verdict vocabulary retired (source: v4-02, v4-03, v4-05 · RETIRES architecture §Design Philosophy — "Outcomes are values" enumerations, §Established Decisions [Timing-Tolerance Model], [Probabilistic-Assertion Policy], §Conventions — Data model conventions' `slo_tier` column; obs-plan §5, §10; design-system §Surface: cli — the Manual, Residual and Hold token rows)

## Epoch 6 — The network and the credentials

- Security posture restated for a harness that holds credentials (source: v4-11 · REPLACES security-plan §Threat Model Summary — tier, data classification, attack surface, auth model; §Secret Management; §Data Protection)
- Two-host path reachable (source: v4-10, v4-04 · [GATE_REACHABILITY], placed here; follows Pulse's `Network OTLP receiver behind the token` and `Door reachable from another host`)
- Channel refusals (source: v4-10, v4-04 · REPLACES architecture §Conventions — Outbound emission, §Occupied Resources — Ports, §Standard Contracts — Liveness equivalent; security-plan §Data Protection — "No HTTPS/TLS surface", §Security Anti-Patterns → Data Protection — the TLS ban)
- Credential hygiene (source: v4-11 · REPLACES security-plan §Secret Management, §Security Anti-Patterns → Secrets, → Logging; BUILDS ON obs-plan §11 — PII Scrubbing's field allowlist; security-plan §Bootstrap phases — secret-scanning-ci-gate)
- Host clock difference measured (source: v4-20 · BUILDS ON architecture §Cross-cutting — Determinism discipline's wall-clock rule)

## Epoch 7 — The living world and the long run

### Sub-block — the world lives
- Living background (source: v4-14)
- Four event families (source: v4-15 · intent §6 — «Out of this version»)
- Load bound re-measured (source: v4-05 · REPLACES architecture §On-disk artifacts — `contracts/pulse-load-envelope.toml`; security-plan §Input Validation — the manifests row)

### Sub-block — hours
- Run written as it goes (source: v4-21, v4-09 · REPLACES obs-plan §1 — Heartbeat ticks "CLI: N/A", §6 — Sink configuration's "File rotation: N/A"; layout-templates §Surface: cli — the heartbeat line; architecture §On-disk artifacts — the per-run journal)
- Engine memory and database growth measured (source: v4-21 · follows Pulse's `Engine memory measured`)

### Sub-block — the whole run
- Whole-run grading (source: v4-20)
- Recorded run replayed (source: v4-22 · follows Pulse's `Recorded stream replays to the same result`)

## Epoch 8 — Polish & ship

### Sub-block — the pair's scenario
- Bad-version rollout end to end (source: v4-23 · intent §7 point 1; pairs with Pulse's `Bad-version scenario end to end`)
- Large-model reading recorded (source: v4-23 · security-plan §Security Anti-Patterns → Code Patterns — a new spawn class escalates)
- Ten-hour run completed (source: v4-21 · intent §7 point 2)

### Sub-block — the close
- Records restated (source: v4-08 · RETIRES architecture §Project Intent, §Cross-cutting — Trust boundary, Scope law; input.md; the security rule's opening line)
- No-survivor sweep (source: v4-24 · intent §7 point 4)
- Version close (source: v4-06 · pairs with Pulse's `Version close on Linux`)

## Phase 3 — entries the merge added, split or moved (sources as above; decisions in `merge-decisions.md`)

- Epoch 1 · `Linux-only base CI and harness` split into `Linux-only base CI` and `Fast feedback on Linux` (source: v4-07 · BUILDS ON security-plan §Bootstrap phases — dep-security-ci-gate, secret-scanning-ci-gate; test-plan §9 — Pipeline structure, Matrix builds)
- Epoch 3 · `Run read from the command line` now carries the console design restated without the hold (source: v4-09, v4-20, v4-03 · REPLACES design-system §Surface: cli — Tokens, Component Patterns 1–4; layout-templates §Surface: cli — Signature placement, Output structure)
- Epoch 3 · `Harness verbs on the one form` added (source: v4-09, v4-12 · REPLACES test-plan §1 — the 5-command requirements; the §3 key files `5-command implementation` and `Status endpoint shape` were not opened by the orchestrator, the tests validator cites them)
- Epoch 5 · `Spawned sidecar and shared-machine gate retired` moved after `Single-list scenario form retired`; `Old verdict vocabulary retired` split into `Model and manual vocabulary retired` (v4-02, v4-03) and `Short-run tiers retired` (v4-05)
- Epoch 6 · `Credential hygiene` moved ahead of `Two-host path reachable`; `Fixed local address retired` split out of `Channel refusals` (source: v4-04, v4-10 · RETIRES architecture §Conventions — Outbound emission's fixed address, §Standard Contracts — Liveness equivalent, §Occupied Resources — Ports)
- Epoch 7 · `Own log over a run of hours` added (source: v4-21, v4-09 · REPLACES obs-plan §1 — Heartbeat ticks, Logging stack's one-writer sink; §6 — Sink configuration)
- Epoch 8 · `Agent start boundary` added (source: v4-23, v4-11 · BUILDS ON security-plan §Security Anti-Patterns → Code Patterns, §Input Validation — the untrusted-text ingest rows)
