# Codebase Research — 2026-08-16-fingerprint-storm-live-proof

## Scope
- **Depth:** deep · **Reads:** 21 targeted (13 Conductor, 5 SUT at `d090314`, 3 artifact/snapshot) · **Globs/Greps:** 12 · **Code-graph queries:** 1

The chunk's central premise is arithmetic over a shipped derivation, so research prioritized (a) re-deriving
that arithmetic from source rather than accepting it, (b) verifying every dictated SUT citation against the
SUT repo, and (c) sizing the blast radius of each option in the P4 design space.

## Files inspected

**Conductor**
- `crates/conductor-emit/src/exception.rs` (full, 459 lines) — the derivation, `FingerprintVariant`,
  `normalize_stacktrace`/`normalize_frame`/`is_absolute_path_start`, and the six pinning unit tests.
- `crates/conductor-core/src/phase_spec.rs` (`:29`, `:63`, `:116-156`, `:338-351`) — `EmissionShape::Exception`,
  `FingerprintVariantSpec`, `MAX_OCCURRENCES = 10_000`, the garde bounds.
- `crates/conductor-core/src/drift.rs` (`:40-95`) — the `UNBACKED_AUTO` pin and `check_sut_drift`.
- `crates/conductor-core/src/coverage.rs` (`:162-166`, `:248-251`) — P-074's classification row.
- `crates/conductor-run/src/dispatch.rs` (`:101-107`, `:205-224`) — the exception dispatch branch,
  `base_exception()`, `wire_variant()`.
- `crates/conductor-run/src/lib.rs` (`:405-460`) — envelope assembly from the observation.
- `crates/conductor-run/tests/dispatch_wire.rs` (`:126-152`, `:225-255`, `:425-440`) — `span_shapes`, the
  `storm_fixture()` loader, the two stream-golden assertions.
- `crates/conductor-timeline/src/scheduler.rs` (`:163-190`) — `paced_slices`, `jittered_gap`.
- `crates/conductor-timeline/tests/pacing.rs` (`:32`) — second consumer of the same fixture.
- `crates/conductor-verify/src/extract.rs` (`:70-120`, `:206-216`) — `observe`, `log_observed_keys`.
- `crates/conductor-verify/tests/readback.rs` (`:117-147`) — the pinned live-diff baseline.
- `crates/conductor-core/src/redact.rs` (`:74`, `:122-128`) — the host-path value scrubber.
- `contracts/pulse-load-envelope.toml` (full) · `scenarios/fingerprint-storm.toml` (full).

**SUT (`andromeda-pulse` @ `d090314`, read-only)**
- `crates/triage/src/pattern/storm.rs` (`:60-90`, `:238-252`, `:335-365`, `:450`, `:682`).
- `pulse-app/src/deterministic_inference.rs` (`:28-42`) · `crates/interpretation/src/markdown.rs` (`:282-292`)
  · `crates/mcp-server/src/tools.rs` (`:430-440`).

## Graph impact
- **`FingerprintVariantSpec`** — references only in `crates/conductor-run/tests/dispatch_wire.rs:238-240`
  and `:402` outside its defining crate; the enum's blast radius is the dispatcher's `wire_variant` match
  (`dispatch.rs:216-224`) plus the TOML deserialization. Adding a member is additive and touches exactly
  those two sites plus the garde surface — no cross-crate threading beyond `conductor-core → conductor-run`.
- Trace: `.andromeda/runs/2026-08-16T12-26-07-phase/tree-query-2026-08-16-fingerprint-storm-live-proof.json`

## The central finding — the operator's component facts hold, the conclusion does not

Every fact in the phase directive's item 1 verified **true**, and its conclusion verified **false**.

**Verified true:**
- `PathVariant` diverges from the base — pinned by `relative_path_is_fingerprint_significant`
  (`exception.rs:368-372`); `derive` rewrites each frame's `file` to `alt/module_{i}.rs`, a RELATIVE path,
  and `normalize_frame` strips absolute paths only (`exception.rs:218-221`, `:262-270`).
