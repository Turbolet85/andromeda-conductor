# Session Handoff

**Last Updated:** 2026-09-09T14:22:25Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **0 ahead at wrap start** — the
operator's push of the prior chunk had landed and its CI run completed. This wrap's chunk commit makes it
**1 ahead and unpushed**, and the push is **load-bearing again for the same structural reason**: the fmt
gate this chunk ships has never run in CI, and its first run necessarily follows the push.)
**Status:** clean
**Last Commit:** `feat(2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate)` (this wrap)

## Position
- Done: **`2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`** — flipped `complete` at this wrap.
- Next: **`/andromeda-phase`** on the first markerless head — **_Port-occupier test hygiene_**
  (`working-route.md:131`), minted at this wrap on the operator's directive and placed ahead of *Release
  build and bundle* so the version cannot close over a listed gate that cannot pass. No `BLOCKED-ON`, so
  phase will not halt. The sibling behind it is *Release build and bundle* (`:133`).
- Coverage **28/32 verified · 1 deferred · 3 unclaimed** (`v2-04`, `v2-21`, `v2-27`). `v2-24` moved
  `planned → deferred` at this wrap (below). This chunk claimed nothing, deliberately — the four pooled
  ids are other work, not this chunk's subject.
- **Evolve:** Epoch 6b at **13 chunks** (12 + this wrap's insertion). Surfaced again per the growth valve;
  the operator ruled **no split** last wrap. Re-surfaces next wrap.

## `v2-24` — DEFERRED here, its input finally read
The runtime-major hypothesis is **FALSIFIED**. CI run `34280136892` installed Evergreen **152.0.4191.66**
(Authenticode `Valid`, `O=Microsoft Corporation`, re-read seven minutes later by the diagnostics step) and
probe (a) still reported `DevToolsActivePort first seen: never within 90s` with the app alive
(`HasExited=False`) and three `msedgewebview2` children resident. Deferred via `matrix.py defer` with a
note carrying BOTH halves:

- **Measured:** the endpoint opens under neither runtime major. The routine arm's own failure has moved
  UPSTREAM — tauri-driver never listens on `:4444` (ECONNREFUSED within ~1.5 s), the same shape as runs
  `34162118841` / `34251573399` / `34256490781`, so it predates both the 152 upgrade and any driver skew;
  probe (b)'s `msedgedriver` log shows its own bind taking 6.9 s, which reads as a startup race.
- **Unmeasured candidates:** a hosted-image policy on remote debugging (an Edge/WebView2 policy registry
  read is the one-line probe) · a session or service-account property · msedgedriver 151 against a 152
  runtime as a cause of the DRIVER failing to start — an axis the prior chunk never tested. Self-hosted
  runners on a public repo are not an option.
- A **cross-version residual** carries what the next version's route intake should re-open.

## Work done
One chunk, two halves. **60 `.rs` files** made `cargo fmt`-clean (rustfmt output only, never hand-edited)
and **one CI step** added — `Formatting gate (cargo fmt)` running `cargo fmt --all --check` in job `rust`
at index 2, before the cache restore, no `continue-on-error`, no `if:`. Plus one chunk-evidence verifier.

The safety property is measured, not argued: **per-file token-multiset identity across all 60 files**
(`token-multiset differences: 0`), with the verifier's known-positive control firing in both directions.
`--all` over the bare form because `Cargo.toml` declares no `default-members`.

**Line counts carry BOTH bases deliberately:** +1581/−486 from parsing `cargo fmt --check`, +1582/−487 from
`git diff --numstat`. Different hunk accounting; not collapsed to one number.

## Gates
Green: advisory-db porcelain (0 lines) · `cargo audit` 0 · `cargo deny check advisories bans licenses
sources` 0 · `cargo fmt --all --check` 0 (**red → green**, baseline was exit 1 / 282 sites) · token
invariant 0 differences/60 files · `cargo clippy --workspace --all-targets -D warnings` 0 · `cargo nextest
run --workspace --profile ci` **902/902 across 55 binaries** · `cargo llvm-cov` collect 0 · coverage floor
**94.07%** vs 60 · `ci.yml` structural probe (`jobs 3 rust_fmt_steps 1 coe 3 a11y_steps 12 a11y_sig
1a3138d5`) · `agent-run.sh status` 0 · **P3 e2e smoke** 12 passing / 2 skipped, driven session attached.

**RED and NOT this chunk's — noted deferral with a named owner:** the per-crate `cargo test -p` runner-
portability gate. `conductor-faults` fails `the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines`.
Basis is three measurements, not the word "pre-existing": parallel exit 101 (6/7) · `--test-threads=1`
exit 0 (7/7) · the same failure at HEAD in a clean worktree with its own `CARGO_TARGET_DIR`. No green ever
existed for this form — the plain `cargo test -p conductor-faults` is named by 0 prior plans/reports; the
suite always ran under nextest (13 mentions, 7/7) and the only prior plain-form mentions are the `--doc`
arm (5), recorded as 0 doctests. **Owner: the route entry minted at `:131`.**

## Drift resolved
**16 amendments across 5 masters · 1 escalation resolved · 0 open.** 10 detector proposals (arch 6,
test-plan 4; five docs clean) plus **6 orchestrator raises** the per-doc detectors could not see.

- **Class A ×11** — the gate-set enumeration gains the fmt gate: `arch:37/:59/:235/:238/:268`,
  `security-plan:87`, `obs-plan:42`, `obs-plan:526` (the plan's Expected amendment, proposed by no
  detector), `a11y-plan:115/:280/:471`. Playbook rules @28 + @88 → routine.
- **Class B ×1** — `test-plan:469`'s "two jobs, both `windows-latest`" → the measured three-job set, named
  by job rather than by line coordinate.
- **Class C ×4** — the runtime-152 falsification: `arch:59`, `test-plan:469` (second claim on the same
  line), `:56`, `:307`. **No playbook rule matched → escalated → operator-approved**, and a rule was minted
  carrying three sharpenings (directness · explicit new epistemic status · discriminator shape) plus a note
  that it was minted on a GENERATOR, not a frequency.
- **`test-plan:307` was reshaped, not extended** (operator correction): a 152 × 151 combination both
  PASSES on the dev host (152.0.4191.53 × 151.0.4129.101) and FAILS on the hosted image (152.0.4191.66 ×
  151.0.4129.101) — same majors, opposite outcomes — so the **pair is not the discriminator**; the runtime
  patch (53 vs 66) is named unmeasured-as-a-cause and the open variable is the hosted image itself.
- **Cascade:** 1 leaf (`.claude/docs/stack.md:42`). CLAUDE.md's GENERATED tier and all five specialist
  summaries verified clear by grep. `a11y-plan:471` — a THIRD verbatim citation of arch §Stack — was caught
  only because the post-amendment sweep ran a known-positive control (3 hits where 2 had been edited).
- **Recorded, not amended:** the fmt step does NOT move security-plan's "six governed spawn forms"; a plain
  `run:` step with a fixed toolchain binary is the class of the existing `cargo build`/`cargo audit` steps.
  No seventh crossing, so no boundary-widening ratification is owed.
- **Routed out:** `a11y-plan:218`'s Linux+`xvfb` targeting is a11y's OWN claim, not a citation of arch
  (`grep -nE 'xvfb|ubuntu-latest' architecture.md` → 0 hits, control firing elsewhere), so it does not ride
  this pass — pre-existing drift for its own channel.

## Notes
- **Curation: T1 0 · T2 2 · T3 0** (filtered 3; 0 conflicts). Both survivors → `.claude/rules/testing.md`,
  both rescued from an exact-0.6 score by a conditional +0.2.
  - **An established command is not a green command** — baseline a gate against the exact targets it will
    run on. The novelty test keys on program + subcommand, which is what a long-established command passes.
  - **A criterion asserting an artifact is UNCHANGED needs a signature over its normalized form** — counts
    cannot discriminate; a probe that cannot fail on the property's negation is not evidence.
  - Rejected with homes elsewhere: the runner-portability mechanism (the minted route entry owns it), the
    `--all`/`default-members` reason (now in `architecture.md`).
- **My own P4 defect, owned in the report:** gate 7 was written into the plan without ever being run at
  HEAD. "This command is established" and "this command is green on these crates" are different claims.
  Second occurrence of the unsatisfiable-gate family in three chunks.
- **Last failed command:** none.

## Deferred learnings
None over the cap. **One `recurrence-despite-learning`:** a sweep of `a11y-plan` for runtime/driver
literals returned real hits (`:115`, `:217`, `:218`, `:333`, `:424`) and I characterised them as
colour-token pairs without reading them, then wrote an ABSENCE claim into the fan-out record. The operator
caught it. The disposition survived — those literals are dated DEV-HOST measurements, not provisional
hosted-image claims, so the falsification retires none of them — but the basis was wrong and is now
corrected in `a11y-plan-amendments.md`. It deduped against the already-correct 2026-09-06 read-the-hits
entry, so no third corpus entry was minted. **Third instance of that family in this one session.**
