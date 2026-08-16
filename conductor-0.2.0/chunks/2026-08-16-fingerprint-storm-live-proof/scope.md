# Scope — Fingerprint-storm live proof

**Marker:** `2026-08-16-fingerprint-storm-live-proof`
**Version:** conductor-0.2.0 · **Epoch 3** — Live proof: the five families
**Working entry:** _fingerprint-storm live proof — identity triple, storm cue thresholds and
exactly-one-incident coalescing (P-017, P-018, P-074)_

## What this chunk builds

The first **family live proof** of the version: `scenarios/fingerprint-storm.toml` driven end-to-end against
a live deterministic-L4 Pulse, with P-017 (fingerprint identity), P-018 (storm cue thresholds) and P-074
(exactly-one-incident coalescing) asserted on read-back surfaces that actually carry the evidence.

The predecessor chunks cleared the path: the ingest→fingerprint gap was Conductor-side and is fixed, and
`conductor preflight` reached `ready:true` for the first time on 2026-08-16. This chunk is the first that
can *use* that green gate for a family rather than for the canary.

Three deliverable strands, per the working entry and the operator's phase directive:

1. **Re-derive the scenario's identity arithmetic against the shipped derivation** — the declared
   variant mix no longer produces the storm it claims (see Scope premises).
2. **Retire two stale identity claims in source/config** — the scenario's header prose and the
   `FingerprintVariant::PathVariant` doc comment both still assert path-insensitivity, which the shipped
   derivation's own tests falsify.
3. **Run the live leg and assert on surfaces that exist** — log-harvest for identity/tier, MCP read-back
   for incident count + freshness.

## Boundaries

- **Pulse stays read-only at `d090314`.** No SUT-side change, no Pulse rebuild, no fixture edit. Every
  fact about Pulse is read from its committed source or measured from its own logs.
- **Deterministic L4 only.** The leg runs with `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`; this proves the
  pipeline carries a canned interpretation end to end, never that the interpretation is trustworthy (the
  recorded 0.3.0 deferral — `.andromeda/residuals.md`). No claim about model quality is in scope.
- **No new fault primitives and no new emit families.** The `FingerprintVariant` primitive already ships;
  this chunk re-shapes how a scenario *composes* variants, and corrects a doc comment — it does not add a
  variant kind unless the P4 design call requires one.
- **Not a CI gate.** A live-Pulse leg is an operator/local gate by construction (`architecture.md` §CI/CD);
  whatever lands in nextest must be green without a live Pulse.
- **The other four families are out of scope** — error-baseline-spike, restart-suppression, pii-scrub and
  connection-lifecycle are their own working entries with their own matrix caps.

## Surfaces and contracts touched

- `scenarios/fingerprint-storm.toml` — the scenario under proof: header prose, `p_ids`, the two
  `[phases.emission]` variant mixes and occurrence counts, the `[[expected]]` block.
- `crates/conductor-emit/src/exception.rs` — the `FingerprintVariant` primitive and the shipped blake3
  derivation; the stale `PathVariant` doc comment lives here.
- `crates/conductor-core` — `UNBACKED_AUTO` and `check_scenario_backing` (the exact-set gate P-074 must
  cross).
- `crates/conductor-verify` — the read-back client and the pinned live key-diff baseline in
  `tests/readback.rs`.
- `conductor-0.2.0/verification-matrix.json#v2-11` — the cap this chunk targets.

## Scope premises, closed at P3 (see `research.md`)

Everything below marked **VERIFIED (operator)** was measured and cited by the operator in the phase
directive. Per the standing rule on dictated cross-project citations, **P3 re-verifies each SUT-side line
reference against the SUT repo at `d090314` before the plan rests on it** — this costs one pass and is not
optional.

- **[premise-corrected: the component facts hold, the conclusion does not — cumulative base-fp count is 12
  within the 30s window, so Autonomous DOES fire (`research.md` §The central finding)]** — every fact the
  directive cites verified true at `d090314`: `PathVariant` diverges (pinned by
  `relative_path_is_fingerprint_significant`), `Identical`/`Line` share the base fp, thresholds are `>=5`
  Suggested (`storm.rs:73`) / `>=10` Autonomous (`:78`) within a 30s sub-window (`:69`, compared at `:245`),
  and variants cycle round-robin per occurrence (`dispatch.rs:102`). But both phases pace across their own
  12 000 ms gap, so the scenario spans ≈24 s — inside the 30 s window — and the base fingerprint accumulates
  **4 + 8 = 12** occurrences, clearing the Autonomous threshold. The directive's "8 cumulative" counts phase
  2 in isolation; the detector counts by wall-clock window. **The leg is not dead on arrival.** What IS
  broken: the scenario declares ONE fingerprint and produces TWO (base ×12, path ×6); neither phase boundary
  lands where its name claims (phase 1 ends at 4 base-fp, so ≥5 Suggested never fires there); and the path
  fingerprint raises its OWN Suggested cue at 6 — a second cue no document names, which makes P-074's
  exactly-one-incident hold by accident rather than by design.
