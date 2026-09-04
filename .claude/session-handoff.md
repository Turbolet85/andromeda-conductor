# Session Handoff

**Last Updated:** 2026-09-04T17:40:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0` at `dd15dc3`, 2026-08-09;
**52 ahead** after this commit — still unpushed, so CI has not run since 2026-08-09)
**Status:** clean
**Last Commit:** `feat(2026-09-04-preconditions-probe-reads-path-handles-by-presence): …` (this wrap)

## Position
- Done: **`2026-09-04-preconditions-probe-reads-path-handles-by-presence`** — `handles-declared` graded
  by the KIND of value each handle carries, so `conductor preconditions` can exit 0 and `agent-run boot`
  reaches the preflight in both shells for the first time since `480bc66`.
- Next: **`/andromeda-phase`** to promote + plan **_Sidecar spawn without a console window_** — the first
  markerless entry, carrying **four CARRYs** (the `run_report`/`run_envelope` doc comments · the rustfmt
  edition mismatch · SR finding 1 `tabs_to_start` · SR finding 4 the load-time alert) **plus the standing
  cargo-audit PREREQ, now the 51st**.
- Coverage **26/32 verified · 6 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) —
  unchanged; this chunk claimed no capability (a defect remediation following the verified `v2-23`).

## Work done
Three files, no new files: two pure predicates in `conductor-core` beside `OBSERVED_HANDLES` —
`flag_declared(value)` (truthiness, no name) and `handle_declared(name, value)` (path handle by
presence-after-trim, every other name delegating to `flag_declared`) — with `conductor-run`'s `declares`
reduced to a thin env wrapper over `flag_declared` and `observe_preconditions` grading per handle.
One truthiness rule in the workspace; the run-contract path routes only to the value-only half, so it
**cannot reach the presence arm by construction**.

**Gates green:** nextest core 316 · run 175 · workspace 852 · `cargo test` both crates (runner
portability) · clippy 0 (one fix-loop iteration, `explicit_auto_deref`). **Mutation:** `conductor-run`
95 mutants (67 caught / 5 missed / 23 unviable), `conductor-core/preconditions.rs` 28 (27 caught /
**0 missed**). The `declares` accepted-deliberate roster measured **4 → 1** as predicted — the three
operator arms relocated into `flag_declared` and are caught there. A pre-existing insensitive check
(`is_unmet` surviving a constant-`true` mutant) was surfaced by that tier and killed.

**Live, both shells:** `conductor preconditions` exit 0 (17:07Z, the satisfied line's first-ever
execution), then `ReadyState` JSON with `ready: true` / `canary_round_trip: "ok"` / `data_dir:
"<redacted>"` from `agent-run.sh` (17:07:17→17:08:04Z) and `agent-run.ps1` (17:11:16→17:12:03Z), zero
`skipped preflight` lines in either, neither script edited. A distinctive data-dir value appeared **0**
times across every output arm.

## Drift resolved
**23 proposals from 7 doc-agents · 20 applied · 3 dismissed · 2 escalations resolved · drift = 0.**

- **arch ×3 sections** — the "third subject is UNSATISFIABLE" verdict and its two restatements retired;
  the `CONDUCTOR_PREFLIGHT_TIMEOUT` skip is now CONDITIONAL and the budget reached and paid.
- **security-plan ×3 (E1, operator-ratified)** — the withdrawn "not a downgrade" reading retired, the
  READ SET row re-pointed to the new predicates, the spawn-row parenthetical corrected (spawn duty
  untouched). A **new playbook rule** was minted for the class ("a chunk ships the fix its master's own
  body names as route-owned"), carrying an explicit defer-to-`Boundary widening`-first clause.
- **layout-templates ×3** — both `[PRECONDITION]` caption arms are now shipped behaviour, not intent.
- **test-plan ×10** — §1/§3 `boot` reachability, the five §6 step-1 lines, and the §12 mutation roster
  (enumerated SET `lib.rs:362:5` alone; classes B/C coordinates re-measured, crate total 8 → 5).
- **design-system ×3 DISMISSED (E2)** — D-platform-claim token-proxy mis-fire: none of the three sites
  states a single-runner capability verdict, and the platform verdict it does state is corroborated.
- **Cascade caught one intra-master duplicate** the detectors missed: `layout-templates:186`'s leading
  parenthetical contradicted the amendment 60 characters later on the same line.

## Notes
- **Curation:** Tier 1 ×1 (a plan's steps can be individually unambiguous and jointly contradictory —
  no gate compares a plan to itself) · Tier 2 ×2, both **additive-facet in-place amends**
  (`testing.md`'s mutation-output entry gains the gitignore-location facet; `verification-harness.md`'s
  fresh-dir entry gains data-dir equality + the quiet-window substitute) · Tier 3 ×0.
  `CLAUDE.md` **134/200**. Two candidates filtered at exactly 0.6 (the two-predicate decomposition; the
  positive-arm-only accessor) — both measured and generalizable, recorded in the friction ledger.
- **`pulse-app` (PID 17404) is still running** — the operator owns stopping it after this commit. It was
  kept up deliberately for this wrap's light gate.
- **Live-Pulse env for any re-run:** `ANDROMEDA_PULSE_DATA_DIR` must be the LIVE app's dir
  (`…\pulse-legs\a11y-20260904-190050` this session) — a Conductor-side fresh dir breaks data-dir
  equality and empties read-back; inter-leg hygiene is the quiet window (≥120s + 30s), not a fresh dir.
- **Unpushed branch stays load-bearing:** 52 commits ahead; *A11y CI gate*'s block cannot clear until a push.
- **Last failed command:** none.
