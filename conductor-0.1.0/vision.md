# Conductor 0.1.0 — Vision

_First-run version intent, distilled from `.andromeda/input.md` (creator brief) +
`.andromeda/architecture.md` §Project Intent. Anchors Phase 0 calibration + Phase 1 synthesis._

## Problem

Pulse v0.2.0 claims 60 formal capabilities (P-001..P-060). The **static** layer is covered
(nextest / ui / a11y tests, mapped by Pulse's verification matrix). The **dynamic** layer does
not exist: nothing can drive a real, running Pulse through its claimed behaviors — establish
baselines, ramp error rates 3.5×, storm a fingerprint 12× in 30s, go silent for exactly 25s —
and check that the right incident surfaces within the right SLO. The Pulse capability spec
reserves a companion app for exactly this (Conductor), writes a per-capability verification
clause, and the roadmap (Phase 4) requires it before Pulse v0.3.0 performance work.

## Who

The **Pulse developer** — solo, local. Runs Conductor next to a real Pulse instance on the dev
host (no cloud, no multi-tenancy). Agent-driven: the headless `scripts/agent-run.sh` path is the
source of truth and release gate; the GUI is a thin convenience shell over the same core.

## Core value

A **scenario-driven OTLP fault-injection + verification harness** that emits precisely shaped
telemetry (traces / metrics / logs) on a **deterministic seeded timeline**, induces the fault
patterns the spec enumerates, and verifies Pulse's reaction — **programmatically via MCP read-back**
where a channel exists, via an **operator checklist** where the reaction is visual. Determinism is
a property the architecture enforces (`current_thread` tokio runtime, seeded RNG); the on-disk
JSONL emission journal is the agent-parseable ground truth, the left side of every SLO check.

## Non-goals (0.1.0 scope guard)

- **NOT a load-tester** — bounded typical/high profiles only; 50k+ spans/s saturation stays in Pulse's own suite.
- **NO Pulse process management** — never starts/stops/restarts Pulse or edits its config; those are operator-pause steps.
- **NO UI automation of Pulse** — visual claims are operator-checklist items (Playwright/axe live in Pulse).
- **NO scenario DSL** — declarative config files + the built-in catalog; new behavior requires a new P-XXX first.
- **NO multi-target / distributed / cloud** — local dev host only; no inbound network listener of Conductor's own.
- **Scope law:** "no scenario without a P-ID."

## What "0.1.0 done" means

- `coverage-matrix.md` complete: all 60 P-IDs classified **auto / drive+observe / static-only**, zero gaps.
- Every catalog scenario runs from the control panel **and** headless; deterministic under a fixed seed; emission journal written per run.
- Run report distinguishes **Pass / Fail / ManualCheck / KnownResidual / Blocked**; manual checklist renders with induced-state context; P-032 reports as KnownResidual (not a surprise failure).
- End-to-end proof against a real Pulse: at minimum `error-baseline-spike`, `fingerprint-storm`,
  `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce
  verified expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle`
  pass observes auto-resolve + resolution summary.