- **Option space for the P4 design call (recorded, not prescribed — the operator explicitly left this to
  P3/P4):** re-shape `PathVariant` to an **absolute**-path change, which `normalize_frame` strips, yielding
  the SAME fingerprint — this matches the product's real semantics *and* preserves the triple's teaching
  value, with a relative-path variant then asserting DIFFERENT separately · or change the storm phases'
  variant mix · or adjust the counts. The operator notes the **product-faithful direction is the first**:
  P-017 is "verified THROUGH the storm", so the variant arithmetic **is** the P-017 assertion.
- **VERIFIED at P3 — the option space carries a decisive cost asymmetry, and the operator's preferred
  direction collides with nothing.** No committed golden records event-attribute values (`span_shapes` keeps
  event NAMES and span attribute KEYS only; a grep for `exception.type`/`ValueError`/`alt/module` across all
  six `.snap` files returns 0), so **changing the variant mix at constant counts is golden-free** while
  **changing occurrence counts breaks four snapshots across two crates**. An absolute path IS stripped by
  `is_absolute_path_start`, passes Conductor's redaction (which anchors only on drive-letter / `/home/` /
  `/Users/`), and leaves `stacktrace_carries_no_absolute_host_path` green. The one consequence to plan for:
  that reshape retires what `relative_path_is_fingerprint_significant` pins, so keeping the relative-path
  narrowing asserted needs a separate variant member.
