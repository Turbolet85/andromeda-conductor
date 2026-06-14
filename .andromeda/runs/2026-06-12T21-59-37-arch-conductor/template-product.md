## Quiz I Fields

### 1. Scale Intent [FIRST]
Options:
- **personal** — single developer, runs locally next to one Pulse instance on the dev host; no auth, no multi-tenancy, no cloud infra. Matches the brief's "solo, local" target user and "local dev host only" non-goal.
- **startup** — small team sharing the harness; would add config/result sharing and light CI integration, but pulls toward infrastructure the brief explicitly excludes.
- **production** — many users / HA / compliance / monitoring; directly contradicts the stated scope (verification companion app, not an end-user product) and the "no cloud, no multi-target" guard.

### 2. Platform
Options refined from product type (desktop control panel over a headless core):
- **Desktop app (Tauri 2 control panel) over a headless-drivable core** — the brief's stated shape: thin GUI shell (scenario picker, start/stop, live counters, run report, operator-pause prompts) over a headless engine driven by `scripts/agent-run.sh`. Matches the dominant "headless core + thin presentation shell" verification-harness pattern in Research I.
- **CLI-only headless core (no GUI)** — drops the Tauri surface; aligns with the single-binary CLI norm of telemetrygen/otelgen, but loses the operator-checklist/pause UX the brief needs for visual (drive+observe) claims.
- **Hybrid (headless core + both CLI and desktop UI as equal front-ends)** — both surfaces are first-class; more parity work to keep the UI a true thin shell over the same commands.

### 3. Primary Language Preference
Options relevant to this domain from Research I:
- **Rust 2024 + tokio** — Pulse-consistent (conventions reuse), mature `opentelemetry-rust` + `opentelemetry-otlp` (`grpc-tonic`) path to OTLP/4317, and the language of choice for the DST projects Research I cites. Trade-off: more emission plumbing is hand-rolled vs Go's batteries-included generators.
- **Go** — lingua franca of CNCF observability; telemetrygen/otelgen/Collector are all Go, so the richest synthetic-OTLP prior art is reusable directly. Trade-off: breaks Pulse stack-consistency and the Tauri-2 control-panel intent; the future Andromeda memory-parser would meet a different stack than it meets in Pulse.

### 4. Target Users
Options specific to this product:
- **The Pulse developer (solo, local)** — the brief's named user: runs Conductor next to a real Pulse on the dev host (roadmap Phase 4 dynamic validation, Phase 5 perf workload source).
- **A small Pulse team / future contributors** — same harness, more hands; would justify shared scenario configs and result history but expands beyond the stated solo scope.
- **CI / agent automation (headless, non-human)** — the `agent-run` path as a first-class "user"; reinforces the agent-driven, headless-drivable design but de-emphasizes the operator-checklist UX.

### 5. Growth Model
Options relevant to this product type:
- **Modular monolith** — one deployable harness with clear module seams (timeline engine · emission primitives · fault helpers · verification/MCP read-back · run report · control panel). Fits a single-binary tool with strong internal boundaries; matches the brief's component decomposition.
- **Monolith** — one codebase, minimal internal boundaries; fastest to build but blurs the engine/verification/UI split the brief leans on (esp. the "UI is a thin shell over the same commands" contract).
- **Plugin/extension system** — scenarios/emission patterns as pluggable units. Tempting for a scenario catalog, but directly fights the "NO scenario DSL; new behavior = new P-XXX first" and anti-feature-creep scope law.

### 6. Development Style
Options (always classic / agent-driven):
- **agent-driven** — structured agent pipeline with end-to-end verification harness; Andromeda generates `scripts/agent-run.*`, structured logs + status surface for agent polling. The brief explicitly names "Development Style: agent-driven (headless-drivable core + `scripts/agent-run.sh`; the UI is a thin shell over the same commands)." Cascades to `tests` and `obs` specialist branches downstream.
- **classic** — manual coding/testing, plans-and-docs only, no harness scaffolding. Contradicts the brief's stated agent-driven intent.

### 7. Core Functionality
**[inferred]** A scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO — programmatically via Pulse's MCP read-back surface where one exists, and via an operator checklist for visual claims.

