# Codebase Research — 2026-08-16-canary-fingerprint-derivation-aligned

## Scope
- **Depth:** deep · **Reads:** 9 (4 Conductor, 5 SUT) · **Globs/Greps:** 11 · **Graph queries:** 1 (44 rows)

The chunk's stated mechanism was falsified mid-research, so the pass went deeper on the SUT side than a
derivation change would normally warrant. That was the right depth: the falsification is only visible by
tracing the compared field to its producer.

## Files inspected

**Conductor**
- `crates/conductor-emit/src/exception.rs` (full, 1-280) — `fingerprint()` at `:115-123` (FNV-1a over
  `exception_type` + `\n`-separated frame `function`s, `{:016x}`); `render_stacktrace()` at `:166-172`
  emitting `at {function} ({file}:{line})\n` per frame; the wire event at `:155-163` binding that render to
  `exception.stacktrace`; the P-017 variant tests at `:243-277`.
- `crates/conductor-verify/src/preflight.rs:347-375` — `assert_canary`: polls `query_incident_list`, then for
  each incident id calls `retrieve_telemetry_slice` and tests
  `fingerprint_refs(&slice).iter().any(|fp| fp == &canary.fingerprint)` (`:367`) — **exact full-string
  equality**, no prefix logic anywhere.
- `crates/conductor-verify/src/extract.rs:105-120, 250-262` — `fingerprint_refs` is the shared reader used by
  both the canary and per-check extraction; `readers_degrade_to_empty_on_a_malformed_shape` pins that an
  unrecognized shape yields empty rather than erroring.
- `crates/conductor-tauri/ui/src/components/RunReport.tsx:6-7, 54` — renders `fingerprints.length` (a COUNT)
  in `type-data`, never the values.

**SUT (`D:/dev/projects/andromeda-pulse`)**
- `crates/buffer/src/fingerprint.rs:79-169` — the real derivation (see Graph impact below).
- `crates/mcp-server/src/tools.rs:44-51, 422-443` — the 8-tool surface and `dispatch_retrieve_telemetry_slice`.
- `pulse-app/src/inference_runtime.rs:660-710` — the only production `EvidenceRefs` construction.
- `pulse-app/src/deterministic_inference.rs:30-40` — the canned L4 fixture.
- `crates/interpretation/src/schema.rs:150-176` — `L4Output`, with `evidence_refs` and `fingerprint` as
  distinct model-authored fields.

## Graph impact (44 rows, `tree-query-2026-08-16-canary-fingerprint-derivation-aligned.json`)
- **`exception/fingerprint()`** — 10 references. Producers/consumers: `conductor-emit/src/lib.rs:24`
  (re-export), `conductor-run/src/lib.rs:30` (import) and `:223` (`emit_canary` computes the expected value),
  `conductor-run/tests/canary_wire.rs:22,148,149`, plus 6 in-crate test sites in `exception.rs`. **No caller
  outside `conductor-emit` / `conductor-run`** — the seam holds and a signature-compatible change has zero
  cross-crate blast radius beyond those two.
- **`preflight/CanaryMarker#fingerprint`** — 2 refs (`preflight.rs:52` field, `:366` the comparison).
- **`extract/fingerprint_refs()`** — 4 refs (`extract.rs:113,257`, `preflight.rs:20,366`).
- **`run_record/RunRecord#fingerprints`** — 9 refs across `conductor-cli/src/render.rs:185`,
  `conductor-core/{lamp.rs:92,run_record.rs:68,104}`, `conductor-report/{db.rs:84,213, journal.rs:79,
  report.rs:148}`, `conductor-verify/src/record.rs:79`. This is the ENVELOPE field, fed from
  `Observation#fingerprints` (read-back), **not** from `exception::fingerprint()` — the two never meet.
- **Zero** references to `Fnv1a` outside `exception.rs` (private helper).

## Patterns detected
- **Shared-reader discipline** (`preflight.rs:377-378`): the canary and per-check extraction read Pulse's
  shapes through one definition in `crate::extract`, deliberately. Any change to what the canary compares
  must respect that both callers share the reader.
- **Degrade-to-empty on unknown shape** (`extract.rs:257-262`): every reader returns empty rather than
  erroring, which is why a field that is *structurally absent* and one that is *empty* are indistinguishable
  downstream — precisely the ambiguity that hid this chunk's real blocker.
