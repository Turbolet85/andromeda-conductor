# layouts extract

## Relevance
partial — the chunk adds no surface, region, component, focus stop or modal; its only layout touchpoint is the cli preflight readiness line and the `[BLOCKED]` missing-tool rendering that a fifth `required_tools` pin reaches, plus the cli pipe/headless discipline the operator-gated live leg runs under.

## Constraints
- The cli MCP preflight readiness line carries protocol version, tool count and canary result, with an `[OK]` / `[BLOCKED]` prefix so the gate reads without color (per layout-templates §Surface: cli → Component — Header / banner). Pinning `retrieve_incident_events` as a fifth required tool moves the denominator that line reports; whether the shipped line derives the count from the manifest or from a literal is research's question.
- The `run` and `suite` wireframe samples both print `tools 4/4` (per layout-templates §Surface: cli → Output structure — `conductor run <scenario>` and → Output structure — `conductor suite`). These are illustrative samples, not a rendering contract; the sample becoming stale against a five-name `required_tools` set is a doc-side observation for the wrap's drift pass, never a reason to change rendering.
- A `Blocked` outcome (here: the missing-tool precondition any pre-S Pulse would now trip) renders with the `[BLOCKED]` bracket label, the `count-blocked` mapping, the named precondition string in its detail, and measurement fields as `—` / null, never a red error (per layout-templates §Surface: cli → Component — Primary content block 2 (verdict / report-state lines); §Component — Primary content block 1 (results / SLO table)).
- The per-P-ID lamp set is closed at six and the run-level qualifier set is `[ENVIRONMENT-SUSPECT]` + `[PRECONDITION]`; the round must mint no new bracket label, lamp or qualifier for assertion 7 or for any of the seven grades (per layout-templates §Surface: cli → Component — Primary content block 2).
- The results/SLO table is exactly 6 columns and the cli output structure is stable because downstream agents parse it; adding a column without a `--format` flag is a breaking change (per layout-templates §Surface: cli → Component — Primary content block 1; → IA notes).
- The headless agent-driven path never blocks on a prompt, and the live suite is reached only through `run --live` (a stage flag/selector, never a sixth harness command), leading with the non-priming `conductor preconditions` probe that refuses at exit 1 with one host-path-free `[PRECONDITION]` line per unmet subject (per layout-templates §Surface: cli → Primary screens (commands), the `scripts/agent-run.{sh,ps1}` entry; → IA notes, Headless invariant).
- Error output goes to stderr, sanitized (no absolute host paths / internal struct names / stack traces), as `error: <short>` + `hint: <fix>`; stdout stays reserved for raw artifact data (per layout-templates §Surface: cli → Component — Footer / terminator + error output).

## Patterns to follow
- If the round's graded results surface through a Conductor-rendered run report, per-check detail rides as INDENTED plain-text lines beneath the P-ID verdict line — no new column, no new bracket label, no new ANSI/token entry (per layout-templates §Surface: cli → Component — Primary content block 2, Per-check detail region). Whether the round's evidence is a Conductor run report at all, or test-written files under the chunk's `evidence/`, is research's question.
- Status is always paired with its ASCII prefix (`[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]`) so it survives `NO_COLOR` / piping (per layout-templates §Surface: cli, the Signature placement paragraph; → IA notes, Pipe discipline).
- `[PRECONDITION]` has two reachable arms (one line per unmet subject, or the single satisfied line); a reader keying on the bracket label must not treat it as failure-only (per layout-templates §Surface: cli → Primary screens (commands), the `conductor preconditions` entry).

## Anti-patterns to avoid
- Rendering the missing-tool `Blocked` (or an UNGRADED absent sample) as a red `[FAIL]` — `Blocked` is never-measured and never red (per layout-templates §Surface: cli → Component — Primary content block 1).
- Minting a seventh lamp or a new run-level bracket label for the incident-events assertion (per layout-templates §Surface: cli → Component — Primary content block 2).
- Any interactive prompt on the headless live-leg path (per layout-templates §Surface: cli → IA notes, Headless invariant).

## Contract bindings
- layouts ↔ arch/security (MCP contract): the readiness line's tool count and the missing-tool `[BLOCKED]` string are rendered from the `contracts/mcp-contract.toml` `required_tools` set and the preflight's named precondition; the precondition's wording and host-path freedom are owned by the security/arch plans, layouts owns only its placement (per layout-templates §Surface: cli → Component — Header / banner).
- layouts ↔ a11y: ASCII prefixes are the stated basis of NO_COLOR / screen-reader compliance (per layout-templates §Surface: cli, the Signature placement paragraph).

## Acceptance criteria contributions
- (layouts) After the fifth `required_tools` pin, a preflight against Pulse S renders its readiness line with an `[OK]` prefix and a tool count whose denominator matches the pinned set; against a Pulse lacking `retrieve_incident_events` it renders `[BLOCKED]` with the named missing-tool precondition, never a generic or red failure (per layout-templates §Surface: cli → Component — Header / banner; → Component — Primary content block 2).
- (layouts) The chunk adds no column to the 6-column results/SLO table and no bracket label beyond the closed six-lamp set plus the `[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]` qualifiers (per layout-templates §Surface: cli → Component — Primary content block 1; → Component — Primary content block 2).
- (layouts) The live leg's refusal on an unmet live-Pulse subject prints host-path-free `[PRECONDITION]` lines and exits 1 without firing any leg (per layout-templates §Surface: cli → Primary screens (commands), the `scripts/agent-run.{sh,ps1}` entry).
