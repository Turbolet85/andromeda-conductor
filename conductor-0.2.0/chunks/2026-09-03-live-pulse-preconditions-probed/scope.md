# Scope — Live-Pulse preconditions probed before a leg is scheduled

**Marker:** `2026-09-03-live-pulse-preconditions-probed`
**Version:** conductor-0.2.0 · **Epoch:** 6a — Verification follow-ups
**Working entry (verbatim surface):** _Live-Pulse preconditions probed before a leg is scheduled, so an absent
SUT is named once rather than absorbed per chunk_

---

## What this chunk builds

A **non-mutating precondition probe** over the live-Pulse toolchain, whose result is a stated precondition with
a known state at scheduling time — replacing the present pattern where each chunk, each gate and each wrap
discovers the SUT's absence independently and absorbs it as a local deferral.

The probe answers three questions and reports them; it changes nothing about the SUT.

**Two decisions resolved at P4 (operator-selected), amended in here because the build statement above did not
anticipate the first and deliberately left the second open:**
1. **The probe owns subjects 1+2; the run contract gains exactly ONE term.** Research found subject 3 covered
   1-of-3, and its missing `ANDROMEDA_PULSE_MCP_ENABLED` handle fits the existing `ShellDeclaration` taxonomy
   exactly (same class as the shipped `l4-deterministic`) — so a `[[term]]` is appended to
   `contracts/pulse-run-contract.toml`, which makes preflight NAME a cause it currently swallows as an opaque
   unreachable-readback. Subjects 1 and 2 stay with the probe, because `ShellDeclaration` means "observable as
   a declaration in Conductor's own environment" and neither a TCP connect nor a `PATH` lookup is a
   declaration. `sidecar-built`'s measurably-false `asserted` rationale is **surfaced as a wrap amendment, not
   silently re-classified** — re-classifying moves what the preflight gate can block on, which is a larger
   change than this chunk's title covers. The rejected third option (a new `CheckKind` unifying everything
   into the contract) is recorded in the plan's rejected-approaches section. Note this is a **behaviour change
   to the shipped preflight gate**, and it is deliberate: an undeclared handle already breaks read-back today,
   so the term makes an existing failure legible rather than adding a new one.
2. **The probe surfaces BOTH ways.** A standalone `conductor preconditions` verb (no canary — that is the
   whole point), AND `agent-run boot` gains it as a leading arm that short-circuits before the canary. This
   applies the shipped arch precedent one rung earlier: the fifth named precondition already sits before the
   canary arms and skips the poll rather than paying it, because an unmet launch condition explains a failed
   canary. The harness command count stays **five**, so no test-plan amendment is required
   (`verification-harness.md:24` forbids a 6th without one).

### The three probe subjects (stated by the working entry; re-verified at HEAD)

1. **`127.0.0.1:4317` reachable** — is anything listening on Pulse's OTLP ingest port.
2. **`andromeda-pulse-mcp` resolvable on the inherited `PATH`** — the MCP sidecar's spawn-resolution input.
   Its absence is the failure mode `security.md` already predicts: preflight returns `[BLOCKED]` in ~2 ms with
   all four tools `absent`, which at row level is indistinguishable from a genuine SUT-side gate failure.
3. **The three `ANDROMEDA_PULSE_*` handles declared** — enumerated at HEAD as exactly
   `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_L4_DETERMINISTIC`, `ANDROMEDA_PULSE_MCP_ENABLED`
   (workspace grep over `crates/` · `scripts/` · `contracts/`: 21 / 8 / 6 occurrences respectively).

### What HEAD already covers (P3 closure — the completion check, measured against `contracts/pulse-run-contract.toml` + `conductor-core/src/run_contract.rs`)

The chunk is **not already done**, and the gap is not uniform across the three subjects. The shipped contract
carries five `[[term]]`s, and `RunContract::evaluate` lets **only** a `CheckKind::ShellDeclaration` term ever
be unmet (`run_contract.rs:153-157`; `observed_env()` at `:169` likewise yields only those terms' `env`
names). Against that:

| Subject | HEAD coverage | Finding |
|---|---|---|
| 1 — `:4317` reachable | **no term at all** | No contract term names egress reachability. `probe_egress` exists but is reached only from `execute_scenario`, i.e. after scheduling. |
| 2 — sidecar on `PATH` | a term exists, **classified `asserted`** | `sidecar-built`'s own `causes` reads "reaching term evaluation at all proves it" — but a `PATH` miss short-circuits to `[BLOCKED]` in ~0 s *before* term evaluation, so the term is satisfied by construction precisely when it would be false. It cannot catch its own falsity — the same failure shape already recorded for `warmup_ms`. |
| 3 — three `ANDROMEDA_PULSE_*` declared | **1 of 3** | Only `l4-deterministic` is a `ShellDeclaration` term (`env = ANDROMEDA_PULSE_L4_DETERMINISTIC`). `ANDROMEDA_PULSE_MCP_ENABLED` is **not a term at all**, though security-plan requires it in Conductor's own environment. `ANDROMEDA_PULSE_DATA_DIR` appears only inside `shared-data-dir`, which is `declared-not-observable` and carries **no `env` field**, so it is never read as a declaration. |

