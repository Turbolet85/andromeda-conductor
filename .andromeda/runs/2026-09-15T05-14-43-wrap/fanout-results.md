# Fan-out results — 2026-09-14-emit-scrubber-and-percentile-math-under-test

**Pass:** P2 reconcile · 7 Explore doc-agents, one per spec source, one parallel batch.
**Outcome: 7 / 7 returned `proposals: []`. Zero fan-out proposals, zero escalations.**

No raw twin is saved for any doc: every return carried an unambiguous `proposals: []`, so stripping the
surrounding evaluation commentary changed no proposal content (raw ≡ saved for the parsed value). The
returns did carry HTML-escaped entities in their COMMENTARY (`&lt;u64&gt;`, `&amp;`) — the known transport
escaping — but no proposal field was affected, the parsed list being empty in all seven.

| doc | verdict | detectors evaluated |
|---|---|---|
| `architecture.md` | `proposals: []` | D-arch-resources · D-arch-decisions · D-platform-claim |
| `security-plan.md` | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim |
| `design-system.md` | `proposals: []` | D-design-tokens · D-design-derived-count · D-platform-claim |
| `layout-templates.md` | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim |
| `test-plan.md` | `proposals: []` | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim |
| `obs-plan.md` | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim |
| `a11y-plan.md` | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim |

## Why each returned empty (the load-bearing reasons, per agent)

- **No new surface.** The report's *Coverage of new surfaces* bullet states "no new external surface,
  hot-path op or UI element"; the only symbol movement is the REMOVAL of three `LatencyProfile` accessors
  with 0 call sites. This is what emptied D-arch-resources, D-design-tokens, D-layout-surface,
  D-a11y-surface and D-obs-instrumentation.
- **Dependencies are workspace dev-deps at an unchanged package count.** `proptest` + `rstest`, both
  `.workspace = true` and both already named in arch's dev-test stack (`architecture.md:208`) and
  test-plan §2/§4/§7/§12; lockfile 562 → 562, delta is edges only. This emptied D-arch-decisions,
  D-security-deps and D-tests-framework.
- **No schema, no logging, no telemetry write.** *Schema / config* says "No migration, no violation schema,
  no scrub/redaction shape changed"; the chunk adds tests over the pre-existing scrubber and pins
  `%APPDATA%` / `~/.cargo` as the deliberate NON-scrub boundary belonging to `conductor-core::redact` —
  which obs-plan §9/§11 already attribute there. This emptied D-obs-redaction, D-obs-stack and
  D-a11y-obs-schema.
- **The moved counts appear in no master as literals.** Independently swept by three agents and by the
  orchestrator: `\b(ten timeouts|57 survivors|unviable|227|221|115)\b` over all seven masters returns ONE
  hit, `test-plan.md:612`, which is `conductor-verify`'s dated 2026-09-04 spawn.rs record ("16 mutants: 13
  caught, 1 unviable, 2 accepted") — a different unit and population, dispositioned NO CHANGE. This
  emptied all three D-*-derived-count detectors.
- **The gate changes AFFIRM the standing posture rather than retiring it.** *Harness / gate surface*
  records the gate as operator-local with no CI step invoking it and `scripts/agent-run.{sh,ps1}` keeping
  their five-command surface, untouched — which is what §4:228 / §9:466 already state. The two
  `Spec claims disproved by measurement` entries are instrument semantics, not capability claims about
  where a leg can run, so per D-platform-claim's own rule (a STATING sentence must be quotable) there is
  no hit in any of the seven.
- **The §12 amendment is not detector drift.** The test-plan agent identified it explicitly as an
  EXPECTED amendment already owned by this wrap's P2 — consistent with the report's *Spec-master edits:
  none at report time* — and correctly declined to propose it.

## Orchestrator-raised (Validate check 5)

ONE amendment, raised by the orchestrator because no detector proposed it and the chunk plan carries it as
its sole `Expected amendments (wrap)` entry (`plan.md:378-381`): **`test-plan.md` §10 + §12 — the
`conductor-emit` accepted-deliberate class, its equivalence basis, and the roster's registration change.**
Verdict ROUTINE (the report substantiates it; the operator's wrap directive item 4 settles its content and
its basis). Applied in this pass; see `.andromeda/test-plan-amendments.md` §
`2026-09-14-emit-scrubber-and-percentile-math-under-test` for the four body edits, the sweep and its
dispositions.

One rule is PROPOSED at the wrap card rather than minted here: the playbook has no entry for a mutation
survivor accepted on proven EQUIVALENCE (all 46 existing rules cover other classes; §10's prior wording
admitted only "a standing rule prescribes the untested shape"). A recorded operator direction settles this
proposal, never the class — so the rule goes to the operator.

## Escalations

None. Zero fan-out proposals, so Validate checks 1 and 2 had nothing to match or contradict; check 3's one
divergence (the timeout criterion re-asserted at the measured 15 against the plan's "ten") is JUSTIFIED by
the criterion's own binding to "this chunk's own run" and is directive-ratified, recorded in the report's
*Deviations*; check 4's absence claims each cite their search with every hit dispositioned; check 6's
single `Spec claims disproved` entry (the gate never reads `timeout.txt`) is DISPOSED by routing to P5 as
a CARRY.

**Drift = 0 at exit of P2.**
