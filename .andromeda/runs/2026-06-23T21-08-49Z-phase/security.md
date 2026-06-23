# security extract

## Relevance
Partial — the chunk's terminal UI (isatty gate, inquire confirm, paused-count display) crosses the error-handling/logging boundary; the interactive resolver itself has zero crypto/auth/network scope.

## Constraints
- At the `inquire` confirm rendering edge, the (redacted) `prompt` text MUST NOT leak absolute host paths, internal struct names, or sensitive run context — it surfaces to the operator's terminal per §Error Handling §Boundary error-sanitization (security-plan §Error Handling; §Input Validation non-path string labels clarification from 2026-06-15-structured-logging-stack amendment).
- The paused-count mirror line obeys "status never color-alone": `[HOLD]` ASCII prefix MUST precede any color overlay; non-TTY and `NO_COLOR`-gated color must not strip the prefix per §Error Handling + a11y binding (security-plan §Error Handling).
- `inquire` crate enters `Cargo.lock` and MUST pass `cargo-audit` ≥0.22.1 and `cargo deny check` green (including any transitive advisories) before merge per §Dependency Security Audit tool + §Bootstrap phases dep-audit-tooling-install (security-plan; 2026-06-23-line-oriented-output-rendering amendment applies to transitive advisory ignores in deny.toml if needed).
- The resolver does NOT introduce a second hold/decision model — it implements the **one** `PauseResolver` trait the core defined (scope-law per security-plan §Anti-Patterns Universal).
- Any hold-await emit MUST NOT stamp from the virtual clock — use `std::time::SystemTime`/`Instant` per §Logging anti-pattern (security-plan §Security Anti-Patterns § Logging).

## Patterns to follow
- Reuse the tty-gate primitive (`std::io::IsTerminal` on stdin/stdout) already established in ch3's `stdout_color()` — do not invent a second tty-detection method (security-plan §Established Decisions: Development Style, determinism discipline).
- Non-TTY path delegates to `HeadlessResolver::proceed()` immediately — "headless never blocks" is the source-of-truth agent-path discipline (scope-plan ch4/5 integration point).
- Render-seam error surfaces use `anyhow` at the CLI edge; internal `PauseResolver` impl uses typed errors per §Error Handling (thiserror / seam-crate enums, not anyhow internally).

## Anti-patterns to avoid
- NEVER render the raw `HoldPoint::prompt` unredacted to the terminal — pass it through the sanitization wall (scope.md acceptance intent: "(redacted)").
- NEVER hardcode a color or text format as the sole hold indicator — the `[HOLD]` ASCII prefix MUST be present unconditionally (security-plan §Anti-Patterns: Logging).
- NEVER spawn a shell or evaluate operator input in the resolver — the resolver reads a predefined hold state from the core, not user config (security-plan §Anti-Patterns: Code Patterns § spawn/eval ban).

## Contract bindings
- **obs ↔ hold-pause emit:** the hold await may emit a bounded `tracing` event (run_id-tagged, using SystemTime not virtual clock); redaction edge enforces the prompt is not leaked raw (scope.md boundaries / security-plan §Error Handling).
- **tests ↔ resolver:** the non-interactive path (headless) is testable without a TTY wait; interactive tests must inject a stub resolver or use the headless path (security-plan §Anti-Patterns: Logging "never silently downgrade").
- **design ↔ terminal color:** the `[HOLD]` color palette (amber per design-system.md) is the optional tty-gated overlay on the fixed ASCII prefix (design / a11y binding, scope.md).

## Acceptance criteria contributions
- (security) `inquire` crate passes `cargo audit` ≥0.22.1 + `cargo deny check` (no unresolved advisories or banned crates); `Cargo.lock` committed un-drifted.
- (security) All `inquire`-rendered prompts redact absolute paths and internal struct names; only `scenario` / `p_id` / `step` / sanitized `prompt` text are visible.
- (security) Paused-count `[HOLD]` line includes the ASCII prefix unconditionally; color is additive, never the sole hold encoder.
- (security) Hold-await emit (if any) uses `std::time::SystemTime`/`Instant`, never tokio virtual clock; no raw hold context leaked to structured log.

## Relevant amendment history
**2026-06-23-line-oriented-output-rendering** (this chunk's rendering deps era) — `inquire` + owo-colors + indicatif + comfy-table entered audit/deny; deny.toml gained an advisory-ignore for transitive `number_prefix` (via `indicatif`) and a license-allow for `Zlib` (`foldhash` transitive). This chunk must ensure `inquire` itself stays green in the same audit run. Precedent: 2026-06-15-structured-logging-stack clarified that obs identity env-handles (`CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV`) are non-path labels, not validation boundaries — the same discipline applies to the redacted `prompt` text passed to `inquire`.
