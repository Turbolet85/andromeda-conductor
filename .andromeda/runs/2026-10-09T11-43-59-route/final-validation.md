# Phase 5 — final validation, conductor-0.4.0

_Run over `working-route-draft.md` as approved at the third Phase 4 (61 entries, 8 epochs). The operator's
approval, by relay on 2026-10-09: «APPROVED as it stands — 61 entries, 24 requirements v4-01..v4-24». No entry was
edited between that approval and this check._

| Check | Result | Reading |
|---|---|---|
| [ANTI_LEAKAGE] | PASS | no entry matches the letter's pattern, and none holds "step-by-step" or "as follows:" |
| [PER_CHUNK_LENGTH] | PASS | 61 entries, each 25 words or fewer |
| [SIZING_DISTRIBUTION] | PASS | entries per epoch 7 · 5 · 9 · 7 · 8 · 8 · 9 · 8; median 8, largest 9, smallest 5 |
| [GATE_REACHABILITY] | PASS for the engine-backed gate; NOT MET AS WRITTEN for the two-host path — carried on the operator's approval, reason below | see below |

## [GATE_REACHABILITY]

- **The engine-backed gate.** Its reachability entry is in Foundation: `Engine-backed check pipe reachable` — one
  dev-host run to a recorded green verdict. Every later entry that asserts against a live engine on one machine
  (Epochs 3, 4, 7) lands on that pipe. Met.
- **The two-host path.** Its reachability entry, `Two-host path reachable`, exists but sits at the head of Epoch 6's
  path work, not in Foundation. The letter (`phase-1/validation.md` [GATE_REACHABILITY], re-checked here) asks for a
  Foundation entry. It cannot be one: the engine's network receiver and the door's reach from another host open in
  Pulse's Epoch 4 (`andromeda-pulse-0.4.0/working-route.md` at `b3ac58a`), so before that point no second-host run
  can reach a verdict at all. The letter's own arm for this check — "Fail → halt with the offending lines (no insert
  after approval)" — was not taken, for this reason: the deviation was reported to the operator at each of the three
  Phase 4 reviews (report point 4 every time) and the route was approved as it stands with it in view. The offending
  lines, for the record: `Two-host path reachable` (Epoch 6) and the entries that assert on that path after it —
  `Channel refusals`, `Fixed local address retired`, `Host clock difference measured`, `Bad-version rollout end to
  end`.

## Review rounds behind this draft

Three Phase 4 reviews, each answered by a relay file of the operator's, outside this repository:
`conductor-route-phase4-rerun-2026-10-09.md` (the intent edited; run again from Phase A),
`conductor-route-phase4-rerun-second-2026-10-09.md` (F7b edited; run again from Phase A), and the third answer,
given in the session: approved as it stands. The first and second passes are kept whole under `first/` and
`second/`.
