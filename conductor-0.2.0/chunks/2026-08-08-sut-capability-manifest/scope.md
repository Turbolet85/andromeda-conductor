# Scope — 2026-08-08-sut-capability-manifest

**Working-route entry (verbatim):**
> SUT capability manifest — validated versioned refs/ Pulse artifact as the accepted P-ID source, replacing the compile-time 001..=060 bound, malformed reporting blocked

**Epoch:** 1 — Foundation: re-aim at the SUT (keystone)
**Version:** conductor-0.2.0
**Capability:** `verification-matrix.json#v2-01`
**Carried annotations:** none (no `PREREQ:` / `CARRY:` on the entry)

---

## Why this chunk exists

Conductor is a harness whose whole purpose is proving a live Pulse. Over a 41-day pause Pulse shipped 22 new
capabilities (P-061..P-082) and Conductor cannot so much as *name* them: the accepted capability set is a
**compile-time constant**, so a scenario keyed to anything above P-060 fails validation at load. Every
downstream proof in this version is aimed at a stale target until that constant stops being the source of
truth. This is the keystone chunk of the keystone epoch — nothing in Epochs 2–6 is worth running before it.

**OBSERVED (intent §4 F1):** `crates/conductor-core/src/scenario.rs:23-33` validates `P-NNN` with
`(1..=60).contains(&n)` and returns `"expected P-NNN with NNN in 001..=060"`; the unit test at `:195` asserts
`"P-061"` MUST be rejected. Pulse's ledger now reaches P-082.

**EXPECT (intent §4 F1):** the accepted capability set tracks the SUT instead of a compile-time constant —
sourced from a versioned SUT capability artifact under `refs/`, so **a Pulse release is a data update, not a
code change**. Widening the constant to `001..=082` is named in the intent as the *minimum* and is explicitly
**not the answer**: it re-freezes the same defect one version later. Take it only as a declared interim, never
as the deliverable.

## What this chunk builds

1. **A versioned SUT capability artifact** at `contracts/pulse-capabilities.toml` enumerating Pulse's current
   capability set. _(Amended at P5 validation-1 — the scope originally said `.andromeda/refs/`, following the
   intent's "under `refs/`" wording. Planning surfaced that `.andromeda/refs/` holds only Pulse-authored
   planning documents, none parsed at runtime, and is absent from arch's directory tree, whereas `contracts/`
   is arch's registered runtime-read config root already holding the sibling `mcp-contract.toml`. Operator
   decision at P4: `contracts/`. This avoids an arch §Occupied Resources + directory-tree amendment.)_
2. **`conductor-core` sources the accepted P-ID set from that artifact** rather than from the hardcoded
   `(1..=60)` range in the scenario-model validator, so scenario validation accepts exactly the SUT's real
   capability set.
3. **A validated load boundary.** The manifest is a new on-disk parsed input at a trust boundary: it is
   validated at load, and a malformed / absent / unparseable manifest surfaces a **named precondition** rather
   than being silently downgraded, silently widened, or panicking.
4. **The superseded unit test** at `scenario.rs:195` — which encodes the old bound by asserting `"P-061"` must
   be rejected — is replaced by tests keyed to the manifest-sourced set.

## Boundaries — explicitly NOT this chunk

- **NOT the drift check** (`v2-02`, the next chunk) — this chunk makes the set data-sourced; failing loudly when
  Pulse advances past what Conductor knows is separate.
- **NOT the coverage classification** of P-061..P-082 into auto / drive+observe / static-only /
  not-Conductor's (`v2-03`), and NOT the out-of-scope rendering treatment.
- **NOT new scenarios** for the in-lane capabilities P-067 / P-072 / P-079 (`v2-04`).
- **NOT any live-Pulse work** — no MCP read-back, no emission, no preflight change. This chunk is offline and
  fully CI-verifiable.
- **Conductor does not fix or modify Pulse** (version non-goal); the artifact is Conductor's own record of the
  SUT's claimed surface, not a change to the SUT.

## Surfaces and contracts touched

- **`conductor-core`** — the scenario model's P-ID validation (garde surface); the source of the accepted set.
- **`contracts/`** — a new versioned capability artifact alongside `mcp-contract.toml` (runtime-read config
  data, not a spec source).
- **Scope law** — "no scenario without a P-ID" is preserved and strengthened: the P-ID space stays the SUT's
  (`P-NNN`), never Conductor's own (`v2-NN`).
- **Verdict/error wall** — the load failure path must land on the correct side of the wall (see open question).
- **Determinism** — sourcing the set from a committed artifact must not introduce run-to-run variance.

## Open questions — RESOLVED at planning (P4/P5)

1. **Which side of the verdict/error wall does a malformed manifest land on?** → **RESOLVED: two-tier**
   (operator decision at P4). The *loader* returns a typed `conductor-core` harness fault (`Result::Err`) —
   matching both in-repo precedents (`Scenario::from_toml_str` → `CoreError::Config`, and
   `ContractManifest::load` → `VerifyError::Manifest`, documented "never a verification verdict"). The
   route line's "malformed reporting blocked" is satisfied at the *gate/run* surface, which already renders
   `[BLOCKED]` + a named precondition. Decisive structural argument (arch extract): a `Blocked` row requires
   `run_id`/`seed`/`scenario`/`p_ids`/`slo_tier`, none of which exist at manifest-load time. This chunk builds
   only the loader tier.
2. **Does the accepted-set source also feed `coverage.rs`?** → **RESOLVED: no — `v2-03` owns it.** Research
   found `coverage.rs` is a `static COVERAGE: [CapabilityRow; 60]` of `&'static str`, documented "no runtime
   IO" (`coverage.rs:46`), so manifest-sourcing it is a *representation change*, not a re-point. `coverage.rs`
   and its two "exactly sixty" tests stay green and untouched here; the plan carries an explicit
   do-not-modify guard.
3. **Manifest path handle.** → **RESOLVED: no override handle.** The manifest resolves from `default_path()`
   exactly as `ContractManifest` does. If /implement finds cause to add one it must be `CONDUCTOR_*`-namespaced,
   resolved through the existing `conductor_core::config_path::resolve_under` at the **cli edge**, carry
   traversal-rejection tests, and be registered in arch §Occupied Resources at wrap — never added quietly.

_(Also settled at P4, not originally a scope question: `Scenario::from_toml_str(&str)` **keeps its signature**.
The code-graph reported 1 production caller; a grep cross-check found 5 across 4 crates, so threading a
manifest parameter would turn a contained validator change into a four-crate ripple.)_

## Definition of done for this chunk

A scenario naming any capability present in the manifest validates at load; one absent from it is rejected with
a message that names the manifest rather than a hardcoded range; a malformed or absent manifest produces the
named precondition instead of a silent pass or a panic; and adding a future Pulse release requires editing only
the artifact, with no Rust source change. Workspace lint + tests green, `Cargo.lock` un-drifted.