- `Identical` and `LineVariant` share the base fingerprint — pinned by
  `line_insensitive_variants_match_the_base` (`exception.rs:351-362`).
- Pulse's thresholds at `d090314`, all four citations correct: `DEFAULT_DETECTION_SUB_WINDOW_SECONDS = 30`
  (`storm.rs:69`), `DEFAULT_SUGGESTED_THRESHOLD = 5` (`:73`), `DEFAULT_AUTONOMOUS_THRESHOLD = 10` (`:78`),
  compared `count >= detector.autonomous_threshold` (`:245`).
- Variants cycle strictly round-robin per occurrence, restarting each phase —
  `variants[point.occurrence as usize % variants.len()]` (`dispatch.rs:102`), and `paced_slices` numbers
  occurrences `0..emissions` within each phase (`scheduler.rs:169-177`).
- `base_exception()` is a fixed 2-frame spec (`dispatch.rs:205-214`); the fingerprint is a pure function of
  content, seed-independent.

**The arithmetic, re-derived:**

| | occurrences | base fp | path fp |
|---|---|---|---|
| phase `storm-same-fp-6x` (gap 12 000 ms) | 6 → `I,P,L,I,P,L` | **4** | 2 |
| phase `storm-same-fp-12x` (gap 12 000 ms) | 12 → 4 full cycles | **8** | 4 |
| **cumulative** | 18 over ≈24 s | **12** | **6** |

Both phases pace their occurrences *across* their own gap, so the whole scenario spans ≈24 s (`jitter_ms = 50`).
Pulse's detection sub-window is **30 s**, so at the final emission every one of the base fingerprint's 12
occurrences is still inside the counting window: `12 >= 10` ⇒ **the Autonomous cue DOES fire and a Tier-1
incident can form.** The directive's "8 cumulative in phase 2" counts phase 2 in isolation; the detector
counts by wall-clock window, not by phase.

**What IS broken — three defects, none of which is "no incident forms":**
1. **The scenario declares one fingerprint and produces two.** The header's "the identical / path-variant /
   line-variant triple all share ONE fingerprint, so a mixed-variant storm counts toward a SINGLE threshold"
   is false: base ×12 and path ×6.
2. **Neither phase boundary lands where its name claims.** `storm-same-fp-6x` ends with **4** base-fp
   occurrences, so the ≥5 Suggested cue does not fire at that boundary; it fires mid-phase-2. The P-018
   assertion the scenario exists to make (6 → Suggested, 12 → Autonomous, at the phase boundaries) is not
   what the SUT sees.
3. **A second cue fires.** The path fingerprint independently reaches 6 ≥ 5 and raises its own Suggested
   cue. Pulse's Tier-1 coordinator accepts Autonomous cues alone, so this probably does not become a second
   incident — but P-074's "exactly ONE incident" would then hold by accident, not by design. **No document
   names this second cue.**

## The decisive cost asymmetry in the P4 option space

`span_shapes` captures `batch` · `name` · `trace_id` · `span_id` · `parent_span_id` · `status` · **event
NAMES** · **span attribute KEYS** (`dispatch_wire.rs:126-142`) — never attribute values. Confirmed against
the committed artifacts: every span in `dispatch_wire__storm_stream_seed_4317017.snap` reads
`events: ["exception"], attribute_keys: []`, the file holds **18** spans (6 + 12 ✓), and a grep for
`exception.type` / `ValueError` / `alt/module` across **all six** committed `.snap` files returns **0**.

Therefore:
- **Changing the variant MIX at constant occurrence counts is golden-free** — variant identity lives in event
  attribute values, which no golden records.
- **Changing occurrence COUNTS breaks four committed snapshots across two crates** —
  `dispatch_wire__storm_stream_seed_{4317017,7}` and `pacing__fixture_emission_stream_seed_{4317017,7}` —
  because span/trace ids are seeded per call and the count shifts the whole sequence. `scenario.rs:567` loads
  the same fixture for a third (non-golden) assertion.

