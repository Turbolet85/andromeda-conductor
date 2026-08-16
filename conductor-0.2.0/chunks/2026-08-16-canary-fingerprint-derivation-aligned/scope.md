# Scope — Canary fingerprint derivation aligned

**Marker:** `2026-08-16-canary-fingerprint-derivation-aligned`
**Version:** conductor-0.2.0 · **Epoch:** 3 — Live proof: the five families
**Working entry:** _Canary fingerprint derivation aligned — Conductor's expected fingerprint is what Pulse
actually derives, so the round-trip's last precondition can pass_

---

## What this chunk builds

Conductor's exception fingerprint stops being a Conductor-invented value that a doc comment *claimed*
matched Pulse, and becomes **Pulse's own derivation, recomputed on Conductor's side over the same input
Pulse receives**. The preflight canary round-trip's final precondition (`canary fingerprint not found in
telemetry slice`) can then pass on its merits rather than failing by construction.

This is the **third generation of the preflight-green blocker** (count → primary key → derivation) and the
first whose obstacle was proven *by construction* rather than by symptom. The 2026-08-16 live leg drove the
canary through every prior stage successfully — ingest counted 15 spans, the buffer appended all of them
(27 `duckdb.append` lines, **zero** `reject_reason`), the `buffer.tick` trio read `12/12/12`,
`storms_detected_total` reached 2 with `severity_hint: "autonomous"` at `occurrence_count: 10`, and an
incident formed (`item_id: 1`) — then stopped at the fingerprint comparison.

### The two derivations as they stand

| | Conductor (today) | Pulse |
|---|---|---|
| Algorithm | FNV-1a 64-bit | blake3, truncated to 16 bytes |
| Input | `exception_type` + `\n` + each frame's `function` | `exception_type` + `\0` + `normalize_stacktrace(stacktrace)` |
| Rendered width | 16 hex chars (`{:016x}`) | read back as an **8-char hex prefix** of the first 4 bytes |

Conductor: `crates/conductor-emit/src/exception.rs:115-123` (`fingerprint`), with the wire stacktrace built
separately by `render_stacktrace` (`:166-172`) as `at {function} ({file}:{line})\n` per frame.
Pulse: `andromeda-pulse crates/buffer/src/fingerprint.rs` — **VERIFIED first-hand in P3** against the on-disk
SUT repo. `compute_exception_fingerprint` (`:79-96`) hashes `exception_type.as_bytes()` + `b"\0"` +
`normalize_stacktrace(stacktrace)` with blake3, copying the first 16 bytes out.
`normalize_stacktrace` (`:118-131`) keeps the first 3 non-empty lines, `normalize_frame`s each (`:139-169`
strips absolute Unix/Windows paths, `0x…` hex addresses, and `:line(:col)` suffixes) and joins with `\n`.
`fingerprint_to_hex_prefix` (`:102-108`) renders the first 4 bytes as 8 lowercase hex chars. One citation
correction: the derivation occupies `:79-96`, not `:79-110` — the wider span runs into the hex-prefix helper.
The algorithm, input composition, separator byte, truncation width and normalization steps are all as the
CARRY stated.

**Equality is impossible by width alone**, before algorithm or input are even considered.

### [premise-corrected: `fingerprint_refs` carries the L4 model's `evidence_refs`, never the buffer's computed fingerprint — so aligning the derivation cannot make the round-trip's last precondition pass]

The working entry's premise — *Conductor adopts Pulse's derivation, **so** the round-trip's last precondition
can pass* — is **FALSIFIED**. The derivation mismatch is real, but it is not what blocks the gate, and
removing it changes nothing about this precondition. P3 traced the field the gate compares against, end to
end in the SUT's source:

1. `dispatch_retrieve_telemetry_slice` returns `fingerprint_refs = incident.evidence_refs.fingerprint_hashes.clone()`
   (`andromeda-pulse crates/mcp-server/src/tools.rs:436-441`).
2. The **only** production construction of `EvidenceRefs` is `pulse-app/src/inference_runtime.rs:684-701`,
   which sets `fingerprint_hashes: parsed.evidence_refs.clone()` — `parsed` is the **L4 model's output**
   (`L4Output.evidence_refs: Vec<String>`, `crates/interpretation/src/schema.rs:169`). Every other
   `EvidenceRefs { … }` in the repo is inside a `#[cfg(test)]` module.