- **Seeded identity, content-pure fingerprint** (`exception.rs:8-11, 245-277`): span identity is seeded, the
  fingerprint is a pure function of content. Preserved by any derivation swap.
- **Pulse's hash discipline** (`fingerprint.rs:17-31`): `exception.message` is deliberately excluded from the
  preimage so incidentally-captured secrets cannot influence the hash — a constraint any adopted derivation
  inherits for free by hashing type + normalized stacktrace only.

## Conventions to follow
- **Verdict/error wall at the gate** (`preflight.rs:349-375`): `assert_canary` returns a `CanaryFidelity`
  value on every path including call errors — never a `Result::Err`.
- **Exact-string comparison, no width logic** (`preflight.rs:367`): there is currently no prefix/truncation
  anywhere on the comparison path; introducing one would be new behavior, not a tweak.
- **Relative frame paths** (`conductor-run/src/lib.rs:137`): the canary's frame is
  `("conductor::run::preflight_canary", "conductor-run/src/lib.rs", 1)` — already relative, so Pulse's
  absolute-path stripping is a no-op on it and `stacktrace_carries_no_absolute_host_path`
  (`exception.rs:315`) already guards the class.

## New files to create
- None.

## Files to modify
- `crates/conductor-emit/src/exception.rs` — the derivation, its doc comment (`:115-117` claims a P-017
  clause-(c) semantic that the first-3-lines bound narrows), the module header (`:5-8`), and the variant tests.
- `crates/conductor-emit/Cargo.toml` + root `Cargo.toml` `[workspace.dependencies]` + `Cargo.lock` — only if a
  hashing dep lands. **No hashing crate is in the tree today** (`blake3`/`sha2`/`fnv`/`twox`/`xxhash` all
  absent from every manifest; `blake3` count in `Cargo.lock` is 0), so this would be a genuinely new normal dep.
- `crates/conductor-run/src/lib.rs` — the two ride-along doc twins the working entry names: `emit_canary`'s
  doc at `:216-220` (still says "computed to match Pulse's derivation") and `warm_up_canary_service`'s doc at
  `:234-237` (still teaches the disproved bootstrap rationale).
- `crates/conductor-verify/src/preflight.rs` — only if the canary's assertion is re-aimed (the open decision).
- `crates/conductor-run/tests/canary_wire.rs` — pins the fingerprint at `:22,148,149`.

**Not to modify (checked, negative findings):**
- **No golden carries a fingerprint literal** — `grep -c fingerprint` is 0 across all six committed snapshots
  (`dispatch_wire__*` ×2, `pacing__*` ×2, `replay__*` ×2). No re-baselining is owed.
- **No webview change** — `RunReport.tsx` renders a fingerprint COUNT, not values. This also resolves the P2
  design↔layouts binding contradiction: design's "fingerprints in `--color-id-cyan`" and layouts' "the
  desktop identifier set excludes fingerprints" are both satisfied by a count cell in the data tier. Neither
  spec is wrong and the chunk touches neither surface.

## Open questions
- **What should the canary's final precondition assert, now that no read-back field carries a
  Conductor-attributable value under deterministic L4?** → blocks: **plan-decision**. P4 must resolve before
  synthesis; it is architecture-touching (arch §Standard Contracts describes the round-trip) so it goes to the
  operator with a recommendation.
- **Does the derivation alignment land in this chunk at all, given it no longer serves preflight-green and
  costs a new normal dependency while `cargo audit` cannot parse its DB?** → blocks: **plan-decision**.
  Bundled with the question above.

## PREREQ re-measured (standing `cargo audit` deferral, 21st pin)
`cargo audit` → **true exit 1**, byte-identical `error loading advisory database: parse error: duplicate
advisory ID: RUSTSEC-2026-0244`. Overlap `cargo deny check advisories bans licenses sources` → **true exit 0**
(`advisories ok, bans ok, licenses ok, sources ok`). Basis unchanged: advisory-DATABASE fault, no floor to
raise. **If a hashing dep lands this chunk the basis changes** — the tree stops being zero-delta, and the new
dep would be scanned by `cargo deny` alone.
