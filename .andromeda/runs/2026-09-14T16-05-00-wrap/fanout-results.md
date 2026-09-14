# Fan-out results — 2026-09-13-p-025-measurement-contract-for-pulse

Seven doc-agents, one per spec source, one parallel batch. **3 proposals across 2 docs; 5 docs clean.**

| doc | verdict | detectors evaluated |
|---|---|---|
| architecture.md | **2 proposals** (raw twin saved) | D-arch-resources ✗ · D-arch-decisions ✓ · D-platform-claim ✓ |
| security-plan.md | `proposals: []` | D-security-input ✓ · D-security-subprocess ✓ · D-security-deps ✓ · D-platform-claim ✓ |
| design-system.md | `proposals: []` | D-design-tokens ✓ · D-design-derived-count ✓ · D-platform-claim ✓ |
| layout-templates.md | `proposals: []` | D-layout-surface ✓ · D-layout-derived-count ✓ · D-platform-claim ✓ |
| test-plan.md | `proposals: []` | D-tests-coverage ✓ · D-tests-framework ✓ · D-tests-obs-harness ✓ · D-tests-derived-count ✓ · D-platform-claim ✓ |
| obs-plan.md | **1 proposal** (raw twin saved) | D-obs-instrumentation ✗ · D-obs-stack ✓ · D-obs-redaction ✓ · D-platform-claim ✓ |
| a11y-plan.md | `proposals: []` | D-a11y-surface ✓ · D-a11y-obs-schema ✓ · D-platform-claim ✓ |

## Clean returns — the bases each agent recorded

- **security-plan** — returned bare `proposals: []` per the contract (YAML only). No new external-input
  surface, no sidecar-spawn/data-dir touch, no dependency delta.
- **design-system** — no new UI element (Coverage row records `tokens n/a`, no render site); the
  `contracts/` 4 → 5 count appears in no palette row, ANSI-map entry, token label or reuse tally (the two
  textual `four`/`fourth` hits are an unrelated typographic tier at `:27` and a cross-reference at `:257`);
  no platform verdict retired.
- **layout-templates** — no surface to map to a wireframe; `grep 'contracts'` over the doc returns **zero**
  hits, so no caption/wireframe/selector bakes the moved count; its platform-adjacent sentences (`:108`,
  `:190`, `:261`, `:297`, `:328`) name platforms without stating a retired verdict.
- **test-plan** — no new code path (the one `.rs` touch is doc-comment-only, the TOML comment-only); gates
  1–4 are exactly the §4-mandated runners; harness surface unchanged; the doc's six `contracts/` occurrences
  (`:92`, `:145`, `:148`, `:280`, `:287`, `:610`) each NAME a member or the directory and none states a
  member count.
- **a11y-plan** — no interactive element added; neither the violation schema nor the obs log schema moved
  (criterion 4 confirms no span/envelope/critical-path delta), so the eleven-key envelope reproduced in §1
  and §3 still matches obs §6; `would lift it` / `last_seen` / `last_observed_unix_nanos` all return zero
  in the doc.

## Re-derivation-tell check (the prompt's own ban)

All three proposals rest on the report plus their own document. `basis` fields point at
`architecture.md:175-178`, `architecture.md:233` and `obs-plan.md:349` — the doc under evaluation and the
claim site the report itself names. **No proposal cites a source, manifest or lockfile location the report
does not carry**, so none is rejected before validation.

## Transport note

The obs-plan return arrived HTML-escaped (`&` rendered as the entity inside its `rationale`); decoded on
save, `entities=0` probed clean on the stored twin. The other six returns carried no entities.
