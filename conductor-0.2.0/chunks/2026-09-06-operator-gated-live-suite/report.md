# Report — 2026-09-06-operator-gated-live-suite

**Chunk:** Operator-gated live suite — one named re-runnable live-Pulse proof invocation composing the
scattered legs in a single correct shape and landing a durable evidence record, plus the three CARRY'd
live-path observables asserted from persisted records; never a CI gate.
**Date:** 2026-09-06
**Commits:** none yet — this wrap authors the chunk's first commit (HEAD `76decf7`, branch
`build/conductor-0.2.0`, 2 ahead of upstream).

## Changes (structured — detectors read this)

- **Files:** `scripts/agent-run.sh` (+87/−2) · `scripts/agent-run.ps1` (+94/−3) ·
  NEW `scenarios/auto-resolve-idle-window.toml` (33) · NEW `crates/conductor-run/tests/live_suite.rs` (165) ·
  NEW `crates/conductor-run/tests/live_suite_harvest.rs` (189) · NEW
  `conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/evidence/` (3 frozen self-obs captures +
  `live-suite-verdict.md`). Basis: `git status --short` + `git diff --stat scripts/` + `wc -l`.
- **Symbols / APIs:** no production symbol added, changed or removed — the entire Rust delta is two `tests/`
  targets. **Shell functions added** (`agent-run.sh`): `live_leg_budget_sec`, `live_leg`, `live_suite`;
  **module-scope vars** `LIVE_LOG`, `LIVE_CAPTURE_DIR`. PowerShell mirrors: `Get-LiveLegBudgetSec`,
  `Invoke-LiveLeg`, `Invoke-LiveSuite`, `$LiveLog`, `$LiveCaptureDir`. **No new port, socket or env var** —
  the suite READS the existing `ANDROMEDA_PULSE_*` handles and writes none; no `CONDUCTOR_*` handle was
  added (the capture dir is a fixed repo-relative path under the gitignored `runs/`). Existing callers of
  every touched surface are unchanged: the `run` verb's `--unit` / `--integration` / `--e2e` / bare arms
  keep their bodies verbatim (the new `--live` is a sibling `case` branch).
- **Crates / modules:** none added or removed. `crates/conductor-run` gains two test targets; its
  `Cargo.toml` is byte-unchanged — the existing `live-pulse = []` feature sufficed.
- **Dependencies:** none added, none bumped. `grep -c '^name = ' Cargo.lock` = **564**, unchanged;
  `git status --porcelain Cargo.lock` empty.
- **Schema / config:** one new scenario TOML (`auto-resolve-idle-window`, `p_ids = ["P-022"]`,
  `slo_tier = "<90s"`, one silent phase `gap_ms = 165000` / `occurrences = 0`, zero `[[expected]]` —
  declare-only). No migration, no config key, no violation-schema or scrub-shape change.
- **Spec-master edits:** none by this chunk (implement is read-only on the seven; P2 owns any).
- **Counts / qualifiers moved:** workspace test count **873 → 878** (+5, the new harvest target). Basis:
  `runs/gate-nextest.log` vs the prior wrap's handoff. Catalog scenario count grew by one
  (`scenarios/*.toml`). No documented literal of either is known to be baked in a master — the
  derived-count detectors should confirm against their own docs.
