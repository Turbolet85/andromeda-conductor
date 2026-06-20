# Fan-out results — 2026-06-19-abrupt-silence-fault wrap

7 doc-agents, one per spec source. Report = single source.

| doc | proposals |
|---|---|
| arch | `[]` — no new crate/port/IPC/env; AbruptSilence is a lib symbol in the already-registered conductor-faults crate |
| security-plan | `[]` — no external-input surface, no subprocess/data-dir touch, no new deps |
| design-system | `[]` — no UI element |
| layout-templates | `[]` — no user-facing surface |
| test-plan | `[]` — unit+doctest at mandated tier; nextest/clippy/llvm-cov match §2/§4; explicitly reasoned the deferred fault_duration_ms is NOT drift |
| obs-plan | **1 proposal** — D-obs-instrumentation (warning) ↓ |
| a11y-plan | `[]` — no interactive UI element, no schema change |

## obs-plan proposal (D-obs-instrumentation, warning)
- **section:** obs-plan §4 (Span/Trace Coverage — fault-injection spans)
- **change:** the `fault.silence` span must carry `fault_duration_ms` as an explicit null sentinel when the silence is permanent (vs an integer ms for a bounded gap), so a permanent stop is distinguishable from a deferred-instrumentation gap.
- **rationale (agent):** report flags the `fault.silence` span as deferred + notes a permanent stop has no finite duration; obs-plan §4 lists fault.silence (with fault_duration_ms) as a must-trace span.

## Orchestrator validation
- **Playbook line 22-24 principle** (downstream-sequenced deferral = build-sequencing, not drift, when the chunk leaks/violates nothing): the `fault.silence` span instrumentation is route-sequenced to Epoch 7/8 (timeline wiring); this chunk instruments nothing and leaks nothing → the *deferral* is not drift. (Rule's literal scope says "Foundation-epoch"; this is Epoch 4 — partial match, hence escalate to confirm + generalize.)
- **Precedent:** the immediately-prior gap chunk (P-015, also Epoch 4) had the identical obs-instrumentation deferral and its obs detector returned `[]` (self-cleared). The over-fire here was triggered by the report explicitly naming the fault_duration_ms tension.
- **D-arch over-reach precedent (playbook line 37-39):** detectors over-fire on these self-contained fault-helper chunks; dismissal-by-confirmation is the established loop.
- **The proposed amendment is forward-design, not current-truth drift:** obs-plan §4's fault.silence description is the *intended* instrumentation (valid for when wired); nothing this chunk did made it false. The null-sentinel nuance is an Epoch-7/8 design refinement.
- **Verdict:** escalate (partial playbook match + a bundled forward-design idea) → recommend DISMISS + codify a generalized playbook rule + keep the null-sentinel as a tracked follow-up.
