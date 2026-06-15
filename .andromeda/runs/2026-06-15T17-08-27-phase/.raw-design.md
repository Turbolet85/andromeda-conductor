# design extract

## Relevance
partial — structured logging is non-visual backend infrastructure; design domain contributes observability/diagnostic token constraints only.

## Constraints
- (design-system §Motion) All logging output must respect `prefers-reduced-motion: reduce` context if any UI consumes logs; no animation libraries in logging stack itself (design-system §0.3 expression level).
- (design-system §Anti-Patterns) NEVER use emoji or decorative color in machine-parseable output — ASCII-only prefixes + color paired with text labels only, `NO_COLOR` respected (design-system §cli §Anti-Patterns).
- (design-system §Brand Identity) Service-identity fields must ground logs to "Conductor the harness" domain (mission-control console, not generic telemetry); preserve brand personality through structured field naming (avoid generic "event"/"message" only — use operational context like `verdict`, `phase_line`, `hold_state`).
- (design-system §Color Palette / §cli) If CLI mirrors logs to stderr, use only the ANSI tokens defined (green ANSI 114 `#7EE787`, amber ANSI 179 `#E3B341`, red ANSI 203 `#F85149`, blocked ANSI 60 `#565F89`, ID-cyan ANSI 117 `#7DCFFF`, never custom color codes).
- (design-system §Surface: cli) Logs piped to stdout must strip ANSI when piped (via `anstream`); human-readable logs on stderr only; respect `TERM=dumb` + `NO_COLOR`.

## Patterns to follow
- (design-system §Verdict / report-state lamp) Per-run logs carry verdict/report-state context (not color-alone; pair every status code with text: `[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]` / `[MANUAL]` / `[RESIDUAL]`).
- (design-system §Typography / cli) Status codes and P-ID references in logs rendered as monospace data tier (no special styling needed in JSON, but `run_id` / P-ID stamps carry domain meaning per mono-tier semantics).

## Anti-patterns to avoid
- NEVER emit OTLP or pull OTel SDK into this chunk (scope: self-obs JSON via `tracing-subscriber`, no export protocol).
- NEVER use generic telemetry event shape; anchor field naming to Conductor's verdict/hold/phase vocabulary (reject statistical-average `event.level` / `event.message` unless explicitly bridging to standard log formats).
- NEVER omit `run_id` on log lines (the obs invariant mandates "every line carrying `run_id`" — cross-cutting correlation key).

## Contract bindings
obs-plan §3 / §6 (log JSON schema + service-identity fields) ↔ logging stack (this chunk realizes the schema); zero-unlogged-panics invariant ↔ panic hook capture; cli surface (design-system §cli) ↔ stderr formatting (if logs surface to UI, must respect ANSI token set + `NO_COLOR`).

## Acceptance criteria contributions
- (obs) Panic hook captures every unlogged panic into the structured JSON log stream, satisfying zero-unlogged-panics invariant.
- (obs) Every log line carries `run_id` field so all self-obs for a run shares the cross-cutting correlation key.
- (obs) JSON output respects `NO_COLOR`, `TERM=dumb`, piped stdout ANSI stripping (via `anstream` if logs appear in CLI output).
- (obs) Service-identity fields (`service.name` / version) stamp Conductor harness attribution on every line per obs-plan §3.

## Relevant amendment history
(none) — amendment history file missing (expected on fresh project; treated as empty).