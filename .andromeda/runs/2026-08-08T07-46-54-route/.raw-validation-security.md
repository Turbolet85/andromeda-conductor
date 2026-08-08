# Security validation — route draft

## Insert
- Between `SUT load envelope — proven-good storm bounds for scenario authors plus environment-suspect flagging of over-envelope runs` and `Workspace-key divergence probe — two-launch verdict plus a named precondition replacing the opaque corpus Blocked`: **"Secret-scanning CI gate — no secret-shaped string in the `conductor-*` workspace, plus secret-file ignores, build red on any hit"** (epoch: `Epoch 1`)
  Reason: The `secret-scanning-ci-gate` bootstrap phase (security-plan §Bootstrap phases, §Secret Management) is the only unrealized security bootstrap item — absent from `.github/workflows/ci.yml` and from `.gitignore`'s `*.p12`/`*.pem`/`*.cer` guards — and §Anti-Patterns Secrets bans skipping it; Foundation placement puts it before the Epoch 6 release bundle.

## Rewrite
- `SUT capability manifest — versioned refs/ Pulse artifact as the accepted P-ID source, replacing the compile-time 001..=060 bound`: "versioned refs/ Pulse artifact as the accepted P-ID source" → "validated versioned refs/ Pulse artifact as the accepted P-ID source (malformed ⇒ blocked)"
  Reason: This chunk moves the accepted P-ID set from a compile-time bound to a new on-disk parsed artifact — a boundary security-plan §Input Validation requires validated at load (the pinned-manifest row's mismatch ⇒ `blocked`, never a silent downgrade), and no chunk currently carries that marker.
- `A11y CI gate — violation JSON into the obs envelope, service-tagged, failing the build on any violation`: "violation JSON into the obs envelope" → "redacted violation JSON into the obs envelope"
  Reason: The axe violation artifact is a new agent-parseable artifact carrying DOM/asset references, which security-plan §Error Handling (run-report artifact sanitization) and §Anti-Patterns Logging forbid from leaking absolute host paths; requirement v2-24 names the redaction boundary but the chunk drops it.
- `Workspace-key divergence probe — two-launch verdict plus a named precondition replacing the opaque corpus Blocked`: "a named precondition replacing the opaque corpus Blocked" → "a host-path-free named precondition replacing the opaque corpus Blocked"
  Reason: The divergence being named is detected-project-root vs data-dir, so the precondition string recorded into `<run_id>.md` / `runs.db` is path-derived by construction — exactly the absolute-host-path leak banned by security-plan §Anti-Patterns Logging / §Error Handling.
- `pii-scrub live proof — seven PII categories scrubbed across corpus and report excerpts (P-035, P-047, P-048)`: "scrubbed across corpus and report excerpts" → "scrubbed across corpus and report excerpts, verdicts recorded without persisting excerpt content"
  Reason: A failing scrub assertion naturally wants the offending excerpt as evidence, which would persist Pulse corpus content into Conductor's artifacts — banned by security-plan §Anti-Patterns Data Protection (corpus is SUT-owned, read-back only, never persisted).
