# Scope — Canary storm inside Pulse's Autonomous band

**Marker:** `2026-08-15-canary-storm-autonomous-band` · **Version:** conductor-0.2.0 · **Epoch 3 — Live proof: the five families**

**Working entry (intent anchor):**
> Canary storm inside Pulse's Autonomous band — the preflight canary forms a Tier-1 incident and preflight
> reaches `ready:true` against a live Pulse

---

## Why this chunk exists

Pulse is settled and proven across four chunks (`b08e10a` fingerprint-feed-capture-repair · `2961f4e`
workspace-key-alignment · `3ac3d9d` corpus-key-persistence · `d090314` tier-1-incident-path-investigation).
The last preflight-green blocker is **Conductor's own constant**: the preflight canary emits a storm sized to
clear Pulse's *cue* floor but not its *incident-forming* threshold, so no Tier-1 incident can ever form and
`conductor preflight` can never reach `ready:true` no matter how healthy Pulse is.

This chunk is the correction, plus the live leg that proves it.

## What this chunk builds

1. **The canary storm lands in Pulse's Autonomous band.** `CANARY_STORM_COUNT`
   (`crates/conductor-run/src/lib.rs:111`, currently `6`) rises past `DEFAULT_AUTONOMOUS_THRESHOLD`. The
   exact margin is **the plan's call** within two measured bounds (below).
2. **The constant's doc comment moves with it.** `lib.rs:109-110` currently states the now-insufficient
   rationale — "over Pulse's `>=5 in 30s` retry-storm floor (P-018) so the gate's incident is raised
   deterministically" — which is the artifact that encoded the wrong reasoning. It is fix scope, not cleanup.
3. **Tests follow the constant.** Known call sites: `crates/conductor-run/tests/canary_wire.rs:20,101,130,182`.
4. **The live preflight-green attempt.** An operator-gated leg against a live Pulse at its current HEAD,
   aiming at `ready:true`. This entry OWNS that attempt.
5. **Two verdict-doc closures** (record-closing appends, citing Pulse evidence paths, never copying bodies):
   - `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` §Re-run gains
     the arm-zero closure — the feed is healthy 1:1:1 on a second driver; the zero was tier-band arithmetic,
     not a defect.
   - `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/fingerprint-feed-verdict.md` gains
     its §5 closure pointer.

## The two measured bounds on the margin

Both verified first-hand in Pulse's source at HEAD `d090314`:

- **Floor:** `count >= detector.autonomous_threshold` (`crates/triage/src/pattern/storm.rs:245`) against
  `DEFAULT_AUTONOMOUS_THRESHOLD = 10` (`:78`) — so **ten occurrences suffice; the margin sits above 10, not
  above 11**.
- **Ceiling:** `DEFAULT_DETECTION_SUB_WINDOW_SECONDS = 30` (`:69`) — the trigger population is only those
  occurrences within 30s before now, so the whole raised count must land inside **one** 30s window.
- Context: `DEFAULT_SUGGESTED_THRESHOLD = 5` (`:73`); Pulse's Tier-1 coordinator accepts **only** Autonomous
  cues (`crates/triage/src/cadence/coordinator.rs:390`, with its own test at `:861-864` documenting the band).
  Measured Pulse-side at its Tier-1 chunk: 2 incidents at >=10, zero at 6.

The plan must **state the arithmetic** rather than assume the window is satisfied (operator directive 3).

## Boundaries — what this chunk does NOT do

- **The `fingerprint-storm` SCENARIO does not move.** `scenarios/fingerprint-storm.toml` is already two-phase
  by design (occurrences 6 → Suggested, then 12 → Autonomous). Only the **preflight canary** constant changes.
  Touching the scenario would destroy a deliberate two-band test.
- **No Pulse-side changes.** Pulse is settled; this chunk consumes it.
- **No scenario-family live proof.** The five families (fingerprint-storm, error-baseline-spike,
  restart-suppression, pii-scrub, connection-lifecycle) are their own Epoch-3 entries below this one.
- **No new preflight preconditions**; the gate's five named preconditions are unchanged.

## Folded annotations (from the working entry)

**PREREQ — `cargo audit` re-check.** Standing deferral since `2026-08-08-sut-capability-manifest`, ratified
at the `2026-08-10-workspace-key-divergence-probe` wrap, compact standing form, **18th pin**. Re-run it,
record the result, verify `cargo deny` ran green as the overlap. Do NOT raise the floor, do NOT add a
`deny.toml` ignore, do NOT edit CI. Close the deferral the moment it parses. (Re-verified at the
2026-08-15 route wrap: still exit 1, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, `Cargo.lock`
zero lines, `cargo deny` true exit 0.)

**CARRY — the substance.** Folded into §What this chunk builds and §The two measured bounds above.

**Expected amendment at this chunk's wrap** (from the CARRY): `architecture.md:174` encodes the now-false
"the read-back tools filter incidents by the `workspace` column (= data_dir; `andromeda-pulse-mcp.rs:74` +
`tools.rs:336`)". Since Pulse's workspace-key chunk the sidecar reads the key the **app publishes under the
data dir**; raw `data_dir` is the fallback only. This is the chunk's amendment floor at wrap, not phase work.

## Operator PHASE directives (stated intent, this invocation)

