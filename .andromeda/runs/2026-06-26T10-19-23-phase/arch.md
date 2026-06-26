# arch extract

## Relevance
Relevant — touches workspace structure (conductor-tauri primary + conductor-core/new conductor-run + conductor-cli), run execution wiring, and compiler-enforced module seams.

## Constraints
1. Workspace boundary rules: crate-per-seam Cargo workspace; forbidden cross-seam deps won't compile (arch §Compiler-enforced module seams).
2. Runtime flavor pinned: tokio `current_thread` for determinism; core-owned `Builder::new_current_thread()` isolated from Tauri's multi_thread shell (arch §Established Decisions [Async Runtime Flavor]).
3. Headless-drivable core: pipeline extraction must keep engine logic in runtime-agnostic library (conductor-core or new conductor-run) usable outside Tauri; CLI path stays green (arch §Design Philosophy headless-drivable + §Established Decisions [Workspace / Core Structure]).
4. Verdict/error wall: outcomes are values, `Result::Err` reserved for harness faults; trait abstraction (PauseResolver) for both bins' resolver without embedding pause logic in verdicts (arch §Established Decisions).
5. IPC method pinned: Tauri 2 `Channel` for in-app live updates only — no per-message JSON overhead, no SSE/WebSocket, no OS toasts (arch §Real-time Strategy).
6. No network/HTTP service: only OTLP/gRPC egress to `127.0.0.1:4317` + MCP read-back + Tauri IPC (arch §Established Decisions [Backend Framework]).
7. RunRecord persistence unchanged: JSONL journal + runs.db + run-report envelope (blocked state on preflight fail) per standard contract (arch §Standard Contracts run-report envelope).

## Patterns to follow
1. Deterministic seeded phase scheduler: core-owned `current_thread` + `ChaCha8Rng` seed guarantee "same scenario+seed ⇒ same stream shape" across both bins (arch §Determinism under a seed + §Established Decisions [Determinism RNG]).
2. Thin-shell pattern: both `scripts/agent-run.sh` (headless) and Tauri commands call the same core library identically; CLI is release gate, GUI is convenience (arch §Headless-drivable core, thin shells).
3. Trait abstraction for cross-bin concerns: generalize `&CliResolver` → trait object so CLI + Tauri each supply their own pause-resolution without duplicating logic (arch §Conventions error handling).
4. Crate-per-seam principle: if a new `conductor-run` crate lands, it becomes a workspace member with explicit `Cargo.toml` edges; if absorbed into `conductor-core`, no new crate boundary is created (arch §Compiler-enforced module seams + §Inherited Defaults).

## Anti-patterns to avoid
1. No runtime cross-contamination: do not spawn multi_thread subtasks in core-owned `current_thread` runtime; Tauri's shell multi_thread stays the GUI concern (arch §Established Decisions [Async Runtime Flavor]).
2. No blocking on the event loop: operator-pause logic routes through trait abstraction; CLI may block on `inquire` prompt, Tauri never blocks (resolves via headless default) (arch §Cross-cutting Patterns development-style agent-driven).
3. No HTTP/REST exposure: Channel is IPC-only, no inbound listener, no REST endpoints (arch §Established Decisions [Backend Framework]).

## Contract bindings
- **Workspace crate naming (pipeline library home)** ↔ ALL domains (everyone inherits the chosen library's public API).
- **PauseResolver trait abstraction** ↔ security (input validation/prompt logic isolation) + tests (test-mode resolver impl).
- **Tauri Channel payload + runs.db persistence** ↔ design (titlebar heartbeat rendering, live counter readout) + obs (self-obs JSON via logs/conductor-tauri.jsonl, no raw input logging per capture-fidelity ceiling).
- **Headless CLI parity after extraction** ↔ tests (test-plan Path 7 CLI↔Tauri verification, nextest).

## Acceptance criteria contributions
1. "(arch) Pipeline library lives in conductor-core or new conductor-run per crate-per-seam workspace boundary; no new forbidden cross-seam deps (arch §Compiler-enforced module seams + §Inherited Defaults)." — P4 decision pending.
2. "(arch) PauseResolver trait object generalizes &CliResolver; CLI CliResolver impl + Tauri never-block impl, no pause logic drift (arch §Conventions error handling)."
3. "(arch) Core-owned tokio::Builder::new_current_thread() runtime isolated from Tauri's multi_thread shell; seeded RNG + current_thread preserve 'same scenario+seed ⇒ same stream shape' (arch §Established Decisions [Async Runtime Flavor] + [Determinism RNG])."
4. "(arch) Tauri Channel streams live counters (spans/logs/exceptions emitted) + target status (OTLP-egress/preflight), per §Real-time Strategy; no per-message JSON overhead, no OS toasts."
5. "(arch) RunRecord → Blocked (preflight gate fail) → runs.db + JSONL journal persistence unchanged from CLI path (arch §Standard Contracts run-report envelope)."
6. "(arch) Headless conductor-cli behavior byte-identical post-extraction; test-plan Path 7 CLI↔Tauri parity gate (arch §Design Philosophy headless-drivable core)."

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold — MSRV 1.94.1**: All seams including conductor-tauri inherit security-plan ≥1.94.1; workspace pins `rust-version = "1.94.1"` (per §Stack / §Infrastructure / §Inherited Defaults).
- **2026-06-15-design-token-typography-bundle — frontend stack registered**: React 19.x + Vite 8.0.16 + Tailwind v4.1 + npm; titlebar component (ch2-shipped, used by this chunk's DEV-cycler removal) + ch3 start/stop wiring depend on this stack (per §Stack / §Occupied Resources / §Inherited Defaults).
- **2026-06-24-frameless-window-shell — Tauri build-system coupling**: logs/conductor-tauri.jsonl artifact (GUI self-obs sibling) + @tauri-apps/api (IPC client) + tauri-build generate_context! resolves ui/dist at COMPILE time — webview bundle must build before cargo compile of conductor-tauri (ensure_frontend step into agent-run.sh + CI) (per §Stack / §Occupied Resources / §Infrastructure Patterns).
- **2026-06-24-sanitized-stderr-agent-mode-logging — agent-mode + logs artifact**: CONDUCTOR_AGENT_MODE env var (read-only, flag || env-set) + logs/agent-latest.jsonl artifact; CLI↔Tauri parity requires both bins preserve agent-mode filtering + emit to logs/ sibling (per §Occupied Resources).