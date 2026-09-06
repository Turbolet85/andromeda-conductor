# Live-suite verdict — 2026-09-06-operator-gated-live-suite

**Captured:** 2026-09-06, one invocation of `bash scripts/agent-run.sh run --live`, exit 0.
**SUT:** live `pulse-app` at HEAD `83d4060`, deterministic L4 + MCP enabled, fresh data dir, started by
the operator before the leg and left running after it.
**Firing form:** the six-item env block in one paste — POSIX-form `PATH` prefix resolving
`andromeda-pulse-mcp`, `ANDROMEDA_PULSE_DATA_DIR` equal to the live app's dir, `ANDROMEDA_PULSE_MCP_ENABLED`,
`ANDROMEDA_PULSE_L4_DETERMINISTIC`. The probe ran first (`conductor preconditions`, exit 0, non-priming —
it fires no canary) and the suite followed.

## Legs, predicted vs measured

| leg | scenario | predicted | measured | frozen |
|---|---|---|---|---|
| b1 | `degraded-mode-report` | gate ready; graded route; a row carrying `latency_ms` | `[MANUAL]` · `latency_ms 6045` | `b1.jsonl` (74 lines) |
| b2 | `degraded-mode-report` (immediate) | canary dedupes inside the 120s window; gate not-ready-but-connected; `Blocked` row at exit 0 | `[BLOCKED]` · gate line present | `b2.jsonl` (233 lines) |
| a | `auto-resolve-idle-window` | fresh canary, then 165s idle outlasts the 120s+30s window; empty active set | `[RESIDUAL]` · `KnownResidual` · `verdict null` | `a.jsonl` (66 lines) |

The driven a11y arm ran (not skipped) on WebView2 152.0.4191.62: **1 passing in 54s** —
`the real HOLD dialog traps focus, Space toggles a row, Escape resolves NoGo and restores focus`. This is
the arm CARRY 1 was about; the quiet-window shape delivered a real operator hold without a collision.

## The three observables

**1 — the `AutoResolved` degraded route.** Leg a's envelope carries `state "KnownResidual"` /
`verdict null` / `fingerprints []`, and its self-obs stream carries the route discriminator
`declare-only read-back empty: no active incident outlived the emission window`
(`target conductor_run::execute`), which only that arm emits. There is no `degraded` field on the
envelope to read — `RunRecord`'s eleven fields carry none — so the state mapping is the proxy.

**The mapping was measured to DISCRIMINATE, not to be constant.** Leg b1 drove the same declare-only
shape through the graded route and landed `ManualCheck`. So `KnownResidual` on leg a is a measurement,
not a foregone conclusion — which also corrects a standing reading: under deterministic L4 a read-back
does **not** always come back degraded (b1's did not).

**2 — the manual-path latency subtraction.** Leg b1: `journal_emitted_at 2026-09-06T09:11:55Z`,
`read_back_observed_at 2026-09-06T09:12:01Z`, `latency_ms 6045`. The instants differ by 6s = 6000ms;
the skew is **45ms**, inside the 1s bound the instants' second-precision imposes. A sum or a quotient
at these values misses by orders of magnitude, so the bound still separates them.

**3 — the readiness-gate line, both arms.** Present in leg b2 only:
`preflight blocked: readiness gate not satisfied` (`target conductor_run::canary`); absent from b1 and
a. Both arms are needed because the mutation site deletes the `!` in the guard, and an inverted guard
passes a one-arm assertion. Matched on the exact string: `canary.rs` carries three other
`preflight blocked:` lines whose connect arms return early.

**These do NOT retire the tier acceptances.** The three sites stay accepted-deliberate in test-plan §12 —
a live leg is never a CI gate. What this buys is the behavioural coverage the accepted class cannot.

## Grading

`crates/conductor-run/tests/live_suite_harvest.rs` pins the lines above as literals and grades them in
the DEFAULT suite (5 tests, all passing). `crates/conductor-run/tests/live_suite.rs` is the capture tool
behind `--features live-pulse`; it prints, it does not assert.

## Process census

Taken from the host process list before and after — a pre-leg baseline is what makes the post-leg
reading mean anything.

| process | started by | final state |
|---|---|---|
| `pulse-app.exe` (pid 64156) | the operator, for this run | **left running** — the SUT; the operator stops it. A leg never kills what it did not start. |
| `tauri-driver` | the driven a11y arm | terminated — wdio `onComplete` → `tauriDriver.kill()` |
| `msedgedriver.exe` | the driven a11y arm | terminated — same teardown |
| `conductor-tauri.exe` | the driven a11y arm | terminated — same teardown |
| `node.exe` (wdio workers) | the driven a11y arm | terminated — `@wdio/local-runner` shut down gracefully |
| `andromeda-pulse-mcp` | each leg's preflight | terminated with its leg's `conductor` process |

Post-leg listing matches the pre-leg baseline exactly: only `pulse-app.exe` remains, holding `:4317`.
No listener on `:4444` / `:4445`. **Stop form** for an interrupted run:
`taskkill /F /IM msedgedriver.exe /IM conductor-tauri.exe` plus the node CLI.

## Second run — the wrap's light gate, and what it changed

The wrap re-ran `agent-run run --live` literally (2026-09-06, same env, same tree, exit 0). **Observables 2
and 3 reproduced exactly** — B1 `[MANUAL]`, B2 `[BLOCKED]` with the gate line present in B2's capture only
(b1 0 · b2 1 · a 0), and the a11y arm passed again (1 passing in 54.1 s, WebView2 152.0.4191.62).

**Observable 1 did NOT reproduce, and that is the finding.** Leg A landed `[MANUAL]`, not `[RESIDUAL]`: its
envelope (run `2026-09-06T10-07-08-396`) carries `state "ManualCheck"`, `latency_ms 165055` and **3
fingerprints** — the preflight canary's incident was still ACTIVE at read-back, so `observe` returned it,
`route_read_back` took the `Graded` arm and the `declare-only read-back empty` line never fired.

Cause is arithmetic, not flakiness in the mechanism: Pulse's worst case is 120 s idle **plus up to a full
30 s observer tick = 150 s**, and the scenario's window was 165 s — a 15 s margin. Run 1 cleared it; run 2
did not. The scenario is therefore widened to **`gap_ms = 200000`** (50 s margin), and **that value is not
yet live-proven** — the next live leg confirms it.

What this does and does not cost: the harvest's pinned literals remain a true record of what run 1 measured
(the `AutoResolved` arm IS reachable and this is what it looks like), and observables 2 and 3 are proven
twice. Observable 1 is proven ONCE, on a window now known to have been marginal.

## Honest limits

- The suite is **not** a CI gate and must not become one (test-plan §9; architecture §Established
  Decisions [CI/CD]).
- Leg a's row records `latency_ms 165050` against a declared `slo_tier "<90s"`. Nothing is graded
  against that tier — a declare-only scenario has no `[[expected]]` checks and carries `verdict null` —
  so the tier is a declaration, not a breached bound. Same shape as `incident-auto-resolution`, whose
  header records the identical reasoning.
- `ANDROMEDA_PULSE_L4_DETERMINISTIC` is load-bearing for observable 3, not merely for reproducibility:
  canned-L4 formation is ~2s, so b2's storm lands well inside the 120s window. Real-model formation
  (~110s measured) would push it outside and silently cost the present arm.
- Observable 3's absent arm is graded from the measured per-leg presence triple, not from a
  line-by-line replay of all 373 frozen lines.
