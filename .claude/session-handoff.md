# Session Handoff

**Last Updated:** 2026-09-06T09:42:50Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0` at `59d5b7c` — **3 ahead** after this
chunk commit. CI has not run since 33954347685 on `59d5b7c`; the a11y job is still absent from `ci.yml`, so
*A11y CI gate*'s BLOCKED-ON stands.)
**Status:** clean
**Last Commit:** `feat(2026-09-06-operator-gated-live-suite): …` (this wrap)

## Position
- Done: **`2026-09-06-operator-gated-live-suite`** — master's last `complete`, Epoch 6b's second chunk.
  `agent-run run --live` ships in both shells: a leading non-priming `conductor preconditions` probe, then
  B1 → B2 → a 150 s quiet window → A → the driven a11y arm, freezing each leg's self-obs because the sink
  truncates. **One green live run, exit 0, every leg landing its predicted state** — B1 `[MANUAL]`
  (`latency_ms 6045`), B2 `[BLOCKED]` (the deduped not-ready-but-connected gate), A `[RESIDUAL]`
  (`KnownResidual`, the AutoResolved arm). The driven a11y arm RAN rather than skipping: 1 passing in 54 s on
  WebView2 152.0.4191.62 — the real HOLD dialog, which is what CARRY 1 existed for.
- Next: **`/andromeda-phase`** to promote + plan the next markerless head — **_Run-report envelope conformance
  gate — every run journal row schema-complete and host-path-free, build failing on violation_**
  (`working-route.md:113`). No live Pulse needed for it.
- Coverage **26/32 verified · 6 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) — unchanged;
  this chunk claimed no capability (each pooled entry is owned by a different route entry; `v2-04` is the near
  miss and stays pooled, reasoning in the plan's Implementation notes).
- **Evolve:** Epoch 6a diagnosed (`.andromeda/runs/2026-09-05T07-42-45-evolve-diagnose/`) — no nudge; Epoch 6b
  is 2 chunks in, far below the growth valve.

## Work done
Three new files (`scenarios/auto-resolve-idle-window.toml` — one 165 s silent phase, `p_ids = ["P-022"]`,
declare-only; `crates/conductor-run/tests/live_suite.rs` behind the existing `live-pulse` feature, the capture
tool that PRINTS; `crates/conductor-run/tests/live_suite_harvest.rs`, the graded half pinning the measured
lines as literals in the default suite) plus the `--live` stage arm in both shells. `Cargo.lock` untouched at
564 packages; `crates/conductor-run/Cargo.toml` byte-unchanged — the existing feature sufficed.

**The three CARRY'd live-path observables are proven from persisted records**, and observable 3 on BOTH arms
(gate line present in B2 only — b1 0, b2 1, a 0), which is what the inverted-guard mutant requires. Observable 2
measured `latency_ms 6045` against a 6000 ms instant difference — **45 ms skew**, inside the 1 s bound the
instants' second precision imposes. These do NOT retire the tier acceptances; test-plan §12's roster is
untouched.

**Measured, and better than predicted:** leg B1 landed `ManualCheck`, not `KnownResidual` — so
`observation.degraded` was FALSE under deterministic L4. That makes observable 1's discriminator meaningful
rather than constant-true, and it falsified a spec universal (below).

## Drift resolved
**12 amendments applied · 1 escalation resolved · 1 proposal rejected · 3 withdrawn at the source.**
17 proposals from 7 doc-agents; full record at `.andromeda/runs/2026-09-06T09-37-04-wrap/fanout-results.md`.
- **`architecture.md:69`** — the universal "under deterministic L4 every read-back returns `degraded_mode`" is
  RETIRED, measured false by leg B1. `degraded` now reads as a per-read-back property, measured both ways. The
  degraded route to `KnownResidual` and `findings-counter-refresh`'s own outcome are unchanged.
- **`architecture.md`** — `runs/live-suite/{leg}.jsonl` registered in §Occupied Resources; the `runs/` tree
  gloss narrowed so "never overwritten" no longer covers the leg-stemmed child.
- **`obs-plan.md` ×3** — `degraded_mode_response` retired at §3, §4, §6: zero occurrences under `crates/`, never
  implemented. Applied NARROWED — only that field was measured, so the wider "no scenario extra exists" the
  proposals asked for was not applied.
- **`test-plan.md` ×5 · `layout-templates.md` · `security-plan.md` ×2** — the `--live` stage registered across
  §2/§3/§9/§11 and the cli surface map; the preconditions short-circuit restated over the path SET
  `{boot, run --live}`.
- **REJECTED — the `sleep(N)` carve-out.** A detector asked §11 to PERMIT a fixed wall-clock wait. The 150 s
  quiet window is in the harness BETWEEN legs, not inside a test, so the ban was never violated and needed no
  exception. Applied instead: a scope clarification that re-asserts nothing inside a test may sleep to
  synchronise. A ban is not weakened to accommodate something outside its scope.

## Notes
- **ESCALATION resolved by fixing the code, on your ruling.** The security detector found the harness's first
  recursive delete — `rm -rf "$RUNS_DIR/live-suite"` on a `CONDUCTOR_RUNS_DIR`-derived path with no
  canonicalize, no `--`, no non-empty assertion, executing BEFORE any Rust-side `resolve_under` rejection could
  run. Rather than document the residual and widen two bans, both shells now use the file's own non-recursive
  `cleanup` idiom (`rm -f …/*.jsonl`). `grep -nE 'rm -r|Remove-Item -Recurse'` over both scripts → **0 hits**.
  No security ban was widened.
- **Curation:** Tier 1 ×1 extension (the universal-quantifier facet — enumerate the family before writing
  "every"/"no"; and a correct neighbour is not evidence) · Tier 2 ×2 new (`verification-harness.md`: the four
  silent `--live` mechanics; `security.md`: no `CONDUCTOR_*`-derived path into a recursive delete) · Tier 3 ×0 ·
  filtered 0 · conflicts 0 · deferred 0. `CLAUDE.md` **134/200**.
- **Light gate:** nextest **878/878** · doctest ✓ · clippy workspace ✓ · clippy `--features live-pulse` ✓ ·
  advisory-db porcelain clean (HEAD `5a0ebedf`) → `cargo audit` exit 0 (18 allowed warnings) · `cargo deny` all
  four ok · `Cargo.lock` 564 · catalog gates 12/12 · both shells' fall-through exit 2 · refusal path exit 1
  naming both unmet subjects with 0 host paths · live suite exit 0.
- **Carried to *A11y CI gate*:** a PRE-EXISTING inconsistency in `a11y-plan.md` its own doc-agent surfaced as
  out-of-scope and nobody proposed — `:565` still calls reduced-motion "the one platform-dependent assertion"
  while `:218`/`:424`/`:597` already retired that verdict. Re-verify those line numbers before acting; they are
  the agent's, unre-derived here.
- **Gitignored residue of the run** (rides no commit): `runs/live-suite/`, `runs/scenario-load-check/`,
  `runs/gate-*.log`, `runs/live-suite-run.log`, `runs/live-refusal-*.log`. The frozen copies under the chunk's
  `evidence/` and the harvest literals pin the FIRST run's lines; a later `--live` run mints new run_ids and
  overwrites only `runs/live-suite/`.
- **`pulse-app`** was UP for this whole session (overseer-launched, pid 64156, holding `:4317`, data dir
  `…\pulse-legs\a11y-20260906-110201`) and is **left running — the overseer stops it after the commit**. The
  wrap never stops it; a leg never kills what it did not start. Post-leg census matched the pre-leg baseline
  exactly — the whole wdio tree self-terminated, no listener on `:4444`/`:4445`.
- **Live-Pulse env for any re-run (unchanged):** POSIX-form `PATH` prefix resolving `andromeda-pulse-mcp`,
  `ANDROMEDA_PULSE_DATA_DIR` = the live app's dir (equality), `ANDROMEDA_PULSE_MCP_ENABLED` +
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` true. The whole block in ONE paste — a partial set fails silently as a
  ~0 s `[BLOCKED]`. Order: `conductor preconditions` (non-priming) → `bash scripts/agent-run.sh run --live`.
- **Last failed command:** none.
