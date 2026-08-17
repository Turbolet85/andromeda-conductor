# Codebase Research — 2026-08-17-fingerprint-semantics-token-leading

## Scope
- **Depth:** moderate · **Reads:** 8 · **Globs/Greps:** 6 · **Graph queries:** 2 (186 rows, 84 rows)

## The headline finding (not anticipated by scope)

**This is not only a doc/test re-aim — Conductor's `fingerprint()` currently computes a DIFFERENT value
than Pulse for any frame path containing `/`.**

`crates/conductor-emit/src/exception.rs:292-300` transcribes Pulse's *old* `is_absolute_path_start`,
which has **no token-boundary guard**. Pulse at `efabe8e` guards it with `is_token_boundary`. For the
committed base fixture (`src/worker.rs`, `exception.rs:347-348`) the two now disagree:

| | normalized frame |
|---|---|
| Conductor today (`exception.rs:251-334`) | `at conductor::worker::handle (src)` |
| Pulse at `efabe8e` | `at conductor::worker::handle (src/worker.rs)` |

Different preimage ⇒ different blake3 ⇒ different fingerprint. So the derivation transcription itself
is the core code change; the variant model, the prose and the tests are downstream of it. Nothing in
this repo catches it, because every gate compares Conductor against Conductor — the same
structural blind spot the 2026-08-15 amendment recorded for the prior derivation claim.

## Files inspected
- `crates/conductor-emit/src/exception.rs` (full, 523 lines) — the derivation, `FingerprintVariant`
  (6 members), `render_stacktrace`, the transcribed scanner, and 13 unit tests.
- `andromeda-pulse crates/buffer/src/fingerprint.rs` (:60-265, read-only) — the SUT derivation at
  `efabe8e` plus its own axis tests, which are the authoritative ground truth for the redesign.
- `scenarios/fingerprint-storm.toml` (full) — the prose at :5-21 and the two `variants` lists at :55, :63.
- `crates/conductor-run/src/dispatch.rs` (:95-110, :203-225) — variant cycling + `wire_variant` mapping.
- `crates/conductor-core/src/phase_spec.rs` (:141-159) — `FingerprintVariantSpec`, the serde wire enum.
- `crates/conductor-run/tests/storm_harvest.rs` (:60-100) — `single_fingerprint` / `retry_storm_surfaced`
  / `no_storm_surfaced` / `tier_ladder_reached_autonomous`.
- `crates/conductor-run/tests/snapshots/dispatch_wire__storm_stream_seed_4317017.snap` (head) — the
  golden's projection.
- `crates/conductor-timeline/tests/snapshots/pacing__fixture_emission_stream_seed_4317017.snap` (head).

## Graph impact
- **`FingerprintVariant` / `FingerprintVariantSpec` / `leading_path_segment` / `normalize_frame`** — 84
  rows. Consumers are exactly four surfaces: `conductor-core/src/phase_spec.rs` (the wire enum),
  `conductor-run/src/dispatch.rs:216-223` (`wire_variant`), `conductor-run/tests/dispatch_wire.rs`
  (`:239`, `:240`, `:402`), and `conductor-emit/src/exception.rs` itself. **No cross-crate consumer
  outside those** — the blast radius is narrow and fully enumerated.
- **`fingerprints` / `fingerprint_refs` in `conductor-verify`** (`extract.rs:236`, `:273`,
  `preflight.rs:69`, `record.rs:79`, plus test doubles) are a **separate symbol family** — read-back-fed,
  never fed by the local derivation. This independently confirms the security extract's "the two must
  not meet" constraint: nothing in this chunk's blast radius touches them.

## Scope premise closure

