# security extract

## Relevance
Partial — the chunk introduces new CLI rendering surfaces (output presentation layer) but does not modify threat boundaries, input validation logic, or secret handling.

## Constraints
1. All new dependencies (owo-colors, indicatif, comfy-table) must pass `cargo audit` / `cargo deny` green before merge (security plan §Dependency Security, CI integration).
2. `Cargo.lock` must remain committed and drift-free; the three new crate versions are deterministic audit inputs (security plan §Dependency Security, Pinning subsection).
3. Color output MUST be gated on tty detection and ALWAYS paired with ASCII status prefixes (`[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]`); color never encodes meaning alone (security plan §Error Handling, sanitized external responses + a11y contract in scope).
4. Run-report artifacts (tables rendered to stdout or files) MUST NOT leak absolute paths (canonicalized `CONDUCTOR_RUNS_DIR`, `ANDROMEDA_PULSE_DATA_DIR`) or internal struct names — mirror the existing Markdown report sanitization (security plan §Error Handling, run-report artifact sanitization).
5. No new `unsafe` code across the render/output seams (security plan §Security Anti-Patterns § Universal, last ban).

## Patterns to follow
1. Type-erased `anyhow` errors at the `conductor-cli` edge; render seams keep internal `thiserror` enums strictly internal (security plan §Error Handling, error format).
2. Piped / non-tty output auto-suppresses color via owo-colors' `if_supports_color` — no explicit agent-mode format logic (that is ch5); the existing output remains unambiguous without escape codes (scope.md boundary ch5).
3. Reuse the existing Markdown report's verdict→lamp state mapping (`Verdict { Pass, Fail, CalibrationRegion }` / `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`); render the same canonical states without inventing a second classification (scope.md acceptance intent).

## Anti-patterns to avoid
1. NEVER expose internal error details (stack traces, file paths, library versions) in CLI output — sanitize at the `anyhow` edge (security plan §Security Anti-Patterns § Logging + API).
2. NEVER interpolate operator-supplied values or paths directly into rendered tables — canonicalized paths and `run_id` are identity fields only; do not leak absolute host paths (security plan §Error Handling, run-report artifact sanitization).

## Contract bindings
Design §palette / layout-templates §CLI Primary screens (terminal-color mapping + per-line layout); a11y §never-color-alone rule (ASCII prefix always present); tests §CI security gate (cargo-audit / cargo-deny integration within existing pipeline).

## Acceptance criteria contributions
1. (security) `cargo deny check` + `cargo audit` pass (owo-colors, indicatif, comfy-table audit green; no new advisories).
2. (security) `Cargo.lock` committed + drift-free; new dependency versions recorded.
3. (security) All CLI output lines render with ASCII status prefix present; color stripped or piped output remains unambiguous.
4. (security) Run-report tables and stdout do not contain absolute filesystem paths or internal struct/field names.

## Relevant amendment history
2026-06-15-dependency-audit-gate — cargo-audit/cargo-deny versions are minimum floors (audit tool versions not lock-pinnable; advisory DB runtime-fetched); toolchain ≥1.94.1 confirmed done (clears tar-rs symlink-chmod CVE-2026-33056). Three new crates in Cargo.lock enter the audit scope.