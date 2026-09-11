# 1A — Hierarchical decomposition (Conductor 0.3.0)

_Phase 1 sub-step 1A. Depth per `complexity-outline.md`: every epoch depth 1 (epoch → chunks
directly), so no sub-block tier appears — 1C drops nothing but the annotations below._

---

### Epoch 1 — Foundation: the measurements the closures rest on (depth 1 → 2 chunks)

- Hosted-runner endpoint cause probed (source: intent §F1.1 `v3-01`; precedent chunk `2026-09-08-hosted-runner-webview2-session`, diagnose-only with zero source delta)
- P-025 measurement contract for Pulse (source: intent §F3 EXPECT unblocked half `v3-07`; obs-plan §10 SLO measurement table — the tier thresholds and the `latency_ms` definition the contract must be stated against)

### Epoch 2 — Scenario assertion hygiene (depth 1 → 3 chunks)

- Structurally-dead assertion class retired (source: intent §F2.1–2 `v3-04`; class precedent `architecture-amendments.md:403`; security-plan §Input Validation governs the scenario-config surface being edited)
- Scenario tier honesty across the corpus (source: intent §F2.3 `v3-05`; obs-plan §10 tier threshold table)
- Scenario-assertion audit gate (source: intent §F2 EXPECT `v3-06`; test-plan §10 Quality Gates for the gate's home)

### Epoch 3 — The a11y capability's terminal (depth 1 → 2 chunks)

- A11y CI gate at an honest terminal (source: intent §F1 EXPECT `v3-02`; a11y-plan §9 CI Integration — the routine arm's gate, job `a11y`)
- Keyboard and focus-order coverage ownership (source: intent §F1.2 `v3-03`; a11y-plan §5 Keyboard Navigation `run-console-idle` focus order + §3 Keyboard test harness)

### Epoch 4 — Live proof against a real Pulse (depth 1 → 4 chunks)

- Real-model leg posture and grading rule (source: intent §F4 EXPECT `v3-09` first half; test-plan §6 E2E driver table — the live read-back leg is an operator-local gate, never CI)
- Interpretation proven live (source: intent §F4 EXPECT `v3-09` second half)
- Diagnostic-quality cluster off the drift pin (source: intent §F4 EXPECT `v3-10`)
- Hue-shift budget graded hard (source: intent §F3 EXPECT + §SEQUENCING `v3-08` — Conductor cannot satisfy this alone; placed last in the epoch, and its clearing event is named in the entry's own words so promotion can raise the `BLOCKED-ON` annotation verbatim)

### Epoch 5 — Polish & ship (depth 1 → 2 chunks)

- Full-gate regression over the moved surfaces (source: test-plan §10 Quality Gates; a11y-plan §9)
- Version close on measured evidence (source: vision §What "0.3.0 done" means; intent §Out-of-scope — release artifacts re-produced only if a 0.3.0 change moved them)

---

## Count check

2 + 3 + 2 + 4 + 2 = **13 chunks**, exactly the Phase 0 estimate (0 % variance, well inside ±20 %).

## Annotation obligations carried to promotion (NOT authored into the route line)

Route authored zero `BLOCKED-ON` annotations in 0.2.0 — the grammar lives at promotion and the annotated
long-form entries sit in `route-archive.md`. Two obligations travel forward rather than into the flat list:

- **Hue-shift budget graded hard** — `BLOCKED-ON: Pulse emitting the contracted observable` (the measurable
  event that clears it, per this project's standing rule that a block names its clearing EVENT and never a
  standing state). The entry text already names that event, so the annotation is a lift, not a re-derivation.
- **A11y CI gate at an honest terminal** — its terminal fork is decided by the Foundation probe's reading;
  promotion must not pre-commit either branch.