3. Under `ANDROMEDA_PULSE_L4_DETERMINISTIC` — required by the run contract and used by every live leg — the
   canned output is `"evidence_refs": []` (`pulse-app/src/deterministic_inference.rs:35`), so
   `fingerprint_refs` is **always the empty array**.
4. With real L4 it would carry model-authored free text, not a hash — Pulse's own fixtures show
   `"fp:db-sat"` / `"span:abc123"` (`crates/interpretation/src/markdown.rs:399`, `:424`).
5. The buffer's computed fingerprint is written to the `span_events.fingerprint` BLOB column, and **no MCP
   tool reads `span_events`** — the surface is exactly 8 tools (`crates/mcp-server/src/tools.rs:44-51`) and
   zero of them touch that table.

**Conductor's exception fingerprint therefore has no read-back surface at all.** The canary's final
precondition is unsatisfiable as designed, independent of derivation, algorithm or width. This also
supersedes the chunk's named research question: **prefix-vs-full-width is moot for the gate**, because there
is no wire value to compare against at either width.

A second consequence, worth stating because later Epoch-3 entries depend on it: under deterministic L4 the
L4-authored fields (`title`, `severity`, `fingerprint`, `evidence_refs`) are fixture constants, so **no
read-back field varies with what Conductor emitted**. The only incident properties that still track the
canary are `kind` / `scope` / `scope_id` (derived from the cue, not from L4) and `opened_at_unix_nano`.

### What the derivation alignment IS still worth

Aligning is no longer the route to preflight-green, but it is not empty work: Conductor's fingerprint encodes
its *prediction of how Pulse groups exceptions*, and the two derivations predict differently. Conductor hashes
every frame up to `MAX_FRAMES = 64`; Pulse hashes only the first 3 normalized lines. Two exceptions differing
only at frame 4+ are **distinct** to Conductor and **identical** to Pulse — so the P-017 identity triple and
the downstream `fingerprint-storm live proof` entry currently rest on a grouping rule the SUT does not use.

## Boundaries

- **Conductor adopts Pulse's derivation.** Pulse is the system under test — its fingerprint is the fact and
  Conductor's is the expectation. Nothing in this chunk changes, patches, or proposes a change to Pulse.
- **The fingerprint's INPUT moves from frame functions to the rendered stacktrace.** **VERIFIED** — Pulse
  hashes the `stacktrace` string it received (`compute_exception_fingerprint(exception_type, stacktrace)`),
  so Conductor must hash the same string through the same normalization, not a parallel construction over
  `frames[].function`. Conductor's `render_stacktrace` (`exception.rs:166-172`) is what fills the wire's
  `exception.stacktrace` attribute (`:160`), so it is the exact preimage.
- **The P-017 semantic contract must survive.** **VERIFIED, and it does not survive unchanged.**
  `FingerprintVariant` asserts path-insensitive, line-insensitive, type-sensitive, frame-sensitive identity.
  Pulse's `normalize_frame` strips absolute paths and `:line(:col)`, so path/line-insensitivity holds — but
  its **first-3-non-empty-lines** bound is real, so a frame-variant perturbing a frame beyond the third
  becomes indistinguishable. The committed variants are safe: `FingerprintVariant::FrameVariant` perturbs
  `frames.first_mut()` (`exception.rs:104-106`) and the canary's spec carries a single frame — but the
  narrowing is a genuine behavior change that must be recorded, not glossed. Note Pulse's rendered frames
  keep `file` only when it is a RELATIVE path (`normalize_frame` strips absolute ones), which Conductor's
  synthetic frames already are.
- **Not a scenario-catalog change.** `scenarios/fingerprint-storm.toml` and the rest of the catalog stay as
  they are; this chunk changes a derivation and the tests that pin it, not declared scenario shape.
- **No code is written in this phase** — plan.md says WHAT, `/andromeda-implement` does it.

## Surfaces and contracts touched

- `crates/conductor-emit/src/exception.rs` — `fingerprint()`, its doc comment, the `Fnv1a` helper (whose
  fate the plan decides: replaced outright, or retained if some other caller still needs it), and the
  module header's "the fingerprint Conductor expects Pulse to derive" narrative.
- `crates/conductor-run/src/lib.rs` — the canary path (`canary_spec`, `emit_canary`, `emit_canary_storm`)
  consumes `fingerprint()`; the preflight gate compares the result against
  `retrieve_telemetry_slice.fingerprint_refs`.
- **A hashing dependency becomes a NORMAL (not dev/test-only) dependency of the emit path.** The dep-kind
  boundary rule applies — a test-scoped dep cannot back a shipped signature. This is the chunk's manifest
  touchpoint: `Cargo.toml` + `Cargo.lock` move, and the supply-chain gate must be re-run against a tree
  that is no longer zero-delta.
