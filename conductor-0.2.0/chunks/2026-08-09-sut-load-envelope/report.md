# Report — 2026-08-09-sut-load-envelope

**Chunk:** SUT load envelope — proven-good storm bounds recorded for scenario authors + over-envelope runs flagged environment-suspect (conductor-core, v2-06/v2-07)
**Date:** 2026-08-10
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:**
  - NEW `contracts/pulse-load-envelope.toml` · NEW `crates/conductor-core/src/load_envelope.rs`
  - MOD `crates/conductor-core/src/{lib.rs, error.rs}`
  - MOD `crates/conductor-report/src/{db.rs, report.rs}`
  - MOD `crates/conductor-run/src/lib.rs`
  - MOD `crates/conductor-cli/src/{paths.rs, render.rs, commands/run.rs, commands/suite.rs}` · MOD `crates/conductor-cli/tests/cli_smoke.rs`
  - MOD `crates/conductor-tauri/src/commands.rs`
  - MOD all 34 `scenarios/*.toml` (3-line header only — no config-value change)

- **Symbols / APIs:**
  - `conductor-core` NEW public: `LoadEnvelope` · `EnvelopeTerms` · `Exemption` · `EnvelopeStatus` (`InEnvelope` | `EnvironmentSuspect(String)`, + `label()`/`cause()`/`is_suspect()`) · `check_load_envelope()` · `scenario_duration_ms()` · `CoreError::LoadEnvelope` variant
  - `conductor-report` NEW: `RunsDb::insert_envelope()` · `RunsDb::get_envelope()`. CHANGED signature: `RunReport::write()` / `RunReport::render()` each take `&EnvelopeStatus`
  - `conductor-run` NEW: `classify_run()`. CHANGED signature: `persist()` +`&EnvelopeStatus`, `drive_run()` +`&EnvelopeStatus`
  - `conductor-cli` NEW: `render::envelope_caption()` · `Paths::load_envelope()` · `Paths.load_envelope_path` field
  - `conductor-tauri` NEW private: `load_envelope()`. CHANGED signature: `run_thread()` +`EnvelopeStatus`
  - **No new IPC method, no new Tauri command, no new endpoint, no new port/socket, no new env var.**

- **Crates / modules:** no crate added/removed. One new module `conductor-core::load_envelope`. No new cross-seam dependency edge (verified: `cargo check --workspace` clean, no `Cargo.toml` diff).

- **Dependencies:** **none added, none bumped.** `Cargo.lock` un-drifted; `package-lock.json` untouched; zero `Cargo.toml` diff.

- **Schema / config:**
  - NEW committed config artifact `contracts/pulse-load-envelope.toml` (`sut_version` · `captured_at` · `provenance` · `[envelope]` 3 terms · `[[exempt]]` array of `{scenario, reason}`). Fixed path via `LoadEnvelope::default_path()` → `resolve_under`, **deliberately no `CONDUCTOR_*` override handle** (mirrors the capability-manifest precedent).
  - NEW `runs.db` table `run_envelope (run_id TEXT PK, classification TEXT NOT NULL, cause TEXT)`, additive under the same `CREATE TABLE IF NOT EXISTS` bootstrap. **The `runs` table is byte-unchanged** — still eleven columns, still `PRIMARY KEY (run_id, scenario)`; asserted by a test reading `pragma_table_info('runs')`.
  - The per-run JSONL journal envelope is **unchanged** — still exactly eleven fields; a test asserts the run-level qualifier does not leak into it.

