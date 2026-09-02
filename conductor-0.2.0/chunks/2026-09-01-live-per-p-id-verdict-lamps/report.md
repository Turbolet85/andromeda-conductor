# Report — 2026-09-01-live-per-p-id-verdict-lamps

**Chunk:** Live per-P-ID verdict lamps — coverage rows carrying each capability's actual verdict state with not-yet-run visibly distinct from failed, plus the CARRY the desktop surface never got: a run-level load-envelope banner outside the lamp column
**Date:** 2026-09-02T05:41Z
**Commits:** (none since last_wrap — this chunk's work is uncommitted; wrap commits it)

## Changes (structured — detectors read this)

- **Files:**
  - modified: `crates/conductor-run/src/lib.rs` · `crates/conductor-tauri/src/commands.rs` ·
    `crates/conductor-tauri/src/main.rs` · `crates/conductor-tauri/ui/src/App.tsx` ·
    `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` ·
    `crates/conductor-tauri/ui/src/components/RunReport.tsx` ·
    `crates/conductor-tauri/ui/src/components/RunReport.css` ·
    `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` · `crates/conductor-tauri/ui/wdio.conf.ts`
  - new: `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` · `crates/conductor-run/tests/lamps_fixture.rs`
  - untouched-but-verified: `crates/conductor-tauri/ui/src/Gallery.tsx` (research named it a caller of BOTH
    changed components; the optional-prop choice kept it unmodified — a required prop would have broken its
    `tsc --noEmit`)

- **Symbols / APIs:**
  - NEW `conductor_run::read_envelope(runs_dir: &Path, run_id: &str) -> anyhow::Result<Option<EnvelopeStatus>>`
    — pub, the read-side counterpart to `persist`. Delegates to the shipped `RunsDb::get_envelope`
    (`conductor-report/src/db.rs:222`), which had **0 callers** before this chunk and now has exactly one.
  - NEW `#[tauri::command] conductor_tauri::commands::run_envelope(run_id: Option<String>)` — the **eighth**
    IPC command; registered in `main.rs`'s `generate_handler!` and in the mock-runtime test list.
  - NEW `conductor_tauri::commands::EnvelopeStanding { label, cause, suspect }` (`Serialize`) +
    `impl From<EnvelopeStatus>` — the command's return projection; `label` is carried from
    `EnvelopeStatus::label()`, never re-spelled in TypeScript.
  - NEW exported TS interface `EnvelopeStanding` in `RunReport.tsx`.
  - CHANGED `RunReport` gains an **optional** `envelope?: EnvelopeStanding | null` prop — **keeps both
    existing callers** (`App.tsx:267`, `Gallery.tsx:212`); optionality is what makes that true.
  - CHANGED `CoverageMatrix` — **no signature change**; its already-declared `lamps` prop is now SUPPLIED at
    `App.tsx` (previously only `Gallery.tsx` passed it). Both callers remain.
  - NEW module-private `LAMP_SEVERITY` + `lampsByPId()` in `App.tsx`.
  - **Ports / sockets:** none added. The dev-only `4444`/`4445` driver pair is pre-existing and unchanged.
  - **Env vars:** none added. `CONDUCTOR_RUNS_DIR` (existing handle) gains a new WRITER — `wdio.conf.ts` sets
    it on the tauri-driver spawn so the app reads the seeded fixture dir.

- **Crates / modules:** none added, removed or re-scoped. `conductor-run` gains one integration-test target
  (`lamps_fixture`). The `conductor-tauri → conductor-report` edge was deliberately NOT created (see
  Reverted / negative API facts).

- **Dependencies:** **none added or bumped** — cargo or npm. `Cargo.lock` byte-un-drifted (verified);
  `package-lock.json` untouched; `assert_fs` was already a `conductor-run` dev-dependency.

- **Schema / config:** no migration, no new table, no new column. The pre-existing `run_envelope` table
  (`(run_id, classification, cause)`) gains its first read path. No violation/scrub/redaction shape changed.

- **Spec-master edits:** none (implement authors none; P2 owns them).

- **Counts / qualifiers moved:**
  - **Tauri command handlers 7 → 8.** `obs-plan.md` §1 (line 49) bakes the literal "all **SEVEN** handlers"
    plus their full enumeration, measured 2026-09-01. `run_envelope` is the eighth.
  - **Routine `--e2e` spec results 6 passing / 2 skipped → 8 passing / 3 skipped.** Any doc baking the
    prior tally is stale.
  - **a11y contrast pairs asserted 11 → 13** (`PAIRS` 9 + `EXTRA_PAIRS` 2 → 4). `a11y-plan.md` §6's pair
    TABLE still enumerates no `--status-residual` row, so the doc and the harness now differ in reach.

- **Dev-tool versions:** none installed or upgraded.

- **Harness / gate surface:**
  - `wdio.conf.ts` now SEEDS its own subject before the session: `seedFixtureRuns()` copies the committed
    `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` into the gitignored `runs/e2e-fixture/` as
    `lamps-fixture.jsonl`, and the tauri-driver spawn carries `CONDUCTOR_RUNS_DIR=runs/e2e-fixture`
    (repo-relative — `resolve_under` rejects absolute handles).
  - The routine spec `accessibility.e2e.ts` gains 3 assertions (2 asserting, 1 context-skipping).
  - No change to `scripts/agent-run.{sh,ps1}`, the 5-command surface, or any status/verdict shape.

- **Cross-project / external claims:** none. No Pulse process was started; the routine arm needs no live SUT.

- **Reverted / negative API facts:**
  - A `db.read_envelope` **tracing span was deliberately NOT created.** obs-plan's bounded span-name set has
    no `db.*` wildcard (only `timeline.execute*`, `verify.readback*`, `tauri.command.*`), so minting one
    would widen a spec enumeration implement may not author; and the in-repo DB-span convention is a
    `#[tracing::instrument(name = "db.insert_run")]` ATTRIBUTE on the `RunsDb` method in `conductor-report`,
    outside this chunk's modify-set. The read logs an obs-plan §6 Boundary-call-wrapper `info!` line inside
    the `tauri.command.run_envelope` span instead.
  - A `conductor-tauri → conductor-report` dependency edge was considered and rejected (plan §Constraints);
    the projection routes through `conductor-run`, which already holds that edge.
  - `capabilities/default.json` was deliberately NOT edited — app-defined `#[tauri::command]`s are not
    ACL-gated (`.claude/rules/security.md` §Session Additions 2026-06-26; arch's 2026-06-24 dismissal).

- **Spec claims disproved by measurement:**
  - `obs-plan.md` §1 line 49 **contradicts itself in one sentence**: it opens by stating the
    `#[tracing::instrument]` attribute does not stack with `#[tauri::command]` (correct, measured 2026-09-01)
    and closes with "instrument via `#[tracing::instrument]`". The trailing clause is the stale half.
  - `layout-templates.md` §Component — Primary content block 1 documents the desktop coverage row as
    **`lamp glyph · scenario label · P-ID · Mode · slo_tier · latency_ms`**; the shipped
    `CoverageMatrix.tsx` renders **`P-ID · Capability · Mode · Status`** — four cells, lamp LAST rather
    than first, no `slo_tier` and no `latency_ms` column. **Pre-existing, not introduced here** (this
    chunk fills the existing Status cell and adds no column), surfaced because the chunk is the first to
    populate that cell. Added to this report after the fan-out: the detectors read the report alone, and
    its omission is why no layout-templates proposal covered the chunk plan's expected amendment for it.
  - **Self-obs sink landing site is now per-arm** (found by the arch detector, then measured on disk):
    `tauri_log_path()` resolves the sink as `runs_dir.parent()/logs`, so the routine `--e2e` arm's
    `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` moves `conductor-tauri.jsonl` from the workspace root to
    `runs/logs/`. Evidence: `runs/logs/conductor-tauri.jsonl` 4741 B written at the leg's own instant
    (matching `runs/e2e-fixture/runs.db`) while root `logs/conductor-tauri.jsonl` stayed stale.
  - No other spec assertion was measured false this chunk.

- **Coverage of new surfaces:**
  - `run_envelope` (`#[tauri::command]`, IPC boundary) → validation `resolve_under` traversal guard ✓ (garde
    n/a — not scenario config) · instrumentation `tauri.command.run_envelope` span + `latency_ms` info line ✓
    · PII `sanitize_error` at the edge ✓ · tests integ ✓ (`run_envelope_command_returns_null_when_no_run_has_an_envelope`)
    · a11y n/a · tokens n/a
  - `conductor_run::read_envelope` (rusqlite read boundary) → validation n/a (internal) · instrumentation
    Boundary-call-wrapper `info!` ✓, **no span** (see Reverted / negative API facts) · PII no path in the
    logged value ✓ · tests unit ✓ (`read_envelope_round_trips_what_persist_wrote_and_is_none_for_an_unknown_run`)
    · a11y n/a · tokens n/a
  - Coverage-matrix verdict lamps (UI element) → validation n/a · instrumentation n/a · PII n/a · tests e2e ✓
    (2 assertions, live WebView2) · a11y ✓ (label + `aria-hidden` glyph + closed-six asserted; contrast
    asserted in both declared themes) · tokens ✓ (existing `LAMP_META` tokens, no new value)
  - Run-report load-envelope banner (UI element) → validation n/a · instrumentation n/a · PII the label/cause
    cross the `sanitize_error` edge ✓ · tests **`unrunnable-here`** — the routine arm's seeded journal records
    no `run_envelope` row, so the banner's DOM state cannot be produced there; the spec context-skips (never
    passes green) and the populated arm is proven by the Rust round-trip + the IPC test, never a rendered DOM
    · a11y contrast ✓ (`--status-residual` over `--color-raised-1`/`--color-base` asserted at 4.5:1 in both
    declared themes — render-independent), rendered-DOM axe **`unrunnable-here`** for the same reason ·
    tokens ✓ (`--status-residual`, `--motion-micro`, `--ease-quiet`, spacing/radius by name; no hex, no px)

## Deviations from intent

1. **Plan step 1's `tracing::info_span!` became an obs-plan §6 Boundary-call-wrapper `info!` line.** The
   directive was unsatisfiable as written: a `db.*` read span widens obs-plan's bounded span-name set (a spec
   enumeration implement may not author), and the in-repo DB-span convention lives on the `RunsDb` method in
   `conductor-report`, outside the modify-set. The read is still observable, inside the
   `tauri.command.run_envelope` span. **Raised as an obs-plan escalation for this wrap to decide.**
