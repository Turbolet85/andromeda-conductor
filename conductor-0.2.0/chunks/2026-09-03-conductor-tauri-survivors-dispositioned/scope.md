# Scope — conductor-tauri survivors dispositioned

**Marker:** `2026-09-03-conductor-tauri-survivors-dispositioned`
**Version:** conductor-0.2.0 · **Epoch:** 6a — Verification follow-ups
**Working entry (intent, verbatim):** _conductor-tauri survivors dispositioned — the crate's first
mutation score turns into a killing test or a cited accepted-deliberate entry per survivor_

---

## What this chunk builds

The `conductor-tauri` mutation tier produced its first score at
`2026-09-02-mutation-tier-restored-for-conductor-tauri` — 43 planned, 43 tested, and a standing
survivor set that chunk deliberately left undispositioned. This chunk turns **every named survivor**
into one of exactly two terminal outcomes:

1. **killed** — a test that fails against the mutant and passes against the real code; or
2. **classified accepted-deliberate against a CITED standing rule** — a real disposition, never a
   deferral, valid only where a standing rule prescribes the untested shape.

No third outcome exists. No numeric `--fail-under` threshold is introduced (test-plan `:500`
states the score moves as a *consequence* of dispositioning named survivors and is never itself
the acceptance).

## The survivor set (the chunk's work list)

Enumerated in
`conductor-0.2.0/chunks/2026-09-02-mutation-tier-restored-for-conductor-tauri/evidence/mutation-tally.md`,
re-verified this phase against the live sources — every named line resolves to a real function.

**Two counts, both correct, on different runs of an identical tree:**

| view | missed | timeout | total | by file |
|---|---|---|---|---|
| implement run (2026-09-03T05:11Z) | 19 | 3 | **22** | `commands.rs` 14 · `pause.rs` 5 · `main.rs` 3 |
| wrap light gate (re-run) | 18 | 3 | **21** | `commands.rs` 14 · `pause.rs` 4 · `main.rs` 3 |

The single delta is `pause.rs:85 TauriResolver::kind -> "xyzzy"`, **missed** on one run and
**unviable** on the other. Its sibling (`-> ""`) was missed on both. `kind`'s two mutants are
therefore **one disposition unit**, and the headline legitimately reads 21 or 22.

### `commands.rs` — 14 survivors

| site | fn (decl line) | note |
|---|---|---|
| `resolve_handle:38` | `fn resolve_handle` (:37) | **the `resolve_under` traversal guard** |
| `scenarios_dir:44` | (:43) | caller of the guard |
| `runs_dir:48` | (:47) | caller of the guard |
| `manifest_path:52` | (:51) | caller of the guard |
| `resolve_selection:81` + `:81:18` (`==`→`!=`) | (:76) | 2 mutants |
| `load_all:89` | (:88) | catalog read |
| `list_scenarios:113` | (:112, `#[tauri::command]`) | catalog command |
| `list_scenarios_impl:125` | (:121) | catalog impl |
| `run_report:168` | (:167, `#[tauri::command]`) | run-data command |
| `run_envelope:224` | (:223, `#[tauri::command]`) | run-data command |
| `start_run:260` + `:263:8` (delete `!`) | (:253, `#[tauri::command]`) | 2 mutants |
| `run_thread:296` | (:286) | background-thread driver |

### `pause.rs` — 4 stable (5 in the 22-view)

`TauriResolver::kind:85` (:84) — `""` missed on both runs; `"xyzzy"` the flip-flopping 22nd ·
**timeout** `HoldGate::deliver:66` (:65) — `true`, `false` · **timeout** `resolve_operator_hold:105`
(:101). The three timeouts are a distinct class from the misses and may not admit the same
disposition shape.

### `main.rs` — 3 survivors

`main:18` (:17) · `tauri_log_path:44` (:43) — `None`, `Some(Default::default())`.
`main.rs` carries **no `#[cfg(test)] mod tests` at all** (measured this phase), which is the direct
reason all three survive.

## Disposition constraints (binding, not advisory)

