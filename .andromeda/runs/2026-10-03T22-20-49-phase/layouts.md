# layouts extract

## Relevance
partial. The chunk adds and changes no desktop-webview element, and it adds no cli verb, column or bracket label. Its only layout touch points are the cli preflight readiness line's tool count (the fifth pinned tool), the existing missing-tool `[BLOCKED]` rendering, and the `[PRECONDITION]` refusal that `--live` prints before the round. Both legs are a gated cargo test (`p075_round_live`), not a rendered surface.

## Constraints
- The cli header's MCP preflight readiness line carries the protocol version, the TOOL COUNT and the canary result, behind an `[OK]`/`[BLOCKED]` prefix. Both cli wireframes show the sample `tools 4/4`. Pinning `retrieve_incident_events` as a fifth required tool must move that count to 5 everywhere it renders. Whether the count comes from `READBACK_TOOLS` / `contracts/mcp-contract.toml` or is a literal in code or a doc sample is for research to answer (per layout-templates §Component — Header / banner; §Output structure — `conductor run <scenario>`; §Output structure — `conductor suite`).
- A preflight that lacks the new tool renders through the EXISTING missing-tool precondition. That means a `[BLOCKED]` row or line in the count-blocked mapping, with the named precondition string in its detail, its measurement columns rendering `—` / null, and never red. No new label or state is added for it (per layout-templates §Component — Primary content block 1; §Component — Primary content block 2).
- The per-P-ID verdict/report-state label set is CLOSED at six, and the run-level non-lamp qualifier set is `[ENVIRONMENT-SUSPECT]` + `[PRECONDITION]`. A seventh graded assertion is a new CHECK, never a seventh lamp, a new bracket label or a new qualifier (per layout-templates §Component — Primary content block 2).
- The 6-column results/SLO table is a stable, agent-parsed output. Adding a column without a `--format` flag is a breaking change, so per-assertion grading cannot widen it. Any per-check grain goes in the Markdown report's indented per-check detail line: no new column, no new label, no new ANSI or token entry (per layout-templates §IA notes [cli]; §Component — Primary content block 2, "Per-check detail region").
- `scripts/agent-run.{sh,ps1} run --live` leads with the non-priming `conductor preconditions` probe. An unresolved `andromeda-pulse-mcp` on the round's `PATH` refuses the suite at exit 1 with one host-path-free `[PRECONDITION]` line per unmet subject. That is a refusal, not a skip, and no leg fires (per layout-templates §Primary screens (commands), `conductor preconditions` and `scripts/agent-run.{sh,ps1}` bullets).
- Headless invariant: the agent-driven path never blocks on an `inquire` prompt, and output is pipe-clean (ANSI stripped when piped, `NO_COLOR` honored, ASCII prefixes only in piped output). The hands-off round must not introduce an interactive gate (per layout-templates §IA notes [cli]; §Component — Primary navigation (verb structure)).
- Error output goes to stderr, sanitized: no absolute host paths, no internal struct names, no stack traces (`error:` + `hint:`). This applies to any new refusal or fault text the round or the pin adds, including the Linux `PATH` that points at S2's `target/release` (per layout-templates §Component — Footer / terminator + error output).

## Patterns to follow
- The indented-detail-line precedent (the `Blocked` precondition string, the `KnownResidual` note, the per-check detail line) is where finer-grain grading text goes beneath a P-ID verdict (per layout-templates §Component — Primary content block 2).
- The `[PRECONDITION]` caption has two arms: one line per unmet subject naming its statement and candidate causes, or the single satisfied line. A reader keying on the label must not treat it as failure-only (per layout-templates §Primary screens (commands), `conductor preconditions`).
- Status is paired with an ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`, `✓`/`✗`/`→`/`•`) in every line that carries a verdict, so any round summary the leg prints reads without color (per layout-templates §Surface: cli, the paragraph after Signature placement).
- Counts in captions are derived, never literals (the coverage roll-up is "every number manifest-derived"). The readiness line's tool count follows the same discipline (per layout-templates §Primary screens (commands), `conductor coverage`).

## Anti-patterns to avoid
- Minting a new bracket label, lamp, qualifier or results-table column for assertion 7 or for the new tool (per layout-templates §Component — Primary content block 2; §IA notes [cli]).
- Rendering a missing-tool `Blocked` as red or as an error, or with measured values in its columns (per layout-templates §Component — Primary content block 1).
- A host path (the Pulse checkout's `target/release`, a data dir) in a `[PRECONDITION]` line, an `error:` line or the run report (per layout-templates §Component — Footer / terminator + error output; §Primary screens (commands), `scripts/agent-run.{sh,ps1}`).

## Contract bindings
- The readiness-line tool count binds to architecture §Standard Contracts (the MCP read-back contract and preflight gate) and to `READBACK_TOOLS` / `contracts/mcp-contract.toml`. The rendered count and the pinned set must agree.
- The missing-tool `[BLOCKED]` detail string binds to the security rules' five named preconditions, each with its own host-path-free string. The pin keeps the count at five and adds no sixth.
- Status never color-alone (`[BLOCKED]`/`[PRECONDITION]` prefixes, `NO_COLOR`) binds to a11y-plan §3 and the design-system ANSI mappings (count-blocked ↔ ANSI 60, Residual-mute ↔ ANSI 246).
- The `--live` preconditions refusal binds to test-plan §3 (the 5-command harness, operator-gated live-suite stage).

## Acceptance criteria contributions
- With `retrieve_incident_events` pinned, any rendered preflight readiness line reports the tool count derived from the pinned set (5 when every tool is present). No surface, doc sample or test fixture still asserts `tools 4/4` as the required total (per layout-templates §Component — Header / banner).
- A preflight against a sidecar lacking `retrieve_incident_events` renders the existing missing-tool `[BLOCKED]` with its named precondition, and its measurement fields render `—` / null. No new label appears, and the per-P-ID label set stays at six (per layout-templates §Component — Primary content block 2).
- The cli results/SLO table keeps exactly 6 columns, and the cli verb set is unchanged by this chunk (per layout-templates §Component — Primary content block 1; §IA notes [cli]).
- A `--live` invocation whose `PATH` does not resolve `andromeda-pulse-mcp` exits 1 with a host-path-free `[PRECONDITION]` line naming that subject, and no leg fires (per layout-templates §Primary screens (commands), `scripts/agent-run.{sh,ps1}`).
