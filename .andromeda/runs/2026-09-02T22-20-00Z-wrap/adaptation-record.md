# Adaptation Record — 0-pending wrap · 2026-09-02T22:20:00Z

**Path:** no-op (0 pending in master · tree dirty only with expected-transient bookkeeping —
`friction-log.ndjson` + `session-handoff.md`), carrying an operator route-adaptation request.
**Phases run:** P3 curation · P4 code-graph refresh · P5 route-resolve. No P1 report, no P2 fan-out,
no P7 master flip (no chunk to flip).
**Directive:** operator wrap arguments, 2026-09-02 — context correction + 6 items + anchor-only pinning.

---

## Context correction (accepted)

The handoff's `Next: /andromeda-phase → A11y CI gate` is **superseded** by the boundary-#3 ruling
(2026-09-02): CODE-facing audit findings become the corrective FIRST chunk of the new epoch, so every
later 6a chunk runs on hardened machinery. Recorded here because the handoff's own Position line was the
artifact that carried the stale prescription; P6 rewrites it.

Second correction, measured this wrap: the handoff also predicted the **evolve nudge** for Epoch 5. It
does not fire — `.andromeda/runs/2026-09-02T15-24-02-evolve-diagnose/proposals.md` is titled
`Epoch 5 "Verification surfaces"`, i.e. the epoch is already diagnosed. The prediction was written before
that run existed.

---

## Items and dispositions

| # | item | disposition |
|---|---|---|
| 1 | Mint M1 at the 6a head | **APPLIED** — minted as the first markerless entry |
| 2 | Mint M2 after M1 | **APPLIED** — minted directly after M1 (the operator's lean; the tail was the named alternative) |
| 3 | Playbook seed rule six | **APPLIED** — appended verbatim; 36 → 37 rules |
| 4a | CORRECTION to the CLAUDE.md code-graph rule | **APPLIED** — in place, both false spans, dated tag |
| 4b | PostToolUse rustfmt-only ruling | **APPLIED** — Tier 1 |
| 4c | Pre-2026-09-02 "0 callers" claims | **APPLIED** — folded into 4a's correction (additive facet, not a sibling entry) |
| 4d | P14 frontend assertion rule | **APPLIED** — Tier 2, `.claude/rules/frontend.md` |
| 5 | Friction retraction | **APPLIED** — `scope: "problem"`, `index: 0` |
| 6 | `preflight-env` liveness probe | **APPLIED as MINT** on the operator's stated lean — see the open point below |
| — | Anchor-only pinning | **APPLIED** — the 45th standing cargo-audit PREREQ migrated verbatim |

### The three minted entries (Epoch 6a head, in order)

1. **Mutation tier restored for conductor-tauri** — the parity test's CLI binary declared in the build
   graph rather than found by chance. Source `runs/2026-09-02T15-49-17-code-audit/proposals.md:35-77`
   (`mutation-tier-aborted`; 43 mutants planned, **0** tested, exit 4). Acceptance includes the
   runner-portability gate on a FRESH target dir. The FORM (escargot-style build · test moved to the
   bin-owning package · artifact dependency) is deliberately unnamed — P3-research territory, per the
   directive.
2. **conductor-run composition-root survivors dispositioned** — every standing survivor gains a killing
   test or a cited `accepted_deliberate` entry. Source `proposals.md:78-154` (`monotonic` file_max,
   898 → 1012 → 1115 code lines; 24 of 25 crate survivors in that one file; **12 of 18** carried
   survivors undispositioned). Split along the proposal's own survivor clusters. Whether the FILE also
   splits is research territory and the disposition half does not depend on it.
3. **Live-Pulse preconditions probed before a leg is scheduled** — so an absent SUT is named once rather
   than absorbed per chunk. Source `runs/2026-09-02T15-24-02-evolve-diagnose/proposals.md:548` (L3
   band-aid; 5 in-epoch facts, 4 more in Epoch 4). Probe subjects: `:4317` · sidecar on `PATH` · the
   three `ANDROMEDA_PULSE_*` handles. Conductor does no Pulse process management by scope law, so the
   probe REPORTS — it never launches.

### Trajectory gate

All three mints are trajectory-class (new chunk ahead). None halted: route-resolve's pre-direction clause
is satisfied when a recorded operator direction names BOTH the entry and its disposition. Items 1 and 2
were imperative ("Mint M1 at the 6a head" / "Mint M2 after M1"). **Item 6 was an armed question** — the
directive enumerated mint · fold · residuals and stated a lean; the lean was applied and is flagged for
override rather than treated as settled.

### Anchor-only pinning

The standing cargo-audit PREREQ (external-decay class, **45th**) moved verbatim from *A11y CI gate* onto
*Mutation tier restored*, the new first markerless entry. Ordinal unchanged — a migrating pin numbers the
forthcoming probe, and no probe ran on *A11y CI gate* (`.claude/docs/session-learnings.md`, 2026-08-20).
*A11y CI gate* keeps its `BLOCKED-ON` and all **3** CARRYs; its trajectory question defers one chunk.
Asserted after the write: 0 frozen (`[{marker}]`-prefixed) lines changed; 112 → 118 lines.

---

## Verification of dictated coordinates

Every measured claim the directive carried was re-derived against the freshly rebuilt rust plane before
being written anywhere. Trace:
`.andromeda/runs/2026-09-02T22-20-00Z-wrap/tree-query-wrap-adaptation.json`.

| claim | dictated | measured | disposition |
|---|---|---|---|
| `persist` | 10 sites / 8 callers | **10 / 8** | exact — written as given |
| `read_envelope` | 5 sites / 3 callers | **5 / 3** | exact — written as given |
| `RunsDb::get_envelope` | 6 sites / **3** callers | **6 / 4** | **corrected** — the measured 4 was written |
| old anchor `'%/persist().%'` | returns 0 | **0 rows**; raw symbol `…conductor-run 0.1.0 persist().` | mechanism confirmed |
| "DB holds no row at all for `persist` / `execute_scenario`" | false | both indexed; `execute_scenario` = **7 / 5** | correction extended to cover both symbols |

`get_envelope`'s four distinct callers: `envelope_status_round_trips_per_run` (×3),
`read_envelope`, `an_over_envelope_run_is_environment_suspect_not_fail`,
`an_in_envelope_run_records_its_standing_too`.

---

## Code-graph refresh (P4)

Both planes rebuilt on the new schema, `built.views` sidecar present on each — the one-time rebuild the
directive predicted, not a defect.

```
rust ok 29s 2331/10706
ts   ok  2s  701/1562
```

`tree.db.commit` re-pointed to the new HEAD after the commit (source-free delta: the index still reflects
the same source).

---

## Open point for the operator

**Item 6's placement rides a lean, not a ruling.** *Live-Pulse preconditions probed* sits third in Epoch
6a, after M1 and M2. If it belongs elsewhere — folded into *Operator-gated live suite* in 6b, or in
`.andromeda/residuals.md` as a next-version concern — say so and it moves; nothing downstream depends on
the slot.
