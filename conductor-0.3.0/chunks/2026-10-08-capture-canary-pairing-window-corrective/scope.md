# Scope — 2026-10-08-capture-canary-pairing-window-corrective

**Working entry (`working-route.md:100`):** Capture canary pairing-window corrective — a canary storm ticking
milliseconds before the emission instant prints what Pulse's own log bears out, never a false `pipeline-fault`.

**Matrix target:** none. The entry was minted by a 0-pending adaptation and adds no requirement; the ledger reads
11 of 11 verified and this chunk claims no capability (`inputs#I1` §4: say so at the review rather than adding one).
`v3-09` stays verified on the sixth series and is not re-read here.

**Why it exists:** the founder's pick by dialog, 2026-10-08 — "Починить в 0.3.0" — relayed verbatim by the pc
overseer (`inputs#I1` §1; the entry's own CARRY block carries the same word).

## What this chunk builds

1. **The capture's canary pairing reads a selected digest's inference across the emission instant.** Today the
   capture hands `canary_attempts` only the Pulse lines stamped strictly before the scenario's emission instant
   (`crates/conductor-run/tests/real_model_live.rs:721-730`, the filter `at < emitted` at `:726`; the pairing is
   `canary_attempts`, `crates/conductor-run/tests/real_model_common/mod.rs:73`). After the change, a retry-storm
   cue-bearing digest whose tick is stamped before the instant prints the outcome Pulse's own log bears out, even
   when its prompt assembly, parse and incident outcome are stamped after the instant.
2. **The scenario's own storm digest still never prints as a canary.** The plan states how the two are told apart
   when their stamps sit 5-6 s apart and Pulse's lines carry no digest identity (`inputs#I1` §3, first question).
3. **Tests that pin it on recorded shapes**, with no live drive: the d1 and d2 shapes of the sixth series (third
   line reads the surfaced/deduped outcome), d3 as a two-line control, and a failing-first control — the new test
   reads `pipeline-fault` on the d1 and d2 shapes before the change (`inputs#I1` §3, second question).
4. **A second dated correction in the contract**, add-only: `contracts/pulse-real-model-leg-posture.md`, under
   "The canary classification" (`:306`), beside the existing `[corrected 2026-09-29 …]` note (`:316`).

- The smallest change that does (1) and (2) keeps the SELECTION of cue-bearing ticks where it is (a tick stamped
  before the instant) and moves only what the PAIRING may read (the lines after the instant, up to the next
  retry-storm tick). The selection covers both things the block prints: the `canary:` lines and the closing count
  of other cue-bearing digests. The scenario's own retry-storm tick is stamped after the instant by construction
  (the instant is the `timeline.execute` span's open, taken before the scenario's first emission; its storm reaches
  the Autonomous band about 5 s later), so it is never selected and it bounds the last canary's segment. Verified
  at P3 by a replay of the pairing over the sixth series' recorded Pulse log (`research.md` §The replay).
- The pairing is order-based, not stamp-based, and it has to be: in d1 the third canary storm's dedupe line and
  the scenario digest's prompt assembly carry the same millisecond stamp (06:32:52.168Z). Verified at P3: in the
  recorded log's file order the incident outcome line precedes the next prompt assembly on all four same-stamp
  pairs in the three drives' windows. A fixture keeps that order and says where it was read.
- The unit-testable seam is the shared module (`real_model_common/mod.rs`): the capture is compiled only under
  the `live-pulse` feature, so the stamp comparison moves into the shared module with its stamp parser, and the
  new tests join the existing pairing tests in `real_model_grading/canary_pairing.rs`, a child of the default-suite
  `real_model_harvest` target. Verified at P3.
- A d3-shaped storm (tick stamped after the instant) keeps printing no third `canary:` line. The fix does not try
  to recover it; the contract's stated limit is extended to say so. Verified at P3 (the replay prints d3's two
  lines and its count unchanged).
- `[premise-corrected: the contract's "The canary classification" clause sits inside "## The drive series"
  (`contracts/pulse-real-model-leg-posture.md:213-321`), the 2026-09-29 series' own section, which no digest
  pins; the six later dated sections are each held by a sha256 pre-registration pin
  (`real_model_grading/mod.rs:155`, six `pre_registered` callers)]` Item 4's note therefore lands inside the first
  series' section and moves no pin. The entry places it there in so many words ("a second dated correction there,
  add-only") and the existing 2026-09-29 note is the precedent. "Byte for byte" is held as: no existing line of
  the contract is edited or removed, the six pinned sections keep their digests, and the one addition is a
  contiguous block directly after the 2026-09-29 note. Named at the P5 review for the operator to overrule.

## Folded freight (`working-route.md:100`, one block, per `route.py pins`: `CARRY:` 2091 chars)

The CARRY block is folded whole. Its named coordinates were re-read at take-up (2026-10-08):

