# Session Handoff

**Last Updated:** 2026-09-15T05:57:40Z
**Branch:** `build/conductor-0.3.0` · **0 ahead of `origin/build/conductor-0.3.0` at this write** — measured,
not inherited; this wrap's commit will leave it 1 ahead. The operator pushes.
**Status:** clean — **all 14 light-gate entries green by their `expect`**, no deferral, no `leg` entry, no
red. (Entry 12's `exit 1` IS its expect: the host-path grep finding nothing.)
**Last Commit:** `feat(2026-09-14-emit-scrubber-and-percentile-math-under-test)` — see below.

## Position
- Done: **`2026-09-14-emit-scrubber-and-percentile-math-under-test`** — `conductor-emit`'s first
  dispositioned mutation score: 221 mutants over `exception.rs` + `latency.rs` whole, ending 1 missed /
  202 caught / 15 timeouts / 3 unviable, with the single survivor ratified as an equivalent mutant.
  Master-route is now **127 complete, 0 pending** — flipped at P7 after the gates, with the record's desc
  corrected in the same write ("the ten timeouts" → the measured FIFTEEN, naming why) and its line
  flip-compacted 3086 → 204 chars, the verbatim original archived to `route-archive.md`.
- Next: **`Structurally-dead assertion class retired`** — `conductor-0.3.0/working-route.md:24`, head of the
  markerless tail, second entry of **Epoch 2 — Scenario assertion hygiene**. No `PREREQ:`, no `BLOCKED-ON:`.
- Coverage **2/11 verified · 9 unclaimed** (`v3-02`…`v3-06`, `v3-08`…`v3-11`) — unchanged: this chunk
  claimed nothing (the plan read the pool and found all nine about the scenario corpus, a11y, live/dynamic
  or secret-scanning), so the coverage gate is a `claimed 0 — gate no-op`.

## Work done
Brought `conductor-emit`'s first-measured mutation surface under discriminating test — the `exception.rs`
host-path scrubber trio and `latency.rs` percentile math — and dispositioned the result. **+325/−69 over
seven files** from implement, plus one wrap-time source line (below). Three public `LatencyProfile`
accessors were REMOVED (0 callers workspace-wide); `proptest` + `rstest` joined the crate's
`[dev-dependencies]` as workspace deps, lockfile package count unchanged at **562** (edges only). The
roster gained a `[[unit]]` registration concept and `conductor-emit`'s row; `scripts/mutation-gate.py`
gained declaration-based registration, `files` scoping and a 0-mutant FAIL guard.

## Gates (P7.1)
All 14 green. Workspace **982 tests** (909 → 982), `conductor-emit` **150** (77 → 150), coverage **94.58 %
lines**, `cargo audit` 0 (1246 advisories · 562 packages · 7 allowed) over a clean advisory-db porcelain,
`cargo deny` 0, `cargo doc` still "generated 5 warnings" and 0 `conductor_core`.

**The mutation tier reproduced on a THIRD independent run:** 221 mutants, `missed 1 · caught 202 ·
expected 1`, `MUTATION GATE conductor-emit: PASS`, same lone survivor. Two things fall out. The equivalence
acceptance is not a one-run artefact. And the unviable caveat is now confirmed three ways —
`skip_line_number_suffix`'s `+=`→`-=`, recorded UNVIABLE in run 2, is a **TIMEOUT** here as it was in run 1
(at `:341:7`, one line below run 2's `:340:7`, the shift being this wrap's doc-comment edit). A mutant that
will not compile cannot time out.

**One divergence recorded, not smoothed:** the relay carried coverage 94.74 %; this wrap's own run of the
plan's gate command measures **94.58 %**. The criterion is ≥ 60, so nothing turns on it — but the measured
figure is the one reported.

## Drift resolved
**All seven detectors returned `proposals: []` — zero fan-out drift.** The pass's one amendment was
orchestrator-raised under Validate check 5 (the plan's sole `Expected amendments (wrap)` entry):
`test-plan.md` §10 + §12 gained `conductor-emit`'s accepted-deliberate member, the **equivalence** basis as
a second acceptance ground beside "a standing rule prescribes the untested shape", and the roster's
registration change. Cascade re-derived two leaves (`.claude/rules/testing.md`,
`.claude/docs/tests-summary.md`); `CLAUDE.md`'s warnings block carries zero mutation content, so none was
owed there. Sweep control fired in 4 files before any 0-hit was trusted. **Drift = 0.**

## Notes
- **The timeout criterion moved 10 → 15 and is reported at the measured value.** The plan's criterion says
  "all ten timeouts"; the ten were the audit's `--shard 1/4` (115 of 227 mutants). This chunk's population
  is the two files WHOLE (221), which produced fifteen. Discharged by the ledger's by-class table, not by
  matching the number. The plan's text stands as the record of what was asked.
- **One instrument gap is now route-owned:** `scripts/mutation-gate.py` never reads `timeout.txt`, so a
  caught→timeout regression passes it silently. Pinned as a `CARRY` on `Scenario-assertion audit gate`
  (`working-route.md:28`) at an operator-answered armed halt — no markerless entry named the mutation gate,
  so the directive's target had no measured referent.
- **A second "gap" was drafted and withdrawn.** The report briefly claimed the gate's `(file, description)`
  identity contradicts the documented rule; `test-plan.md:615` already states the multiset match and that
  two members may share that pair. The measurement (three same-description `quantile` mutants) stands; the
  conclusion did not. Logged as a recurrence — see below.
- **Two of three dictated coordinates were off and the measured ones were used:** the fingerprint doc
  comment is at `exception.rs:161`, not `:158-160`; the gate's verdict block is `:137-151`. Every other
  figure in the wrap directive reproduced exactly (+325/−69, lockfile 562, 1/202/15/3 = 221).
- **One wrap-time source edit:** `exception.rs`'s `fingerprint` doc comment re-stamped to record the SUT
  transcription as re-verified current at Pulse HEAD `83d4060` (`git log efabe8e..83d4060 --
  crates/buffer/src/fingerprint.rs` → 0 commits, measured against the SUT repo on disk this wrap). Scoped to
  that ONE site: `scenario.rs:691` and `canary.rs:115` cite different SUT files whose currency was not
  measured, and were deliberately left alone.
- **Unviable moved 1 → 3, and two of the three are a host LNK1104 link race, not a code fact** — both were
  observed dead in run 1 (`:286:9` caught, `:340:7` timeout). Without this the next audit reads it as a
  property of the code.
- **Curation: T1 0 · T2 2 · T3 0 · 1 below-threshold reject · 1 recurrence.** Both Tier-2 entries went to
  `host-win32.md`: `$TMPDIR` is unset (a `$TMPDIR/...` redirect silently writes to `/`; the OS temp is
  `$TEMP`), and a saved sub-agent extract must be persisted with the Write tool, never a text-mode python
  write (measured: all twelve extracts in one phase run dir are 100 % CRLF).
- **`recurrence-despite-learning`: CLAUDE.md's 2026-08-09 entry** ("Before asserting that document A says X,
  grep A", extended to an agent asserting a false claim about its OWN source) did not prevent the withdrawn
  finding above. Logged rather than re-minted as a third entry — the remedy belongs in the owning step's
  reference, not in another corpus line.
- **Playbook rule PROPOSED, not minted:** no rule covers a mutation survivor accepted on proven
  EQUIVALENCE (46 rules, none matching). The operator's directive settled this instance; the class ruling
  is the operator's.
- **`v3-08` stays BLOCKED** — unchanged; it needs a Pulse release emitting the contracted observable
  (Epoch 4, `working-route.md:42`).
- **Still open from prior sessions:** the n=1 deferred escalation class; the audit-debt chunk's discarded
  wrap `gates` evolve record. The `quantile` 14-vs-11 correction remains queued for the next
  `code-metrics.ndjson` `corrections[]` — deliberately NOT re-filed here.
- **Last failed command:** none.
