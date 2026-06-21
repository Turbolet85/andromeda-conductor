# design extract

## Relevance — partial

This chunk renders human-facing Markdown artifacts. The report surface touches typography (prose + mono), color status encoding (Verdict/ReportState prefixes as ASCII, no color-alone), spacing (table/list density), and artifact hygiene (no host-paths / struct names leaked). CLI typographic tier and motion rules apply by proxy (the same status-color semantics); desktop-webview-specific components (frameless titlebar, dialogs) are out of scope.

## Constraints

- per §Brand Identity: Verdict/ReportState color encodes Verdict (`Pass` → green, `Fail` → red, `CalibrationRegion` → amber hold, `Blocked` → slate-violet, `ManualCheck` → neutral lavender, `KnownResidual` → muted dashed) — use ASCII prefix over color (Markdown has no color; prefix IS the status encoding).
- per §Color Palette: reserve mono ID-cyan `#7DCFFF` for the status-tier data (P-IDs, run_id, SLO timings, fingerprints); secondary text `#A9B1D6` for body prose and metadata; blocked slate-violet `#565F89` for the Blocked state lamp prefix (`[BLOCKED]`), never red.
- per §Typography: IBM Plex Sans body (14px / 1.5) for prose + descriptions; JetBrains Mono (13px / 1.55, tabular-nums, `#7DCFFF` ID-cyan) for P-IDs, run_id, SLO timings (`<5s`/`<20s`/`<90s`), latency_ms, fingerprints, and code blocks.
- per §Component Patterns §6 (Run-report view): each P-ID line carries verdict-first lamp prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`) + text label (never color alone; Color-Only rule §1.4.1).
- per §Anti-Patterns §Universal Bans: never use color purely for decoration; every prefix communicates state, never omit the ASCII encoding where MD has no color tier.
- per §Anti-Patterns §Universal Bans: never conflate "no result yet" with "failed" — a blocked row is a distinct slate-violet state carrying the named precondition string, never a silent downgrade to red Fail.

## Patterns to follow

- Verdict-first lamp precedence (scope §): use `verdict` when present (Some(Pass|Fail|CalibrationRegion) → [PASS]/[FAIL]/[HOLD]) before falling back to `state` (Blocked → [BLOCKED]); apply this shared helper across all surfaces (Markdown, coverage-matrix, cli, desktop) so the status reads consistently.
- Blocked-row null rule (scope §): a blocked row shows identity + slo_tier only; measurement fields (latency_ms, journal_emitted_at, read_back_observed_at, fingerprints) render as em-dash or absent marker — never the literal `null` or struct-name leak.
- Status-never-color-alone invariant (§): ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]` is the status encoding in Markdown (no color available); pair every status with text label, never omit.
- Run-level summary section (scope §): includes run_id (mono), seed, per-state counts, generated-at timestamp; uses mono ID-cyan for the run_id stem and SLO tiers in the body.

## Anti-patterns to avoid

- NEVER leak absolute host paths, internal struct names, or stack traces into the `.md` artifact (artifact hygiene — CLAUDE.md universal invariant; sanitize at render edge).
- NEVER render a blocked row with the five measurement fields as literal `null` or a Rust struct-debug name; use em-dash or render-absent semantics.
- NEVER use color alone to encode status in Markdown (color is absent); the ASCII prefix IS the status encoding — omitting the prefix creates a silent failure (Color-Only a11y rule).

## Contract bindings

- **Status-never-color-alone** binds to a11y §Use of Color SC 1.4.1 (color alone insufficient; text prefix required even though MD has no color rendering, this preserves the invariant downstream when the report is rendered/quoted in other surfaces).
- **Verdict-first lamp precedence** (shared helper) carries from §Brand Identity signature element (the verdict lamp is a supporting convention) and cascades to coverage-matrix (Epoch 6 §ch4), cli (Epoch 8), and desktop (Epoch 9) so the status always reads the same.
- **Artifact hygiene** binds to security-review §Artifact Leakage (no host-paths / struct names) and CLAUDE.md universal invariant (all artifacts stay clean).

## Acceptance criteria contributions

- (design) Verdict-first lamp precedence applied: `Some(Pass|Fail|CalibrationRegion)` renders [PASS]/[FAIL]/[HOLD]; `None` falls back to state (Blocked → [BLOCKED]); CalibrationRegion row shows [HOLD], never [MANUAL].
- (design) Blocked row renders identity + slo_tier; measurement fields render as em-dash, never `null` or struct name.
- (design) ASCII prefix [PASS]/[FAIL]/[HOLD]/[BLOCKED]/[MANUAL]/[RESIDUAL] paired with status text; no color-alone encoding (status-never-color-alone invariant, even in Markdown).
- (design) No absolute host path, struct name, or stack trace leaked into `runs/<run_id>.md` (artifact hygiene).

## Relevant amendment history

(none) — No amendments to design-system.md touch the run-report render seam; the token-bundle amendment (2026-06-15, §Tokens, `:root` not `@theme`) is webview-only and does not affect Markdown generation.