### 8. Timing-Tolerance Model (deterministic stream shape vs wall-clock SLO assertions)
Research I flags this as the project's central tension: the timeline is reproducible under a seed, but every assertion is a wall-clock SLO ("candidate ≤2s after 30s persistence"), and DST's "abstract physical time" trick is unavailable because Pulse runs in real time as a black box.
- **Fixed per-SLO tolerance bands** — each expected-outcome block carries an explicit slack window (e.g. ±N%/±Nms) layered on the spec SLO; simplest to reason about, but bands are hand-tuned per claim.
- **Emission-journal-relative assertion** — assert timing against the recorded wall-clock journal (actual send times) rather than intended schedule, absorbing emission jitter; closest to the brief's "journal = left side of every SLO check," but still needs a tolerance on the read-back side.
- **Hardware-profile-aware budgets** — scale SLO windows by a measured host profile (the spec's per-tier `<5s/<20s/<90s` is already hardware-profile-aware); most faithful to P-060, but adds a calibration/profiling step before runs.

### 9. Read-Back Channel Dependency Posture (MCP preconditions + what's auto vs operator)
Research I calls the MCP read-back a hard dependency with real preconditions (non-default `mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`), and notes the auto-vs-drive+observe split is gated by what Pulse's MCP actually returns.
- **MCP-required (fail fast)** — refuse to run auto scenarios unless the MCP surface is present and responsive; cleanest assertions, but blocks any verification when MCP is unavailable.
- **MCP-preferred, operator-fallback** — use MCP read-back where present; otherwise downgrade those checks to operator-checklist (drive+observe) items at runtime. Matches the brief's coverage-matrix reality (auto vs drive+observe gated by MCP), at the cost of a variable assertion surface per run.
- **Version-pinned MCP contract** — pin/assert the expected MCP tool-surface version and degrade explicitly on mismatch; guards the "MCP/JSON-RPC surfaces evolve fast / governance moved to Linux Foundation" risk Research I raises, but adds compatibility-tracking overhead.

### 10. Probabilistic-Assertion Policy (deterministic hard-fail vs model-interpretive calibration-region)
Research I confirms a model-backed (LLM) detector can't use exact-match assertions, and that *where* each P-ID's boundary sits and *how wide* the calibration region is are themselves spec decisions — the industry direction is calibration-region + human-in-the-loop.
- **Two-state split (hard / calibration-region)** — exactly the brief's policy: deterministic claims hard pass/fail; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting) checked against a calibration region and reported for human, never hard-failed on exact values.
- **Three-state (hard / soft-warn / report-only)** — adds a middle "warn but don't fail" tier for borderline interpretive cases; finer signal, but more classification judgment per P-ID and a more complex run-report taxonomy.

### 11. Run-History & Artifact Persistence (emission journal + run report storage)
Research I notes harnesses of this kind are file-based/embedded, with JSON/JSONL/Markdown the norm and SQLite the usual ceiling; the brief already specifies a Markdown coverage matrix and run report plus a per-run emission journal.
- **File-only (Markdown report + JSONL journal, per run on disk)** — matches both the brief's stated artifacts and the prevailing file-based norm; zero DB dependency, but cross-run querying/trend analysis is manual.
- **File artifacts + embedded SQLite index** — keep on-disk artifacts as ground truth, add an embedded store for structured run history/comparison across seeds; enables seed-to-seed exploration (the DST rigor Research I highlights) at the cost of a small storage layer the niche usually avoids.

## Pre-filled Values
- **Scale Intent: personal** [inferred from: "Target user: The Pulse developer (solo, local)... local dev host only"; non-goals "NO cloud... local dev host only"]
- **Platform: Desktop app (Tauri 2 control panel) over a headless-drivable core** [inferred from: "Product type: Desktop app (Tauri 2 control panel) over a headless-drivable core"; "Control panel (minimal UI)"]
- **Primary Language Preference: Rust 2024 + tokio** [inferred from: "Constraints / stack intent: Rust 2024 + tokio; Tauri 2 for the control panel — Pulse-consistent"]
- **Target Users: The Pulse developer (solo, local)** [inferred from: "Target user: The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host"]
- **Development Style: agent-driven** [inferred from: "Development Style: agent-driven (headless-drivable core + `scripts/agent-run.sh`; the UI is a thin shell over the same commands)"]
- **Core Functionality** [inferred] — see field 7 above (from "What this is" + "Core idea" in the Extracted section).
- **Growth Model: (not pre-filled)** — the brief decomposes Conductor into clear modules (engine / primitives / fault helpers / verification / UI) suggesting modular-monolith, but does not name a growth model explicitly; left to confirm.

## Quiz I Scope Note
Quiz I collects PRODUCT-level decisions: what, for whom, how it grows, and how we develop it (classic vs agent-driven).
Technical decisions are collected later by:
- Quiz II (architectural forks: framework, database, deployment, API style, module boundaries, validation library, error handling).
- Specialist skills (tests / obs / security / design / a11y — their respective domains). In particular, the *web frontend framework / CSS / component library inside the Tauri webview* (design), the *assertion/test-strategy mechanics and harness driver selection* (tests), and the *structured-log / status-surface format for agent polling* (obs) are explicitly out of scope here.
