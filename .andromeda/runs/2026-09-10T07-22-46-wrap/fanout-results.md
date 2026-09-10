# Fan-out results — 2026-09-09-port-occupier-test-hygiene

**Wrap run:** 2026-09-10T07-22-46 · **Detectors dispatched:** 7 (one per spec master) · **Proposals: 0**

All seven returns were `proposals: []`. Per `amendment-flow.md` §Fan-out, empty returns are recorded in this
ONE consolidated artifact. **No `.raw-fanout-{doc}.md` twin was written:** the twin exists to preserve
proposal content that stripping could mangle, and no return carried a proposal — each carried only
explanatory negative-evidence prose around its YAML, whose substance is reproduced below. Recorded here so
the choice is auditable rather than silent.

**Entity decode:** the return transport HTML-escaped the angle brackets of a binding arrow in the test-plan
return (the `test-plan §3 ↔ obs-plan §3` arrow arrived as an escaped ASCII form). Decoded on ingest;
**entities = 0** in this saved file — stated without reproducing the escape sequences, so a future reader's
probe cannot false-red on this file's own prose.

| Doc | Detectors evaluated | Verdict |
|---|---|---|
| architecture.md | D-arch-resources · D-arch-decisions · D-platform-claim | `proposals: []` |
| security-plan.md | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim | `proposals: []` |
| design-system.md | D-design-tokens · D-design-derived-count · D-platform-claim | `proposals: []` |
| layout-templates.md | D-layout-surface · D-layout-derived-count · D-platform-claim | `proposals: []` |
| test-plan.md | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim | `proposals: []` |
| obs-plan.md | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim | `proposals: []` |
| a11y-plan.md | D-a11y-surface · D-a11y-obs-schema · D-platform-claim | `proposals: []` |

## Why every detector came back clean

The chunk's modify-set is two crate-local test files with **no `src/` delta**, so the structural detector
families had no subject:

- **Resource / decision registries (arch):** `Symbols / APIs` and `Crates / modules` are both "none added or
  changed". A crate-local **test target** is not a class arch §Occupied Resources registers (ports, MCP tool
  names, Tauri commands, on-disk artifacts, env handles, workspace crates); `conductor-faults` and the
  `:4317` occupier bind were already registered (`architecture.md:162`, `:223`, `:145`, `:246`).
- **Input validation / subprocess / dependencies (security):** no new external-input surface; the sidecar
  spawn, preflight and data-dir are untouched; `Dependencies: none` with `Cargo.lock` byte-unchanged.
- **Tokens / surfaces (design, layouts):** `Coverage of new surfaces: none` — no UI element, so no `tokens`
  flag and no wireframe entry to map.
- **Instrumentation / stack / redaction (obs):** no new operation symbol; no OTel/exporter symbol; the
  one-shot control *exercised* the §6 field allowlist (removing `"port"`, observing red, reverting) rather
  than bypassing it.
- **a11y surface / schema:** no interactive element; `Schema / config: none`, so the violation envelope is
  unchanged on both sides of the a11y ↔ obs bind.
- **Derived counts (×4 detectors):** the three moved counts — nextest binaries 55 → 56, `port_occupier.rs`
  tests 7 → 6, `conductor-faults` doctests 0-inherited → 3-measured — have **no baked occurrence** in any
  master. Each detector swept independently and found 0 hits; numeric near-misses were read and dismissed
  (`comfy-table 7` a crate version; `6 columns` a table shape; `13px / 1.55` a type-scale row;
  `P-003 port-occupier` coverage-matrix capability samples, not a test-file count).
- **Platform claim (cross-doc, all 7):** neither trigger bullet retires a platform/runner/driver verdict.
  `Harness / gate surface` is "none" (four `git diff --quiet` probes at exit 0), and the single
  `Spec claims disproved` entry retires a **doctest-count** record, not a capability verdict. Each detector
  confirmed its own platform sentences are host/entry-point mentions or CI arrangements, which the
  detector's own rule excludes.

## One near-miss, read and dismissed by its own detector

The test-plan detector independently surfaced `test-plan.md:226`'s `(full, non-doc)` phrasing against the
measured 3 doctests, and accepted the report's disposition: the phrase reads as "the full suite, as opposed
to the doc-only arm", and it bakes no count — **no hit**. This is the same non-escalation the report records
under *Decisions & corrections*, reached independently here.

## Validate — the six checks (orchestrator)

1. **Playbook check** — no proposals to match; nothing staged to apply or escalate.
2. **Cross-contradiction** — no proposals, so no opposing pair.
3. **Intent-consistency** — consistent. The chunk did what its working-route entry asked (the fault crate
   green under the shared-process runner; the unmeasured crates **measured**) and, on the operator's P5
   direction, widened the sweep to all nine members — a widening already carried in `plan.md`, so the report
   matches its amended acceptance criteria. The one recorded deviation (the smoke form) touches no
   acceptance criterion and no route-entry intent.
4. **Absence needs evidence** — every absence claim in the report cites the search that establishes it (the
   55/902 sweep, the `0 doctests` sweep, the `runner-portab` sweep, the instance-enumeration grep, four
   `git diff --quiet` probes). Each detector's absence claim likewise names its own sweep.
5. **Expected-amendments reconciliation** — `plan.md` carries **no** `Expected amendments (wrap)` list, and
   its §Implementation notes states the measured basis for that absence (`test-plan.md:538` states the
   own-binary remedy generically and enumerates no instances — 0-hit grep). A deliberate evidenced absence,
   not a silently under-run floor; nothing to reconcile.
6. **Disproved-claims disposition** — the single entry (the `--doc` arm's "0 doctests") is **DISPOSED**: no
   master states it, so no amendment is owed; its home is the frozen route entry, and the correction rides
   the master-record desc at the P7 flip per the operator's wrap directive 4.

**Result: 0 amendments · 0 escalations · 0 open. drift = 0 with no master edited — the chunk touched no
spec-governed surface.** No cascade pass is owed (the cascade walks CHANGED sources; none changed).