- **Dev-tool versions:** none.
- **Harness / gate surface:** the `run` verb gains a fifth stage flag **`--live`** in BOTH shells at
  identical semantics — the operator-gated live-Pulse suite. Usage strings and header comments updated in
  both. It composes: `conductor preconditions` (leading, non-priming, refuses non-zero on an unmet subject)
  → leg B1 → leg B2 → a 150 s quiet window → leg A → the driven a11y arm (`npm run a11y:driven`, preceded
  by `ensure_frontend` + `cargo build --release -p conductor-tauri --features tauri/custom-protocol`).
  Each leg's `logs/agent-latest.jsonl` is frozen to `runs/live-suite/{leg}.jsonl` before the next leg runs.
  **NEW SHELL DELETION PRIMITIVE — the harness's first recursive remove.** `scripts/agent-run.sh:100` is
  `rm -rf "$LIVE_CAPTURE_DIR"` (where `LIVE_CAPTURE_DIR="$RUNS_DIR/live-suite"`), and
  `scripts/agent-run.ps1:121` is `Remove-Item -Recurse -Force $LiveCaptureDir`. Basis:
  `grep -n 'rm -rf\|rm -r ' scripts/agent-run.sh` → 1 hit at :100;
  `git show HEAD:scripts/agent-run.sh | grep -nE 'rm -r'` → **0 hits at HEAD**, so this is the first in the
  file's history. `RUNS_DIR="${CONDUCTOR_RUNS_DIR:-runs}"` cannot expand empty (`:-` substitutes on empty as
  well as unset), so the path cannot degenerate to `/live-suite`; there is no `--` end-of-options guard and
  no non-empty assertion. The file's own established idiom for artifact removal is non-recursive and
  explicitly named (`cleanup`, `:146`: `rm -f "$RUNS_DIR/$id.jsonl" "$RUNS_DIR/$id.md"`). Flagged for the
  security detector; the operator's global policy denies agents `rm -rf`.
- **Cross-project / external claims:** the suite drives a live `andromeda-pulse` at HEAD `83d4060`
  (deterministic L4 + MCP enabled, operator-launched, fresh data dir). Facts read from that repo and
  relied on: Pulse's dedupe on the `(kind, scope, scope_id)` tuple and its 120 s idle +
  30 s-resolver-tick auto-resolve. Basis: the live run's measured behaviour (leg B2 deduped, leg A's
  canary resolved), not a re-read of Pulse's source this chunk.
- **Reverted / negative API facts:** the plan authorised editing `crates/conductor-run/Cargo.toml` "only if
  a second gate proves necessary" — it did not, so no manifest edit shipped. No `CONDUCTOR_*` handle was
  minted for the capture directory, deliberately: a new handle would need arch registration and the fixed
  repo-relative path under gitignored `runs/` needs none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  **`architecture.md:69` — "under deterministic L4 every read-back returns `degraded_mode`" is FALSE as a
  universal.** Measured on leg B1 of this chunk's live run: deterministic L4 active, `degraded-mode-report`
  read back an open incident, `route_read_back` → `Graded` → `manual_record` →
  `state_for(observation, ReportState::ManualCheck)`, and the row landed **`ManualCheck`**. `state_for`
  (`crates/conductor-run/src/execute.rs:233-235`) returns `KnownResidual` **iff** `observation.degraded`, so
  `ManualCheck` proves `degraded == false` on that read-back. Evidence:
  `evidence/b1.jsonl` + the persisted envelope in `evidence/live-suite-verdict.md`
  (`state "ManualCheck"`, `latency_ms 6045`, run `2026-09-06T09-11-09-325`). Basis for the site:
  `grep -nE 'every read-back returns .degraded|always.{0,20}degraded' .andromeda/*.md` → **1 hit,
  architecture.md only** (the other six masters: 0). The sentence's surrounding observation about
  `findings-counter-refresh` landing `KnownResidual` is untouched by this measurement — what is falsified is
  the UNIVERSAL quantifier, not that scenario's outcome. **Needs disposition.**
