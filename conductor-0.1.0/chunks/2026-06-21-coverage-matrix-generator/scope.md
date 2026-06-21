# Scope — Coverage-matrix generator

**Marker:** `2026-06-21-coverage-matrix-generator`
**Version:** conductor-0.1.0 · **Epoch 6 (Run report & persistence) — chunk 4 of 4**
**Working-route intent:** _Coverage-matrix generator — all 60 P-IDs classified auto/drive+observe/static-only_

## What it builds

A generator that produces **`coverage-matrix.md`** — Conductor's definition-of-done artifact: every Pulse
capability **P-001..P-060 enumerated with zero gaps**, each classified into exactly one **Conductor
verification mode**:

- **auto** — Conductor drives AND asserts (emission journal + MCP read-back + journal-relative timing).
- **drive+observe** — Conductor induces the state; the operator confirms the visual/UX claim via the
  generated checklist (halo hue/breathing, widget invariants, dropdown order, no-notification).
- **static-only** — no dynamic telemetry dimension (e.g. P-038 clipboard, P-040 MCP-independence, P-046
  export UI, P-049 encryption-at-rest, P-051 storage UI, P-054 four-host matrix); these stay with Pulse's
  own test matrix — Conductor explicitly does NOT duplicate them (but provides the workload where one is needed).

The deliverable is **the generator + the encoded 60-row classification + the rendered artifact**, not the
scenario implementations (those are Epoch 7) and not live verification.

## Classification source of truth

- `.andromeda/input.md` §Coverage classification — defines the three Conductor modes + names the static-only
  set; states "a P-XXX missing from the matrix is a defect" and "0.1.0 done = matrix complete, zero unclassified".
- **Seed:** the Pulse audit verdict tables in `.andromeda/refs/` — `pulse-v0_2_0-capability-audit-2026-06-12.md`
  and the structured `capability-verification-matrix.json` (all 60 ids, titles, categories, Pulse-side
  verification_mode + per-P notes). **Note the lens shift:** that JSON carries *Pulse's* modes
  (`automated-nextest` / `by-construction` / …) — the generator re-maps each P-ID to *Conductor's* mode
  (the JSON's "Dynamic-verification (Conductor)" notes on P-025/P-027/P-037/P-045 mark the timing claims
  Conductor owns; the visual half of those is drive+observe).

## Boundaries

- **In:** the classification data model (P-ID → Conductor mode, with title/category/short rationale), the
  Markdown render, the artifact write, completeness enforcement (all 60, zero gaps).
- **Out:** scenario catalog (Epoch 7), live per-scenario verification, the desktop coverage-matrix *view*
  (Epoch 9), the CLI coverage table render (Epoch 8) — those consume this classification, they don't define it.
- **Completeness is the bar:** every P-001..P-060 present and classified; a missing/unclassified id is a defect.
- **Likely placement:** `conductor-core` for the classification model (the 60-row table + mode enum) and
  `conductor-report` for the Markdown render + artifact write (mirrors the `RunReport::render` pure/clock-free
  seam → exact-string golden testable). Final crate split resolved in planning.
- Artifact path: project-root `coverage-matrix.md` per the architecture directory structure
  (CONDUCTOR_RUNS_DIR / artifact-dir overridability resolved in planning).

## Surfaces / contracts touched

- The 60 P-ID coverage classification (input.md §Coverage classification + refs audit seed).
- `conductor-core::Lamp::for_record` reuse (carried follow-up: coverage-matrix / cli / desktop must never
  re-derive lamp precedence) — see open question on whether Epoch-6 matrix renders a live-status column.
- The new `coverage-matrix.md` on-disk artifact (kebab-case Markdown, like the run report).

## Open questions — RESOLVED in planning (P4)

1. **Status column vs classification-only → CLASSIFY-ONLY** (user-confirmed, P4 AskUserQuestion). The Epoch-6
   matrix renders the static classification (mode per P-ID) only — no `runs.db` read, no `Lamp` usage (there is
   no per-P-ID run index and no scenarios until Epoch 7). `Lamp::for_record` reuse (follow-up (c); `lamp.rs:9`
   doc) is the Epoch-8 CLI table + Epoch-9 desktop-view status-overlay seam, documented but not wired now.
2. **Crate split → model in `conductor-core`, render in `conductor-report`** (mirrors `RunRecord`→`RunReport`).
3. **Mode enum → new `conductor-core::CoverageMode` { Auto, DriveObserve, StaticOnly }**, distinct from
   `ClaimClass` (hard/calibration) — a separate axis.
4. **Write semantics → atomic deterministic OVERWRITE** (`.tmp`→rename), NOT the run-report's `create_new`:
   `coverage-matrix.md` is a regenerated project-root singleton (definition-of-done), so regeneration must
   succeed. (Surfaced in planning — the borrowed "never-overwrite" assumption was wrong for a singleton.)
5. **Classification is code-native** (committed Rust `const` table, the source of truth), seeded at authoring
   time from `refs/capability-verification-matrix.json` + input.md — NOT read from disk at runtime (no new
   input boundary; keeps the render pure).

## Definition of done (chunk-level)

- All 60 P-IDs (P-001..P-060) emitted, each classified into exactly one of {auto, drive+observe, static-only},
  zero unclassified/missing — completeness asserted by test.
- Pure/clock-free render → exact-string golden test; artifact write is loud-never-overwrite-safe consistent
  with the run-report seam.
- Lamp precedence reused (not re-derived) wherever a verdict/report-state status is shown.
- Gates: `cargo nextest` workspace green · clippy `-D warnings` clean · doctest 0 · no new dependency unless justified.
