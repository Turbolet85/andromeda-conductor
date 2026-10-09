# 1A tree — conductor-0.4.0 (third derivation)

_Phase 1 sub-step 1A, after the operator's edit of F7b at the second Phase 4. Hierarchical outline per epoch; the
sub-block tier is visible here and dropped in 1C. Each leaf: candidate title + `(source: …)` — the requirement id
it serves, then the master section it BUILDS ON, REPLACES or RETIRES. A plan section is cited only where it was
opened in this run (`plans-not-opened.md`); the keyed §3 and §Infrastructure Patterns files were opened in this
pass and are cited as `key …`. Pulse entries are named as they stand in `andromeda-pulse-0.4.0/working-route.md`
at `b3ac58a`. `carried` = an entry of the second pass's merged draft, re-checked against the revised `v4-18` and
unchanged; `reworded` and `new` say what this pass did._

## Epoch 1 — Foundation: a console program on Linux, read through the door

### Sub-block — what leaves first
- Conductor's window retired · carried (source: v4-01, v4-09 · RETIRES architecture §Stack — Desktop shell / Desktop frontend / Push, §Established Decisions [Real-time Strategy] and the `a11y` half of [CI/CD], §Occupied Resources — ports 4444/4445, the msedgedriver egress, the Tauri handles; architecture key `build-system` — the frontend bundle step and the Tauri-tree `deny.toml` entries; security-plan §Dependency Security — the frontend supply chain, the third dependency class and the Tauri-tree accepted exceptions, §Security Anti-Patterns → Code Patterns — the driver-stack, seventh and eighth spawn forms and the Tauri bans; obs-plan §1 — the desktop-webview and ipc-internal surfaces; test-plan §1 — the desktop-webview and Tauri ipc surfaces, §6 — the drivers table, §9 — the E2E (webview) stage, §10 — the `tauri` floor among the build-failure conditions and the roster's `conductor-tauri` member; a11y-plan and all nine of its keys (tool pick, bootstrap phases, CI integration, contrast, focus, keyboard, screen-reader pattern, violation schema, WCAG mapping) — read here: its only assertable surface is the webview and it flags the command line not-assertable in §1, §4, §5, §9 and §11; the window sections of design-system.md and layout-templates.md, NOT opened — named as masters this chunk's wrap re-subjects, not as cited sections)
- Panel-shaped types retired · carried (source: v4-01, v4-09 · RETIRES architecture §Standard Contracts — Live update channel, §Conventions — Config conventions' dialog-row bound, §On-disk artifacts — `logs/conductor-tauri.jsonl`, `runs/a11y/`; obs-plan §1 — the Tauri logging sinks and log file locations, §4 — both-surface parity; test-plan §1 — Critical Path 7, §5 — cross-surface parity, §6 — Scenario: Both-surface parity, §7 — the webview-arm fixture trees; BUILDS ON layout-templates §Surface: cli — Primary screens)

### Sub-block — Linux
- Linux-only base CI · carried (source: v4-07 · RETIRES architecture §Established Decisions [CI/CD] — the Windows runners, `agent-run.ps1`; architecture keys `ci-cd-approach` and `build-system` — "every CI job runs on a Windows runner"; test-plan key `bootstrap-phases` — the two-shell binding; test-plan §9 — Matrix builds; BUILDS ON security-plan §Bootstrap phases — dep-security-ci-gate, secret-scanning-ci-gate; test-plan §6 — the static-gate legs)
- Fast feedback on Linux · carried (source: v4-07 · BUILDS ON test-plan §9 — Pipeline structure, §11 → CI; `phase-1/synthesis-protocol.md` §Placement)

### Sub-block — the door
- Engine read through the door · carried (source: v4-12 · REPLACES architecture §Established Decisions [MCP Read-Back Client], §Conventions — Inbound verification; security-plan §Input Validation — the MCP child stdout row; test-plan §1 — the ipc-internal read-back surface, §5 — the stub tier, §8 — what to mock; follows Pulse's `Door inside the engine's process`)
- Door reads checked without an engine · carried (source: v4-12 · REPLACES test-plan §5 — Boundary types' sidecar-stub row and the item-key fidelity note, §8 — What to mock; key `5-command implementation` — `boot` against the stub in CI)
- Engine-backed check pipe reachable · carried (source: v4-12, as revised — checks that need an engine run where an engine is; CI runs only what claims Conductor alone and holds no engine credential · [GATE_REACHABILITY]; REPLACES test-plan §9 — Live-Pulse scenarios, §11 → CI and → Test Strategy's "never fake Pulse's reaction as a CI verdict", which the revised requirement keeps)

## Epoch 2 — One way to describe a run

- World of named services · carried (source: v4-13 · BUILDS ON architecture §Service / process names — emitted OTLP service identities, §Conventions — Outbound emission's shared vocabulary; input.md §Capabilities 2 — virtual topology)
- Schedule on one timeline · carried (source: v4-15, v4-16 · BUILDS ON architecture §Design Philosophy — Determinism under a seed, §Established Decisions [Determinism RNG], §Cross-cutting — Determinism discipline; test-plan §7 — the seeded stream goldens)
- Declared truth per event · carried (source: v4-17)
- Run description: world plus schedule · carried (source: v4-16, v4-06 · BUILDS ON architecture §Established Decisions [Validation Library], [Scenario Config Format]; security-plan §Input Validation — the scenario-config row, §Security Anti-Patterns → Input; test-plan §1 — the config-parsing coverage trigger)
- Discrimination at description time · reworded (source: v4-18, as revised — a paired control differing in exactly one declared thing, graded differently · REPLACES architecture §On-disk artifacts — `contracts/scenario-audit-ledger.toml`'s after-the-fact record; test-plan §6 — the scenario-assertion audit leg, whose `discriminates` flag is "recorded, never graded")

## Epoch 3 — The reaction on the timeline

### Sub-block — reading
- Gate before a run, through the door · carried (source: v4-12, as revised — admission is the engine owner's consent; nothing is sent to an engine that has not admitted Conductor · REPLACES architecture §Standard Contracts — Readiness gate, §Established Decisions [Read-Back Dependency Posture]; security-plan §Security Anti-Patterns → Universal — the five named preconditions)
- Run attribution · carried (source: v4-12 · BUILDS ON architecture §Established Decisions [Read-Back Dependency Posture] — freshness as the carrier, runtime-state fidelity)
- Reaction log on the timeline · carried (source: v4-19 · REPLACES security-plan §Security Anti-Patterns → Data Protection — the corpus-content ban and its one exception; architecture §Standard Contracts — Corpus access)

### Sub-block — grading and reading back
- Per-event grading · carried (source: v4-20, v4-05 · REPLACES architecture §Established Decisions [Timing-Tolerance Model], [Probabilistic-Assertion Policy]; obs-plan §5, §10 — the tier budgets)
- Run record for the one form · carried (source: v4-19, v4-20 · REPLACES architecture §Standard Contracts — Run report envelope, Per-check record, §On-disk artifacts — `runs.db`; obs-plan §6 — the envelope schema; test-plan keys `status-endpoint-shape` and `log-format`, §7 — the envelope/journal goldens; residual `residuals.md:19`)
- Run read from the command line · carried (source: v4-09, v4-20, v4-03 · REPLACES design-system §Surface: cli — Tokens, Component Patterns 1–4; layout-templates §Surface: cli — Signature placement, Output structure, results table, verdict lines; the console's signature once the hold leaves is the design master's own rule re-homed — the operator's word)
- Harness verbs on the one form · carried (source: v4-09, v4-12 · REPLACES test-plan §1 — the 5-command requirements, §6 — the per-scenario `boot` / `run` steps; keys `5-command-implementation`, `test-data-bootstrap`)
- Own-log gates on the one form · carried (source: v4-12 · REPLACES obs-plan §9 — the producer of the log both gates grade; key `bootstrap-phases-for-downstream-skills` — obs-ci-gate-wire)

## Epoch 4 — Regression proofs through the door

- Accepted capability set re-based · carried (source: v4-06 · REPLACES architecture §Established Decisions [Accepted Capability Set], §On-disk artifacts — `contracts/pulse-capabilities.toml`, `coverage-matrix.md`; test-plan §6 — the coverage-completeness gate and its scenario-backing leg; `.andromeda/refs/`; follows Pulse's `Capability record re-based`; built beside the 0.3.0 pin)
- Short regression runs: rates and error scope · carried (source: v4-16, v4-18 · intent §3; REPLACES test-plan §1 — Critical Path 1's harvest-tier proof)
- Short regression runs: identity and capture · carried (source: v4-16, v4-18 · intent §3; REPLACES test-plan §1 — Critical Path 2 and the `pii-scrub` harvest proof)
- Short regression runs: quiet and lifecycle · reworded — the silence subjects moved to an entry of their own (source: v4-16, v4-18, v4-06 · intent §3; REPLACES test-plan §1 — Critical Paths 3 and 4, §6 — their scenarios; the three unclear subjects)
- Short regression runs: silence subjects · new (source: v4-16, v4-18 as revised — the control keeps the event and removes what should silence it, graded "reported" · intent §3 — the activity floor and restart suppression; REPLACES test-plan §1 — Critical Path 3's bypass arm, §6 — Scenario: Restart-suppression incl. one bypass case)
- Capability backing gate on the one form · carried (source: v4-06, v4-16 · REPLACES test-plan §6 — the scenario-backing leg)
- Regression set driven against a live engine · reworded — "graded differently" (source: v4-16, v4-18 · intent §7 point 3; readiness for Pulse's `Theme 0 checked by the external harness`)

## Epoch 5 — What leaves with the old form

### Sub-block — the proofs and the subjects
- Log-file harvest and live suite retired · carried (source: v4-04 · RETIRES test-plan §1 — the harvest-tier critical paths, §2 — the operator-local wall-clock legs, §5 — the live legs, §9 — Live-Pulse scenarios; security-plan §Input Validation — the capture and span-landing ingest rows; architecture §On-disk artifacts — `runs/live-suite/`, `runs/span-landing/`)
- Port fault and shared-machine scenarios retired · carried (source: v4-04 · RETIRES architecture §Occupied Resources — Ports, §Cross-cutting — Trust boundary's "single deliberate exception"; security-plan §Threat Model Summary — the port-bind vector; obs-plan §4 — `fault.port_occupier`; test-plan §1 — the chaos-test trigger's occupier half)
- Local-model grading retired · carried (source: v4-02 · RETIRES architecture §On-disk artifacts — `contracts/pulse-real-model-leg-posture.md`, §Established Decisions [Read-Back Dependency Posture] — the L4 postures; security-plan §Input Validation — the `l4_posture` key and the real-model capture row; obs-plan §4 — Real-model posture; test-plan §6 — the real-model interpretation leg, Scenario: Known-residual classification)
- Window grading retired · carried (source: v4-03 · RETIRES architecture §On-disk artifacts — `contracts/pulse-p025-measurement-contract.md`, §Conventions — the `[[checklist]]` table; obs-plan §4 — the delegated-timing family; layout-templates §Surface: cli — the hold-point component; design-system §Surface: cli — Component Patterns 1 and 2; test-plan §1 — the drive+observe untestable zone, §11 → Universal's checklist exception)

### Sub-block — the form, its gate, its vocabulary
- Single-list scenario form retired · carried (source: v4-16, v4-06 · RETIRES architecture §Conventions — Config conventions, §On-disk artifacts — `contracts/scenario-audit-ledger.toml`, §Established Decisions [CI/CD] — the scenario-assertion audit gate; security-plan §Input Validation — the scenario-config row; test-plan §6 — the scenario-assertion audit leg, §7 — the `wrapped-gloss` fixture)
- Spawned sidecar and shared-machine gate retired · carried (source: v4-04 · RETIRES architecture §Service / process names — launched child process, §Environment variables — the `ANDROMEDA_PULSE_*` handles, §On-disk artifacts — `contracts/mcp-contract.toml`, `contracts/pulse-run-contract.toml`; security-plan §Security Anti-Patterns → Code Patterns rule (a), → Input — the sidecar bans; test-plan §8 — the `stub_pulse_mcp` rows, §10 — roster members 1, 3 class C and 4, §11 → Mocking's sidecar bans.)
- Model and manual vocabulary retired · carried (source: v4-02, v4-03 · RETIRES architecture §Design Philosophy — "Outcomes are values" enumerations, §Established Decisions [Probabilistic-Assertion Policy]; design-system §Surface: cli — the Manual and Hold token rows, the Residual row's lamp use; test-plan §11 → E2E's reported-states ban)
- Short-run tiers retired · carried (source: v4-05 · RETIRES architecture §Established Decisions [Timing-Tolerance Model], §Conventions — Data model conventions' `slo_tier` column; obs-plan §5, §10)

## Epoch 6 — The network and the credentials

### Sub-block — before a credential is held
- Security posture restated for a harness holding credentials · carried (source: v4-11 · REPLACES security-plan §Threat Model Summary — tier, data classification, attack surface, auth model; §Secret Management; §Data Protection; the loaded security rule)
- Credential hygiene · carried (source: v4-11 · REPLACES security-plan §Secret Management, §Security Anti-Patterns → Secrets, → Logging; BUILDS ON obs-plan §11 — PII Scrubbing's field allowlist; security-plan §Bootstrap phases — secret-scanning-ci-gate; design-system's universal ban on a styled credential surface, read at its one line)
- Engine address and identity given to a run · carried (source: v4-10, v4-11, as revised — what Conductor verifies the engine against is given with the address, its source stated; the address stays out of committed artifacts and a record names the engine by the operator's label · BUILDS ON security-plan §Error Handling — run-report artifact sanitization, §Security Anti-Patterns → Logging)

### Sub-block — the path
- Two-host path reachable · carried (source: v4-10, v4-04 · [GATE_REACHABILITY], placed here; REPLACES test-plan §11 → Universal's "loopback only"; follows Pulse's `Network OTLP receiver behind the token` and `Door reachable from another host`)
- Channel refusals · carried (source: v4-10, v4-11 · REPLACES security-plan §Data Protection — "No HTTPS/TLS surface", §Security Anti-Patterns → Data Protection — the TLS ban)
- Fixed local address retired · carried (source: v4-04, v4-10 · RETIRES architecture §Conventions — Outbound emission's fixed address, §Standard Contracts — Liveness equivalent, §Occupied Resources — Ports; test-plan §10 — roster member 3 class B, which exists because the endpoint is not injectable)
- Host clock difference measured · carried (source: v4-20 · BUILDS ON architecture §Cross-cutting — Determinism discipline's wall-clock rule)

## Epoch 7 — The living world and the long run (paired with Pulse's Epoch 6)

### Sub-block — the bound and the world
- Load bound re-measured · carried (source: v4-05 · REPLACES architecture §On-disk artifacts — `contracts/pulse-load-envelope.toml`; security-plan §Input Validation — the manifests row; test-plan §1 — the chaos-test trigger's "bounded, never load", §10 — Performance budgets; follows Pulse's `Telemetry store on disk`)
- Own log over a run of hours · carried (source: v4-21, v4-09 · REPLACES obs-plan keys `heartbeat-ticks` ("CLI: N/A") and `logging-stack` (the one-writer sink); §6 — Sink configuration)
- Living background · carried (source: v4-14)
- Four event families · carried (source: v4-15 · intent §6 — «Out of this version»)

### Sub-block — hours
- Run written as it goes · carried (source: v4-21, v4-09 · REPLACES architecture §On-disk artifacts — the per-run journal; layout-templates §Surface: cli — the heartbeat line)
- Disk bound over ten hours · carried (source: v4-21, as revised — record, journal and log bounded in size on disk, the bound stated · REPLACES obs-plan key `log-file-location` — "Rotation: N/A"; test-plan key `5-command-implementation` — `logs` retention, "no rotation needed — bounded synthetic runs")
- Engine memory and database growth read through the door · carried (source: v4-21, as revised, v4-04 · follows Pulse's `Telemetry store on disk`; the door offering these two figures is NOT stated by any entry of Pulse's route — a line owed on Pulse's side)

### Sub-block — the whole run
- Whole-run grading · carried (source: v4-20)
- Recorded run replayed · carried (source: v4-22 · follows Pulse's `Recorded stream replays to the same result`)

## Epoch 8 — Polish & ship

### Sub-block — the pair's scenario
- Notification record read through the door · carried (source: v4-12, as revised · readiness for Pulse's `Theme 5 checked by the external harness`; the door offering the notification record is NOT stated by any entry of Pulse's route — a line owed on Pulse's side)
- Bad-version rollout end to end · carried (source: v4-23 · intent §7 point 1; pairs with Pulse's `Bad-version scenario end to end`)
- Agent's answer taken in · carried (source: v4-23, as revised, v4-11 · BUILDS ON security-plan §Input Validation — the untrusted-text ingest rows; no spawn form is added, so §Security Anti-Patterns → Code Patterns is not touched)
- Large-model reading recorded · carried (source: v4-23, as revised — Conductor does not start the agent)
- Ten-hour run completed · carried (source: v4-21 · intent §7 point 2)

### Sub-block — the close
- Records restated · carried (source: v4-08 · RETIRES architecture §Project Intent, key `deployment-model` — "necessarily co-located on the dev host", §Cross-cutting — Trust boundary, Scope law; input.md; test-plan §11 → Universal's project-specific ban)
- No-survivor sweep · carried (source: v4-24 · intent §7 point 4)
- Version close · carried (source: v4-06 · pairs with Pulse's `Version close on Linux`)

## Phase 3 — entries the third merge added or reworded (decisions in `merge-decisions.md`)

- Epoch 3 · `Two runs back to back` added after `Harness verbs on the one form` (source: v4-12, v4-18 as revised · REPLACES test-plan §6 — the firing form's quiet window between legs, §11 → E2E — the between-legs wait; test-plan key `5-command-implementation` — the live suite's leg order)
- Epoch 4 · `Short regression runs: quiet and lifecycle` gains "engine start-state dependence stated" (source: v4-12, v4-18 · test-plan §9 — Live-Pulse scenarios' uptime precondition, §10 — Zero-flakiness budget)
- Epoch 6 · `Channel refusals` names the door credential and the rejected-credential arm (source: v4-10, v4-11 · security-plan §Security Anti-Patterns → Data Protection, → Universal)
- Epoch 6 · `Channel refusals checked without an engine` added after `Channel refusals` (source: v4-10, v4-11, v4-12 · BUILDS ON test-plan §1 — the security-vector negative-test trigger, §8 — What to mock, the loopback egress stub)