- **SIX survivors sit in the security-plan §Input Validation forbidden domain and MUST be KILLED** —
  accepted-deliberate is unavailable for them: `resolve_handle:38` (the guard itself),
  its three callers `scenarios_dir:44` / `runs_dir:48` / `manifest_path:52`, and the two
  `run_id`-class commands `run_report:168` / `run_envelope:224`.
  A `resolve_handle` mutant returning `Ok(Default::default())` bypasses `resolve_under` entirely.
- **`run_envelope:224` is double-governed** — it additionally answers to layout-templates `:144`
  (the load-envelope banner qualifier). `[premise-corrected: the tally's "loses its always-rendered
  label" reading is FALSE. layout-templates :144, design-system §cli Component Patterns 4 and
  a11y-plan §4 independently state the banner is omitted entirely when the run stayed in-envelope,
  and run_envelope's own doc comment (commands.rs:218-221) says "A run that recorded no envelope
  row, or an absent runs dir, yields None — the banner is simply absent, never an error". Ok(None)
  is therefore the SPECIFIED render for an in-envelope run; the defect the mutant introduces is an
  OVER-envelope run rendering nothing.]` The killing assertion must drive an over-envelope subject
  and assert the label is present — never assert the banner is unconditionally present.
- **`tauri_log_path:44` is obs-adjacent — and measured INTO the guard domain.** obs-plan `:103-105`
  states the Tauri backend sink is written **unconditionally** with stderr only as an open-failure
  fallback, so a `None` return silently drops the self-obs stream.
  `[premise-corrected: its domain status is NOT weaker than the six. tauri_log_path calls
  resolve_under directly (main.rs:46), so it sits on the same traversal guard — a
  Some(Default::default()) mutant bypasses that guard, not merely the stream. Treat both mutants as
  must-kill-grade.]` Both are reachable from a new `#[cfg(test)] mod tests` in `main.rs`, which
  today has none.

## Boundaries

**In scope**
- Test-side work inside `crates/conductor-tauri/` that kills survivors, plus any committed fixture
  content and its production-reader round-trip pinner.
- A recorded per-survivor disposition ledger in this chunk's `evidence/`, naming for each survivor
  either the killing test or the cited rule.
- One re-run of the tier to measure the resulting score (budget basis **~4 m 18 s** for 43 mutants
  at `--jobs 2` — the measured figure, never the 5400 s the prior plan guessed).

**Out of scope**
- `conductor-run` composition-root survivors — the **next** Epoch 6a entry owns them, per crate.
- Any numeric mutation threshold, `--fail-under`, or CI gate on the mutation score (test-plan `:500`
  forbids it; the tier is audit-tier, non-blocking).
- Production-code behavior change. Killing a survivor is a TEST-side act; if a survivor can only be
  killed by changing production behavior, that is a finding to surface, not a licence to refactor.
- Re-running or re-scoring `conductor-cli` / any other crate's tier.

## Surfaces and contracts touched

- `crates/conductor-tauri/src/commands.rs` — `#[cfg(test)] mod tests` (:343, 6 tests today).
- `crates/conductor-tauri/src/pause.rs` — `#[cfg(test)] mod tests` (:115, 6 tests today).
- `crates/conductor-tauri/src/main.rs` — no test module today.
- Committed fixture precedent: `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` +
  `lamps_fixture.rs` (round-trip pinner) and `fixtures/over-envelope.toml` + `envelope_fixture.rs`.
  `[premise-corrected: cross-package reuse is UNAVAILABLE, so the fork is closed. crate_edges shows
  conductor-tauri → {conductor-core, conductor-run} with ZERO inbound, and conductor-run's fixtures
  live in its tests/ integration dir, which is not a dependable surface of the crate. This chunk
  seeds its own subject under crates/conductor-tauri/. Consequence: envelope_fixture.rs's shape uses
  assert_fs::TempDir, which conductor-tauri's dev-deps do NOT carry (the manifest comment records
  assert_cmd/assert_fs leaving with the parity leg) — so a dev-dependency addition is in play and
  falls under the red-audit dependency-admission rule.]`
- Contracts consulted, all re-verified this phase: test-plan `:500` (§10 Mutation-survivor
  disposition) · security-plan `:98` (§Input Validation) · layout-templates `:144` · obs-plan
  `:103-105` · `.claude/rules/testing.md` `:63` (2026-06-26 background-thread deferral), `:64`
  (2026-06-27 extract DI-free core), `:75` (2026-09-02 committed fixture + production-reader
  round-trip).

