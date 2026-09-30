# Cascade dispositions — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

## The search
`cascade.py sweep` over `cascade-patterns.toml` (10 patterns, every control fired on the pre-pass masters at baseline
`f33d6b7`; listing in `cascade-sweep.txt`). The patterns key on what each retired claim SAYS:
- `no-timeout-read` — the gate never / does not read `timeout.txt`;
- `silent-regress` — a caught→timeout regression passes;
- `rostered-nowhere` — the timeouts rostered nowhere;
- `missed-only-cmp` — the gate compares `missed.txt` alone;
- `expected-set` — the roster's "expected SET" (missed-only);
- `per-chunk-mut` — mutation runs per chunk / where a chunk runs the instrument;
- `chunk-touches` — a chunk that TOUCHES a crate re-runs the tier;
- `pass-empty` — a zero-row unit PASSES on an empty `missed.txt`;
- `crate-local-set` — the fixture family as the SET of crate-local trees;
- `rust-roundtrip` — the fixture pin as a Rust round-trip.

Sections read beside the sweep: test-plan §4 `:228`, §7 `:403-404`, §9 `:469`, §10 `:506`, §12 `:619` / `:621` (whole lines,
before and after the edit); architecture §Stack `:39` and the Directory structure `scripts/` block.

## Every row the listing printed
- `.andromeda/test-plan.md:619` `silent-regress` (standing, `edited`) — **no change**: the matched text is this pass's own
  DATED history ("Until 2026-09-30 … so a caught→timeout regression passed silently"), true of the pre-2026-09-30 gate.
- `.claude/rules/testing.md:19` `no-timeout-read` · `silent-regress` · `missed-only-cmp` · `pass-empty` (leaf) —
  **re-derived**: the mutation bullet now states the epoch-boundary cadence, the four-tally grade, the timeout owner and
  the `selftest` proof.
- `.claude/docs/tests-summary.md:12` `missed-only-cmp` · `pass-empty` (leaf) — **re-derived**. The same line also carried
  "the gate reads `missed.txt`/`caught.txt` only and never `timeout.txt`", which no pattern matched ("never" without
  "reads") — found by reading the line, re-derived with it.
- `.claude/docs/session-learnings.md:669` `no-timeout-read` · `silent-regress` (curation, Tier 3) — **routed to P3**:
  preserve-verbatim; the 2026-09-16 entry records the gate finding with no owner, which this chunk resolved — an in-place
  extension is curation's to write.
- `.claude/rules/frontend.md:55` `expected-set` (curation) — **no change**: a true claim sharing the token (a reached/
  expected SET keyed on element position in the a11y harness), unrelated to the roster.
- Zero-row patterns (`rostered-nowhere`, `per-chunk-mut`, `chunk-touches`, `crate-local-set`, `rust-roundtrip`): each
  control fired on the pre-pass master; after the pass no master, leaf, curation home or base states them.

## Leaves re-derived by the table (beyond the rows)
- `.claude/docs/stack.md:41` (architecture §Stack leaf) — re-derived: the gate's `selftest` verb and fixture tree.
- CLAUDE.md `GENERATED:setup:overview` `scripts/` line (`:16`) — **no change**: it names the directory's key entrypoint
  (`agent-run.{sh,ps1}`), not an inventory; the full `scripts/` listing lives in the architecture tree, amended this pass.
- CLAUDE.md `GENERATED:setup:warnings` — no mutation or fixture claim; unchanged.
- `.claude/rules/verification-harness.md`, `observability.md`, `a11y.md`, `docs/conventions.md` (provenance names an amended
  master) — no mutation / fixture-family / operator-instrument claim (`grep -n 'mutation-gate\|mutation-roster'` → 0
  outside the three leaves above); unchanged.

## Lateral binds
- test-plan §3 ↔ obs-plan §3: §3 untouched this pass. a11y ↔ obs schema: untouched.
- Registry size (D-arch-registry-size): `arch-registry-check.py measure` after apply → §Established Decisions 38 111 B,
  §Occupied Resources 38 028 B, `registries: within target` (neither registry edited).
