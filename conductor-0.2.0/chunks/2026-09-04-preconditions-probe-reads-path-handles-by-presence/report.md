# Report — 2026-09-04-preconditions-probe-reads-path-handles-by-presence

**Chunk:** Preconditions probe reads path handles by presence — `handles-declared` graded by the kind of value each handle carries, so `conductor preconditions` can exit 0 and `agent-run boot` reaches the preflight in both shells for the first time since `480bc66`
**Date:** 2026-09-04T17:15Z
**Commits:** none since `last_wrap` beyond `a981a97 feat(2026-09-04-sr-findings-remediation)` (this chunk is uncommitted at authoring time)

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-core/src/preconditions.rs` · `crates/conductor-core/src/lib.rs` · `crates/conductor-run/src/lib.rs`
- **Symbols / APIs:**
  - **NEW public (conductor-core):** `flag_declared(value: Option<&str>) -> bool` — the affirmative-truthiness rule, taking a value and NO name; `handle_declared(name: &str, value: Option<&str>) -> bool` — per-handle dispatch (the PATH-valued handle by presence-after-trim, every other name delegating to `flag_declared`). Both pure: no env read, no filesystem, no socket. Re-exported from `conductor_core` (`lib.rs`).
  - **NEW private const (conductor-core):** `PATH_VALUED_HANDLE = "ANDROMEDA_PULSE_DATA_DIR"`, pinned as a member of `OBSERVED_HANDLES` by a drift-guard test.
  - **CHANGED (conductor-run, crate-private):** `declares(name)` — reduced to a thin env-reading wrapper delegating its value test to `flag_declared`. **Behaviour is unchanged**; the truthiness rule moved, it did not weaken. **Remaining-caller fact:** its production callers went **2 → 1** — `observe_run_contract` (`lib.rs:351`) still calls it; `observe_preconditions` no longer does. It is NOT a sole-caller-by-default claim: the one caller was counted after the change (plus one test caller at `:1675`).
  - **CHANGED (conductor-run, public):** `observe_preconditions()` — the `declared` set is now built per handle, reading each value and grading it through `handle_declared` (`lib.rs:383`). Signature, return type and the `PreconditionObservation.declared: BTreeSet<String>` seam are byte-unchanged.
  - **Unchanged by design:** `Preconditions::evaluate`, `PreconditionObservation`, `PreconditionSubject`, `UnmetPrecondition`, `PreconditionsStatus`, `OBSERVED_HANDLES` (membership and type), `observe_run_contract`.
  - **No** new IPC method, endpoint, port, socket, or env var. The three `ANDROMEDA_PULSE_*` handles are pre-existing; only how each is GRADED changed.
- **Crates / modules:** none added, none removed. Changed: `conductor-core` (`preconditions`), `conductor-run` (lib root).
- **Dependencies:** **none added, none bumped.** No `Cargo.toml` and no `Cargo.lock` delta (verified by `git status`); package count **564**, unchanged.
- **Schema / config:** none. No migration, no config key, no violation schema, no scrub/redaction shape change.
- **Spec-master edits:** none — this report is authored before P2; every spec change is P2's to apply.
- **Counts / qualifiers moved:**
  - **`conductor-run`'s `declares` accepted-deliberate mutation roster: 4 members → 1.** Stated in `test-plan.md` §12 (`:608`) as the enumerated set `lib.rs:358:5` (→ `false`), `:360:11`, `:360:21`, `:360:26`. Measured after this chunk: **`lib.rs:362:5` (→ `false`) alone**. The three operator arms (`==`, `||`, `==`) relocated into `conductor_core::flag_declared` and are CAUGHT there; `declares -> true` is now caught by `declares_rejects_a_name_this_environment_does_not_declare`.
  - **`conductor-run`'s total accepted-deliberate set: 8 → 5**, and **all three surviving classes' line numbers moved**. `test-plan.md` §12 (`:610`) states class B as `lib.rs:518:27` + `lib.rs:559:25` ×2 and class C as `lib.rs:66:8`. Measured now: class B = `lib.rs:559:27` + `lib.rs:600:25` ×2, class C = `lib.rs:68:8`. Identity confirmed by function and COLUMN (27, 25×2, 8 all preserved), not by line number.
  - **`conductor-core/src/preconditions.rs` mutation tier: first run ever, and it ends at 0 missed.** 28 mutants → 27 caught / 1 unviable / **0 missed**. No prior figure exists in any doc.
- **Dev-tool versions:** none — no external CLI tool installed or upgraded.
- **Harness / gate surface:** **`agent-run boot`'s observable behaviour changed, with no edit to either script.** Its leading `conductor preconditions` arm can now exit 0, so `boot` proceeds to `conductor preflight --json` and emits `ReadyState` JSON. Measured live in BOTH shells (below). `scripts/agent-run.sh` and `scripts/agent-run.ps1` are byte-unchanged; the change is upstream in the probe. No new harness verb, no status/verdict shape change, no CI step.
- **Cross-project / external claims:** the live legs ran against **andromeda-pulse** (external repo). `pulse-app` was relaunched by the operator at 2026-09-04T17:00Z with deterministic-L4 + MCP enabled, data dir `…\pulse-legs\a11y-20260904-190050`; the sidecar was taken from that repo's `target/release` on `PATH`. Basis read: Conductor's own probe output and the two `boot` legs' `ReadyState` JSON — no claim is made about Pulse's internals beyond what those surfaced (protocol `2024-11-05`, four tools present, `canary_round_trip: ok`).
- **Reverted / negative API facts:** none in shipped code. (A first-draft DESIGN — freezing `declares()` and duplicating the truthiness rule in `conductor-core` — was rejected at the P5 review before any code was written; see Decisions.)
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none. This chunk disproves nothing; it makes a set of already-recorded defect statements stale in the OTHER direction (the defect they record is now fixed) — dispositioned under Expected amendments below.
- **Expected amendments (from plan):**
  1. `architecture.md` §Standard Contracts (Liveness equivalent) — **carried.** Motivating fact: `handles-declared` is satisfiable (probe exit 0 measured 17:07Z) and the `boot` skip is no longer unconditional (both shells emitted `ReadyState`). The section currently states the subject "is currently UNSATISFIABLE" and the skip "is UNCONDITIONAL as shipped".
  2. `architecture.md` §Occupied Resources (`CONDUCTOR_PREFLIGHT_TIMEOUT`) — **carried.** Motivating fact: the timeout was reached and paid on both legs (47s and 47s wall-clock, warm-up 45s + poll). The entry states it "has been unreached through `boot` on every run since `480bc66`".
  3. `architecture.md` §Occupied Resources (`ANDROMEDA_PULSE_MCP_ENABLED`) — **carried.** Motivating fact: the probe's CHECK is no longer uniformly value-gated; the path handle is presence-graded. The entry states "of the three subjects only the flag-valued … can ever be met".
  4. `security-plan.md` §Security Anti-Patterns → Universal + §Input Validation (declaration-only READ SET row) — **carried.** Motivating fact: the fix landed, and the non-widening witness holds (`DATA_DIR=false` declares; `MCP_ENABLED=false` does not). The withdrawn "not a downgrade" bullet named this chunk as its owner.
  5. `test-plan.md` §3 `boot` + §1 harness `boot` + the five §6 step-1 lines — **carried.** Motivating fact: `boot` reaches the preflight in both shells; §6's step-1 lines were re-pointed at a direct `conductor preflight --json` because `boot` could not be used.
  6. `test-plan.md` §12 (`:608`, and `:610`'s line numbers) — **carried.** Motivating fact: the Counts bullet above (roster 4 → 1; classes B/C line numbers moved; the enumerated SET is the operative form).
  7. `layout-templates.md` §Surface: cli — Primary screens — **carried.** Motivating fact: the `[PRECONDITION]` satisfied arm is now a REACHED state — `[PRECONDITION] every live-Pulse precondition is satisfied` printed at 17:07Z — where the doc records it as "intent, not a reachable state"; and the `conductor preflight` screen is reachable through `boot` again.
  8. `obs-plan.md` — **not carried.** The condition was a new span/attribute or a self-obs line lacking `run_id`; this chunk adds neither (the probe's existing `info!`/`warn!` inherit `run_id` from `main.rs`'s `init_observability`). Fact stated here anyway.
  9. `a11y-plan.md` — **not carried.** The `conductor` verb SET is already recorded not-assertable, and this chunk adds no UI surface. Fact stated here anyway.
- **Coverage of new surfaces:**
  - `conductor_core::flag_declared` → validation n/a (pure predicate over an already-read value; the boundary read is the caller's) · instrumentation n/a (no I/O, no boundary) · PII n/a (receives a value, never renders or logs it) · tests **unit ✓** (12 `#[rstest]` cases: `true`/`1`/`TRUE`/`True`/` 1 ` accept; absent/empty/whitespace/`false`/`0`/`yes`/path-like reject) · a11y n/a · tokens n/a
  - `conductor_core::handle_declared` → validation n/a · instrumentation n/a · PII n/a · tests **unit ✓** (8 path-handle cases + a 3-name × 11-value delegation-equality assertion + the swap-detector pair + the `OBSERVED_HANDLES` membership drift guard) · a11y n/a · tokens n/a
  - `conductor_run::observe_preconditions` (changed) → validation n/a (reads only fixed, code-owned handle names) · instrumentation **✓** (existing `info!`/`warn!` boundary lines inside the caller's span, `run_id` present, no new span name, no attribute outside `ALLOWLISTED_FIELDS`) · PII **redacted ✓** (handle NAMES only — measured: a distinctive data-dir value appears **0** times across the probe, `--json`, and both live `boot` artifacts; the gate's own `data_dir` renders `<redacted>`) · tests **unit ✓ + live ✓** · a11y n/a (`conductor` verb SET is not-assertable, a11y-plan §1) · tokens n/a
  - `agent-run boot` (behaviour, no script edit) → validation n/a · instrumentation n/a · PII **redacted ✓** · tests **live ✓ both shells** · a11y n/a · tokens n/a

