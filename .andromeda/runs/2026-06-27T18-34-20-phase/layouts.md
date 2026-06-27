# layouts extract

## Relevance
partial — chunk extends existing CLI surfaces with live Pulse verification, does not create new layout structures

## Constraints
1. Preflight readiness gate must complete canary bridge round-trip (emit→ingest→read-back→fidelity assertion) before flipping to `ready: true` (per §cli Primary screens preflight command + §IA notes readiness gate policy)
2. Canary failure modes must render as distinct `[BLOCKED]` with named precondition string, never silent pass (per §cli Primary content block 2 Blocked-row format + scope "distinct `Blocked` with named precondition")
3. Five-family live MCP-verified E2E verdict states must render verdict-first (Pass/Fail/ManualCheck/KnownResidual/Blocked) in the existing verdict/report-state lines with paired ASCII prefix (per §cli Primary content block 2 verdict-line format + §IA notes "signal never color-alone")
4. All verdict/report output must survive NO_COLOR / piping / headless `isatty` policy (per §cli IA notes "ANSI auto-stripped when piped; headless path never blocked on prompt")
5. Preflight readiness line must accommodate canary result within the existing "protocol {version} tools {count}/{count} canary {result}" format (per §cli §Output structure — `conductor run`)
6. Blocked verdict rows for live MCP failures must render precondition string below the verdict line in dimmed ANSI 146 (per §cli Primary content block 2 example "precondition: mcp-server feature + …")

## Patterns to follow
1. Verdict lamp + paired ASCII prefix — every verdict cell pairs color with bracket prefix (`[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]`) (layout-templates §cli Primary content block 2)
2. Blocked precondition naming — e.g. "canary-emission tool missing" / "corpus visibility timeout" / "content-fidelity mismatch" — distinct failure mode per named feature (layout-templates §cli Primary content block 2 Blocked example)
3. Preflight readiness line format — "protocol 2024-11-05 tools 4/4 canary {ok|failed:reason}" after the header (layout-templates §cli §Output structure — `conductor run`)
4. Verdict state never downgraded — Blocked/Manual/Residual are distinct non-verdict states, never red; live MCP read-back failures → Blocked with precondition, never Fail (layout-templates §cli Primary content block 2 + scope "verdict/error wall")

## Anti-patterns to avoid
1. Color-only status — never render canary/verdict state via ANSI code alone; always pair with ASCII prefix / text (layout-templates §cli IA notes "signal never color-alone")
2. Blocked as red error — Blocked preconditions (canary timeout / content mismatch) are hollow-ring / never-measured states, never `status-fail` red (layout-templates §cli Primary content block 2 "Blocked ≠ red — never measured")
3. Hardcoded readiness-line format — keep protocol/tool-count/canary result in the existing one-line format; never wrap or extend into multi-line status (layout-templates §cli §Output structure stability for agent parsing)

## Contract bindings
- **Verdict/report-state lines** ↔ `conductor-report` (JSONL journal + runs.db + Markdown envelope): renders journal-relative SLO latency, verdict-first lamp per live MCP read-back outcome
- **Preflight readiness gate + canary result** ↔ `conductor-verify` (MCP client, canary round-trip): gate logic determines `ready:true/false` exit code for `agent-run boot`
- **CLI output structure** ↔ `agent-run.sh` 5-command harness + downstream agent parsing: output format (comfy-table columns, verdict-line format, preflight-line format) must remain stable and parseable

## Acceptance criteria contributions
1. Preflight readiness line displays canary round-trip result (layout-templates §cli Primary screens preflight command, §Output structure `conductor run`) — pass iff canary emit→ingest→read-back→fidelity assertion completes before `ready:true`.
2. Live MCP-verified verdict states render in cli verdict lines with paired ASCII prefix (layout-templates §cli Primary content block 2) — pass iff all five families render non-Blocked verdict state with ASCII prefix + color.
3. Blocked preconditions from canary/live MCP failures render with named precondition string below verdict line (layout-templates §cli Primary content block 2 Blocked format) — pass iff each failure mode is distinctly named (e.g. "canary-corpus-visibility-timeout" / "mcp-query-incident-list-timeout" / "canary-content-fidelity-mismatch").
4. Report artifacts (JSONL + `runs.db` + Markdown) persist without host-path / internal struct-name leakage (layout-templates §cli Component — Footer / error output "sanitized per security plan") — pass iff all output paths and identifiers are artifact-relative or unambiguous P-ID/run_id.

## Relevant amendment history
- **2026-06-23-5-command-agent-run-harness** — registered `conductor preflight [--json]` verb as the readiness gate / `agent-run boot` entrypoint (exits 0 iff ready:true); this chunk wires the canary bridge that determines that exit code (scope §Blocking gap, §Core deliverable)
- **2026-06-23-line-oriented-output-rendering** — registered `conductor coverage` verb + corrected `conductor report` output to colored `comfy-table` results view; this chunk's evidence artifacts use the report envelope (scope §Evidence, JSONL + runs.db + Markdown)
