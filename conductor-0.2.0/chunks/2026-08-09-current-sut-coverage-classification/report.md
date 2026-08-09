# Report — 2026-08-09-current-sut-coverage-classification

**Chunk:** Current-SUT coverage classification — every manifest capability classified across four modes incl. the recorded not-Conductor's boundary, retiring the KNOWN_UNCLASSIFIED residual ledger (conductor-core, v2-03)
**Date:** 2026-08-09
**Commits:** (none yet — this wrap authors the chunk commit; prior HEAD `0ab873b feat(2026-08-09-sut-drift-check)`)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-core/src/coverage.rs` — 4th `CoverageMode` variant; table widened 60 → 82 rows; module/field/fn docs de-hardcoded; 2 literal-60 tests replaced by 6 manifest-relative ones.
  - `crates/conductor-core/src/drift.rs` — `KNOWN_UNCLASSIFIED` → `&[]` + doc comment rewritten (retirement recorded, re-population discouraged).
  - `crates/conductor-report/src/coverage.rs` — module doc; footer golden split (exact-string over synthetic rows + manifest-relative totals); row-count assertion manifest-relative; leak list extended.
  - `crates/conductor-cli/src/render.rs` — coverage-table doc; `for n in 1..=60` → iterate `coverage_matrix()`.
  - `crates/conductor-cli/src/cli.rs` — clap **help text** (user-visible) de-hardcoded.
  - `crates/conductor-cli/src/commands/coverage.rs` — module doc.
  - `crates/conductor-cli/tests/cli_smoke.rs` — stdout assertion now iterates every classified P-ID.
  - `crates/conductor-tauri/src/commands.rs` — command doc; `Some(60)` → `Some(coverage_matrix().len())`.
  - `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — `CoverageMode` union + `MODES` const; `tally()` now iterates all four modes.
  - **SPEC BODIES (see Deviations):** `.andromeda/architecture.md:33` · `.andromeda/design-system.md:7,:257` · `.andromeda/layout-templates.md:121,:177,:222`.
- **Symbols / APIs:** `CoverageMode::NotConductors` (new variant, serde wire `not-conductors`) · `CoverageMode::ALL` widened `[_;3]`→`[_;4]` · `CoverageMode::label()` gains an arm · `KNOWN_UNCLASSIFIED` now empty. **Unchanged:** `coverage_matrix() -> &'static [CapabilityRow]` signature · `CapabilityRow` shape · `Verdict` · `ReportState` · the six-lamp set · every `#[tauri::command]` name · no new env var, port, MCP tool or CLI verb.
- **Crates / modules:** none added or removed. Changed: `conductor-core`, `conductor-report`, `conductor-cli`, `conductor-tauri` (+ its webview subtree).
- **Dependencies:** none added, none bumped. `Cargo.lock` un-drifted (`git diff --exit-code` clean). No `deny.toml` change.
- **Schema / config:** no migration, no config key, no violation schema. `contracts/pulse-capabilities.toml` **unchanged** (operator decision: classification stays code-native; the manifest remains ids-only, so `check_sut_drift` stays a genuine two-source comparison).
- **Coverage of new surfaces:**
  - `CoverageMode::NotConductors` (classification value, rendered to Markdown / CLI table / webview) → validation n/a (not deserialized input; it is a command **return**) · instrumentation n/a (no new span; `coverage_matrix()` stays a no-IO `static`) · PII n/a · tests unit (7 refs incl. wire-spelling golden) · a11y — renders as DOM text in the Mode cell and as text in the CLI table, never color-only; `tally()` counts it · tokens n/a (no new token, hex or ANSI; the follow-on chunk owns visual treatment).

## Deviations from intent

1. **Three spec BODIES were edited during /implement** (`architecture.md`, `design-system.md`, `layout-templates.md`). Justification at the time: the approved `plan.md` listed them under *Codebase touchpoints → Files to modify*, and the operator directly instructed one of the edits (`layout-templates.md:222`) at the P5 review. **This is a process deviation the operator has since ruled on: the pipeline fix is encoded (plans can no longer route spec masters through implement), and this wrap performs the instance repair — the three bodies keep their edits, and this wrap authors the sidecar entries + cascade that the skipped half of the amendment flow owes.** The bodies are at current truth; only the history half was missing.
2. **`layout-templates.md:222` folded in at operator direction** (P5), overriding the layouts distiller's "may stay" on an illustrative sample. Justification: a baked count in a sample is the same stale-derived-fact class as one in prose, and leaving it would have recreated the `:37`-vs-`:121` self-contradiction the chunk eliminates. The caption's per-state tally and its step denominator were fixed **together** (77+1+1+1+1+1 = 82 = denominator).
3. **`scope.md` amended 3× under validation-1** (intent-incomplete, not defect): the P-075/P-076 classification call research settled from Pulse's own matrix; the literal-60 surface being 11 code sites rather than 2; and the third stale doc site the operator folded in.

