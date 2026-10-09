# Security validation — route draft

## Insert
- Between `Bad-version rollout end to end` and `Large-model reading recorded`: **"Agent start boundary — developer's agent started as a fixed program, nothing through a shell; no model credential held; its answer bounded and scrubbed before recording (v4-23, v4-11)"** (epoch: `Epoch 8`)
  Reason: per security-plan §Security Anti-Patterns → Code Patterns (every spawn class is a governed form, a new crossing escalated) and §Input Validation's untrusted-text ingest rows — v4-23 has Conductor itself start a third-party agent whose text enters a record, and v4-11's "no credential of a model's" appears in no draft line.

## Reorder
- Move `Credential hygiene` before `Two-host path reachable`
  Reason: per security-plan §Bootstrap phases `secret-management-init` and §Security Anti-Patterns → Secrets / → Data Protection (never persist credentials into `runs.db` or journals) — the two-host path is the first chunk to read the token and door credential and it writes a recorded verdict, so the read-source and absence-from-artifacts boundary must exist before it.

## Rewrite
- `Conductor's window retired`: "fetched-driver dependency class leave" → "fetched-driver dependency class, Tauri-tree audit exceptions leave"
  Reason: per security-plan §Dependency Security — Accepted exceptions: `deny.toml`'s Tauri-tree advisory ignores and license allows (`deny.toml:14-40`, `:62-64`) are justified by a dependency path that leaves the lock with the window crate, and an exception whose comment names nothing in the tree is retired (the `number_prefix` precedent).

- `Linux-only base CI and harness`: "one Linux job, builds cached, fast checks apart, wall-clock recorded; Windows runners and the PowerShell harness twin leave" → "one Linux job keeping supply-chain and secret-scan gates, builds cached, fast checks apart, wall-clock recorded; Windows runners, PowerShell twin leave"
  Reason: per security-plan §Bootstrap phases `dep-security-ci-gate` and `secret-scanning-ci-gate` — the audit, deny, locked-build and secret-scan steps all live in the `rust` job on the `windows-latest` runner this chunk removes (`.github/workflows/ci.yml:18`, `:59`, `:76`, `:141-145`), so the Foundation CI chunk must name their carry-over.

- `Security posture restated for a harness holding credentials`: "attack surface of remote egress and door" → "remote-egress and door attack surface, the loaded security rule"
  Reason: per security-plan §Secret Management ("Conductor owns no secrets"), materialized as the always-loaded `.claude/rules/security.md` ("no network service, no secrets … never introduce one") — the draft leaves that rule standing until `Records restated` in Epoch 8, so two epochs of credential work would run under a rule forbidding it.

- `Two-host path reachable`: "encrypted channel" → "encrypted, engine-verified channel"
  Reason: per security-plan §Security Anti-Patterns → Data Protection (a target promoted beyond loopback must not skip transport verification) — this is the first non-loopback egress carrying a token, and the draft defers engine verification to the following `Channel refusals`.

- `Credential hygiene`: "absent from run description, record, journal, report, logs" → "absent from arguments, run description, record, journal, report, logs, errors"
  Reason: per security-plan §Security Anti-Patterns → Secrets (never credentials in argv) and §Error Handling (sanitized binary edge) — the absence list omits both surfaces, and "given at run time" leaves an argument open as the carrier.

- `Credential hygiene`: "unauthenticated engine reported blocked" → "secret-scan gate names their shapes"
  Reason: per security-plan §Secret Management → Secret-scan gate shape, the gate is a self-maintained rule set with no entropy scoring, so it catches a credential class only once its shape is named. The displaced blocked arm moves to `Channel refusals` (next item).

- `Channel refusals`: "no token over a plain channel to another host; unverifiable engine refused" → "no token in plaintext to another host; unverifiable engine refused, unauthenticated engine reported blocked"
  Reason: with `Credential hygiene` moved ahead of the two-host path, its blocked arm needs the reachable authenticated path and belongs with the other refusals — a distinct `blocked` state, never a failed check, per security-plan §Security Anti-Patterns → Universal (no silent downgrade).
