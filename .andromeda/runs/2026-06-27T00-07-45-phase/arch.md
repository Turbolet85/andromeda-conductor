# arch extract

## Relevance
Relevant — a desktop (Tauri) view component consuming conductor-core vocabulary and frontend stack; conditional Tauri command addition possible

## Constraints
1. Per §Stack and Technologies (Desktop frontend), the UI must use React 19 + Vite 8.0.16 + Tailwind v4.1 (Oxide) + npm (§Inherited Defaults Frontend)
2. Per §Occupied Resources (Frontend asset subtree), code lives in `crates/conductor-tauri/ui/src/` (not a Cargo workspace member) with design tokens declared on `:root` via Tailwind v4 `@theme`
3. Per §Conventions (Naming patterns), TypeScript files follow JavaScript camelCase naming, components use React functional + hooks, strict mode enforced (no `any`)
4. Per §Conventions (Error handling + Verdict/error wall), the `Verdict` / `ReportState` / `Lamp` types are values (not error variants) and must be consumed as-is from conductor-core without re-definition
5. Per §Established Decisions ([Real-time Strategy]), live updates use Tauri 2 `Channel` only (if live counter data flows to the view); no SSE/WebSocket/polling
6. If the open question resolves to a live-data read-only command, per §Conventions (Internal core↔UI), the command follows `#[tauri::command]` request/response pattern with no ACL gating (deny-by-default capability posture unchanged per scenario-suite-picker learning)
7. Per §Infrastructure Patterns (Build system), the webview bundle (`ui/dist/`) must be built (`npm run build`) BEFORE any workspace `cargo build`/`nextest`/`clippy` of `conductor-tauri` — `ensure_frontend` step wired into `agent-run.{sh,ps1}` (ch6 bootstrap)

## Patterns to follow
1. Reuse the existing `StatusLamp` component from `crates/conductor-tauri/ui/src/components/StatusLamp.tsx` (landed in ch5 component-primitives); project `RunRecord` → `Lamp` via the verdict-first precedence of `conductor-core::Lamp::for_record` (verdict chosen first, report state as tiebreaker)
2. Mirror `conductor-core/src/lamp.rs` spellings via `ui/src/lamp.ts` `LAMP_META` (the shared vocabulary; never re-spell the glyph/label/token-color palette)
3. Follow the component structure of existing Epoch-9 surfaces (`Titlebar` / `RunControls` / `ScenarioPicker` under `ui/src/`) — co-located CSS per convention, semantic HTML/ARIA (table/list + row semantics), keyboard reachability
4. Use design-token-sourced styling via the committed 34 `:root` tokens + 6 type-role classes in `crates/conductor-tauri/ui/src/styles/tokens.css` (Tailwind v4 `@theme` tree-shakes non-namespace tokens)
5. If the view streams live data from a new Tauri command, use the established `Channel` pattern from ch6 (conductor-run library, live-counter drive registered in amendments 2026-06-26)

## Anti-patterns to avoid
1. Do not re-spell the lamp palette, glyph set, or label text — consume `LAMP_META` from `ui/src/lamp.ts` as the single source of truth, byte-consistent with `conductor-core::Lamp` (§Conventions)
2. Do not render color alone without glyph + label (per §Conventions, never color-alone — the accessibility principle enforced across Epoch 9)
3. Do not introduce new workspace crates or dependencies beyond the locked frontend stack (React/Vite/Tailwind/npm per §Inherited Defaults; exact versions in amendments 2026-06-15)

## Contract bindings
- **Verdict/ReportState/Lamp types** ↔ conductor-core: The view consumes `Verdict` / `ReportState` / `Lamp` enum shapes; lamp vocabulary is read-only (no re-definition per §Standard Contracts run-report envelope §Conventions) and must stay byte-consistent with conductor-core spellings
- **Coverage classification** ↔ conductor-core generator + CLI coverage-table: The per-P-ID auto/drive+observe/static-only split must mirror the classification the existing coverage-matrix generator and CLI coverage table use (single source of truth — shared vocabulary, never re-authored)
- **RunRecord shape** ↔ conductor-report/runs.db: If sourcing live data via a new read-only Tauri command, the command's response shape must match conductor-report's `RunRecord` (seed / verdict / state / latency_ms / fingerprints / etc.) — the §Standard Contracts envelope invariant
- **Build ordering** ↔ conductor-tauri bin + CI harness: The frontend dist (`ui/dist/`) must be built before any Rust `cargo` command compiles `conductor-tauri` — the constraint registered in amendments 2026-06-24 (frameless-window-shell, tauri-build `generate_context!` coupling)

## Acceptance criteria contributions
1. (arch) Coverage-matrix view component lives in `crates/conductor-tauri/ui/src/` per workspace boundary rules and the occupied-resources frontend asset subtree (§Occupied Resources, amended 2026-06-15)
2. (arch) Per-P-ID lamp vocabulary reused byte-for-byte from `conductor-core::Lamp` via `ui/src/lamp.ts` `LAMP_META` without re-spelling or re-defining; verdict-first lamp selection (`CalibrationRegion` row renders HOLD, not Manual) matches `Lamp::for_record` semantics (§Conventions, amended 2026-06-21 run-report-envelope-serializer)
3. (arch) TypeScript strict (no `any`) + design-token-sourced Tailwind styling per §Inherited Defaults Frontend; `npm audit` gate = 0 (parallel to cargo-audit); `package-lock.json` committed (§Inherited Defaults Frontend, amended 2026-06-15)
4. (arch) If a new read-only `#[tauri::command]` is added (pending open-question resolution): conforms to §Conventions request/response pattern; propagates no credentials/secrets (local-only tool per §Cross-cutting trust boundary); command-added workspace build remains green (`cargo build` / `nextest` / `clippy -D` on `conductor-run` + `conductor-core` + `conductor-report` seams if the command reads runs.db)

## Relevant amendment history
1. **2026-06-15-design-token-typography-bundle** (amended §Stack Frontend + §Occupied Resources asset-subtree + §Inherited Defaults): Established the frontend toolchain (React 19 + Vite 8.0.16 + Tailwind v4.1 Oxide + npm) and the `crates/conductor-tauri/ui/` asset structure (not a Cargo member; `node_modules/` + `dist/` git-ignored; design tokens on `:root`; fonts self-hosted WOFF2). This chunk lands in that asset tree per the scope.

2. **2026-06-24-frameless-window-shell** (amended §Occupied Resources artifacts + §Stack Frontend row + §Infrastructure Patterns Build system): Registered `logs/conductor-tauri.jsonl` artifact; documented tauri-build's `generate_context!` coupling requiring webview bundle (`ui/dist/`) built before workspace `cargo` commands — the `ensure_frontend` step wired into `agent-run.{sh,ps1}` (bootstrapped in ch6). The coverage-matrix view, shipping in ch10 (Epoch 9), depends on this build-order invariant already being in place.

3. **2026-06-26-live-counter-channel-stream** (amended §Occupied Resources crate-names + §Infrastructure Patterns directory tree): Registered `conductor-run` library crate (9th workspace member) — the run composition root sitting above the seams, shared by both CLI + Tauri bins. If the coverage-matrix open question resolves to a live-data Tauri command, the command path will interface with conductor-run's `RunRecord` shape and potentially the `Channel` live-counter pattern (already pinned in §Real-time Strategy).