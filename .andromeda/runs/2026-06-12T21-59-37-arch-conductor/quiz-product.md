# Quiz I — Product Layer Results

## Core Fields
- Scale Intent: personal — solo developer, local dev host, no cloud / no multi-tenancy
- Platform: Desktop app (Tauri 2 control panel) over a headless-drivable core — thin GUI shell (scenario/suite picker, start/stop, live emission counters + target status, run report view, operator-pause prompts) over a headless engine driven by `scripts/agent-run.sh`
- Primary Language: Rust 2024 + tokio
- Target Users: The Pulse developer (solo, local) — runs next to a real Pulse instance on the dev host (Pulse roadmap Phase 4: dynamic capability validation; Phase 5: workload source for performance measurement)
- Growth Model: Modular monolith — one deployable harness with hard module seams: timeline engine · emission primitives · fault helpers · verification/MCP read-back · run report · control panel
- Development Style: agent-driven
- Core Functionality: A scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO — programmatically via Pulse's MCP read-back surface where one exists, and via an operator checklist for visual claims.

## Domain-specific Fields
- Timing-Tolerance Model: Hybrid journal-relative deadline + tier-scaled bounded slack, split by assertion class. Latency is always measured as `read_back_observed_at − journal_emitted_at` (journal-relative, never wall-clock-from-test-start). Deterministic/hard SLOs get a tolerance band scaled to the spec's hardware-profile-aware tiers (<5s / <20s / <90s). Sample-count floors (50-sample latency / 10-span error-rate) and model-interpretive timings route to the calibration-region bucket instead of hard-failing. Per-run warm-up jitter calibration (measured-baseline tolerance) is the designated additive upgrade path if per-host flakiness appears — it fits inside the same journal-relative frame without reopening the model.
- Read-Back Channel Dependency Posture: Version-pinned MCP contract + preflight gate, per-scenario explicit degrade. At suite start, run MCP `initialize` and assert the negotiated protocol version plus presence of each required tool (`query_incident_list` · `retrieve_report` with `degraded_mode` · `retrieve_telemetry_slice` · `mark_incident_resolved`) against a pinned contract manifest. On absence/mismatch, every dependent auto scenario reports as **blocked** — a distinct report state, never pass/fail/manual-check — with the named precondition (e.g. `mcp-server` cargo feature + `ANDROMEDA_PULSE_MCP_ENABLED`). Operator-checklist fallback happens only where the catalog already defines a drive+observe observation for that P-ID; auto claims are never silently downgraded.
- Probabilistic-Assertion Policy: Two-state split (hard-fail / calibration-region). Deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) = hard pass/fail. Model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting) = calibration-region checks + report-for-human, never hard-failed on exact values. Near-edge/far-edge annotation inside the calibration region remains available later as report metadata without adding a new state.
- Run-History & Artifact Persistence: File artifacts + embedded SQLite index. Per-run JSONL emission journal + Markdown run report stay on disk as the agent-parseable ground truth; a thin `runs.db` (rusqlite 0.31, bundled SQLite ≥3.38 with JSON1) indexes run_id · seed · scenario · P-IDs · verdict · fingerprints · timestamps for cross-run queries, seed-to-seed comparison, and the P-036 fingerprint-recurrence ("Previously seen" across two sessions) check. Synchronous, append-mostly storage module — no async DB layer.

## Recommendation Adherence
- Timing-Tolerance Model: accepted
- Read-Back Channel Dependency Posture: accepted
- Probabilistic-Assertion Policy: accepted
- Run-History & Artifact Persistence: accepted
All recommendations accepted. (Growth Model — core field — user picked the soft-default Modular monolith.)

## Defaults Applied
None — all fields explicitly chosen.
