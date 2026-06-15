# arch extract

## Relevance — partial
Structured logging is foundational self-observation infrastructure (cross-cutting discipline per §Cross-cutting Patterns), but excludes OTLP emission (separate product path in Epoch 3) and excludes redaction/sanitization (next chunk). This chunk wires the init surface and JSON stack, the two prior stack amendments pin `tracing 0.1.44` + `tracing-subscriber 0.3.23` as already-registered dependencies (§Stack and Technologies / §Established Decisions amends).

## Constraints
1. **Stack pinning:** `tracing 0.1.44` + `tracing-subscriber 0.3.23` (JSON feature) — pre-pinned in prior amendment, per architecture §Stack and Technologies + §Established Decisions.
2. **Not OTel SDK:** Self-observation emits JSON to stdout/file only; OTLP is the separate PRODUCT emission path (per architecture §Design Philosophy "Outcomes are values," §Stack Technologies row, §Established Decisions [OTLP Emission Strategy]). Conflating the two is the anti-pattern this chunk guards against.
3. **Runtime-agnostic init surface:** Callable identically from `conductor-cli` (headless source of truth) and `conductor-tauri` shells per architecture §Design Philosophy "Headless-drivable core, thin shells" and §Inherited Defaults "Runtime-agnostic core library."
4. **Service-identity on every line:** `service.name` + version fields stamped per architecture §Conventions "Outbound emission: OpenTelemetry Semantic Conventions are the shared vocabulary (`service.name`, …)" — applies to self-obs JSON metadata even though self-obs is NOT OTel.
5. **`run_id` correlation key:** Threading on every line satisfies the obs invariant ("every line carrying `run_id`") per the scope's obs contract ("cross-cutting correlation key").
6. **Zero-unlogged-panics invariant:** `std::panic::set_hook` capture routes panics into the structured stream per scope definition.
7. **Workspace boundaries respected:** Init surface lives in `conductor-core` (runtime-agnostic, both CLI and Tauri depend on it) per architecture §Established Decisions "Module Boundaries, Crate-per-seam Cargo workspace" and §Inherited Defaults "Crate-per-seam Cargo workspace."

## Patterns to follow
1. **Determinism discipline** — wall-clock stamps from `std::time::SystemTime`/`Instant`, never tokio's virtual clock (architecture §Cross-cutting Patterns "Determinism discipline"), so journal-relative SLO math is not polluted.
2. **Agent-driven development style** — headless `conductor-cli` path is the release gate and primary testing route; the init surface surfaces to both binaries identically (architecture §Cross-cutting Patterns "Development Style: agent-driven").
3. **Structural seam discipline** — init code (setup + panic hook) lives in `conductor-core` library so it is compile-checked when both binaries link it; no code duplication at the binary edges.

## Anti-patterns to avoid
1. **Pulling OTel SDK** — the ecosystem's `opentelemetry-otlp` exporter or OpenTelemetry's own subscriber is forbidden; architecture pinned raw OTLP message structs precisely to avoid the SDK's batch tasks (§Established Decisions [OTLP Emission Strategy]). Self-obs and product emission are separate paths.
2. **Tokio virtual clock in logs** — using tokio's deterministic virtual-time clock in self-obs timestamps would break journal-relative SLO math; wall-clock from `std::time` is the boundary (architecture §Determinism discipline).
3. **Panics unlogged or logged to stderr/panic args** — scope mandates structured capture; unhandled panics or raw panic messages defeat the zero-unlogged-panics invariant.

## Contract bindings
- **obs↔tests harness:** The structured JSON log stream (this chunk) is the ground truth the test regression harness and golden-file comparisons read; it must parse deterministically and carry `run_id` for correlation (per obs-plan §3/§6 and the scope's "agent-parseable log stream").
- **obs↔security:** Redaction + anyhow-edge sanitization (next chunk) attaches on top of this stack; the seam must be buildable for allowlist injection without restructuring.

## Acceptance criteria contributions
1. **(arch) Init surface lives in `conductor-core` per workspace boundary rules (architecture §Inherited Defaults "Crate-per-seam Cargo workspace").**
2. **(arch) Structured JSON self-obs emits to stdout/file, zero OTel SDK (architecture §Stack "tracing 0.1.44 + tracing-subscriber 0.3.23"; NOT opentelemetry-otlp).**
3. **(arch) Every log line carries `service.name` + `run_id` fields per obs invariant (architecture §Conventions "OpenTelemetry Semantic Conventions"; scope "cross-cutting correlation key").**
4. **(arch) `std::panic::set_hook` routes panics into structured stream (scope "zero-unlogged-panics invariant").**
5. **(arch) Timestamps from `std::time` (not tokio virtual clock) to preserve journal-relative SLO math (architecture §Cross-cutting Patterns "Determinism discipline").**
6. **(arch) `Cargo.lock` remains committed + drift-free, cargo-audit/deny gate green per supply-chain invariant (scope "regression gate … clippy, audit/deny"; prior amendment 2026-06-14).**

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold:** Self-observation stack row added to §Stack and Technologies — pinned `tracing 0.1.44` + `tracing-subscriber 0.3.23` as architecture-registered dependencies (why: obs-plan §3 self-obs stack; NOT an OTel SDK — OTLP remains the PRODUCT emission).
- **2026-06-14-cargo-workspace-scaffold:** MSRV raised 1.88.0 → 1.94.1 (security-plan §Dependency Security required bump ≥1.94.1 for tar-rs CVE-2026-33056); workspace now pins 1.95.0 build toolchain with `rust-version = "1.94.1"` floor — relevant to dependency resolution for `tracing` + `tracing-subscriber`.