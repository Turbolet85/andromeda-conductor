# Scope — 2026-09-10-release-build-and-bundle

**Working entry (verbatim intent):** _Release build and bundle — release binary plus Tauri 2 bundle with a
final SLO verification pass_

The version's final entry. `conductor-0.2.0` closes here: the two remaining unclaimed capabilities
(`v2-27`, `v2-21`) both resolve inside this chunk, flipping the matrix done-test.

## What this builds

1. **A release binary** — `cargo build --release` over the workspace, produced only behind the standing
   supply-chain gate (`cargo audit` + `cargo deny check` green, `Cargo.lock` committed and un-drifted).
   The `conductor-cli` `agent-run` binary is the release gate; the GUI is convenience.
2. **A Tauri 2 bundle** — the `conductor-tauri` bundle. `tauri.conf.json` already declares
   `bundle.active: true`, `targets: "all"`, and the three icon paths, so this is a build-and-verify job,
   not a bundle-configuration job.
3. **A final SLO verification pass recorded against the shipped artifact** — the pass that makes `v2-27`'s
   acceptance true. Its subject is the artifact this chunk produces, not a debug build.
4. **Two carried defects landed with that pass** (the two open `CARRY` annotations, below).

## Boundaries

- **No new capability surface.** No new scenario, no new `CONDUCTOR_*` handle, no new port, no change to the
  5-command harness discipline, no OTLP or MCP contract change.
- **No route mutation.** This chunk does not create, retire or reorder route entries.
- **The version stops here.** Nothing moves to 0.3.0 as part of this chunk.
- **Scope law holds unchanged** — shipped binaries open no inbound listener; the `:4317` port-occupier
  remains the sole deliberate bind.

## Surfaces and contracts touched

| Surface | Why it is in scope |
|---|---|
| `Cargo.toml` (workspace) · `crates/*/Cargo.toml` | release profile + the version stamp question below |
| `crates/conductor-tauri/tauri.conf.json` | bundle production; carries its own `version` field |
| `crates/conductor-core/src/obs.rs` | CARRY A — the panic-hook race, test-side remedy |
| `scripts/agent-run.{sh,ps1}` | the final verification pass is driven through the harness |
| `conductor-0.2.0/verification-matrix.json` | `v2-27` + `v2-21` claims |
| `crates/conductor-tauri/ui/**` (wdio/SR leg) | CARRY C — SR announcement variance, if owned here |

## Folded freight (the working entry's annotations)

The entry carries **three** `CARRY` annotations at annotation positions (offsets 99, 1352, 3276). A fourth
occurrence of the token at offset 4167 is **prose inside CARRY C**, not an annotation — it does not fold.
Of the three, one is already discharged. **Two are open.**

### CARRY A — the process-global panic-hook race (OPEN)

From `2026-09-03-live-pulse-preconditions-probed`, operator WRAP directive item 1. Folded verbatim in
substance; the entry marks it *"verified first-hand at HEAD"*.

- **Re-verified at HEAD (2026-09-10, this promotion):** the coordinates are **correct**.
  `conductor-core/src/obs.rs` holds `take_hook`/`set_hook` pairs at exactly `:535-540`
  (`panic_hook_emits_one_error_line`, fn at `:528`) and `:582-587`
  (`panic_hook_redacts_host_path_in_payload`, fn at `:575`). A third `set_hook` at `:172`
  (`install_panic_hook`) is the production installer and is **not** part of the race.
- **The mechanism, as the entry states it** `[inferred]` — under `cargo test` (one process, parallel
  threads) one test restoring the previous hook mid-flight leaves the other's deliberate panic handled by
  the DEFAULT hook and its buffer empty. Observed red exactly once; passes since; did not reproduce at HEAD
  in 3 forced-parallel runs. *A race that passes is still a race* — testing.md §Quality gates owns it as a
  shared-state defect to remove at the cause.
- **Pre-existing and UNMASKED, not caused** — both sites are in HEAD's source.
- **Remedy named by the entry:** serialize the two against each other, or give them one shared guard.
  The entry notes relocating them to their own test binary would widen `conductor-core`'s public API for
  tests alone, since both use crate-private helpers — **confirmed at HEAD**: both call `build_subscriber`,
  `fixed_identity` and `log_panic`, all crate-private.
- **Disposition:** in scope. "Land it with this entry's final verification pass."

### CARRY B — `agent-run.ps1 --e2e` exiting 0 over a red wdio suite (DISCHARGED — no work)

From the 2026-09-05 0-pending adaptation. The entry itself records the discharge:
**DISCHARGED by `2026-09-07-a11y-ci-gate`** (operator wrap-directive item 3b) — both shells now assert the
runner's PRINTED verdict at identical semantics (the `Spec Files:` failed count, the per-spec skip tally
against the expected-skip SET, the `[webview2 <version> windows]` driven-session banner), with the exit read
from the BARE command, never through a pipe. The entry's own words: *"Nothing owed here."*

