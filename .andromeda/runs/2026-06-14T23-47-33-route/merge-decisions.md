# Merge Decisions — conductor-0.1.0 route (Phase 3)

_Format: {validator} {type} · {action} · {reason / adjustment / tradeoff}_
_23 suggestions judged. Net: 6 chunks added (60 → 66). Cross-validator conflicts resolved inline._

## Security
- security Insert (error-boundary sanitization) · adjusted · merged with obs redaction reorder into one Foundation chunk "Log + error-boundary redaction" (anyhow-edge + processor-stage scrub); avoids duplicating the existing redaction layer.
- security Insert (dependency version pins) · rejected · duplicates existing coverage — toolchain ≥1.94.1 already in "Cargo workspace scaffold", tauri ≥2.10.3 in "Frameless window shell", libsqlite3-sys float in "Dependency-audit gate".
- security Insert (secret-scanning CI gate) · rejected · security-plan marks it explicitly optional with tool deferred to setup-project and Conductor owns no secrets; firm CI control is the dep-audit gate. (Surface in Phase 4 as an opt-in.)
- security Reorder (redaction before envelope serializer) · applied · satisfied by the stronger obs move of redaction to Foundation (earlier than requested).

## Design
- design Insert (component primitives library) · applied (adjusted placement) · added before "Coverage-matrix view" (first lamp-consuming view) rather than after the picker; lamps/dialog/checklist precede composite views per design-system §Component Patterns.
- design Reorder (paused-count signature before picker) · rejected · already in the requested position — "Paused-count hold-point signature" already sits immediately after "Frameless window shell" and before the picker.

## Tests
- tests Insert (test framework + coverage tooling) · applied (adjusted) · added to Foundation as "Test framework + fixtures + coverage tooling"; rstest folded in (absorbs the test-data-bootstrap install).
- tests Insert (test data bootstrap) · rejected · rstest install folded into the framework chunk; TempDir/seeded-generation fixture wiring is per-seam and coupled to the timeline generator (Epoch 2), not a standalone Foundation chunk.
- tests Insert (CI test-job config) · adjusted · merged with the quality-gate suggestion into one Polish "CI quality-gate config" chunk; Epoch 6 (persistence) was the wrong epoch for CI policy.
- tests Insert (quality gate config) · applied (adjusted) · placed in Polish as "CI quality-gate config" (coverage threshold + flakiness + JUnit/llvm-cov artifacts); audit/deny enforcement left with Foundation's dep-audit gate.
- tests Reorder (completeness gate → Epoch 6) · rejected · the completeness gate is the ship-time definition-of-done and needs all 60 scenarios (Epoch 7); it stays in Polish. The "Coverage-matrix generator" already provides the early artifact in Epoch 6.
- tests Rewrite (determinism-replay harness) · applied · names the determinism machinery (insta golden + proptest + tokio start_paused); arrow rephrased to "yields" to keep ↓ the only arrow.
- tests Rewrite (5-command harness) · applied · names concrete command semantics (boot=preflight, status=disk-read, …) per test-plan §3; retained .sh + .ps1 marker.

## Obs
- obs Insert (panic-capture hook) · adjusted · folded into the "Structured logging stack" chunk (std::panic::set_hook capture) — cohesive self-observation init, avoids a one-line Foundation chunk.
- obs Insert (obs CI conformance gate) · applied · added Polish chunk "Obs CI conformance gate" (log upload + schema conformance + zero-unlogged-panics check) per obs-plan §9.
- obs Reorder (artifact redaction → Foundation) · applied · redaction is a tracing-subscriber processor-stage layer; moved to Foundation as the merged "Log + error-boundary redaction" (wins the topological tie with security's Epoch-6 placement).
- obs Rewrite (structured logging stack — service identity as fields) · applied · folded into the logging-stack rewrite (per-line JSON fields, no OTel SDK) alongside the panic hook.

## A11y
- a11y Insert (a11y tooling install) · adjusted · merged with the focus/ARIA-binding and contrast-harness inserts into one Epoch-9 "Desktop a11y harness setup" chunk.
- a11y Insert (focus + ARIA library binding) · adjusted · merged into "Desktop a11y harness setup"; shadcn/Radix is in-stack reuse (no install), so it does not warrant its own chunk.
- a11y Insert (contrast-verification harness) · adjusted · merged into "Desktop a11y harness setup" (colorjs.io token-pair contrast).
- a11y Insert (screen-reader test-spec) · adjusted · folded into the "Desktop a11y verification" chunk (NVDA/VoiceOver manual spec) — supplemental, not a standalone phase.
- a11y Insert (a11y CI gate + violation JSON) · applied · added Polish chunk "A11y CI gate + violation JSON" (axe/contrast/keyboard into obs envelope, service-tagged).
- a11y Rewrite (desktop a11y assertion harness) · applied · rewrote the existing harness chunk to "Desktop a11y verification" naming the four must-be-accessible paths it asserts.

## Conflict resolutions
- Redaction placement (security Epoch-6-early vs obs Foundation): obs wins on topological grounds (processor-stage layer governs all emissions from the start); single Foundation chunk satisfies both + security's error-sanitization insert.
- CI-gate convergence in Polish: kept domain-distinct chunks (tests quality-gate / obs conformance / a11y violation-JSON) rather than one mega-chunk; audit/deny enforcement stays with Foundation's dep-audit gate to avoid implying audit runs only at ship.

## Deferred to user (Phase 4)
- None hard-deferred. Note to raise: secret-scanning CI gate was rejected as optional/no-tool — user may opt in.