- **VERIFIED (operator, and WIDENED at P3) — the stale twin is not one site but FOUR.**
  `crates/conductor-emit/src/exception.rs:85-86` (`FingerprintVariant::PathVariant`) does claim "different
  source PATH per frame => SAME fingerprint (path-insensitive)" — confirmed. Research found three more the
  directive did not name: `exception.rs:40-41` (`Frame`'s struct doc: "`file` and `line` are deliberately
  fingerprint-INSENSITIVE"), `exception.rs:45` (the `file` field's own doc), and
  `crates/conductor-core/src/phase_spec.rs:149` (`FingerprintVariantSpec::Path`: "Different source path ⇒
  same fingerprint (path-insensitive)"). The module header (`:8-11`) and the `fingerprint()` doc were
  already corrected by the predecessor chunk — these four were missed. All ride this chunk.
- **VERIFIED (operator) — identity cannot be asserted through read-back under deterministic L4.**
  `fingerprint_refs` and the slice's `fingerprints[]` are **always empty**
  (`deterministic_inference.rs:35` → `markdown.rs:286` → `tools.rs:436`). What *does* carry identity
  evidence is **Pulse's own log lines**, harvested from `{data_dir}/logs/agent-latest.jsonl.<date>` since
  the GUI subsystem makes the console tee yield 0 bytes; slice by a pre-leg line count. **P3 confirmed which
  fields ride which line, and only one of the three carries identity:** the storm-DETECTED line
  (`storm.rs:342-348`) carries `cue_kind` · `severity_hint` · `occurrence_count` · `window_seconds` ·
  **`fingerprint_hex`** (an 8-char lowercase prefix, `:450`, pinned at `:682`); the emit line (`:353-357`)
  and the metric line (`:360-364`) carry `severity_hint` + `occurrence_count` but **no fingerprint**. So the
  P-017/P-018 assertions are planned on **log-harvest for identity/tier** and **MCP for incident count +
  freshness** (P-074's exactly-one).
- **VERIFIED — the assertion surface is a new capability, not a configuration.** No Conductor code reads any
  SUT log today: every `agent-latest.jsonl` reference under `crates/` is Conductor's **own** artifact
  (`conductor-cli/src/paths.rs:89`, `conductor-core/src/obs.rs:59`, plus test files). P4 must settle where
  the reader lives, whether it is a test-only affordance or a shipped seam, and how it stays inside the
  artifact-hygiene rule — arch records that a tenth workspace member would be a registered architectural act.
- **VERIFIED and sharpened — the `[[expected]]` `Contains "RetryStorm"` token grades MCP read-back text,
  and the envelope's `fingerprints[]` cannot be populated on this leg.** `observe` composes
  `observation.text` from `list_text(&list)` plus each incident's `markdown` (`extract.rs:90-100`), which is
  what `Contains` grades. But `observation.fingerprints` is fed solely by `fingerprint_refs(&slice)`
  (`extract.rs:114`) and flows to the envelope at `lib.rs:413`/`:456` — so under deterministic L4 the field
  is always `Some([])`. **test-plan §6 requires it populated for this scenario; that requirement cannot hold
  on a deterministic-L4 live leg** and is an Expected amendment at wrap, never a silent redefinition.
- **VERIFIED — `slo_tier = "<20s"` and `seed = 4317017` are calibration points this leg can measure for the
  first time.** The scenario header flags the 30s-window cadence and the `<20s` tier as Epoch-8 calibration
  points; any re-calibration must stay inside the closed `<5s`/`<20s`/`<90s` set.

## Folded annotations from the working entry

- **CARRY (from `2026-08-09-in-lane-sut-scenarios`) — `P-074` is in `conductor_core::UNBACKED_AUTO`**
  (**[premise-corrected: NINE entries, not ten — `drift.rs:61-63` reads `P-031, P-033, P-034, P-039, P-041,
  P-042, P-043, P-044, P-074`, so the coverage roll-up's qualifier moves 9 → 8]**) and **this entry is its
  named owner**. `check_scenario_backing` is
  exact-set in BOTH directions, so naming `P-074` in a scenario's `p_ids` and removing it from the pin must
  land in **ONE commit** or the gate fails either way. `fingerprint-storm.toml` already exists naming
  P-017/P-018 — adding `P-074` to its `p_ids` is what backs the claim. (Re-confirmed in the operator's
  directive item 4.)

- **CARRY (from `2026-08-13-first-live-green-preflight`) — two thirds of the read-back key-diff are still
  un-retired, and this is the first entry that can retire them.** The live leg witnessed exactly one shape,
  `query_incident_list returned keys [items, next_cursor, total]`, matching the committed baseline in
  `conductor-verify/tests/readback.rs::the_read_back_key_sets_are_pinned_as_the_live_diff_baseline`
  exactly — because `retrieve_report` and `retrieve_telemetry_slice` are only reached with a **non-empty
  corpus**, which no arm produced. Their readers (`markdown` · `degraded_mode` · `span_refs` ·
  `fingerprint_refs` · `items[].{id|incident_id,status,severity,title}`) therefore remain **stub-proven
  only**, and every reader degrades to empty on an unrecognized shape rather than erroring — so a live
  field-name divergence there is **silent** and reads downstream as an ordinary `Blocked` or a failed
  `Contains`. The witness is already wired: each tool's key set is logged once per pass on the existing
  `verify.readback.observe` / `verify.readback.preflight` spans. **Diff those two key sets against the
  pinned baseline BEFORE trusting any measured verdict from this family**, and land any divergence as a
  finding plus a stub test, never a silent reader fix.

- **CARRY (from `2026-08-14-canary-fingerprint-feed-capture`) — superseded on its blocker claim, live on
  its telemetry-reading rules.** Its "third blocker" (the ingest→fingerprint gap) is retired by the CARRY
  below. What still stands is **how to read the storm detector's telemetry**:
  `tracked_fingerprints_count` is a **60s-windowed gauge over DISTINCT fingerprints, sampled at a 15s tick
  AFTER eviction**, so a healthy identical-fingerprint storm reads **1**, never its occurrence count, and a
  late sample reads 0 on a healthy path; the window-immune discriminators are the cumulative
  `storms_detected_total` (which discriminates presence at BOTH tiers) and `fingerprints_evicted_total`,
  and `severity_hint` names the tier. **Never read that gauge as evidence of absence.** Evidence:
  `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/fingerprint-feed-verdict.md` §5.

- **CARRY (from `2026-08-16-canary-fingerprint-derivation-aligned`) — the third blocker is RETIRED; the
  feed is healthy and preflight is green.** The gap was Conductor-side (a constant span identity against
  Pulse's `spans` PRIMARY KEY), fixed at the predecessor chunk; the 2026-08-16 leg measured 27
  `duckdb.append` lines with ZERO `reject_reason` and `severity_hint: "autonomous"` at
  `occurrence_count: 10`, and `conductor preflight` reached `ready:true`. **This family is no longer
  blocked on Pulse.** Two things this entry owns:
  1. **`scenarios/fingerprint-storm.toml`'s header prose states the P-017 claim measurement falsified** —
     it says the fingerprint is "INSENSITIVE to source path + line"; the **path half is FALSE**. Pulse's
     `normalize_frame` strips **absolute** paths only, and Conductor's frames are relative by construction
     (guarded by `stacktrace_carries_no_absolute_host_path`), so a relative-path change **is**
     identity-significant; the "first 3 normalized stack frames" half of that same sentence is correct and
     is the second narrowing. Source/config is not wrap's to edit (the `scheduler.rs` precedent), so the
     fix rides this entry. **The operator's directive extends this to a second stale claim in the same
     header** — "the identical / path-variant / line-variant triple all share ONE fingerprint" — so the
     header prose carries **two** falsified statements, not one.
  2. **The identity triple's expectations must be re-derived against the shipped derivation** —
     `FingerprintVariant::PathVariant` now yields a DIFFERENT fingerprint, so a scenario asserting the
     triple aggregates to one storm must not include a path variant among the same-fingerprint members.
  Both narrowings are recorded in `architecture.md` §Established Decisions [Read-Back Dependency Posture]
  and pinned by `crates/conductor-emit/src/exception.rs` unit tests.

- **Operator recipe for reaching a live leg (directive item 5) — SIX items.** **[premise-corrected: my P3
  "FIVE items" reading was a narrow-basis claim. The working-route entry does say five, but it was authored
  2026-08-13; `.claude/rules/verification-harness.md:47` carries the accumulated chain and adds a sixth at
  2026-08-14. The route entry is the stale citation; the rules file is authoritative.]**
  (1) sidecar built AND on PATH; (2) `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` + `ANDROMEDA_PULSE_DATA_DIR`
  exported in the launching shell; (3) **`ANDROMEDA_PULSE_MCP_ENABLED=true` in Conductor's OWN
  environment** — `spawn.rs` passes only the data dir via `.env(...)`, so inheritance is the sole channel,
  and without it every arm measures the read-back-unreachable path; (4) poll budget at or above the contract
  floor (`agent-run boot` derives it); (5) `pulse-app` launched from a cwd OUTSIDE this repo, with Conductor
  run from its repo root (all four `contracts/*.toml` resolve cwd-relative); (6) **paired
  `RUST_LOG=info,conductor_emit=debug` on the boot/scenario legs** to surface the `emit.batch` wire-shape
  witness — the bare per-target form replaces the default filter and fails
  `agent_mode_routes_self_obs_to_the_log_file_not_stderr`, and neither form may ride an invocation that also
  runs the test suite. Plus the operator's **fresh data dir**; all Pulse reads remain read-only at `d090314`.

- **PREREQ — re-check `cargo audit` (24th pin at this chunk; standing deferral since
  `2026-08-08-sut-capability-manifest`).** Ratified by the operator at the
  `2026-08-10-workspace-key-divergence-probe` wrap, so it re-pins silently — no further ratification HALT.
  Fault class: **advisory-DATABASE** — byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` at true
  exit 1, re-measured on cargo-audit 0.22.2 (the latest published), so **no released tool version can read
  the DB and there is no floor to raise**. Basis re-verified at the previous chunk: `Cargo.lock` moved but
  only by dependency EDGES inside an existing entry, with zero new `[[package]]`, so the audit surface was
  unchanged. **`cargo deny check advisories bans licenses sources` is the sole coverage and must be
  VERIFIED green, never assumed.** Do NOT raise a floor, do NOT add a `deny.toml` ignore, do NOT edit CI.
  Close the deferral the moment it parses. Full rationale:
  `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

## Capability this chunk targets

`verification-matrix.json#v2-11` — _fingerprint-storm family live-proven_ (`dynamic-external`, currently
`planned`, `chunk: null`; previously DECLINED by `2026-08-14-canary-fingerprint-feed-capture` and returned
to the pool). Its acceptance as written asserts that "the identity triple resolves to one fingerprint" —
**whose truth depends on the P4 variant-shape design call above**, since under the shipped derivation the
triple as currently declared does not. The claim-exit invariant (concretize without weakening) is
adjudicated at P5, not here.