**Disposition: folds as a recorded closure, not as work.** It is written here so this chunk does not
re-open a settled item. Two format details the discharge calibrated, preserved because the final pass
depends on them: the `Spec Files:` line **omits** the `failed` term when nothing failed, and the skip tally
is `N skipped` plus per-spec `-` markers, never the token `pending`.

### CARRY C — the SR leg's run-to-run announcement variance (OPEN; ownership is an operator call)

From `2026-09-07-sr-findings-fixed`, wrap light gate. **Real and NOT closed.**

- **Re-verified at HEAD (2026-09-10, this promotion) — the tally is exact.**
  `chunks/2026-09-07-sr-findings-fixed/evidence/nvda-pass.live.run3-lightgate.json` holds 51 rows:
  **33 announced-as-expected / 15 not-run-here / 2 subject-absent / 1 not-announced**. The mover is
  `S3-04` ("Escape resolves NoGo; focus restored"), `outcome: not-announced`, `state: aborted`,
  `arm: agent`.
- **Rate:** 1 row of 51, on the third of three runs — far rarer than the original finding described, which
  is why two runs missed it. The entry's own lesson: *"ONE observation cannot separate variance from
  regression" applies to TWO observations too.*
- **The entry's `hypothesis:` marker text, preserved verbatim** `[inferred]` — *"the `nvda_named_window:
  false` correlation is the thread to pull first — it recurred across sessions and is already recorded as
  the attach signal."*
- **⚠ This promotion's re-verification WEAKENS that hypothesis, and the weakening is a finding, not a
  correction of the CARRY:** the field is **not** a discriminator across the three live runs —
  | run | `nvda_named_window` | S3-04 |
  |---|---|---|
  | `run1` | `true` | announced-as-expected |
  | `run2` | **`false`** | **announced-as-expected** |
  | `run3-lightgate` | `false` | **not-announced** |
  `run2` is the counter-example: the same `false` value with the row announcing normally. So the
  correlation is 1-of-2 on this evidence — necessary-at-best, and not sufficient. Whatever this chunk does
  with CARRY C must not treat that field as the cause on the strength of the CARRY's wording alone.
- **Coordinate correction:** `nvda_named_window` is **not** a top-level field of the evidence JSON. It lives
  at `subjects.{subject}.foreground.nvda_named_window`. A probe reading it at top level gets `None` and
  would mis-read every run as unset.
- **Constraint on any remedy** — test-plan §10's zero-flake bar: the fix is at the cause. **No retry, no
  runner pin, no in-test `sleep(N)`** — that carve-out was proposed and REJECTED at
  `2026-09-06-operator-gated-live-suite`.
- **Disposition — folded in, pending an operator fork raised at P4.** The entry pins it here because *"this
  entry's final SLO verification pass is the plausible owner"* and states: *"if it wants its own entry, that
  is the operator's call at promotion."* It is folded per that stated default so scope is complete; the
  fork is put to the operator at P4. If it is forked out, note that creating a new route entry is outside
  `/andromeda-phase`'s powers — it would be a wrap route-resolve adaptation.

## Capabilities this chunk is expected to claim

- **`v2-27` · Release build and bundle** (`method: manual`) — the chunk's own subject. Acceptance as
  written: *a release binary and a Tauri 2 bundle are produced with cargo-audit and cargo-deny green and
  Cargo.lock un-drifted, and a final SLO verification pass is recorded against the shipped artifact.*
- **`v2-21` · Conductor's own verification ledger** (`method: by-construction`) — acceptance as written:
  *verification-matrix.json exists with exactly one entry per requirements.md capability, ids unique; no
  P-NNN appears in an id position anywhere in the matrix (Pulse P-IDs occur only inside acceptance text).*
  `[inferred]` This is an enumeration over committed artifacts that `/andromeda-implement` can itself
  write, so it satisfies the claim-reachability rule (its decisive artifact is not wrap-authored). To be
  confirmed at P3/P5.

## Open questions — CLOSED at P3 (see `research.md` §Scope premise closure)

- **The version stamp.** VERIFIED as **unmandated**: `architecture.md` registers no workspace-package or
  `tauri.conf.json` version policy and its amendment sidecar is explicitly empty on that axis. Newly
  measured: **no test pins the live value** — the two `"service.name":"conductor"`/`"0.1.0"` literals are a
  synthetic `format!` fixture (`baseline_harvest.rs:206`) and a verbatim historical capture (`:369`), while
  `canary_obs_witness.rs:70` asserts the key, not the value. But the stamp IS observable at two `env!`
  sites — `obs.rs:51` (`service.version` on every self-obs line) and `conductor-verify/src/client.rs:93`
  (`clientInfo.version` on the MCP `initialize` wire). A bump breaks nothing and is required by nothing →
  a genuine operator fork, raised at P4.
- **What "a final SLO verification pass" concretely names.** VERIFIED: obs-plan defines it as a JSON field
  assertion, `latency_ms <= threshold(slo_tier)` against 5000/20000/90000 ms at report-generation time,
  never an instrument. Two binding constraints found: a graded `latency_ms` requires a **live Pulse** (else
  every row is `Blocked` with null measurements and asserts nothing), and
  `constellation-severity-live-wiring`'s `<20s` tier is **unattainable by construction** and already
  ledgered `open` in `.andromeda/residuals.md` as **not blocking the release** — so the pass records it as a
  named ledgered exception. The leg set is an operator fork, raised at P4.
- `[premise-corrected: cargo-tauri is not on PATH; Cargo.toml declares only tauri-build; @tauri-apps/cli
  appears 0× in ui/package.json and ui/package-lock.json]` **Bundle targets on this host.** The bullet asked
  the wrong question. Targets are not the obstacle — **no Tauri CLI exists in this repo or on this host at
  all**, and `architecture.md:204` states that only the Tauri CLI runs `beforeBuildCommand`. `v2-27`'s
  bundle half therefore cannot be produced at HEAD until the CLI is acquired; the plan takes
  `cargo install tauri-cli` as a decisive lean (the established host-dev-tool pattern).

## P4 decisions (operator, one review round) — scope amended to match

Validation-1 classified all three as **intent-incomplete**: planning surfaced what the scope had left open,
so the scope is amended here rather than the plan re-planned.

1. **The version stamp BUMPS 0.1.0 → 0.2.0**, in `Cargo.toml` and `tauri.conf.json` together. The scope had
   this as an open question; it is now a step. Consequence recorded: the stamp reaches `service.version` on
   every self-obs line and `clientInfo.version` on the MCP `initialize` wire, so the recorded pass describes
   itself as the version it ships.
2. **The SLO pass drives bounded in-lane live legs** through the shipped release binary — three legs chosen
   for attainable tiers with real margin — rather than the composed `run --live` suite, whose auto-resolve
   leg test-plan records as not run-stable even under the stretched bootstrap posture.
3. **CARRY C stays in this chunk** as a bounded cause investigation, per the route entry's own default. Not
   a commitment to a fix: whatever is unsettled routes to residuals with its measurement. a11y-plan makes the
   SR leg supplemental and never sole, so no release claim rests on `S3-04` either way.

## Discovered in planning, recorded not fixed

**The ledgered P-079 tier defect is one member of a class of eight.** Measured over all 36 scenario TOMLs
(summed `^gap_ms` against the closed tier deadlines), and validated against three independent live
measurements from the previous chunk where summed phases predicted `latency_ms` to within ~200 ms.
17 scenarios declare a tier smaller than their own phase duration, but they are three different things:
**9 are accepted-deliberate** (`<90s` as the "closest honest bucket", already operator-ratified in
`matrix#v2-12`'s notes — `<90s` is the largest bucket in a closed set), **2 are mis-declared beyond every
tier**, and **6 are mis-declared where a larger tier would hold** — one of those six being the already-
ledgered `constellation-severity-live-wiring`. Scenario calibration is not release work; the finding is
handed to wrap's route-resolve to extend the existing residual, and this chunk's legs deliberately exclude
the known-unattainable scenario.

## Capability-claim closure

- **`v2-21`** — the `[inferred]` reachability tag is DROPPED. Measured at HEAD: `requirements.md` declares
  32 `v2-NN` capabilities, `verification-matrix.json` holds 32 entries, the id sets are **equal**, all ids
  unique, and zero ids match `^P-\d+$` — the acceptance already holds, and an enumeration `/implement`
  itself can write satisfies claim-reachability. Design point for the plan: the matrix path is
  version-scoped, so a gate must resolve the active version rather than baking `conductor-0.2.0`.
- **CARRY A's mechanism** — the `[inferred]` tag is DROPPED, re-derived true at HEAD: coordinates correct,
  the three helpers verified private with no `lib.rs` re-export, and the code-graph impact query bounds the
  race to **exactly the two named tests** (`log_panic` has 3 callers — production plus those two;
  `take_hook` has exactly 2 sites). The remedy space narrows to a shared in-code guard, because test-plan
  `:538`'s "never serializing the file" is parenthetically scoped to a RUNNER knob
  (`--test-threads=1` / a profile knob), not to an in-code mutex.
