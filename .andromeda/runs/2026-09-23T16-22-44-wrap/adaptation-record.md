# Adaptation record — 0-pending wrap, 2026-09-23T16:22:44Z

Path: Setup step 6, no-op path + P5 operator-requested adaptation (route-resolve §Operator-requested adaptation).
Master: 137 records · 137 complete · 0 pending. Tree at entry: only expected-transient bookkeeping (handoff,
friction log) + an untracked run-dir file from the prior wrap — counted git-clean.

Source: the operator's ADAPTATION RELAY (2026-09-23), passed as this wrap's arguments. Every item names its entry
and its disposition, which satisfies the trajectory gate (route-resolve §Edits + gradient); each applied line
cites the relay.

## Relay premises re-verified at `edc0af8` before applying
- `crates/conductor-core/src/drift.rs:48-57`: states the four ids need "injecting a known root cause with
  deterministic mode OFF". Confirmed.
- `.andromeda/architecture.md` = 148 508 B; `## Established Decisions` 49 134 B, `## Occupied Resources`
  48 859 B (per-section byte split in python). Their sum is 97 993 B = 66.0 %. Confirmed.
- `.andromeda/drift-base.md:27-31`: `D-arch-resources`' invariant is registration only. Confirmed.
- `.github/workflows/ci.yml:61-70`: the flakiness-budget step. Confirmed. `${{ env.` / `if: env.` across
  `.github/workflows/*.yml` → 1 hit, `ci.yml:354`, inside a comment. Confirmed 0 live references.
- `.claude/rules/host-win32.md:136`: the 2026-09-17 `${{ env.X }}` Session Addition. Confirmed.

## Items and dispositions
1a. **Minted** at `working-route.md:46`, directly above the cluster entry: *Real-model capture path handles
    guarded and stale read-back texts corrected*. The cluster's two CARRY blocks moved here verbatim (471 and
    548 chars, the same `route.py pins` sizes before and after). It has no leg and a CONTEXT naming its origin.
1b. **Annotated** the cluster entry (now `:50`; title, ids and CONTEXT unchanged) with `BLOCKED-ON: the
    real-model preflight canary`. It clears when one real-model drive gets past the canary.
2.  **Minted** at `:48`, directly after 1a: *Architecture registries compacted under the read cap*. It has a
    CONTEXT carrying the measured arithmetic, with the growth projection labelled `hypothesis:`, and a CARRY to
    propose a `D-arch-collision` detector at its wrap.
3.  **Annotated** `:55`, the *Secret-scanning CI gate* (the relay's `:51` before the insert), with a CARRY for
    the undeclared-env-key CI step, including the open GITHUB_ENV readability question.

## Standing annotations touched by this tail
- `:52` `BLOCKED-ON: one Pulse release emitting that observable`. Premise re-verified: Pulse HEAD is still
  `83d4060` (2026-08-31), which predates the 2026-09-13 measurement contract, so no release has shipped the
  observable. The block stands.
- No PREREQ stood on the old first markerless entry, so the insertion re-pinned none.
- Epoch growth: Epoch 4 now holds 2 frozen entries + 4 markerless. That is under the ~10 valve, so there is
  no nudge.

## Not run on this path
P1/P2 (no chunk, no report, no fan-out) · P3 (the conversation carried no corrections) · P7 gates/flip/compaction.
