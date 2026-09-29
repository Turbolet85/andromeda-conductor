# Scope — Diagnostic-quality cluster off the drift pin

**Marker:** `2026-09-29-diagnostic-quality-cluster-off-the-drift-pin` · conductor-0.3.0 · Epoch 4 — Live proof against a real Pulse
**Working entry:** `working-route.md:50`, the first markerless entry. It carries a BLOCKED-ON, and Setup's halt was
answered **take it up anyway** by the founder's ruling of 2026-09-29, quoted verbatim in the operator directive
`D:/dev/projects/additional/pc-overseer/relays/conductor-phase-50-2026-09-29.md` item 1 ("nothing is skipped or
deferred, the route runs in order, a problem met is solved now"). So a deferral of `v3-09` is NOT an outcome this chunk
may end on (directive item 1).

## Intent (the working entry, verbatim)
> Diagnostic-quality cluster off the drift pin — P-031, P-033, P-034 and P-044 on an exercised path, gate pin and
> committed matrix moving together

## What this chunk builds, in order
The order is the directive's (items 2 and 3): the blocker first, then the entry's own scope.

### Problem 1 — the real-model preflight canary (the BLOCKED-ON; directive item 2)
The only graded real-model drive stopped at the preflight canary. Research **measures the cause** before any fix is
chosen, using a stated, bounded number of canary drives, each carrying the evidence needed to tell the candidate causes
apart (directive item 7: the plan states how many drives it runs and why, never an open-ended loop). The fix lands
where the cause is:
- **Conductor-side** (the canary's stimulus shape, its digest, the poll budget): fixed in this chunk.
- **Pulse-side** (its incident prompt, thresholds or creation predicate): **stop and report**. A Pulse change goes
  through Pulse's own route and the overseer carries it there; Conductor never edits Pulse (directive item 2).
- The block **clears when one real-model drive gets past the canary** (the entry's BLOCKED-ON, verbatim).

### Problem 2 — the entry's own scope (directive item 3)
- The four diagnostic-quality capabilities run on the real-model leg, which is the exercised path:
  - `P-031` Report Structure
  - `P-033` Ranked Hypothesis Generation
  - `P-034` Suggested Investigation Steps
  - `P-044` Retrieval-Augmented Interpretation (Memory & Learning)

  (Titles and classes from `crates/conductor-core/src/coverage.rs:275/287/293/353`, all `CoverageMode::Auto`.)
- **The gate pin:** the four ids leave `UNBACKED_AUTO` (`crates/conductor-core/src/drift.rs:61-63`, 8 ids today →
  4). `check_scenario_backing` holds that set to exact equality against the catalog, so a scenario must name each id.
- **The committed matrix:** `coverage-matrix.md:3` reads `43 auto (8 unbacked)`. It moves in the SAME change as the
  pin, because the artifact is byte-compared to its render (`v3-10` notes: "dropping four ids therefore reds both arms
  unless the pin and the artifact move in the same change").
- **Matrix capabilities in play:**
  - `v3-10` (*Diagnostic-quality cluster backed by an exercised path*): unclaimed, `planned`.
  - `v3-09` (*Real-model interpretation leg*): `deferred` at the 2026-09-23 wrap, with the entry named as the owner of
    any re-attempt.

## Boundaries
- In: the real-model canary's measurement and its Conductor-side fix; scenario(s) naming the four ids on the real-model
  path; `UNBACKED_AUTO` + `coverage-matrix.md` (+ whatever render/test asserts them) moving together; the real-model
  harvest grading those ids; the matrix entries `v3-09` / `v3-10`.
- Out: any edit to Pulse. Pulse is read at its committed HEAD `f15536b` only, through
  `git -C D:/dev/projects/andromeda-pulse show f15536b:{path}`, never its worktree. The worktree holds uncommitted
  P-025 work (measured 2026-09-29: 20+ modified tracked files) (directive item 4).
- Out: `:52` (Hue-shift budget, BLOCKED-ON a Pulse release) and the remaining route entries.
- Out: a CI step for any real-model leg. It is operator-gated and never a CI gate (`v3-09` requirement; `drift.rs`
  doc comment: "a non-deterministic live leg that can never be a CI gate").

## Folded freight (from `working-route.md:50`; `route.py pins` lists 2 blocks on this line)
- **CONTEXT (from 2026-09-22-interpretation-proven-live)** (684 chars), coordinates re-verified 2026-09-29 at HEAD
  `a76420a`:
  - `v3-09` was DEFERRED, not verified. Its one graded drive was run `2026-09-23T07-39-39-845`
    (`conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt`, present, 12 355 B).
  - The CONTEXT states this as measured: "the real model answered the canary's one cue-bearing digest and formed no
    incident, so the scenario never emitted — `NoAttributableIncident`, model-side, measured from Pulse's own log".
    Verified at `f15536b`: `emit_incident_outcome` logs `interpretation.incident.created` on both create AND dedupe
    (`inference_runtime.rs:912-933`), so zero such lines after a parse `ok` means an early return, and every early
    return reads a model field. Which field (Dismiss, severity None, or the resolution-summary flag) stays
    unobservable.
  - "Under the real-model posture the preflight canary is itself a real-model gate", per that chunk's research P-H
    (`research.md:143`). Verified at `f15536b`: the same predicate (`:755-760`) applies, and `preflight_for` runs the
    canary under either posture. One correction to P-H's wording: a `watch` decision with severity ≠ `none` also
    creates an incident (`Decision::Watch`, `schema.rs@f15536b`).
- **BLOCKED-ON** (619 chars): "the real-model preflight canary — clears when one real-model drive gets past the
  canary".
  - The four ids are provable only on the real-model leg. **Re-verified:** `drift.rs` `UNBACKED_AUTO` doc comment
    `:48-56` (the entry's `:48-57` holds), reads "proving them means injecting a known root cause with
    deterministic mode OFF".
  - The entry's own CARRY blocks moved to the capture-hardening entry (operator relay item 1a), so none remain here.

## Premises (closed at P3 against research — `research.md` §Scope premise closure)
- **The canary's cause is one of three open candidates, per directive item 2, marked there as unmeasured
  (verbatim):** "model nondeterminism, a digest too weak to warrant an incident, or Pulse's incident prompt /
  thresholds". Verified at Pulse `f15536b`: all three stay open, and they separate only by outcome frequencies
  under controlled variation.
  - Every incident-suppressing exit reads a model-authored field (`inference_runtime.rs:755-760`).
  - Sampling is stochastic by construction (no `--seed` / `--temp` in `build_llama_cli_args`).
  - The model sees `conductor-canary`'s RED row, the cue line and any `CORPUS MATCHES`, never the exception type or
    message (`assembler.rs:600-690`).
  - A canary gets ONE model decision per drive (one cue-bearing digest).
- **No surface shows WHY.** Verified: at `f15536b` Pulse logs no decision or severity on the silent exits, logs no
  raw runner output, and persists no dismissed output. A direct discriminator needs a Pulse-side observability
  change, which is a Pulse route item carried by the overseer (directive item 2).
- **The acceptance conflict.** Verified: the re-drive ban stands in four places:
  - the `v3-09` acceptance ("recorded and never re-driven");
  - `contracts/pulse-real-model-leg-posture.md:176-178`;
  - `scripts/agent-run.sh:160` (prose, unenforced);
  - test-plan §9 ("fired once and never re-driven").

  The founder's ruling forbids deferring `v3-09`, and the canary measurement needs a series. The acceptance changes
  only through its own lifecycle (P5 concretization preview, never weakened), so this is a P4 fork for the operator.
- [premise-corrected: P-031 and P-034 are gradable from the real-model capture once it prints them — the six P-031
  sections always render, and Investigation Steps renders as a numbered list (`markdown.rs@f15536b`) — but P-044's
  digest-side retrieval is NOT directly observable: Pulse logs only the candidate count before selection
  (`assembler.rs:309-318`), and the selected matches reach only the unlogged prompt payload. P-044's one external
  witness is an inference chain through the report-side `## Previously Seen` (P-036's sibling selection); its
  grading basis is a P4 fork.]
- **Which scenario names the ids.** Verified: `check_scenario_backing` reads catalog `p_ids` only (`drift.rs:368`),
  so any scenario backs an id. The honest backing is `real-model-interpretation`, the one path exercising
  interpretation. Its `p_ids` pin (`scenario.rs:1528`) moves with the TOML.

## CI verdicts read at Setup (last wrap flip `67e8cb1` through HEAD `a76420a`; `ci.py conclusion`, one call)
- `a76420a` — **green** (CI#36529176256, 3/3 checks, wall 576 s).
- `17379d6` — **not green** (CI#36529121865, wall 94 s): all three jobs `cancelled`: "A11y gate (routine arm · axe ·
  contrast · violation JSON)", "Frontend gate (npm audit · build)", "Rust gate (build · test · lint · supply-chain ·
  coverage)".
  - The Rust gate intersects this chunk (the drift pin and coverage gates run there), so the verdict folds in.
    Verified against the run: CI#36529121865 was cancelled at 06:04:55Z by the `ci-${{ github.ref }}`
    `cancel-in-progress` group once CI#36529176256 (`a76420a`, 06:03:59Z) started. That run concluded `success`, and
    the diff between the two shas is two run-dir JSON trails, so no failing subject exists.
- `0f9b17b`, `80e9e96` — untested (pushed under a later tip).
- `67e8cb1` — **green** (CI#36015554627, 3/3 checks, wall 712 s).

## Operator directives for this chunk
- **Live drives need the overseer's slot** (directive item 5). Pulse's P-025 implement runs its own live leg on this
  host (OTLP `127.0.0.1:4317`). A Conductor drive that starts a real Pulse must not overlap it. Research and planning
  run freely. Every live / real-model entry in the plan carries an operator step before it runs: "the overseer
  confirms the host is free".
- **Pulse coordinates:** read at committed HEAD `f15536b` only, re-verified at phase time (2026-09-29: HEAD =
  `f15536b909af814e35eba43a50988cb863223d16`). Pulse moves when its P-025 chunk wraps (directive item 4).
- **Host:** memory is tight and Pulse is building, so cargo runs with `CARGO_INCREMENTAL=0` and `CARGO_BUILD_JOBS=2`.
  `target/` is cold after the 2026-09-29 `cargo clean`, so the first cargo gate is a full cold build (directive item 6).
- **Cost:** real-model drives spend LLM calls. The plan states how many canary drives it runs and why (directive
  item 7).
