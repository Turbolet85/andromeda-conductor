# Scope — `2026-09-18-real-model-leg-posture-and-grading-rule`

**Epoch 4 — Live proof against a real Pulse** · head entry, `conductor-0.3.0/working-route.md:42`

**Working entry (verbatim intent):** Real-model leg posture and grading rule — deterministic mode off,
operator-gated, never a CI gate, grading and per-leg quiet window fixed beforehand

---

## What this chunk is

The **posture half** of Epoch 4's live-proof pair. It fixes — *in advance of any drive* — the launch posture,
the grading rule and the per-leg quiet window under which a real-model (non-deterministic L4) live leg will be
run and judged. The drive itself belongs to the next route entry (`Interpretation proven live`).

The separation is by design, not sequencing convenience. `verification-matrix.json#v3-09` `notes` (route Phase
0/1, 2026-09-11) states it: *"The grading rule is stated in ADVANCE by design: a chunk that drives first and
decides the grading afterwards satisfies the letter and defeats the capability, which is why posture and drive
are separate route entries."* A posture authored after seeing a leg's output is not a posture; it is a
rationalization. This chunk therefore ships a **committed, dated statement** that the drive chunk is bound by.

## What it builds

1. **The launch posture for a real-model leg** — the complete, stated set of SUT-side and harness-side
   conditions under which the leg runs, and the ones that must *not* be set. Anchored on what the registry
   already records: `ANDROMEDA_PULSE_L4_DETERMINISTIC` OFF is the defining term (architecture.md:197 — the flag
   swaps a canned `L4Output` for the Llama-3.2-3B inference; Conductor **reads** it as a run-contract
   `shell-declaration` proxy and never sets it), alongside the already-in-use boot-wide
   `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` lever whose disposition for *this* leg must be stated rather
   than inherited.
2. **The grading rule** — how a non-deterministic top-hypothesis assertion is graded, written so that it
   decides a verdict without reference to the run that produced it. It must state at minimum: what counts as
   "the top-ranked hypothesis identifies the injected root cause", which `Verdict`/`ReportState` a miss maps
   to, and — explicitly, per v3-09's acceptance — that **a leg that fails grades as a failure rather than
   being re-driven until it passes**.
3. **The per-leg quiet window and serialization rule** — the interval each real-model leg requires before and
   after it, with its arithmetic stated. v3-09 `notes`: *"live legs are serialized rather than merely ordered,
   because the system under test dedupes a new incident against any open one and every run fires its own
   preflight canary, so each leg rides its own quiet window."*
4. **The operator gate** — the leg is operator-invoked and **never a CI gate**, stated as a property of the
   posture rather than left to the invocation path's current shape.
5. **PREREQ discharge** — close the standing rust gate deferral (below).

## Folded annotations (from the working entry, re-verified at promotion)

