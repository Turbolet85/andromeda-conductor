# Codebase Research — 2026-08-10-pulse-run-contract

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 4 · **Graph queries:** 6

## Files inspected
- `crates/conductor-verify/src/preflight.rs` (full, 352) — the gate. Four precondition `const`s
  (`:99`, `:107`, `:111`, plus the inline version/tool `format!`s), the cascade at `:164-184`, the private
  `NotFound{EmptyCorpus,FingerprintAbsent}` at `:242-258`, `poll_canary` at `:273`, and the `ReadyState`
  struct at `:74-84` (8 fields).
- `crates/conductor-run/src/lib.rs` (1-200) — the composition root. `preflight()` `:48`, `readiness()` `:70`,
  `canary_gate()` `:112`, `emit_canary()` `:136`, `canary_poll()` `:156`, and **two full `ReadyState` struct
  literals** (`unreachable_state` `:86-103`, `canary_blocked_state` `:167-184`).
- `crates/conductor-cli/src/commands/preflight.rs` (full, 33) — the single render call site; exit 0 iff
  `ready`, `[BLOCKED] preflight — {precondition}` otherwise.
- `crates/conductor-core/src/load_envelope.rs` (1-120) — loader + the `EnvelopeStatus` run-level-qualifier
  precedent (`:74-106`).
- `crates/conductor-core/src/config_path.rs` (full, 75) — `resolve_under` (`:17`): rejects absolute,
  rejects `..`, canonicalize-checks containment.
- `crates/conductor-core/src/capability_manifest.rs` (grep) — `default_path()` `:31`, `load()` `:36`,
  `e.kind()`-only read error `:39`, `sanitize_error` on parse `:42`, separate `validate()` `:60-76`.
- `crates/conductor-verify/src/spawn.rs` (full) — fixed program const, `.env(...)` data-dir, metacharacter
  reject; the shipped test asserts `envs.len() == 1, "only the data-dir env is set"`.
- `crates/conductor-core/src/drift.rs` (40-95) — `UNBACKED_AUTO` (10 ids incl. `P-073`); its doc comment
  names **this entry** as P-073's owner ("The remaining six retire as their scenarios land").
- `contracts/pulse-load-envelope.toml` + `contracts/mcp-contract.toml` — the two committed artifact shapes;
  the envelope's `provenance` field and its `[[exempt]]` reason-carrying ledger.

## Graph impact (from `tree-query-2026-08-10-pulse-run-contract.json`)
- **`run_preflight`** — 7 rows. Real call sites: `conductor-run/src/lib.rs:127` (`canary_gate`),
  `conductor-verify/src/preflight.rs:215` (`preflight_boot`), `conductor-verify/src/lib.rs:43` (re-export);
  test sites: `tests/preflight.rs:12`, `:26` (`drive_with`), `:148`. **A new parameter threads 3 source +
  3 test sites.**
- **`ReadyState`** — 81 ref rows across 5 files, and critically **four full struct-literal constructions**:
  `preflight.rs:191-198`, `preflight.rs:222-234`, `conductor-run/src/lib.rs:89-101`,
  `conductor-run/src/lib.rs:168-183`. **Adding a field to `ReadyState` breaks all four** — this is the
  caller-threading list, and it is the single biggest cost driver between the two design options.
- **`preflight_boot`** — 2 rows: `conductor-verify/src/lib.rs:43` (re-export), `tests/preflight_spawn.rs:8`.
- **`emit_canary`** — 1 row: `conductor-run/src/lib.rs:117` (`canary_gate`). Private, single caller —
  changing the storm's shape or adding a warm-up is contained.
- **`crate_edges`** — `conductor-verify → conductor-core`; `conductor-run → conductor-verify`;
  `conductor-cli → conductor-verify`. `conductor-tauri` reaches verify only through `conductor-run`, so a
  verify-internal change has no GUI blast radius.
- **`RunContract` / `run_contract`** — **0 rows**. Consulted-and-empty: the name is free in every crate.

## Patterns detected
- **Committed-manifest loader** (`capability_manifest.rs:31,36,39,42`; `load_envelope.rs:117`): `default_path()`
  returning a workspace-relative `PathBuf`, `load(path)` mapping the read error to `e.kind()` only and the
  parse error through `sanitize_error`, then a separate `validate()` returning `CoreError::Config` per
  violated bound. Two instances — the third should be indistinguishable.
- **Precondition const + linear cascade** (`preflight.rs:99-111`, `:164-184`): one `&'static str` per named
  condition, selected by an `if/else if` chain ordered most-fundamental-first, with the chosen string placed
  in `blocked_precondition` and logged once at `:188`.
- **Run-level qualifier that is deliberately NOT a state** (`load_envelope.rs:68-106`): `EnvelopeStatus` rides
  the run, carries a `label()` for the always-rendered text, and its doc comment states the `ReportState` set
  stays closed at five. The precedent if a contract term qualifies the run rather than a check.
