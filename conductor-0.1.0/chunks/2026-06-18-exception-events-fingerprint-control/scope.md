# Scope — Exception events + fingerprint control

**Marker:** `2026-06-18-exception-events-fingerprint-control`
**Version:** conductor-0.1.0 · Epoch 3 (Emission primitives) chunk 3/8
**Primary crate:** `conductor-emit` (builds on the prior chunk's `span_tree.rs` / `message.rs`)
**Pulse P-IDs:** P-006 (exception events) · P-017 / P-018 (fingerprint identity + path/line variants, line-insensitive fingerprint)

## What it builds
Exception span **events** on the raw-OTLP error-span scaffold: attach an OTel `exception` span event carrying
`exception.type` / `exception.message` / `exception.stacktrace` to an error span, with **deterministic control
over the stacktrace content** so Conductor can drive Pulse's exception-fingerprinting into known relationships
(authoritative: input.md §Capabilities lines 56-58 + §Scenario catalog line 108, **amended P-017 clause (c)**):

The **same-fingerprint triple** (amended clause (c) — all three derive the SAME fingerprint):
- **identical** — same type + same frames + same line ⇒ the **same** fingerprint;
- **path-variant** — same frames, different file/module path ⇒ the **same** fingerprint (path-insensitive);
- **line-variant** — same frames, different line number ⇒ the **same** fingerprint (line-insensitive).

The **different-fingerprint contrast** (proves the fn discriminates — required for the both-directions test):
- **type-variant** — a different `exception.type` ⇒ a **different** fingerprint;
- **frame-variant** — different stack-frame functions ⇒ a **different** fingerprint.

Plus a **path+line-insensitive fingerprint primitive**: compute — locally, deterministically, stable across
Rust versions/platforms (mirroring the ChaCha8-over-StdRng cross-version-stability decision; shape-projection
not wall-clock) — the fingerprint Conductor **expects** Pulse to derive from a given exception's content. The
fingerprint is a function of exception CONTENT (type + frame-function signatures), NOT of the RNG seed (the
seed drives the exception span's identity bytes, like `error_trace_request`). **Stable across path and line
variation; varies on exception type and frame-function content.** This expected fingerprint is what the
emission journal records (feeding the run-report `fingerprints` array contract and later read-back verification).

## Boundaries / out of scope
- NOT the fingerprint-**storm** fault (storm-cue thresholds, N-distinct-fingerprint generation) — that is
  Epoch 7 "Fingerprint-storm scenarios". Here: the per-exception primitive only.
- NOT severity logs (next chunk, P-007); NOT latency shaping; NOT multi-service topology.
- NOT MCP read-back / verification — emission-side only. This chunk **produces + journals the expected**
  fingerprint; it does not yet assert Pulse's derived one.
- Stays inside loopback gRPC egress to `127.0.0.1:4317`; opens no listener; determinism-under-seed preserved.

## Surfaces / contracts touched
- `conductor-emit` exception-event builder API (extends `span_tree.rs` / `message.rs`; the existing
  `TraceEmitter` egress path is unchanged).
- OTel exception-event semantic convention: the `exception` event name + `exception.type` /
  `exception.message` / `exception.stacktrace` attributes.
- The emission-journal line schema — the `fingerprints` field (tests/obs-owned) and the run-report
  `fingerprints` JSON array (`runs.db` JSON1 TEXT array).

## Boundary to resolve in planning (crate seam)
CLAUDE.md's module map attributes **"fingerprint generation"** to `conductor-faults`, but this chunk sits in
the **emit** epoch and the fingerprint here derives directly from the exception-event content. Resolve whether
the pure fingerprint-compute fn lands in `conductor-emit` (co-located with the exception content it derives
from) or `conductor-faults` (the module-map home, which depends on emit). A crate-seam decision for P4
(AskUserQuestion if the distillers + codebase research don't settle it).

## Acceptance intent (val-1 anchor)
Same seed + same exception spec ⇒ identical emitted exception-event shape **and** identical expected
fingerprint; the **identical / path-variant / line-variant** cases all produce the **SAME** fingerprint
(amended P-017 clause (c)), while **type-variant / frame-variant** produce a **DIFFERENT** one; the expected
fingerprint(s) are journaled per the `RunRecord.fingerprints` line schema; gates green (nextest · clippy `-D` ·
cargo-audit + cargo-deny · `Cargo.lock` un-drifted).
