# Merge Decisions — conductor-0.2.0 route

_Phase 3 audit. 25 suggestions from 5 validators: 13 applied · 8 adjusted · 2 rejected · 2 deferred._
_Tally note: the Deferred section below lists **three** entries — the third (a11y gate nature) is a HALF-deferral whose applied half is counted under `adjusted`, so it does not add a fourth to the deferred count. 13+8+2+2 = 25._
_All three deferred entries were RESOLVED by the operator in Phase 4 — see `review-feedback-1.md`. This file is the point-in-time Phase 3 record and is not rewritten._
_Chunk count 28 → 34 (6 Inserts applied). Format: `{validator} {type} · {decision} · {reason}`._

## Applied

security Rewrite · applied · SUT capability manifest gains "validated … malformed reporting blocked" — a new on-disk parsed artifact is a trust boundary security-plan §Input Validation requires validated at load, and the never-silently-downgrade rule is a universal invariant.
security Rewrite · applied · Workspace-key divergence probe gains "host-path-free" — the precondition string is detected-root-vs-data-dir by construction, so it is an absolute-host-path leak unless constrained (security-plan §Anti-Patterns Logging).
security Rewrite · applied · pii-scrub live proof gains "verdicts recorded without persisting excerpt content" — a failing scrub assertion naturally wants the offending excerpt, which would persist SUT-owned corpus content (and PII) into Conductor artifacts (security-plan §Anti-Patterns Data Protection).
design Insert · applied · "Out-of-scope classification treatment" into Epoch 1 — v2-03 introduces a FOURTH classification bucket that has no treatment among design's six status treatments / six CLI bracket prefixes, so not-Conductor's would collapse into Blocked and the completeness gate would read it as a gap.
design Rewrite · applied · Operator-pause live firing gains "frozen-count hold-point" — the count freezing at the exact hold value is the signature element a live hold must prove (design §Brand Identity, layout-templates §Signature placement).
design Rewrite · applied · Dependency polish gains "preserving the stop-in-place hold spinner" — an indicatif major bump can silently regress the CLI mirror of the signature (design §Surface: cli Component Patterns, §Per-Surface Bans).
tests Insert · applied · "Dispatcher determinism goldens" into Epoch 2 — replacing coarse_emit changes the emission stream shape 0.1.0's committed insta goldens lock, and the zero-retry flakiness budget rests on that invariant (test-plan §1 property-test trigger, §7, §10).
tests Insert · applied · "Webview E2E harness leg" into Epoch 5 — test-plan §9 defines `agent-run run --e2e` → tauri-driver under xvfb with §3 `.sh`/`.ps1` parity, a leg the Epoch 5 sweep and gate both assume but 0.1.0 never covered.
tests Rewrite · applied · Per-check read-back extraction gains "degraded_mode mapping to KnownResidual" — test-plan §6 Scenario 5 / Critical Path 5 names it and no chunk carried it; arch §Read-Back Dependency Posture assigns KnownResidual to exactly this case.
obs Insert · applied · "Fault-application spans" into Epoch 3 — obs-plan §4 mandates fault.silence/ramp/port_occupier beneath timeline.execute, conductor-faults has no instrumentation, and Epoch 3's restart-suppression + connection-lifecycle legs exercise exactly those faults.
obs Insert · applied · "Run-report envelope conformance gate" into Epoch 6 — obs-plan §9 records the envelope gate as explicitly "not-yet-built" (the shipped gate asserts only the §3 self-obs base line), and 0.2.0 is where the envelope first carries live read-back values.
obs Reorder · applied · scenario.run root span tree moved Epoch 5 → Epoch 2 (before First live green preflight) — obs-plan §4 roots all seven must-trace paths at scenario.run, so leaving it late runs five family proofs plus the lifecycle pass with leaf spans and no root correlation.
obs Rewrite · applied · Per-check latency measurement rephrased onto the closed slo_tier set with per-check latency_ms in the run-report envelope — obs owns latency_ms/slo_tier, so a finer budget must extend that record rather than fork a parallel one (obs-plan §5/§10).

## Adjusted

security+a11y Rewrite · adjusted · A11y CI gate merged: security's "redacted" applied and a11y's "no retries" applied; the disputed sink phrase ("into the obs envelope") was DROPPED rather than re-worded, leaving the chunk neutral on record shape — that question is deferred below.
design+a11y Reorder · adjusted (deduped, same direction) · Live per-P-ID verdict lamps moved to the head of Epoch 5 rather than into Epoch 4. Satisfies a11y's hard dependency (the sweep's six-state contrast + not-color-alone assertions need all six verdict states rendering) and design's earlier-is-better direction; not pulled into Epoch 4 because that would put GUI work inside the live-proof epoch, and design's step-index-echo dependency is soft (the header echo already shipped in 0.1.0).
design Rewrite · adjusted · Live per-P-ID verdict lamps gains "not-yet-run rows visibly distinct from failed" (design §Rejected Defaults — a real hazard when most P-IDs will have no live run); the "header echoing the held step index" half was dropped as already shipped in 0.1.0's paused-count-hold-point-signature.
tests Insert · adjusted (placement) · "Operator-gated live suite" placed at the HEAD of Epoch 6 rather than after the completeness gate — the gate asserts every CI gate green and the live suite is its evidence source, so it must precede it.
a11y Insert + Rewrite · adjusted (merged into one rewrite) · The proposed "Six-state lamp and motion assertions" chunk and the "Desktop a11y sweep" rewrite were folded into a single sweep line covering axe, contrast, not-color-alone, keyboard-trap escape, focus restoration and reduced-motion — same harness, same session, so a separate chunk would be a micro-chunk; the five explicit SC ids were dropped as domain content the plan already owns.

## Rejected

a11y Insert · rejected · "A11y harness install" — already realized: the axe-core/Lighthouse tooling over the tests' tauri-driver/WebdriverIO session shipped at 2026-06-27-desktop-a11y-harness-setup, and intent §4 F14 says explicitly "Do NOT re-author the harness". The validator saw only a11y-plan + the draft, not the 0.1.0 route.
a11y Insert · rejected · "Token-pair contrast harness" — same: colorjs.io token-pair contrast shipped with that same 2026-06-27 chunk (intent §3 and F14 name "axe-core / colorjs.io / keyboard"). Epoch 5 RUNS this harness, it does not build it.

## Deferred to user (Phase 4)

security Insert · deferred · "Secret-scanning CI gate" (Epoch 1) — a genuine unrealized security-plan §Bootstrap phases item, but NOT among the intent's 22 findings, so it is scope beyond the authored version intent and would need a new capability id to stay matrix-tracked. User's call to add or leave for a later version.
obs Rewrite · deferred · Where a11y violation JSON lands — obs-plan §9 (2026-06-27 amendment) holds the run-report envelope and the self-obs log line as DISTINCT record shapes and says violations are self-obs lines; a11y-plan §3 binds a11y emissions to the obs §6 envelope (folding axe rule-id into fingerprints[]); intent F15 follows a11y's wording. A plan-vs-plan conflict route should not silently settle.
a11y Rewrite · deferred (HALF — applied half counted under `adjusted` above; not a fourth deferred item) · Whether the a11y gate is a CI gate or an operator/local gate — a11y-plan §9 says operator/local (dynamic proof needs a live Pulse); intent F15 says "gated in CI" and notes only that it is Linux+xvfb, not Windows. The "no retries" half was applied; the gate nature is deferred.
