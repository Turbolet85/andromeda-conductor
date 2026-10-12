# Operator pass — 2026-10-12-panel-shaped-types-retired

Made on the operator's explicit word, 2026-10-12 (this session): a product fix first (the catalogue
roll-up reads `1 scenario` for one, `N scenarios` otherwise, pinned by
`render::tests::scenarios_rollup_is_singular_for_one_and_plural_otherwise`), the gate block re-run
whole, then entry 22, the pre-CI commit, entry 23 (the guarded push) and entry 24 (the CI read).

## Gate block re-run after the roll-up fix

```
entries 24 · green 21 · red 0 · recorded 0 · timeout 0 · not-run 3
```

1218 tests passed under nextest (`bash scripts/agent-run.sh run`) and under
`cargo test --workspace --lib --bins --tests`; the new roll-up test ran by name in both.

## Entry 22 — hygiene (`--operator 22`)

```
 22 probe       green · exit 0 · 0.3s · 446 B → 22.log · history: unmoved · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ga… (71 chars)
entries 24 · green 1 · red 0 · recorded 0 · timeout 0 · not-run 23
```

Its output (the verdict line, copied from `22.log`):

```
hygiene: clean — read 52 (runs 45 · evidence 0 · inputs 7) · trails 14 not read · copies 5 not read by P1 — 0 host paths kept · binary 0 not read by P1
```
