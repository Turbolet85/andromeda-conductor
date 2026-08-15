# layouts extract

## Relevance
Partial — no webview surface is touched, but the chunk's live leg drives `conductor preflight` and the run-header readiness line, which are documented cli-surface output structures.

## Constraints
- The preflight verb's rendering contract is fixed: `conductor preflight [--json]` must emit the `ReadyState` JSON *or* a `[PASS]`/`[BLOCKED]` line and exit 0 iff `ready:true`, non-zero on a Blocked precondition — raising `CANARY_STORM_COUNT` must change the *outcome*, never that shape (per layout-templates §Surface: cli → Primary screens).
- The canary result is a *rendered field*: §Component — Header / banner requires the preflight readiness line to carry protocol version, tool count and canary result with an `[OK]`/`[BLOCKED]` prefix, identifiers in the ID-cyan mapping and metadata dimmed. Whether the shipped header already renders the canary result in that shape is research's question (per layout-templates §Surface: cli → Component — Header / banner).
- Every status must be paired with an ASCII text prefix so the signal is never color-alone and survives `NO_COLOR` / piping / screen readers — this applies to whatever the raised-count canary reports (per layout-templates §Surface: cli → Signature placement, closing paragraph).
- The headless invariant: `agent-run boot` — which *is* the preflight entrypoint — must never be gated on an interactive `inquire` prompt; the operator-gated live leg must not introduce one (per layout-templates §Surface: cli → IA notes, Headless invariant).
- Pipe discipline: raw artifact data (the `--json` `ReadyState`) on stdout, human messages on stderr, ANSI auto-stripped when piped, no emoji in machine-parseable output (per layout-templates §Surface: cli → IA notes, Pipe discipline).
- The per-P-ID bracket-label set is closed at six (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) plus the single run-level `[ENVIRONMENT-SUSPECT]` qualifier; a raised canary count may not introduce a new label or lamp state (per layout-templates §Surface: cli → Component — Primary content block 2).
- Any roll-up number the chunk moves must stay manifest-derived, never a baked literal, and must render identically on all three surfaces (Markdown · cli · webview `unbacked` prop) (per layout-templates §Surface: cli → Primary screens, `conductor coverage [--write]`).

## Patterns to follow
- **Readiness-gate line** (§Surface: cli → Component — Header / banner): bracket prefix carries the gate for `NO_COLOR`, negotiated identifiers in ANSI 117, metadata dimmed in ANSI 146 — the existing shape a changed canary outcome reads through.
- **Run-level qualifier riding outside the lamp column** (§Surface: cli → Output structure `conductor run` + Component — Primary content block 2): `[ENVIRONMENT-SUSPECT]` printed once per run, ASCII label always printed, omitted entirely when in-envelope. This is the precedent to reuse if the load-envelope premise turns out to touch the canary — never a new per-check state.
- **Blocked ≠ red** (§Surface: cli → Component — Primary content block 1 and 2): a Blocked precondition carries its named precondition string and renders measurement columns as `—` / null, never a red error. A live leg that does not reach `ready:true` reads as Blocked-with-precondition, not as a failure.
- **Output stability as a parse contract** (§Surface: cli → IA notes, Command model): downstream agents parse cli output, so adding a column or field without a `--format` flag is a breaking change.

## Anti-patterns to avoid
- Do not introduce a seventh lamp state, a new bracket label, or an extra table column to express the raised canary/storm count (per layout-templates §Surface: cli → Component — Primary content block 1 and 2 · IA notes Command model).
- Do not signal the canary/preflight result by color alone, or with emoji, in piped/machine-parseable output (per layout-templates §Surface: cli → IA notes, Pipe discipline).
- Do not add an interactive gate to the live leg's boot path (per layout-templates §Surface: cli → IA notes, Headless invariant).

## Contract bindings
- **ASCII bracket prefixes → a11y / `NO_COLOR`**: layouts states the pairing as a layout requirement; a11y derives the conformance claim (per layout-templates §Surface: cli → Signature placement + §Decisions Log Notes).
- **`[PASS]`/`[BLOCKED]` line + `ReadyState` JSON → tests/obs harness**: this output *is* the release gate the `agent-run` harness parses, so any shape change is a harness-contract change (per layout-templates §Surface: cli → Primary screens · IA notes Command model).
- **ANSI codes ↔ CSS var token names → design**: layouts references tokens by name only (ANSI 114/179/203/60/117/146); the design plan owns the values (per layout-templates §Surface: cli → IA notes, Multi-surface coordination).
- **Derived roll-up counts → webview coverage matrix `unbacked` prop**: if the P5 `v2-10` claim moves any manifest-derived count, the same number must reach the webview via the `unbacked_auto` command, never mirrored in TypeScript (per layout-templates §Surface: cli → Primary screens, `conductor coverage [--write]`).

## Acceptance criteria contributions
- (layouts) `conductor preflight [--json]` still emits the `ReadyState` JSON or a `[PASS]`/`[BLOCKED]` line and exits 0 iff `ready:true`, non-zero on a Blocked precondition, after the constant moves (per layout-templates §Surface: cli → Primary screens).
- (layouts) The run/suite header readiness line still renders protocol version · tool count · canary result with its `[OK]`/`[BLOCKED]` ASCII prefix, so the gate reads under `NO_COLOR` and when piped (per layout-templates §Surface: cli → Component — Header / banner).
- (layouts) No new bracket label, lamp state, or table column is introduced for the raised canary count; the per-P-ID label set stays six plus the one run-level qualifier (per layout-templates §Surface: cli → Component — Primary content block 2).
- (layouts) The live leg's `agent-run boot` path completes without an interactive prompt, and any roll-up count the chunk moves remains manifest-derived on all three surfaces rather than a literal (per layout-templates §Surface: cli → IA notes Headless invariant · Primary screens `conductor coverage`).

## Relevant amendment history
- **2026-06-23-5-command-agent-run-harness** — registered the `conductor preflight [--json]` verb (readiness gate / `agent-run boot` entrypoint; exit 0 iff `ready:true`) and the `agent-run.sh` 5-command set + `run` stage flags in §cli Primary screens. This is the exact surface the chunk's live leg exercises; the exit-code and prefix wording documented there is the contract to hold.
- **2026-08-09-sut-load-envelope** — added the run-level `[ENVIRONMENT-SUSPECT]` caption (ANSI 246, ASCII label always printed, outside the lamp column, omitted when in-envelope) and stated the six verdict labels are the closed per-P-ID set. Directly germane to the scope's `[inferred]` premise that the canary does not interact with `contracts/pulse-load-envelope.toml`: if it does, this is the documented rendering precedent, and the lamp set still may not grow.
- **2026-08-13-dispatcher-determinism-goldens** — re-based the `[ENVIRONMENT-SUSPECT]` sample caption to name the breaching emitting phase and the sustained term rather than a whole-scenario literal, per the guidance that a replacement literal simply re-stales. Same guidance applies if this chunk's storm-count change alters what that caption can produce.
- **2026-08-09-interpretation-correctness-posture · 2026-08-09-in-lane-sut-scenarios · 2026-08-10-pulse-run-contract** — three successive re-bases of the coverage roll-up `(N unbacked)` qualifier (11→10→9), each detector-raised at wrap off the report's "counts moved" bullet. Precedent: if the P5 `v2-10` claim decision moves a manifest-derived count, the caption sample in the plan is a wrap-time amendment, not phase or implementation work.
