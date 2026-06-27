# arch extract

## Relevance
Relevant — UI surfaces over locked core types in the conductor-tauri frontend stack.

## Constraints
1. Frontend stack: React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide via `@tailwindcss/vite`) per §Stack and Technologies (Desktop frontend row)
2. Crate location: `crates/conductor-tauri/ui/src/components/` per §Occupied Resources (Frontend asset subtree — npm, not Cargo member; `node_modules/`/`dist/` git-ignored; `package-lock.json` committed + `npm audit` gate)
3. Run-report envelope locked: per §Standard Contracts (run_id/seed/scenario/p_ids/verdict/state/journal_emitted_at/read_back_observed_at/latency_ms/slo_tier/fingerprints); Blocked rows populate identity fields only, verdict/latency_ms/fingerprints as JSON null
4. Verdict/ReportState types immutable: `Verdict ∈ {Pass, Fail, CalibrationRegion}` and `ReportState ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}` are §Standard Contracts shapes; ManualCheck applies to operator-checklist (no programmatic read-back) AND auto-measured calibration-region checks per §Probabilistic-Assertion Policy
5. Verdict-first lamp precedence: RunRecord→Lamp via `lampForRecord` (mirrors conductor-core::Lamp::for_record); lamp chosen by verdict first (verdict-first), not state; `Verdict::default_report_state` maps Pass→Pass / Fail→Fail / CalibrationRegion→ManualCheck per §Probabilistic-Assertion Policy
6. Component primitives locked: StatusLamp (verdict lines, label+glyph) + OperatorChecklist (controlled `items` + `onToggle`; native-checkbox induced-state rows) are shipped from component-primitives-library (ch6, §Occupied Resources Frontend asset subtree); DO NOT author new types
7. Status never color-alone: every verdict/state carries text label + glyph per §Conventions (tty-gated color on CLI; color + label+glyph on desktop)
8. If backend command added: thin read-only query returning conductor-core/report type with `#[derive(Serialize)]` per §Standard Contracts Tauri commands pattern; consumed via `invoke<T[]>()` in TS (carried data-sourcing pattern from ch6 coverage-matrix-view)

## Patterns to follow
1. Data-sourcing: if backend query command needed, single-source an existing conductor-core or conductor-report type (never re-author shape in TS); add `#[derive(Serialize)]` to the return struct; invoke via `invoke<T[]>` in TS per the coverage-matrix-view pattern
2. Verdict-first lamp: RunRecord→Lamp projection via `lampForRecord` (defined in ch6's ui/src/lamp.ts, mirrors conductor-core::Lamp::for_record); lamp's text + glyph chosen by verdict field, not state field, per §Probabilistic-Assertion Policy (a calibration row renders HOLD, not Manual)
3. Component reuse: StatusLamp + OperatorChecklist are shipped primitives; reuse directly from component-primitives-library; LAMP_META constants already defined
4. Design-token binding: color via `:root` tokens (Tailwind v4.1 `@theme` tree-shakes non-namespace tokens), self-hosted WOFF2 fonts (no CDN), 34 `:root` tokens + 6 type-role classes per design-token-typography-bundle (ch6, §Stack)

## Anti-patterns to avoid
1. No new verdict/state/lamp types in TS — Verdict/ReportState/Lamp are conductor-core types; lampForRecord projection already defined in ch6; never re-author
2. No new lamp or checklist primitives — StatusLamp + OperatorChecklist ship from component-primitives-library; reuse directly
3. Color-alone status — every verdict/state MUST carry label + glyph; color encodes state only; never rely on color alone per §Conventions
4. No backend seam modification — the chunk is UI-focused; conductor-report and conductor-core types stay immutable; any backend addition is a thin read-only query returning an existing type

## Contract bindings
- **conductor-core ↔ ui**: RunRecord envelope (run_id/seed/scenario/p_ids/verdict/state/...) per §Standard Contracts; Verdict/ReportState/Lamp enums are conductor-core types consumed by the UI
- **conductor-report ↔ ui**: the run-report Markdown shape mirrors the React view; both consume identical RunRecord data (journal_emitted_at/read_back_observed_at/latency_ms/slo_tier/fingerprints columns)
- **conductor-run ↔ ui**: if run-report data sourcing wires a live `runs.db` query now (vs. deferred to Epoch 10), it routes through conductor-run's planned query surface (currently preflight/execute_scenario/persist/drive_run; per 2026-06-26 amendment)
- **design-system ↔ ui**: StatusLamp + color tokens (lamp_code amber/red/green) + typography (6 type-role classes) inherit design-token-typography-bundle (ch6, §Stack and §Occupied Resources)
- **tests ↔ ui**: DEV/Gallery.tsx fixtures exercise sample RunRecord + verdict states; real E2E data comes from Epoch 10 (live Pulse proof)

## Acceptance criteria contributions
1. "(arch) React 19 components live in `crates/conductor-tauri/ui/src/components/` per §Occupied Resources."
2. "(arch) Run-report view surfaces all RunRecord fields per §Standard Contracts envelope (run_id/seed/scenario/p_ids/verdict/state/journal_emitted_at/read_back_observed_at/latency_ms/slo_tier/fingerprints); Blocked rows populate identity fields only, verdict/latency_ms/fingerprints as JSON null."
3. "(arch) Status lines use StatusLamp primitive (never new lamp types); every verdict/state carries label + glyph, never color-alone per §Conventions."
4. "(arch) Operator-checklist view reuses OperatorChecklist primitive (controlled `items` + `onToggle`; native-checkbox induced-state rows) per §Occupied Resources."
5. "(arch) Verdict-first lamp precedence: RunRecord→Lamp via `lampForRecord` (verdict chosen first, state second) per §Probabilistic-Assertion Policy."
6. "(arch) If backend query command added: returns conductor-core/report type with `#[derive(Serialize)]`, consumed via `invoke<T[]>()` in TS per the data-sourcing pattern (ch6 precedent)."

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** (§Stack and Technologies, §Occupied Resources, §Inherited Defaults): Frontend stack locked React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide) + Fontsource WOFF2 + @tauri-apps/api; `crates/conductor-tauri/ui/` asset subtree registered (npm, not Cargo; `package-lock.json` committed + `npm audit` gate). Enables this chunk's frontend surface and design-token bindings.
- **2026-06-21-run-report-envelope-serializer** (§Probabilistic-Assertion Policy, §Read-Back Dependency Posture): ManualCheck widened to include auto-measured calibration-region checks; Verdict→ReportState default mapping (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`); verdict-first lamp precedence (calibration row renders HOLD, not Manual). Directly constrains how run-report view renders verdicts and which state each maps to.
- **2026-06-24-frameless-window-shell** (§Stack, §Occupied Resources, §Infrastructure Patterns): @tauri-apps/api (window/IPC client) added to Desktop frontend stack; `logs/conductor-tauri.jsonl` artifact registered; tauri-build `generate_context!` frontend-before-cargo coupling documented (ensure_frontend in agent-run + CI). Enables Tauri command registration if backend query is needed; locks build-order constraint (npm dist/ built before cargo builds conductor-tauri).
- **2026-06-26-live-counter-channel-stream** (§Occupied Resources, §Infrastructure Patterns): conductor-run crate registered (9th workspace member: preflight + execute_scenario + persist + live-counter drive_run, above seams, below both bins, shared by cli + tauri). Potential future source of RunRecord query surface if run-report view opts to wire live `runs.db` now vs. deferring to Epoch 10 (open scope question in chunk's scope.md).