**`PREREQ: close rust gate deferral`** — folded whole. `cargo clippy --workspace --all-targets -- -D warnings`
has deferred **twice consecutively** on zero `.rs` delta; the **third consecutive re-pin trips the age
trigger**, so this chunk runs it rather than re-pinning it. Every named coordinate re-verified against the
artifact at promotion (the fold's own duty — annotations arrive as hypotheses):

| Claim in the annotation | Verified against | Result |
|---|---|---|
| deferred since `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration` | that chunk's `report.md:290` | ✓ "**defer** (zero `.rs` delta)" |
| a second consecutive deferral | `2026-09-17-keyboard-and-focus-order-coverage-ownership/report.md:75` | ✓ "**`defer`** … zero `.rs` and zero `Cargo.*` delta" |
| last ran green at `2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm` | that chunk's `report.md:87` | ✓ "ran · green · exit 0" |
| the pin was restored, not inherited | handoff + the two intervening reports | ✓ chain is **restored**; this is re-pin ordinal 2 |

**`[inferred]` — the deferral's subject measured green in CI on the tree this chunk starts from.** Setup 5a read
`e75fcb9`'s check-runs: *Rust gate (build · test · lint · supply-chain · coverage)* completed **success**
(run 35255156862). That is evidence the lint arm passes on this tree; it is **not** a discharge of the pin,
which asks for the local run. Recorded so the local run is expected cheap, not so it can be skipped.

## Boundaries — what this chunk does NOT do

- **It does not drive a leg.** No live run against a real model, no incident asserted, no hypothesis graded.
  The next entry (`Interpretation proven live`) owns the drive.
- **It does not claim `v3-09`.** v3-09's acceptance quantifies over the drive *and* the posture ("A live leg
  runs … and the leg asserts …"), so this chunk only **partially advances** it: per the matrix contract's
  multi-chunk rule the cap stays `chunk: null` and is cited in this chunk's `plan.md` provenance. Confirmed
  at P5.
- **It does not touch `v3-08`.** That cap is BLOCKED on a Pulse release emitting the P-025 observable —
  externally clearing, not Conductor's to clear, and a different route entry's subject.
- **It does not re-open the deterministic-L4 legs.** The existing `--live` suite (H · B1 · B2 · quiet · A ·
  driven-a11y) keeps its current posture and its verified results; this chunk adds a real-model posture beside
  it, and must not weaken or re-grade what deterministic legs already proved.
- **It opens no inbound listener and adds no network reach** (scope law, unchanged).

## Surfaces and contracts it touches

- `scripts/agent-run.{sh,ps1}` — the operator-gated `--live` suite and its `preconditions` short-circuit; any
  real-model arm lives here or beside it. `[inferred]` — the entry names neither file.
- `contracts/` — the natural home for a committed, code-unread posture/measurement statement, by the precedent
  of `pulse-p025-measurement-contract.md`. `[inferred]` — form to be settled in P3/P4.
- `.andromeda/residuals.md:9` — the `2026-08-09-interpretation-correctness-posture · absorbed:v3-09` pin, which
  states this exact subject and must stay consistent with what ships.
- `contracts/pulse-run-contract.toml` — `warmup_ms` / `min_canary_poll_seconds`, the terms the harness shells
  already parse for the `boot` budget and the preflight floor a real-model leg's timing must respect.
- The `UNBACKED_AUTO` set in `conductor-core` (P-031 / P-033 / P-034 / P-044) — the diagnostic-quality cluster
  this posture ultimately serves. Read-only here; the cluster is a later entry's subject.

## Premises — CLOSED at P3 (research.md holds the derivations)

- **VERIFIED — a real-model leg cannot be the existing suite with the flag flipped.** `agent-run.sh:118-121`
  and `verification-harness.md`'s 2026-09-06 entry (c) both state it: canned-L4 formation ~2s lands leg B2's
  storm inside Pulse's 120s idle window, **real-model formation measured ~110s would fall outside it** — "an
  env missing that flag does not make the leg slow, it makes it measure the wrong thing". Every per-leg budget
  and the 150s quiet window are calibrated to the canned path. The ~110s figure is a 2026-09-06 SUT-timing
  measurement this chunk cannot re-measure without driving; it is carried with its date and witness, and the
  drive chunk confirms it.
- **`[premise-corrected: `Thresholds::from_env` resolves the window ONCE at Pulse's boot, so no per-leg value
  exists to choose]` — the bootstrap-window lever's disposition is SUITE-wide, not leg-specific.**
  `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is registered boot-wide and is the one handle Conductor neither
  sets nor reads. The posture must therefore state a single boot-time disposition binding the whole real-model
  suite, and record which posture actually booted by grepping Pulse's log for the target
  `triage.baseline.bootstrap_window.override` — never the emitting function name. The prior wording would have
  asked for a choice the SUT cannot express.
- **VERIFIED and sharpened — the grading rule needs a non-determinism model, and the observable is narrower
  than assumed.** The assertable surface is `retrieve_report`'s markdown, which Conductor's
  `extract.rs:102` pushes into `Observation.text`, where `ComparisonKind::Contains` grades it — so a
  hypothesis assertion needs no new check kind. Position in the `hypotheses` array is the rank.
- **ADDED at P3 (scope did not state it) — the hypothesis observable does not exist for a single-storm leg.**
  `retrieve_report` renders `## Hypotheses` only when `incident.resolution_summary_text` parses as `L4Output`
  (`degraded_mode = parsed_l4.is_none()`), and neither writer fires on first incident creation: the live
  writer fires only on a **dedupe re-generation** of an already-active incident matching `(kind, scope,
  scope_id)`, the other only after resolution. So the posture must fix an emission profile that produces a
  SECOND cue-bearing digest on one identity — otherwise the leg reads back a degraded report with nothing to
  grade. This is the single most consequential thing the posture states.
- **ADDED at P3 — a real-model leg cannot be graded on an SLO tier.** `SloTier` is closed at `<5s`/`<20s`/`<90s`
  with `deadline_ms()` capped at 90 000, while formation alone measured ~110s and `latency_ms` is
  journal-relative over the whole emission window. The posture must name a non-tier disposition; the
  declare-only row plus harvest-tier hard assertion is the only shape the system already has for it.

## Why it is here (provenance)

- Route entry `conductor-0.3.0/working-route.md:42`, Epoch 4 head.
- `verification-matrix.json#v3-09` (*Real-model interpretation leg*) — partially advanced, not claimed.
- `.andromeda/residuals.md:9` — the absorbed `2026-08-09-interpretation-correctness-posture` residual.
- `.andromeda/architecture.md:70` — the caveat recording that under deterministic L4 "Conductor green" means
  the pipeline carried a canned answer end to end, and that proving the real thing needs a non-deterministic
  live leg that can never be a CI gate.
