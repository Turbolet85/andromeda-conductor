# Playbook — Conductor

<!--
Amendment-validation rules consulted by /andromeda-wrap-session's main agent when it validates the
amendments its fan-out proposed. This file GROWS from dogfood — it starts near-empty. One rule per entry:

  - pattern: {the class of amendment this matches}
    verdict: routine | escalate       # routine → apply silently; escalate → halt + ask the user
    note: {why}

"Main is uneasy" (no rule matches but it looks strange) → escalate too; a confirmed escalation pattern
becomes a new rule here. Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).
-->

## Rules
- pattern: a Foundation-epoch chunk uses plain `cargo test` / `#[test]` before the "Test framework + fixtures" chunk installs cargo-nextest / rstest
  verdict: routine
  note: test-plan §2/§4 names cargo-nextest as the TARGET runner; interim `cargo test` is build-sequencing, not drift against the test strategy. Confirmed with the user on 2026-06-15 (config-validation-surface wrap); recurred from the prior chunk.
- pattern: an external audit/dev CLI tool (e.g. cargo-audit / cargo-deny) is installed a patch/minor behind the version the security-plan names, and the gate runs green
  verdict: routine
  note: dev CLI tools can't be pinned in `Cargo.lock`; the security-plan versions are minimum FLOORS and the real signal (the RustSec advisory DB) is fetched fresh at runtime, so a tool ≥ floor running green satisfies the gate. Confirmed with the user on 2026-06-15 (dependency-audit-gate wrap); security-plan §Dependency Security now states floors.
- pattern: a Foundation-epoch chunk defers a cross-cutting concern (redaction, CI gate, coverage tooling) that the obs/security/test plan already sequences as a SEPARATE downstream bootstrap phase, and the deferral introduces no actual leak/violation in the current chunk
  verdict: routine
  note: a plan that lists the concern as its own bootstrap phase (e.g. obs-plan §3 `pii-scrubbing-wire`) has already sequenced it later; deferring it is build-sequencing, not drift, when the current chunk leaks nothing. Confirmed with the user on 2026-06-15 (structured-logging-stack wrap; D-obs-redaction fired because logging landed before the redaction layer — which is the next chunk). Generalizes the test-runner sequencing rule above.
- pattern: a redaction/scrubbing amendment reconciles a spec's redaction WORDING to a sound host-file-path-anchored implementation (value-scrub on absolute host paths + field-name allowlist + Display-not-Debug edge), where the chunk report shows no actual host-path / struct-name leak
  verdict: routine
  note: when the redaction is implemented and the report demonstrates the invariant holds (host paths masked, no Debug struct dumps, allowlisted identity fields like `target` preserved), reconciling over-literal spec wording (e.g. "remove all `module::` tokens") to the sound model is documentation alignment, not a hygiene gap. Confirmed with the user on 2026-06-15 (log-error-boundary-redaction wrap; D-obs-redaction escalate → reconcile §6/§11). Distinct from the deferral rule above (that was redaction NOT YET built; this is redaction built + reconciling wording).
- pattern: an amendment reconciles a spec's illustrative mechanism or wording to the sound implementation actually shipped, where the chunk report demonstrates the invariant still holds (names / values / contract preserved; only the form or mechanism differs)
  verdict: routine
  note: generalizes the redaction-reconciliation rule above to ANY spec-illustration → sound-impl alignment. When the report proves the invariant holds, correcting an over-literal or framework-naive spec illustration is documentation alignment, not drift. Confirmed with the user on 2026-06-15 (design-token-typography-bundle wrap; design-system §Tokens `@theme` → `:root` because Tailwind v4 tree-shakes non-namespace `@theme` tokens + forbids `@media` nesting). Recurred: redaction wording (log-error-boundary-redaction) → token-emission mechanism (this chunk).
- pattern: a drift detector proposes a dependency bump / version change for a dependency the chunk's report shows it did NOT add or modify, where the manifest already satisfies the spec's stated floor
  verdict: routine
  note: dismiss — NOT this chunk's drift. The report is the single source of what changed this chunk; a dependency it never touched (already at/above the spec floor) is out of scope this wrap. The detector likely read the spec's historical rationale (e.g. "arch originally pinned tauri 2.10.1 → bump to ≥2.10.3") as a current gap. Verify against the manifest before escalating. Confirmed with the user on 2026-06-16 (seeded-phase-scheduler wrap; D-security-deps escalate misfired on `tauri` — already pinned 2.10.3, untouched — while the chunk only added audit-green rand_chacha/rand_core).
- pattern: a drift detector flags an OTel SDK crate (`opentelemetry` / `opentelemetry_sdk`) present in the dependency tree as a TRANSITIVE dep of `opentelemetry-proto` (the PRODUCT OTLP proto lib), where the chunk report shows it is never initialized/used and self-observation remains tracing-JSON-only
  verdict: routine
  note: the no-OTel-SDK invariant (obs-plan §3/§11) is BEHAVIORAL — the harm is initializing/using an SDK for self-obs (batch tasks break `current_thread` determinism + pollute the PRODUCT stream). A dormant transitive SDK crate pulled by opentelemetry-proto's default features does not violate it (nothing runs); a `default-features = false` trim is a tracked optimization, not drift. Confirmed with the user on 2026-06-17 (raw-otlp-message-scaffold wrap; D-obs-stack escalate → accept+document; obs-plan §3 now states the invariant is about SDK init/use, not tree presence).
- pattern: D-arch-resources proposes registering a public library API symbol (struct / enum / fn / const / method) in arch §Occupied Resources, where the chunk's actual occupied resources (ports / sockets / endpoints / IPC methods / events / env vars / workspace crates) are already registered there
  verdict: routine
  note: dismiss — over-reach. arch §Occupied Resources tracks OCCUPIED RESOURCES (ports/sockets/routes/crates/artifacts/env vars), not per-crate public API surface; the detector's own enumerated list does not include library symbols. When the chunk's real resources are already registered (here: the `:4317` socket + the `conductor-faults` crate both already appear in §Occupied Resources), registering its API symbols is detail arch deliberately omits. Precedent: the 8 conductor-emit chunks added many public symbols (RateCurve / EmitError / TraceEmitter / LatencyProfile / PiiCorpus / ServiceTopology …) and registered none — D-arch-resources returned clean on those wraps. Confirmed with the user on 2026-06-19 (port-occupier-fault wrap; D-arch-resources warning → dismiss).
- pattern: a fault-helper PRIMITIVE (a type/constructor in `conductor-faults`) route-sequences its obs `fault.*` span to the later timeline-wiring epoch (Epoch 7/8), and the current chunk emits no span and leaks nothing — D-obs-instrumentation fires on the type's arrival
  verdict: routine
  note: dismiss — the must-trace OPERATION is the fault APPLICATION under the timeline, not the helper type's definition; a primitive that ships before its driver carries no must-trace op yet, so the deferred `fault.silence` / `fault.ramp` / `fault.port_occupier` span is build-sequencing, not drift (obs-plan §4 still describes the intended instrumentation correctly). Generalizes the Foundation-epoch downstream-deferral rule above to Epoch-4 fault helpers. The prior gap chunk (P-015) self-cleared the identical case; this one over-fired only because the report named the `fault_duration_ms`-for-permanent-silence tension (tracked as an Epoch-7/8 follow-up, not a current-truth amendment). Confirmed with the user on 2026-06-20 (abrupt-silence-fault wrap; D-obs-instrumentation warning → dismiss).