## Folded annotations

### CONTEXT (operator WRAP directive 2026-09-03 — recorded pre-direction, naming this entry and slot)

Folded above in full: the 43/43 basis, the 21-or-22 headline with `kind` as one disposition unit,
the six forbidden-domain must-kills, the layout-templates double-count, and the acceptance quoted
verbatim from the prior plan's criterion 2:

> (tests) Every NAMED `conductor-tauri` survivor ends killed or classified accepted-deliberate
> against a cited standing rule; no numeric `--fail-under` threshold is introduced (per test-plan §10
> Mutation-survivor disposition).

The prior chunk dispositioned NONE **by design** — its criterion 2 was authored when the survivor
set was unmeasurable by construction, and 8 of the 22 sit outside its touchpoint list.

**LIKELY SHAPE (the entry marks this a hypothesis, preserved verbatim):** _"the path
helpers and catalog commands are unobservable without a staged repo root, and the run-data commands
are currently tested at exactly their empty case — which IS the mutants' return value"_ → killing
them needs seeded fixtures inside `conductor-tauri`'s test module, a staged repo root plus a
populated runs dir.

`[premise-corrected: the hypothesis is HALF right. Its second clause is VERIFIED — commands.rs:447
run_envelope_command_returns_null_when_no_run_has_an_envelope asserts `standing.is_null()`, which is
exactly what the `Ok(None)` mutant returns, and run_report_command_returns_a_well_formed_record_list
asserts only that the payload deserializes. Its first clause is FALSE for the path helpers:
resolve_handle(var, default) takes `default` as a PARAMETER, so both the resolve-succeeds and the
reject arms are reachable with NO staged root and NO unsafe env mutation — pass an unset var name
with an in-scope default, then with a `../escape` default. Separately, a staged root could not point
at the repo's real scenarios/ anyway: resolve_under's base is std::env::current_dir(), which under
cargo test is the CRATE dir, and it rejects both absolute candidates and `..`, so any fixture must
live UNDER crates/conductor-tauri/.]`

The three `pause.rs` **timeouts** are a distinct class from the 18 misses (a timeout is
a hang, not an unobserved return), and no folded annotation states how they should be dispositioned.

`[premise-corrected: they ARE killable, so the two-outcome acceptance covers them and no third
treatment is needed. The cause is measured, not speculative: HoldGate::deliver's two mutants and
resolve_operator_hold's body mutant each stop the oneshot from ever being sent, and two existing
tests — gate_delivers_the_decision_to_the_awaiting_hold (pause.rs:166) and
resolve_operator_hold_command_delivers_the_decision (pause.rs:192) — then block forever on
`rx.await`, so the binary times out before any assertion reports. Bounding those awaits with
tokio::time::timeout converts timeout → caught. Note the layouts extract's "the timeouts have no
distinguishing observable" reasons about the DOM surface, a different tier, and does not bear on the
Rust tier where these are plainly observable.]`

### PREREQ (standing, external decay — operator directive 2026-09-02; ratified at the 2026-08-10 wrap)

**Re-check `cargo audit`; the advisory DB has not parsed since 2026-08-09. This is the 46th.**

- Run with the exit captured **BEFORE any pipe** (`cargo audit; AUDIT=$?`).
- Run `cargo deny check advisories bans licenses sources` **UNCONDITIONALLY** as the overlap probe —
  never gated on whether the lockfile moved.
- **Expected signature (probe-auto-satisfy):** `cargo audit` exit **1**, first diagnostic
  `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; overlap
  `cargo deny` exit **0**. Reproduced byte-identically at the 44th and 45th re-checks.
- **ANY deviation** — a changed diagnostic line, a moved exit code, an overlap shift — restores the
  FULL form and is reported, never re-pinned.
- Dependency-delta basis under this red gate: assert the **package count** plus `cargo deny` green
  over the NEW lock, **never** lockfile byte-identity.
- **External-decay standing pin, NOT a source-delta gate deferral — never clear it on a rust-gate
  closure.** Full rationale: the `2026-08-09-interpretation-correctness-posture` chunk report.

## Acceptance (the entry's own, verbatim)

The prior plan's criterion 2, quoted above, is this chunk's acceptance. No numeric threshold.
