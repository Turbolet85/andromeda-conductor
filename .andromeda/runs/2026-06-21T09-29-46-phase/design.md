# design extract

## Relevance
partial — the chunk builds a preflight-readiness gate (infrastructure layer, no UI surface) but binds to design validation through report-state serialization and the `Blocked` precondition semantics

## Constraints
- Per design-system §Brand Identity, `Blocked` is a distinct `ReportState` reserved for "never measured" preconditions; the preflight gate's 3 failed assertions (protocol mismatch / tool absence / canary failure) each map to `Blocked` with a named precondition string (design-system §Color Palette, semantic colors table)
- Per §Color Palette §Semantic Colors, `Blocked` renders as slate-violet `#565F89` with a hollow-ring glyph + text label, never a silent downgrade to `Fail` — the preflight integrity invariant forbids collapsing failed readiness into a passed run
- The `ReadyState` struct must serialize for downstream report envelope (design-system §Surface: desktop-webview §Component Patterns #6, run-report view) and CLI output (§Surface: cli §Component Patterns #4, verdict/report-state lines); color-as-state binding applies
- Per §Motion, the preflight check is synchronous (no animation); if serialized for UI display, the blocked state renders in-place, motionless
- Per §Anti-Patterns §Universal Bans, state color is always paired with text label or icon, never color alone — `Blocked` precondition strings are mandatory on output

## Patterns to follow
- The `Blocked` precondition strings (protocol mismatch / tool absence / canary failure) inherit from design-system §Brand Identity: terse status callouts with no alarm, operational reporting (e.g., "protocol version mismatch: expected `2024-11-05`, got `<peer_version>`")
- Readiness assertions use the verdict/error wall discipline (transport/MCP errors → typed `Blocked`/harness `Err`, never panic) per §Anti-Patterns §Per-Surface Bans §desktop-webview (no unstyled errors; design integrity at the boundary)
- The contract manifest (`contracts/` file) is a single source of truth, analogous to design-system's design tokens as an immutable reference (pinned protocol `2024-11-05` + tool list mirrored in report)

## Anti-patterns to avoid
- NEVER use color alone for the `Blocked` state in report output — pair with text (`Blocked` label + precondition string)
- NEVER silence or downgrade a failed preflight to a `Pass`/`Fail` verdict — the `ReportState::Blocked` must propagate unmolested to the run-report view
- NEVER use generic error panics; all transport/MCP errors must surface as named `Blocked` preconditions (design-system §Brand Identity: calm under load, no alarm)

## Contract bindings
**readiness ↔ report serialization:** the `ReadyState` struct serializes into the run-report envelope (downstream chunk 6 `runs.db` wiring); the three `Blocked` precondition strings appear in the report-state lines (desktop-webview §Component Patterns #6 run-report, cli §Component Patterns #4 verdict lines) with the slate-violet `#565F89` color (readiness state is never hidden, always present in the final report artifact)

**readiness ↔ tests harness:** the preflight gate must be drivable over an injected transport for tests + the CI `ready:true` leg; the stub-child MCP-server binary exercises the gate end-to-end, validating the contract manifest binding and the canary round-trip

## Acceptance criteria contributions
- (design) The `ReadyState` struct serializes with all three assertion results (`negotiated_protocol_version`, `required_tools` map, `canary_round_trip`) legible in the run-report
- (design) Each preflight failure maps to `ReportState::Blocked` with a distinct, terse precondition string (protocol/tool/canary) — never color-only signal
- (design) The CLI output for `Blocked` state pairs the slate-violet ANSI color (117 cyan for the tool name / version; 60 violet for the `Blocked` label) with the `[BLOCKED]` text prefix + precondition string (design-system §Surface: cli §Component Patterns #4, §Tokens platform-specific, NO_COLOR safe)

## Relevant amendment history
(none) — no prior amendments to design-system.md touch the preflight readiness / `Blocked` precondition semantics; the amendment on 2026-06-15 (token CSS declaration) does not affect this chunk's serialization contracts