Two further HEAD facts that bound the build:
- **The sidecar spawn carries no creation flags** (`conductor-verify/src/spawn.rs:80`, `Command::new(PULSE_MCP_PROGRAM)`; no `creation_flags` / `CREATE_NO_WINDOW` anywhere in the crate), so the measured console-pane host-path channel is still open at HEAD. Subject 2 must therefore be a **`PATH` lookup, never a spawn** — a probe that spawned to test resolvability would itself open the disclosure channel this chunk explicitly does not own.
- **`run_id` and `port` are both already allowlisted** (`conductor-core/src/redact.rs:29`, `:51`), and `ServiceIdentity::resolve` mints a `run_id` when none is supplied (`obs.rs:53`) — so a scheduling-time probe satisfies obs §11's never-skip-`run_id` rule by construction, with no code change and no design fork. This closes the obs extract's open question.

## Why (the recorded cost this repays)

Source: `.andromeda/runs/2026-09-02T15-24-02-evolve-diagnose/proposals.md:548` — level candidate **L3**,
_band-aid — live-Pulse unavailability absorbed per chunk_. Coordinate re-verified at that exact line.

The diagnosis counted **5 in-epoch facts** absorbing live-Pulse unavailability one chunk at a time —
implement/smoke (`p-075`) · fix-loop (`a11y-sweep`) · gates (`a11y-sweep`) · report (`screen-reader`) ·
gates (`screen-reader`) — plus **4 more in Epoch 4**, one of them a soft-exit. Each instance is *correct*
per-instance behaviour, which is exactly why it never halts and never accumulates a visible cost.

- `hypothesis:` naming the precondition up front keeps a cold gate re-run from measuring the absence and
  false-redding, which is the treatment the light-gate rule already prescribes case by case. **Split by the
  P3 closure into its two halves:**
  - The MECHANISM half is **verified at HEAD** — absence really is misread, three ways, each recorded with
    its measurement: a `PATH` miss yields `[BLOCKED]` in ~0 s in the same row shape a genuine SUT-side gate
    failure produces, with elapsed time the only discriminator (`verification-harness.md:54`); a leg whose
    guard skips at exit 0 cannot be distinguished from a full pass by its exit code alone
    (`:58`, extended 2026-09-02); and a `boot` fired as a cheap go/no-go cost two 12-minute runs reporting
    "no hold raised" while preflight had just answered `ready:true` (`:53`, extended 2026-09-01).
  - The REMEDY half — that naming it up front is *sufficient* to prevent the false red — remains
    `[inferred]`, and unavoidably so: it is a claim about a treatment that does not exist yet, so no HEAD
    measurement can reach it. It is this chunk's own thesis, to be proven by the chunk rather than assumed
    by the plan.

## Boundaries

- **The probe REPORTS — it never launches.** Conductor does no Pulse process management by scope law
  (stated by the working entry).
- **The probe must not be, or invoke, `conductor preflight` / `agent-run boot`.** **VERIFIED at HEAD** — and
  it is a standing project rule, not an inference: `verification-harness.md:53(a)` states "NEVER run `boot`
  before `conductor run` on the same data dir", extended 2026-09-01 to bind **any leg that fires a
  preflight**, because preflight runs its OWN canary storm and Pulse dedupes against any open incident on the
  `(kind, scope, scope_id)` tuple. Measured cost: two 12-minute runs lost. The reliable shape is a QUIET
  WINDOW (≥120 s idle + a 30 s resolver tick after any preceding canary), never a `boot` immediately before.
  - **New P3 finding:** the shipped run-contract observation is *welded to the canary path* — the sole
    production caller of `observe_run_contract` is `canary_gate` (`conductor-run/src/lib.rs:316`). The
    reusable seam is one level down and pure: `RunContract::evaluate(&declared)` +
    `RunContract::observed_env()` in `conductor-core` (`run_contract.rs:153`, `:169`), which take their
    input as a value and touch no canary. The probe reuses those, never `canary_gate`.
- **Zero SUT mutation:** no OTLP emission, no incident formation, no corpus write, no MCP `tools/call` that
  changes state. This is what separates the probe from the existing readiness gate.
- **No inbound listener.** The trust boundary is unchanged: the `:4317` port-occupier remains the shipped
  binaries' sole deliberate bind, and this probe is a *client-side connectability check*, never a bind.
