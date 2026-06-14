## 6. Creator Brief Excerpt

_Source: `.andromeda/input.md` (creator brief) + the run-invocation **obs scope corrective** supplied to `/andromeda-obs` (creator's verbatim obs steering for this run). Scope-corrective items are tagged `[scope-corrective]`._

### Must-Work Scenarios

These are the flows the creator names as critical — must-trace priorities for obs Phase 1 (in addition to tests' critical paths). All "observation" here is **Conductor observing Pulse**, never Conductor observing itself.

- **Emission journal as ground truth (the self-record):** "Wall-clock-stamped **emission journal** per run = ground truth of what was sent when (the left side of every SLO check)." `[scope-corrective]` reinforces: "The per-run `<run_id>.jsonl` emission journal ('the left side of every SLO check') is the ground-truth artifact, NOT 'observability telemetry.'"
- **MCP read-back as the observability surface:** "**MCP read-back client** over Pulse's tested tool surface — `query_incident_list` · `retrieve_report` (markdown + degraded_mode flag) · `retrieve_telemetry_slice` · `mark_incident_resolved`." `[scope-corrective]`: "The 'observability' that matters is MCP read-back of Pulse's reaction … the verification layer, not self-instrumentation."
- **Preflight canary before trusting read-back:** "preflight must canary-round-trip a known incident before any scenario trusts read-back."
- **End-to-end proof set (verbatim):** "at minimum `error-baseline-spike`, `fingerprint-storm`, `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce verified expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle` pass observes auto-resolve + resolution summary."
- **Determinism of the stream:** "Deterministic under a seed: same scenario + seed ⇒ same stream shape."
- **Both surfaces emit a journal:** "Every catalog scenario runs from the control panel AND headless; deterministic under a fixed seed; emission journal written per run."

### Rigor Hints

- **Tier intent (verbatim, run-invocation):** `[scope-corrective]` "Net (Minimal tier): keep it light. Self-observation = structured tracing logs (the emission journal + sanitized stderr) + at most a few in-process counters surfaced to the existing Tauri live-counter Channel."
- **Scale / deployment:** "The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host." Local, single-user, no cloud.
- **Determinism is a hard quality bar:** `[scope-corrective]` "same-seed ⇒ same stream is a hard quality bar"; an exporter that "spawns batch/background tasks on the runtime" would break "the `current_thread` determinism the arch deliberately chose."
- **SLOs are Pulse's, measured journal-relative:** `[scope-corrective]` "Every latency/SLO is `read_back_observed_at − journal_emitted_at` against Pulse's behavior. Conductor has no production SLO of its own (it is not a service, and not a load-tester — saturation is explicitly out of scope)." Pulse's per-tier SLO timing: "<5s / <20s / <90s, hardware-profile-aware."
- **Assertion-policy split:** "deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) = hard pass/fail; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting) = calibration-region checks + report-for-human, never hard-failed on exact values."
- **Not a load-tester:** "NOT a load-tester: bounded 'typical / high' load profiles only (P-060 SLO checks); 50k+ spans/sec saturation regimes are explicitly out."
- **Control surface, not a dashboard:** "Control panel (minimal UI) … A control surface, not a dashboard."

### Obs Anti-Patterns (creator's explicit asks)

The OTel/OTLP relationship is **inverted** — these are hard bans for this run.

- **The OTel/OTLP in the stack is the PRODUCT, not the obs mechanism:** `[scope-corrective]` "`opentelemetry-proto` is used to hand-build the fault telemetry Conductor emits AT Pulse (controlled errors / fingerprints / severities). That is Conductor's job — injecting synthetic OTLP into the system-under-test — NOT Conductor instrumenting itself."
- **NO OTel SDK / exporter for self-observation:** `[scope-corrective]` "Do NOT wire `opentelemetry` + `opentelemetry_sdk` + an OTLP exporter to observe Conductor itself." Why: (a) an exporter spawns batch/background tasks, breaking `current_thread` determinism; (b) self-telemetry would pollute the very OTLP stream Conductor injects at Pulse; (c) full OTel self-obs is massive over-scope for a local, single-user, Minimal-tier tool. "(This mirrors Pulse's own hard invariant: 'NO OTel SDK in the self-observation runtime.')"
- **NO metrics backend, NO trace exporter, NO collector, NO self-instrumentation spans:** `[scope-corrective]` "keep it light … NO metrics backend, NO trace exporter, NO collector, NO OTel SDK, NO self-instrumentation spans."
- **Emission-journal format is owned upstream — obs DERIVES, does not re-author:** `[scope-corrective]` "Its shape is already DEFINED by test-plan §3 (structured JSONL via `tracing-subscriber` json; wall-clock stamps from `std::time::SystemTime`/`Instant`, NEVER tokio's virtual clock — that corrupts journal-relative SLO math). obs DERIVES from that, it does not re-author it."
- **The ONE thing obs owns downstream — the redaction layer:** `[scope-corrective]` "the field-allowlist / redaction layer (no absolute host paths, no internal struct names in artifacts)."
- **No recursion — Conductor observes Pulse, not itself:** `[scope-corrective]` "Conductor does not observe itself observing Pulse."
- **PII scrubbing is about the synthetic corpus Conductor EMITS, not user data:** `[scope-corrective]` "PII scrubbing concerns the synthetic corpus Conductor emits, not user data (it owns none)." (The P-047 seven-category corpus — emails, JWT, bearer, API keys, credit cards, SSN, secret-like key=value — is emitted to verify Pulse scrubs it.)
- **No dashboards-only / human-review obs:** "A control surface, not a dashboard." The agent-readable surface is the journal + sanitized stderr, not a Grafana/Kibana UI.
- **No scenario DSL:** "NO scenario DSL — declarative config files + the built-in catalog; new behavior = new P-XXX first."