- **Counts / qualifiers this chunk moved:**
  - **`contracts/` committed artifacts: 2 → 3.** Was `mcp-contract.toml` + `pulse-capabilities.toml`; now also `pulse-load-envelope.toml`. Sites enumerating them: `architecture.md` §Occupied Resources (on-disk artifacts bullets) and the §Infrastructure Patterns directory tree, whose `contracts/` comment names exactly two.
  - **`runs.db` tables: 1 → 2** (`runs` + `run_envelope`). Sites stating the old qualifier: `architecture.md` §Stack table ORM row ("~one-table run-metadata index") and §Established Decisions [ORM] ("`runs.db` is ~one indexed table").
  - **Bracket labels appearing in cli stdout: 6 → 7.** The seventh is `[ENVIRONMENT-SUSPECT]`. **It is NOT a lamp and NOT a `ReportState`** — it is a run-level qualifier rendered outside the lamp column, exactly like the `not-conductors` Mode cell precedent. The **lamp set is still six** and `ReportState` is still five. Sites enumerating the six labels as a set: `design-system.md` §Surface: cli (status-prefix list + Verdict/report-state lines), `test-plan.md` §Surface: cli and §driver table (selector labels), `layout-templates.md` cli wireframes. Correct fix = note the non-lamp label / name the set; **never** re-state a fresh literal, and never imply a seventh lamp.
  - **Non-lamp reuses of the Residual-mute tier (ANSI 246 ↔ `--status-residual`): 2 → 3.** Was the `hint:` stderr label + the out-of-scope Mode cell; now also the environment-suspect caption. Site stating the old count verbatim: `design-system.md` §Color Palette — "ALSO the shared recessive tier for **two** NON-lamp uses". (Note: no derived-count detector is scoped to `design-system`; the two shipped ones cover `layout-templates` and `test-plan` only.)
  - **Workspace tests: 463 → 494** (+31), zero retries.
  - **Deliberately NOT moved:** `ReportState` variants **5** · lamp treatments **6** · `UNBACKED_AUTO` **10** · coverage classification **82 capabilities / 66 in scope** · scenario catalog **34** · the eleven-field run-report envelope · the eleven-column `runs` table.

- **Coverage of new surfaces:**
  - `contracts/pulse-load-envelope.toml` (new external-input surface) → validation **✓** (bounds-checked at load: non-empty identity/provenance, positive terms, no duplicate/reasonless exemption; fixed path through `resolve_under`, no `CONDUCTOR_*` handle; read faults carry `e.kind()` only, asserted not to contain the path) · instrumentation **✓** (`tracing::info!` on load, mirroring the capability manifest) · PII **n/a** · tests **unit ✓** · a11y **n/a** · tokens **n/a**
  - `run_envelope` table + its writes (new schema surface) → validation **n/a** (self-generated values) · instrumentation **n/a** · PII **redacted✓** (identity-only cause; tests assert no absolute host path and no internal struct/field name) · tests **unit + integration ✓** (round-trip both variants, duplicate-insert is `Err`, `runs`-table contract asserted unchanged) · a11y **n/a** · tokens **n/a**
  - cli run-level envelope caption (new UI element) → validation **n/a** · instrumentation **n/a** · PII **redacted✓** · tests **unit ✓** (plain keeps the ASCII label with no escapes · colored overlays escapes · absent when in-envelope · tier is neither fail nor blocked) · a11y **n/a** (cli is not an a11y-assertable surface per a11y-plan §11 Universal) · tokens **✓** (reuses ANSI 246, **zero new palette/ANSI entries**)
  - Markdown run-report envelope banner (new UI element) → validation **n/a** · instrumentation **n/a** · PII **redacted✓** (test asserts no host path / struct name in the banner) · tests **unit ✓** (banner present + above the tally · absent when in-envelope · alters no check row · no leaks) · a11y **n/a** (Markdown artifact) · tokens **✓** (no color channel — label + emphasis carry the signal)
  - **webview run-report banner → NOT LANDED this chunk (deferred, see Deviations).** No `.tsx`/`.css`/`LAMP_META` change; zero UI source delta. Detectors should NOT expect a new webview element — and the GUI consequently carries no envelope signal, which is the recorded gap.

## Deviations from intent

1. **Four caller files edited that `research.md` did not list** — `conductor-cli/src/commands/{run,suite}.rs`, `conductor-cli/src/paths.rs`, `conductor-tauri/src/commands.rs`. *Justification:* the plan's own steps 5–7 (classify the run · persist it · render the CLI caption) are unimplementable without them; research enumerated the seam files but not the call graph threading the new run-level value. Judged in-scope helpers under the discipline's gray-area rule rather than soft-exiting, since the intent was unambiguous and only the enumeration was incomplete.
2. **`crates/conductor-cli/tests/cli_smoke.rs` fixture completed.** *Justification:* five smoke tests failed `could not read load envelope (NotFound)` — the fixture builds a temp repo-shaped root copying in exactly the contracts the CLI needs, and the CLI now needs a third. Completing the fixture was correct; defaulting a missing envelope would have been a silent downgrade (banned).
3. **The webview banner (plan step 7, third surface) was NOT implemented — deferred.** *Justification:* `v2-07`'s acceptance names *"its report"*, satisfied by the Markdown report; the webview would have pulled a new read command into `conductor-tauri/src/commands.rs` and shipped a UI change whose axe/contrast gate is display-gated to Linux+xvfb and cannot be verified on this host. A recorded deferral beat an unverifiable scope widening. **Consequence, stated plainly: a GUI user sees no envelope signal.** Needs an owner.
4. **`sut_version = "v0.3.0"`, not the `pulse-0.2.0` shown in the approved option preview.** *Justification:* the preview string was illustrative; the committed `contracts/pulse-capabilities.toml` says `v0.3.0` and consistency with the sibling manifest governs.
5. **Two `#[allow(clippy::too_many_arguments)]`** (`drive_run`, `run_thread`), each with a cited reason. *Justification:* the project's existing precedent for a contract-driven argument list is `RunRecord::measured`'s allow-with-reason; bundling would only rename the same moved values.

