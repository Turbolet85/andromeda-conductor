# tests extract

## Relevance
Partial — the chunk surfaces existing tested types (`RunRecord`, operator-checklist from `conductor-core`/`conductor-report`) via new desktop-webview views, but E2E wiring and GUI-parity tests are deferred to Epoch 9; no test infrastructure or harness changes introduced.

## Constraints
1. Per §4 (Unit Test Strategy, conductor-tauri/ui bullet, amended 2026-06-15): Frontend React/Tailwind builds are **build-gated only** (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke) — NO Rust/nextest unit tests. GUI is convenience-only.
2. Per §1 (Scope Summary, conductor-tauri entity): Desktop-webview is **partially-testable** and **secondary to the CLI release gate**; the webview leg is a secondary surface, not the test gate.
3. Per §6 (E2E Test Strategy, desktop-webview surface): Desktop surfaces are tested via **tauri-driver (headless, `wdio run`)** using **text/role selectors on text-paired status labels** (color never sole signal, Design System mandate).
4. Per §3 (Harness Contract, Status endpoint shape): Any backend read-only command must return an existing `conductor-core`/`conductor-report` type with `#[derive(Serialize)]` — no new serialization burden or envelope shape change.
5. Per amendment 2026-06-26: **GUI-parity + Tauri-integration tests deferred to Epoch-9 tauri-driver harness chunk**; run logic coverage is satisfied at unit (conductor-run stream+persist) + CLI E2E (Critical Path 7, byte-identical Blocked envelope post-extraction).
6. Per §2 (Test Strategy / Agent-runnable invariants): E2E surfaces must produce **machine-parseable output** (no Percy/visual-regression; no human color-judgment); text labels and role attributes must survive `NO_COLOR` piping.

## Patterns to follow
1. Per §4 + 2026-06-15: React/Tailwind tests follow the **build-gated pattern** (TypeScript + `vite build` + `npm audit`), mirroring ch5/ch6 posture. E2E wiring is sequenced separately.
2. Per scope + ch6 precedent: **Read-only `#[tauri::command]` returns existing type** — thin data-sourcing only, zero business logic. Backend mirrors `coverage-matrix-view` ch6 pattern (invoke `T[]`, never re-author the shape).
3. Per scope + §4 conductor-report bullet: **`RunRecord`→`Lamp` projection via `lampForRecord`** reuses ch6 logic (mirrors `conductor-core::Lamp::for_record`); no new verdict-rendering algorithm.
4. Per §2: **Run-report envelope shape is golden-locked at unit level** (conductor-report exact-string `assert_eq!` per canonical-serialization pattern); frontend mirrors via type projection, not independent tests.

## Anti-patterns to avoid
1. Per §4 + 2026-06-15: **Do NOT author Rust/nextest unit tests for the React/Tailwind bundle** — build gates only.
2. Per 2026-06-26: **Do NOT introduce Tauri-integration frame-sequence assertions or GUI-parity tests in this chunk** — those are deferred to Epoch-9 tauri-driver harness (run logic parity is proved at CLI E2E tier, §2 agent-runnable invariants).
3. **Do NOT re-author `RunRecord` serialization or verdict-enum logic in the frontend** — reuse the `RunRecord` Serialize projection + `lampForRecord` from ch6.
4. Per Design System (§6): **Do NOT signal status via color alone** — every verdict lamp pairs glyph + text label (StatusLamp contract from ch6 reuse).

## Contract bindings
- **tests ↔ obs (§3 Log format):** Run-report envelope (`run_id/seed/scenario/verdict/state/latency_ms/slo_tier/fingerprints`) is pinned per Test Harness §3 Status endpoint shape; frontend mirrors via `RunRecord` type projection (no schema drift). Self-obs stream (conductor-tauri.jsonl) is separate artifact (amended 2026-06-24).
- **tests ↔ design-system (§2 + §6):** Status labels must be color-never-sole (text + glyph via `StatusLamp`), survive `NO_COLOR`, assert via tauri-driver text/role selectors.
- **tests ↔ route (sequencing):** Tauri-integration + GUI-parity tests deferred to Epoch-9 tauri-driver harness chunk (amendment 2026-06-26). This chunk introduces NO new test mechanisms.

## Acceptance criteria contributions
1. "(tests) Frontend build gates (`tsc --noEmit` + `vite build` + `npm audit`) pass for the run-report + operator-checklist view components."
2. "(tests) The read-only Tauri command (if added) returns an existing `conductor-core`/`conductor-report` type via `#[derive(Serialize)]` (no new serialization surface)."
3. "(tests) Desktop-webview E2E (tauri-driver) assertions for run-report verdict lines and operator-checklist items are deferred to Epoch-9 tauri-driver harness chunk (tracked under §6 E2E Test Strategy)."
4. "(tests) Every verdict/state label in the run-report view pairs text + glyph via `StatusLamp` contract (from ch6 reuse); color is never the sole signal."

## Relevant amendment history
1. **2026-06-15-design-token-typography-bundle** (§4 Unit Test Strategy, conductor-tauri/ui bullet): Frontend React/Tailwind bundle is **build-gated only** (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview`), NOT Rust/nextest unit tests. Frontend E2E coverage (tauri-driver) deferred to Epoch 9. Reason: GUI is "convenience-only" per Design System; no JS/TS unit runner adopted (playbook §12 Decisions Log).
2. **2026-06-26-live-counter-channel-stream** (§5 Integration Test Strategy): **Tauri-integration + GUI-parity tests deferred to Epoch-9 tauri-driver harness chunk**. Run logic is covered at unit tier (conductor-run stream+persist+abort) and CLI parity E2E (Critical Path 7, byte-identical Blocked envelope post-extraction). Reason: GUI Channel async assertion is thread-timing-dependent, would flake against zero-retry bar. New playbook rule appended to pre-empt re-fire on ch5/ch8.
