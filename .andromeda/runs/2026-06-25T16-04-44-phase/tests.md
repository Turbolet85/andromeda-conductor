# tests extract

## Relevance
Partial — the chunk adds UI/command control surfaces over the unchanged core; picker is new (introduces shadcn/Radix deps), commands drive existing paths, no model/schema change.

## Constraints
- Per test-plan §2 (Test Strategy), the 5-command discipline applies: `start_run`/`stop_run` are Tauri IPC commands wired to the existing CLI `run`/cleanup logic (test-plan §3 Integration row: "Tauri command/Channel IPC layer is agent-drivable headless via tauri-driver"); both headless and GUI paths must exit the same way — test-plan §5 Cross-surface parity.
- Per test-plan §4 Unit Test Strategy (conductor-tauri/ui), React/Tailwind frontend code carries no Rust unit tests; gated by `tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke; E2E webview coverage is Epoch 9 ch9/ch10.
- Per test-plan §1 Surfaces (desktop-webview), the picker + run-control layout must be keyboard-operable (Radix roving focus), selectors text-paired (not color-alone per a11y-plan §Visual), tauri-driver headless-runnable.
- Per test-plan §3 Status endpoint shape + test-plan §5 Both-surface parity (Critical path), `start_run(selection)` must produce identical `runs.db` envelope verdict/state/seed to the headless `conductor run` for the same scenario+seed.
- Per test-plan §10 Coverage triggers (supply-chain audit), `npm audit` clean + `package-lock.json` committed on shadcn/Radix deps; `tauri ≥ 2.10.3` dependency-pinned (test-plan §1 Surfaces, security Vector 3 CVE gates).
- Per test-plan §3 (Tauri capabilities), extend `capabilities/*.json` deny-by-default with ONLY `list_scenarios`/`list_suites`/`start_run`/`stop_run` allowlist; never blanket or pre-add later-chunk permissions (no run-report/pause/Channel-stream command perms here).

## Patterns to follow
- Tauri `#[tauri::command]` functions tested via language-native IPC test client (test-plan §1 ipc-internal surface); JSON schema assertion on response shape for list/start/stop payloads (test-plan §5, list/start/stop contract).
- Cross-surface parity: `start_run(P-ID, seed=S)` run producing identical `runs.db` row (verdict/state/seed) as `agent-run.sh run <P-ID> --seed S` (test-plan §5 Critical path 7, Both-surface parity).
- Frontend build-gate contract (test-plan §4 amendment 2026-06-15): `ui/` TypeScript → `tsc --noEmit` (strict, no `any`/`as`) + `vite build` → `ui/dist` + `vite preview` render smoke; lexically validate the picker is roving-keyboard-operable (cmdk/Radix primitives guarantee this; a negative test asserts no `shell-open`/unsafe command construction).

## Anti-patterns to avoid
- Running the `start_run` scenario string through shell construction or passing it via argv without validation against the known catalog (test-plan §1 Surfaces security Vector 3; scenario enum MUST be validated client-side before exec).
- Pre-adding capability permissions for run-report/operator-pause/Channel-stream commands in this chunk (those are ch5/ch6/ch7/ch8; deny-by-default extends ONLY for list/start/stop here).
- Adopting JS/TS unit tests (jest/vitest) for the frontend bundle (test-plan §4 amendment 2026-06-15 — frontend E2E via tauri-driver, not js-native unit; no `jest.setup.js` / vitest config in this chunk).

## Contract bindings
- **Tauri commands ↔ core CLI pipeline** — `start_run(selection)` invokes the same `conductor-timeline` + `emit` + `verify` + `report` seams the headless `conductor run` drives; must share the core-owned `current_thread` runtime + `runs.db` write + JSONL journal write (test-plan §3 5-command run, test-plan §1 ipc-internal Tauri surface, test-plan §5 Path 7 parity).
- **Frontend deps (shadcn/Radix/cmdk) ↔ security supply-chain** — `npm audit` clean + `package-lock.json` committed (test-plan §10 supply-chain trigger; obs-plan §Dependency Security cascades to `ui/`).
- **Tauri capabilities ↔ security deny-by-default** — test-plan §1 Surfaces (security Vector 3) + test-plan §10 trigger; ACL must enumerate ONLY the commands live in this chunk (test-plan scope rule: no pre-add of ch5/ch6/ch7/ch8 command perms).

## Acceptance criteria contributions
- "(tests) Tauri `list_scenarios`/`list_suites` command response validates against the pinned scenarios catalog; each entry includes P-ID(s) + title."
- "(tests) Cross-surface parity: `start_run(P-ID, seed=S)` produces identical `runs.db` envelope (verdict/state/latency_ms/slo_tier) as `conductor run <P-ID> --seed S` for the same scenario."
- "(tests) Frontend build-gate: `ui/tsc --noEmit` (strict) + `vite build` → `ui/dist` + `npm audit` green + `package-lock.json` committed (shadcn/Radix/cmdk deps)."
- "(tests) Tauri capabilities remain deny-by-default; only `list_scenarios`/`list_suites`/`start_run`/`stop_run` allowlist entries added; negative test asserts no `shell-open` / unsafe command construction in start_run scenario handling."
- "(tests) `cargo nextest` workspace + doctest + clippy `-D warnings` + `cargo audit` + `cargo deny check` green; `Cargo.lock` + `package-lock.json` re-committed un-drifted."

## Relevant amendment history
- **2026-06-15 design-token-typography-bundle** (§4, conductor-tauri/ui): React/Tailwind frontend tests build-gated (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke); webview E2E deferred to Epoch 9 ch9/ch10. **Applies here:** no Rust nextest unit tests for the picker UI code; frontend validation happens at build-time (TS strict) + E2E later.
- **2026-06-16 test-framework-fixtures-coverage-tooling** (§4): external-CLI tool versions (`cargo-nextest 0.9.137`, `cargo-llvm-cov 0.8.7`) are reference floors (any green-running install satisfies); crate dev-deps caret-resolved with `Cargo.lock` authoritative. **Applies here:** `ui/package.json` dev-deps (shadcn, Radix, cmdk, vite, typescript, etc.) caret-resolved, `package-lock.json` committed; tool floors reused.
- **2026-06-24 frameless-window-shell** (§3, Log format): Tauri backend self-obs sink `logs/conductor-tauri.jsonl` live; distinct from emission-journal `runs/<run_id>.jsonl`. **Applies here:** `start_run`/`stop_run` commands self-observe via existing `conductor-tauri` `tracing` sink; run carries `run_id`; no new obs policy (processor-stage redaction reused).