## Decisions & corrections

- **Operator decision (P4, recommended-first):** `v2-06` asserts the envelope via *artifact + duration gate + pinned exemptions* rather than extending `EmissionSpec` with a rate term — keeps the chunk out of Epoch 2's dispatcher territory and out of the scenario model.
- **Operator decision (P4, recommended-first):** `environment-suspect` is a **run-level qualifier**, not a sixth `ReportState`. Evidence that drove the recommendation: `ReportState` has 204 references across 16 files in 6 crates plus the webview `LAMP_META` mirror.
- **Research finding that reshaped the chunk:** the committed scenario model has **no rate or occurrence-count field**, so the envelope's storm-rate axis is not derivable; and a naive duration bound would have false-flagged `activity-floor` (3900s of which ~50 min are deliberate quiet). Hence the two-term artifact with only the duration term asserted, plus an exact-set exemption ledger.
- **Validation-1 amended `scope.md`** (classified *intent-incomplete*): "every scenario is asserted inside it" now reads "inside it **or** an exact-set pinned exemption carrying its reason".
- **`cargo audit` — FIFTH consecutive red**, byte-identical `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1. Unchanged advisory-DATABASE fault with nothing to raise a floor to. Remedy stayed the **bounded wait alone**, with `cargo deny check` **observed** green (advisories · bans · licenses · sources all ok, exit 0). No floor raise, no `deny.toml` ignore, no CI edit.
- **Derived-count class recurred on a THIRD document.** `design-system.md` bakes "two NON-lamp uses" of ANSI 246, which this chunk moves to three — but the two shipped derived-count detectors are scoped to `layout-templates` and `test-plan` only. Candidate for detector growth.

## Outcome

**Acceptance criteria met** for the surfaces landed; the webview criterion is explicitly deferred (Deviation 3).

Gates — all green, 4 fix iterations:
- `cargo nextest run -p conductor-core -p conductor-report -p conductor-run -p conductor-cli` → **306/306**
- `cargo nextest run --workspace --profile ci` → **494/494**, zero retries
- `cargo test --workspace --doc` → ok · `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `cargo deny check advisories bans licenses sources` → **all four ok** (exit 0, observed)
- `cargo audit` → **red, deferred** (fifth; advisory-DB parse fault — see Decisions)
- `npm --prefix crates/conductor-tauri/ui run a11y` → **skipped, recorded**: zero UI source delta (`ui/dist` byte-identical to its last green run, so it proves nothing new) and the harness is display-gated to Linux+xvfb. Never a silent pass.

Smoke (boot-path changed → fired): `bash scripts/agent-run.sh run` exit 0 · `… status` exit 0 and truthful. Beyond the harness verbs, the real `conductor.exe` was driven over a purpose-built over-envelope fixture:
- caption `[ENVIRONMENT-SUSPECT] scenario "over-envelope-probe" runs 900s, over the proven-good envelope ceiling of 600s (Pulse v0.3.0, captured 2026-08-09) — this run's read-back is not evidence about the SUT`, then `[BLOCKED] over-envelope-probe`, **exit 0**;
- report banner rendered above the tally; `run_envelope` row persisted; `runs` table still eleven columns; journal line still eleven fields with no qualifier leakage; check state `Blocked`, never `Fail`;
- **no false positives** — an in-envelope scenario and the 3900s *exempt* `activity-floor` both rendered no caption, which is the whole reason the exemption ledger exists.

Verification matrix: `v2-07` `status: implemented` with three named test refs. `v2-06` remains `chunk: null` (partially advanced — its storm-rate half is owed to Epoch 2's *Faithful emission dispatcher*).
