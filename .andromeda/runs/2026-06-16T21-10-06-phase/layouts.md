# layouts extract

## Relevance
partial — the chunk builds a write-side emission-journal writer (non-UI infrastructure), touches no layout surfaces directly

## Constraints
- per layout-templates §Primary Surfaces: the desktop-webview run-report view and cli report output eventually consume the journal's canonical timestamps (latency_ms / read-back stamps derived from `journal_emitted_at`); the writer ensures source-of-truth integrity but surfaces no layout itself
- per layout-templates §IA notes §Multi-surface coordination: the journal schema must support both desktop verdict-line rendering and cli comfy-table population without adapting between surfaces; tokens stay by-name, data delivery stays unified
- per layout-templates §Decisions Log §Notable surface-specific deferrals: the Markdown run report and runs.db SQLite index are deferred (not this chunk's responsibility) — the journal writer is the file artifact precedent, not the rendered display
- per layout-templates §IA notes §Pipe discipline: cli output must preserve journal data on stdout, human messages on stderr; the journal schema must be machine-parseable for headless agent consumption (no emoji, structured JSONL, deterministic field order)
- per layout-templates §IA notes §Headless invariant: wall-clock stamping from `std::time` (not virtual/scheduled time) is the canonical source, ensuring operator-pause holds + asynchronous verdict delivery don't skew SLO math
- per layout-templates §Component — Primary content block 2: per-P-ID verdict lines pair latency_ms (derived from journal timestamps) with slo_tier so state is never color-alone on either surface

## Patterns to follow
- Paused-count signature placement (layout §Signature placement strategy): the journal writer records the frozen count value at the exact operator-pause hold moment, enabling both desktop §Component — Header and cli §Component — Hero to echo the same frozen snapshot at the decision point; the journal entry's timestamp is the hold-point's wall-clock source of truth
- Verdict lamp resolution (layout §Component — Primary content block 1 & 2): each per-P-ID verdict line in the journal must include sufficient data (phase, slo_tier, latency_ms, status state) to render the 6-state lamp unambiguously (Pass / CalibrationRegion / Fail / ManualCheck / KnownResidual / Blocked) on both surfaces without lossy mapping
- Token-by-name coordination (layout §IA notes §Multi-surface coordination): the journal schema uses same names as the design-token mapping (count-nominal, count-hold, status-fail, count-blocked, color-id-cyan) so that css-var desktop rendering and ANSI cli rendering consume identical data from the journal

## Anti-patterns to avoid
- No emoji or non-ASCII prefixes in the journal JSONL lines (layout §IA notes §Pipe discipline: ASCII prefixes `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` only in piped output); the journal writer must emit flat JSON, never pre-formatted status strings
- Do not timestamp from virtual/scheduled time (layout §Constraint §Determinism invariant): wall-clock `std::time` only; tokio's virt-clock and scheduler sleeps must not leak into `journal_emitted_at`
- Do not render human-facing status display in the journal lines themselves (layout §Decisions Log §Key layout choices: "raw artifact data on stdout, human messages on stderr"); the journal is the data tier, not the display tier; render concern belongs downstream (verdict-line formatting lives in the report renderer, not the writer)

## Contract bindings
- tests/obs ↔ journal schema (obs-plan §Log/JSON schema ownership; test-plan §Run-report / journal envelope) — the tests/obs specialist defines the line format and field allowlist; the writer implements per that spec
- verdict-lamp rendering ↔ desktop-webview / cli surfaces (layout §Component — Primary content block 1 & 2 depend on structured journal fields to render the 6-state lamp unambiguously without lossy mapping)

## Acceptance criteria contributions
- (layouts) Journal emitted_at timestamps sourced from `std::time` wall-clock per layout-templates §Determinism invariant; SLO math consumes `journal_emitted_at − read_back_observed_at` (determinism required).
- (layouts) Per-P-ID verdict lines in journal include state / slo_tier / latency_ms fields sufficient to render the 6-state lamp (Pass / CalibrationRegion / Fail / ManualCheck / KnownResidual / Blocked) unambiguously on both desktop-webview and cli surfaces per layout-templates §Component — Primary content block 1 & 2.
- (layouts) Journal JSONL lines are machine-parseable flat JSON, not pre-formatted status display, per layout-templates §IA notes §Pipe discipline (emoji-free, ASCII field names, no internal struct leaks).

## Relevant amendment history
(none) — the layout-templates-amendments file does not exist; this is the first extraction for the layouts domain on this project.