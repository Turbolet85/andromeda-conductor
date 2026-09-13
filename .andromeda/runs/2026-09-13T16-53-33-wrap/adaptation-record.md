# Adaptation Record — conductor-0.3.0 · 0-pending wrap · 2026-09-13T16:53:33Z

Path: Setup step 6 **no-op / operator-requested adaptation**. No chunk pending (master-route: 124
`complete`, 0 `pending`), so no P1 report, no P2 fan-out, no P4/P7 gates. P5 route-resolve and P3
curation ran, per the no-op path's two named exceptions.

## Tree classification (why this is the no-op path, not a HALT)

`git status --short` at Setup carried exactly:

| entry | class |
|---|---|
| `M .andromeda/code-metrics.ndjson` | bookkeeping — the code-audit snapshot ledger |
| `M .andromeda/friction-log.ndjson` | bookkeeping — the evolve telemetry ledger |
| `M .claude/session-handoff.md` | bookkeeping — the second writer (session-end convention) |
| `?? .andromeda/runs/2026-09-13T09-57-01-evolve-diagnose/` | untracked run dir — audit trail awaiting a commit to ride |
| `?? .andromeda/runs/2026-09-13T11-34-19-code-audit/` | untracked run dir — same |

All five are named in `session-state-contract.md` as expected-transient; the tree is git-CLEAN for
this path's purposes and all five ride this commit. `working-route.md` was **not** dirty at Setup,
so this is the adaptation case, not the operator-route-edit case.

## Item 1 — insert a corrective entry ahead of `P-025 measurement contract for Pulse`

**Disposition: APPLIED** (factual apply under a recorded operator direction).

Trajectory gate: the class is *new chunk ahead*, normally a HALT. The directive named BOTH the entry
and its disposition, which satisfies the gate per `route-resolve.md` §Operator-requested adaptation —
applied and cited, not re-asked.

**Placement** — inserted at `conductor-0.3.0/working-route.md:17`, directly before P-025 (now `:19`),
under `### Epoch 1 — Foundation: the measurements the closures rest on`. Anchor-only: the diff is
**+2/−0** (entry + its `   ↓` separator) and contains **no `[{marker}]`-prefixed line**. Nothing else
in the tail moved.

**Annotation migration** — none owed. The rule re-pins the previous first markerless entry's
next-entry annotations onto an entry inserted ahead of it; P-025 carried none (verified before the
edit). The tail's only two annotations are `CARRY`s on the Epoch-3 entries (now `:29`, `:31`), which
are not next-entry pins and did not move.

**Title** — the directive's text kept verbatim: 24 words (25 whitespace tokens; the standalone em-dash
is the difference), within the ≤25-word route register, WHAT-not-HOW.

## Item 2 — the entry's freight

**Disposition: APPLIED WITH ONE REFINEMENT.**

The directive supplied the freight as "all in the 6b audit run dir, nothing to re-derive here". Each
coordinate was nevertheless verified against the artifacts before authoring, because entry freight
freezes under drift=0 and the route grammar requires naming coordinates the next reader cannot
cheaply re-derive. Four of five reproduced exactly:

| freight claim | measured | verdict |
|---|---|---|
| 25 survivors across `c-mutation-{unit}.json` `survivors[]` | 25 (cli 3 · core 0 · run 5 · tauri 3 · verify 14) | exact |
| 8 of them in `bin/stub_pulse_mcp.rs` → 17 in scope | 8 (`:18 :37 :45 :46 :51 :58 :63 :75`); 25 − 8 = 17 | exact |
| `c-duplication.json` `top[0..4]` = five `conductor-emit/tests/` pairs, 36–44 lines | 44 · 42 · 39 · 38 · 36, `egress.rs` in four | exact |
| `journal.rs:158-181` ↔ `record.rs:137-160`, both `#[cfg(test)]` | both ranges hold the identical eleven-key array; `#[cfg(test)]` opens at `journal.rs:76` / `record.rs:75` | exact |
| `unused_deps` = `conductor-emit → conductor-core`; **the crate's only reference is an intra-doc link at `error.rs:5`** | `{"conductor-emit": ["conductor-core"]}` ✓ — but **three** references exist under `conductor-emit/src` | **refined** |

**The refinement.** `grep -rn 'conductor_core\|conductor-core' crates/conductor-emit/src/` returns
three hits: `error.rs:5` is a resolvable rustdoc intra-doc link (`` [`conductor_core::CoreError`] ``),
while `latency.rs:8` and `topology.rs:21` name the crate in plain prose backticks and resolve to
nothing. The directive's claim is true under the reading that matters — only one *resolvable*
reference — but a take-up reader running the obvious grep gets 3 and would read the annotation as
wrong. The freight was authored to the measured three, so that grep now confirms the entry instead of
contradicting it, and the removability prediction is marked `hypothesis:` because no build was run
without the dependency.

This is a factual refinement of freight, not a change to the entry, its placement or its
disposition — all three stand exactly as directed.

## Item 3 — the handoff's superseded `Next`

**Disposition: APPLIED.** The directive's CONTEXT CORRECTION is shipped by this wrap's handoff
rewrite: `Next` now names the inserted entry; P-025 follows it.

## Curation (P3)

Ran — the conversation carried corrections. **0 entries applied**, 2 candidates analyzed, 0 conflicts,
0 deferred:

- *The audit's `unused_deps` signal needs a resolvability check, not a grep count* — deduped (Filter 1)
  as an additive facet of the standing CLAUDE.md false-positive-grep entry, then scored **exactly 0.6**
  (measurement +0.4, specific-technical-detail +0.2) → rejects at threshold. Neither conditional +0.2
  fired: the fact reached a durable home this wrap, the working-route entry's own freight.
- *Andromeda route-file structural grammar* (underscore-delimited preamble; bare-line master records) —
  scope-excluded as friction telemetry, and it restates grammar the new-session skill already documents.

## Out of scope by design (recorded, not done)

Per the directive, left to the next audit's confirm: the first mutation measurement of
emit / report / timeline / faults; `conductor-core`'s shard rotation; applying the standing stub
exclusion. The audit's own Skips section corroborates the first two (`declined` — outside the
operator-chosen baseline-parity scope; `budget-exhausted` — shard 1/4 as the baseline ran).

## Gates

No P7 on this path. No light gate (no chunk, no plan Test Commands), no drift gate (no fan-out), no
coverage flip (no capability claimed). Coverage is unchanged at **1/11 verified · 10 unclaimed** —
re-read, not inherited.