## Decisions & corrections

- **Representation (operator, P4):** classification stays **code-native**; the manifest stays ids-only. Rejected: moving title/category/mode into the TOML — it would have made `coverage_matrix()` fallible + runtime-IO (fanning out to 4 crates) and collapsed `check_sut_drift` into a self-check.
- **Lane modes (operator, P4):** `auto` = P-073/P-074/P-079 · `drive+observe` = P-067/P-072/P-075. P-075 is drive+observe because three of the four timing budgets it aggregates (P-025, P-027, P-037) are already drive+observe — classifying the aggregate as auto would contradict its own members.
- **`:222` folded in (operator, P5)** — see Deviations 2. General principle stated by the operator: illustrative-sample status does not exempt a baked count.
- **Wire spelling `not-conductors`** (implement): the requirement's prose is "not-Conductor's"; the apostrophe was dropped because the value is parsed by a CLI column, a JSON wire and a TS union, and the existing family is lowercase-kebab (`auto` / `drive+observe` / `static-only`).
- **Correction (research):** the tests distiller's literal-60 inventory was correct but incomplete — a first-hand grep found 8 further doc/help sites, one of which (`cli.rs:49`) is user-visible clap help text, plus a false positive to guard (`render.rs:42` `Lamp::Blocked => 60` is an ANSI color code).
- **Correction (research):** the layouts distiller called the CLI "coverage/SLO table" 6 columns, conflating two tables — `render.rs:161` shows the coverage table is 4 columns, `:140` the run/SLO table is 6.
- **Sourcing finding:** the 22 capability titles are not reproducible from this repo. P-061..P-078 came from Pulse's `andromeda-pulse-0.3.0/requirements.md`; **P-079..P-082 exist only in Pulse's `verification-matrix.json`**, minted mid-build as operator-surfaced caps.

## Outcome

**Met acceptance criteria: yes.** All 82 manifest-accepted ids classified into exactly one mode (set-equality against the loaded manifest, zero unclassified, zero orphans); the 16-row not-Conductor's set explicitly enumerated; `KNOWN_UNCLASSIFIED == []` with `check_sut_drift` still `Err` on an injected unclassified id; no literal count or P-ID range left in any workspace assertion; `Verdict`/`ReportState`/the six-lamp set untouched.

**Gates green (0 fix-loop iterations):** `cargo nextest run -p conductor-core` **192/192** · `npm run build` (tsc --noEmit + Vite) ✓ · `cargo nextest run --workspace --profile ci` **441/441 zero retries** · `cargo test --workspace --doc` ok · `cargo clippy --workspace --all-targets -- -D warnings` clean (9 crates) · `cargo audit` exit 0 · `cargo deny check` advisories/bans/licenses/sources ok · `Cargo.lock` un-drifted · `bash scripts/agent-run.sh run` **exit 0**.

**Smoke:** boot-path half ✓ — `agent-run.sh run` exit 0, plus the real binary driven directly (`conductor coverage` / `--write`): 82 rows, tally `43 auto · 16 drive+observe · 7 static-only · 16 not-conductors` summing to 82, new mode rendering as text on both surfaces, zero identifier leaks. `agent-run.sh status` not applicable (needs a `run_id` from a live run; no boot-path change). UI half **skipped — headless**: the tauri-driver/axe harness is display-gated to Linux+xvfb with a live Pulse; `tsc --noEmit` + the mock-runtime IPC test stood in.

**Surfaced (nothing authored by implement):**
1. `.andromeda/test-plan.md:78` — live spec claim "enumerates all **60** P-IDs"; also lists only three modes and references `coverage-matrix.md`. **This wrap's W12 citation target.**
2. `crates/conductor-tauri/ui/src/components/CoverageMatrix.css:1` — stale "all 60 capabilities" comment. Nit; noted only.
3. `coverage-matrix.md` has never existed in this repo though `architecture.md` §Occupied Resources registers it. Generating it to verify created an untracked file `git add -A` would have committed against a plan declaring zero new files, so it was removed after inspection. **Routed to route-resolve as a CARRY on the Epoch-6 coverage-gate entry**, not an amendment.