- **Not a verdict.** A probe result is harness-side scheduling information; it must not manufacture a
  `Verdict`/`ReportState`, and specifically must not mint a `[BLOCKED]` row for a scenario that was never run.
- **Not a Pulse-side change.** Nothing in this chunk edits or assumes an edit to `andromeda-pulse`.

## Surfaces and contracts touched

- **`conductor-emit`** — `probe_egress` (`crates/conductor-emit/src/client.rs:42`) **VERIFIED** as exactly the
  shape subject 1 needs: it connects a tonic `Channel` with the bounded `DEFAULT_CONNECT_TIMEOUT` and drops
  it, exporting no OTLP message, and a refused / unreachable / timed-out transport surfaces as
  `EmitError::Transport` (`Result::Err`) — never a verdict. Its own doc comment assigns the remaining work
  to this chunk: *"Orchestrating the gate into a run is the CLI's job."* It already carries the
  ephemeral-port test pair test-plan §5 mandates (`conductor-emit/tests/egress.rs:85` connectable-stub,
  `:93` refused), and its only production caller is `execute_scenario` (`conductor-run/src/lib.rs:487`), so
  a scheduling-time caller is additive.
- **`conductor-cli` verb surface** — `[premise-corrected: the enum carries FIVE verbs, not four —
  `Run`/`Suite`/`Report`/`Preflight`/`Coverage` (`cli.rs:49`). The four-verb reading came from a
  head-truncated orientation grep; `layout-templates.md`'s five-verb documentation is CORRECT and not
  stale.]* A probe surface distinct from `Preflight` remains the expected shape, and `Coverage` is the
  precedent for a verb that reports a classification without running a scenario.
- **`scripts/agent-run.{sh,ps1}`** — the 5-command harness (`boot` / `run` / `status` / `cleanup` / `logs`)
  at `agent-run.sh:72`; `boot` today is exactly `conductor preflight --json` under a contract-derived
  `preflight_budget_sec` timeout, with no precondition reported ahead of it. Whether the probe rides an
  existing verb or gains its own is a plan-time question, but two constraints now bound it:
  **`.sh`/`.ps1` parity is mandatory** (test-plan §3; the 2026-08-13 amendment records the two shells
  actually diverging and killing every live run), and **a 6th harness command requires a test-plan
  amendment** (`verification-harness.md:24` — "the harness is small by design"), which is a wrap-owned
  channel this chunk's phase cannot author.
- **Artifact hygiene** — the probe's output is a Conductor-generated artifact: host-path-free, no internal
  struct names, sanitized at the `anyhow` edge and through the tracing field allowlist. A `PATH` miss must be
  named without echoing the `PATH`.
- **Verdict/error wall** — a probe that cannot answer is a harness fact, not a verification outcome.

## Folded annotations

- **CONTEXT** (operator WRAP directive 2026-09-02, item 6; placement RATIFIED by the operator at the
  `2026-09-03-conductor-tauri-survivors-dispositioned` wrap — kept as placed, ahead of the entries whose legs
  need a live Pulse). The adaptation record's open point is closed; the enumerated alternatives at mint were
  folding into an existing entry, or `.andromeda/residuals.md`. **Verified:** the cited source line, the 5+4
  fact counts, the per-step attribution and the three probe subjects all reproduce at
  `proposals.md:548-575`.
- **PREREQ** (standing, external decay) — re-check `cargo audit`. **Discharged at promotion, the 48th
  consecutive reproduction:** `cargo audit` exit **1**, first diagnostic
  `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; overlap
  `cargo deny check advisories bans licenses sources` exit **0** (`advisories ok, bans ok, licenses ok,
  sources ok`). Both exits read directly from the bare command, never through a pipe. Byte-identical to the
  pinned signature ⇒ no deviation, re-pins silently; the standing deferral (since 2026-08-09, ratified at the
  2026-08-10 wrap) stands, on the same basis — the RustSec advisory DB itself will not parse, a DATABASE
  fault with no tool floor to raise. **This promotion-time reproduction is an EARLY read, not the whole
  discharge:** both gates are listed in the plan's `## Test Commands` with their disposition, so /implement
  and wrap's light gate reproduce the same signature — wrap's light gate re-runs Test Commands only, so a
  PREREQ absent from that list is one wrap cannot re-pin.

## Out of scope

- Fixing the `boot`-then-`run` self-collision itself (P3's own proposal is a *pipeline* fix to the plan
  template's LIVE-LEG rule, not a project change).
- The console-window sidecar spawn defect — it is its own markerless route entry in this epoch.
- Launching, installing or supervising `pulse-app` or `andromeda-pulse-mcp`.
- Retiring any existing deferral already recorded in a prior chunk's report.
