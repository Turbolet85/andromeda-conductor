# Phase 4 review — feedback round 1 (operator)

## Decision 1 — the five deferred validator Inserts

**Ruling: none as chunks of their own — hold the intent's scope — with one fold and two residuals.**

- **Folded, not minted:** the security Insert (`CI-fetched WebView2 runtime disposition`) goes into **`v3-02`'s
  chunk plan**, not a chunk of its own, on the operator's stated ground that *the pin's only stated exit
  condition IS `v3-02`'s terminal fork*. Carried to the matrix `notes` for `v3-02` so promotion inherits it.
- **Recorded as residuals** in `requirements.md` §Carried residuals: the unbuilt console surfaces (design +
  a11y, deduped) and Critical Path 7's control-panel-launched parity (tests).
- **Route line count unchanged by this decision** — the fold and the residuals add no chunk.

### The coupling the operator attached to the console-surfaces residual

Recorded verbatim in substance because it changes how `v3-02` must be planned, not merely what is remembered:

> If the routine arm starts gating in CI, a11y-plan §11's landmark ban could turn the unshipped footer into a
> RED **in that very gate** — so `v3-02`'s planning must measure whether the routine specs pass against the
> current console before promising a green gate.

This is a falsification risk aimed at `v3-02`'s own acceptance: "routine arm's asserted verdict green in CI" is
a promise that cannot be made from the CI-endpoint question alone, because a second, independent cause of red
(a missing `contentinfo` landmark on a window the ban covers) lives in a surface this version deliberately did
not build. It travels into `v3-02`'s matrix `notes` as a planning obligation, alongside the WebView2 pin.

## Decision 2 — `secret-scanning-ci-gate`

**Ruling: absorb as a 0.3.0 capability.** Minted as **`v3-11`** (Theme 5), removed from §Carried residuals,
and given one chunk at the head of Epoch 5 so the full-gate regression that follows exercises it. Its 0.2.0
carried-residual entry — "Revisit at 0.3.0 scoping" — is thereby discharged rather than re-carried a third time.

Not intent-sourced, and the requirements line says so: the authored intent neither names nor excludes it, so
`observed_gap` will be `null` in the matrix while every intent-derived capability carries its OBSERVED clause.

## Net effect

- Chunks **13 → 14**; epochs unchanged at 5; Epoch 5 goes 2 → 3 chunks.
- Capabilities **10 → 11** (`v3-01`…`v3-11`).
- Carried residuals **2 → 3** (secret-scanning out to a capability; console surfaces and Critical Path 7 in).
- Dispositions unchanged for the four intent findings.
