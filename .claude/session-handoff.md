# Session Handoff

**Last Updated:** 2026-09-03T20:32:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **49 ahead** after this commit)
**Status:** clean
**Last Commit:** `feat(2026-09-03-live-pulse-preconditions-probed): the probe that names an absent SUT once,
and the boot arm that skips the preflight rather than paying it`

## Position
- Done: **2026-09-03-live-pulse-preconditions-probed** — `conductor preconditions [--json]` ships: a
  non-mutating, non-priming probe over `:4317` reachability, `andromeda-pulse-mcp` on the inherited `PATH`
  (a directory walk, **never** a spawn) and the three `ANDROMEDA_PULSE_*` handles read presence-only. `boot`
  gains it as a leading arm that short-circuits and **skips** the preflight — proven by the ABSENCE of any
  `ReadyState` JSON, in both shells. The run contract gained one `shell-declaration` term (`mcp-enabled`),
  taking `[[term]]` 5 → 6 and the observable env set 1 → 2.
- Next: **`/andromeda-phase`** to promote + plan **_A11y CI gate_** — the next markerless entry. **It carries
  `BLOCKED-ON:` at annotation position, so phase's Setup will HALT on it: taking it up anyway, or skipping to
  the next non-blocked entry, is your call.** It also now carries the **standing cargo-audit PREREQ, the 49th**.
- Coverage **26/32 verified · 6 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) — unchanged;
  this chunk claimed nothing, so the coverage gate was a no-op.

## Work done
Three new files (`conductor-core/src/preconditions.rs` 383 · `conductor-cli/src/commands/preconditions.rs` 38 ·
`conductor-run/tests/run_contract_pin.rs` 104), 13 modified. **No dependency delta** — package count 564
unchanged, `Cargo.lock` byte-unchanged; the `which` crate was explicitly rejected so none would land under the
standing red audit. The evaluator is PURE (no env read, no IO, no socket), so both arms of all three subjects
are reachable with no `unsafe { set_var }` — the shape `testing.md:67` mandates for any env-sourced gate.

Gates: nextest **691/691** · `cargo test` green under both runners · clippy clean · `agent-run.sh run`
**824/824** · smoke MINT-THEN-READ (fresh `run_id` minted, `status` read it back at exit 0, `Blocked` spine).

## Drift resolved
**36 proposals · 35 applied · 1 dismissed · 6 escalations resolved · 2 playbook rules minted (37 → 39).**
Per doc — arch 7 (+1 dismissed under `:61`, the CLI-verb over-reach rule: the verb's home is layout-templates),
security-plan 6, design-system 5, layout-templates 7, test-plan 9, a11y-plan 1, **obs-plan 0 (clear, with
reasoned negatives on all four detectors)**.
- **Escalated and ratified:** the 6 security-plan proposals. `:58`'s dismiss precondition FAILED — this chunk
  does add external-input surfaces (a `PATH` directory walk, two handles with no §Input Validation row) — so
  `:124` (boundary widening, never-routine) governed. Applied as records on the basis that the probe adds no
  write, no spawn and no new subprocess crossing. Per `:124` no rule was minted; the class keeps escalating.
- **Two claims disproved, both disposed:** `anstream` is named as the cli TTY gate in design-system and
  layout-templates but is absent from both manifests and all of `conductor-cli/src` (the shipped gate is
  `owo-colors` + `std::io::IsTerminal`, two independent per-stream decisions) — retired at 5 sites; and
  `sidecar-built`'s `asserted` rationale is measurably false, recorded beside the `warmup_ms` case and
  deliberately NOT re-classified.
- **Cascade caught late:** my first sweep keyed on the connectives a claim uses (`is a` / `=`) and MISSED
  `verification-harness.md:18`, which states the retired "boot is a preflight gate" mechanism with an em-dash.
  Re-swept on the claim's SHAPE; 6 leaves re-derived in total.

## Notes
- **Curation:** Tier 2 × 2 — a NEW `testing.md` entry (34 → 35) on **word-anchoring the drive-letter token**
  in a host-path negative test (`\b[A-Za-z]:[\\/]`, never bare — `p:/` inside `http://127.0.0.1:4317` matches
  the unanchored form), and an **in-place extension** of `security.md`'s standing-deferral entry recording that
  the deferral's reproduction form must be listed in the plan's Test Commands or the wrap cannot re-pin it.
  CLAUDE.md untouched at **133/200**.
- **Deferred learnings** (both landed at EXACTLY 0.6, which rejects under Filter 4's lean default):
  - A shared `CARGO_TARGET_DIR` across two git worktrees lets a binary built from the *other* tree satisfy a
    run — the exit code reads green and only the TEST COUNT (263 vs 277) distinguishes it. No durable home;
    re-propose if it recurs.
  - An absolute `CONDUCTOR_RUNS_DIR` fails at `Paths::resolve()` *before* the verb dispatches, so every verb
    prints empty stdout. **Not lossy** — the same fact landed this wrap as a test-plan amendment at two sites
    (§3 Per-test isolation, §2 fixtures), so the master owns it.
- **Two findings routed, not fixed** (operator directive, both verified first-hand):
  - **Panic-hook race** → CARRY on *Release build and bundle* (v2-27). `obs.rs:491-496` and `:532-537` both
    take/restore the process-global hook; under `cargo test` they race. Seen red once, passes since — a race
    that passes is still a race, and zero-flakiness owns it. Pre-existing, UNMASKED not caused.
  - **rustfmt edition mismatch** → CARRY on *Sidecar spawn without a console window*. The hook runs bare
    `rustfmt` with no `--edition` against an edition-2024 workspace, so every pass re-wraps imports 2015-style
    (this chunk: three files' diffs are largely re-wraps) and it defeated one anchored Edit. The hook row is
    the overseer's to encode; the instance remedy is a one-file `rustfmt.toml`.
- **Observed, unrouted:** `design-system` states `indicatif 0.18` where the manifest/lock say **0.17.11**
  (and `inquire 0.7` vs **0.9.4**). Pre-existing, not this chunk's drift — the route's *Dependency polish*
  entry already owns the indicatif question.
- **Process hygiene:** census across 8 process names — **zero stragglers**; no LISTENING socket on `:4317`.
  Nothing was spawned: the probe is a `PATH` lookup by design.
- **Last failed command:** none.
