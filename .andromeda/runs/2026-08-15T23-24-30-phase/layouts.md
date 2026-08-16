# layouts extract

## Relevance
**Partial** — the chunk is a Rust emit-path/enumeration fix with no webview surface work; the only layout-owned touchpoint is the **cli `conductor preflight` readiness output** (the gate whose `ready:true` this chunk must reach) and its `[OK]`/`[BLOCKED]` render.

## Constraints
- The `conductor preflight [--json]` verb is a registered cli surface whose contract is: emit the `ReadyState` JSON (or a `[PASS]`/`[BLOCKED]` line) and **exit 0 iff `ready:true`, non-zero on a Blocked precondition** — per `layout-templates` §Surface: cli — Primary screens (commands). Any change to the canary path must leave this contract intact.
- The preflight readiness line is a documented header element carrying **protocol version · tool count · canary result**, with negotiated identifiers in ANSI 117 and an `[OK]`/`[BLOCKED]` prefix so the gate reads without color — per §Surface: cli — Component — Header / banner (and shown in §cli Output structure `conductor run <scenario>` as `preflight  protocol …  tools 4/4  canary ok`). The canary's outcome is what that final term renders; whether the code already prints a distinguishable canary term there is research's question.
- A not-ready gate must render as `[BLOCKED]` (ANSI 60, `count-blocked` mapping) **with its named precondition string**, measurement fields as `—`/null — never a red error, because `Blocked` ≠ red (never measured) — per §Surface: cli — Component — Primary content block 2 (verdict / report-state lines) and §Component — Primary content block 1 (results / SLO table).
- Every cli status must pair its color with an **ASCII bracket prefix** so the signal survives `NO_COLOR` / piping / screen readers — a stated layout requirement, per §Surface: cli — Signature placement (closing paragraph) and §IA notes — Pipe discipline.
- The headless invariant binds the re-run leg: the `agent-run boot` / source-of-truth path is **never gated on an interactive prompt**, and raw artifact data stays on stdout with human messages on stderr — per §Surface: cli — IA notes (Headless invariant · Pipe discipline).
- Output structure is a parsed contract: downstream agents parse it, so adding a `comfy-table` column without a `--format` flag is a breaking change — per §Surface: cli — IA notes (Command model). Applies if the canary correction alters what the gate reports.

## Patterns to follow
- **`[BLOCKED]` + named-precondition** is the established render for a never-measured gate (`• P-022  Blocked  mcp-server feature + ANDROMEDA_PULSE_MCP_ENABLED + matching data-dir`) — reuse this shape for any preflight non-ready outcome rather than inventing one (§cli Component — Primary content block 2).
- **Run-level, non-lamp qualifier outside the lamp column** — the `[ENVIRONMENT-SUSPECT]` caption (ANSI 246, printed once per run, ASCII label always printed, omitted entirely when not applicable) is the standing precedent if the chunk needs to surface a producer/envelope caveat without touching per-check state (§cli Component — Primary content block 2 · §cli Output structure `conductor run`).
- **Derived counts are sourced, never literals** in cli captions/labels — the roll-up caption's numbers are manifest-derived and the doc's samples are kept re-basable (§cli Primary screens, `conductor coverage [--write]`).
- **Terminator discipline** — a single dimmed closing line naming the artifact destination (ANSI 146, un-colorized when piped); errors to stderr as `error: <short>` + detail + `hint: <fix>` (§cli Component — Footer / terminator + error output).

## Anti-patterns to avoid
- Do **not** add a new bracket label / seventh lamp state for the canary-enumeration gap: the per-P-ID verdict/report-state set is **closed at six**, plus the one run-level non-lamp qualifier (§cli Component — Primary content block 2).
- Do **not** render a preflight that fails to reach `ready:true` as a red `[FAIL]` — the "no surprise failure" rule requires `[BLOCKED]` with its precondition (§cli Component — Primary content block 1 / block 2).
- Do **not** introduce color-only or emoji signalling in machine-parseable (piped) output — ASCII prefixes only there (§cli IA notes — Pipe discipline).

## Contract bindings
- **cli ASCII-prefix pairing ↔ a11y / `NO_COLOR`** — `layout-templates` states the pairing as the layout requirement from which a11y / NO_COLOR conformance derives (§cli Signature placement; §Decisions Log Notes).
- **`conductor preflight --json` `ReadyState` on stdout + exit code ↔ the agent-run harness / test-gate domain** — the JSON shape and the `exit 0 iff ready:true` rule are the parsed contract the harness gates on (§cli Primary screens; §IA notes Headless invariant). If this chunk's `v2-10` claim is measured through that verb, its output shape is the shared boundary.
- **No webview binding this chunk** — the desktop-webview wireframes carry no preflight line (§Surface: desktop-webview wireframes), so a canary/emit correction adds no webview region, component, or focus-order position.

## Acceptance criteria contributions
- (layouts) `conductor preflight [--json]` still emits the `ReadyState` JSON (or the `[PASS]`/`[BLOCKED]` line) and exits **0 iff `ready:true`**, non-zero otherwise (per `layout-templates` §Surface: cli — Primary screens).
- (layouts) The preflight readiness line renders `protocol … · tools N/N · canary <result>` with an `[OK]`/`[BLOCKED]` ASCII prefix, legible under `NO_COLOR` / piping (per `layout-templates` §Surface: cli — Component — Header / banner).
- (layouts) A gate that does not reach `ready:true` renders `[BLOCKED]` (ANSI 60) carrying its **named precondition**, measurement fields `—`/null — never a red `[FAIL]` (per `layout-templates` §Surface: cli — Component — Primary content block 2).
- (layouts) No new bracket label or lamp state is introduced for the canary gap; the per-P-ID set stays six and any run-level caveat rides outside the lamp column (per `layout-templates` §Surface: cli — Component — Primary content block 2).

## Relevant amendment history
- **`2026-06-23-5-command-agent-run-harness`** — registered the `conductor preflight [--json]` verb (cli verb surface 3→4) as the readiness gate / `agent-run boot` entrypoint, plus the 5-command harness and `run` stage flags. This is the verb this chunk's live leg must drive to `ready:true`; its exit-code contract was fixed here.
- **`2026-08-09-sut-load-envelope`** — established the **run-level, non-lamp** `[ENVIRONMENT-SUSPECT]` caption and explicitly named the six per-P-ID labels as the closed set. Directly relevant: it is the only sanctioned way to surface a run-level producer/envelope caveat without adding a state.
- **`2026-08-13-dispatcher-determinism-goldens`** — re-based that caption's sample to name the breaching emitting phase and the asserted sustained terms rather than baking a duration literal; the standing guidance is to name the term SET, since a replacement literal simply re-stales. Applies if this chunk's measurement lands any new sample text in the plan.
- **`2026-08-09-interpretation-correctness-posture` · `2026-08-09-in-lane-sut-scenarios` · `2026-08-10-pulse-run-contract`** — successive re-bases of the coverage roll-up caption's `(N unbacked)` qualifier whenever `conductor_core::UNBACKED_AUTO` moved. Relevant only as a guard: this chunk's Boundaries exclude scenario-catalog changes, so the qualifier should not move; if it does, all three surfaces' captions re-render from the manifest, never from a literal.
