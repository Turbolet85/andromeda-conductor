# Security validation — route draft

## Rewrite
- `Channel refusals`: "no token in plaintext to another host" → "no token or door credential in plaintext to another host"
  Reason: per security-plan §Security Anti-Patterns → Data Protection (never plaintext or unverified transport on ANY surface promoted beyond loopback) — Epoch 6 promotes two surfaces, the stream and the door read path, and `Two-host path reachable` hands a run both a token and a door credential, yet the refusal line names only the token, so the door credential's plaintext arm sits on no line.

- `Channel refusals`: "unauthenticated engine reported blocked" → "rejected credential reported blocked"
  Reason: per security-plan §Security Anti-Patterns → Universal (each failed precondition surfaces as its own named `blocked` state, never a generic one) — beside "unverifiable engine refused", "unauthenticated engine" reads as the same engine-identity failure, while v4-11's arm is the opposite direction: the engine refusing the token or door credential Conductor presents.
