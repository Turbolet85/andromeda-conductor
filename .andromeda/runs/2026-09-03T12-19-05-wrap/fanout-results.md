# Fan-out results — 2026-09-03-conductor-run-composition-root-survivors-dispositioned

7 Explore doc-agents, one per spec source, one parallel batch. Detectors scoped from `drift-base.md`;
each doc got its own entries plus the cross-doc `D-platform-claim`.

| doc | verdict | detectors evaluated |
|---|---|---|
| architecture | `proposals: []` | D-arch-resources · D-arch-decisions · D-platform-claim |
| security-plan | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim |
| design-system | `proposals: []` | D-design-tokens · D-design-derived-count · D-platform-claim |
| layout-templates | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim |
| **test-plan** | **4 proposals** (raw twin: `.raw-fanout-test-plan.md`) | D-tests-coverage · D-tests-framework · D-tests-obs-harness · **D-tests-derived-count** · D-platform-claim |
| obs-plan | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim |
| a11y-plan | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim |

**Entity decode:** the `test-plan` return carried `&amp;` inside two `section:` values ("Quality Gates
&amp; Coverage Targets"); decoded before parsing, `entities=0` on the decoded text. The other six returns
were entity-clean. Only `test-plan` warranted a raw twin (it carried proposals AND needed decoding).

## Why the six empty returns are substantive, not vacuous

This chunk's Changes bullets are `none` for Symbols/APIs, Crates/modules, Dependencies, Schema/config,
Harness/gate surface, and Coverage-of-new-surfaces — production source is byte-unchanged and every added
line is a test. Six detectors' invariants are therefore not engaged, and each agent said so with a cited
basis rather than a bare null:

- **architecture** — nothing lands that §Occupied Resources / §Standard Contracts would owe a registration
  for; `cargo audit`'s red exit is the already-recorded external advisory-DB fault (`architecture.md:196`).
  Arch never attributes the `declares` env read to any crate, so the re-attribution has no duplicate there.
- **security-plan** — the sidecar appears only as a measurement, and matches §Security Anti-Patterns
  (`:314`, `:354`) as written; §Dependency Security's dependency-delta condition is not engaged at zero
  delta; the advisory-DATABASE bounded-wait remedy at `:176-177` is exactly what the report records.
  Independently noted: **`security-plan.md:114` already attributes `declares` correctly**, which is why
  the re-attribution is test-plan-only.
- **design-system** / **layout-templates** — the moved counts (`declares` 6→4; the 25→8 tally) collide
  numerically with unrelated literals in both docs (the 6-state lamp set, the 82-capability manifest count),
  and both agents correctly refused those as different referents rather than proposing a rewrite.
- **obs-plan** — no new must-trace operation; the only library named (`tokio`'s `io-util`) is explicitly
  not added; the one artifact write was scanned host-path-clean.
- **a11y-plan** — no interactive element; neither the violation schema nor the obs envelope moved; the
  fixture files the routine arm seeds are untouched.

## test-plan proposals (all applied)

1. **D-tests-derived-count** — §12 `declares` bullet: retire the `x6` literal, name the surviving arm SET,
   record the 2026-09-03 shrink. *Applied.*
2. **D-tests-derived-count** (`dependent-of` #1) — §10: drop the `×6` from the inline roster reference.
   *Applied, folded into one coherent edit with #3.*
3. **D-tests-derived-count** — §10: re-attribute the `declares` env-reading edge `conductor-verify` →
   `conductor-run`. *Applied.*
4. **D-tests-derived-count** — §12: new bullet recording `conductor-run`'s accepted-deliberate classes B
   and C. *Applied.*

**Validation:** all four routine (derived-count + attribution reconciliation, each substantiated by the
report's own measurements); **0 escalations**. Check 4 (absence-needs-evidence) re-derived by the
orchestrator rather than accepted: `grep -n 'declares'` over test-plan returns 3 hits (`:500`, `:607`,
`:608` — the last is `conductor-tauri`'s triple, correct), `grep -nE '(x|×)6'` returns 2 (both amended),
and the cross-master sweep returns zero hits outside test-plan. Check 5 (expected-amendments floor): the
plan's three entries are all covered by proposals 1-4. Check 6 (disproved-claims): all three report entries
disposed — two by these amendments, the third (the code-audit "Suspected shape") routed to `scope.md`'s
premise closure, since no spec master states it.

**Cascade:** test-plan's two leaves (`.claude/rules/testing.md:19`, `.claude/docs/tests-summary.md:12`)
carry the read-out rule and the phrase "the accepted roster is a SET recorded in test-plan §12" — no count,
no crate attribution — so they were consistent by construction and required no re-derivation. That
set-naming shape is what made them immune to this amendment.
