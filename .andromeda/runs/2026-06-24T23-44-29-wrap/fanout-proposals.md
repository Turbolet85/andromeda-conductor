# P2 fan-out proposals — 2026-06-24-paused-count-hold-point-signature

7 Explore doc-detectors, one per spec source. Parsed proposals (raw returns in the transcript).

## arch — `proposals: []`
D-arch-resources clean (no new IPC/port/socket/endpoint/env-var/crate; the token is a design-system concern, correctly deferred). D-arch-decisions clean (CSS-only motion reaffirmed; no new library; `import.meta.env.DEV` is a Vite built-in already in §Stack).

## security-plan — `proposals: []`
D-security-input clean (no external-input surface — `RunState`/props are internal TS; the DEV cycler is `import.meta.env.DEV`-gated scaffolding). D-security-subprocess clean (sidecar/data-dir untouched). D-security-deps clean (no new dep; package-lock + Cargo.lock undrifted).

## design-system — 1 proposal (APPLIED, routine)
- detector: D-design-tokens · severity: warning
- section: §Motion (calibrated to expression level 0.3)
- change: add a Motion-tokens table registering `--motion-micro` (150ms) / `--motion-heartbeat` (1600ms) / `--ease-quiet`.
- rationale: report Changes/Schema adds `--motion-heartbeat: 1600ms` to tokens.css; §Motion had no named-token inventory → the new token was unregistered.
- **Validation:** routine — the token was user-approved at /andromeda-phase P5 (report §Decisions D2); intent-consistent (the approved scope-amendment); no contradiction. Applied (body table + design-summary cascade + sidecar). No escalation.

## layout-templates — `proposals: []`
D-layout-surface clean — the titlebar count region's default/hold/abort behavior + the aria-live announcement are ALREADY in §Component-Header; this chunk adds run-state BEHAVIOR to the existing region, no new surface. (Tokens deferred to design-system per layout §convention.)

## test-plan — `proposals: []`
D-tests-coverage clean (ui/ is build-gated per §4 — tsc+vite; no Rust unit tier mandated; webview E2E deferred to closing Epoch-9). D-tests-framework clean (nextest runner matches §2/§4). D-tests-obs-harness clean (harness/envelope/JSONL untouched).

## obs-plan — `proposals: []`
D-obs-instrumentation clean (UI render, no telemetry / no `#[tauri::command]` / no must-trace op; frontend is console-only per §3). D-obs-stack clean (no OTel SDK/exporter). D-obs-redaction clean (no logging/artifact path; no host-path/struct leak).

## a11y-plan — `proposals: []`
D-a11y-surface clean — aria-live="assertive" HOLD flip + not-color-alone label + reduced-motion drop + abort-motionless are ALREADY covered by a11y-plan §4/§5/§6/§11; the implementation matched the spec. D-a11y-obs-schema clean (no schema change).

---
**Totals:** 1 amendment (design-system §Motion, routine) · 0 escalations · 6 docs clean. Drift = 0 after apply.
