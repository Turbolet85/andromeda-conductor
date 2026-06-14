# Brand Quiz Context — Conductor

Constant context for all Phase 2 brand-quiz research sub-agents (Q1..Q5). Extracted by the orchestrator from `architecture.md` (Project Intent), `security-plan.md` (Threat Model Summary), and `tooling-decisions.md` (Phase 1).

## Project Intent (from architecture.md)

- **product_type:** Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO. The GUI is explicitly **"a control surface, not a dashboard."**
- **audience / target_users:** The Pulse developer — solo, local, technical. A single developer-operator running Conductor beside a real Pulse instance on the dev host. No non-technical users.
- **platforms:** Desktop (Tauri 2 webview) + headless CLI. Surfaces: `desktop-webview`, `cli`. No web, no mobile.
- **scale_intent:** Personal — solo developer, local dev host, no cloud, no multi-tenancy.
- **growth_model:** Modular monolith — one deployable harness with compiler-enforced crate-per-seam boundaries (`conductor-core`/`-timeline`/`-emit`/`-faults`/`-verify`/`-report`/`-cli`/`-tauri`).
- **core_functionality:** Emits precisely-shaped OTLP telemetry (traces/metrics/logs) on a deterministic seeded timeline; induces fault patterns (error-rate ramps at 3.5×, fingerprint storms 12×/30s, exact silence/gap/resume, port-occupier on :4317); verifies Pulse's reaction programmatically via MCP read-back (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`) or via an operator checklist for visual claims. Outcomes are typed verdicts (Pass/Fail/CalibrationRegion) and report states (Pass/Fail/ManualCheck/KnownResidual/Blocked). The control surface: scenario/suite picker, start/stop, live emission counters + target status, run-report view, operator-pause prompts. Ground truth is an on-disk JSONL emission journal + Markdown run report + SQLite `runs.db` index.
- **development_style:** **agent-driven** — the headless `scripts/agent-run.sh` path is the source of truth and the release gate; the GUI is a thin shell over the same core commands.

## Security (from security-plan.md — Threat Model Summary)

- **security_tier:** Minimal (0) — single-developer, local-only, no-cloud tool; no PII/payment/health/credentials owned by Conductor; zero network exposure (loopback gRPC/MCP client, no inbound listener). No compliance triggers. → Brand work is NOT gated by auth/login UX rigor; there is no auth surface to style.
- **auth_approach:** none — no user accounts, no authenticated surface. Forward-guardrail only: never add interactive login to the headless source-of-truth path.

## Tooling (from tooling-decisions.md — Phase 1)

- **family_chosen:** React
- **frontend_framework:** React 19.x (Vite, SPA — not SSR; Preact 10.x size fallback)
- **css_tool:** Tailwind CSS v4.1 (`@tailwindcss/vite`, static offline stylesheet)
- **component_library:** shadcn/ui (Radix UI Primitives 1.x; MIT; copy-paste, locally vendored)
- **mobile_framework:** N/A
- **cli_surface_stack:** clap 4.5 + anstream/anstyle + owo-colors 4 (styling) + indicatif 0.18 (live counters) + comfy-table 7 (SLO tables) + inquire 0.7 (operator-pause prompts) — TTY-gated; second surface, surface-specific tooling.

## Brand calibration notes (from input.md creator brief)

- Domain language to mine: "scenario-driven," "fault injection," "deterministic seeded timeline," "emission journal as ground truth," "verdict/error wall," "calibration region," "known-residual," "blocked precondition," "coverage matrix (P-001..P-060, zero gaps)," "operator checklist," "the conductor" (orchestrating an emission timeline — a musical/orchestral metaphor is latent in the name).
- Tone: an instrument for a precise, technical, single operator — diagnostic rigor, determinism, trustworthy ground-truth artifacts. Not a consumer dashboard, not marketing-glossy. A "control surface" — closer to an audio console / oscilloscope / mission-control panel than a SaaS analytics dashboard.
- Negative anchors: NOT a load-tester, NOT a Pulse process manager, NOT a dashboard. Avoid generic observability-SaaS aesthetics (Datadog/Grafana clone look) unless deliberately chosen.
