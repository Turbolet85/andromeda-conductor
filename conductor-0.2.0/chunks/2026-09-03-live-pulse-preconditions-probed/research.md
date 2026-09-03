# Codebase Research — 2026-09-03-live-pulse-preconditions-probed

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11 · **Graph queries:** 3 (`rust` plane, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural
  extraction (48 KB across 59 lines: `awk` index of every line with byte sizes → offset-bounded reads
  covering all 59 spans, `## Session Additions` included, 20 curated entries); `.claude/rules/observability.md`
  — read IN FULL (40 lines, both Session Additions); `.claude/rules/testing.md` — structurally indexed
  (57 KB / 79 lines) with the three on-point entries read (`:67`, `:74`, `:75`). `verification-harness.md`
  and `observability.md` both carry `scripts/agent-run.*` in their `paths:`, so both auto-load on this
  chunk's harness touchpoint; `testing.md` covers `crates/**/src/**/*.rs`.

## Files inspected
- `contracts/pulse-run-contract.toml` (full) — five `[[term]]`s + `[incident_formation]`; the decisive artifact for the completion check.
- `crates/conductor-core/src/run_contract.rs` (`:60-68`, `:152-175`, `:237`) — `CheckKind` enum; `evaluate` filters `ShellDeclaration` only; `observed_env` yields only those terms' `env` names. Both are PURE functions of their inputs.
- `crates/conductor-run/src/lib.rs` (`:316`, `:345-362`, `:487`) — `observe_run_contract` (crate-private) reads the env via `declares` and delegates to core's `evaluate`; its sole production caller is `canary_gate:316`. `execute_scenario:487` is `probe_egress`'s only production caller.
- `crates/conductor-emit/src/client.rs` (`:23`, `:36-46`) — `DEFAULT_OTLP_ENDPOINT`; `probe_egress` connect-and-drop with a bounded timeout, `Err` on refusal, doc-stated "orchestrating the gate into a run is the CLI's job".
- `crates/conductor-cli/src/cli.rs` (`:20-55`) — `Commands` carries FIVE verbs: `Run` · `Suite` · `Report` · `Preflight` · `Coverage`.
- `crates/conductor-core/src/redact.rs` (`:21-68`) — `ALLOWLISTED_FIELDS`; `run_id` at `:29`, `port` at `:51`.
- `crates/conductor-core/src/obs.rs` (`:32-53`, `:184-187`) — `ServiceIdentity::resolve` mints a `run_id` when none is supplied.
- `crates/conductor-verify/src/spawn.rs` (`:15`, `:80`, `:134`) — `PULSE_MCP_PROGRAM` fixed name; `Command::new` with NO creation flags.
- `scripts/agent-run.sh` (`:60-90`) — `latest_run_id`; `boot` = `conductor preflight --json` under `preflight_budget_sec`.
- `crates/conductor-emit/tests/egress.rs` (`:85`, `:93`) — the shipped connectable-stub / refused test pair.

## Graph impact (rust plane, `db_state: fresh`, `probe_hits: null` on all three — no 0-row query, so no leaf claim rests on a pattern)
- **`probe_egress`** — 12 rows: production caller `execute_scenario` @ `conductor-run/src/lib.rs:487`, plus the re-export at `conductor-emit/src/lib.rs:22`, the import at `conductor-run/src/lib.rs:30`, and the test pair @ `conductor-emit/tests/egress.rs:85`/`:93`. A scheduling-time caller is **additive** — no signature change, so no caller threading is owed.
- **`observe_run_contract`** — 3 rows: `canary_gate` @ `conductor-run/src/lib.rs:316` (sole production caller) + two unit tests @ `:1566`/`:1589`. **The contract observation is welded to the canary path**; the reusable seam is one level down in `conductor-core`.
- **`declares` / `observed_env` / `RunContract` / `RunContractStatus` / `preflight`** — 10 rows locating the surface: `declares` and `observe_run_contract` are `conductor-run`-private; `evaluate`/`observed_env` are public on `conductor-core`'s `RunContract`.

## Patterns detected
- **The `check`-kind taxonomy is an honesty model, and one term is mis-classified against it** (`run_contract.rs:60-68`): `ShellDeclaration` is the only kind that can be unmet, `Asserted` means true-by-construction, `DeclaredNotObservable` is recorded and never blocks. `sidecar-built` is `Asserted` with the rationale "reaching term evaluation at all proves it" — but a `PATH` miss short-circuits to `[BLOCKED]` in ~0 s *upstream* of term evaluation (`verification-harness.md:54`), so the term is by-construction-true exactly when it is false.
- **Env-at-the-caller is already the shipped shape** (`run_contract.rs:153` + `conductor-run/src/lib.rs:345-352`): core's `evaluate` takes `&BTreeSet<String>` and reads no environment; `conductor-run` does the `std::env::var` read. This is precisely what `testing.md:67` mandates for any env-sourced gate condition, so the probe extends an existing compliant pattern rather than introducing one.
- **A verb that reports a classification without running a scenario already exists** — `Coverage` (`cli.rs:49-54`), which renders the coverage matrix and optionally writes it. The nearest precedent for the probe's verb shape.
- **Non-mutating egress probing is a solved primitive with its own sanctioned test shape** (`client.rs:42` + `tests/egress.rs:85`/`:93`): ephemeral-port in-process stub for the reachable arm, refusal for the unreachable one — exactly test-plan §5's mandate, so no new test technique is needed.
- **`boot` writes no run artifacts** (`verification-harness.md:47`, final extension): `conductor preflight` produces no journal, no `runs.db` row, and does not refresh `logs/agent-latest.jsonl`. Any criterion asserting a runtime artifact needs a scenario leg, not a probe/gate leg.

## Conventions to follow
- **Bounded span-name set** (`observability.md:24`): no probe name exists in it. The governing precedent is the 2026-09-01 operator ruling — a new boundary logs a §6 `info!` with the outcome on the allowlisted `message` field inside the caller's existing span and mints NO span (obs extract). Adding a member by amendment is the alternative sanctioned route (`emit.logs_batch` precedent) but is a wrap-owned channel.
- **Harness size is capped**: `verification-harness.md:24` — "Do not add a 6th command without a test-plan amendment." A 6th `agent-run` verb is therefore not available to this chunk without an Expected amendment.
- **Exit-code semantics fork by surface**: `conductor preflight` exits 0 iff `ready:true` (`verification-harness.md:45`) — a deliberate exception to the run-path rule that reported states exit 0. A probe must pick its side explicitly, and `:58`'s extension warns that a guard designed to skip at exit 0 makes the exit code unable to distinguish a pass from a total skip.
- **`.sh`/`.ps1` parity is binding** (test-plan §3), with the 2026-08-13 amendment recording the two shells actually diverging (a fixed 30 s wrapper in `.sh`, absent in `.ps1`) and killing every live run.
- **Artifact hygiene is processor-stage** (`observability.md:29`): the allowlist DROPS non-allowlisted field names and the scrub masks absolute host-file paths to `<redacted>`. `run_id` and `port` are allowlisted; a resolved program path and the `PATH` value are not, and must not be emitted at all.

## Extract signals closed late (both were open when the first draft of this file was written)
- **design's "does the probe's emit path already route through the shipped TTY/color gate?"** — YES, and the
  gate is stronger than the constraint asked for. `crates/conductor-cli/src/render.rs` documents that all
  styling shares ONE `stdout_color()` tty decision (`:8`), and there are TWO independent gates —
  `:133` (stdout) and `:141` (stderr) — each requiring a terminal AND `NO_COLOR` unset AND `TERM != dumb`.
  This satisfies design's "stderr's own `IsTerminal` gate distinct from the stdout gate" with shipped code.
- **tests' "do the shipped scripts already expose a probe verb, and in both shells?"** — NO probe verb in
  either, and the verb sets are at parity: `.sh` cases at `agent-run.sh:73/81/118/130/144` and `.ps1` cases at
  `agent-run.ps1:77/91/133/143/156` are the same five (`boot`/`run`/`status`/`cleanup`/`logs`). `.ps1`'s `boot`
  also derives its budget from the contract (`Get-PreflightBudgetSec`) and exits 124 on overrun, mirroring
  `.sh`'s `timeout`.

## Surfaced spec↔reality divergence (NOT amended here — phase does not edit spec sources)
- **`design-system.md` §Surface: cli names `anstream` as the TTY-gating mechanism; the code uses
  `owo-colors` + `std::io::IsTerminal`.** `anstream` is absent from the workspace manifest and from
  `conductor-cli`'s (`Cargo.toml:63` and `crates/conductor-cli/Cargo.toml:20` declare `owo-colors` only),
  and `grep` finds no `anstream` reference in `conductor-cli/src/`. `architecture.md`'s stack table agrees
  with the code (owo-colors 4 · indicatif · comfy-table · inquire, no anstream). The layouts extract
  inherited the same `anstream` wording, so both derived statements trace to one master. The BEHAVIOUR the
  spec mandates is fully shipped — only the named mechanism is wrong. → an **Expected amendment** for
  wrap against `design-system.md` (with `layout-templates.md` as the cascade site), not a code change and
  not a phase edit.

## New files to create
- None required. Every seam the probe needs already exists; the deliverable is composition plus (per the P4 fork below) contract data.

## Files to modify
Provisional pending the P4 fork on where the probe surfaces:
- `contracts/pulse-run-contract.toml` — add/reclassify terms so subjects 1–3 are representable (see Open questions 1).
- `crates/conductor-core/src/run_contract.rs` — if a new `CheckKind` or a per-term observation shape is needed; `evaluate`/`observed_env` are the pure seam.
- `crates/conductor-run/src/lib.rs` — the env read (`declares`, `observed_env` caller) and a probe entry point NOT routed through `canary_gate`.
- `crates/conductor-cli/src/cli.rs` + its dispatch — the verb surface (a 6th verb here is unconstrained; the harness's 6th COMMAND is not).
- `scripts/agent-run.sh` + `scripts/agent-run.ps1` — only if the probe rides an existing verb; at identical semantics.
- Crate-local companions: `crates/conductor-emit/tests/egress.rs` (existing pattern to extend, not change), `conductor-core`/`conductor-run` `#[cfg(test)]` modules, and any test pinning the contract's term SET by exact set-equality (the `check_scenario_backing` precedent) — a data pin is not a call, so the graph's caller query does not surface it.
- **Seam facts:** no new dependency is contemplated, so no normal-vs-dev visibility question arises; `RunContract`/`RunContractStatus`/`CheckKind` are already `pub` on `conductor-core` and re-exported where used, so no export-surface change is owed unless a new type is introduced. No signature change is planned for `probe_egress`, so its caller set is informational rather than a threading list.

## Open questions
1. **Do the contract-data changes belong to this chunk?** Subjects 1 and 3 have no representable term and subject 2's term is `Asserted` (structurally unable to report). The natural home is `contracts/pulse-run-contract.toml` + possibly a `CheckKind` change — but re-classifying `sidecar-built` alters gate semantics, since only `ShellDeclaration` can be unmet, and `ShellDeclaration` means "observable as a declaration in Conductor's own environment", which a `PATH` lookup and a TCP connect are *not*. → blocks: **plan-decision** (a contestable fork; P4 resolves with a marked recommendation).
2. **Where does the probe surface?** A new CLI verb is unconstrained, but a 6th `agent-run` command needs a test-plan amendment phase cannot author; riding `boot` risks the `:58` exit-code ambiguity. → blocks: **plan-decision**.
3. **Does the probe emit a span, or a `message`-bearing `info!` inside its caller's span?** The bounded set holds no probe name and the 2026-09-01 ruling prefers the no-new-span route. → blocks: **implementation-scope** (the file list stays provisional for /implement either way).

## Self-check
Read 4 files the extracts flagged to modify (`run_contract.rs`, `lib.rs`, `client.rs`, `cli.rs`) · every pattern cited to `file:line` · no new-file paths proposed · every modify path verified to exist · all three graph queries returned non-empty with `probe_hits: null`, so no claim here rests on an unprobed pattern.