1. **`[inferred]` hex-address axis usability → PARTIALLY FALSIFIED, and precisely.**
   `render_stacktrace` (`:217-223`) emits exactly `at {function} ({file}:{line})\n` — there is **no
   address slot**, so an address variant cannot be expressed through the existing `Frame`. It is also
   **position-sensitive**: an address appended at line END normalizes away completely (the hex is
   stripped and `normalize_frame`'s trailing-space trim at `:271-273` removes the separator), but an
   address inserted mid-line leaves a doubled space and breaks equality. Using this axis therefore
   requires a render-shape change and a trailing-only placement.
2. **`[inferred]` absolute-path axis unusable by construction → VERIFIED.**
   `stacktrace_carries_no_absolute_host_path` (`:513-521`) asserts the rendered stacktrace contains no
   `C:\` / `/Users/` / `/home/`, and `:414-424` additionally asserts `!after.file.starts_with('/')`. A
   base cannot be absolute, so the absolute axis (same-fp only *between two absolutes*) is unreachable.
3. **`[inferred]` dispatcher variant distribution → VERIFIED, and it reproduces Pulse's split exactly.**
   `dispatch.rs:101-103` cycles `variants[occurrence % variants.len()]`. With `["identical","path","line"]`:
   phase 1 (6 occurrences) = 2 each → 4 base-fp + 2 path-fp; phase 2 (12) = 4 each → 8 base + 4 path;
   cumulative **12 base / 6 path**. Pulse's P-075 note derived ~12/~6 independently — the two agree.
4. **`[inferred]` `storm_harvest.rs` may need no change → VERIFIED.** `single_fingerprint` (`:63-73`)
   collects distinct `fingerprint_hex` and errors on more than one; restoring a single fingerprint makes
   it correct **as written**. Its three sibling predicates key on `cue_kind` / `occurrence_count`, none of
   which is path-semantics-dependent. No change needed.

## Patterns detected
- **Variant cycling is positional, not weighted** (`dispatch.rs:102`): `occurrence % variants.len()`. A
  same-fp set of size *k* over *n* occurrences puts exactly `n/k` on each member, so the storm's
  threshold arithmetic is a direct function of the list length.
- **Only ONE scenario uses a multi-member variant list** — `fingerprint-storm.toml` (`:55`, `:63`). Every
  other scenario uses `["identical"]`; `fingerprint-distinct.toml` uses `["type"]`/`["frame"]`. The TOML
  blast radius is a single file.
- **`FingerprintVariantSpec::RelativePath` is used by NO scenario** (grep: zero `relative_path` in
  `scenarios/`). It exists only for the unit-tier assertion at `exception.rs:401-408`.
- **Both committed goldens are variant-INDEPENDENT.** `pacing__*` carries only
  `EmissionPoint { phase_index, occurrence }` + elapsed ms; `dispatch_wire__*` projects
  `SpanShape { batch, name, trace_id, span_id, parent_span_id, status, events }` where `events` holds the
  event NAME (`"exception"`) — the projection contains zero stacktrace or fingerprint content
  (`grep -c "worker\|src/"` = 0). Changing only the `variants` list, at unchanged `occurrences` / `gap_ms`
  / `seed`, does not move either golden.
- **`leading_path_segment` (`:46-48`) exists solely to model the superseded rule** and is consumed by
  `PathVariant::derive` (`:127`) and the two path tests (`:419`, `:429-435`).

## Conventions to follow
- **Transcribe-and-pin with the HEAD recorded** — `exception.rs:160-162` names the SUT source file and
  the transcription relationship; the module doc (`:8-12`) and `fingerprint()`'s doc restate the
  narrowings. All three restate the same rule and all three go stale together.
- **Test the derivation, not the enum** (`testing.md`, test-plan §11): the existing tests assert through
  `fingerprint(&spec)` output, never on `FingerprintVariant` internals. `only_the_leading_path_segment_reaches_the_preimage`
  (`:428-435`) is the shape a replacement pinning test should take.
- **Load envelope is untouched by a mix change**: the two asserted terms are per-emitting-phase duration
  and rate; `occurrences` (6/12) and `gap_ms` (12000) do not move, so the gate and the
  `[ENVIRONMENT-SUSPECT]` caption are unaffected.

## New files to create
- (none)

## Files to modify
- `crates/conductor-emit/src/exception.rs` — transcribe Pulse's `is_token_boundary` guard into
  `is_absolute_path_start`; redraw `FingerprintVariant` same-fp membership; retire
  `leading_path_segment` and the superseded module/fn doc claims; re-aim
  `same_fingerprint_variants_match_the_base` (`:382-396`),
  `the_path_variant_differs_below_an_unchanged_leading_segment` (`:413-424`),
  `only_the_leading_path_segment_reaches_the_preimage` (`:428-435`),
  `relative_path_is_fingerprint_significant` (`:401-408`).
- `crates/conductor-core/src/phase_spec.rs` (`:141-159`) — the `FingerprintVariantSpec` doc comments state
  the superseded rule per member; the `Path`/`RelativePath` split collapses.
- `crates/conductor-run/src/dispatch.rs` (`:216-223`) — `wire_variant`'s mapping follows whatever the
  enum becomes.
- `crates/conductor-run/tests/dispatch_wire.rs` (`:239`, `:240`, `:402`) — the three
  `FingerprintVariantSpec` construction sites (caller threading, enumerated from the graph result).
- `scenarios/fingerprint-storm.toml` — the prose (`:5-21`) and both `variants` lists (`:55`, `:63`).

**NOT a touchpoint:** `.andromeda/architecture.md` §Read-Back Dependency Posture carries the P-017 clause
that this chunk supersedes, but a spec master is never a file-to-modify — routing one through `/implement`
bypasses the amendment flow whole (no sidecar history, no playbook validation, and the cascade walks only
APPLIED amendments). It rides the plan as an `Expected amendments (wrap)` entry instead. The arch extract
called this edit `/implement`'s; the plan-template discipline overrides that.

## Open questions
- **Does the same-fp set become `{Identical, Line}`, or is an address axis added?** The address axis
  needs a `render_stacktrace` shape change and trailing-only placement (closure 1), which puts new
  synthetic content on the wire for a chunk whose premise is "move to the SUT, change nothing else."
  → blocks: **plan-decision** (P4 resolves before synthesis).
- **Does `FingerprintVariantSpec` keep both `Path` and `RelativePath` as different-fp members, or
  collapse to one?** No scenario names `relative_path`, so collapsing costs no config migration; keeping
  both preserves two named unit assertions. → blocks: **plan-decision**.