- **Expected amendments (from plan):**
  - `test-plan.md` §3 — **carried**; fact in *Harness / gate surface* (the `--live` stage flag in both
    shells). Search: `grep -c -- '--e2e' .andromeda/test-plan.md` → **5 hits**; the stage-flag set is stated
    at §3's command body (`:145`).
  - `test-plan.md` §9 — **carried**; fact in *Harness / gate surface* (the composed stage) and *Outcome*
    (the owed per-feature clippy command, run green). Search: `grep -n 'workflow_dispatch'
    .andromeda/test-plan.md` → **1 hit at `:464`**, the sanctioned live-leg SET.
  - `layout-templates.md` §Surface: cli — **carried**; fact in *Harness / gate surface* (the flag + its exit
    rule). Search: `grep -n -- '--e2e' .andromeda/layout-templates.md` → **1 hit at `:189`**.
  - `obs-plan.md` §4 — **carried**; fact in *Spec claims disproved* is adjacent but distinct, so stated
    here explicitly: **`degraded_mode_response` is named by obs-plan as a report-seam envelope extra and has
    no implementation.** Search: `grep -c 'degraded_mode_response'` → obs-plan **3 hits** (`:225` "Additional
    fields per scenario", `:347` required-log-fields context, `:442` the known-residual-path field list);
    **0 hits in each of the other six masters**; **0 hits anywhere under `crates/` at implement time** — the
    single hit there now is this chunk's own `live_suite_harvest.rs:62` doc comment *asserting the absence*,
    a self-referential mention, not an implementation.
  - `architecture.md` §Occupied Resources — Service/process names — **not carried** (it was conditional on
    the census naming an uninventoried process; it did not). The fact is stated anyway: the census names
    `tauri-driver` + the native WebDriver (arch `:148`), `andromeda-pulse-mcp` (`:158`) and
    `conductor-tauri`'s stream (`:169`); `msedgedriver` is inventoried by role and by the
    `CONDUCTOR_MSEDGEDRIVER` handle rather than by executable name. Search: per-name `grep -c` over
    architecture.md → tauri-driver 5, andromeda-pulse-mcp 2, conductor-tauri 14, msedgedriver 0.
- **Coverage of new surfaces:**
  - `agent-run {sh,ps1} run --live` (harness stage flag) → validation n/a (no external input; the leading
    `conductor preconditions` probe is the guard) · instrumentation ✓ (each leg runs `--agent-mode`, so the
    self-obs stream carries `run_id`; the suite echoes per-leg progress) · PII redacted✓ (refusal output
    measured host-path-free, `grep -cE 'C:\\|/c/|D:\\|/d/|%APPDATA%|/Users/|/home/'` → 0) · tests: the
    refusal path + both shells' fall-through exercised; the live path proven by a full green run · a11y n/a
    (no UI element) · tokens n/a (reuses the existing `[PRECONDITION]` caption and the closed lamp set; no
    new label, ANSI entry or `ReportState`).
  - `scenarios/auto-resolve-idle-window.toml` (catalog entry) → validation ✓ (garde + the load-path
    `check_*` methods, via the production reader) · instrumentation ✓ (drives the standard scenario spine)
    · PII n/a · tests ✓ (`cargo nextest run -p conductor-core -E 'test(committed)'`, 12 passing, incl.
    `every_committed_scenario_points_authors_at_the_envelope` and
    `the_committed_catalog_matches_the_committed_envelope`) · a11y n/a · tokens n/a.
  - `crates/conductor-run/tests/live_suite_harvest.rs` (graded evidence tier, default suite) → validation
    n/a · instrumentation n/a · PII redacted✓ (the pinned literals measured host-path-free and struct-name
    free) · tests ✓ (5 passing) · a11y n/a · tokens n/a.
  - `crates/conductor-run/tests/live_suite.rs` (capture tool, `--features live-pulse`) → validation n/a ·
    instrumentation n/a · PII redacted✓ · tests: **unrunnable-here by design in the default suite** — it is
    feature-gated off every default `nextest`/`clippy`/release path and ran green against the live legs ·
    a11y n/a · tokens n/a.

## Deviations from intent

1. **The `--live` arm freezes captures to `runs/live-suite/` (gitignored), not directly to the chunk's
   `evidence/`.** Plan step 7 said "copy `logs/agent-latest.jsonl` into the chunk's `evidence/`". Justified:
   `scripts/agent-run.{sh,ps1}` are generic, permanently-shipped harness files and must not hardcode a
   version-workspace path (`conductor-0.2.0/chunks/{marker}/`), which would rot at the next chunk and go
   stale at the next version; parameterising it would have required a new `CONDUCTOR_*` handle the plan did
   not authorise. The copy into `evidence/` is performed as step 14's own act instead. Same artifacts, same
   grading, no new handle, and the harness stays reusable by the next live chunk.
2. **Two in-scope fixes to this chunk's own new capture tool** (2 fix-loop iterations, no scope trigger).
   (a) It selected leg B1's journal by newest-matching-scenario — but B1 and B2 drive the SAME scenario by
   design, so newest-wins returned B2's `Blocked` row, which carries no `latency_ms`, for a question asked
   about B1; replaced with an exact link through the `run_id` each leg's own frozen capture carries. (b) It
   resolved `runs/` repo-relative while cargo runs a test with the CRATE dir as cwd; anchored to
   `CARGO_MANIFEST_DIR/../..` the way `load_envelope.rs`'s own tests do. Both surfaced as loud failures on
   the first capture rather than as a silently-wrong row.

## Decisions & corrections

- **P4 fork, operator-decided: the invocation shape** — a stage flag on `agent-run run` driving
  `live-pulse`-gated targets, chosen over feature-gated targets alone or a stage flag alone. A new
  `conductor-cli` verb and a sixth `agent-run` command were both rejected as outside test-plan §9's
  sanctioned live-leg SET / the 5-command discipline.
- **P4 fork, operator-decided: the per-run canary identity is DECLINED.** The quiet window stays the
  recorded shape. Basis added by research: the dedupe collision is itself the lever that produces
  observable 3's negative arm (a connected client with an unsatisfied gate), so salting `scope_id` would
  have removed the lever along with the hazard.
- **P5 review round 2, operator self-correction:** "every committed declare-only scenario reads back
  2–75 s after its canary" is false — `ack-cooldown` is 370 s of silent phases and reached the
  `AutoResolved` arm live on 2026-08-21 (`severity_harvest.rs:505-513`). Reachability is PER-SCENARIO. The
  new 165 s scenario is therefore chosen on wall clock and declared intent, never on reachability. I had
  propagated the false universal into three artifacts before it was caught; all three are corrected.
- **P5 review round 2:** `conductor coverage | grep P-022` proved nothing about the new file
  (`commands/coverage.rs:12-20` prints the static `coverage_matrix()` and never loads the catalog) —
  replaced with the catalog-reading unit tests, and the replacement was verified by running
  `cargo nextest list` on the filter rather than assumed.
- **P5 review round 3:** the hermetic scenario-load check was only hermetic in a shell without the live
  env; with it set, `canary_gate` reaches `emit_canary_storm` and the "check" becomes a priming storm.
  Wrapped in `env -u` like its sibling, with the comment stating that the wrap — not the code path — is
  what makes it non-priming.
- **Standing reading corrected by measurement:** see *Spec claims disproved* — `architecture.md:69`.

## Outcome

**Acceptance criteria, re-asserted against the diff:**

*Hermetic (proven without a live Pulse):*
- Default gates green with the live targets absent — `cargo nextest run --workspace --profile ci` **878/878**,
  `cargo test --workspace --doc` ok. ✓
- `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` exit 0. ✓
- The new scenario loads through the production reader; `cargo nextest run -p conductor-core -E 'test(committed)'`
  **12/12**. ✓
- `--live` is a stage flag on `run` (a sibling `case` branch, not a sixth command), present in both shells,
  reachable from no default path, and **no CI workflow gained a live stage** (`.github/workflows/` untouched
  — not in the diff). ✓
- An incomplete live-Pulse env REFUSES: exit **1**, `[PRECONDITION]` lines naming `sidecar-resolvable` and
  `handles-declared` with candidate causes, **0** host paths, and no leg fired (no capture dir created). ✓
- Host-tool handle absence skips at exit 0 rather than refusing — the two classes stay distinct (the driven
  arm's own wdio guard; unexercised this run because the handle was present). ✓ (by construction)
- No seventh lamp, no new `ReportState`, no new ANSI or token entry. ✓
- `grep -c '^name = ' Cargo.lock` = **564**, deliberate-bind set unchanged. ✓
- Boot-path smoke: `agent-run.{sh,ps1} run --nonsense-flag` exit **2** on both shells. ✓
- `cargo audit` exit 0 (18 allowed warnings) and `cargo deny check advisories bans licenses sources` all ok,
  with the advisory-db porcelain read FIRST (empty, HEAD `5a0ebedf`). ✓

*Live-leg (operator-gated, all met on one green run, exit 0):*
- Observable 1 — leg A: envelope `state "KnownResidual"` / `verdict null` / `fingerprints []`, plus the
  `declare-only read-back empty: no active incident outlived the emission window` line. ✓
- Observable 2 — leg B1: `latency_ms 6045` vs a 6000 ms instant difference, **skew 45 ms** inside the 1 s
  bound the second-precision instants impose. ✓
- Observable 3 — both arms: the gate line present in B2 only (b1 **0**, b2 **1**, a **0**). ✓
- Leg B2 exits 0 with a `[BLOCKED]` row — the expected outcome, not a failure. ✓
- Each leg's self-obs frozen before the next ran; three distinct per-leg records, not one truncated
  survivor. ✓
- `evidence/` greps clean: **0** absolute host paths, **0** internal seam-crate struct names across all
  three captures; only allowlisted `target` module paths. ✓
- The a11y leg ran over the existing tauri-driver + WebdriverIO session; no second automation stack in the
  census. ✓ — **1 passing in 54 s** on WebView2 152.0.4191.62.

**Gates run:** `cargo nextest run --workspace --profile ci` · `cargo test --workspace --doc` ·
`cargo clippy --workspace --all-targets -- -D warnings` ·
`cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` ·
`git -C $CARGO_HOME/advisory-db status --porcelain` → `cargo audit` → `cargo deny check …` ·
`grep -c '^name = ' Cargo.lock` · `cargo nextest run -p conductor-core -E 'test(committed)'` ·
the env-stripped hermetic scenario load · the env-stripped refusal path · both shells' fall-through ·
`bash scripts/agent-run.sh run --live` (live, exit 0).
**Smoke** (boot-path changed): `bash scripts/agent-run.sh run --unit` through the modified `case` → exit 0,
878/878; plus mint-then-read `agent-run.sh status 2026-09-06T09-16-47-440` returning leg A's exact envelope
rather than newest-residue.

**Light-gate re-run (wrap, post-implement — a second live run at the operator's direction):** all commands
green, suite exit 0. Observables 2 and 3 reproduced exactly (B1 `[MANUAL]`, B2 `[BLOCKED]`, gate line in B2's
capture only; a11y arm 1 passing in 54.1 s). **Observable 1 did NOT reproduce:** leg A landed `[MANUAL]` with
`latency_ms 165055` and 3 fingerprints (run `2026-09-06T10-07-08-396`) — the canary incident was still active
at read-back. Cause is the margin, not the mechanism: 165 s against Pulse's 150 s worst case (120 s idle + up
to a full 30 s tick) left 15 s. `scenarios/auto-resolve-idle-window.toml` is widened to `gap_ms = 200000`
(50 s margin) in this wrap; **the widened value is not yet live-proven**. The harvest's pinned literals stay
valid as the record of what the arm looks like when reached; observable 1 is proven ONCE, on a window now
known to have been marginal. Catalog gates + full suite re-run green over the widened scenario (12/12, 878/878).

**Outcome basis:** implement's P4 report as given, plus this wrap's own re-derivations — the
`architecture.md:69` site (implement reported the measurement without locating its site; this report locates
it), the Expected-amendment site searches, and the `rm -rf` history check
(`git show HEAD:scripts/agent-run.sh`). The operator's wrap directive supplied items 1–5; item 2's premise
("0 hits over the seven masters") was re-derived here and found to be **1 hit** — see *Spec claims disproved*.

**Process hygiene** (implement P4's census, re-measured at this wrap — the host list is readable here):

| process | started by | final state |
|---|---|---|
| `pulse-app.exe` (pid 64156) | the operator, for this run | **left running** — the SUT; the operator stops it after the commit. A leg never kills what it did not start. |
| `tauri-driver` · `msedgedriver.exe` · `conductor-tauri.exe` · `node.exe` ×N | the driven a11y arm | terminated — wdio `onComplete` → `tauriDriver.kill()` |
| `andromeda-pulse-mcp` | each leg's preflight | terminated with its leg's `conductor` process |

Post-leg listing matched the pre-leg baseline exactly (only `pulse-app.exe`, holding `:4317`); no listener
on `:4444`/`:4445`. Nothing this chunk started was orphaned.