- **The storm shape is already correct** (`conductor-run/src/lib.rs:105-107,146-150`): `CANARY_STORM_COUNT = 6`
  identical-fingerprint exceptions emitted back-to-back, documented as "over Pulse's `>=5 in 30s` retry-storm
  floor (P-018)".

## Conventions to follow
- **Loader placement**: `conductor-core`, one module per manifest, re-exported from `lib.rs` beside
  `CapabilityManifest` / `LoadEnvelope` (`crates/conductor-core/src/lib.rs:38` is the sibling export line).
- **No `CONDUCTOR_*` override**: both SUT-facing manifests resolve via `default_path()` only; `resolve_under`
  (`config_path.rs:17`) guards operator-supplied handles, which this manifest deliberately does not have.
- **Host-path-free strings**: `redact_value` is applied at the seam before anything enters `ReadyState`
  (`preflight.rs:125`, `:152`, `:221`, `:232`).
- **`Ok(Blocked)` never `Err`**: every readiness outcome returns `Ok(ReadyState)`; only manifest-load faults
  reach `Err` (`conductor-run/src/lib.rs:117` uses `.context(...)` for exactly that).

## New files to create
- `contracts/pulse-run-contract.toml` — the recorded run contract: `sut_version` · `captured_at` ·
  `provenance` · the terms. Name unconstrained (graph collision check empty).
- `crates/conductor-core/src/run_contract.rs` — the loader + `validate()`, third instance of the pattern.
- `scenarios/<name>.toml` — **conditional** on open question 2 (P-073 backing).

## Files to modify
- `crates/conductor-core/src/lib.rs` — `mod run_contract;` + the re-export beside the two siblings (`:38`).
- `crates/conductor-verify/src/preflight.rs` — new precondition `const`(s) + cascade arm(s).
- `crates/conductor-run/src/lib.rs` — `canary_gate` `:112-129` if `run_preflight` gains a parameter; **plus
  `unreachable_state` `:86-103` and `canary_blocked_state` `:167-184` if `ReadyState` gains a field**;
  `emit_canary`/`canary_poll` if a warm-up or budget term lands.
- `crates/conductor-verify/tests/preflight.rs` — `drive_with` `:26` threads any new `run_preflight` param;
  new legs per new term.
- `crates/conductor-verify/src/lib.rs:43` — the re-export line if the exported type set changes.
- `crates/conductor-verify/tests/preflight_spawn.rs:8` — only if `preflight_boot`'s signature changes.
- `crates/conductor-core/src/drift.rs:61-63` — `UNBACKED_AUTO` minus `P-073`, **conditional** on question 2.

## The decisive finding — Term C is bootstrap readiness, not storm shape
The scope inherited "the storm shape (≥5 same-fingerprint within 30s)" as a contract term. The code already
satisfies it: `emit_canary` emits **6** identical-fingerprint exceptions in a tight loop, explicitly citing the
`>=5 in 30s` floor. Yet the probe recorded `cues_emitted: 0` with `services_in_bootstrap: 1` /
`services_ready: 0` throughout. The cue evaluator never *considered* the service, so the storm count was never
the binding constraint — **the service being out of baseline bootstrap is**. A term prescribing storm shape
would restate what already holds and leave the actual blocker unnamed.

Second, `canary_poll()` (`:156-163`) defaults to **30 attempts × 1s**, while Pulse's L3 digest cadence alone is
20-60s with L4 behind it. Even once a cue fires, the default budget can expire before an incident exists; the
probe used `CONDUCTOR_PREFLIGHT_TIMEOUT=90`. The default is a contract-relevant term in its own right.

## Open questions
- **How does preflight OBSERVE `ANDROMEDA_PULSE_L4_DETERMINISTIC` on a process Conductor does not launch?**
  The read-back surface offers nothing: the 4 live-buffer tools return empty cross-process (arch §Standard
  Contracts) and the 4 corpus tools expose no mode field; `query_incident_list` takes no arguments. Candidates
  are (a) an operator-declared term the contract file records and preflight reports, or (b) Conductor's own
  inherited env as a same-shell proxy. The v2-17 precedent binds: name the condition and its candidate causes,
  never claim a measurement. → **blocks: plan-decision**
- **Does P-073's backing take a scenario TOML?** `UNBACKED_AUTO`'s doc names this entry as its owner and says
  the six "retire as their scenarios land" (the `P-079` → `constellation-severity-live-wiring.toml` precedent),
  but v2-18's acceptance is a preflight assertion with `method: integration`. The answer decides whether
  `drift.rs:61` and a new `scenarios/*.toml` are in this commit. → **blocks: plan-decision**
- **Does the contract surface as a `ReadyState` field or as precondition strings only?** String-only threads
  nothing; a new field breaks **four** struct literals across two crates. → **blocks: implementation-scope**