- Test surfaces pinning the old derivation — the `exception.rs` unit tests, `conductor-run`'s
  `tests/canary_wire.rs`, and any golden or fixture carrying a literal fingerprint value. P3 enumerates
  them from the code-graph rather than by grep.
- `.andromeda/architecture.md` §Established Decisions [Read-Back Dependency Posture] + §Standard Contracts
  both currently record the mismatch as BY CONSTRUCTION with the gate ending `ready:false` at this
  precondition. Landing the fix makes that wording stale — an **expected amendment at this chunk's wrap**,
  not a phase edit.

## Folded annotations (from the working entry)

**CARRY — two source twins ride this chunk with the fix** (wrap does not edit source; the `scheduler.rs`
comment precedent):
- `crates/conductor-run/src/lib.rs:218` — `emit_canary`'s doc still says the fingerprint is "computed to
  match Pulse's derivation", the exact claim measured false.
- `warm_up_canary_service`'s doc still teaches the disproved bootstrap rationale (the 45s warm-up aimed at
  a 3,600s `BootstrapState::Ready` gate the canary's RetryStorm path never crosses).

**CARRY — live-leg recipe correction, the console tee is retired:** `pulse-app` is a Windows GUI-subsystem
binary and never attaches to a console, so a tee yields 0 bytes. Its real telemetry sink is
`{data_dir}/logs/agent-latest.jsonl.<date>`, harvested post-leg and sliced to the leg window by pre-leg line
count. **Capturing Pulse's own ERROR-level `duckdb.append` `reject_reason` lines stays mandatory** — no
Conductor log can supply them.

**CARRY — `v2-10` ("Live `ready:true` preflight") claims HERE**, on a leg that actually reaches
`ready:true`, with `conductor-0.2.0/chunks/2026-08-15-canary-spans-pulse-fingerprints/evidence/leg-verdict.md`
as the recorded basis its matrix `notes` already points at. It returned to the pool unclaimed at the last two
chunks; claiming it requires a measured leg, never an argument that the known obstacles are gone.

**PREREQ — re-check `cargo audit`** (standing deferral since `2026-08-08-sut-capability-manifest`; ratified
at the `2026-08-10-workspace-key-divergence-probe` wrap; **21st pin**; basis: advisory-DATABASE fault —
byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, no floor exists to raise; overlap:
`cargo deny check advisories bans licenses sources` true exit 0 across all four classes; full rationale:
`conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`). Close the deferral the moment
it parses — **and note the basis CHANGES at this chunk if the hashing dependency lands**, which restores the
full pin form: the tree is no longer zero-delta, so `Cargo.lock` moves and the deferral can no longer rest on
"no dependency delta".

## Definition of done (shape — P4 writes the acceptance criteria, post-premise-correction)

The falsified premise splits the original goal in two. Keeping the GOAL (preflight reaches `ready:true`) and
changing the MECHANISM (the canary's assertion, not the derivation) is what the evidence supports:

1. Conductor's expected fingerprint is computed by Pulse's real derivation over the input Pulse hashes —
   verified against the SUT source, not a doc claim — so Conductor's prediction of Pulse's exception grouping
   is correct where it is used (P-017/P-018 and the downstream storm live proof).
2. The P-017 identity narrowing (frame-sensitivity bounded to the first 3 normalized lines) is explicit and
   recorded rather than silently inherited.
3. **The canary's final precondition asserts something that can actually be observed.** What it should assert
   instead is the chunk's real open decision — it is an architecture-touching choice (arch §Standard Contracts
   describes the round-trip), so it goes to the operator at P5 and lands as an Expected amendment at wrap.
4. If a hashing dependency lands it is a normal dep of the emit path, `Cargo.lock` committed, with the
   audit/deny overlap re-run against the changed tree. **P3 re-measured the PREREQ: `cargo audit` is red for
   the 21st time** (byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1) while
   `cargo deny check advisories bans licenses sources` is **true exit 0** across all four classes — so a new
   dep would land while the primary scanner cannot parse its database, covered only by the deny overlap.
5. `v2-10` claims only on a leg that actually reaches `ready:true`. If the re-aimed precondition is not
   settled and proven this chunk, `v2-10` stays pooled with the measurement recorded — as the last two chunks
   did, and for the same reason: claim on what a leg measured, never on an argument that the obstacles are gone.
