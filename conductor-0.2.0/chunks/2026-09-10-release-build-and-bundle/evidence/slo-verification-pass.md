# Final SLO verification pass — against the SHIPPED artifact

Legs driven through `./target/release/conductor` (the binary this chunk built,
reporting `conductor 0.2.0`), each behind a 150 s quiet window, never preceded by `agent-run boot`.
Live Pulse: operator-launched 21:16:17 local, PID 29608, fresh data dir, deterministic L4, MCP on,
**default bootstrap window** (no override).

## Result — the pass, on three clean legs

| leg | scenario | P-ID | state | tier | `latency_ms` | deadline | margin | SLO | `run_check` |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `live-only-service-truth` | P-067 | `ManualCheck` | `<20s` | 18 115 | 20 000 | 1 885 | **PASS** | 0 |
| 2 | `exception-event-capture` | P-006 | `Pass` | `<5s` | 2 063 | 5 000 | 2 937 | **PASS** | 1 |
| 3 | `investigate-actions-functional` | P-072 | `ManualCheck` | `<90s` | 35 115 | 90 000 | 54 885 | **PASS** | 0 |

All three are non-`Blocked`, all three exited 0, and **`latency_ms <= threshold(slo_tier)` holds on
every one** — the measurement `v2-27` names. `run_check` rows measure **0 / 1 / 0**, matching each
scenario's declared shape (legs 1 and 3 are declare-only with an empty `expected`).

Leg 3 is a **swap, made between runs on operator direction and disclosed here and in the report**.
The originally-planned leg 3 was `cross-incident-recurrence` (P-036); it hard-failed and its reading
is retained below as committed evidence rather than discarded. The replacement was chosen on a
**prior live green** rather than tier arithmetic: P-072 had been driven live twice at the previous
chunk (35 120 ms and 35 111 ms) and carries the widest margin of the proven set. This run makes
**three measurements across two days — 35 120 / 35 111 / 35 115, a 9 ms spread** — so the leg is
demonstrably run-stable, which is precisely what the failed leg lacked.

The tier-duration model held on every leg: declared phase sums of 18 000 / 2 000 / 35 000 ms predicted
18 115 / 2 063 / 35 115 — within 115 / 63 / 115 ms. Seven independent measurements now support it
(four here including the retained failure, three at the previous chunk).

## Retained: the `cross-incident-recurrence` FAIL

| scenario | P-ID | state | tier | `latency_ms` | deadline | SLO | `run_check` |
|---|---|---|---|---|---|---|---|
| `cross-incident-recurrence` | P-036 | **`Fail`** | `<20s` | 6 151 | 20 000 | PASS | 1 |

Journal and self-obs committed as `leg3-2026-09-10T19-40-47-134.*`. Note its SLO assertion PASSED
(6 151 ≤ 20 000) and it was never `Blocked` — the failure is a scenario CHECK, diagnosed below. It is
kept because a discovered pre-existing defect is a deliverable, not a run to be re-rolled away.

Deliberately EXCLUDED: `constellation-severity-live-wiring`, whose `<20s` tier is unattainable by
construction and already ledgered open in `.andromeda/residuals.md`. The exclusion is about the
unattainable TIER only — the capability that scenario stands for was verified non-blocked by `v2-04`
at the previous chunk, so this pass forgoes no coverage this chunk owes.

## Leg 3 — a pre-existing structural defect, surfaced not fixed

`[FAIL] cross-incident-recurrence`, exit 1, `state: Fail`. The failing check is index 0:

```
kind = "Contains", class = "Hard", expected = "Previously seen"
```

**It is not caused by anything this chunk changed, and it is not a transient.**

- The scenario has **never been driven live before** — no chunk's `evidence/` names it. This was its
  first live run, so its checks had been authored but never validated against a real SUT.
- Read-back **succeeded**: the self-obs stream records
  `retrieve_telemetry_slice returned keys [fingerprint_refs, incident…]` and
  `read-back observed the incident corpus`. The corpus was read; the token is simply not in it.
- The token is **inferred, and the scenario says so itself**: `:15` — "the 'Previously seen' reference
  token is INFERRED (the spec uses prose…)" — while `:12` declares it `Hard` on the belief that it is
  "an index lookup, not a model judgement".
- **`grep -rn 'Previously seen' crates/` returns FOUR hits**, all in
  `crates/conductor-core/src/scenario.rs:1189-1202`, inside
  `#[test] fn cross_incident_recurrence_asserts_previously_seen_via_hard_contains`. That test reads the
  TOML and asserts the scenario **DECLARES** `Contains` + `Hard` + `"Previously seen"` — it pins the
  DECLARATION, never the SUT's behaviour. So the conclusion is unchanged (nothing on the SUT-facing
  path produces the token), but the correct statement is "no PRODUCER exists", not "no reference
  exists".

  *(Correction: an earlier draft of this file said the grep "returns nothing". That was false and the
  mechanism is worth keeping — the probe actually run was
  `grep -rn 'Previously seen' scenarios/ crates/ | head -5`, whose five slots were entirely consumed
  by the scenario's own comment lines, and a claim about `crates/` was then written from a clipped
  view of a combined search. A verdict read off a truncated result, which the shell discipline
  forbids explicitly. Operator-caught.)*

  Two consequences for the residual. Retiring this class later **touches that test too** — it would go
  red the moment the declaration changes, so it is part of the class's blast radius. And its existence
  is a plausible reason the declaration was never swept: a green test carrying the token's name makes
  the declaration look validated when what is validated is only that the TOML says what it says. That
  makes this finding a member of the companion class the project already tracks — **a test whose INPUT
  is the artifact under change** (testing.md, 2026-06-22 extended 2026-09-07). Note the prescribed
  sweep for that class, `grep -rln "<scenario-name>" crates/**/tests`, would **miss** it: the test is
  an inline `#[cfg(test)]` module in `src/`, not a `tests/` binary.

This is an unswept member of a class this project has already ruled on twice: an INFERRED `Contains`
token declared `Hard` against a read-back surface that cannot carry it under deterministic L4 — the
same shape as `Contains "RetryStorm"` (retired to declare-only at `2026-08-16-fingerprint-storm-live-proof`)
and the severity-tier tokens (retired at `2026-08-21`). It is the exact sibling of the `CountAtLeast`
class the P-079 residual describes.

**Not fixed here, for two reasons.** `scenarios/` is not in this chunk's modify-set (out-of-scope by
the fix-loop's Trigger 3), and the standing ruling on its sibling class is to *"take the class
together rather than retiring this one alone, which would contradict an accepted sibling"*. A
piecemeal retire of one token would repeat the error the residual warns against.

**Authoring cause, owned:** the legs were selected at P4 on TIER ATTAINABILITY alone — summed phase
duration against the tier deadline — and not on whether their checks had ever passed live. Leg 1 was
chosen because it was proven live the previous chunk; legs 2 and 3 were not, and leg 2 passing was
luck rather than evidence. This is the "an established command is not a green command" lesson applied
to scenarios: a scenario whose checks have never been exercised live is UNKNOWN, not green. A leg
selection should require a prior live green, or accept that the leg is itself an experiment.
