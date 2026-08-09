# Fan-out results — 2026-08-09-sut-drift-check

7 doc-agents, one per spec source, one parallel batch. **Every doc returned `proposals: []` — zero drift.**

Each return carried the YAML payload followed by evaluation notes. Per `amendment-flow.md` a raw twin is kept
only for a doc whose return needed stripping or carried proposals; all seven were proposal-free, so their
verdicts and the substance of their notes are consolidated here rather than in seven near-empty twins — the
notes contain the four out-of-scope observations below, which is the content worth preserving.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | `proposals: []` |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | `proposals: []` |
| design-system | D-design-tokens | `proposals: []` |
| layout-templates | D-layout-surface | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | `proposals: []` |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` |

## Why each held

- **arch** — Changes state no IPC/endpoint/port/socket/env-var/crate added, no dependency moved, schema none. The
  new items are same-crate library symbols inside the already-registered `conductor-core`; arch registers at
  system grain, not per-function API (the standing library-symbol over-reach rule, playbook:37). `CoreError::SutDrift`
  is thiserror-per-seam on the `Err`-is-harness-fault side of the verdict/error wall. `BTreeSet` / `let-else` are std.
- **security-plan** — no new external-input surface (both inputs are already-validated in-process artifacts, and
  `contracts/pulse-capabilities.toml` is read-only and unedited); the sidecar spawn / preflight / data-dir are
  untouched; `Cargo.toml` and `Cargo.lock` untouched with `cargo audit` exit 0 and `cargo deny` all four classes ok.
- **design-system / layout-templates** — no UI rendered, no cli verb, no webview region; the report's coverage row
  marks the new fn explicitly *not* an external surface. Both invariants hold vacuously.
- **test-plan** — 7 unit tests at the tier §2/§4 mandate for a `conductor-core` path; every command in the Outcome
  table is one test-plan already specifies; determinism proven by the byte-identical-message test; the envelope /
  JSONL / `runs.db` schemas are byte-unchanged so the §3 ↔ obs §3 bind is untouched.
- **obs-plan** — the new fn is not one of §4's must-trace paths and carries no span by decision (§11 bans
  instrumenting non-critical paths); no OTel SDK/exporter/metric; no logging or artifact write added, and the one
  new operator-visible string is test-asserted free of host paths and internal type names.
- **a11y-plan** — no interactive UI element; the four must-be-accessible paths are untouched; the violation schema
  and the obs envelope it binds to are byte-unchanged.

## Out-of-scope observations (raised by detectors, NOT proposed — verified first-hand by main)

None is this chunk's drift: the report's Changes add no crate, no UI, and no classification row, so each is
pre-existing committed-tree state. Each claim was checked with grep rather than taken from the detector.

| # | claim | verified | status |
|---|---|---|---|
| 1 | arch §Stack says "60-P-ID coverage tables" | **true** — `architecture.md:33` | pre-existing |
| 2 | design-system says "all 60 capabilities" / "the full 60-row wall" | **true** — `design-system.md:7`, `:257` | pre-existing |
| 3 | layout-templates contradicts itself: "82 loaded" vs "the full 60-row wall (P-001..P-060)" | **true** — `layout-templates.md:37` vs `:121` | pre-existing; residue of an incomplete de-hardcoding sweep in `2026-08-08-sut-capability-manifest`, which updated the header strip (`:37`) but not the component prose (`:121`) |
| 4 | obs-plan says "8 workspace crates" while there are 9 | **true** — `obs-plan.md:25`, `:645` | pre-existing; stale since `conductor-run` was extracted (2026-06-26) |

**Disposition:** dismissed as not-this-chunk's-drift under the established precedent (`playbook.md:46` — a
pre-existing inconsistency the chunk did not introduce belongs to a dedicated reconcile pass, not this wrap),
and **pinned as route CARRYs in P5** so they are not lost: #1–#3 (the 60-vs-manifest wording) onto the Epoch-1
*Current-SUT coverage classification* entry, which is the chunk that makes the classification cover the ledger
and is the natural owner of that wording; #4 (crate count) onto the Epoch-6 *Dependency polish* entry, which
already carries three doc-vs-artifact gaps from the prior chunk.

**Drift = 0:** 0 proposals · 0 escalations · 0 amendments applied · no spec body changed ⇒ no cascade required.
