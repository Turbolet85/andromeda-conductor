# Scope — 2026-08-09-sut-drift-check

**Working entry (verbatim):**
> SUT-drift check — loud failure when Pulse's ledger advances past the capability set Conductor knows
> `CARRY:` the accepted set now ships as `contracts/pulse-capabilities.toml`
> (`sut_version`/`captured_at`/`capabilities[]`) with `CapabilityManifest::load` + `accepts` — build the drift
> check over THAT, don't re-source it.

**Version:** conductor-0.2.0 · **Epoch:** 1 — Foundation: re-aim at the SUT
**Capability:** `verification-matrix.json#v2-02` (intent §Theme 1 · F1, second clause)

---

## The gap this closes

`2026-08-08-sut-capability-manifest` made the *accepted* set data (`contracts/pulse-capabilities.toml`,
82 ids, `sut_version = "v0.3.0"`), so a scenario may now name any id Pulse actually ships. But nothing
asserts that Conductor has *reckoned with* those ids. The coverage classification
(`crates/conductor-core/src/coverage.rs`) is still a `static [CapabilityRow; 60]` over `P-001..P-060`,
with tests that assert exactly-sixty and contiguity. So today the manifest knows 82 capabilities and the
classification knows 60 — a 22-id blind spot (`P-061..P-082`) that no gate reports.

That is precisely the failure mode the version exists to end: 22 new Pulse capabilities went unnoticed
across a 41-day pause because nothing in the repo could notice. Widening a number does not fix it; a
*check* does.

## What this chunk builds

A drift check over the two artifacts that already exist — the shipped `CapabilityManifest` and the
shipped coverage classification — that fails loudly, naming the unknown ids, when the manifest carries
capabilities the classification does not cover, and passes when the two are in sync.

Per the CARRY: the accepted set is **read through `CapabilityManifest::load` / `accepts`**. This chunk
does not re-source, re-parse, duplicate, or re-derive the accepted set, and does not introduce a second
place where "which P-IDs exist" is written down.

The check's failure output is the deliverable as much as the boolean: it must name the drifted ids (and
the manifest's `sut_version` / `captured_at`, which exist to make the message actionable), so an operator
reading a red gate knows which Pulse release moved and what is owed — not merely that a count mismatched.

Amended at P4: the comparison also reports the **reverse** direction — a classified row with no manifest
entry, i.e. a capability Pulse retired. `v2-02`'s acceptance does not require it, and it reports nothing
today (the classification is a subset of the manifest), but it is one extra set difference and therefore
a free latent guard rather than added risk.

## Boundaries

- **Does NOT classify anything.** Extending the classification to the current SUT set is the next
  chunk (*Current-SUT coverage classification*, `v2-03`), which carries its own CARRY about `coverage.rs`
  being `&'static str` rows with "no runtime IO" and its two exactly-sixty tests. This chunk builds the
  detector; that chunk makes it green. **RESOLVED at P4 (operator decision) — amended here, per
  validation.md's intent-incomplete rule:** the scope originally framed this as a two-way choice (hard
  gate red until `v2-03`, vs a non-gating signal). Planning surfaced a third and better option, which was
  chosen: the check is **live and gating from day one against a pinned known-gap ledger** holding today's
  22 unclassified ids. It is green on the committed tree, and fails the moment a manifest id appears
  outside the pin (a Pulse release), a pinned id gets classified (pin rot), or a classified row loses its
  manifest backing. `v2-03` empties the pin and the check becomes the plain zero-drift assertion. The pin
  is a *residual ledger*, never a second source of the accepted set.
- **Does NOT query a live Pulse.** `v2-02`'s method is `unit`; the manifest is the recorded SUT proxy and
  the check is an offline artifact-vs-artifact assertion. Detecting that Pulse's *live* ledger moved past
  the manifest is a different (live-path) concern and is not in this chunk.
- **Does NOT edit the manifest** — `contracts/pulse-capabilities.toml` stays as shipped; re-aiming at a
  newer Pulse remains a manifest edit, and this check is what makes such an edit impossible to make
  silently.
- **Does NOT touch scenario validation.** `Scenario::from_toml_str_with` / `check_capabilities` /
  `pid_format` are unchanged; this is a coverage-universe check, not a membership check.
- **No new crate edge.** Both artifacts live in `conductor-core`.

## Surfaces and contracts touched

| Surface | Involvement |
|---|---|
| `crates/conductor-core/src/capability_manifest.rs` | consumed (`load` / `accepts` / `default_path`) — read-only |
| `crates/conductor-core/src/coverage.rs` | consumed (`coverage_matrix()`) — the classified universe |
| `crates/conductor-core/src/error.rs` | modified — a new `SutDrift` variant on the `#[non_exhaustive]` `CoreError` (its documented extension point); a drift failure must respect the verdict/error wall, never a `Verdict`/`ReportState` |
| CLI / agent-run surface | **RESOLVED at P4: not touched.** The check lands in the core unit tier only, matching `v2-02`'s declared `method: unit`; CI enforcement rides the existing `agent-run run` dogfood step, and operator-facing surfacing stays with the Epoch-6 *Coverage completeness gate* entry that already owns it |
| `contracts/pulse-capabilities.toml` | read-only input; never rewritten by the check |

## Invariants this chunk must not break

- The accepted set is DATA (manifest), never a compile-time constant — the check must not reintroduce a
  hardcoded id range or a second hardcoded count.
- Verdict/error wall: a drift condition is a harness-side failure, not a scenario `Blocked`/`Fail`.
- Artifact hygiene: the failure message names capability ids and manifest metadata — never absolute host
  paths or internal struct names.
- Determinism: the check reads committed artifacts only; same tree ⇒ same result.