1. **The live leg runs against Pulse at its current HEAD `d090314`, with BOTH binaries REBUILT first** —
   `cargo build -p pulse-app` and `cargo build -p mcp-server --bin andromeda-pulse-mcp --features mcp-server`
   in the Pulse repo. `pulse-app`'s observability changed after the last arm (the diagnostic leaves are
   compiled into the binary), and **stale-binary evidence is the failure class this pair of projects has been
   burned by**. **Fresh data dir** for the leg.
2. **The v2-10 re-claim, if research confirms provability, runs the FULL lifecycle at P5** — premise-check
   the decline note's three causes against Pulse's evidence paths (all three dissolved), then the
   concretization preview against the pooled acceptance text. Concretize the mechanism, **never weaken the
   outcome**. A leg that unexpectedly does NOT reach `ready:true` is a **recorded result feeding
   un-claim/decline — never stretched**.
3. **Margin is the plan's call** within the two bounds, with the arithmetic stated.

## Host preconditions for the live leg (the six-item recipe)

Authoritative source is **`.claude/rules/verification-harness.md`** — the frozen capture-entry CARRY carries
only the first five; the sixth was added by the 2026-08-14 wrap's curation.

1. Sidecar `andromeda-pulse-mcp` **built and on PATH**.
2. `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` + `ANDROMEDA_PULSE_DATA_DIR` exported in the launching shell.
3. `ANDROMEDA_PULSE_MCP_ENABLED=true` in **Conductor's own** environment (inheritance is the only channel —
   `spawn.rs` passes only the data dir via `.env(...)`).
4. Poll budget at or above the run-contract floor (`agent-run boot` derives it).
5. `pulse-app` launched from a cwd **outside** this repo; Conductor always run from its repo root.
6. `RUST_LOG=info,conductor_emit=debug` **on the `boot` leg only** — the paired form, never bare, and never
   on an invocation that also runs the test suite.

**Evidence-reading trap to respect:** with the sidecar unreachable the preflight blocks before the canary
gate runs, so the canary never emits — a MISSING `emit.batch` witness line is *leg-never-ran* evidence, not
wire evidence. Check the binary before drawing any conclusion from an absence.

## Premises — CLOSED at P3 (see `research.md` §Scope premise closure)

- **VERIFIED** `CANARY_STORM_COUNT` has no consumers beyond `lib.rs` and `canary_wire.rs` — code-graph
  `rows: 5` (four in `canary_wire.rs`, one in the `lib.rs` loop). Leaf constant, zero cross-crate blast
  radius despite being `pub`.
- **VERIFIED** The canary emits its storm as an unpaced burst (`for i in 0..N { export().await }`, no
  sleep — unlike the warm-up, which does sleep), one span with one exception event per occurrence. At
  N = 12 the burst completes ~250× inside Pulse's 30s detection sub-window. Arithmetic stated in
  `research.md` §The margin arithmetic.
- **VERIFIED, refined** The envelope's code basis genuinely excludes the canary — `check_load_envelope`
  takes `&[Scenario]` and `classify` takes `&Scenario`. Separately, `pulse-run-contract.toml` carries a
  `load-envelope` term asserting the canary stays inside the bounds with `check = "asserted"`
  (unmeasured by construction). The arithmetic confirms it holds with ~100× headroom on the rate term,
  so the assertion is now backed by a computation rather than by construction.
- **VERIFIED** No golden pins the canary count — six committed snapshots (`dispatch_wire__*`,
  `pacing__*`, `replay__*`), none canary-related.
- **`[premise-corrected: the only BootstrapState::Ready gate in crates/triage/ is on the P-014 silence
  cue (evaluate.rs:164, inside evaluate_service_went_silent); the RetryStorm path has no baseline or
  bootstrap dependency (storm.rs:245-285), and Pulse measured 2 incidents with baseline_state at 0 rows]`**
  v2-10's provability: all three causes its decline note names are dissolved, and the bootstrap-gate
  cause was **misattributed** rather than merely outgrown. The count is the remaining blocker, as the
  entry claims. The live leg is the proof; a leg that does not reach `ready:true` is a recorded result
  feeding un-claim/decline, never stretched.

## Corrections this chunk surfaced (owned at wrap, not phase work)

- **Second expected amendment:** `architecture.md` §Occupied Resources states "Pulse gates cue evaluation
  on `BootstrapState::Ready`" too broadly — it gates *service-went-silent* evaluation. The same
  over-broad wording sits in `.claude/rules/verification-harness.md` (2026-08-10, extended 2026-08-13).
- **Metric-reading correction:** `storms_detected_total` increments on BOTH the Suggested and Autonomous
  branches, so it discriminates storm PRESENCE, not TIER; `severity_hint` names the tier.
- **Out of scope, recorded so it is not re-derived:** `warm_up_canary_service`'s stated purpose is now
  disproved on a second axis — it aims at a gate the storm path never crosses. Harmless, not this
  chunk's to change.

## Surfaces touched

| Surface | Nature |
|---|---|
| `crates/conductor-run/src/lib.rs` | the constant + its doc comment |
| `crates/conductor-run/tests/canary_wire.rs` | test call sites follow the constant |
| `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` | §Re-run closure append |
| `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/fingerprint-feed-verdict.md` | §5 closure pointer |
| `conductor-0.2.0/verification-matrix.json` | `v2-10` claim decision at P5 (ledger, not spec) |
| `.andromeda/architecture.md` §Occupied Resources | expected amendment **at wrap**, not here |
