# Codebase Research — 2026-08-15-canary-storm-autonomous-band

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 9 · **Graph queries:** 1 (`rows: 5`, `db_state: regenerated`)
- Two repos: Conductor (the change) and Pulse at HEAD `d090314` (the SUT facts the change is sized against).

## Files inspected

**Conductor**
- `crates/conductor-run/src/lib.rs` (105–250) — `CANARY_STORM_COUNT` (:111) + its doc comment (:109-110), `canary_spec`, `emit_canary_storm` (:128-138), `canary_gate`, `emit_canary` (:204-215), `warm_up_canary_service` (:221-236), `canary_poll` (:243-).
- `crates/conductor-run/tests/canary_wire.rs` (95–135, plus the graph's four reference lines) — the constant's assertion sites.
- `crates/conductor-emit/src/exception.rs` (128–158) — `exception_trace_request`: exactly ONE `root` span per request, carrying ONE `exception` event.
- `crates/conductor-core/src/load_envelope.rs` (174–330) — `phase_breach`, `check_load_envelope` (:310), `LoadEnvelope::classify` (:224).
- `contracts/pulse-load-envelope.toml` — the three terms.
- `contracts/pulse-run-contract.toml` — `[incident_formation]` + the `load-envelope` term.
- `crates/*/tests/snapshots/` — the six committed goldens.

**Pulse (SUT facts, HEAD `d090314`)**
- `crates/triage/src/pattern/storm.rs` (65–95, 240–285) — thresholds, the detection sub-window, and the emit ladder.
- `crates/triage/src/cue/evaluate.rs` (147–185) — the sole `BootstrapState::Ready` gate.
- `crates/triage/src/cadence/coordinator.rs` (380–400) — the Tier-1 Autonomous-only arm.
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (60–95) — post-alignment workspace-key resolution.
- `andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/premise-check.md` — the live two-run measurement.

## Graph impact
- **`CANARY_STORM_COUNT`** — `rows: 5` total references: four in `crates/conductor-run/tests/canary_wire.rs` (:19, :100, :129, :181) and one in `crates/conductor-run/src/lib.rs` (the `for i in 0..CANARY_STORM_COUNT` loop). **Leaf constant** — despite being `pub`, no other crate references it, so raising it has zero cross-crate blast radius. Consulted-and-confined is the finding; no threading files to enumerate.

## Patterns detected
- **The storm is an unpaced burst** (`crates/conductor-run/src/lib.rs:134-137`): `for i in 0..CANARY_STORM_COUNT { traces.export(exception_trace_request(..., base.wrapping_add(i), spec)).await? }` — no sleep, no gap, one sequential unary gRPC export per occurrence. Contrast `warm_up_canary_service` (:231-234), which *does* `tokio::time::sleep(gap)` between its emissions.
- **One request = one span = one exception event** (`crates/conductor-emit/src/exception.rs:136-150`): a single `root` span with `vec![exception_event(spec)]`. So occurrences map 1:1 to fingerprint-bearing spans.
- **The warm-up is separate and fingerprint-free** (`lib.rs:210-212, 232`): `terms.warmup_emissions` benign `trace_request(..., "canary-warmup")` spans precede the storm. They carry no exception event, so they contribute **no** fingerprint occurrences. This is what reconciles the last capture's "nine spans" — 3 warm-up + 6 storm, of which only the 6 could ever count.
- **Pulse's emit ladder is one-shot-per-tier per window** (`storm.rs:245-285`): `should_emit_autonomous = count >= autonomous_threshold && last_emitted ∈ {None, Some(Suggested), Some(Curious)}`. A storm that crosses 10 fires Autonomous directly when unfired, or escalates from a prior Suggested — either path reaches Autonomous, so a mid-burst Suggested emission is not a trap.

## Conventions to follow
- **Doc comments must not encode a numeric rationale that can move** — the shipped comment (`lib.rs:109-110`) went stale precisely by baking `>=5 in 30s`. State the *relation* (clears the Autonomous threshold with headroom) and cite the SUT symbol, not a copied literal.
- **`Scenario`-shaped gates do not see the canary** — `check_load_envelope(&LoadEnvelope, &[Scenario])` (`load_envelope.rs:310`) and `classify(&self, &Scenario)` (:224) both key on catalog scenarios.
- **Live evidence is read from the self-obs stream**, not the emission journal; a missing `emit.batch` line means the leg never ran (`.claude/rules/verification-harness.md`).

## Files to modify
- `crates/conductor-run/src/lib.rs` — the constant (:111) and its doc comment (:109-110).
- `crates/conductor-run/tests/canary_wire.rs` — the four assertion sites follow the constant symbolically (they already assert against `CANARY_STORM_COUNT`, not a literal, so they need review rather than renumbering).
- `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` — §Re-run arm-zero closure.
- `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/fingerprint-feed-verdict.md` — §5 closure pointer.
- `conductor-0.2.0/verification-matrix.json` — the `v2-10` claim decision (P5, ledger).

## The margin arithmetic (directive 3 — stated, not assumed)

**Floor.** `count >= detector.autonomous_threshold` (`storm.rs:245`) against `DEFAULT_AUTONOMOUS_THRESHOLD = 10` (:78) ⇒ **10 suffices**. Measured live: `occurrence_count=10 severity_hint=autonomous` (premise-check §3).

**Window ceiling.** `DEFAULT_DETECTION_SUB_WINDOW_SECONDS = 30` (:69); the trigger population is occurrences within 30s before now. The storm is an unpaced loop of N sequential loopback unary RPCs. Even at a deliberately pessimistic ~10 ms per round trip, N = 12 completes in ~0.12 s — **~250× inside** the 30 s window. The window is not a binding constraint at any plausible margin.

**Load-envelope ceiling.** `max_sustained_rate_spans_per_s = 10000`, `max_sustained_storm_ms = 600000`. N = 12 spans in ~0.12 s ≈ 100 spans/s ⇒ **~100× under** the rate ceiling; the burst is ~5000× under the storm-duration ceiling. Inside by orders of magnitude on both terms.

**Chosen margin: 12.** It clears the floor by 2 (survives losing an occurrence without dropping to Suggested), and it is the project's own established "reaches Autonomous" number — `scenarios/fingerprint-storm.toml`'s second phase already uses `occurrences = 12` for exactly this transition, so the canary and the scenario agree on what Autonomous costs. Leaned per `scenarios/fingerprint-storm.toml`.

## Scope premise closure

1. **VERIFIED** — `CANARY_STORM_COUNT` is confined to `lib.rs` + `canary_wire.rs` (graph `rows: 5`). Leaf; no cross-crate blast radius.
2. **VERIFIED (with the arithmetic above)** — the storm is an unpaced burst; the 30 s sub-window is satisfied by ~250×.
3. **VERIFIED, with a refinement worth carrying** — the code basis genuinely excludes the canary (both envelope entry points take `Scenario`). *But* `contracts/pulse-run-contract.toml` carries a `load-envelope` term asserting "the warm-up and poll budget together must stay inside the proven-good bounds", with `check = "asserted"` — satisfied by construction, therefore unmeasured, the same shape that could not catch the warm-up term's own falsity. The arithmetic above confirms the assertion is TRUE with ~100× headroom, so this is a closure, not a defect — but it is now backed by a computation rather than by construction.
4. **VERIFIED** — six committed goldens (`dispatch_wire__*`, `pacing__*`, `replay__*`); none references the canary.
5. **PREMISE-CORRECTED** — see below. This is the chunk's most consequential finding.

## The corrected premise (v2-10's blocking cause was misattributed)

`v2-10`'s decline note, `architecture.md` §Occupied Resources, and `.claude/rules/verification-harness.md` all record that Pulse "gates cue evaluation on `BootstrapState::Ready`" (`crates/triage/src/cue/evaluate.rs:164`) — a 3,600 s per-service wall-clock window that `baseline_state`'s 0 rows resets on every launch — and treat that as the reason the canary storm forms no incident.

**The citation is correct; the attribution is not.** That line is the only `BootstrapState::Ready` gate in all of `crates/triage/src/` (grep, HEAD `d090314`), and it sits inside **`evaluate_service_went_silent`** — the P-014 activity-floor/silence cue. The **RetryStorm path does not pass through it**: `storm.rs:245-285` keys solely on count-in-window versus the thresholds and the `last_emitted` ladder, with no baseline or bootstrap dependency at all.

Measured confirmation (Pulse `premise-check.md`, two runs on two fresh data dirs): `baseline_state` holds **0 rows** while `triage.pattern.storm.detected` fires twice, `triage.cue.emit` ten times, and **2 active incident rows** persist. Cues fired with no baseline, because the storm cue never needed one.

**Consequence:** all three causes `v2-10`'s decline note names are dissolved — (a) the bootstrap gate never applied to this path, (b) workspace-key divergence is fixed (the app publishes its stamped key and the sidecar reads it via `read_published_workspace_key(data_dir)`, falling back to `data_dir` — `andromeda-pulse-mcp.rs:77-79`), (c) the fingerprint feed is proven healthy. The count is the remaining blocker, exactly as the entry claims.

**Corollary for the warm-up:** `warm_up_canary_service`'s stated purpose (carry the service out of bootstrap before the counted storm) is now disproved a second time and on a different axis — not merely 80× too short, but aimed at a gate the storm path never crosses. It remains harmless (3 benign spans) and **out of this chunk's scope**; recorded here so a later entry owns it rather than re-deriving it.

## Second expected amendment (wrap, not phase)

Beyond the `architecture.md:174` correction the entry already carries, `architecture.md` §Occupied Resources states the bootstrap claim too broadly — "Pulse gates cue evaluation on `BootstrapState::Ready`" reads as all cue evaluation. It gates *service-went-silent* evaluation. The same over-broad wording sits in `.claude/rules/verification-harness.md` (2026-08-10 entry, extended 2026-08-13).

## A metric-reading correction to carry

The 2026-08-14 chunk established `storms_detected_total` as the window-immune discriminator against the `tracked_fingerprints_count` gauge. That holds for **presence**, but Pulse increments it on **both** the Suggested and the Autonomous branch (`storm.rs`, the two `fetch_add` sites at :255 and :275) — so it cannot distinguish tiers. The tier is named by `severity_hint` on `triage.pattern.storm.detected`. A live leg reading `storms_detected_total: 1` must not infer Autonomous from it.

## Open questions
- none — all five scope premises are closed, and the margin is decided with its arithmetic stated.
