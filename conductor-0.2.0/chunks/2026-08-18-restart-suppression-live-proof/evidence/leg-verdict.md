# Leg verdict — restart-suppression live proof

**Run** `2026-08-18T21-40-46-519` · fresh data dir · one leg, all witnesses.
**SUT:** andromeda-pulse HEAD `efabe8e` (source tree clean except its own friction-log);
`pulse-app.exe` built 2026-08-17 23:04:33, `andromeda-pulse-mcp.exe` 2026-08-17 22:27:25 — the same
ratified pair leg A of the prior chunk ran.
**Recipe:** the six-item set, with two deliberate properties — NO `SEED` in the environment (the
TOML-declared 4317015 must govern: the disposition's live proof) and NO `RUST_LOG` (a suite-running
leg). `pulse-app` launched from the data dir (outside any git workspace); sidecar resolved from the
debug dir via PATH.

## Outcome — every designed witness landed, at the predicted sample offsets

| Witness | Predicted (design arithmetic) | Measured |
|---|---|---|
| P-015 restart pair | 2 × `restart_emit`, gap > 20s | 2 × `restart_event`, gaps **25s, 26s** |
| P-016 suppressed half | drops at persistence 10–13 | check-lines at **10, 11, 12, 13** (in-window, not bypassed); **0 leaked** young not-bypassed emits |
| P-057 absolute arm | triggers at ~21–29, reason absolute | **10 triggers**, all `("error_rate_spike","absolute")`, persistence **20–29**; kept young cues abs 0.057–0.088 (the predicted 8.6% → 5.7% decay) |
| P-016 surgical crossing | plain-kept from ~38 | **41 plain-kept** emits, persistence **38–49+**, abs < 0.05 (sample: mag 4.97, conf 0.38, curious) |
| Autonomous keeper | ≥ s90, mag ~5–6.7, conf ≥ 0.9 | **22 autonomous** cues from persistence **90**, mag **5.91 → 6.73**, conf **0.90 → 0.92+** |

Envelope: `seed: 4317015` (no harness override — the SEED disposition proven live), `verdict: null`,
`state: KnownResidual` (the declare-only landing, non-Blocked), `latency_ms: 168192` vs the ~168s
emission window (whole-run basis; `<90s` = the honesty bucket), fingerprints = the constant `det-*`
triple (payload-invariant, as measured for the prior family). No `[ENVIRONMENT-SUSPECT]` caption —
in-envelope. Preflight went ready fast: `query_incident_list` 0 → **1 incident at the second poll
attempt** (~21:41:32); the canary incident survived the whole scenario window (9 auto-resolve ticks
in-slice, no resolution), so the row never depended on the keeper incident.

## New measurements (this leg)

1. **`triage.cue.tick`'s `cues_suppressed` / `bypass_triggered` counters are REDACTED live**
   (`"<redacted>"` on every tick line) — Pulse's default-deny field allowlist predates the
   chunk-#63 counters. The suppressed-half predicate therefore rides the check-line + emit-absence
   pair, never a counter. **Next-visit intake** (their §8 leaf for `triage.cue.tick`).
2. **The relative bypass arm never fired and cannot**: all 10 triggers read `absolute`. Confirmed
   arithmetic: alpha_short/alpha_long = (1/30)/(1/300) = 10 equals `magnitude_bypass_multiplier`,
   and the 0.01 baseline floor caps the transient young-service peak at ~9.7x — a relative-labeled
   trigger is unreachable for error cues at shipped constants. **Next-visit intake** (the α-ratio
   coincidence looks unintended).
3. **`persistence_seconds` = cumulative service samples** (the premise correction that reshaped
   this chunk) held exactly live: the drop band opened at the warm-up floor (persistence 10 = the
   service's 10th span) and every offset matched the sample arithmetic, because the scenario paces
   ~1 span/s.
4. The keeper incident was not yet in `query_incident_list` at read-back (count stayed 1 — the
   canary's; the autonomous cues fired ~26s before read-back, inside L3/L4 latency). The row never
   needed it; recorded for completeness.

## Honest limits

- One service, one leg: the suppression semantics are proven under Pulse's **sample-count**
  persistence (which the capability spec words as spike *duration* — the divergence is recorded as
  intake, and the scenario's 1 span/s pacing is what makes the two coincide numerically).
- Freshness remains the read-back carrier; nothing here strengthens payload identity (unchanged
  deterministic-L4 limit).
- The P-057 claim is the absolute arm + the neither-condition suppressed case; the relative arm is
  recorded unreachable-by-construction, witnessed only as Pulse's own label vocabulary.

## Artifacts

- `leg-witnesses.jsonl` — verbatim slice extracts (restart detect/emit ×2, drop check-lines,
  bypass triggers, a redacted tick sample, the young-bypassed / plain-kept / autonomous emits).
- `2026-08-18T21-40-46-519.jsonl` / `.md` — the run journal + report; `envelope-status.json` —
  the mint-then-read `status` readout (run_id match verified).
- The pinned-verbatim fixtures + predicates: `crates/conductor-run/tests/restart_harvest.rs`
  (7 leg lines, byte-identical; all witnesses green under `cargo nextest`).
- Capture basis: `{data_dir}/logs/agent-latest.jsonl.2026-08-18`, pre-leg line count 15,595,
  post-leg 97,176 (81,581-line slice; slice retained in the session scratchpad, not committed —
  the witness extract above is the committed record).
