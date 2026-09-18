# Fan-out results — 2026-09-18-real-model-leg-posture-and-grading-rule

7 Explore doc-agents, one per spec source, one parallel batch. 19 detectors total (each doc's own set plus the
cross-doc `D-platform-claim`).

| doc | verdict | detectors evaluated |
|---|---|---|
| **arch** | **2 proposals** (raw twin: `.raw-fanout-arch.md`) | D-arch-resources · D-arch-decisions · D-platform-claim |
| security-plan | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim |
| design-system | `proposals: []` | D-design-tokens · D-design-derived-count · D-platform-claim |
| layout-templates | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim |
| test-plan | `proposals: []` | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim |
| obs-plan | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim |
| a11y-plan | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim |

## Why the six returned clean

The chunk's Changes carry `Symbols / APIs: none` (zero `.rs` delta), `Dependencies: none`, `Schema / config:
none`, `Harness / gate surface: none — deliberately`, and `Counts / qualifiers moved: none — verified`. With
those bullets empty, most detectors have no trigger by construction: no new external-input surface
(D-security-input), no dependency (D-security-deps), no UI element (D-design-tokens, D-layout-surface,
D-a11y-surface), no moved count to sweep (the three `*-derived-count` detectors), no new operation symbol
(D-obs-instrumentation), no telemetry dependency (D-obs-stack), no logging or artifact write
(D-obs-redaction), and no harness/schema change (D-tests-obs-harness, D-a11y-obs-schema).

`D-platform-claim` was evaluated by all seven and declined by all seven on the same reasoning: the report's
single `Spec claims disproved by measurement` entry retires a **fixture-content** claim, not a platform,
runner or driver verdict, so the detector's trigger — a sentence STATING a retired verdict, quoted — has no
subject. Two agents noted independently that the green CI run in the report's cross-project bullet *confirms*
rather than falsifies the `windows-2022` readings their docs already carry.

## Two independent corroborations worth keeping

- **test-plan** checked its own two sites touching the `evidence_refs` fact (`test-plan.md:76`, `:335`) and
  found both already stating the CORRECTED reading — "the fixture's `evidence_refs` now populates with a
  constant `det-*` triple — measured 2026-08-18 at SUT HEAD `efabe8e`". It proposed nothing because there is
  nothing stale there.
- **obs-plan** reported the same for its own baseline (`obs-plan.md:131`, `:314-315`), and that obs-plan
  carries no `deterministic_inference.rs:35` citation and no "pinned to `[]`" wording at all.

Together these confirm the report's finding that **no spec master carries the stale claim** — only the
`.claude/rules/verification-harness.md` rule file did, which is P3 curation's channel, not P2's.
