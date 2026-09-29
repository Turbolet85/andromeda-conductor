
## 2026-09-29-hue-shift-budget-graded-hard — P-025 graded hard; the retired instrument kept as history
**Section:** §4 Span / Trace Coverage → Known-residual classification path → Delegated-timing family (the P-025 passage)
**Change:** was "P-025's ≤2s bound is UNMEASURABLE through `metric.constellation.hue_update_ms` … TICK QUANTIZATION … P-025 therefore grades as a MECHANISM PIN with no pass arm". It also said the bound "lifts only if Pulse emits a DIFFERENT QUANTITY … `transitioned_at_unix_nano` when it falls … Still Pulse intake", with every coordinate as measured at `83d4060`.
Now P-025 grades HARD at the harvest tier: PASS, worst 684.98 ms, from one graded leg against a checkout carrying `e98d838`. That leg's in-window samples were a rise of 684.98 ms (anchored to its incident's creation line at 29.98 ms) and a fall to `none` of 430.79 ms.
- `duration_ms` is paint minus `tier_effective_at_unix_nano`. The rise source is `opened_at_unix_nano`; the fall source is `resolved_at_unix_nano`, not `transitioned_at_unix_nano`. Acknowledgement does NOT lower the tier. Pulse emits one sample per changed service, and only for changes the canvas witnessed.
- The grade is taken by `grade_in_window` under the contract's §The grading rule: every sample in [phase-2 start, `scenario.run` close], worst observation, inclusive, absence UNGRADED, a breach a hard Fail relayed to Pulse. All four delegated-timing bounds now grade hard.
- SCOPE: deterministic L4, the widget visible, the rise poll-bounded.
- The tick-quantization reading at `83d4060`, and its test, stay as the retired instrument's record. So does the finding that neither named SUT candidate would have lifted the bound; the shipped fix is neither.
- Pulse coordinates are read at `226554a` unless dated `83d4060`.
**Why:** the passage named its own precondition (Pulse emitting the contracted quantity) and its own expiry, and the graded leg is that precondition, measured. The fall source and acknowledgement were measured wrong at `226554a`, which is the CARRY's two premises, now verified.
**Ref:** .andromeda/runs/2026-09-29T21-30-19-wrap/