## Deviations from intent

1. **A second `conductor-core` mutation run used `target/mutants-core-2026-09-04b`**, not the plan's literal `target/mutants-core-2026-09-04`. *Justification:* the first run surfaced a survivor that was killed in-chunk; `.claude/rules/testing.md` (2026-09-03) requires a FRESH per-run output directory because a reused one is stale by construction. The plan listed one run per file and did not anticipate a re-run.
2. **A backgrounded `rm -rf … && cargo mutants …` compound was permission-denied**; re-issued as granular per-command runs with no `rm`. *Justification:* `host-win32.md` prescribes exactly this (rm+launch compounds get denied and abort mid-chain); no plan step was skipped.
3. **One survivor was killed beyond the plan's steps** — `PreconditionsStatus::is_unmet` (constant-`true` mutant). *Justification:* in-scope (same file, in the modify-set) and OWED by test-plan §10, which requires every named survivor of a run the chunk performs to end killed or classified. Pre-existing, surfaced only because this file's tier had never been run.
4. **The `boot` witness was initially deferred, then closed within the session** on the operator's post-implement directive once `pulse-app` was relaunched. *Justification:* the plan pre-sanctioned the deferral shape; the directive withdrew it. No criterion is left unmeasured.

## Decisions & corrections

- **Operator, P4 fork:** the per-handle KIND lives in `conductor-core` beside `OBSERVED_HANDLES` (not at the `conductor-run` observation site). Reason: `OBSERVED_HANDLES` is pinned there by exact-set equality, so a handle added to the list would otherwise be graded by a fallback in another crate that no gate sees.
- **Operator, P5 review — three notes, all verified at HEAD before applying:**
  1. **The plan contradicted itself.** Step 5 froze `declares()` while step 8 predicted its mutation roster would shrink — but all four accepted mutants live inside that function (columns verified: `=`@11, `|`@21, `=`@26 on `v == "true" || v == "1"`). Neither the mechanical checks nor validation-1 has a predicate for step-vs-step consistency. Resolved by delegating rather than freezing.
  2. **Test Commands wrote into the repo root** (`audit.txt`, `mutants-*/`). `git check-ignore` confirmed only `mutants.out/` is ignored by name; moved to `target/` and added `--jobs 2`.
  3. **The live leg's data dir was wrong.** A Conductor-side per-leg fresh dir would break data-dir equality (the sidecar must read the corpus `pulse-app` writes) and empty every read-back; inter-leg hygiene is the quiet window instead, since a fresh dir needs a `pulse-app` restart.
