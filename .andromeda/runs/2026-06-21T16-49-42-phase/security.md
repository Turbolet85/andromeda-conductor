# security extract

## Relevance
Partial — operator-pause orchestration introduces a new Tauri IPC command + hold-point model traversing IPC/serialization boundaries; core seam (types/logic) stays in `conductor-core` (local, not IPC-exposed).

## Constraints
1. Hold-point model + go/no-go decision types MUST use serde for IPC round-trip; all hold prompts redacted of absolute paths and internal struct names per security-plan.md §Error Handling (run-report artifact sanitization rule).
2. Hold resolution is a typed VALUE (`go` / `no-go` outcome), never a `Result::Err` — the verdict/error wall reserves `Err` for genuine harness faults (security-plan.md §Error Handling, Verdict/error wall discipline).
3. `#[tauri::command]` for pause/resume MUST ship with **deny-by-default capabilities** limiting to the actual pause/resume surface only; the operator-pause command is part of the minimal IPC set (security-plan.md §Security Anti-Patterns § Code Patterns, Tauri capability minimization).
4. Headless resolver MUST NOT block; auto-resolution deterministically returns the defined outcome and is recorded in the run report without wall-clock perturbation to the seeded emission stream (security-plan.md §Threat Model Summary § Attack surface, loopback-client model; scope-law discipline: "no deliberate blockers in the headless path").

## Patterns to follow
1. Resolver abstraction as a trait in `conductor-core` decouples *where* the hold is awaited from *how* it is answered — both CLI and Tauri shells drive the same core (star-topology invariant: seam crates import only `conductor-core`).
2. Tests resolve holds via a stub auto-resolving resolver under `tokio::time` `start_paused`; human pause is outside the virtual clock (per security-plan.md §Threat Model Summary § Attack surface, determinism preservation via wall-clock-only gap).
3. Hold outcomes recorded in run report + `runs.db` carry identity fields (`scenario`, `P-ID`, `step`, `outcome`) WITHOUT absolute paths or seam-internal names (per security-plan.md §Error Handling, run-report artifact sanitization).

## Anti-patterns to avoid
1. NEVER expose internal seam struct names or absolute paths in the hold-point prompt or go/no-go outcome — all text surfaces the Tauri command boundary MUST be sanitized (security-plan.md §Security Anti-Patterns § Logging: "NEVER expose stack traces, absolute file paths, or internal struct/field names").
2. NEVER allow a hold resolution to panic or surface as a `Result::Err` on the read-back path — typed values only; `tonic::Status`/MCP responses treated as verification inputs (security-plan.md §Security Anti-Patterns § Universal: "NEVER let a malformed child/transport input panic").
3. NEVER skip the Tauri capabilities file for the operator-pause command — it is part of the minimal IPC set and MUST be explicitly allowed, not relying on a permissive default (security-plan.md §Security Anti-Patterns § Code Patterns: "NEVER ship Tauri commands without a minimal capabilities file").

## Contract bindings
**Tauri IPC** ↔ security-plan.md §Threat Model Summary § Attack surface (in-process Tauri IPC boundary; no network exposure); **verdict/error wall** ↔ security-plan.md §Error Handling (go/no-go outcomes are typed values in `Ok(...)`, not `Err` variants); **run-report sanitization** ↔ security-plan.md §Error Handling (hold prompts/outcomes recorded without path/internal-name leak); **headless determinism** ↔ tests §CI Integration (hold resolver stub + `start_paused` ensure test reproducibility).

## Acceptance criteria contributions
1. (security) Hold-point model + go/no-go types serde-roundtrip through IPC and run report; all text prompts/outcomes redacted of absolute paths and internal struct names (grep verifies: no `%`, no `/dev/`, no canonical paths in serialized artifact).
2. (security) Pause/resume `#[tauri::command]` present in `capabilities.json` deny-by-default with explicit allow for the two commands only; unauthorized IPC blocked.
3. (security) Headless resolver auto-resolves deterministically; test under `start_paused` confirms zero wall-clock perturbation to emission stream (same scenario + seed ⇒ identical shape + recorded hold outcome).
4. (security) Hold outcome recorded in run report/`runs.db` as typed value (`scenario` · `P-ID` · `step` · `go`/`no-go`); `no-go` never panics or surfaces as `Result::Err`.

## Relevant amendment history
**2026-06-15-structured-logging-stack** — `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs env-handles) noted as non-path string labels requiring no validation, distinct from `CONDUCTOR_*` path handles that canonicalize + bounds-check. Not directly applicable to hold points, but establishes the precedent for distinguishing identity labels (no validation) from path handles (canonicalize-and-bounds-check).