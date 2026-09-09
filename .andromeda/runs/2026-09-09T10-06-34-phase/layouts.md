# layouts extract

## Relevance
Partial — no surface, wireframe, component-placement or focus-order work; layouts contributes only a no-regression guard on the cli rendered surface (7 fmt sites sit in `conductor-cli`, 10 in `conductor-tauri`) and on the CI-reachability rules its plan states for the harness.

## Constraints
- The cli output structure is a stability contract, not cosmetics — layout-templates §cli IA notes (Command model) requires it stay stable across versions because downstream agents parse it, and calls a column change without a `--format` flag a breaking change; a formatting-only pass must therefore leave the rendered shape identical. Whether rustfmt actually touches any render site in `conductor-cli`/`conductor-tauri` is research's question.
- The two table shapes must stay distinct and unchanged: 6 columns for the results/SLO table (`run`/`suite`) and 4 for the coverage matrix (`coverage`), widths terminal-detected and never hardcoded (per layout-templates §cli Component — Primary content block 1).
- Every status must remain paired with its ASCII bracket prefix (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]`/`[MANUAL]`/`[RESIDUAL]`) so the signal survives `NO_COLOR`/piping; the per-P-ID lamp set is closed at six and the run-level qualifier set (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) rides outside the lamp column (per layout-templates §cli Component — Primary content block 2).
- `--live` is required to be reachable from no default and no CI path (per layout-templates §cli Primary screens, the `scripts/agent-run.{sh,ps1}` bullet) — the added `ci.yml` step must not create such a path.
- The headless invariant: the agent/CI path is never gated on an interactive prompt (per layout-templates §cli IA notes — Headless invariant); a `cargo fmt --check` step satisfies this by construction, but the added step must not be chained onto a prompting leg.
- Pipe discipline is per-stream: raw artifact data on stdout, human messages on stderr, ANSI auto-stripped when piped (per layout-templates §cli Component — Header / banner and §IA notes — Pipe discipline) — this is the mechanism any before/after output comparison must use to be meaningful.
- The desktop-webview surface is out of the pass's reach by the chunk's own Rust-only boundary; layout-templates §Surface: desktop-webview mandates no `.rs`-side layout structure, so no wireframe/component/focus-order change is required or permitted here.

## Patterns to follow
- Adapted-per-surface, never forked: the same label/caption exists on cli stdout, the Markdown report and the webview banner (per layout-templates §cli IA notes — Multi-surface coordination and §desktop-webview IA notes) — if the pass were to move any rendered literal on one leg, the other two would diverge; the guard is that it moves none.
- Compare piped, ANSI-stripped output at a fixed terminal width when establishing that the rendered surface is unchanged — the tables size themselves from the terminal (per layout-templates §cli Component — Primary content block 1) and colour is `IsTerminal`-gated per stream (per §cli Component — Header / banner).
- The byte-identity partition the chunk plans for `.rs` files has a rendering analogue this plan already relies on: the ASCII label always prints regardless of colour, so a text-level diff of piped output is a complete check of the layout contract (per layout-templates §cli Component — Primary content block 2).
- Set-naming over baked literals: layout-templates states its counts and verb set by mechanism (`Commands` declares the verb set; roll-up numbers manifest-derived, never a literal) — no doc-side literal should be introduced or re-pinned on account of this chunk (per layout-templates §cli Primary screens — `conductor coverage`).

## Anti-patterns to avoid
- Do not let a re-wrap add, drop or reorder a `comfy-table` column, or split/alter a bracket label or caption string — that is a breaking output change under layout-templates §cli IA notes (Command model).
- Do not mint a new bracket label, lamp state, ANSI entry or token as a side effect; the lamp set is closed at six and the qualifier set rides outside it (per layout-templates §cli Component — Primary content block 2).
- Do not attach the new CI step to anything that reaches `--live` or an `inquire` prompt (per layout-templates §cli Primary screens, agent-run bullet, and §cli IA notes — Headless invariant).

## Contract bindings
- layouts ↔ tests/CI-harness domain: layout-templates §cli Primary screens (the `agent-run.{sh,ps1}` bullet) is the plan's home for stage-flag and CI-arm statements (`--unit`/`--integration`/`--e2e`/`--live`, the `CONDUCTOR_A11Y_STRICT` strict arm). The harness/tests domain owns where the fmt gate sits in `ci.yml`; layouts owns only that the added job/step disturbs neither the stage set nor `--live`'s no-CI-path rule.
- layouts ↔ docs/test-plan: `test-plan.md:455` (Lint row) is the descriptor this chunk makes true; layout-templates does not own that row and states no requirement on it.

## Acceptance criteria contributions
- (layouts) cli rendered output is byte-identical across the pass when piped at a fixed width: the results/SLO table keeps exactly 6 columns and the coverage table exactly 4, with the roll-up caption shape unchanged (per layout-templates §cli Component — Primary content block 1).
- (layouts) All six per-P-ID bracket labels and the run-level `[ENVIRONMENT-SUSPECT]` / `[PRECONDITION]` qualifier lines still print, each paired with its ASCII prefix, with no new label, lamp, token or ANSI entry introduced (per layout-templates §cli Component — Primary content block 2).
- (layouts) The added CI step introduces no path from a default/CI run to `--live` and blocks on no interactive prompt (per layout-templates §cli Primary screens, `scripts/agent-run.{sh,ps1}` bullet, and §cli IA notes — Headless invariant).

## Relevant amendment history
- `2026-09-06-operator-gated-live-suite` — registered `--live` on the agent-run bullet with the explicit "reachable from no default or CI path" rule, and fixed the refuse-vs-skip classes apart. Why it matters here: this chunk edits the only workflow file, so the rule it added is the one a new CI step could silently violate.
- `2026-09-07-a11y-ci-gate` — amended the same bullet so the host-tool skip is the LAX arm and an affirmative `CONDUCTOR_A11Y_STRICT` exits non-zero, i.e. CI adds a third outcome rather than replacing either; it also records that two sentences carrying the same claim sat ~400 chars apart on ONE line, so a single-sentence edit would have left a retired verdict standing intra-line. Why it matters: precedent for how a CI arm is stated on this surface, and a caution that this file's cli bullet packs multiple claims per physical line. Note: this entry is truncated mid-word in the sidecar ("would ha"), with its tail apparently orphaned at the file's end — the history is readable but the entry is incomplete.
- `2026-09-07-sr-findings-fixed` (part 2, cascade) — conditioned the verbatim citation of test-plan §3's shell-parity claim: parity is a property of the scripts together with the invoking environment. Why it matters: CI is such an invoking environment, so a gate added to `ci.yml` sits on the conditioned side of that claim.
- `2026-08-16-fingerprint-storm-live-proof` — established the de-literalization rule after a sample count re-staled three times. Why it matters: it is the standing bar against introducing any fresh literal into this plan on account of a chunk; a formatting/CI chunk should move no count here at all.