| Coordinate the block names | Read at take-up |
|---|---|
| `crates/conductor-run/tests/real_model_live.rs`, the canary block, its `at < emitted` filter | present, `:721-730`, the filter at `:726` |
| `canary_attempts` in `crates/conductor-run/tests/real_model_common/mod.rs` | present, `:73-151` |
| the sixth series' `evidence/attempt-ledger.md`, its section on the third `canary:` line | present, `:251-276` |
| the three `rm-capture-d*.txt` beside it | present; d1 prints three `canary:` lines (`:531-533`), d2 three (`:556-558`), d3 two (`:573-574`) |
| `contracts/pulse-real-model-leg-posture.md`, the `[corrected 2026-09-29 …]` note under "The canary classification", `:316` | present at `:316`; the clause opens at `:306` |

What the founder's pick bounds, as the block states it: the harness change and its tests; no GPU and no new live
series; committed captures keep what they printed; the decision rule and every pre-registered section stay byte
for byte, and the rule still counts emissions and never reads these tokens. The contract correction is a second
dated note, add-only.

## Causal claims carried in (closed at P3, marker kept)

- measured at the sixth series' d1 and d2: "ticks 4 ms and 2 ms before the instant, prompt assemblies 1 ms and
  2 ms after it; Pulse's own log shows each storm parsed `ok` and deduped, with no inference error and no
  inference skip; d3's tick fell 11 ms after the instant and its capture prints two `canary:` lines and no third".
  Verified at P3 against the recorded log (d1 tick -4 ms, prompt +1 ms; d2 tick -2 ms, prompt +2 ms; d3 tick
  +11 ms). The mechanism is reproduced: the pre-fix composition, replayed over the recorded log, prints the same
  `canary` lines as each of the three committed captures, line for line.
- hypothesis: "the fix can be proven on recorded log lines, a fixture built from the stamps the ledger holds,
  without a live drive — the phase decides; if a live drive is the only honest proof it says so and stops for the
  overseer's word, since a GPU run needs the founder's new word". Verified at P3: the recorded lines are enough to
  reproduce the defect and to show the corrected reading, so no live drive is planned. What stays unmeasured is
  named in `research.md` §What the proof does not reach.
- the ledger's own explanation of the race (`:265-276`): the order of the two stamps is a race of a few
  milliseconds between Conductor's last canary storm and its emission stamp. Verified at P3 as far as the record
  goes (d3's third storm reads Suggested 1 ms before the instant, Autonomous 4 ms after it, its tick 11 ms after
  it). This chunk does not change that race; it changes what the capture reads on either side of it.

## From the relay (`inputs#I1`) and the invocation (`inputs#I2`)

- **Small** (`inputs#I1` §2): the canary pairing in the real-model capture and the tests that pin it. No scenario,
  script, manifest or shipped-crate change. If research finds the honest fix is wider, stop at the plan review and
  say so. Verified at P3: the write set is three files under `crates/conductor-run/tests/` and the contract.
- **Nothing recorded is re-rendered** (`inputs#I1` §2): the frozen chunks stay byte for byte, and the harvest's
  pins on the printed tokens stay as they are. Verified at P3: no pin, capture or measured table is in the write
  set.
- **The plan answers three questions explicitly** (`inputs#I1` §3, `inputs#I2`); research answers each in
  `research.md` §The three questions:
  1. what the classifier prints, after the fix, for a canary storm whose tick precedes the instant by
     milliseconds, and how the scenario's own storm digest is still told apart (d1: canary prompt 06:32:46.356Z,
     scenario prompt 06:32:52.168Z);
  2. the failing-first control on the recorded d1 and d2 shapes;
  3. that architecture §Occupied Resources and §Established Decisions are not grown; an amendment that grows
     either frees bytes there first. Measured at take-up with `scripts/arch-registry-check.py measure --file
     .andromeda/architecture.md`: §Occupied Resources 38114 B, §Established Decisions 38082 B, threshold 38115 B.
- **Order and pace** (`inputs#I1` §4): the version close follows; unhurried. Read at P3 against
  `working-route.md:102`: the version close is the one markerless entry left.

## Boundaries (out of scope)

- No GPU, no live drive, no new series. No `pulse-app` launch.
- No committed capture, frozen chunk or evidence tree is edited. The harvest's digest pins and its `measured`
  tables keep the tokens as printed.
- The decision rule (the span between the rule's two marker lines) and every pre-registered series section of the
  contract stay byte for byte.
- No scenario, script, manifest or shipped-crate change; no dependency change.
- `v3-09` is not re-read, re-graded or re-concretized.
- The race itself (when Conductor's last canary storm is emitted relative to its emission stamp) is not moved.

## Review

A fix wider than the pairing and its tests stops at the P5 review and is named there. A fork between designs that
both stay inside the bounds is asked at P4.

## CI (Setup 5a)

Base: the last master flip `d54751f`; shas read newest first, one `ci.py conclusion` call.

| sha | Verdict as read | Wall-clock |
|---|---|---|
| `39e197b` | `CI 39e197b: verdict not yet available` (in progress, checks 3/3, run `CI#37750526236`) — landed as read, unresolved, never as a green | not yet available |
| `d54751f` | green, checks 3/3, run `CI#37745625899` | 573 s |

No red and no `not green` row, so nothing is dispositioned here.
