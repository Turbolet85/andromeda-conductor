# security extract

## Relevance — partial
This chunk implements CI quality-gate infrastructure (GitHub Actions YAML + nextest config changes). It introduces new output artifacts (JUnit XML, lcov/HTML coverage reports) that must follow error-sanitization discipline, and it must preserve existing security gates; it does not introduce new runtime threat boundaries.

## Constraints — domain rules that apply
1. **JUnit XML artifact sanitization** — The nextest-emitted JUnit output MUST NOT leak absolute filesystem paths (canonicalized `CONDUCTOR_*` dirs, `ANDROMEDA_PULSE_DATA_DIR`) or internal struct/field names in test assertion messages or metadata; output must be operator-safe per security-plan §Error Handling, artifact sanitization principle.
2. **Coverage report metadata sanitization** — The lcov/HTML coverage artifacts MUST NOT expose absolute paths, internal crate names, or build-directory-relative paths in report headers or filenames; only relative crate identifiers are allowed (security-plan §Error Handling).
3. **Audit gate preservation** — The existing `cargo-audit` + `cargo-deny` CI gates (established per §Dependency Security) MUST remain green and hard-failing; this chunk's CI config changes must not downgrade, suppress, or bypass them — they are the Minimal-tier residual-risk control (security-plan §Threat Model Summary, §Security Anti-Patterns §Universal).
4. **Zero-flakiness determinism enforcement** — The nextest `ci` profile's zero-retry invariant must be asserted as a hard CI failure (not silent retry-on-flake); determinism is the foundation for reproducible golden tests (security-plan §Cross-cutting Patterns Determinism discipline).
5. **No silent artifact suppression** — Justified exceptions (coverage thresholds, flakiness budget values) must be documented inline in CI config + `.config/nextest.toml` with rationale comments; do not introduce suppression-flags or unvetted gating tools (mirrors deny.toml accepted-exceptions pattern per amendment 2026-06-27-desktop-a11y-harness-setup).

## Patterns to follow
1. **Artifact output hygiene** — Gate-step + artifact-upload scaffold (established here) is reused by downstream "Obs CI conformance gate" chunk; cross-confirm artifact consumption format (JUnit → agent-parser, coverage → dashboard) with test-plan §9 to avoid downstream rework.
2. **CI gate numerics as pinned floors** — Coverage threshold and flakiness budget values are plan-time decisions (test-plan §10); pin them in CI YAML with documented floor vs stretch rationale (mirrors cargo-audit/cargo-deny floor-based approach per amendment 2026-06-15-dependency-audit-gate).

## Anti-patterns to avoid
1. **NEVER silently suppress `cargo audit` or `cargo deny` in CI** — A green audit gate is non-negotiable for the Minimal-tier threat model (security-plan §Threat Model Summary: "dependency/supply-chain audit" is the residual-risk control for `bundled` SQLite + OTLP/gRPC/MCP tree). Downgrading/bypassing it contradicts the tier (§Security Anti-Patterns §Universal).
2. **NEVER commit JUnit/coverage artifacts containing absolute paths or internal crate structure** — These are agent-parseable and shared across dev hosts; paths must be relative/normalized at source, not redacted at read-time (security-plan §Error Handling, amendment 2026-06-24-sanitized-stderr-agent-mode-logging).

## Contract bindings
- **tests §CI Integration** — This chunk establishes the artifact-upload + gate-step scaffold that downstream "Obs CI conformance gate" and "A11y CI gate" chunks reuse; coordinate gate ordering + artifact formats (JUnit consumption, coverage artifact naming) with test-plan §9.
- **obs §PII Scrubbing** — Any operator paths in JUnit assertion errors must not reach artifact outputs; sanitize at generation time (nextest config), not at obs read-back.

## Acceptance criteria contributions
1. (security) JUnit XML emitted by nextest `ci` profile contains no absolute filesystem paths; all paths in test-assertion messages are relative or internal symbolic (no `CONDUCTOR_*` / `ANDROMEDA_*` canonicalized directories leaked).
2. (security) Coverage report (lcov/HTML) metadata/headers contain only relative crate identifiers; no absolute paths from build environment.
3. (security) `cargo audit` and `cargo deny` (advisories + bans + sources + licenses) remain hard-fail steps in CI pipeline; this chunk's changes do not downgrade, suppress, or remove them.
4. (security) nextest `ci` profile confirms `retry = 0`; any flake surfaced as hard test failure, not silent retry.

## Relevant amendment history
- **2026-06-15-dependency-audit-gate** — Audit tooling versions (`cargo-audit`, `cargo-deny`) are minimum floors, not strict locks; the RustSec advisory DB is runtime-fetched. This chunk must preserve the existing audit gates (already wired to `.github/workflows/`); tool versions remain floor-based.
- **2026-06-27-desktop-a11y-harness-setup** — npm-audit gate made dev-aware (`--omit=dev` for test tooling; production tree stays clean). Pattern applies to CI gates in this chunk: justified exceptions (coverage thresholds, flakiness budget) documented inline in config; no unvetted suppression tools.