**The operator's preferred direction is feasible and collides with nothing.** Reshaping `PathVariant` to an
ABSOLUTE path is stripped by `is_absolute_path_start` (`exception.rs:262-270`: leading `/` + a path char, or
`X:\`/`X:/`) ⇒ same fingerprint as the base. A synthetic non-host absolute prefix (e.g. `/opt/...`) also
passes Conductor's redaction untouched — `redact_value` anchors only on drive-letter, `/home/`, `/Users/`
(`redact.rs:122-128`) — and `stacktrace_carries_no_absolute_host_path` (`exception.rs:450-458`) asserts over
`base()` only, checking those same three markers. One consequence to plan for: that reshape retires what
`relative_path_is_fingerprint_significant` currently pins, so preserving the relative-path narrowing needs a
separate variant member (a `FingerprintVariantSpec` addition — additive, two call sites, plus its garde rule).

## Patterns detected
- **Variant cycling is index-modulo, per phase** (`dispatch.rs:102`): occurrence counts and variant-list
  length interact multiplicatively — a 3-variant list over 6 occurrences gives exactly 2 each.
- **Read-back key-set witness already covers all three tools** (`extract.rs:84`, `:96`, `:111` via
  `log_observed_keys`, emitted at `info` on `message`, `extract.rs:214-216`); `retrieve_report` /
  `retrieve_telemetry_slice` are witnessed only for the FIRST incident (`i == 0`).
- **Envelope `fingerprints[]` is read-back-fed only** (`lib.rs:413`, `:456` from
  `observation.fingerprints`, filled at `extract.rs:114` from `fingerprint_refs(&slice)`).
- **Pinned live-diff baseline exists for all three tools** (`readback.rs:133`/`:139`/`:143`) but is asserted
  against `serve_stub` alone — exactly the "stub-proven only" gap the CARRY describes.
- **Pulse's storm-detected log line carries the fingerprint** (`storm.rs:342-348`): `cue_kind`,
  `severity_hint`, `occurrence_count`, `window_seconds`, **`fingerprint_hex`** — an **8-char lowercase
  prefix** (`fingerprint_to_hex_prefix`, `:450`, pinned by a test at `:682`). The two sibling lines
  (`:353-357` emit, `:360-364` metric) carry `severity_hint` + `occurrence_count` but **no fingerprint**.

## Conventions to follow
- **Committed SUT-facing contract read**: fixed `default_path()` → `resolve_under`, no `CONDUCTOR_*`
  override, read faults carry `e.kind()` only (`contracts/*.toml` precedent).
- **garde bounds on any new spec member**: `MAX_OCCURRENCES = 10_000` (`phase_spec.rs:29`), `dive` never
  `skip`, non-empty variant lists.
- **Snapshot regeneration is never interactive** — `cargo insta review` is banned (test-plan §11).
- **Load envelope has enormous headroom**: asserted terms are 10 000 spans/s and 600 000 ms per emitting
  phase; the storm's phases run at 0.5/s and 1/s over 12 s each. Any plausible re-shape passes, and the
  `[[exempt]]` ledger stays empty.

## New files to create
- None identified as required. The log-harvest reader (if it becomes a shipped seam rather than a test-only
  affordance) is the only candidate for a new module, and its placement is an open question below — no
  Conductor code reads any SUT log today (every `agent-latest.jsonl` reference in `crates/` is Conductor's
  **own** artifact: `conductor-cli/src/paths.rs:89`, `conductor-core/src/obs.rs:59`, and test files).

## Files to modify
- `scenarios/fingerprint-storm.toml` — header prose (two falsified claims), `p_ids` (+`P-074`), the two
  `[phases.emission]` blocks, possibly `slo_tier`.
- `crates/conductor-emit/src/exception.rs` — `FingerprintVariant::PathVariant` doc (`:85-86`) and its
  `derive` arm; **`Frame`'s struct doc (`:40-41`) and its `file` field doc (`:45`) carry the same falsified
  "fingerprint-INSENSITIVE" claim and the operator's directive did not name them**; the affected unit tests.
- `crates/conductor-core/src/phase_spec.rs` — **`FingerprintVariantSpec::Path`'s doc (`:149`) carries a
  third instance of the falsified claim, also unnamed in the directive**; plus any new variant member and
  its garde rule.
- `crates/conductor-core/src/drift.rs` (`:61-63`) — remove `"P-074"` from `UNBACKED_AUTO`, **in the same
  commit** as the `p_ids` edit.
- `crates/conductor-run/src/dispatch.rs` (`:216-224`) — `wire_variant` match arm, if a variant is added.
- `crates/conductor-run/tests/dispatch_wire.rs` · `crates/conductor-timeline/tests/pacing.rs` — only if
  occurrence counts change (then their four snapshots regenerate).
- `crates/conductor-verify/tests/readback.rs` — only if the live key-set diff finds a divergence.

## Corrections to dictated facts (verified first-hand)
- **`UNBACKED_AUTO` holds NINE entries**, not ten: `P-031, P-033, P-034, P-039, P-041, P-042, P-043,
  P-044, P-074` (`drift.rs:61-63`). Naming P-074 moves the coverage roll-up's qualifier **9 → 8**. The
  layouts extract was right; the scope's CARRY ("10 entries after `P-079` retired") is stale.
- **The live-leg operator recipe is SIX items.** ~~FIVE~~ — **corrected post-P5 by operator override; this
  finding was itself a narrow-basis claim.** The `Canary fingerprint-feed capture` working entry does say
  "FIVE items, not the four on record", but that text was authored 2026-08-13 and the working-route greps
  clean for `SIXTH`/`RUST_LOG`. The accumulated chain lives in `.claude/rules/verification-harness.md:47`,
  which adds a sixth at 2026-08-14: **paired `RUST_LOG=info,conductor_emit=debug`** on the boot/scenario
  legs, surfacing the `emit.batch` wire-shape witness — the BARE per-target form replaces the default filter
  instead of adding to it and fails `agent_mode_routes_self_obs_to_the_log_file_not_stderr`, and neither
  form may ride an invocation that also runs the test suite. Items 1-5: sidecar built and on PATH ·
  `…L4_DETERMINISTIC` + `…DATA_DIR` exported in the launching shell · **`ANDROMEDA_PULSE_MCP_ENABLED=true`
  in Conductor's own environment** (inheritance is the sole channel) · poll budget at or above the contract
  floor · `pulse-app` launched from a cwd OUTSIDE this repo with Conductor run from its repo root.
  **Lesson for the next chunk: for anything in the live-leg/harness domain, read the path-scoped rules file,
  not the working-route entry — route text is a point-in-time citation, the rules file accumulates.**
- **Pulse log-line citations are one to three lines off** but substantively correct: the three `info!`
  targets sit at `storm.rs:342` / `:353` / `:360` (the directive cited `:344`/`:355`/`:362`, which land on
  field lines inside those blocks). `fingerprint_hex` **does** ride the first line, at `:347`.
- **The deterministic-L4 chain verified exactly as cited**: `deterministic_inference.rs:35`
  (`"evidence_refs": []`) → `markdown.rs:286` (`evidence_refs: l4.evidence_refs…`) → `tools.rs:436`
  (`fingerprint_refs` ← `incident.evidence_refs.fingerprint_hashes`).

## Open questions
1. **Which re-shape does the operator want, now that the arithmetic is corrected?** The leg is not
   dead-on-arrival (Autonomous fires at 12), so this is a choice about *what the scenario asserts*, not a
   rescue. → blocks: **plan-decision** (P4 resolves before synthesis).
2. **Where does the log-harvest reader live, and is it shipped or test-only?** No precedent exists; arch
   fixes only that a tenth workspace member would be a registered architectural act. → blocks:
   **plan-decision**.
3. **test-plan §6 requires the envelope's `fingerprints` field populated for this scenario, which a
   deterministic-L4 live leg cannot satisfy** — `observation.fingerprints` is fed solely by
   `fingerprint_refs`, pinned `[]` upstream. The acceptance criteria must not assert it; §6's signal is an
   Expected amendment at wrap, never a silent redefinition. → blocks: **plan-decision**.