2. **`LAMP_SEVERITY` lives in `App.tsx`, not beside `LAMP_ORDER` in `lamp.ts`.** The plan puts the join in
   `App.tsx`; `lamp.ts` is outside the modify-set, so placing it there would have been an out-of-scope edit.
   Worth revisiting when a second consumer needs the ordering.
3. **Coverage reach, stated as a limit rather than claimed as proof** (not a plan deviation — the plan
   sanctioned the context-skip — but the acceptance's reach is narrower than its prose reads):
   the e2e proves the collision rule at **one pair** (`Blocked` over `Pass`), not the full six-level order,
   which stands by construction in `LAMP_SEVERITY`; and the not-yet-run cell is asserted by its **text**,
   its binding to `--text-tertiary` resting on shipped CSS plus the contrast pairs rather than a
   computed-style assertion.

## Decisions & corrections

- **Operator ruling (P5, phase):** the P-ID collision rule is **worst-lamp-wins** under
  `Fail > Blocked > Hold > Manual > Residual > Pass`, chosen over latest-record-wins because the coverage
  matrix is the definition-of-done surface and a latest-wins rule would render `Pass` on a row whose sibling
  scenario was `Blocked`. Reachable, not theoretical: 5 of the 45 catalog P-IDs are named by 2–3 scenarios.
- **Operator ruling (P5, phase):** the routine arm gets a **harness-seeded fixture** subject, after the
  premise-check found `/runs/` is gitignored — on a clean tree every row reads "Not yet run", and on this dev
  host the assertion would have passed off ~60 uncommitted local journals. Two alternatives declined: driving
  a real GUI run (its runtime depends on whether `andromeda-pulse-mcp` resolves on PATH — ~0s vs minutes,
  against a zero-retry budget) and asserting only the not-yet-run half (leaves the headline capability
  unproven).
- **Operator directive (implement):** the fixture's RECORD CONTENT must be a **committed file** copied in by
  the seed, plus **one Rust test** round-tripping it through the production reader. Rationale, verbatim: an
  inline journal in `wdio.conf.ts` is an artifact nobody reviews, and a fixture that still PARSES after a
  persist-side change but no longer MEANS what it did would pass the e2e silently — a parse failure is loud,
  a semantic drift is not. Landed; one precision applied — the record content is journal-borne, so the pin
  round-trips through `conductor_core::read_run_journal` (what `run_report` single-sources), while the new
  envelope accessor keeps its own round-trip.
- **Operator convention (implement, "from now on"):** **every leg that boots an external process ends with a
  real process census.** Under remote operation the operator cannot see the process list, so a leg exiting 0
  is not evidence it left nothing running — only a census is. A leg that boots an external process names its
  **stop form** beside its **firing form**.
- **Measured this chunk (census, `tasklist` before and after the `--e2e` leg):** baseline clean → post-leg
  clean. `tauri-driver`, `msedgedriver.exe`, `conductor-tauri.exe` and 4 `node.exe` all **terminated** by
  wdio's `onComplete` → `tauriDriver.kill()`; no Pulse process was ever started. Ports 4444/4445 held only
  kernel `TIME_WAIT` client sockets (PID 0, no LISTENING socket). Firing form
  `bash scripts/agent-run.sh run --e2e` (guarded by `CONDUCTOR_MSEDGEDRIVER`; unset ⇒ skip at exit 0);
  stop form self-terminating via `onComplete`, and on an interrupted run
  `taskkill /F /IM msedgedriver.exe /IM conductor-tauri.exe` plus the node CLI.

## Outcome

**Acceptance criteria met.** All gates green, exits read direct (never through a pipe):

- `cargo nextest run --workspace --profile ci` → **763/763 passed**
- `cargo test -p conductor-tauri` → 13 · `cargo test -p conductor-run` → 30 · `cargo test --workspace --doc`
  → green (the runner-portability gate: green under BOTH runners)
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `npm run typecheck` · `typecheck:e2e` · `build` → exit 0 · `npm audit --omit=dev` → 0 vulnerabilities
- **`bash scripts/agent-run.sh run --e2e` → exit 0 · 8 passing, 3 skipped** against a live WebView2
  151.0.4129.107 session on the release bundle. Both new lamp assertions passed: `P-019` rendered **Blocked**,
  beating its sibling record's `Pass`.
- **PREREQ-1 (rust gate deferral) closed by execution** — this chunk carries a real `.rs` delta, so the
  deferred gates ran on their own merits; the zero-delta premise was re-verified at fold, not echoed (one
  nuance found: `ece1d24` did touch `ui/package.json`, but as a script line with `package-lock.json`
  untouched, so the cargo-tree rationale stands).
- **PREREQ-2 (43rd consecutive `cargo audit` re-check, COMPACT form) discharged, signature byte-identical:**
  `cargo audit` true exit **1**, first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`;
  `cargo deny check advisories bans licenses sources` true exit **0**; both captured BEFORE any pipe;
  `Cargo.lock` un-drifted. No deviation → the bounded wait continues; no floor raise, no `deny.toml` ignore,
  no CI edit.

**Smoke:** the UI self-verify is listed in the plan's Test Commands, so it ran as a P2 gate and P3 recorded it
rather than re-running — a genuine boot (a real WebView2 session attached to the release binary, mounted
`#root`, and served the new command's data), stronger than a CLI status read.

**Matrix:** `v2-30` `status: implemented` with 5 refs (3 e2e + 2 Rust pin).