- **My refinement beyond the operator's note 1:** TWO core predicates rather than one. `declares` delegates to the value-only `flag_declared`, never to the name-keyed `handle_declared`, so the run-contract path cannot reach the presence arm even if a future contract term named a path handle — the boundary holds by construction rather than by discipline.
- **One correction to the operator's note 2:** the prior-chunk `target/` precedent is a single chunk (`target/mutants-2026-09-03`), not all of them; two others used `{run_dir}/…` and `mutants.out`. The `target/` conclusion stands regardless.
- **A friction-ledger record was retracted.** The deferral record said the `ReadyState` witness was "unprovable on this host"; accurate at its timestamp, but it reads as a durable host property when the true scope was "unprovable while the operator's SUT is down". Left standing it would have misled a later diagnosis.
- **A rejected approach was declined under pressure, not adopted:** a stand-in `:4317` listener would have closed the witness hermetically; the plan's Constraints forbid it (testing.md bans binding `:4317` in a test, and a faked sidecar would be spawned by the preflight that follows). No substitute was attempted.

## Outcome

Every acceptance criterion re-asserted against the DIFF:

- **(arch) `handles-declared` satisfiable** — **MET.** `conductor preconditions` exit **0**, printing `[PRECONDITION] every live-Pulse precondition is satisfied` (2026-09-04T17:07Z, full operator env). Structurally impossible before this chunk.
- **(arch) probe still reports a finding + non-zero exit, never `Err`/verdict; connect-and-drop only** — **MET.** Bare-host run named all three subjects at exit 1; no `Result::Err`, no `Verdict`/`ReportState`, no bind, `:4318` untouched (diff introduces no socket).
- **(security) run-contract grading behaviourally unchanged; non-widening witness** — **MET.** `ANDROMEDA_PULSE_MCP_ENABLED=false` leaves `handles-declared` naming that handle alone; `ANDROMEDA_PULSE_DATA_DIR=false` declares. `declares` routes to `flag_declared` (value-only), so no run-contract term can reach the presence arm.
- **(security) handle NAME only, never its value** — **MET.** A distinctive data-dir value appears **0** times in the human arm, the `--json` arm, and both live `boot` artifacts; the gate's `data_dir` renders `<redacted>` against a real live path.
- **(tests) new test exercises READ → GRADE; green under both runners; no `unsafe { set_var }`** — **MET.** `observe_preconditions_grades_each_handle_from_its_own_environment` + `declares_rejects_a_name_this_environment_does_not_declare`. `cargo nextest` and `cargo test` both green on both crates. The diff contains no `unsafe`.
- **(tests) mutation tier re-run, roster re-derived as an enumerated SET** — **MET.** See Counts bullet. `conductor-run` 95 mutants (67 caught / 5 missed / 23 unviable), `conductor-core/preconditions.rs` 28 (27 caught / 0 missed / 1 unviable).
- **(tests) both shells' `boot` emits `ReadyState` JSON** — **MET.** `bash` 17:07:17→17:08:04Z exit 0; `pwsh` 17:11:16→17:12:03Z exit 0. Both `ready: true`, `canary_round_trip: "ok"`, `blocked_precondition: null`, 4/4 tools `present`, `data_dir: "<redacted>"`. Negative control: **0** `skipped preflight` lines in either.
- **(layouts) three subjects, no `Verdict`/`ReportState`/per-P-ID row; `--json` shape unchanged; no new label/ANSI/lamp** — **MET.** `--json` payload is the same four-key shape (`satisfied`, `unmet[]` of `{subject, statement, causes}`, `checked_at`); the diff adds no render code.
- **(design) satisfied arm omits the caption and renders its ASCII label under `NO_COLOR`; unmet is a named-precondition state, never red Fail** — **MET.** The satisfied line printed for the first time; unmet lines carry `[PRECONDITION]` + statement + causes.
- **(obs) no host path in any log/attribute/payload; no span outside the bounded set; no attribute outside the allowlist** — **MET.** Diff adds no span and no attribute; `run_id` present on the probe's lines.
- **(a11y) no WCAG/ARIA obligation; 5-command surface intact** — **MET.** No UI surface in the diff; the harness still ships exactly five commands.
- **(supply-chain) deny green, audit reproduces the pinned signature, both read directly** — **MET.** `cargo audit` exit **1**, first diagnostic `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories bans licenses sources` exit **0** (`advisories ok, bans ok, licenses ok, sources ok`). Signature UNCHANGED — the 50th re-pin.

