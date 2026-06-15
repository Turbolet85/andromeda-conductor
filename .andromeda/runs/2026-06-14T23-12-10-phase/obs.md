# obs extract

## Relevance
relevant — the workspace scaffold pins the obs logging-stack versions + establishes the crate seams that later carry instrumentation; it must NOT pull in a banned OTel SDK.

## Constraints
- No OTel SDK for self-observation; `tracing` 0.1.44 + `tracing-subscriber` 0.3.23 (JSON) are the only self-obs mechanism per obs-plan §3 (OTel SDK init).
- `[workspace.dependencies]` PIN tracing, tracing-subscriber, opentelemetry-proto 0.32.0, tonic 0.14.6 — no floating versions per obs-plan §3 (Logging stack) + §1.
- Service identity via `env!("CARGO_PKG_NAME")` compile-time + `$CONDUCTOR_ENV` runtime per obs-plan §3 (Service identity).
- Crate-per-seam dependency structure enforced — cross-seam edges must fail per obs-plan §1 (Module Boundaries) + §4.
- `Cargo.lock` committed per obs-plan §3 (Bootstrap logger-stack-install) + §9 (CI artifact handling).

## Patterns to follow
- Root `[workspace.package]` + `[workspace.dependencies]` pinned-version pattern per obs-plan §3 + §1.
- Wire `tracing`/`tracing-subscriber` into the binary crates (conductor-cli, conductor-tauri) at logging-init time; the seam libs stay instrumentation-ready but don't import the logger at scaffold time (deferral) per obs-plan §4.
- `rust-toolchain.toml` ≥1.94.1 (security-plan; scope acceptance hints).

## Anti-patterns to avoid
- NEVER add an OTel SDK (`opentelemetry_sdk`/`_api`) — breaks `current_thread` determinism + pollutes the PRODUCT OTLP stream per obs-plan §11 + §1 (creator-explicit ban).
- NEVER hardcode service identity — use `env!("CARGO_PKG_NAME")` per obs-plan §11 (Universal).
- NEVER floating versions for tracing/opentelemetry-proto/tonic in `[workspace.dependencies]` per obs-plan §1.

## Contract bindings
- **obs ↔ tests**: the JSONL log schema (journal_emitted_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) is owned by test-plan §3 and obs aligns to it (obs-plan §3 Log format). The binding materializes in later feature chunks; the scaffold just provides the crates + pinned logger versions.

## Acceptance criteria contributions
- (obs) `cargo build --workspace` succeeds with the logger stack resolving via `[workspace.dependencies]` (obs-plan §3).
- (obs) No OTel SDK in the dependency tree — `cargo tree | grep -i opentelemetry_sdk` returns nothing (obs-plan §11).
- (obs) `Cargo.lock` committed (deterministic obs artifact snapshots) per obs-plan §9.

## Relevant amendment history
(none) — initial cargo scaffold chunk; no prior obs-plan amendments.