**Gates green.** Commands run: `cargo audit` (expected-RED probe, signature unchanged) · `cargo deny check advisories bans licenses sources` · `cargo nextest run -p conductor-core` (316) · `-p conductor-run` (175) · `cargo test -p conductor-core` · `-p conductor-run` · `cargo nextest run --workspace --profile ci` (**852 passed, 0 skipped**) · `cargo clippy --workspace --all-targets -- -D warnings` (exit 0) · `cargo mutants` ×3.

**Smoke:** the boot-path changed, so P3 fired and closed live in both shells (above).

**Process hygiene** (implement P4 census, re-measured at this wrap against the host process list):

| Process | Started by | Final state |
|---|---|---|
| `andromeda-pulse-mcp` (sidecar, ×2 legs) | this run | terminated — spawned and reaped per call; absent from the post-leg census |
| `conductor.exe` / `cargo` / `rustc` / `nextest` / `cargo-mutants` children / `pwsh` | this run | terminated — absent from the post-leg census |
| `pulse-app.exe` (PID 17404) | the operator, for this run | **left running** — stays up through this wrap's light gate; the overseer stops it after the commit |

`:4317` shows the expected LISTENING socket owned by PID 17404 plus one `TIME_WAIT` with PID 0 — a kernel remnant of a closed connection, not a survivor.
